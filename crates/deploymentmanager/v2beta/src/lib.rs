#[derive(
    Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
)]
pub struct QueryParameters {
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "$.xgafv")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "V1 error format."]
    pub xgafv: ::std::option::Option<QueryParametersXgafvEnum>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "access_token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OAuth access token."]
    pub access_token: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ query_parameters_defaults :: alt () }", setter(into))]
    #[serde(rename = "alt")]
    #[serde(default = "query_parameters_defaults :: alt")]
    #[doc = "Data format for response."]
    pub alt: QueryParametersAltEnum,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "callback")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "JSONP"]
    pub callback: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Selector specifying which fields to include in a partial response."]
    pub fields: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
    pub key: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "oauth_token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OAuth 2.0 token for the current user."]
    pub oauth_token: ::std::option::Option<::std::string::String>,
    #[builder(
        default = "{ query_parameters_defaults :: pretty_print () }",
        setter(into)
    )]
    #[serde(rename = "prettyPrint")]
    #[serde(default = "query_parameters_defaults :: pretty_print")]
    #[doc = "Returns response with indentations and line breaks."]
    pub pretty_print: ::std::primitive::bool,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "quotaUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
    pub quota_user: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "uploadType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
    pub upload_type: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "upload_protocol")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
    pub upload_protocol: ::std::option::Option<::std::string::String>,
}
impl QueryParameters {
    pub fn builder() -> QueryParametersBuilder {
        QueryParametersBuilder::default()
    }
}
mod query_parameters_defaults {
    pub fn alt() -> super::QueryParametersAltEnum {
        serde_json::from_str(&"\"json\"").unwrap()
    }
    pub fn pretty_print() -> ::std::primitive::bool {
        serde_json::from_str(&"true").unwrap()
    }
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "V1 error format."]
pub enum QueryParametersXgafvEnum {
    #[serde(rename = "1")]
    #[doc = "v1 error format"]
    _1,
    #[serde(rename = "2")]
    #[doc = "v2 error format"]
    _2,
}
impl ::std::default::Default for QueryParametersXgafvEnum {
    fn default() -> Self {
        Self::_1
    }
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Data format for response."]
pub enum QueryParametersAltEnum {
    #[serde(rename = "json")]
    #[doc = "Responses with Content-Type of application/json"]
    Json,
    #[serde(rename = "media")]
    #[doc = "Media download with context-dependent Content-Type"]
    Media,
    #[serde(rename = "proto")]
    #[doc = "Responses with Content-Type of application/x-protobuf"]
    Proto,
}
impl ::std::default::Default for QueryParametersAltEnum {
    fn default() -> Self {
        Self::Json
    }
}
pub mod resources {
    pub mod composite_types {
        pub mod methods {
            pub mod list {
                #[derive(
                    Clone,
                    Debug,
                    PartialEq,
                    derive_builder :: Builder,
                    serde :: Serialize,
                    serde :: Deserialize,
                )]
                pub struct QueryParameters {
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "filter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A filter expression that filters resources listed in the response. The expression must specify the field name, a comparison operator, and the value that you want to use for filtering. The value must be a string, a number, or a boolean. The comparison operator must be either `=`, `!=`, `>`, or `<`. For example, if you are filtering Compute Engine instances, you can exclude instances named `example-instance` by specifying `name != example-instance`. You can also filter nested fields. For example, you could specify `scheduling.automaticRestart = false` to include instances only if they are not scheduled for automatic restarts. You can use filtering on nested fields to filter based on resource labels. To filter on multiple expressions, provide each separate expression within parentheses. For example: ``` (scheduling.automaticRestart = true) (cpuPlatform = \"Intel Skylake\") ``` By default, each expression is an `AND` expression. However, you can include `AND` and `OR` expressions explicitly. For example: ``` (cpuPlatform = \"Intel Skylake\") OR (cpuPlatform = \"Intel Broadwell\") AND (scheduling.automaticRestart = true) ```"]
                    pub filter: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "The maximum number of results per page that should be returned. If the number of available results is larger than `maxResults`, Compute Engine returns a `nextPageToken` that can be used to get the next page of results in subsequent list requests. Acceptable values are `0` to `500`, inclusive. (Default: `500`)"]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "orderBy")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Sorts list results by a certain order. By default, results are returned in alphanumerical order based on the resource name. You can also sort results in descending order based on the creation timestamp using `orderBy=\"creationTimestamp desc\"`. This sorts results based on the `creationTimestamp` field in reverse chronological order (newest result first). Use this to sort resources like operations so that the newest operation is returned first. Currently, only sorting by `name` or `creationTimestamp desc` is supported."]
                    pub order_by: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies a page token to use. Set `pageToken` to the `nextPageToken` returned by a previous list request to get the next page of results."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"500").unwrap()
                    }
                }
            }
        }
    }
    pub mod deployments {
        pub mod methods {
            pub mod delete {
                #[derive(
                    Clone,
                    Debug,
                    PartialEq,
                    derive_builder :: Builder,
                    serde :: Serialize,
                    serde :: Deserialize,
                )]
                pub struct QueryParameters {
                    #[builder(
                        default = "{ query_parameters_defaults :: delete_policy () }",
                        setter(into)
                    )]
                    #[serde(rename = "deletePolicy")]
                    #[serde(default = "query_parameters_defaults :: delete_policy")]
                    #[doc = "Sets the policy to use for deleting resources."]
                    pub delete_policy: QueryParametersDeletePolicyEnum,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn delete_policy() -> super::QueryParametersDeletePolicyEnum {
                        serde_json::from_str(&"\"DELETE\"").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Sets the policy to use for deleting resources."]
                pub enum QueryParametersDeletePolicyEnum {
                    #[serde(rename = "DELETE")]
                    #[doc = ""]
                    Delete,
                    #[serde(rename = "ABANDON")]
                    #[doc = ""]
                    Abandon,
                }
                impl ::std::default::Default for QueryParametersDeletePolicyEnum {
                    fn default() -> Self {
                        Self::Delete
                    }
                }
            }
            pub mod get_iam_policy {
                #[derive(
                    Clone,
                    Debug,
                    PartialEq,
                    derive_builder :: Builder,
                    serde :: Serialize,
                    serde :: Deserialize,
                )]
                pub struct QueryParameters {
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "optionsRequestedPolicyVersion")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Requested IAM Policy version."]
                    pub options_requested_policy_version:
                        ::std::option::Option<::std::primitive::i64>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod insert {
                #[derive(
                    Clone,
                    Debug,
                    PartialEq,
                    derive_builder :: Builder,
                    serde :: Serialize,
                    serde :: Deserialize,
                )]
                pub struct QueryParameters {
                    #[builder(
                        default = "{ query_parameters_defaults :: create_policy () }",
                        setter(into)
                    )]
                    #[serde(rename = "createPolicy")]
                    #[serde(default = "query_parameters_defaults :: create_policy")]
                    #[doc = "Sets the policy to use for creating new resources."]
                    pub create_policy: QueryParametersCreatePolicyEnum,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "preview")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If set to true, creates a deployment and creates \"shell\" resources but does not actually instantiate these resources. This allows you to preview what your deployment looks like. After previewing a deployment, you can deploy your resources by making a request with the `update()` method or you can use the `cancelPreview()` method to cancel the preview altogether. Note that the deployment will still exist after you cancel the preview and you must separately delete this deployment if you want to remove it."]
                    pub preview: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn create_policy() -> super::QueryParametersCreatePolicyEnum {
                        serde_json::from_str(&"\"CREATE_OR_ACQUIRE\"").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Sets the policy to use for creating new resources."]
                pub enum QueryParametersCreatePolicyEnum {
                    #[serde(rename = "CREATE_OR_ACQUIRE")]
                    #[doc = ""]
                    CreateOrAcquire,
                    #[serde(rename = "ACQUIRE")]
                    #[doc = ""]
                    Acquire,
                    #[serde(rename = "CREATE")]
                    #[doc = ""]
                    Create,
                }
                impl ::std::default::Default for QueryParametersCreatePolicyEnum {
                    fn default() -> Self {
                        Self::CreateOrAcquire
                    }
                }
            }
            pub mod list {
                #[derive(
                    Clone,
                    Debug,
                    PartialEq,
                    derive_builder :: Builder,
                    serde :: Serialize,
                    serde :: Deserialize,
                )]
                pub struct QueryParameters {
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "filter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A filter expression that filters resources listed in the response. The expression must specify the field name, a comparison operator, and the value that you want to use for filtering. The value must be a string, a number, or a boolean. The comparison operator must be either `=`, `!=`, `>`, or `<`. For example, if you are filtering Compute Engine instances, you can exclude instances named `example-instance` by specifying `name != example-instance`. You can also filter nested fields. For example, you could specify `scheduling.automaticRestart = false` to include instances only if they are not scheduled for automatic restarts. You can use filtering on nested fields to filter based on resource labels. To filter on multiple expressions, provide each separate expression within parentheses. For example: ``` (scheduling.automaticRestart = true) (cpuPlatform = \"Intel Skylake\") ``` By default, each expression is an `AND` expression. However, you can include `AND` and `OR` expressions explicitly. For example: ``` (cpuPlatform = \"Intel Skylake\") OR (cpuPlatform = \"Intel Broadwell\") AND (scheduling.automaticRestart = true) ```"]
                    pub filter: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "The maximum number of results per page that should be returned. If the number of available results is larger than `maxResults`, Compute Engine returns a `nextPageToken` that can be used to get the next page of results in subsequent list requests. Acceptable values are `0` to `500`, inclusive. (Default: `500`)"]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "orderBy")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Sorts list results by a certain order. By default, results are returned in alphanumerical order based on the resource name. You can also sort results in descending order based on the creation timestamp using `orderBy=\"creationTimestamp desc\"`. This sorts results based on the `creationTimestamp` field in reverse chronological order (newest result first). Use this to sort resources like operations so that the newest operation is returned first. Currently, only sorting by `name` or `creationTimestamp desc` is supported."]
                    pub order_by: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies a page token to use. Set `pageToken` to the `nextPageToken` returned by a previous list request to get the next page of results."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"500").unwrap()
                    }
                }
            }
            pub mod patch {
                #[derive(
                    Clone,
                    Debug,
                    PartialEq,
                    derive_builder :: Builder,
                    serde :: Serialize,
                    serde :: Deserialize,
                )]
                pub struct QueryParameters {
                    #[builder(
                        default = "{ query_parameters_defaults :: create_policy () }",
                        setter(into)
                    )]
                    #[serde(rename = "createPolicy")]
                    #[serde(default = "query_parameters_defaults :: create_policy")]
                    #[doc = "Sets the policy to use for creating new resources."]
                    pub create_policy: QueryParametersCreatePolicyEnum,
                    #[builder(
                        default = "{ query_parameters_defaults :: delete_policy () }",
                        setter(into)
                    )]
                    #[serde(rename = "deletePolicy")]
                    #[serde(default = "query_parameters_defaults :: delete_policy")]
                    #[doc = "Sets the policy to use for deleting resources."]
                    pub delete_policy: QueryParametersDeletePolicyEnum,
                    #[builder(
                        default = "{ query_parameters_defaults :: preview () }",
                        setter(into)
                    )]
                    #[serde(rename = "preview")]
                    #[serde(default = "query_parameters_defaults :: preview")]
                    #[doc = "If set to true, updates the deployment and creates and updates the \"shell\" resources but does not actually alter or instantiate these resources. This allows you to preview what your deployment will look like. You can use this intent to preview how an update would affect your deployment. You must provide a `target.config` with a configuration if this is set to true. After previewing a deployment, you can deploy your resources by making a request with the `update()` or you can `cancelPreview()` to remove the preview altogether. Note that the deployment will still exist after you cancel the preview and you must separately delete this deployment if you want to remove it."]
                    pub preview: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn create_policy() -> super::QueryParametersCreatePolicyEnum {
                        serde_json::from_str(&"\"CREATE_OR_ACQUIRE\"").unwrap()
                    }
                    pub fn delete_policy() -> super::QueryParametersDeletePolicyEnum {
                        serde_json::from_str(&"\"DELETE\"").unwrap()
                    }
                    pub fn preview() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Sets the policy to use for creating new resources."]
                pub enum QueryParametersCreatePolicyEnum {
                    #[serde(rename = "CREATE_OR_ACQUIRE")]
                    #[doc = ""]
                    CreateOrAcquire,
                    #[serde(rename = "ACQUIRE")]
                    #[doc = ""]
                    Acquire,
                    #[serde(rename = "CREATE")]
                    #[doc = ""]
                    Create,
                }
                impl ::std::default::Default for QueryParametersCreatePolicyEnum {
                    fn default() -> Self {
                        Self::CreateOrAcquire
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Sets the policy to use for deleting resources."]
                pub enum QueryParametersDeletePolicyEnum {
                    #[serde(rename = "DELETE")]
                    #[doc = ""]
                    Delete,
                    #[serde(rename = "ABANDON")]
                    #[doc = ""]
                    Abandon,
                }
                impl ::std::default::Default for QueryParametersDeletePolicyEnum {
                    fn default() -> Self {
                        Self::Delete
                    }
                }
            }
            pub mod update {
                #[derive(
                    Clone,
                    Debug,
                    PartialEq,
                    derive_builder :: Builder,
                    serde :: Serialize,
                    serde :: Deserialize,
                )]
                pub struct QueryParameters {
                    #[builder(
                        default = "{ query_parameters_defaults :: create_policy () }",
                        setter(into)
                    )]
                    #[serde(rename = "createPolicy")]
                    #[serde(default = "query_parameters_defaults :: create_policy")]
                    #[doc = "Sets the policy to use for creating new resources."]
                    pub create_policy: QueryParametersCreatePolicyEnum,
                    #[builder(
                        default = "{ query_parameters_defaults :: delete_policy () }",
                        setter(into)
                    )]
                    #[serde(rename = "deletePolicy")]
                    #[serde(default = "query_parameters_defaults :: delete_policy")]
                    #[doc = "Sets the policy to use for deleting resources."]
                    pub delete_policy: QueryParametersDeletePolicyEnum,
                    #[builder(
                        default = "{ query_parameters_defaults :: preview () }",
                        setter(into)
                    )]
                    #[serde(rename = "preview")]
                    #[serde(default = "query_parameters_defaults :: preview")]
                    #[doc = "If set to true, updates the deployment and creates and updates the \"shell\" resources but does not actually alter or instantiate these resources. This allows you to preview what your deployment will look like. You can use this intent to preview how an update would affect your deployment. You must provide a `target.config` with a configuration if this is set to true. After previewing a deployment, you can deploy your resources by making a request with the `update()` or you can `cancelPreview()` to remove the preview altogether. Note that the deployment will still exist after you cancel the preview and you must separately delete this deployment if you want to remove it."]
                    pub preview: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn create_policy() -> super::QueryParametersCreatePolicyEnum {
                        serde_json::from_str(&"\"CREATE_OR_ACQUIRE\"").unwrap()
                    }
                    pub fn delete_policy() -> super::QueryParametersDeletePolicyEnum {
                        serde_json::from_str(&"\"DELETE\"").unwrap()
                    }
                    pub fn preview() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Sets the policy to use for creating new resources."]
                pub enum QueryParametersCreatePolicyEnum {
                    #[serde(rename = "CREATE_OR_ACQUIRE")]
                    #[doc = ""]
                    CreateOrAcquire,
                    #[serde(rename = "ACQUIRE")]
                    #[doc = ""]
                    Acquire,
                    #[serde(rename = "CREATE")]
                    #[doc = ""]
                    Create,
                }
                impl ::std::default::Default for QueryParametersCreatePolicyEnum {
                    fn default() -> Self {
                        Self::CreateOrAcquire
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Sets the policy to use for deleting resources."]
                pub enum QueryParametersDeletePolicyEnum {
                    #[serde(rename = "DELETE")]
                    #[doc = ""]
                    Delete,
                    #[serde(rename = "ABANDON")]
                    #[doc = ""]
                    Abandon,
                }
                impl ::std::default::Default for QueryParametersDeletePolicyEnum {
                    fn default() -> Self {
                        Self::Delete
                    }
                }
            }
        }
    }
    pub mod manifests {
        pub mod methods {
            pub mod list {
                #[derive(
                    Clone,
                    Debug,
                    PartialEq,
                    derive_builder :: Builder,
                    serde :: Serialize,
                    serde :: Deserialize,
                )]
                pub struct QueryParameters {
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "filter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A filter expression that filters resources listed in the response. The expression must specify the field name, a comparison operator, and the value that you want to use for filtering. The value must be a string, a number, or a boolean. The comparison operator must be either `=`, `!=`, `>`, or `<`. For example, if you are filtering Compute Engine instances, you can exclude instances named `example-instance` by specifying `name != example-instance`. You can also filter nested fields. For example, you could specify `scheduling.automaticRestart = false` to include instances only if they are not scheduled for automatic restarts. You can use filtering on nested fields to filter based on resource labels. To filter on multiple expressions, provide each separate expression within parentheses. For example: ``` (scheduling.automaticRestart = true) (cpuPlatform = \"Intel Skylake\") ``` By default, each expression is an `AND` expression. However, you can include `AND` and `OR` expressions explicitly. For example: ``` (cpuPlatform = \"Intel Skylake\") OR (cpuPlatform = \"Intel Broadwell\") AND (scheduling.automaticRestart = true) ```"]
                    pub filter: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "The maximum number of results per page that should be returned. If the number of available results is larger than `maxResults`, Compute Engine returns a `nextPageToken` that can be used to get the next page of results in subsequent list requests. Acceptable values are `0` to `500`, inclusive. (Default: `500`)"]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "orderBy")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Sorts list results by a certain order. By default, results are returned in alphanumerical order based on the resource name. You can also sort results in descending order based on the creation timestamp using `orderBy=\"creationTimestamp desc\"`. This sorts results based on the `creationTimestamp` field in reverse chronological order (newest result first). Use this to sort resources like operations so that the newest operation is returned first. Currently, only sorting by `name` or `creationTimestamp desc` is supported."]
                    pub order_by: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies a page token to use. Set `pageToken` to the `nextPageToken` returned by a previous list request to get the next page of results."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"500").unwrap()
                    }
                }
            }
        }
    }
    pub mod operations {
        pub mod methods {
            pub mod list {
                #[derive(
                    Clone,
                    Debug,
                    PartialEq,
                    derive_builder :: Builder,
                    serde :: Serialize,
                    serde :: Deserialize,
                )]
                pub struct QueryParameters {
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "filter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A filter expression that filters resources listed in the response. The expression must specify the field name, a comparison operator, and the value that you want to use for filtering. The value must be a string, a number, or a boolean. The comparison operator must be either `=`, `!=`, `>`, or `<`. For example, if you are filtering Compute Engine instances, you can exclude instances named `example-instance` by specifying `name != example-instance`. You can also filter nested fields. For example, you could specify `scheduling.automaticRestart = false` to include instances only if they are not scheduled for automatic restarts. You can use filtering on nested fields to filter based on resource labels. To filter on multiple expressions, provide each separate expression within parentheses. For example: ``` (scheduling.automaticRestart = true) (cpuPlatform = \"Intel Skylake\") ``` By default, each expression is an `AND` expression. However, you can include `AND` and `OR` expressions explicitly. For example: ``` (cpuPlatform = \"Intel Skylake\") OR (cpuPlatform = \"Intel Broadwell\") AND (scheduling.automaticRestart = true) ```"]
                    pub filter: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "The maximum number of results per page that should be returned. If the number of available results is larger than `maxResults`, Compute Engine returns a `nextPageToken` that can be used to get the next page of results in subsequent list requests. Acceptable values are `0` to `500`, inclusive. (Default: `500`)"]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "orderBy")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Sorts list results by a certain order. By default, results are returned in alphanumerical order based on the resource name. You can also sort results in descending order based on the creation timestamp using `orderBy=\"creationTimestamp desc\"`. This sorts results based on the `creationTimestamp` field in reverse chronological order (newest result first). Use this to sort resources like operations so that the newest operation is returned first. Currently, only sorting by `name` or `creationTimestamp desc` is supported."]
                    pub order_by: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies a page token to use. Set `pageToken` to the `nextPageToken` returned by a previous list request to get the next page of results."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"500").unwrap()
                    }
                }
            }
        }
    }
    pub mod resources {
        pub mod methods {
            pub mod list {
                #[derive(
                    Clone,
                    Debug,
                    PartialEq,
                    derive_builder :: Builder,
                    serde :: Serialize,
                    serde :: Deserialize,
                )]
                pub struct QueryParameters {
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "filter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A filter expression that filters resources listed in the response. The expression must specify the field name, a comparison operator, and the value that you want to use for filtering. The value must be a string, a number, or a boolean. The comparison operator must be either `=`, `!=`, `>`, or `<`. For example, if you are filtering Compute Engine instances, you can exclude instances named `example-instance` by specifying `name != example-instance`. You can also filter nested fields. For example, you could specify `scheduling.automaticRestart = false` to include instances only if they are not scheduled for automatic restarts. You can use filtering on nested fields to filter based on resource labels. To filter on multiple expressions, provide each separate expression within parentheses. For example: ``` (scheduling.automaticRestart = true) (cpuPlatform = \"Intel Skylake\") ``` By default, each expression is an `AND` expression. However, you can include `AND` and `OR` expressions explicitly. For example: ``` (cpuPlatform = \"Intel Skylake\") OR (cpuPlatform = \"Intel Broadwell\") AND (scheduling.automaticRestart = true) ```"]
                    pub filter: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "The maximum number of results per page that should be returned. If the number of available results is larger than `maxResults`, Compute Engine returns a `nextPageToken` that can be used to get the next page of results in subsequent list requests. Acceptable values are `0` to `500`, inclusive. (Default: `500`)"]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "orderBy")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Sorts list results by a certain order. By default, results are returned in alphanumerical order based on the resource name. You can also sort results in descending order based on the creation timestamp using `orderBy=\"creationTimestamp desc\"`. This sorts results based on the `creationTimestamp` field in reverse chronological order (newest result first). Use this to sort resources like operations so that the newest operation is returned first. Currently, only sorting by `name` or `creationTimestamp desc` is supported."]
                    pub order_by: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies a page token to use. Set `pageToken` to the `nextPageToken` returned by a previous list request to get the next page of results."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"500").unwrap()
                    }
                }
            }
        }
    }
    pub mod type_providers {
        pub mod methods {
            pub mod list {
                #[derive(
                    Clone,
                    Debug,
                    PartialEq,
                    derive_builder :: Builder,
                    serde :: Serialize,
                    serde :: Deserialize,
                )]
                pub struct QueryParameters {
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "filter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A filter expression that filters resources listed in the response. The expression must specify the field name, a comparison operator, and the value that you want to use for filtering. The value must be a string, a number, or a boolean. The comparison operator must be either `=`, `!=`, `>`, or `<`. For example, if you are filtering Compute Engine instances, you can exclude instances named `example-instance` by specifying `name != example-instance`. You can also filter nested fields. For example, you could specify `scheduling.automaticRestart = false` to include instances only if they are not scheduled for automatic restarts. You can use filtering on nested fields to filter based on resource labels. To filter on multiple expressions, provide each separate expression within parentheses. For example: ``` (scheduling.automaticRestart = true) (cpuPlatform = \"Intel Skylake\") ``` By default, each expression is an `AND` expression. However, you can include `AND` and `OR` expressions explicitly. For example: ``` (cpuPlatform = \"Intel Skylake\") OR (cpuPlatform = \"Intel Broadwell\") AND (scheduling.automaticRestart = true) ```"]
                    pub filter: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "The maximum number of results per page that should be returned. If the number of available results is larger than `maxResults`, Compute Engine returns a `nextPageToken` that can be used to get the next page of results in subsequent list requests. Acceptable values are `0` to `500`, inclusive. (Default: `500`)"]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "orderBy")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Sorts list results by a certain order. By default, results are returned in alphanumerical order based on the resource name. You can also sort results in descending order based on the creation timestamp using `orderBy=\"creationTimestamp desc\"`. This sorts results based on the `creationTimestamp` field in reverse chronological order (newest result first). Use this to sort resources like operations so that the newest operation is returned first. Currently, only sorting by `name` or `creationTimestamp desc` is supported."]
                    pub order_by: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies a page token to use. Set `pageToken` to the `nextPageToken` returned by a previous list request to get the next page of results."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"500").unwrap()
                    }
                }
            }
            pub mod list_types {
                #[derive(
                    Clone,
                    Debug,
                    PartialEq,
                    derive_builder :: Builder,
                    serde :: Serialize,
                    serde :: Deserialize,
                )]
                pub struct QueryParameters {
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "filter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A filter expression that filters resources listed in the response. The expression must specify the field name, a comparison operator, and the value that you want to use for filtering. The value must be a string, a number, or a boolean. The comparison operator must be either `=`, `!=`, `>`, or `<`. For example, if you are filtering Compute Engine instances, you can exclude instances named `example-instance` by specifying `name != example-instance`. You can also filter nested fields. For example, you could specify `scheduling.automaticRestart = false` to include instances only if they are not scheduled for automatic restarts. You can use filtering on nested fields to filter based on resource labels. To filter on multiple expressions, provide each separate expression within parentheses. For example: ``` (scheduling.automaticRestart = true) (cpuPlatform = \"Intel Skylake\") ``` By default, each expression is an `AND` expression. However, you can include `AND` and `OR` expressions explicitly. For example: ``` (cpuPlatform = \"Intel Skylake\") OR (cpuPlatform = \"Intel Broadwell\") AND (scheduling.automaticRestart = true) ```"]
                    pub filter: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "The maximum number of results per page that should be returned. If the number of available results is larger than `maxResults`, Compute Engine returns a `nextPageToken` that can be used to get the next page of results in subsequent list requests. Acceptable values are `0` to `500`, inclusive. (Default: `500`)"]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "orderBy")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Sorts list results by a certain order. By default, results are returned in alphanumerical order based on the resource name. You can also sort results in descending order based on the creation timestamp using `orderBy=\"creationTimestamp desc\"`. This sorts results based on the `creationTimestamp` field in reverse chronological order (newest result first). Use this to sort resources like operations so that the newest operation is returned first. Currently, only sorting by `name` or `creationTimestamp desc` is supported."]
                    pub order_by: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies a page token to use. Set `pageToken` to the `nextPageToken` returned by a previous list request to get the next page of results."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"500").unwrap()
                    }
                }
            }
        }
    }
    pub mod types {
        pub mod methods {
            pub mod list {
                #[derive(
                    Clone,
                    Debug,
                    PartialEq,
                    derive_builder :: Builder,
                    serde :: Serialize,
                    serde :: Deserialize,
                )]
                pub struct QueryParameters {
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "filter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A filter expression that filters resources listed in the response. The expression must specify the field name, a comparison operator, and the value that you want to use for filtering. The value must be a string, a number, or a boolean. The comparison operator must be either `=`, `!=`, `>`, or `<`. For example, if you are filtering Compute Engine instances, you can exclude instances named `example-instance` by specifying `name != example-instance`. You can also filter nested fields. For example, you could specify `scheduling.automaticRestart = false` to include instances only if they are not scheduled for automatic restarts. You can use filtering on nested fields to filter based on resource labels. To filter on multiple expressions, provide each separate expression within parentheses. For example: ``` (scheduling.automaticRestart = true) (cpuPlatform = \"Intel Skylake\") ``` By default, each expression is an `AND` expression. However, you can include `AND` and `OR` expressions explicitly. For example: ``` (cpuPlatform = \"Intel Skylake\") OR (cpuPlatform = \"Intel Broadwell\") AND (scheduling.automaticRestart = true) ```"]
                    pub filter: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "The maximum number of results per page that should be returned. If the number of available results is larger than `maxResults`, Compute Engine returns a `nextPageToken` that can be used to get the next page of results in subsequent list requests. Acceptable values are `0` to `500`, inclusive. (Default: `500`)"]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "orderBy")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Sorts list results by a certain order. By default, results are returned in alphanumerical order based on the resource name. You can also sort results in descending order based on the creation timestamp using `orderBy=\"creationTimestamp desc\"`. This sorts results based on the `creationTimestamp` field in reverse chronological order (newest result first). Use this to sort resources like operations so that the newest operation is returned first. Currently, only sorting by `name` or `creationTimestamp desc` is supported."]
                    pub order_by: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies a page token to use. Set `pageToken` to the `nextPageToken` returned by a previous list request to get the next page of results."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"500").unwrap()
                    }
                }
            }
        }
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Async options that determine when a resource should finish."]
    pub struct AsyncOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "methodMatch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Method regex where this policy will apply."]
        pub method_match: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pollingOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deployment manager will poll instances for this API resource setting a RUNNING state, and blocking until polling conditions tell whether the resource is completed or failed."]
        pub polling_options: ::std::option::Option<::std::boxed::Box<PollingOptions>>,
    }
    impl AsyncOptions {
        pub fn builder() -> AsyncOptionsBuilder {
            AsyncOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { \"audit_configs\": [ { \"service\": \"allServices\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" }, { \"log_type\": \"ADMIN_READ\" } ] }, { \"service\": \"sampleservice.googleapis.com\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\" }, { \"log_type\": \"DATA_WRITE\", \"exempted_members\": [ \"user:aliya@example.com\" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts jose@example.com from DATA_READ logging, and aliya@example.com from DATA_WRITE logging."]
    pub struct AuditConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "auditLogConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The configuration for logging of each type of permission."]
        pub audit_log_configs:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuditLogConfig>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "service")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services."]
        pub service: ::std::option::Option<::std::string::String>,
    }
    impl AuditConfig {
        pub fn builder() -> AuditConfigBuilder {
            AuditConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Provides the configuration for logging a type of permissions. Example: { \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging."]
    pub struct AuditLogConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exemptedMembers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members."]
        pub exempted_members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The log type that this config enables."]
        pub log_type: ::std::option::Option<AuditLogConfigLogTypeEnum>,
    }
    impl AuditLogConfig {
        pub fn builder() -> AuditLogConfigBuilder {
            AuditLogConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The log type that this config enables."]
    pub enum AuditLogConfigLogTypeEnum {
        #[serde(rename = "LOG_TYPE_UNSPECIFIED")]
        #[doc = "Default case. Should never be this."]
        LogTypeUnspecified,
        #[serde(rename = "ADMIN_READ")]
        #[doc = "Admin reads. Example: CloudIAM getIamPolicy"]
        AdminRead,
        #[serde(rename = "DATA_WRITE")]
        #[doc = "Data writes. Example: CloudSQL Users create"]
        DataWrite,
        #[serde(rename = "DATA_READ")]
        #[doc = "Data reads. Example: CloudSQL Users list"]
        DataRead,
    }
    impl ::std::default::Default for AuditLogConfigLogTypeEnum {
        fn default() -> Self {
            Self::LogTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "BaseType that describes a service-backed Type."]
    pub struct BaseType {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "collectionOverrides")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allows resource handling overrides for specific collections"]
        pub collection_overrides:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CollectionOverride>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "credential")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Credential used when interacting with this type."]
        pub credential: ::std::option::Option<::std::boxed::Box<Credential>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "descriptorUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Descriptor Url for the this type."]
        pub descriptor_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "options")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Options to apply when handling any resources in this service."]
        pub options: ::std::option::Option<::std::boxed::Box<Options>>,
    }
    impl BaseType {
        pub fn builder() -> BaseTypeBuilder {
            BaseTypeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Basic Auth used as a credential."]
    pub struct BasicAuth {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "password")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub password: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "user")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub user: ::std::option::Option<::std::string::String>,
    }
    impl BasicAuth {
        pub fn builder() -> BasicAuthBuilder {
            BasicAuthBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Associates `members` with a `role`."]
    pub struct Binding {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "condition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the members in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        pub condition: ::std::option::Option<::std::boxed::Box<Expr>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "members")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the identities requesting access for a Cloud Platform resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. "]
        pub members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Role that is assigned to `members`. For example, `roles/viewer`, `roles/editor`, or `roles/owner`."]
        pub role: ::std::option::Option<::std::string::String>,
    }
    impl Binding {
        pub fn builder() -> BindingBuilder {
            BindingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "CollectionOverride allows resource handling overrides for specific resources within a BaseType"]
    pub struct CollectionOverride {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "collection")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The collection that identifies this resource within its service."]
        pub collection: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "options")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The options to apply to this resource-level override"]
        pub options: ::std::option::Option<::std::boxed::Box<Options>>,
    }
    impl CollectionOverride {
        pub fn builder() -> CollectionOverrideBuilder {
            CollectionOverrideBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Holds the composite type."]
    pub struct CompositeType {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional textual description of the resource; provided by the client when the resource is created."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Creation timestamp in RFC3339 text format."]
        pub insert_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Map of labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`."]
        pub labels:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CompositeTypeLabelEntry>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the composite type, must follow the expression: `[a-z]([-a-z0-9_.]{0,61}[a-z0-9])?`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The Operation that most recently ran, or is currently running, on this composite type."]
        pub operation: ::std::option::Option<::std::boxed::Box<Operation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Server defined URL for the resource."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<CompositeTypeStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "templateContents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Files for the template type."]
        pub template_contents: ::std::option::Option<::std::boxed::Box<TemplateContents>>,
    }
    impl CompositeType {
        pub fn builder() -> CompositeTypeBuilder {
            CompositeTypeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum CompositeTypeStatusEnum {
        #[serde(rename = "UNKNOWN_STATUS")]
        #[doc = ""]
        UnknownStatus,
        #[serde(rename = "DEPRECATED")]
        #[doc = ""]
        Deprecated,
        #[serde(rename = "EXPERIMENTAL")]
        #[doc = ""]
        Experimental,
        #[serde(rename = "SUPPORTED")]
        #[doc = ""]
        Supported,
    }
    impl ::std::default::Default for CompositeTypeStatusEnum {
        fn default() -> Self {
            Self::UnknownStatus
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Label object for CompositeTypes"]
    pub struct CompositeTypeLabelEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Key of the label"]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value of the label"]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl CompositeTypeLabelEntry {
        pub fn builder() -> CompositeTypeLabelEntryBuilder {
            CompositeTypeLabelEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A response that returns all Composite Types supported by Deployment Manager"]
    pub struct CompositeTypesListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "compositeTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A list of resource composite types supported by Deployment Manager."]
        pub composite_types:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CompositeType>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token used to continue a truncated list request."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl CompositeTypesListResponse {
        pub fn builder() -> CompositeTypesListResponseBuilder {
            CompositeTypesListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ConfigFile {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The contents of the file."]
        pub content: ::std::option::Option<::std::string::String>,
    }
    impl ConfigFile {
        pub fn builder() -> ConfigFileBuilder {
            ConfigFileBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The credential used by Deployment Manager and TypeProvider. Only one of the options is permitted."]
    pub struct Credential {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicAuth")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic Auth Credential, only used by TypeProvider."]
        pub basic_auth: ::std::option::Option<::std::boxed::Box<BasicAuth>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceAccount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Service Account Credential, only used by Deployment."]
        pub service_account: ::std::option::Option<::std::boxed::Box<ServiceAccount>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "useProjectDefault")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specify to use the project default credential, only supported by Deployment."]
        pub use_project_default: ::std::option::Option<::std::primitive::bool>,
    }
    impl Credential {
        pub fn builder() -> CredentialBuilder {
            CredentialBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Deployment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional user-provided description of the deployment."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fingerprint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Provides a fingerprint to use in requests to modify a deployment, such as `update()`, `stop()`, and `cancelPreview()` requests. A fingerprint is a randomly generated value that must be provided with `update()`, `stop()`, and `cancelPreview()` requests to perform optimistic locking. This ensures optimistic concurrency so that only one request happens at a time. The fingerprint is initially generated by Deployment Manager and changes after every request to modify data. To get the latest fingerprint value, perform a `get()` request to a deployment."]
        pub fingerprint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Creation timestamp in RFC3339 text format."]
        pub insert_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`."]
        pub labels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DeploymentLabelEntry>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "manifest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. URL of the manifest representing the last manifest that was successfully deployed. If no manifest has been successfully deployed, this field will be absent."]
        pub manifest: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The Operation that most recently ran, or is currently running, on this deployment."]
        pub operation: ::std::option::Option<::std::boxed::Box<Operation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Server defined URL for the resource."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "target")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Input Only] The parameters that define your deployment, including the deployment configuration and relevant templates."]
        pub target: ::std::option::Option<::std::boxed::Box<TargetConfiguration>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "update")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. If Deployment Manager is currently updating or previewing an update to this deployment, the updated configuration appears here."]
        pub update: ::std::option::Option<::std::boxed::Box<DeploymentUpdate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Update timestamp in RFC3339 text format."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl Deployment {
        pub fn builder() -> DeploymentBuilder {
            DeploymentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Label object for Deployments"]
    pub struct DeploymentLabelEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Key of the label"]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value of the label"]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl DeploymentLabelEntry {
        pub fn builder() -> DeploymentLabelEntryBuilder {
            DeploymentLabelEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DeploymentUpdate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. An optional user-provided description of the deployment after the current update has been applied."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`."]
        pub labels:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DeploymentUpdateLabelEntry>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "manifest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. URL of the manifest representing the update configuration of this deployment."]
        pub manifest: ::std::option::Option<::std::string::String>,
    }
    impl DeploymentUpdate {
        pub fn builder() -> DeploymentUpdateBuilder {
            DeploymentUpdateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Label object for DeploymentUpdate"]
    pub struct DeploymentUpdateLabelEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Key of the label"]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value of the label"]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl DeploymentUpdateLabelEntry {
        pub fn builder() -> DeploymentUpdateLabelEntryBuilder {
            DeploymentUpdateLabelEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DeploymentsCancelPreviewRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fingerprint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies a fingerprint for `cancelPreview()` requests. A fingerprint is a randomly generated value that must be provided in `cancelPreview()` requests to perform optimistic locking. This ensures optimistic concurrency so that the deployment does not have conflicting requests (e.g. if someone attempts to make a new update request while another user attempts to cancel a preview, this would prevent one of the requests). The fingerprint is initially generated by Deployment Manager and changes after every request to modify a deployment. To get the latest fingerprint value, perform a `get()` request on the deployment."]
        pub fingerprint: ::std::option::Option<::std::string::String>,
    }
    impl DeploymentsCancelPreviewRequest {
        pub fn builder() -> DeploymentsCancelPreviewRequestBuilder {
            DeploymentsCancelPreviewRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A response containing a partial list of deployments and a page token used to build the next request if the request has been truncated."]
    pub struct DeploymentsListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deployments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The deployments contained in this response."]
        pub deployments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Deployment>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A token used to continue a truncated list request."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl DeploymentsListResponse {
        pub fn builder() -> DeploymentsListResponseBuilder {
            DeploymentsListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DeploymentsStopRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fingerprint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies a fingerprint for `stop()` requests. A fingerprint is a randomly generated value that must be provided in `stop()` requests to perform optimistic locking. This ensures optimistic concurrency so that the deployment does not have conflicting requests (e.g. if someone attempts to make a new update request while another user attempts to stop an ongoing update request, this would prevent a collision). The fingerprint is initially generated by Deployment Manager and changes after every request to modify a deployment. To get the latest fingerprint value, perform a `get()` request on the deployment."]
        pub fingerprint: ::std::option::Option<::std::string::String>,
    }
    impl DeploymentsStopRequest {
        pub fn builder() -> DeploymentsStopRequestBuilder {
            DeploymentsStopRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Diagnostic {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "field")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "JsonPath expression on the resource that if non empty, indicates that this field needs to be extracted as a diagnostic."]
        pub field: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "level")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Level to record this diagnostic."]
        pub level: ::std::option::Option<DiagnosticLevelEnum>,
    }
    impl Diagnostic {
        pub fn builder() -> DiagnosticBuilder {
            DiagnosticBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Level to record this diagnostic."]
    pub enum DiagnosticLevelEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = ""]
        Unknown,
        #[serde(rename = "INFORMATION")]
        #[doc = "If level is informational, it only gets displayed as part of the resource."]
        Information,
        #[serde(rename = "WARNING")]
        #[doc = "If level is warning, will end up in the resource as a warning."]
        Warning,
        #[serde(rename = "ERROR")]
        #[doc = "If level is error, it will indicate an error occurred after finishCondition is set, and this field will populate resource errors and operation errors."]
        Error,
    }
    impl ::std::default::Default for DiagnosticLevelEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: \"Summary size limit\" description: \"Determines if a summary is less than 100 chars\" expression: \"document.summary.size() < 100\" Example (Equality): title: \"Requestor is owner\" description: \"Determines if requestor is the document owner\" expression: \"document.owner == request.auth.claims.email\" Example (Logic): title: \"Public documents\" description: \"Determine whether the document should be publicly visible\" expression: \"document.type != 'private' && document.type != 'internal'\" Example (Data Manipulation): title: \"Notification string\" description: \"Create a notification string with a timestamp.\" expression: \"'New message received at ' + string(document.create_time)\" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information."]
    pub struct Expr {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expression")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Textual representation of an expression in Common Expression Language syntax."]
        pub expression: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl Expr {
        pub fn builder() -> ExprBuilder {
            ExprBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GlobalSetPolicyRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bindings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Flatten Policy to create a backward compatible wire-format. Deprecated. Use 'policy' to specify bindings."]
        pub bindings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Binding>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Flatten Policy to create a backward compatible wire-format. Deprecated. Use 'policy' to specify the etag."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "REQUIRED: The complete policy to be applied to the 'resource'. The size of the policy is limited to a few 10s of KB. An empty policy is in general a valid policy but certain services (like Projects) might reject them."]
        pub policy: ::std::option::Option<::std::boxed::Box<Policy>>,
    }
    impl GlobalSetPolicyRequest {
        pub fn builder() -> GlobalSetPolicyRequestBuilder {
            GlobalSetPolicyRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ImportFile {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The contents of the file."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the file."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl ImportFile {
        pub fn builder() -> ImportFileBuilder {
            ImportFileBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "InputMapping creates a 'virtual' property that will be injected into the properties before sending the request to the underlying API."]
    pub struct InputMapping {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the field that is going to be injected."]
        pub field_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location where this mapping applies."]
        pub location: ::std::option::Option<InputMappingLocationEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "methodMatch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Regex to evaluate on method to decide if input applies."]
        pub method_match: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A jsonPath expression to select an element."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl InputMapping {
        pub fn builder() -> InputMappingBuilder {
            InputMappingBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The location where this mapping applies."]
    pub enum InputMappingLocationEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = ""]
        Unknown,
        #[serde(rename = "PATH")]
        #[doc = ""]
        Path,
        #[serde(rename = "QUERY")]
        #[doc = ""]
        Query,
        #[serde(rename = "BODY")]
        #[doc = ""]
        Body,
        #[serde(rename = "HEADER")]
        #[doc = ""]
        Header,
    }
    impl ::std::default::Default for InputMappingLocationEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Manifest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "config")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The YAML configuration for this manifest."]
        pub config: ::std::option::Option<::std::boxed::Box<ConfigFile>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expandedConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The fully-expanded configuration file, including any templates and references."]
        pub expanded_config: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imports")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The imported files for this manifest."]
        pub imports: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ImportFile>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Creation timestamp in RFC3339 text format."]
        pub insert_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The YAML layout for this manifest."]
        pub layout: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "manifestSizeBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The computed size of the fully expanded manifest."]
        pub manifest_size_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "manifestSizeLimitBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The size limit for expanded manifests in the project."]
        pub manifest_size_limit_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of the manifest."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Self link for the manifest."]
        pub self_link: ::std::option::Option<::std::string::String>,
    }
    impl Manifest {
        pub fn builder() -> ManifestBuilder {
            ManifestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A response containing a partial list of manifests and a page token used to build the next request if the request has been truncated."]
    pub struct ManifestsListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "manifests")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Manifests contained in this list response."]
        pub manifests: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Manifest>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A token used to continue a truncated list request."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ManifestsListResponse {
        pub fn builder() -> ManifestsListResponseBuilder {
            ManifestsListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents an Operation resource. Google Compute Engine has three Operation resources: * [Global](/compute/docs/reference/rest/{$api_version}/globalOperations) * [Regional](/compute/docs/reference/rest/{$api_version}/regionOperations) * [Zonal](/compute/docs/reference/rest/{$api_version}/zoneOperations) You can use an operation resource to manage asynchronous API requests. For more information, read Handling API responses. Operations can be global, regional or zonal. - For global operations, use the `globalOperations` resource. - For regional operations, use the `regionOperations` resource. - For zonal operations, use the `zonalOperations` resource. For more information, read Global, Regional, and Zonal Resources."]
    pub struct Operation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientOperationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] The value of `requestId` if you provided it in the request. Not present otherwise."]
        pub client_operation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Deprecated] This field is deprecated."]
        pub creation_timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] A textual description of the operation, which is set when the operation is created."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] The time that this operation was completed. This value is in RFC3339 text format."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] If errors are generated during processing of the operation, this field will be populated."]
        pub error: ::std::option::Option<OperationError>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "httpErrorMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] If the operation fails, this field contains the HTTP error message that was returned, such as `NOT FOUND`."]
        pub http_error_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "httpErrorStatusCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] If the operation fails, this field contains the HTTP error status code that was returned. For example, a `404` means the resource was not found."]
        pub http_error_status_code: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] The unique identifier for the operation. This identifier is defined by the server."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] The time that this operation was requested. This value is in RFC3339 text format."]
        pub insert_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ operation_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "operation_defaults :: kind")]
        #[doc = "[Output Only] Type of the resource. Always `compute#operation` for Operation resources."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] Name of the operation."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationGroupId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] An ID that represents a group of operations, such as when a group of operations results from a `bulkInsert` API request."]
        pub operation_group_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] The type of operation, such as `insert`, `update`, or `delete`, and so on."]
        pub operation_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] An optional progress indicator that ranges from 0 to 100. There is no requirement that this be linear or support any granularity of operations. This should not be used to guess when the operation will be complete. This number should monotonically increase as the operation progresses."]
        pub progress: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "region")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] The URL of the region where the operation resides. Only applicable when performing regional operations."]
        pub region: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] Server-defined URL for the resource."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] The time that this operation was started by the server. This value is in RFC3339 text format."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] The status of the operation, which can be one of the following: `PENDING`, `RUNNING`, or `DONE`."]
        pub status: ::std::option::Option<OperationStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statusMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] An optional textual description of the current status of the operation."]
        pub status_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] The unique target ID, which identifies a specific incarnation of the target resource."]
        pub target_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] The URL of the resource that the operation modifies. For operations related to creating a snapshot, this points to the persistent disk that the snapshot was created from."]
        pub target_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "user")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] User who requested the operation, for example: `user@example.com`."]
        pub user: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warnings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] If warning messages are generated during processing of the operation, this field will be populated."]
        pub warnings: ::std::option::Option<::std::vec::Vec<OperationWarnings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] The URL of the zone where the operation resides. Only applicable when performing per-zone operations."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl Operation {
        pub fn builder() -> OperationBuilder {
            OperationBuilder::default()
        }
    }
    mod operation_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"deploymentmanager#operation\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "[Output Only] If errors are generated during processing of the operation, this field will be populated."]
    pub struct OperationError {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] The array of errors encountered while processing this operation."]
        pub errors: ::std::option::Option<::std::vec::Vec<OperationErrorErrors>>,
    }
    impl OperationError {
        pub fn builder() -> OperationErrorBuilder {
            OperationErrorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OperationErrorErrors {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] The error type identifier for this error."]
        pub code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] Indicates the field in the request that caused the error. This property is optional."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] An optional, human-readable error message."]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl OperationErrorErrors {
        pub fn builder() -> OperationErrorErrorsBuilder {
            OperationErrorErrorsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "[Output Only] The status of the operation, which can be one of the following: `PENDING`, `RUNNING`, or `DONE`."]
    pub enum OperationStatusEnum {
        #[serde(rename = "PENDING")]
        #[doc = ""]
        Pending,
        #[serde(rename = "RUNNING")]
        #[doc = ""]
        Running,
        #[serde(rename = "DONE")]
        #[doc = ""]
        Done,
    }
    impl ::std::default::Default for OperationStatusEnum {
        fn default() -> Self {
            Self::Pending
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OperationWarnings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] A warning code, if applicable. For example, Compute Engine returns NO_RESULTS_ON_PAGE if there are no results in the response."]
        pub code: ::std::option::Option<OperationWarningsCodeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] Metadata about this warning in key: value format. For example: \"data\": [ { \"key\": \"scope\", \"value\": \"zones/us-east1-d\" } "]
        pub data: ::std::option::Option<::std::vec::Vec<OperationWarningsData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] A human-readable description of the warning code."]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl OperationWarnings {
        pub fn builder() -> OperationWarningsBuilder {
            OperationWarningsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "[Output Only] A warning code, if applicable. For example, Compute Engine returns NO_RESULTS_ON_PAGE if there are no results in the response."]
    pub enum OperationWarningsCodeEnum {
        #[serde(rename = "DEPRECATED_RESOURCE_USED")]
        #[doc = "A link to a deprecated resource was created."]
        DeprecatedResourceUsed,
        #[serde(rename = "NO_RESULTS_ON_PAGE")]
        #[doc = "No results are present on a particular list page."]
        NoResultsOnPage,
        #[serde(rename = "UNREACHABLE")]
        #[doc = "A given scope cannot be reached."]
        Unreachable,
        #[serde(rename = "NEXT_HOP_ADDRESS_NOT_ASSIGNED")]
        #[doc = "The route's nextHopIp address is not assigned to an instance on the network."]
        NextHopAddressNotAssigned,
        #[serde(rename = "NEXT_HOP_INSTANCE_NOT_FOUND")]
        #[doc = "The route's nextHopInstance URL refers to an instance that does not exist."]
        NextHopInstanceNotFound,
        #[serde(rename = "NEXT_HOP_INSTANCE_NOT_ON_NETWORK")]
        #[doc = "The route's nextHopInstance URL refers to an instance that is not on the same network as the route."]
        NextHopInstanceNotOnNetwork,
        #[serde(rename = "NEXT_HOP_CANNOT_IP_FORWARD")]
        #[doc = "The route's next hop instance cannot ip forward."]
        NextHopCannotIpForward,
        #[serde(rename = "NEXT_HOP_NOT_RUNNING")]
        #[doc = "The route's next hop instance does not have a status of RUNNING."]
        NextHopNotRunning,
        #[serde(rename = "INJECTED_KERNELS_DEPRECATED")]
        #[doc = "The operation involved use of an injected kernel, which is deprecated."]
        InjectedKernelsDeprecated,
        #[serde(rename = "REQUIRED_TOS_AGREEMENT")]
        #[doc = "The user attempted to use a resource that requires a TOS they have not accepted."]
        RequiredTosAgreement,
        #[serde(rename = "DISK_SIZE_LARGER_THAN_IMAGE_SIZE")]
        #[doc = "The user created a boot disk that is larger than image size."]
        DiskSizeLargerThanImageSize,
        #[serde(rename = "RESOURCE_NOT_DELETED")]
        #[doc = "One or more of the resources set to auto-delete could not be deleted because they were in use."]
        ResourceNotDeleted,
        #[serde(rename = "SINGLE_INSTANCE_PROPERTY_TEMPLATE")]
        #[doc = "Instance template used in instance group manager is valid as such, but its application does not make a lot of sense, because it allows only single instance in instance group."]
        SingleInstancePropertyTemplate,
        #[serde(rename = "NOT_CRITICAL_ERROR")]
        #[doc = "Error which is not critical. We decided to continue the process despite the mentioned error."]
        NotCriticalError,
        #[serde(rename = "CLEANUP_FAILED")]
        #[doc = "Warning about failed cleanup of transient changes made by a failed operation."]
        CleanupFailed,
        #[serde(rename = "FIELD_VALUE_OVERRIDEN")]
        #[doc = "Warning that value of a field has been overridden. Deprecated unused field."]
        FieldValueOverriden,
        #[serde(rename = "RESOURCE_IN_USE_BY_OTHER_RESOURCE_WARNING")]
        #[doc = "Warning that a resource is in use."]
        ResourceInUseByOtherResourceWarning,
        #[serde(rename = "MISSING_TYPE_DEPENDENCY")]
        #[doc = "A resource depends on a missing type"]
        MissingTypeDependency,
        #[serde(rename = "EXTERNAL_API_WARNING")]
        #[doc = "Warning that is present in an external api call"]
        ExternalApiWarning,
        #[serde(rename = "SCHEMA_VALIDATION_IGNORED")]
        #[doc = "When a resource schema validation is ignored."]
        SchemaValidationIgnored,
        #[serde(rename = "UNDECLARED_PROPERTIES")]
        #[doc = "When undeclared properties in the schema are present"]
        UndeclaredProperties,
        #[serde(rename = "EXPERIMENTAL_TYPE_USED")]
        #[doc = "When deploying and at least one of the resources has a type marked as experimental"]
        ExperimentalTypeUsed,
        #[serde(rename = "DEPRECATED_TYPE_USED")]
        #[doc = "When deploying and at least one of the resources has a type marked as deprecated"]
        DeprecatedTypeUsed,
        #[serde(rename = "PARTIAL_SUCCESS")]
        #[doc = "Success is reported, but some results may be missing due to errors"]
        PartialSuccess,
        #[serde(rename = "LARGE_DEPLOYMENT_WARNING")]
        #[doc = "When deploying a deployment with a exceedingly large number of resources"]
        LargeDeploymentWarning,
    }
    impl ::std::default::Default for OperationWarningsCodeEnum {
        fn default() -> Self {
            Self::DeprecatedResourceUsed
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OperationWarningsData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] A key that provides more detail on the warning being returned. For example, for warnings where there are no results in a list request for a particular zone, this key might be scope and the key value might be the zone name. Other examples might be a key indicating a deprecated resource and a suggested replacement, or a warning about invalid network settings (for example, if an instance attempts to perform IP forwarding but is not enabled for IP forwarding)."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] A warning data value corresponding to the key."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl OperationWarningsData {
        pub fn builder() -> OperationWarningsDataBuilder {
            OperationWarningsDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A response containing a partial list of operations and a page token used to build the next request if the request has been truncated."]
    pub struct OperationsListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A token used to continue a truncated list request."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Operations contained in this list response."]
        pub operations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Operation>>>,
    }
    impl OperationsListResponse {
        pub fn builder() -> OperationsListResponseBuilder {
            OperationsListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Options allows customized resource handling by Deployment Manager."]
    pub struct Options {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "asyncOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Options regarding how to thread async requests."]
        pub async_options: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AsyncOptions>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputMappings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The mappings that apply for requests."]
        pub input_mappings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<InputMapping>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "validationOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Options for how to validate and process properties on a resource."]
        pub validation_options: ::std::option::Option<::std::boxed::Box<ValidationOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "virtualProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional properties block described as a jsonSchema, these properties will never be part of the json payload, but they can be consumed by InputMappings, this must be a valid json schema draft-04. The properties specified here will be decouple in a different section. This schema will be merged to the schema validation, and properties here will be extracted From the payload and consumed explicitly by InputMappings. ex: field1: type: string field2: type: number"]
        pub virtual_properties: ::std::option::Option<::std::string::String>,
    }
    impl Options {
        pub fn builder() -> OptionsBuilder {
            OptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members` to a single `role`. Members can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { \"bindings\": [ { \"role\": \"roles/resourcemanager.organizationAdmin\", \"members\": [ \"user:mike@example.com\", \"group:admins@example.com\", \"domain:google.com\", \"serviceAccount:my-project-id@appspot.gserviceaccount.com\" ] }, { \"role\": \"roles/resourcemanager.organizationViewer\", \"members\": [ \"user:eve@example.com\" ], \"condition\": { \"title\": \"expirable access\", \"description\": \"Does not grant access after Sep 2020\", \"expression\": \"request.time < timestamp('2020-10-01T00:00:00.000Z')\", } } ], \"etag\": \"BwWWja0YfJA=\", \"version\": 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') - etag: BwWWja0YfJA= - version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/)."]
    pub struct Policy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "auditConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies cloud audit logging configuration for this policy."]
        pub audit_configs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuditConfig>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bindings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Associates a list of `members` to a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one member."]
        pub bindings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Binding>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "`etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        pub version: ::std::option::Option<::std::primitive::i64>,
    }
    impl Policy {
        pub fn builder() -> PolicyBuilder {
            PolicyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PollingOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diagnostics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An array of diagnostics to be collected by Deployment Manager, these diagnostics will be displayed to the user."]
        pub diagnostics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Diagnostic>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "failCondition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "JsonPath expression that determines if the request failed."]
        pub fail_condition: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "finishCondition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "JsonPath expression that determines if the request is completed."]
        pub finish_condition: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pollingLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "JsonPath expression that evaluates to string, it indicates where to poll."]
        pub polling_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "JsonPath expression, after polling is completed, indicates where to fetch the resource."]
        pub target_link: ::std::option::Option<::std::string::String>,
    }
    impl PollingOptions {
        pub fn builder() -> PollingOptionsBuilder {
            PollingOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Resource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accessControl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Access Control Policy set on this resource."]
        pub access_control: ::std::option::Option<::std::boxed::Box<ResourceAccessControl>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "finalProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The evaluated properties of the resource with references expanded. Returned as serialized YAML."]
        pub final_properties: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Creation timestamp in RFC3339 text format."]
        pub insert_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "manifest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. URL of the manifest representing the current configuration of this resource."]
        pub manifest: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of the resource as it appears in the YAML config."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The current properties of the resource before any references have been filled in. Returned as serialized YAML."]
        pub properties: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The type of the resource, for example `compute.v1.instance`, or `cloudfunctions.v1beta1.function`."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "update")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. If Deployment Manager is currently updating or previewing an update to this resource, the updated configuration appears here."]
        pub update: ::std::option::Option<::std::boxed::Box<ResourceUpdate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Update timestamp in RFC3339 text format."]
        pub update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The URL of the actual resource."]
        pub url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warnings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. If warning messages are generated during processing of this resource, this field will be populated."]
        pub warnings: ::std::option::Option<::std::vec::Vec<ResourceWarnings>>,
    }
    impl Resource {
        pub fn builder() -> ResourceBuilder {
            ResourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ResourceWarnings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] A warning code, if applicable. For example, Compute Engine returns NO_RESULTS_ON_PAGE if there are no results in the response."]
        pub code: ::std::option::Option<ResourceWarningsCodeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] Metadata about this warning in key: value format. For example: \"data\": [ { \"key\": \"scope\", \"value\": \"zones/us-east1-d\" } "]
        pub data: ::std::option::Option<::std::vec::Vec<ResourceWarningsData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] A human-readable description of the warning code."]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl ResourceWarnings {
        pub fn builder() -> ResourceWarningsBuilder {
            ResourceWarningsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "[Output Only] A warning code, if applicable. For example, Compute Engine returns NO_RESULTS_ON_PAGE if there are no results in the response."]
    pub enum ResourceWarningsCodeEnum {
        #[serde(rename = "DEPRECATED_RESOURCE_USED")]
        #[doc = "A link to a deprecated resource was created."]
        DeprecatedResourceUsed,
        #[serde(rename = "NO_RESULTS_ON_PAGE")]
        #[doc = "No results are present on a particular list page."]
        NoResultsOnPage,
        #[serde(rename = "UNREACHABLE")]
        #[doc = "A given scope cannot be reached."]
        Unreachable,
        #[serde(rename = "NEXT_HOP_ADDRESS_NOT_ASSIGNED")]
        #[doc = "The route's nextHopIp address is not assigned to an instance on the network."]
        NextHopAddressNotAssigned,
        #[serde(rename = "NEXT_HOP_INSTANCE_NOT_FOUND")]
        #[doc = "The route's nextHopInstance URL refers to an instance that does not exist."]
        NextHopInstanceNotFound,
        #[serde(rename = "NEXT_HOP_INSTANCE_NOT_ON_NETWORK")]
        #[doc = "The route's nextHopInstance URL refers to an instance that is not on the same network as the route."]
        NextHopInstanceNotOnNetwork,
        #[serde(rename = "NEXT_HOP_CANNOT_IP_FORWARD")]
        #[doc = "The route's next hop instance cannot ip forward."]
        NextHopCannotIpForward,
        #[serde(rename = "NEXT_HOP_NOT_RUNNING")]
        #[doc = "The route's next hop instance does not have a status of RUNNING."]
        NextHopNotRunning,
        #[serde(rename = "INJECTED_KERNELS_DEPRECATED")]
        #[doc = "The operation involved use of an injected kernel, which is deprecated."]
        InjectedKernelsDeprecated,
        #[serde(rename = "REQUIRED_TOS_AGREEMENT")]
        #[doc = "The user attempted to use a resource that requires a TOS they have not accepted."]
        RequiredTosAgreement,
        #[serde(rename = "DISK_SIZE_LARGER_THAN_IMAGE_SIZE")]
        #[doc = "The user created a boot disk that is larger than image size."]
        DiskSizeLargerThanImageSize,
        #[serde(rename = "RESOURCE_NOT_DELETED")]
        #[doc = "One or more of the resources set to auto-delete could not be deleted because they were in use."]
        ResourceNotDeleted,
        #[serde(rename = "SINGLE_INSTANCE_PROPERTY_TEMPLATE")]
        #[doc = "Instance template used in instance group manager is valid as such, but its application does not make a lot of sense, because it allows only single instance in instance group."]
        SingleInstancePropertyTemplate,
        #[serde(rename = "NOT_CRITICAL_ERROR")]
        #[doc = "Error which is not critical. We decided to continue the process despite the mentioned error."]
        NotCriticalError,
        #[serde(rename = "CLEANUP_FAILED")]
        #[doc = "Warning about failed cleanup of transient changes made by a failed operation."]
        CleanupFailed,
        #[serde(rename = "FIELD_VALUE_OVERRIDEN")]
        #[doc = "Warning that value of a field has been overridden. Deprecated unused field."]
        FieldValueOverriden,
        #[serde(rename = "RESOURCE_IN_USE_BY_OTHER_RESOURCE_WARNING")]
        #[doc = "Warning that a resource is in use."]
        ResourceInUseByOtherResourceWarning,
        #[serde(rename = "MISSING_TYPE_DEPENDENCY")]
        #[doc = "A resource depends on a missing type"]
        MissingTypeDependency,
        #[serde(rename = "EXTERNAL_API_WARNING")]
        #[doc = "Warning that is present in an external api call"]
        ExternalApiWarning,
        #[serde(rename = "SCHEMA_VALIDATION_IGNORED")]
        #[doc = "When a resource schema validation is ignored."]
        SchemaValidationIgnored,
        #[serde(rename = "UNDECLARED_PROPERTIES")]
        #[doc = "When undeclared properties in the schema are present"]
        UndeclaredProperties,
        #[serde(rename = "EXPERIMENTAL_TYPE_USED")]
        #[doc = "When deploying and at least one of the resources has a type marked as experimental"]
        ExperimentalTypeUsed,
        #[serde(rename = "DEPRECATED_TYPE_USED")]
        #[doc = "When deploying and at least one of the resources has a type marked as deprecated"]
        DeprecatedTypeUsed,
        #[serde(rename = "PARTIAL_SUCCESS")]
        #[doc = "Success is reported, but some results may be missing due to errors"]
        PartialSuccess,
        #[serde(rename = "LARGE_DEPLOYMENT_WARNING")]
        #[doc = "When deploying a deployment with a exceedingly large number of resources"]
        LargeDeploymentWarning,
    }
    impl ::std::default::Default for ResourceWarningsCodeEnum {
        fn default() -> Self {
            Self::DeprecatedResourceUsed
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ResourceWarningsData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] A key that provides more detail on the warning being returned. For example, for warnings where there are no results in a list request for a particular zone, this key might be scope and the key value might be the zone name. Other examples might be a key indicating a deprecated resource and a suggested replacement, or a warning about invalid network settings (for example, if an instance attempts to perform IP forwarding but is not enabled for IP forwarding)."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] A warning data value corresponding to the key."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl ResourceWarningsData {
        pub fn builder() -> ResourceWarningsDataBuilder {
            ResourceWarningsDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The access controls set on the resource."]
    pub struct ResourceAccessControl {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcpIamPolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The GCP IAM Policy to set on the resource."]
        pub gcp_iam_policy: ::std::option::Option<::std::string::String>,
    }
    impl ResourceAccessControl {
        pub fn builder() -> ResourceAccessControlBuilder {
            ResourceAccessControlBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ResourceUpdate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accessControl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Access Control Policy to set on this resource after updating the resource itself."]
        pub access_control: ::std::option::Option<::std::boxed::Box<ResourceAccessControl>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. If errors are generated during update of the resource, this field will be populated."]
        pub error: ::std::option::Option<ResourceUpdateError>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "finalProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The expanded properties of the resource with reference values expanded. Returned as serialized YAML."]
        pub final_properties: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The intent of the resource: `PREVIEW`, `UPDATE`, or `CANCEL`."]
        pub intent: ::std::option::Option<ResourceUpdateIntentEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "manifest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. URL of the manifest representing the update configuration of this resource."]
        pub manifest: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The set of updated properties for this resource, before references are expanded. Returned as serialized YAML."]
        pub properties: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The state of the resource."]
        pub state: ::std::option::Option<ResourceUpdateStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warnings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. If warning messages are generated during processing of this resource, this field will be populated."]
        pub warnings: ::std::option::Option<::std::vec::Vec<ResourceUpdateWarnings>>,
    }
    impl ResourceUpdate {
        pub fn builder() -> ResourceUpdateBuilder {
            ResourceUpdateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. If errors are generated during update of the resource, this field will be populated."]
    pub struct ResourceUpdateError {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] The array of errors encountered while processing this operation."]
        pub errors: ::std::option::Option<::std::vec::Vec<ResourceUpdateErrorErrors>>,
    }
    impl ResourceUpdateError {
        pub fn builder() -> ResourceUpdateErrorBuilder {
            ResourceUpdateErrorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ResourceUpdateErrorErrors {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] The error type identifier for this error."]
        pub code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] Indicates the field in the request that caused the error. This property is optional."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] An optional, human-readable error message."]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl ResourceUpdateErrorErrors {
        pub fn builder() -> ResourceUpdateErrorErrorsBuilder {
            ResourceUpdateErrorErrorsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The intent of the resource: `PREVIEW`, `UPDATE`, or `CANCEL`."]
    pub enum ResourceUpdateIntentEnum {
        #[serde(rename = "CREATE_OR_ACQUIRE")]
        #[doc = "The resource is scheduled to be created, or if it already exists, acquired."]
        CreateOrAcquire,
        #[serde(rename = "DELETE")]
        #[doc = "The resource is scheduled to be deleted."]
        Delete,
        #[serde(rename = "ACQUIRE")]
        #[doc = "The resource is scheduled to be acquired."]
        Acquire,
        #[serde(rename = "UPDATE")]
        #[doc = "The resource is scheduled to be updated via the UPDATE method."]
        Update,
        #[serde(rename = "ABANDON")]
        #[doc = "The resource is scheduled to be abandoned."]
        Abandon,
        #[serde(rename = "CREATE")]
        #[doc = "The resource is scheduled to be created."]
        Create,
    }
    impl ::std::default::Default for ResourceUpdateIntentEnum {
        fn default() -> Self {
            Self::CreateOrAcquire
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The state of the resource."]
    pub enum ResourceUpdateStateEnum {
        #[serde(rename = "PENDING")]
        #[doc = "There are changes pending for this resource."]
        Pending,
        #[serde(rename = "IN_PROGRESS")]
        #[doc = "The service is executing changes on the resource."]
        InProgress,
        #[serde(rename = "IN_PREVIEW")]
        #[doc = "The service is previewing changes on the resource."]
        InPreview,
        #[serde(rename = "FAILED")]
        #[doc = "The service has failed to change the resource."]
        Failed,
        #[serde(rename = "ABORTED")]
        #[doc = "The service has aborted trying to change the resource."]
        Aborted,
    }
    impl ::std::default::Default for ResourceUpdateStateEnum {
        fn default() -> Self {
            Self::Pending
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ResourceUpdateWarnings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] A warning code, if applicable. For example, Compute Engine returns NO_RESULTS_ON_PAGE if there are no results in the response."]
        pub code: ::std::option::Option<ResourceUpdateWarningsCodeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] Metadata about this warning in key: value format. For example: \"data\": [ { \"key\": \"scope\", \"value\": \"zones/us-east1-d\" } "]
        pub data: ::std::option::Option<::std::vec::Vec<ResourceUpdateWarningsData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] A human-readable description of the warning code."]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl ResourceUpdateWarnings {
        pub fn builder() -> ResourceUpdateWarningsBuilder {
            ResourceUpdateWarningsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "[Output Only] A warning code, if applicable. For example, Compute Engine returns NO_RESULTS_ON_PAGE if there are no results in the response."]
    pub enum ResourceUpdateWarningsCodeEnum {
        #[serde(rename = "DEPRECATED_RESOURCE_USED")]
        #[doc = "A link to a deprecated resource was created."]
        DeprecatedResourceUsed,
        #[serde(rename = "NO_RESULTS_ON_PAGE")]
        #[doc = "No results are present on a particular list page."]
        NoResultsOnPage,
        #[serde(rename = "UNREACHABLE")]
        #[doc = "A given scope cannot be reached."]
        Unreachable,
        #[serde(rename = "NEXT_HOP_ADDRESS_NOT_ASSIGNED")]
        #[doc = "The route's nextHopIp address is not assigned to an instance on the network."]
        NextHopAddressNotAssigned,
        #[serde(rename = "NEXT_HOP_INSTANCE_NOT_FOUND")]
        #[doc = "The route's nextHopInstance URL refers to an instance that does not exist."]
        NextHopInstanceNotFound,
        #[serde(rename = "NEXT_HOP_INSTANCE_NOT_ON_NETWORK")]
        #[doc = "The route's nextHopInstance URL refers to an instance that is not on the same network as the route."]
        NextHopInstanceNotOnNetwork,
        #[serde(rename = "NEXT_HOP_CANNOT_IP_FORWARD")]
        #[doc = "The route's next hop instance cannot ip forward."]
        NextHopCannotIpForward,
        #[serde(rename = "NEXT_HOP_NOT_RUNNING")]
        #[doc = "The route's next hop instance does not have a status of RUNNING."]
        NextHopNotRunning,
        #[serde(rename = "INJECTED_KERNELS_DEPRECATED")]
        #[doc = "The operation involved use of an injected kernel, which is deprecated."]
        InjectedKernelsDeprecated,
        #[serde(rename = "REQUIRED_TOS_AGREEMENT")]
        #[doc = "The user attempted to use a resource that requires a TOS they have not accepted."]
        RequiredTosAgreement,
        #[serde(rename = "DISK_SIZE_LARGER_THAN_IMAGE_SIZE")]
        #[doc = "The user created a boot disk that is larger than image size."]
        DiskSizeLargerThanImageSize,
        #[serde(rename = "RESOURCE_NOT_DELETED")]
        #[doc = "One or more of the resources set to auto-delete could not be deleted because they were in use."]
        ResourceNotDeleted,
        #[serde(rename = "SINGLE_INSTANCE_PROPERTY_TEMPLATE")]
        #[doc = "Instance template used in instance group manager is valid as such, but its application does not make a lot of sense, because it allows only single instance in instance group."]
        SingleInstancePropertyTemplate,
        #[serde(rename = "NOT_CRITICAL_ERROR")]
        #[doc = "Error which is not critical. We decided to continue the process despite the mentioned error."]
        NotCriticalError,
        #[serde(rename = "CLEANUP_FAILED")]
        #[doc = "Warning about failed cleanup of transient changes made by a failed operation."]
        CleanupFailed,
        #[serde(rename = "FIELD_VALUE_OVERRIDEN")]
        #[doc = "Warning that value of a field has been overridden. Deprecated unused field."]
        FieldValueOverriden,
        #[serde(rename = "RESOURCE_IN_USE_BY_OTHER_RESOURCE_WARNING")]
        #[doc = "Warning that a resource is in use."]
        ResourceInUseByOtherResourceWarning,
        #[serde(rename = "MISSING_TYPE_DEPENDENCY")]
        #[doc = "A resource depends on a missing type"]
        MissingTypeDependency,
        #[serde(rename = "EXTERNAL_API_WARNING")]
        #[doc = "Warning that is present in an external api call"]
        ExternalApiWarning,
        #[serde(rename = "SCHEMA_VALIDATION_IGNORED")]
        #[doc = "When a resource schema validation is ignored."]
        SchemaValidationIgnored,
        #[serde(rename = "UNDECLARED_PROPERTIES")]
        #[doc = "When undeclared properties in the schema are present"]
        UndeclaredProperties,
        #[serde(rename = "EXPERIMENTAL_TYPE_USED")]
        #[doc = "When deploying and at least one of the resources has a type marked as experimental"]
        ExperimentalTypeUsed,
        #[serde(rename = "DEPRECATED_TYPE_USED")]
        #[doc = "When deploying and at least one of the resources has a type marked as deprecated"]
        DeprecatedTypeUsed,
        #[serde(rename = "PARTIAL_SUCCESS")]
        #[doc = "Success is reported, but some results may be missing due to errors"]
        PartialSuccess,
        #[serde(rename = "LARGE_DEPLOYMENT_WARNING")]
        #[doc = "When deploying a deployment with a exceedingly large number of resources"]
        LargeDeploymentWarning,
    }
    impl ::std::default::Default for ResourceUpdateWarningsCodeEnum {
        fn default() -> Self {
            Self::DeprecatedResourceUsed
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ResourceUpdateWarningsData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] A key that provides more detail on the warning being returned. For example, for warnings where there are no results in a list request for a particular zone, this key might be scope and the key value might be the zone name. Other examples might be a key indicating a deprecated resource and a suggested replacement, or a warning about invalid network settings (for example, if an instance attempts to perform IP forwarding but is not enabled for IP forwarding)."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output Only] A warning data value corresponding to the key."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl ResourceUpdateWarningsData {
        pub fn builder() -> ResourceUpdateWarningsDataBuilder {
            ResourceUpdateWarningsDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A response containing a partial list of resources and a page token used to build the next request if the request has been truncated."]
    pub struct ResourcesListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token used to continue a truncated list request."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resources contained in this list response."]
        pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Resource>>>,
    }
    impl ResourcesListResponse {
        pub fn builder() -> ResourcesListResponseBuilder {
            ResourcesListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Service Account used as a credential."]
    pub struct ServiceAccount {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IAM service account email address like test@myproject.iam.gserviceaccount.com"]
        pub email: ::std::option::Option<::std::string::String>,
    }
    impl ServiceAccount {
        pub fn builder() -> ServiceAccountBuilder {
            ServiceAccountBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TargetConfiguration {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "config")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The configuration to use for this deployment."]
        pub config: ::std::option::Option<::std::boxed::Box<ConfigFile>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imports")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies any files to import for this configuration. This can be used to import templates or other files. For example, you might import a text file in order to use the file in a template."]
        pub imports: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ImportFile>>>,
    }
    impl TargetConfiguration {
        pub fn builder() -> TargetConfigurationBuilder {
            TargetConfigurationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Files that make up the template contents of a template type."]
    pub struct TemplateContents {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imports")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Import files referenced by the main template."]
        pub imports: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ImportFile>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "interpreter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Which interpreter (python or jinja) should be used during expansion."]
        pub interpreter: ::std::option::Option<TemplateContentsInterpreterEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mainTemplate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The filename of the mainTemplate"]
        pub main_template: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schema")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The contents of the template schema."]
        pub schema: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "template")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The contents of the main template file."]
        pub template: ::std::option::Option<::std::string::String>,
    }
    impl TemplateContents {
        pub fn builder() -> TemplateContentsBuilder {
            TemplateContentsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Which interpreter (python or jinja) should be used during expansion."]
    pub enum TemplateContentsInterpreterEnum {
        #[serde(rename = "UNKNOWN_INTERPRETER")]
        #[doc = ""]
        UnknownInterpreter,
        #[serde(rename = "PYTHON")]
        #[doc = ""]
        Python,
        #[serde(rename = "JINJA")]
        #[doc = ""]
        Jinja,
    }
    impl ::std::default::Default for TemplateContentsInterpreterEnum {
        fn default() -> Self {
            Self::UnknownInterpreter
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TestPermissionsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of permissions to check for the 'resource'. Permissions with wildcards (such as '*' or 'storage.*') are not allowed."]
        pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl TestPermissionsRequest {
        pub fn builder() -> TestPermissionsRequestBuilder {
            TestPermissionsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TestPermissionsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A subset of `TestPermissionsRequest.permissions` that the caller is allowed."]
        pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl TestPermissionsResponse {
        pub fn builder() -> TestPermissionsResponseBuilder {
            TestPermissionsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A resource type supported by Deployment Manager."]
    pub struct Type {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "base")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Base Type (configurable service) that backs this Type."]
        pub base: ::std::option::Option<::std::boxed::Box<BaseType>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional textual description of the resource; provided by the client when the resource is created."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Creation timestamp in RFC3339 text format."]
        pub insert_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`."]
        pub labels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TypeLabelEntry>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the type."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The Operation that most recently ran, or is currently running, on this type."]
        pub operation: ::std::option::Option<::std::boxed::Box<Operation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Server defined URL for the resource."]
        pub self_link: ::std::option::Option<::std::string::String>,
    }
    impl Type {
        pub fn builder() -> TypeBuilder {
            TypeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Type Information. Contains detailed information about a composite type, base type, or base type with specific collection."]
    pub struct TypeInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The description of the type."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "documentationLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For swagger 2.0 externalDocs field will be used. For swagger 1.2 this field will be empty."]
        pub documentation_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Type of the output. Always `deploymentManager#TypeInfo` for TypeInfo."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The base type or composite type name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schema")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For base types with a collection, we return a schema and documentation link For template types, we return only a schema"]
        pub schema: ::std::option::Option<::std::boxed::Box<TypeInfoSchemaInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Self link for the type provider."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title on the API descriptor URL provided."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl TypeInfo {
        pub fn builder() -> TypeInfoBuilder {
            TypeInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TypeInfoSchemaInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "input")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The properties that this composite type or base type collection accept as input, represented as a json blob, format is: JSON Schema Draft V4"]
        pub input: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "output")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The properties that this composite type or base type collection exposes as output, these properties can be used for references, represented as json blob, format is: JSON Schema Draft V4"]
        pub output: ::std::option::Option<::std::string::String>,
    }
    impl TypeInfoSchemaInfo {
        pub fn builder() -> TypeInfoSchemaInfoBuilder {
            TypeInfoSchemaInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Label object for Types"]
    pub struct TypeLabelEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Key of the label"]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value of the label"]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl TypeLabelEntry {
        pub fn builder() -> TypeLabelEntryBuilder {
            TypeLabelEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A type provider that describes a service-backed Type."]
    pub struct TypeProvider {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "collectionOverrides")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allows resource handling overrides for specific collections"]
        pub collection_overrides:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CollectionOverride>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "credential")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Credential used when interacting with this type."]
        pub credential: ::std::option::Option<::std::boxed::Box<Credential>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customCertificateAuthorityRoots")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of up to 2 custom certificate authority roots to use for TLS authentication when making calls on behalf of this type provider. If set, TLS authentication will exclusively use these roots instead of relying on publicly trusted certificate authorities when validating TLS certificate authenticity. The certificates must be in base64-encoded PEM format. The maximum size of each certificate must not exceed 10KB."]
        pub custom_certificate_authority_roots:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional textual description of the resource; provided by the client when the resource is created."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "descriptorUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Descriptor Url for the this type provider."]
        pub descriptor_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Unique identifier for the resource defined by the server."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Creation timestamp in RFC3339 text format."]
        pub insert_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`"]
        pub labels:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TypeProviderLabelEntry>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The Operation that most recently ran, or is currently running, on this type provider."]
        pub operation: ::std::option::Option<::std::boxed::Box<Operation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "options")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Options to apply when handling any resources in this service."]
        pub options: ::std::option::Option<::std::boxed::Box<Options>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Self link for the type provider."]
        pub self_link: ::std::option::Option<::std::string::String>,
    }
    impl TypeProvider {
        pub fn builder() -> TypeProviderBuilder {
            TypeProviderBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Label object for TypeProviders"]
    pub struct TypeProviderLabelEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Key of the label"]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value of the label"]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl TypeProviderLabelEntry {
        pub fn builder() -> TypeProviderLabelEntryBuilder {
            TypeProviderLabelEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A response that returns all Type Providers supported by Deployment Manager"]
    pub struct TypeProvidersListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token used to continue a truncated list request."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "typeProviders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A list of resource type providers supported by Deployment Manager."]
        pub type_providers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TypeProvider>>>,
    }
    impl TypeProvidersListResponse {
        pub fn builder() -> TypeProvidersListResponseBuilder {
            TypeProvidersListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TypeProvidersListTypesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token used to continue a truncated list request."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "types")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A list of resource type info."]
        pub types: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TypeInfo>>>,
    }
    impl TypeProvidersListTypesResponse {
        pub fn builder() -> TypeProvidersListTypesResponseBuilder {
            TypeProvidersListTypesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A response that returns all Types supported by Deployment Manager"]
    pub struct TypesListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token used to continue a truncated list request."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "types")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A list of resource types supported by Deployment Manager."]
        pub types: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Type>>>,
    }
    impl TypesListResponse {
        pub fn builder() -> TypesListResponseBuilder {
            TypesListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Options for how to validate and process properties on a resource."]
    pub struct ValidationOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schemaValidation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Customize how deployment manager will validate the resource against schema errors."]
        pub schema_validation: ::std::option::Option<ValidationOptionsSchemaValidationEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "undeclaredProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specify what to do with extra properties when executing a request."]
        pub undeclared_properties: ::std::option::Option<ValidationOptionsUndeclaredPropertiesEnum>,
    }
    impl ValidationOptions {
        pub fn builder() -> ValidationOptionsBuilder {
            ValidationOptionsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Customize how deployment manager will validate the resource against schema errors."]
    pub enum ValidationOptionsSchemaValidationEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = ""]
        Unknown,
        #[serde(rename = "IGNORE")]
        #[doc = "Ignore schema failures."]
        Ignore,
        #[serde(rename = "IGNORE_WITH_WARNINGS")]
        #[doc = "Ignore schema failures but display them as warnings."]
        IgnoreWithWarnings,
        #[serde(rename = "FAIL")]
        #[doc = "Fail the resource if the schema is not valid, this is the default behavior."]
        Fail,
    }
    impl ::std::default::Default for ValidationOptionsSchemaValidationEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specify what to do with extra properties when executing a request."]
    pub enum ValidationOptionsUndeclaredPropertiesEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = ""]
        Unknown,
        #[serde(rename = "INCLUDE")]
        #[doc = "Always include even if not present on discovery doc."]
        Include,
        #[serde(rename = "IGNORE")]
        #[doc = "Always ignore if not present on discovery doc."]
        Ignore,
        #[serde(rename = "INCLUDE_WITH_WARNINGS")]
        #[doc = "Include on request, but emit a warning."]
        IncludeWithWarnings,
        #[serde(rename = "IGNORE_WITH_WARNINGS")]
        #[doc = "Ignore properties, but emit a warning."]
        IgnoreWithWarnings,
        #[serde(rename = "FAIL")]
        #[doc = "Always fail if undeclared properties are present."]
        Fail,
    }
    impl ::std::default::Default for ValidationOptionsUndeclaredPropertiesEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
}
