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
        serde_json::from_str(&"json").unwrap()
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
    pub mod changes {
        pub mod methods {
            pub mod create {
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
                    #[serde(rename = "clientOperationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                    pub client_operation_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod get {
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
                    #[serde(rename = "clientOperationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                    pub client_operation_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. Maximum number of results to be returned. If unspecified, the server decides how many results to return."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. A tag returned by a previous list request that was truncated. Use this parameter to continue a previous list request."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: sort_by () }",
                        setter(into)
                    )]
                    #[serde(rename = "sortBy")]
                    #[serde(default = "query_parameters_defaults :: sort_by")]
                    #[doc = "Sorting criterion. The only supported value is change sequence."]
                    pub sort_by: QueryParametersSortByEnum,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sortOrder")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Sorting order direction: 'ascending' or 'descending'."]
                    pub sort_order: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn sort_by() -> super::QueryParametersSortByEnum {
                        serde_json::from_str(&"changeSequence").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Sorting criterion. The only supported value is change sequence."]
                pub enum QueryParametersSortByEnum {
                    #[serde(rename = "changeSequence")]
                    #[doc = ""]
                    ChangeSequence,
                }
                impl ::std::default::Default for QueryParametersSortByEnum {
                    fn default() -> Self {
                        Self::ChangeSequence
                    }
                }
            }
        }
    }
    pub mod dns_keys {
        pub mod methods {
            pub mod get {
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
                    #[serde(rename = "clientOperationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                    pub client_operation_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "digestType")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "An optional comma-separated list of digest types to compute and display for key signing keys. If omitted, the recommended digest type is computed and displayed."]
                    pub digest_type: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[serde(rename = "digestType")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "An optional comma-separated list of digest types to compute and display for key signing keys. If omitted, the recommended digest type is computed and displayed."]
                    pub digest_type: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. Maximum number of results to be returned. If unspecified, the server decides how many results to return."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. A tag returned by a previous list request that was truncated. Use this parameter to continue a previous list request."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod managed_zone_operations {
        pub mod methods {
            pub mod get {
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
                    #[serde(rename = "clientOperationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                    pub client_operation_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. Maximum number of results to be returned. If unspecified, the server decides how many results to return."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. A tag returned by a previous list request that was truncated. Use this parameter to continue a previous list request."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: sort_by () }",
                        setter(into)
                    )]
                    #[serde(rename = "sortBy")]
                    #[serde(default = "query_parameters_defaults :: sort_by")]
                    #[doc = "Sorting criterion. The only supported values are START_TIME and ID."]
                    pub sort_by: QueryParametersSortByEnum,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn sort_by() -> super::QueryParametersSortByEnum {
                        serde_json::from_str(&"startTime").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Sorting criterion. The only supported values are START_TIME and ID."]
                pub enum QueryParametersSortByEnum {
                    #[serde(rename = "startTime")]
                    #[doc = ""]
                    StartTime,
                    #[serde(rename = "id")]
                    #[doc = ""]
                    Id,
                }
                impl ::std::default::Default for QueryParametersSortByEnum {
                    fn default() -> Self {
                        Self::StartTime
                    }
                }
            }
        }
    }
    pub mod managed_zones {
        pub mod methods {
            pub mod create {
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
                    #[serde(rename = "clientOperationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                    pub client_operation_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientOperationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                    pub client_operation_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod get {
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
                    #[serde(rename = "clientOperationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                    pub client_operation_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[serde(rename = "dnsName")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restricts the list to return only zones with this domain name."]
                    pub dns_name: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. Maximum number of results to be returned. If unspecified, the server decides how many results to return."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. A tag returned by a previous list request that was truncated. Use this parameter to continue a previous list request."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientOperationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                    pub client_operation_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientOperationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                    pub client_operation_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod policies {
        pub mod methods {
            pub mod create {
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
                    #[serde(rename = "clientOperationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                    pub client_operation_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientOperationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                    pub client_operation_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod get {
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
                    #[serde(rename = "clientOperationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                    pub client_operation_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. Maximum number of results to be returned. If unspecified, the server decides how many results to return."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. A tag returned by a previous list request that was truncated. Use this parameter to continue a previous list request."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientOperationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                    pub client_operation_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientOperationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                    pub client_operation_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod projects {
        pub mod methods {
            pub mod get {
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
                    #[serde(rename = "clientOperationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                    pub client_operation_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
        pub mod resources {
            pub mod managed_zones {
                pub mod resources {
                    pub mod rrsets {
                        pub mod methods {
                            pub mod create {
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
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "clientOperationId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                                    pub client_operation_id:
                                        ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
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
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "clientOperationId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                                    pub client_operation_id:
                                        ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                            pub mod get {
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
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "clientOperationId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                                    pub client_operation_id:
                                        ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
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
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "clientOperationId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                                    pub client_operation_id:
                                        ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    pub mod resource_record_sets {
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
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. Maximum number of results to be returned. If unspecified, the server decides how many results to return."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "name")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restricts the list to return only records with this fully qualified domain name."]
                    pub name: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. A tag returned by a previous list request that was truncated. Use this parameter to continue a previous list request."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "type")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restricts the list to return only records of this type. If present, the \"name\" parameter must also be present."]
                    pub _type: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod response_policies {
        pub mod methods {
            pub mod create {
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
                    #[serde(rename = "clientOperationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                    pub client_operation_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientOperationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                    pub client_operation_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod get {
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
                    #[serde(rename = "clientOperationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                    pub client_operation_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. Maximum number of results to be returned. If unspecified, the server decides how many results to return."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. A tag returned by a previous list request that was truncated. Use this parameter to continue a previous list request."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientOperationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                    pub client_operation_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientOperationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                    pub client_operation_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod response_policy_rules {
        pub mod methods {
            pub mod create {
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
                    #[serde(rename = "clientOperationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                    pub client_operation_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientOperationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                    pub client_operation_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod get {
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
                    #[serde(rename = "clientOperationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                    pub client_operation_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. Maximum number of results to be returned. If unspecified, the server decides how many results to return."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. A tag returned by a previous list request that was truncated. Use this parameter to continue a previous list request."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientOperationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                    pub client_operation_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientOperationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "For mutating operation requests only. An optional identifier specified by the client. Must be unique for operation resources in the Operations collection."]
                    pub client_operation_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
    #[doc = "A Change represents a set of ResourceRecordSet additions and deletions applied atomically to a ManagedZone. ResourceRecordSets within a ManagedZone are modified by creating a new Change element in the Changes collection. In turn the Changes collection also records the past modifications to the ResourceRecordSets in a ManagedZone. The current state of the ManagedZone is the sum effect of applying all Change elements in the Changes collection in sequence."]
    pub struct Change {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Which ResourceRecordSets to add?"]
        pub additions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ResourceRecordSet>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deletions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Which ResourceRecordSets to remove? Must match existing data exactly."]
        pub deletions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ResourceRecordSet>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier for the resource; defined by the server (output only)."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isServing")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the DNS queries for the zone will be served."]
        pub is_serving: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ change_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "change_defaults :: kind")]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time that this operation was started by the server (output only). This is in RFC3339 text format."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of the operation (output only). A status of \"done\" means that the request to update the authoritative servers has been sent but the servers might not be updated yet."]
        pub status: ::std::option::Option<ChangeStatusEnum>,
    }
    impl Change {
        pub fn builder() -> ChangeBuilder {
            ChangeBuilder::default()
        }
    }
    mod change_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#change")
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Status of the operation (output only). A status of \"done\" means that the request to update the authoritative servers has been sent but the servers might not be updated yet."]
    pub enum ChangeStatusEnum {
        #[serde(rename = "pending")]
        #[doc = ""]
        Pending,
        #[serde(rename = "done")]
        #[doc = ""]
        Done,
    }
    impl ::std::default::Default for ChangeStatusEnum {
        fn default() -> Self {
            Self::Pending
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response to a request to enumerate Changes to a ResourceRecordSets collection."]
    pub struct ChangesListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "changes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The requested changes."]
        pub changes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Change>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "header")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub header: ::std::option::Option<::std::boxed::Box<ResponseHeader>>,
        #[builder(
            default = "{ changes_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "changes_list_response_defaults :: kind")]
        #[doc = "Type of resource."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The presence of this field indicates that there exist more results following your last page of results in pagination order. To fetch them, make another list request using this value as your pagination token. In this way you can retrieve the complete contents of even very large collections one page at a time. However, if the contents of the collection change between the first and last paginated list request, the set of all elements returned are an inconsistent view of the collection. There is no way to retrieve a \"snapshot\" of collections larger than the maximum page size."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ChangesListResponse {
        pub fn builder() -> ChangesListResponseBuilder {
            ChangesListResponseBuilder::default()
        }
    }
    mod changes_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#changesListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A DNSSEC key pair."]
    pub struct DnsKey {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "algorithm")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "String mnemonic specifying the DNSSEC algorithm of this key. Immutable after creation time."]
        pub algorithm: ::std::option::Option<DnsKeyAlgorithmEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time that this resource was created in the control plane. This is in RFC3339 text format. Output only."]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the resource's function."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "digests")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cryptographic hashes of the DNSKEY resource record associated with this DnsKey. These digests are needed to construct a DS record that points at this DNS key. Output only."]
        pub digests: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DnsKeyDigest>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier for the resource; defined by the server (output only)."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isActive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Active keys are used to sign subsequent changes to the ManagedZone. Inactive keys will still be present as DNSKEY Resource Records for the use of resolvers validating existing signatures."]
        pub is_active: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyLength")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Length of the key in bits. Specified at creation time, then immutable."]
        pub key_length: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyTag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The key tag is a non-cryptographic hash of the a DNSKEY resource record associated with this DnsKey. The key tag can be used to identify a DNSKEY more quickly (but it is not a unique identifier). In particular, the key tag is used in a parent zone's DS record to point at the DNSKEY in this child ManagedZone. The key tag is a number in the range [0, 65535] and the algorithm to calculate it is specified in RFC4034 Appendix B. Output only."]
        pub key_tag: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ dns_key_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "dns_key_defaults :: kind")]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publicKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Base64 encoded public half of this key. Output only."]
        pub public_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "One of \"KEY_SIGNING\" or \"ZONE_SIGNING\". Keys of type KEY_SIGNING have the Secure Entry Point flag set and, when active, are used to sign only resource record sets of type DNSKEY. Otherwise, the Secure Entry Point flag is cleared and this key is used to sign only resource record sets of other types. Immutable after creation time."]
        pub _type: ::std::option::Option<DnsKeyTypeEnum>,
    }
    impl DnsKey {
        pub fn builder() -> DnsKeyBuilder {
            DnsKeyBuilder::default()
        }
    }
    mod dns_key_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#dnsKey")
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "String mnemonic specifying the DNSSEC algorithm of this key. Immutable after creation time."]
    pub enum DnsKeyAlgorithmEnum {
        #[serde(rename = "rsasha1")]
        #[doc = ""]
        Rsasha1,
        #[serde(rename = "rsasha256")]
        #[doc = ""]
        Rsasha256,
        #[serde(rename = "rsasha512")]
        #[doc = ""]
        Rsasha512,
        #[serde(rename = "ecdsap256sha256")]
        #[doc = ""]
        Ecdsap256sha256,
        #[serde(rename = "ecdsap384sha384")]
        #[doc = ""]
        Ecdsap384sha384,
    }
    impl ::std::default::Default for DnsKeyAlgorithmEnum {
        fn default() -> Self {
            Self::Rsasha1
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "One of \"KEY_SIGNING\" or \"ZONE_SIGNING\". Keys of type KEY_SIGNING have the Secure Entry Point flag set and, when active, are used to sign only resource record sets of type DNSKEY. Otherwise, the Secure Entry Point flag is cleared and this key is used to sign only resource record sets of other types. Immutable after creation time."]
    pub enum DnsKeyTypeEnum {
        #[serde(rename = "keySigning")]
        #[doc = ""]
        KeySigning,
        #[serde(rename = "zoneSigning")]
        #[doc = ""]
        ZoneSigning,
    }
    impl ::std::default::Default for DnsKeyTypeEnum {
        fn default() -> Self {
            Self::KeySigning
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DnsKeyDigest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "digest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The base-16 encoded bytes of this digest. Suitable for use in a DS resource record."]
        pub digest: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the algorithm used to calculate this digest."]
        pub _type: ::std::option::Option<DnsKeyDigestTypeEnum>,
    }
    impl DnsKeyDigest {
        pub fn builder() -> DnsKeyDigestBuilder {
            DnsKeyDigestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies the algorithm used to calculate this digest."]
    pub enum DnsKeyDigestTypeEnum {
        #[serde(rename = "sha1")]
        #[doc = ""]
        Sha1,
        #[serde(rename = "sha256")]
        #[doc = ""]
        Sha256,
        #[serde(rename = "sha384")]
        #[doc = ""]
        Sha384,
    }
    impl ::std::default::Default for DnsKeyDigestTypeEnum {
        fn default() -> Self {
            Self::Sha1
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parameters for DnsKey key generation. Used for generating initial keys for a new ManagedZone and as default when adding a new DnsKey."]
    pub struct DnsKeySpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "algorithm")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "String mnemonic specifying the DNSSEC algorithm of this key."]
        pub algorithm: ::std::option::Option<DnsKeySpecAlgorithmEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyLength")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Length of the keys in bits."]
        pub key_length: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies whether this is a key signing key (KSK) or a zone signing key (ZSK). Key signing keys have the Secure Entry Point flag set and, when active, are only used to sign resource record sets of type DNSKEY. Zone signing keys do not have the Secure Entry Point flag set and are used to sign all other types of resource record sets."]
        pub key_type: ::std::option::Option<DnsKeySpecKeyTypeEnum>,
        #[builder(default = "{ dns_key_spec_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "dns_key_spec_defaults :: kind")]
        pub kind: ::std::string::String,
    }
    impl DnsKeySpec {
        pub fn builder() -> DnsKeySpecBuilder {
            DnsKeySpecBuilder::default()
        }
    }
    mod dns_key_spec_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#dnsKeySpec")
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "String mnemonic specifying the DNSSEC algorithm of this key."]
    pub enum DnsKeySpecAlgorithmEnum {
        #[serde(rename = "rsasha1")]
        #[doc = ""]
        Rsasha1,
        #[serde(rename = "rsasha256")]
        #[doc = ""]
        Rsasha256,
        #[serde(rename = "rsasha512")]
        #[doc = ""]
        Rsasha512,
        #[serde(rename = "ecdsap256sha256")]
        #[doc = ""]
        Ecdsap256sha256,
        #[serde(rename = "ecdsap384sha384")]
        #[doc = ""]
        Ecdsap384sha384,
    }
    impl ::std::default::Default for DnsKeySpecAlgorithmEnum {
        fn default() -> Self {
            Self::Rsasha1
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies whether this is a key signing key (KSK) or a zone signing key (ZSK). Key signing keys have the Secure Entry Point flag set and, when active, are only used to sign resource record sets of type DNSKEY. Zone signing keys do not have the Secure Entry Point flag set and are used to sign all other types of resource record sets."]
    pub enum DnsKeySpecKeyTypeEnum {
        #[serde(rename = "keySigning")]
        #[doc = ""]
        KeySigning,
        #[serde(rename = "zoneSigning")]
        #[doc = ""]
        ZoneSigning,
    }
    impl ::std::default::Default for DnsKeySpecKeyTypeEnum {
        fn default() -> Self {
            Self::KeySigning
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response to a request to enumerate DnsKeys in a ManagedZone."]
    pub struct DnsKeysListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dnsKeys")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The requested resources."]
        pub dns_keys: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DnsKey>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "header")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub header: ::std::option::Option<::std::boxed::Box<ResponseHeader>>,
        #[builder(
            default = "{ dns_keys_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "dns_keys_list_response_defaults :: kind")]
        #[doc = "Type of resource."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The presence of this field indicates that there exist more results following your last page of results in pagination order. To fetch them, make another list request using this value as your pagination token. In this way you can retrieve the complete contents of even very large collections one page at a time. However, if the contents of the collection change between the first and last paginated list request, the set of all elements returned are an inconsistent view of the collection. There is no way to retrieve a \"snapshot\" of collections larger than the maximum page size."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl DnsKeysListResponse {
        pub fn builder() -> DnsKeysListResponseBuilder {
            DnsKeysListResponseBuilder::default()
        }
    }
    mod dns_keys_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#dnsKeysListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A zone is a subtree of the DNS namespace under one administrative responsibility. A ManagedZone is a resource that represents a DNS zone hosted by the Cloud DNS service."]
    pub struct ManagedZone {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time that this resource was created on the server. This is in RFC3339 text format. Output only."]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the managed zone's function."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dnsName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The DNS name of this managed zone, for instance \"example.com.\"."]
        pub dns_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dnssecConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DNSSEC configuration."]
        pub dnssec_config: ::std::option::Option<::std::boxed::Box<ManagedZoneDnsSecConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "forwardingConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The presence for this field indicates that outbound forwarding is enabled for this zone. The value of this field contains the set of destinations to forward to."]
        pub forwarding_config:
            ::std::option::Option<::std::boxed::Box<ManagedZoneForwardingConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier for the resource; defined by the server (output only)"]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ managed_zone_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "managed_zone_defaults :: kind")]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User labels."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User assigned name for this resource. Must be unique within the project. The name must be 1-63 characters long, must begin with a letter, end with a letter or digit, and only contain lowercase letters, digits or dashes."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nameServerSet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optionally specifies the NameServerSet for this ManagedZone. A NameServerSet is a set of DNS name servers that all host the same ManagedZones. Most users will leave this field unset. If you need to use this field, please reach out to your account team."]
        pub name_server_set: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nameServers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Delegate your managed_zone to these virtual name servers; defined by the server (output only)"]
        pub name_servers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "peeringConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The presence of this field indicates that DNS Peering is enabled for this zone. The value of this field contains the network to peer with."]
        pub peering_config: ::std::option::Option<::std::boxed::Box<ManagedZonePeeringConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "privateVisibilityConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For privately visible zones, the set of Virtual Private Cloud resources that the zone is visible from."]
        pub private_visibility_config:
            ::std::option::Option<::std::boxed::Box<ManagedZonePrivateVisibilityConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reverseLookupConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The presence of this field indicates that this is a managed reverse lookup zone and Cloud DNS resolves reverse lookup queries using automatically configured records for VPC resources. This only applies to networks listed under private_visibility_config."]
        pub reverse_lookup_config:
            ::std::option::Option<::std::boxed::Box<ManagedZoneReverseLookupConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceDirectoryConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field links to the associated service directory namespace. This field should not be set for public zones or forwarding zones."]
        pub service_directory_config:
            ::std::option::Option<::std::boxed::Box<ManagedZoneServiceDirectoryConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visibility")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The zone's visibility: public zones are exposed to the Internet, while private zones are visible only to Virtual Private Cloud resources."]
        pub visibility: ::std::option::Option<ManagedZoneVisibilityEnum>,
    }
    impl ManagedZone {
        pub fn builder() -> ManagedZoneBuilder {
            ManagedZoneBuilder::default()
        }
    }
    mod managed_zone_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#managedZone")
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The zone's visibility: public zones are exposed to the Internet, while private zones are visible only to Virtual Private Cloud resources."]
    pub enum ManagedZoneVisibilityEnum {
        #[serde(rename = "public")]
        #[doc = ""]
        Public,
        #[serde(rename = "private")]
        #[doc = ""]
        Private,
    }
    impl ::std::default::Default for ManagedZoneVisibilityEnum {
        fn default() -> Self {
            Self::Public
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ManagedZoneDnsSecConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultKeySpecs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies parameters for generating initial DnsKeys for this ManagedZone. Can only be changed while the state is OFF."]
        pub default_key_specs:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DnsKeySpec>>>,
        #[builder(
            default = "{ managed_zone_dns_sec_config_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "managed_zone_dns_sec_config_defaults :: kind")]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nonExistence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the mechanism for authenticated denial-of-existence responses. Can only be changed while the state is OFF."]
        pub non_existence: ::std::option::Option<ManagedZoneDnsSecConfigNonExistenceEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies whether DNSSEC is enabled, and what mode it is in."]
        pub state: ::std::option::Option<ManagedZoneDnsSecConfigStateEnum>,
    }
    impl ManagedZoneDnsSecConfig {
        pub fn builder() -> ManagedZoneDnsSecConfigBuilder {
            ManagedZoneDnsSecConfigBuilder::default()
        }
    }
    mod managed_zone_dns_sec_config_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#managedZoneDnsSecConfig")
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies the mechanism for authenticated denial-of-existence responses. Can only be changed while the state is OFF."]
    pub enum ManagedZoneDnsSecConfigNonExistenceEnum {
        #[serde(rename = "nsec")]
        #[doc = ""]
        Nsec,
        #[serde(rename = "nsec3")]
        #[doc = ""]
        Nsec3,
    }
    impl ::std::default::Default for ManagedZoneDnsSecConfigNonExistenceEnum {
        fn default() -> Self {
            Self::Nsec
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies whether DNSSEC is enabled, and what mode it is in."]
    pub enum ManagedZoneDnsSecConfigStateEnum {
        #[serde(rename = "off")]
        #[doc = "DNSSEC is disabled; the zone is not signed."]
        Off,
        #[serde(rename = "on")]
        #[doc = "DNSSEC is enabled; the zone is signed and fully managed."]
        On,
        #[serde(rename = "transfer")]
        #[doc = "DNSSEC is enabled, but in a \"transfer\" mode."]
        Transfer,
    }
    impl ::std::default::Default for ManagedZoneDnsSecConfigStateEnum {
        fn default() -> Self {
            Self::Off
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ManagedZoneForwardingConfig {
        #[builder(
            default = "{ managed_zone_forwarding_config_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "managed_zone_forwarding_config_defaults :: kind")]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetNameServers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of target name servers to forward to. Cloud DNS selects the best available name server if more than one target is given."]
        pub target_name_servers: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<ManagedZoneForwardingConfigNameServerTarget>>,
        >,
    }
    impl ManagedZoneForwardingConfig {
        pub fn builder() -> ManagedZoneForwardingConfigBuilder {
            ManagedZoneForwardingConfigBuilder::default()
        }
    }
    mod managed_zone_forwarding_config_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#managedZoneForwardingConfig")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ManagedZoneForwardingConfigNameServerTarget {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "forwardingPath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Forwarding path for this NameServerTarget. If unset or set to DEFAULT, Cloud DNS makes forwarding decisions based on address ranges; that is, RFC1918 addresses go to the VPC, non-RFC1918 addresses go to the internet. When set to PRIVATE, Cloud DNS always sends queries through VPC for this target."]
        pub forwarding_path:
            ::std::option::Option<ManagedZoneForwardingConfigNameServerTargetForwardingPathEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipv4Address")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "IPv4 address of a target name server."]
        pub ipv4_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipv6Address")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "IPv6 address of a target name server. Does not accept both fields (ipv4 & ipv6) being populated."]
        pub ipv6_address: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ managed_zone_forwarding_config_name_server_target_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "managed_zone_forwarding_config_name_server_target_defaults :: kind")]
        pub kind: ::std::string::String,
    }
    impl ManagedZoneForwardingConfigNameServerTarget {
        pub fn builder() -> ManagedZoneForwardingConfigNameServerTargetBuilder {
            ManagedZoneForwardingConfigNameServerTargetBuilder::default()
        }
    }
    mod managed_zone_forwarding_config_name_server_target_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#managedZoneForwardingConfigNameServerTarget")
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Forwarding path for this NameServerTarget. If unset or set to DEFAULT, Cloud DNS makes forwarding decisions based on address ranges; that is, RFC1918 addresses go to the VPC, non-RFC1918 addresses go to the internet. When set to PRIVATE, Cloud DNS always sends queries through VPC for this target."]
    pub enum ManagedZoneForwardingConfigNameServerTargetForwardingPathEnum {
        #[serde(rename = "default")]
        #[doc = "Cloud DNS makes forwarding decisions based on address ranges; that is, RFC1918 addresses forward to the target through the VPC and non-RFC1918 addresses forward to the target through the internet"]
        Default,
        #[serde(rename = "private")]
        #[doc = "Cloud DNS always forwards to this target through the VPC."]
        Private,
    }
    impl ::std::default::Default for ManagedZoneForwardingConfigNameServerTargetForwardingPathEnum {
        fn default() -> Self {
            Self::Default
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ManagedZoneOperationsListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "header")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub header: ::std::option::Option<::std::boxed::Box<ResponseHeader>>,
        #[builder(
            default = "{ managed_zone_operations_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "managed_zone_operations_list_response_defaults :: kind")]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The presence of this field indicates that there exist more results following your last page of results in pagination order. To fetch them, make another list request using this value as your page token. In this way you can retrieve the complete contents of even very large collections one page at a time. However, if the contents of the collection change between the first and last paginated list request, the set of all elements returned are an inconsistent view of the collection. There is no way to retrieve a consistent snapshot of a collection larger than the maximum page size."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The operation resources."]
        pub operations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Operation>>>,
    }
    impl ManagedZoneOperationsListResponse {
        pub fn builder() -> ManagedZoneOperationsListResponseBuilder {
            ManagedZoneOperationsListResponseBuilder::default()
        }
    }
    mod managed_zone_operations_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#managedZoneOperationsListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ManagedZonePeeringConfig {
        #[builder(
            default = "{ managed_zone_peering_config_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "managed_zone_peering_config_defaults :: kind")]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetNetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The network with which to peer."]
        pub target_network:
            ::std::option::Option<::std::boxed::Box<ManagedZonePeeringConfigTargetNetwork>>,
    }
    impl ManagedZonePeeringConfig {
        pub fn builder() -> ManagedZonePeeringConfigBuilder {
            ManagedZonePeeringConfigBuilder::default()
        }
    }
    mod managed_zone_peering_config_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#managedZonePeeringConfig")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ManagedZonePeeringConfigTargetNetwork {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deactivateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the zone was deactivated, in RFC 3339 date-time format. An empty string indicates that the peering connection is active. The producer network can deactivate a zone. The zone is automatically deactivated if the producer network that the zone targeted is deleted. Output only."]
        pub deactivate_time: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ managed_zone_peering_config_target_network_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "managed_zone_peering_config_target_network_defaults :: kind")]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "networkUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fully qualified URL of the VPC network to forward queries to. This should be formatted like https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}"]
        pub network_url: ::std::option::Option<::std::string::String>,
    }
    impl ManagedZonePeeringConfigTargetNetwork {
        pub fn builder() -> ManagedZonePeeringConfigTargetNetworkBuilder {
            ManagedZonePeeringConfigTargetNetworkBuilder::default()
        }
    }
    mod managed_zone_peering_config_target_network_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#managedZonePeeringConfigTargetNetwork")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ManagedZonePrivateVisibilityConfig {
        #[builder(
            default = "{ managed_zone_private_visibility_config_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "managed_zone_private_visibility_config_defaults :: kind")]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "networks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of VPC networks that can see this zone."]
        pub networks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<ManagedZonePrivateVisibilityConfigNetwork>>,
        >,
    }
    impl ManagedZonePrivateVisibilityConfig {
        pub fn builder() -> ManagedZonePrivateVisibilityConfigBuilder {
            ManagedZonePrivateVisibilityConfigBuilder::default()
        }
    }
    mod managed_zone_private_visibility_config_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#managedZonePrivateVisibilityConfig")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ManagedZonePrivateVisibilityConfigNetwork {
        #[builder(
            default = "{ managed_zone_private_visibility_config_network_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "managed_zone_private_visibility_config_network_defaults :: kind")]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "networkUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fully qualified URL of the VPC network to bind to. This should be formatted like https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}"]
        pub network_url: ::std::option::Option<::std::string::String>,
    }
    impl ManagedZonePrivateVisibilityConfigNetwork {
        pub fn builder() -> ManagedZonePrivateVisibilityConfigNetworkBuilder {
            ManagedZonePrivateVisibilityConfigNetworkBuilder::default()
        }
    }
    mod managed_zone_private_visibility_config_network_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#managedZonePrivateVisibilityConfigNetwork")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ManagedZoneReverseLookupConfig {
        #[builder(
            default = "{ managed_zone_reverse_lookup_config_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "managed_zone_reverse_lookup_config_defaults :: kind")]
        pub kind: ::std::string::String,
    }
    impl ManagedZoneReverseLookupConfig {
        pub fn builder() -> ManagedZoneReverseLookupConfigBuilder {
            ManagedZoneReverseLookupConfigBuilder::default()
        }
    }
    mod managed_zone_reverse_lookup_config_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#managedZoneReverseLookupConfig")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains information about Service Directory-backed zones."]
    pub struct ManagedZoneServiceDirectoryConfig {
        #[builder(
            default = "{ managed_zone_service_directory_config_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "managed_zone_service_directory_config_defaults :: kind")]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "namespace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains information about the namespace associated with the zone."]
        pub namespace:
            ::std::option::Option<::std::boxed::Box<ManagedZoneServiceDirectoryConfigNamespace>>,
    }
    impl ManagedZoneServiceDirectoryConfig {
        pub fn builder() -> ManagedZoneServiceDirectoryConfigBuilder {
            ManagedZoneServiceDirectoryConfigBuilder::default()
        }
    }
    mod managed_zone_service_directory_config_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#managedZoneServiceDirectoryConfig")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ManagedZoneServiceDirectoryConfigNamespace {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deletionTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time that the namespace backing this zone was deleted, empty string if it still exists. This is in RFC3339 text format. Output only."]
        pub deletion_time: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ managed_zone_service_directory_config_namespace_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "managed_zone_service_directory_config_namespace_defaults :: kind")]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "namespaceUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fully qualified URL of the namespace associated with the zone. This should be formatted like https://servicedirectory.googleapis.com/v1/projects/{project}/locations/{location}/namespaces/{namespace}"]
        pub namespace_url: ::std::option::Option<::std::string::String>,
    }
    impl ManagedZoneServiceDirectoryConfigNamespace {
        pub fn builder() -> ManagedZoneServiceDirectoryConfigNamespaceBuilder {
            ManagedZoneServiceDirectoryConfigNamespaceBuilder::default()
        }
    }
    mod managed_zone_service_directory_config_namespace_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#managedZoneServiceDirectoryConfigNamespace")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ManagedZonesListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "header")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub header: ::std::option::Option<::std::boxed::Box<ResponseHeader>>,
        #[builder(
            default = "{ managed_zones_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "managed_zones_list_response_defaults :: kind")]
        #[doc = "Type of resource."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "managedZones")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The managed zone resources."]
        pub managed_zones: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ManagedZone>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The presence of this field indicates that there exist more results following your last page of results in pagination order. To fetch them, make another list request using this value as your page token. In this way you can retrieve the complete contents of even very large collections one page at a time. However, if the contents of the collection change between the first and last paginated list request, the set of all elements returned are an inconsistent view of the collection. There is no way to retrieve a consistent snapshot of a collection larger than the maximum page size."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ManagedZonesListResponse {
        pub fn builder() -> ManagedZonesListResponseBuilder {
            ManagedZonesListResponseBuilder::default()
        }
    }
    mod managed_zones_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#managedZonesListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An operation represents a successful mutation performed on a Cloud DNS resource. Operations provide: - An audit log of server resource mutations. - A way to recover/retry API calls in the case where the response is never received by the caller. Use the caller specified client_operation_id."]
    pub struct Operation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dnsKeyContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Only populated if the operation targeted a DnsKey (output only)."]
        pub dns_key_context: ::std::option::Option<::std::boxed::Box<OperationDnsKeyContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier for the resource. This is the client_operation_id if the client specified it when the mutation was initiated, otherwise, it is generated by the server. The name must be 1-63 characters long and match the regular expression [-a-z0-9]? (output only)"]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ operation_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "operation_defaults :: kind")]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time that this operation was started by the server. This is in RFC3339 text format (output only)."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of the operation. Can be one of the following: \"PENDING\" or \"DONE\" (output only). A status of \"DONE\" means that the request to update the authoritative servers has been sent, but the servers might not be updated yet."]
        pub status: ::std::option::Option<OperationStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of the operation. Operations include insert, update, and delete (output only)."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "user")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User who requested the operation, for example: user@example.com. cloud-dns-system for operations automatically done by the system. (output only)"]
        pub user: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zoneContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Only populated if the operation targeted a ManagedZone (output only)."]
        pub zone_context: ::std::option::Option<::std::boxed::Box<OperationManagedZoneContext>>,
    }
    impl Operation {
        pub fn builder() -> OperationBuilder {
            OperationBuilder::default()
        }
    }
    mod operation_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#operation")
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Status of the operation. Can be one of the following: \"PENDING\" or \"DONE\" (output only). A status of \"DONE\" means that the request to update the authoritative servers has been sent, but the servers might not be updated yet."]
    pub enum OperationStatusEnum {
        #[serde(rename = "pending")]
        #[doc = ""]
        Pending,
        #[serde(rename = "done")]
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
    pub struct OperationDnsKeyContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The post-operation DnsKey resource."]
        pub new_value: ::std::option::Option<::std::boxed::Box<DnsKey>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oldValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The pre-operation DnsKey resource."]
        pub old_value: ::std::option::Option<::std::boxed::Box<DnsKey>>,
    }
    impl OperationDnsKeyContext {
        pub fn builder() -> OperationDnsKeyContextBuilder {
            OperationDnsKeyContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OperationManagedZoneContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The post-operation ManagedZone resource."]
        pub new_value: ::std::option::Option<::std::boxed::Box<ManagedZone>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oldValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The pre-operation ManagedZone resource."]
        pub old_value: ::std::option::Option<::std::boxed::Box<ManagedZone>>,
    }
    impl OperationManagedZoneContext {
        pub fn builder() -> OperationManagedZoneContextBuilder {
            OperationManagedZoneContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PoliciesListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "header")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub header: ::std::option::Option<::std::boxed::Box<ResponseHeader>>,
        #[builder(
            default = "{ policies_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "policies_list_response_defaults :: kind")]
        #[doc = "Type of resource."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The presence of this field indicates that there exist more results following your last page of results in pagination order. To fetch them, make another list request using this value as your page token. In this way you can retrieve the complete contents of even very large collections one page at a time. However, if the contents of the collection change between the first and last paginated list request, the set of all elements returned are an inconsistent view of the collection. There is no way to retrieve a consistent snapshot of a collection larger than the maximum page size."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The policy resources."]
        pub policies: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Policy>>>,
    }
    impl PoliciesListResponse {
        pub fn builder() -> PoliciesListResponseBuilder {
            PoliciesListResponseBuilder::default()
        }
    }
    mod policies_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#policiesListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PoliciesPatchResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "header")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub header: ::std::option::Option<::std::boxed::Box<ResponseHeader>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub policy: ::std::option::Option<::std::boxed::Box<Policy>>,
    }
    impl PoliciesPatchResponse {
        pub fn builder() -> PoliciesPatchResponseBuilder {
            PoliciesPatchResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PoliciesUpdateResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "header")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub header: ::std::option::Option<::std::boxed::Box<ResponseHeader>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub policy: ::std::option::Option<::std::boxed::Box<Policy>>,
    }
    impl PoliciesUpdateResponse {
        pub fn builder() -> PoliciesUpdateResponseBuilder {
            PoliciesUpdateResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A policy is a collection of DNS rules applied to one or more Virtual Private Cloud resources."]
    pub struct Policy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alternativeNameServerConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sets an alternative name server for the associated networks. When specified, all DNS queries are forwarded to a name server that you choose. Names such as .internal are not available when an alternative name server is specified."]
        pub alternative_name_server_config:
            ::std::option::Option<::std::boxed::Box<PolicyAlternativeNameServerConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the policy's function."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableInboundForwarding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allows networks bound to this policy to receive DNS queries sent by VMs or applications over VPN connections. When enabled, a virtual IP address is allocated from each of the sub-networks that are bound to this policy."]
        pub enable_inbound_forwarding: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableLogging")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Controls whether logging is enabled for the networks bound to this policy. Defaults to no logging if not set."]
        pub enable_logging: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier for the resource; defined by the server (output only)."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ policy_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "policy_defaults :: kind")]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User-assigned name for this policy."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "networks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of network names specifying networks to which this policy is applied."]
        pub networks: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PolicyNetwork>>>,
    }
    impl Policy {
        pub fn builder() -> PolicyBuilder {
            PolicyBuilder::default()
        }
    }
    mod policy_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#policy")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PolicyAlternativeNameServerConfig {
        #[builder(
            default = "{ policy_alternative_name_server_config_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "policy_alternative_name_server_config_defaults :: kind")]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetNameServers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sets an alternative name server for the associated networks. When specified, all DNS queries are forwarded to a name server that you choose. Names such as .internal are not available when an alternative name server is specified."]
        pub target_name_servers: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<PolicyAlternativeNameServerConfigTargetNameServer>>,
        >,
    }
    impl PolicyAlternativeNameServerConfig {
        pub fn builder() -> PolicyAlternativeNameServerConfigBuilder {
            PolicyAlternativeNameServerConfigBuilder::default()
        }
    }
    mod policy_alternative_name_server_config_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#policyAlternativeNameServerConfig")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PolicyAlternativeNameServerConfigTargetNameServer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "forwardingPath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Forwarding path for this TargetNameServer. If unset or set to DEFAULT, Cloud DNS makes forwarding decision based on address ranges; that is, RFC1918 addresses go to the VPC, non-RFC1918 addresses go to the internet. When set to PRIVATE, Cloud DNS always sends queries through VPC for this target."]
        pub forwarding_path: ::std::option::Option<
            PolicyAlternativeNameServerConfigTargetNameServerForwardingPathEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipv4Address")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "IPv4 address to forward to."]
        pub ipv4_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipv6Address")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "IPv6 address to forward to. Does not accept both fields (ipv4 & ipv6) being populated."]
        pub ipv6_address: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ policy_alternative_name_server_config_target_name_server_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(
            default = "policy_alternative_name_server_config_target_name_server_defaults :: kind"
        )]
        pub kind: ::std::string::String,
    }
    impl PolicyAlternativeNameServerConfigTargetNameServer {
        pub fn builder() -> PolicyAlternativeNameServerConfigTargetNameServerBuilder {
            PolicyAlternativeNameServerConfigTargetNameServerBuilder::default()
        }
    }
    mod policy_alternative_name_server_config_target_name_server_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#policyAlternativeNameServerConfigTargetNameServer")
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Forwarding path for this TargetNameServer. If unset or set to DEFAULT, Cloud DNS makes forwarding decision based on address ranges; that is, RFC1918 addresses go to the VPC, non-RFC1918 addresses go to the internet. When set to PRIVATE, Cloud DNS always sends queries through VPC for this target."]
    pub enum PolicyAlternativeNameServerConfigTargetNameServerForwardingPathEnum {
        #[serde(rename = "default")]
        #[doc = "Cloud DNS will make forwarding decision based on address ranges; that is, RFC1918 addresses forward to the target through the VPC and non-RFC1918 addresses forward to the target through the internet"]
        Default,
        #[serde(rename = "private")]
        #[doc = "Cloud DNS will always forward to this target through the VPC."]
        Private,
    }
    impl ::std::default::Default
        for PolicyAlternativeNameServerConfigTargetNameServerForwardingPathEnum
    {
        fn default() -> Self {
            Self::Default
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PolicyNetwork {
        #[builder(default = "{ policy_network_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "policy_network_defaults :: kind")]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "networkUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fully qualified URL of the VPC network to bind to. This should be formatted like https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}"]
        pub network_url: ::std::option::Option<::std::string::String>,
    }
    impl PolicyNetwork {
        pub fn builder() -> PolicyNetworkBuilder {
            PolicyNetworkBuilder::default()
        }
    }
    mod policy_network_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#policyNetwork")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A project resource. The project is a top level container for resources including Cloud DNS ManagedZones. Projects can be created only in the APIs console."]
    pub struct Project {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User assigned unique identifier for the resource (output only)."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ project_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "project_defaults :: kind")]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "number")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique numeric identifier for the resource; defined by the server (output only)."]
        pub number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quota")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Quotas assigned to this project (output only)."]
        pub quota: ::std::option::Option<::std::boxed::Box<Quota>>,
    }
    impl Project {
        pub fn builder() -> ProjectBuilder {
            ProjectBuilder::default()
        }
    }
    mod project_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#project")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Limits associated with a Project."]
    pub struct Quota {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dnsKeysPerManagedZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum allowed number of DnsKeys per ManagedZone."]
        pub dns_keys_per_managed_zone: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ quota_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "quota_defaults :: kind")]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "managedZones")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum allowed number of managed zones in the project."]
        pub managed_zones: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "managedZonesPerNetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum allowed number of managed zones which can be attached to a network."]
        pub managed_zones_per_network: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "networksPerManagedZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum allowed number of networks to which a privately scoped zone can be attached."]
        pub networks_per_managed_zone: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "networksPerPolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum allowed number of networks per policy."]
        pub networks_per_policy: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum allowed number of policies per project."]
        pub policies: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceRecordsPerRrset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum allowed number of ResourceRecords per ResourceRecordSet."]
        pub resource_records_per_rrset: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responsePolicyRulesPerResponsePolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum allowed number of rules per response policy."]
        pub response_policy_rules_per_response_policy: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rrsetAdditionsPerChange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum allowed number of ResourceRecordSets to add per ChangesCreateRequest."]
        pub rrset_additions_per_change: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rrsetDeletionsPerChange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum allowed number of ResourceRecordSets to delete per ChangesCreateRequest."]
        pub rrset_deletions_per_change: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rrsetsPerManagedZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum allowed number of ResourceRecordSets per zone in the project."]
        pub rrsets_per_managed_zone: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetNameServersPerManagedZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum allowed number of target name servers per managed forwarding zone."]
        pub target_name_servers_per_managed_zone: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetNameServersPerPolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum allowed number of alternative target name servers per policy."]
        pub target_name_servers_per_policy: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalRrdataSizePerChange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum allowed size for total rrdata in one ChangesCreateRequest in bytes."]
        pub total_rrdata_size_per_change: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whitelistedKeySpecs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DNSSEC algorithm and key length types that can be used for DnsKeys."]
        pub whitelisted_key_specs:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DnsKeySpec>>>,
    }
    impl Quota {
        pub fn builder() -> QuotaBuilder {
            QuotaBuilder::default()
        }
    }
    mod quota_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#quota")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A unit of data that will be returned by the DNS servers."]
    pub struct ResourceRecordSet {
        #[builder(default = "{ resource_record_set_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "resource_record_set_defaults :: kind")]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For example, www.example.com."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rrdatas")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "As defined in RFC 1035 (section 5) and RFC 1034 (section 3.6.1) -- see examples."]
        pub rrdatas: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "signatureRrdatas")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "As defined in RFC 4034 (section 3.2)."]
        pub signature_rrdatas: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ttl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of seconds that this ResourceRecordSet can be cached by resolvers."]
        pub ttl: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier of a supported record type. See the list of Supported DNS record types."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl ResourceRecordSet {
        pub fn builder() -> ResourceRecordSetBuilder {
            ResourceRecordSetBuilder::default()
        }
    }
    mod resource_record_set_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#resourceRecordSet")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ResourceRecordSetsListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "header")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub header: ::std::option::Option<::std::boxed::Box<ResponseHeader>>,
        #[builder(
            default = "{ resource_record_sets_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "resource_record_sets_list_response_defaults :: kind")]
        #[doc = "Type of resource."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The presence of this field indicates that there exist more results following your last page of results in pagination order. To fetch them, make another list request using this value as your pagination token. This lets you retrieve complete contents of even larger collections, one page at a time. However, if the contents of the collection change between the first and last paginated list request, the set of elements returned are an inconsistent view of the collection. You cannot retrieve a consistent snapshot of a collection larger than the maximum page size."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rrsets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource record set resources."]
        pub rrsets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ResourceRecordSet>>>,
    }
    impl ResourceRecordSetsListResponse {
        pub fn builder() -> ResourceRecordSetsListResponseBuilder {
            ResourceRecordSetsListResponseBuilder::default()
        }
    }
    mod resource_record_sets_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#resourceRecordSetsListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Elements common to every response."]
    pub struct ResponseHeader {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For mutating operation requests that completed successfully. This is the client_operation_id if the client specified it, otherwise it is generated by the server (output only)."]
        pub operation_id: ::std::option::Option<::std::string::String>,
    }
    impl ResponseHeader {
        pub fn builder() -> ResponseHeaderBuilder {
            ResponseHeaderBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ResponsePoliciesListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "header")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub header: ::std::option::Option<::std::boxed::Box<ResponseHeader>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The presence of this field indicates that there exist more results following your last page of results in pagination order. To fetch them, make another list request using this value as your page token. In this way you can retrieve the complete contents of even very large collections one page at a time. However, if the contents of the collection change between the first and last paginated list request, the set of all elements returned are an inconsistent view of the collection. There is no way to retrieve a consistent snapshot of a collection larger than the maximum page size."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responsePolicies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Response Policy resources."]
        pub response_policies:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ResponsePolicy>>>,
    }
    impl ResponsePoliciesListResponse {
        pub fn builder() -> ResponsePoliciesListResponseBuilder {
            ResponsePoliciesListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ResponsePoliciesPatchResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "header")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub header: ::std::option::Option<::std::boxed::Box<ResponseHeader>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responsePolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub response_policy: ::std::option::Option<::std::boxed::Box<ResponsePolicy>>,
    }
    impl ResponsePoliciesPatchResponse {
        pub fn builder() -> ResponsePoliciesPatchResponseBuilder {
            ResponsePoliciesPatchResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ResponsePoliciesUpdateResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "header")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub header: ::std::option::Option<::std::boxed::Box<ResponseHeader>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responsePolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub response_policy: ::std::option::Option<::std::boxed::Box<ResponsePolicy>>,
    }
    impl ResponsePoliciesUpdateResponse {
        pub fn builder() -> ResponsePoliciesUpdateResponseBuilder {
            ResponsePoliciesUpdateResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Response Policy is a collection of selectors that apply to queries made against one or more Virtual Private Cloud networks."]
    pub struct ResponsePolicy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User-provided description for this Response Policy."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier for the resource; defined by the server (output only)."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ response_policy_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "response_policy_defaults :: kind")]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "networks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of network names specifying networks to which this policy is applied."]
        pub networks:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ResponsePolicyNetwork>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responsePolicyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User assigned name for this Response Policy."]
        pub response_policy_name: ::std::option::Option<::std::string::String>,
    }
    impl ResponsePolicy {
        pub fn builder() -> ResponsePolicyBuilder {
            ResponsePolicyBuilder::default()
        }
    }
    mod response_policy_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#responsePolicy")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ResponsePolicyNetwork {
        #[builder(
            default = "{ response_policy_network_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "response_policy_network_defaults :: kind")]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "networkUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fully qualified URL of the VPC network to bind to. This should be formatted like https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}"]
        pub network_url: ::std::option::Option<::std::string::String>,
    }
    impl ResponsePolicyNetwork {
        pub fn builder() -> ResponsePolicyNetworkBuilder {
            ResponsePolicyNetworkBuilder::default()
        }
    }
    mod response_policy_network_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#responsePolicyNetwork")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Response Policy Rule is a selector that applies its behavior to queries that match the selector. Selectors are DNS names, which may be wildcards or exact matches. Each DNS query subject to a Response Policy matches at most one ResponsePolicyRule, as identified by the dns_name field with the longest matching suffix."]
    pub struct ResponsePolicyRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "behavior")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Answer this query with a behavior rather than DNS data."]
        pub behavior: ::std::option::Option<ResponsePolicyRuleBehaviorEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dnsName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The DNS name (wildcard or exact) to apply this rule to. Must be unique within the Response Policy Rule."]
        pub dns_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ response_policy_rule_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "response_policy_rule_defaults :: kind")]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Answer this query directly with DNS data. These ResourceRecordSets override any other DNS behavior for the matched name; in particular they override private zones, the public internet, and GCP internal DNS. No SOA nor NS types are allowed."]
        pub local_data: ::std::option::Option<::std::boxed::Box<ResponsePolicyRuleLocalData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ruleName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An identifier for this rule. Must be unique with the ResponsePolicy."]
        pub rule_name: ::std::option::Option<::std::string::String>,
    }
    impl ResponsePolicyRule {
        pub fn builder() -> ResponsePolicyRuleBuilder {
            ResponsePolicyRuleBuilder::default()
        }
    }
    mod response_policy_rule_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("dns#responsePolicyRule")
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Answer this query with a behavior rather than DNS data."]
    pub enum ResponsePolicyRuleBehaviorEnum {
        #[serde(rename = "behaviorUnspecified")]
        #[doc = ""]
        BehaviorUnspecified,
        #[serde(rename = "bypassResponsePolicy")]
        #[doc = "Skip a less-specific ResponsePolicyRule and continue normal query logic. This can be used in conjunction with a wildcard to exempt a subset of the wildcard ResponsePolicyRule from the ResponsePolicy behavior and e.g., query the public internet instead. For instance, if these rules exist: *.example.com -> 1.2.3.4 foo.example.com -> PASSTHRU Then a query for 'foo.example.com' will skip the wildcard."]
        BypassResponsePolicy,
    }
    impl ::std::default::Default for ResponsePolicyRuleBehaviorEnum {
        fn default() -> Self {
            Self::BehaviorUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ResponsePolicyRuleLocalData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localDatas")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All resource record sets for this selector, one per resource record type. The name must match the dns_name."]
        pub local_datas:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ResourceRecordSet>>>,
    }
    impl ResponsePolicyRuleLocalData {
        pub fn builder() -> ResponsePolicyRuleLocalDataBuilder {
            ResponsePolicyRuleLocalDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ResponsePolicyRulesListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "header")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub header: ::std::option::Option<::std::boxed::Box<ResponseHeader>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The presence of this field indicates that there exist more results following your last page of results in pagination order. To fetch them, make another list request using this value as your page token. In this way you can retrieve the complete contents of even very large collections one page at a time. However, if the contents of the collection change between the first and last paginated list request, the set of all elements returned are an inconsistent view of the collection. There is no way to retrieve a consistent snapshot of a collection larger than the maximum page size."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responsePolicyRules")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Response Policy Rule resources."]
        pub response_policy_rules:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ResponsePolicyRule>>>,
    }
    impl ResponsePolicyRulesListResponse {
        pub fn builder() -> ResponsePolicyRulesListResponseBuilder {
            ResponsePolicyRulesListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ResponsePolicyRulesPatchResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "header")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub header: ::std::option::Option<::std::boxed::Box<ResponseHeader>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responsePolicyRule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub response_policy_rule: ::std::option::Option<::std::boxed::Box<ResponsePolicyRule>>,
    }
    impl ResponsePolicyRulesPatchResponse {
        pub fn builder() -> ResponsePolicyRulesPatchResponseBuilder {
            ResponsePolicyRulesPatchResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ResponsePolicyRulesUpdateResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "header")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub header: ::std::option::Option<::std::boxed::Box<ResponseHeader>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responsePolicyRule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub response_policy_rule: ::std::option::Option<::std::boxed::Box<ResponsePolicyRule>>,
    }
    impl ResponsePolicyRulesUpdateResponse {
        pub fn builder() -> ResponsePolicyRulesUpdateResponseBuilder {
            ResponsePolicyRulesUpdateResponseBuilder::default()
        }
    }
}
