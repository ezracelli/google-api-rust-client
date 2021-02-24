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
    pub mod projects {
        pub mod resources {
            pub mod aggregated {
                pub mod resources {
                    pub mod usable_subnetworks {
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
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "filter")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Filtering currently only supports equality on the networkProjectId and must be in the form: \"networkProjectId=[PROJECTID]\", where `networkProjectId` is the project which owns the listed subnetworks. This defaults to the parent project ID."]
                                    pub filter: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The max number of results per page that should be returned. If the number of available results is larger than `page_size`, a `next_page_token` is returned which can be used to get the next page of results in subsequent requests. Acceptable values are 0 to 500, inclusive. (Default: 500)"]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Specifies a page token to use. Set this to the nextPageToken returned by previous list requests to get the next page of results."]
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
                }
            }
            pub mod locations {
                pub mod methods {
                    pub mod get_server_config {
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
                            #[serde(rename = "projectId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
                            pub project_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "zone")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) to return operations for. This field has been deprecated and replaced by the name field."]
                            pub zone: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
                pub mod resources {
                    pub mod clusters {
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
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "clusterId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Deprecated. The name of the cluster to delete. This field has been deprecated and replaced by the name field."]
                                    pub cluster_id: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "projectId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
                                    pub project_id: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "zone")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
                                    pub zone: ::std::option::Option<::std::string::String>,
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
                                    #[serde(rename = "clusterId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Deprecated. The name of the cluster to retrieve. This field has been deprecated and replaced by the name field."]
                                    pub cluster_id: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "projectId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
                                    pub project_id: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "zone")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
                                    pub zone: ::std::option::Option<::std::string::String>,
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
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "projectId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the parent field."]
                                    pub project_id: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "zone")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides, or \"-\" for all zones. This field has been deprecated and replaced by the parent field."]
                                    pub zone: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                        }
                        pub mod resources {
                            pub mod node_pools {
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
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "clusterId")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Deprecated. The name of the cluster. This field has been deprecated and replaced by the name field."]
                                            pub cluster_id:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "nodePoolId")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Deprecated. The name of the node pool to delete. This field has been deprecated and replaced by the name field."]
                                            pub node_pool_id:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "projectId")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://developers.google.com/console/help/new/#projectnumber). This field has been deprecated and replaced by the name field."]
                                            pub project_id:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "zone")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
                                            pub zone: ::std::option::Option<::std::string::String>,
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
                                            #[serde(rename = "clusterId")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Deprecated. The name of the cluster. This field has been deprecated and replaced by the name field."]
                                            pub cluster_id:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "nodePoolId")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Deprecated. The name of the node pool. This field has been deprecated and replaced by the name field."]
                                            pub node_pool_id:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "projectId")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://developers.google.com/console/help/new/#projectnumber). This field has been deprecated and replaced by the name field."]
                                            pub project_id:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "zone")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
                                            pub zone: ::std::option::Option<::std::string::String>,
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
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "clusterId")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Deprecated. The name of the cluster. This field has been deprecated and replaced by the parent field."]
                                            pub cluster_id:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "projectId")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://developers.google.com/console/help/new/#projectnumber). This field has been deprecated and replaced by the parent field."]
                                            pub project_id:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "zone")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field."]
                                            pub zone: ::std::option::Option<::std::string::String>,
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
                    pub mod operations {
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
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "operationId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Deprecated. The server-assigned `name` of the operation. This field has been deprecated and replaced by the name field."]
                                    pub operation_id: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "projectId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
                                    pub project_id: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "zone")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
                                    pub zone: ::std::option::Option<::std::string::String>,
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
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "projectId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the parent field."]
                                    pub project_id: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "zone")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) to return operations for, or `-` for all zones. This field has been deprecated and replaced by the parent field."]
                                    pub zone: ::std::option::Option<::std::string::String>,
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
            pub mod zones {
                pub mod methods {
                    pub mod get_serverconfig {
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
                            #[serde(rename = "name")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The name (project and location) of the server config to get, specified in the format `projects/*/locations/*`."]
                            pub name: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
                pub mod resources {
                    pub mod clusters {
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
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "name")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The name (project, location, cluster) of the cluster to delete. Specified in the format `projects/*/locations/*/clusters/*`."]
                                    pub name: ::std::option::Option<::std::string::String>,
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
                                    #[serde(rename = "name")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The name (project, location, cluster) of the cluster to retrieve. Specified in the format `projects/*/locations/*/clusters/*`."]
                                    pub name: ::std::option::Option<::std::string::String>,
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
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "parent")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The parent (project and location) where the clusters will be listed. Specified in the format `projects/*/locations/*`. Location \"-\" matches all zones and all regions."]
                                    pub parent: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                        }
                        pub mod resources {
                            pub mod node_pools {
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
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "name")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The name (project, location, cluster, node pool id) of the node pool to delete. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`."]
                                            pub name: ::std::option::Option<::std::string::String>,
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
                                            #[serde(rename = "name")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The name (project, location, cluster, node pool id) of the node pool to get. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`."]
                                            pub name: ::std::option::Option<::std::string::String>,
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
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "parent")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The parent (project, location, cluster id) where the node pools will be listed. Specified in the format `projects/*/locations/*/clusters/*`."]
                                            pub parent:
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
                    pub mod operations {
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
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "name")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The name (project, location, operation id) of the operation to get. Specified in the format `projects/*/locations/*/operations/*`."]
                                    pub name: ::std::option::Option<::std::string::String>,
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
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "parent")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The parent (project and location) where the operations will be listed. Specified in the format `projects/*/locations/*`. Location \"-\" matches all zones and all regions."]
                                    pub parent: ::std::option::Option<::std::string::String>,
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
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "AcceleratorConfig represents a Hardware Accelerator request."]
    pub struct AcceleratorConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "acceleratorCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of the accelerator cards exposed to an instance."]
        pub accelerator_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "acceleratorType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The accelerator type resource name. List of supported accelerators [here](https://cloud.google.com/compute/docs/gpus)"]
        pub accelerator_type: ::std::option::Option<::std::string::String>,
    }
    impl AcceleratorConfig {
        pub fn builder() -> AcceleratorConfigBuilder {
            AcceleratorConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for the addons that can be automatically spun up in the cluster, enabling additional functionality."]
    pub struct AddonsConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloudRunConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for the Cloud Run addon, which allows the user to use a managed Knative service."]
        pub cloud_run_config: ::std::option::Option<::std::boxed::Box<CloudRunConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "configConnectorConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for the ConfigConnector add-on, a Kubernetes extension to manage hosted GCP services through the Kubernetes API"]
        pub config_connector_config:
            ::std::option::Option<::std::boxed::Box<ConfigConnectorConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dnsCacheConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for NodeLocalDNS, a dns cache running on cluster nodes"]
        pub dns_cache_config: ::std::option::Option<::std::boxed::Box<DnsCacheConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcePersistentDiskCsiDriverConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for the Compute Engine Persistent Disk CSI driver."]
        pub gce_persistent_disk_csi_driver_config:
            ::std::option::Option<::std::boxed::Box<GcePersistentDiskCsiDriverConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "horizontalPodAutoscaling")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for the horizontal pod autoscaling feature, which increases or decreases the number of replica pods a replication controller has based on the resource usage of the existing pods."]
        pub horizontal_pod_autoscaling:
            ::std::option::Option<::std::boxed::Box<HorizontalPodAutoscaling>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "httpLoadBalancing")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for the HTTP (L7) load balancing controller addon, which makes it easy to set up HTTP load balancers for services in a cluster."]
        pub http_load_balancing: ::std::option::Option<::std::boxed::Box<HttpLoadBalancing>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kubernetesDashboard")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for the Kubernetes Dashboard. This addon is deprecated, and will be disabled in 1.15. It is recommended to use the Cloud Console to manage and monitor your Kubernetes clusters, workloads and applications. For more information, see: https://cloud.google.com/kubernetes-engine/docs/concepts/dashboards"]
        pub kubernetes_dashboard: ::std::option::Option<::std::boxed::Box<KubernetesDashboard>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "networkPolicyConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for NetworkPolicy. This only tracks whether the addon is enabled or not on the Master, it does not track whether network policy is enabled for the nodes."]
        pub network_policy_config: ::std::option::Option<::std::boxed::Box<NetworkPolicyConfig>>,
    }
    impl AddonsConfig {
        pub fn builder() -> AddonsConfigBuilder {
            AddonsConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for returning group information from authenticators."]
    pub struct AuthenticatorGroupsConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this cluster should return group membership lookups during authentication using a group of security groups."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "securityGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the security group-of-groups to be used. Only relevant if enabled = true."]
        pub security_group: ::std::option::Option<::std::string::String>,
    }
    impl AuthenticatorGroupsConfig {
        pub fn builder() -> AuthenticatorGroupsConfigBuilder {
            AuthenticatorGroupsConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "AutoUpgradeOptions defines the set of options for the user to control how the Auto Upgrades will proceed."]
    pub struct AutoUpgradeOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoUpgradeStartTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] This field is set when upgrades are about to commence with the approximate start time for the upgrades, in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format."]
        pub auto_upgrade_start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] This field is set when upgrades are about to commence with the description of the upgrade."]
        pub description: ::std::option::Option<::std::string::String>,
    }
    impl AutoUpgradeOptions {
        pub fn builder() -> AutoUpgradeOptionsBuilder {
            AutoUpgradeOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Autopilot is the configuration for Autopilot settings on the cluster. It is the official product name of what is previously known as AutoGKE"]
    pub struct Autopilot {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enable Autopilot"]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl Autopilot {
        pub fn builder() -> AutopilotBuilder {
            AutopilotBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "AutoprovisioningNodePoolDefaults contains defaults for a node pool created by NAP."]
    pub struct AutoprovisioningNodePoolDefaults {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bootDiskKmsKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Customer Managed Encryption Key used to encrypt the boot disk attached to each node in the node pool. This should be of the form projects/[KEY_PROJECT_ID]/locations/[LOCATION]/keyRings/[RING_NAME]/cryptoKeys/[KEY_NAME]. For more information about protecting resources with Cloud KMS Keys please see: https://cloud.google.com/compute/docs/disks/customer-managed-encryption"]
        pub boot_disk_kms_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diskSizeGb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Size of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB. If unspecified, the default disk size is 100GB."]
        pub disk_size_gb: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diskType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of the disk attached to each node (e.g. 'pd-standard', 'pd-ssd' or 'pd-balanced') If unspecified, the default disk type is 'pd-standard'"]
        pub disk_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "management")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the node management options for NAP created node-pools."]
        pub management: ::std::option::Option<::std::boxed::Box<NodeManagement>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minCpuPlatform")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum CPU platform to be used for NAP created node pools. The instance may be scheduled on the specified or newer CPU platform. Applicable values are the friendly names of CPU platforms, such as minCpuPlatform: Intel Haswell or minCpuPlatform: Intel Sandy Bridge. For more information, read [how to specify min CPU platform](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform) To unset the min cpu platform field pass \"automatic\" as field value."]
        pub min_cpu_platform: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oauthScopes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Scopes that are used by NAP when creating node pools."]
        pub oauth_scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceAccount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Google Cloud Platform Service Account to be used by the node VMs."]
        pub service_account: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shieldedInstanceConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Shielded Instance options."]
        pub shielded_instance_config:
            ::std::option::Option<::std::boxed::Box<ShieldedInstanceConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "upgradeSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the upgrade settings for NAP created node pools"]
        pub upgrade_settings: ::std::option::Option<::std::boxed::Box<UpgradeSettings>>,
    }
    impl AutoprovisioningNodePoolDefaults {
        pub fn builder() -> AutoprovisioningNodePoolDefaultsBuilder {
            AutoprovisioningNodePoolDefaultsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parameters for using BigQuery as the destination of resource usage export."]
    pub struct BigQueryDestination {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datasetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of a BigQuery Dataset."]
        pub dataset_id: ::std::option::Option<::std::string::String>,
    }
    impl BigQueryDestination {
        pub fn builder() -> BigQueryDestinationBuilder {
            BigQueryDestinationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for Binary Authorization."]
    pub struct BinaryAuthorization {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enable Binary Authorization for this cluster. If enabled, all container images will be validated by Binary Authorization."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl BinaryAuthorization {
        pub fn builder() -> BinaryAuthorizationBuilder {
            BinaryAuthorizationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "CancelOperationRequest cancels a single operation."]
    pub struct CancelOperationRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name (project, location, operation id) of the operation to cancel. Specified in the format `projects/*/locations/*/operations/*`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The server-assigned `name` of the operation. This field has been deprecated and replaced by the name field."]
        pub operation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the operation resides. This field has been deprecated and replaced by the name field."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl CancelOperationRequest {
        pub fn builder() -> CancelOperationRequestBuilder {
            CancelOperationRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "CidrBlock contains an optional name and one CIDR block."]
    pub struct CidrBlock {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cidrBlock")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "cidr_block must be specified in CIDR notation."]
        pub cidr_block: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "display_name is an optional field for users to identify CIDR blocks."]
        pub display_name: ::std::option::Option<::std::string::String>,
    }
    impl CidrBlock {
        pub fn builder() -> CidrBlockBuilder {
            CidrBlockBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for client certificates on the cluster."]
    pub struct ClientCertificateConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "issueClientCertificate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Issue a client certificate."]
        pub issue_client_certificate: ::std::option::Option<::std::primitive::bool>,
    }
    impl ClientCertificateConfig {
        pub fn builder() -> ClientCertificateConfigBuilder {
            ClientCertificateConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration options for the Cloud Run feature."]
    pub struct CloudRunConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether Cloud Run addon is enabled for this cluster."]
        pub disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "loadBalancerType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Which load balancer type is installed for Cloud Run."]
        pub load_balancer_type: ::std::option::Option<CloudRunConfigLoadBalancerTypeEnum>,
    }
    impl CloudRunConfig {
        pub fn builder() -> CloudRunConfigBuilder {
            CloudRunConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Which load balancer type is installed for Cloud Run."]
    pub enum CloudRunConfigLoadBalancerTypeEnum {
        #[serde(rename = "LOAD_BALANCER_TYPE_UNSPECIFIED")]
        #[doc = "Load balancer type for Cloud Run is unspecified."]
        LoadBalancerTypeUnspecified,
        #[serde(rename = "LOAD_BALANCER_TYPE_EXTERNAL")]
        #[doc = "Install external load balancer for Cloud Run."]
        LoadBalancerTypeExternal,
        #[serde(rename = "LOAD_BALANCER_TYPE_INTERNAL")]
        #[doc = "Install internal load balancer for Cloud Run."]
        LoadBalancerTypeInternal,
    }
    impl ::std::default::Default for CloudRunConfigLoadBalancerTypeEnum {
        fn default() -> Self {
            Self::LoadBalancerTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Google Kubernetes Engine cluster."]
    pub struct Cluster {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addonsConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configurations for the various addons available to run in the cluster."]
        pub addons_config: ::std::option::Option<::std::boxed::Box<AddonsConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authenticatorGroupsConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration controlling RBAC group membership information."]
        pub authenticator_groups_config:
            ::std::option::Option<::std::boxed::Box<AuthenticatorGroupsConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autopilot")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Autopilot configuration for the cluster. It has the same semantics as AutoGKE and overrides the setting in autogke."]
        pub autopilot: ::std::option::Option<::std::boxed::Box<Autopilot>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoscaling")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cluster-level autoscaling configuration."]
        pub autoscaling: ::std::option::Option<::std::boxed::Box<ClusterAutoscaling>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "binaryAuthorization")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for Binary Authorization."]
        pub binary_authorization: ::std::option::Option<::std::boxed::Box<BinaryAuthorization>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterIpv4Cidr")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IP address range of the container pods in this cluster, in [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `10.96.0.0/14`). Leave blank to have one automatically chosen or specify a `/14` block in `10.0.0.0/8`."]
        pub cluster_ipv4_cidr: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conditions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Which conditions caused the current cluster state."]
        pub conditions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StatusCondition>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The time the cluster was created, in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentMasterVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The current software version of the master endpoint."]
        pub current_master_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentNodeCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The number of nodes currently in the cluster. Deprecated. Call Kubernetes API directly to retrieve node information."]
        pub current_node_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentNodeVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] Deprecated, use [NodePools.version](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters.nodePools) instead. The current version of the node software components. If they are currently at multiple versions because they're in the process of being upgraded, this reflects the minimum version of all nodes."]
        pub current_node_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "databaseEncryption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration of etcd encryption."]
        pub database_encryption: ::std::option::Option<::std::boxed::Box<DatabaseEncryption>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultMaxPodsConstraint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The default constraint on the maximum number of pods that can be run simultaneously on a node in the node pool of this cluster. Only honored if cluster created with IP Alias support."]
        pub default_max_pods_constraint:
            ::std::option::Option<::std::boxed::Box<MaxPodsConstraint>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional description of this cluster."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableKubernetesAlpha")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Kubernetes alpha features are enabled on this cluster. This includes alpha API groups (e.g. v1alpha1) and features that may not be production ready in the kubernetes version of the master and nodes. The cluster has no SLA for uptime and master/node upgrades are disabled. Alpha enabled clusters are automatically deleted thirty days after creation."]
        pub enable_kubernetes_alpha: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableTpu")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enable the ability to use Cloud TPUs in this cluster."]
        pub enable_tpu: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endpoint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The IP address of this cluster's master endpoint. The endpoint can be accessed from the internet at `https://username:password@endpoint/`. See the `masterAuth` property of this resource for username and password information."]
        pub endpoint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expireTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The time the cluster will be automatically deleted in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format."]
        pub expire_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "initialClusterVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The initial Kubernetes version for this cluster. Valid versions are those found in validMasterVersions returned by getServerConfig. The version can be upgraded over time; such upgrades are reflected in currentMasterVersion and currentNodeVersion. Users may specify either explicit versions offered by Kubernetes Engine or version aliases, which have the following behavior: - \"latest\": picks the highest valid Kubernetes version - \"1.X\": picks the highest valid patch+gke.N patch in the 1.X version - \"1.X.Y\": picks the highest valid gke.N patch in the 1.X.Y version - \"1.X.Y-gke.N\": picks an explicit Kubernetes version - \"\",\"-\": picks the default Kubernetes version"]
        pub initial_cluster_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "initialNodeCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of nodes to create in this cluster. You must ensure that your Compute Engine [resource quota](https://cloud.google.com/compute/quotas) is sufficient for this number of instances. You must also have available firewall and routes quota. For requests, this field should only be used in lieu of a \"node_pool\" object, since this configuration (along with the \"node_config\") will be used to create a \"NodePool\" object with an auto-generated name. Do not use this and a node_pool at the same time. This field is deprecated, use node_pool.initial_node_count instead."]
        pub initial_node_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instanceGroupUrls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Use node_pools.instance_group_urls."]
        pub instance_group_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipAllocationPolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for cluster IP allocation."]
        pub ip_allocation_policy: ::std::option::Option<::std::boxed::Box<IpAllocationPolicy>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelFingerprint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fingerprint of the set of labels for this cluster."]
        pub label_fingerprint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "legacyAbac")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for the legacy ABAC authorization mode."]
        pub legacy_abac: ::std::option::Option<::std::boxed::Box<LegacyAbac>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) or [region](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) in which the cluster resides."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available) in which the cluster's nodes should be located. This field provides a default value if [NodePool.Locations](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters.nodePools#NodePool.FIELDS.locations) are not specified during node pool creation. Warning: changing cluster locations will update the [NodePool.Locations](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters.nodePools#NodePool.FIELDS.locations) of all node pools and will result in nodes being added and/or removed."]
        pub locations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "loggingService")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The logging service the cluster should use to write logs. Currently available options: * `logging.googleapis.com/kubernetes` - The Cloud Logging service with a Kubernetes-native resource model * `logging.googleapis.com` - The legacy Cloud Logging service (no longer available as of GKE 1.15). * `none` - no logs will be exported from the cluster. If left as an empty string,`logging.googleapis.com/kubernetes` will be used for GKE 1.14+ or `logging.googleapis.com` for earlier versions."]
        pub logging_service: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maintenancePolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configure the maintenance policy for this cluster."]
        pub maintenance_policy: ::std::option::Option<::std::boxed::Box<MaintenancePolicy>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "masterAuth")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The authentication information for accessing the master endpoint. If unspecified, the defaults are used: For clusters before v1.12, if master_auth is unspecified, `username` will be set to \"admin\", a random password will be generated, and a client certificate will be issued."]
        pub master_auth: ::std::option::Option<::std::boxed::Box<MasterAuth>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "masterAuthorizedNetworksConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The configuration options for master authorized networks feature."]
        pub master_authorized_networks_config:
            ::std::option::Option<::std::boxed::Box<MasterAuthorizedNetworksConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "monitoringService")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The monitoring service the cluster should use to write metrics. Currently available options: * \"monitoring.googleapis.com/kubernetes\" - The Cloud Monitoring service with a Kubernetes-native resource model * `monitoring.googleapis.com` - The legacy Cloud Monitoring service (no longer available as of GKE 1.15). * `none` - No metrics will be exported from the cluster. If left as an empty string,`monitoring.googleapis.com/kubernetes` will be used for GKE 1.14+ or `monitoring.googleapis.com` for earlier versions."]
        pub monitoring_service: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of this cluster. The name must be unique within this project and location (e.g. zone or region), and can be up to 40 characters with the following restrictions: * Lowercase letters, numbers, and hyphens only. * Must start with a letter. * Must end with a number or a letter."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "network")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the Google Compute Engine [network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks) to which the cluster is connected. If left unspecified, the `default` network will be used."]
        pub network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "networkConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for cluster networking."]
        pub network_config: ::std::option::Option<::std::boxed::Box<NetworkConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "networkPolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration options for the NetworkPolicy feature."]
        pub network_policy: ::std::option::Option<::std::boxed::Box<NetworkPolicy>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodeConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parameters used in creating the cluster's nodes. For requests, this field should only be used in lieu of a \"node_pool\" object, since this configuration (along with the \"initial_node_count\") will be used to create a \"NodePool\" object with an auto-generated name. Do not use this and a node_pool at the same time. For responses, this field will be populated with the node configuration of the first node pool. (For configuration of each node pool, see `node_pool.config`) If unspecified, the defaults are used. This field is deprecated, use node_pool.config instead."]
        pub node_config: ::std::option::Option<::std::boxed::Box<NodeConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodeIpv4CidrSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The size of the address space on each node for hosting containers. This is provisioned from within the `container_ipv4_cidr` range. This field will only be set when cluster is in route-based network mode."]
        pub node_ipv4_cidr_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodePools")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The node pools associated with this cluster. This field should not be set if \"node_config\" or \"initial_node_count\" are specified."]
        pub node_pools: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NodePool>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notificationConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Notification configuration of the cluster."]
        pub notification_config: ::std::option::Option<::std::boxed::Box<NotificationConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "privateClusterConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for private cluster."]
        pub private_cluster_config: ::std::option::Option<::std::boxed::Box<PrivateClusterConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "releaseChannel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Release channel configuration."]
        pub release_channel: ::std::option::Option<::std::boxed::Box<ReleaseChannel>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource labels for the cluster to use to annotate any related Google Compute Engine resources."]
        pub resource_labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceUsageExportConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for exporting resource usages. Resource usage export is disabled when this config is unspecified."]
        pub resource_usage_export_config:
            ::std::option::Option<::std::boxed::Box<ResourceUsageExportConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] Server-defined URL for the resource."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "servicesIpv4Cidr")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The IP address range of the Kubernetes services in this cluster, in [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `1.2.3.4/29`). Service addresses are typically put in the last `/16` from the container CIDR."]
        pub services_ipv4_cidr: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shieldedNodes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Shielded Nodes configuration."]
        pub shielded_nodes: ::std::option::Option<::std::boxed::Box<ShieldedNodes>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The current status of this cluster."]
        pub status: ::std::option::Option<ClusterStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statusMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] Deprecated. Use conditions instead. Additional information about the current status of this cluster, if available."]
        pub status_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subnetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the Google Compute Engine [subnetwork](https://cloud.google.com/compute/docs/subnetworks) to which the cluster is connected."]
        pub subnetwork: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tpuIpv4CidrBlock")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The IP address range of the Cloud TPUs in this cluster, in [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `1.2.3.4/29`)."]
        pub tpu_ipv4_cidr_block: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verticalPodAutoscaling")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cluster-level Vertical Pod Autoscaling configuration."]
        pub vertical_pod_autoscaling:
            ::std::option::Option<::std::boxed::Box<VerticalPodAutoscaling>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workloadIdentityConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for the use of Kubernetes Service Accounts in GCP IAM policies."]
        pub workload_identity_config:
            ::std::option::Option<::std::boxed::Box<WorkloadIdentityConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field is deprecated, use location instead."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl Cluster {
        pub fn builder() -> ClusterBuilder {
            ClusterBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "[Output only] The current status of this cluster."]
    pub enum ClusterStatusEnum {
        #[serde(rename = "STATUS_UNSPECIFIED")]
        #[doc = "Not set."]
        StatusUnspecified,
        #[serde(rename = "PROVISIONING")]
        #[doc = "The PROVISIONING state indicates the cluster is being created."]
        Provisioning,
        #[serde(rename = "RUNNING")]
        #[doc = "The RUNNING state indicates the cluster has been created and is fully usable."]
        Running,
        #[serde(rename = "RECONCILING")]
        #[doc = "The RECONCILING state indicates that some work is actively being done on the cluster, such as upgrading the master or node software. Details can be found in the `statusMessage` field."]
        Reconciling,
        #[serde(rename = "STOPPING")]
        #[doc = "The STOPPING state indicates the cluster is being deleted."]
        Stopping,
        #[serde(rename = "ERROR")]
        #[doc = "The ERROR state indicates the cluster is unusable. It will be automatically deleted. Details can be found in the `statusMessage` field."]
        Error,
        #[serde(rename = "DEGRADED")]
        #[doc = "The DEGRADED state indicates the cluster requires user action to restore full functionality. Details can be found in the `statusMessage` field."]
        Degraded,
    }
    impl ::std::default::Default for ClusterStatusEnum {
        fn default() -> Self {
            Self::StatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "ClusterAutoscaling contains global, per-cluster information required by Cluster Autoscaler to automatically adjust the size of the cluster and create/delete node pools based on the current needs."]
    pub struct ClusterAutoscaling {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoprovisioningLocations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available) in which the NodePool's nodes can be created by NAP."]
        pub autoprovisioning_locations:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoprovisioningNodePoolDefaults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "AutoprovisioningNodePoolDefaults contains defaults for a node pool created by NAP."]
        pub autoprovisioning_node_pool_defaults:
            ::std::option::Option<::std::boxed::Box<AutoprovisioningNodePoolDefaults>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableNodeAutoprovisioning")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enables automatic node pool creation and deletion."]
        pub enable_node_autoprovisioning: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceLimits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains global constraints regarding minimum and maximum amount of resources in the cluster."]
        pub resource_limits:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ResourceLimit>>>,
    }
    impl ClusterAutoscaling {
        pub fn builder() -> ClusterAutoscalingBuilder {
            ClusterAutoscalingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "ClusterUpdate describes an update to the cluster. Exactly one update can be applied to a cluster with each request, so at most one field can be provided."]
    pub struct ClusterUpdate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desiredAddonsConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configurations for the various addons available to run in the cluster."]
        pub desired_addons_config: ::std::option::Option<::std::boxed::Box<AddonsConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desiredBinaryAuthorization")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The desired configuration options for the Binary Authorization feature."]
        pub desired_binary_authorization:
            ::std::option::Option<::std::boxed::Box<BinaryAuthorization>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desiredClusterAutoscaling")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cluster-level autoscaling configuration."]
        pub desired_cluster_autoscaling:
            ::std::option::Option<::std::boxed::Box<ClusterAutoscaling>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desiredDatabaseEncryption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration of etcd encryption."]
        pub desired_database_encryption:
            ::std::option::Option<::std::boxed::Box<DatabaseEncryption>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desiredDefaultSnatStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The desired status of whether to disable default sNAT for this cluster."]
        pub desired_default_snat_status:
            ::std::option::Option<::std::boxed::Box<DefaultSnatStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desiredImageType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The desired image type for the node pool. NOTE: Set the \"desired_node_pool\" field as well."]
        pub desired_image_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desiredIntraNodeVisibilityConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The desired config of Intra-node visibility."]
        pub desired_intra_node_visibility_config:
            ::std::option::Option<::std::boxed::Box<IntraNodeVisibilityConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desiredLocations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The desired list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available) in which the cluster's nodes should be located. This list must always include the cluster's primary zone. Warning: changing cluster locations will update the locations of all node pools and will result in nodes being added and/or removed."]
        pub desired_locations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desiredLoggingService")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The logging service the cluster should use to write logs. Currently available options: * `logging.googleapis.com/kubernetes` - The Cloud Logging service with a Kubernetes-native resource model * `logging.googleapis.com` - The legacy Cloud Logging service (no longer available as of GKE 1.15). * `none` - no logs will be exported from the cluster. If left as an empty string,`logging.googleapis.com/kubernetes` will be used for GKE 1.14+ or `logging.googleapis.com` for earlier versions."]
        pub desired_logging_service: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desiredMasterAuthorizedNetworksConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The desired configuration options for master authorized networks feature."]
        pub desired_master_authorized_networks_config:
            ::std::option::Option<::std::boxed::Box<MasterAuthorizedNetworksConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desiredMasterVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Kubernetes version to change the master to. Users may specify either explicit versions offered by Kubernetes Engine or version aliases, which have the following behavior: - \"latest\": picks the highest valid Kubernetes version - \"1.X\": picks the highest valid patch+gke.N patch in the 1.X version - \"1.X.Y\": picks the highest valid gke.N patch in the 1.X.Y version - \"1.X.Y-gke.N\": picks an explicit Kubernetes version - \"-\": picks the default Kubernetes version"]
        pub desired_master_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desiredMonitoringService")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The monitoring service the cluster should use to write metrics. Currently available options: * \"monitoring.googleapis.com/kubernetes\" - The Cloud Monitoring service with a Kubernetes-native resource model * `monitoring.googleapis.com` - The legacy Cloud Monitoring service (no longer available as of GKE 1.15). * `none` - No metrics will be exported from the cluster. If left as an empty string,`monitoring.googleapis.com/kubernetes` will be used for GKE 1.14+ or `monitoring.googleapis.com` for earlier versions."]
        pub desired_monitoring_service: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desiredNodePoolAutoscaling")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Autoscaler configuration for the node pool specified in desired_node_pool_id. If there is only one pool in the cluster and desired_node_pool_id is not provided then the change applies to that single node pool."]
        pub desired_node_pool_autoscaling:
            ::std::option::Option<::std::boxed::Box<NodePoolAutoscaling>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desiredNodePoolId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The node pool to be upgraded. This field is mandatory if \"desired_node_version\", \"desired_image_family\" or \"desired_node_pool_autoscaling\" is specified and there is more than one node pool on the cluster."]
        pub desired_node_pool_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desiredNodeVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Kubernetes version to change the nodes to (typically an upgrade). Users may specify either explicit versions offered by Kubernetes Engine or version aliases, which have the following behavior: - \"latest\": picks the highest valid Kubernetes version - \"1.X\": picks the highest valid patch+gke.N patch in the 1.X version - \"1.X.Y\": picks the highest valid gke.N patch in the 1.X.Y version - \"1.X.Y-gke.N\": picks an explicit Kubernetes version - \"-\": picks the Kubernetes master version"]
        pub desired_node_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desiredNotificationConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The desired notification configuration."]
        pub desired_notification_config:
            ::std::option::Option<::std::boxed::Box<NotificationConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desiredPrivateClusterConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The desired private cluster configuration."]
        pub desired_private_cluster_config:
            ::std::option::Option<::std::boxed::Box<PrivateClusterConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desiredPrivateIpv6GoogleAccess")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The desired state of IPv6 connectivity to Google Services."]
        pub desired_private_ipv6_google_access:
            ::std::option::Option<ClusterUpdateDesiredPrivateIpv6GoogleAccessEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desiredReleaseChannel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The desired release channel configuration."]
        pub desired_release_channel: ::std::option::Option<::std::boxed::Box<ReleaseChannel>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desiredResourceUsageExportConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The desired configuration for exporting resource usage."]
        pub desired_resource_usage_export_config:
            ::std::option::Option<::std::boxed::Box<ResourceUsageExportConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desiredShieldedNodes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for Shielded Nodes."]
        pub desired_shielded_nodes: ::std::option::Option<::std::boxed::Box<ShieldedNodes>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desiredVerticalPodAutoscaling")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cluster-level Vertical Pod Autoscaling configuration."]
        pub desired_vertical_pod_autoscaling:
            ::std::option::Option<::std::boxed::Box<VerticalPodAutoscaling>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desiredWorkloadIdentityConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for Workload Identity."]
        pub desired_workload_identity_config:
            ::std::option::Option<::std::boxed::Box<WorkloadIdentityConfig>>,
    }
    impl ClusterUpdate {
        pub fn builder() -> ClusterUpdateBuilder {
            ClusterUpdateBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The desired state of IPv6 connectivity to Google Services."]
    pub enum ClusterUpdateDesiredPrivateIpv6GoogleAccessEnum {
        #[serde(rename = "PRIVATE_IPV6_GOOGLE_ACCESS_UNSPECIFIED")]
        #[doc = "Default value. Same as DISABLED"]
        PrivateIpv6GoogleAccessUnspecified,
        #[serde(rename = "PRIVATE_IPV6_GOOGLE_ACCESS_DISABLED")]
        #[doc = "No private access to or from Google Services"]
        PrivateIpv6GoogleAccessDisabled,
        #[serde(rename = "PRIVATE_IPV6_GOOGLE_ACCESS_TO_GOOGLE")]
        #[doc = "Enables private IPv6 access to Google Services from GKE"]
        PrivateIpv6GoogleAccessToGoogle,
        #[serde(rename = "PRIVATE_IPV6_GOOGLE_ACCESS_BIDIRECTIONAL")]
        #[doc = "Enables priate IPv6 access to and from Google Services"]
        PrivateIpv6GoogleAccessBidirectional,
    }
    impl ::std::default::Default for ClusterUpdateDesiredPrivateIpv6GoogleAccessEnum {
        fn default() -> Self {
            Self::PrivateIpv6GoogleAccessUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "CompleteIPRotationRequest moves the cluster master back into single-IP mode."]
    pub struct CompleteIpRotationRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the cluster. This field has been deprecated and replaced by the name field."]
        pub cluster_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name (project, location, cluster id) of the cluster to complete IP rotation. Specified in the format `projects/*/locations/*/clusters/*`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://developers.google.com/console/help/new/#projectnumber). This field has been deprecated and replaced by the name field."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl CompleteIpRotationRequest {
        pub fn builder() -> CompleteIpRotationRequestBuilder {
            CompleteIpRotationRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration options for the Config Connector add-on."]
    pub struct ConfigConnectorConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether Cloud Connector is enabled for this cluster."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl ConfigConnectorConfig {
        pub fn builder() -> ConfigConnectorConfigBuilder {
            ConfigConnectorConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parameters for controlling consumption metering."]
    pub struct ConsumptionMeteringConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to enable consumption metering for this cluster. If enabled, a second BigQuery table will be created to hold resource consumption records."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl ConsumptionMeteringConfig {
        pub fn builder() -> ConsumptionMeteringConfigBuilder {
            ConsumptionMeteringConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "CreateClusterRequest creates a cluster."]
    pub struct CreateClusterRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cluster")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A [cluster resource](https://cloud.google.com/container-engine/reference/rest/v1/projects.locations.clusters)"]
        pub cluster: ::std::option::Option<::std::boxed::Box<Cluster>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The parent (project and location) where the cluster will be created. Specified in the format `projects/*/locations/*`."]
        pub parent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the parent field."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl CreateClusterRequest {
        pub fn builder() -> CreateClusterRequestBuilder {
            CreateClusterRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "CreateNodePoolRequest creates a node pool for a cluster."]
    pub struct CreateNodePoolRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the cluster. This field has been deprecated and replaced by the parent field."]
        pub cluster_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodePool")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The node pool to create."]
        pub node_pool: ::std::option::Option<::std::boxed::Box<NodePool>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The parent (project, location, cluster id) where the node pool will be created. Specified in the format `projects/*/locations/*/clusters/*`."]
        pub parent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://developers.google.com/console/help/new/#projectnumber). This field has been deprecated and replaced by the parent field."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl CreateNodePoolRequest {
        pub fn builder() -> CreateNodePoolRequestBuilder {
            CreateNodePoolRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Time window specified for daily maintenance operations."]
    pub struct DailyMaintenanceWindow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] Duration of the time window, automatically chosen to be smallest possible in the given scenario. Duration will be in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) format \"PTnHnMnS\"."]
        pub duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time within the maintenance window to start the maintenance operations. Time format should be in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) format \"HH:MM\", where HH : [00-23] and MM : [00-59] GMT."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl DailyMaintenanceWindow {
        pub fn builder() -> DailyMaintenanceWindowBuilder {
            DailyMaintenanceWindowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration of etcd encryption."]
    pub struct DatabaseEncryption {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of CloudKMS key to use for the encryption of secrets in etcd. Ex. projects/my-project/locations/global/keyRings/my-ring/cryptoKeys/my-key"]
        pub key_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Denotes the state of etcd encryption."]
        pub state: ::std::option::Option<DatabaseEncryptionStateEnum>,
    }
    impl DatabaseEncryption {
        pub fn builder() -> DatabaseEncryptionBuilder {
            DatabaseEncryptionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Denotes the state of etcd encryption."]
    pub enum DatabaseEncryptionStateEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Should never be set"]
        Unknown,
        #[serde(rename = "ENCRYPTED")]
        #[doc = "Secrets in etcd are encrypted."]
        Encrypted,
        #[serde(rename = "DECRYPTED")]
        #[doc = "Secrets in etcd are stored in plain text (at etcd level) - this is unrelated to Compute Engine level full disk encryption."]
        Decrypted,
    }
    impl ::std::default::Default for DatabaseEncryptionStateEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "DefaultSnatStatus contains the desired state of whether default sNAT should be disabled on the cluster."]
    pub struct DefaultSnatStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Disables cluster default sNAT rules."]
        pub disabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl DefaultSnatStatus {
        pub fn builder() -> DefaultSnatStatusBuilder {
            DefaultSnatStatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for NodeLocal DNSCache"]
    pub struct DnsCacheConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether NodeLocal DNSCache is enabled for this cluster."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl DnsCacheConfig {
        pub fn builder() -> DnsCacheConfigBuilder {
            DnsCacheConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
    pub struct Empty {}
    impl Empty {
        pub fn builder() -> EmptyBuilder {
            EmptyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for the Compute Engine PD CSI driver."]
    pub struct GcePersistentDiskCsiDriverConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the Compute Engine PD CSI driver is enabled for this cluster."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl GcePersistentDiskCsiDriverConfig {
        pub fn builder() -> GcePersistentDiskCsiDriverConfigBuilder {
            GcePersistentDiskCsiDriverConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "GetJSONWebKeysResponse is a valid JSON Web Key Set as specififed in rfc 7517"]
    pub struct GetJsonWebKeysResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cacheHeader")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OnePlatform automatically extracts this field and uses it to set the HTTP Cache-Control header."]
        pub cache_header: ::std::option::Option<::std::boxed::Box<HttpCacheControlResponseHeader>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keys")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The public component of the keys used by the cluster to sign token requests."]
        pub keys: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Jwk>>>,
    }
    impl GetJsonWebKeysResponse {
        pub fn builder() -> GetJsonWebKeysResponseBuilder {
            GetJsonWebKeysResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "GetOpenIDConfigResponse is an OIDC discovery document for the cluster. See the OpenID Connect Discovery 1.0 specification for details."]
    pub struct GetOpenIdConfigResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cacheHeader")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OnePlatform automatically extracts this field and uses it to set the HTTP Cache-Control header."]
        pub cache_header: ::std::option::Option<::std::boxed::Box<HttpCacheControlResponseHeader>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "claims_supported")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Supported claims."]
        pub claims_supported: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "grant_types")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Supported grant types."]
        pub grant_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id_token_signing_alg_values_supported")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "supported ID Token signing Algorithms."]
        pub id_token_signing_alg_values_supported:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "issuer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OIDC Issuer."]
        pub issuer: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jwks_uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "JSON Web Key uri."]
        pub jwks_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "response_types_supported")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Supported response types."]
        pub response_types_supported: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subject_types_supported")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Supported subject types."]
        pub subject_types_supported: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GetOpenIdConfigResponse {
        pub fn builder() -> GetOpenIdConfigResponseBuilder {
            GetOpenIdConfigResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration options for the horizontal pod autoscaling feature, which increases or decreases the number of replica pods a replication controller has based on the resource usage of the existing pods."]
    pub struct HorizontalPodAutoscaling {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the Horizontal Pod Autoscaling feature is enabled in the cluster. When enabled, it ensures that metrics are collected into Stackdriver Monitoring."]
        pub disabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl HorizontalPodAutoscaling {
        pub fn builder() -> HorizontalPodAutoscalingBuilder {
            HorizontalPodAutoscalingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "RFC-2616: cache control support"]
    pub struct HttpCacheControlResponseHeader {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "age")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "14.6 response cache age, in seconds since the response is generated"]
        pub age: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "directive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "14.9 request and response directives"]
        pub directive: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expires")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "14.21 response cache expires, in RFC 1123 date format"]
        pub expires: ::std::option::Option<::std::string::String>,
    }
    impl HttpCacheControlResponseHeader {
        pub fn builder() -> HttpCacheControlResponseHeaderBuilder {
            HttpCacheControlResponseHeaderBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration options for the HTTP (L7) load balancing controller addon, which makes it easy to set up HTTP load balancers for services in a cluster."]
    pub struct HttpLoadBalancing {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the HTTP Load Balancing controller is enabled in the cluster. When enabled, it runs a small pod in the cluster that manages the load balancers."]
        pub disabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl HttpLoadBalancing {
        pub fn builder() -> HttpLoadBalancingBuilder {
            HttpLoadBalancingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for controlling how IPs are allocated in the cluster."]
    pub struct IpAllocationPolicy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterIpv4Cidr")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is deprecated, use cluster_ipv4_cidr_block."]
        pub cluster_ipv4_cidr: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterIpv4CidrBlock")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IP address range for the cluster pod IPs. If this field is set, then `cluster.cluster_ipv4_cidr` must be left blank. This field is only applicable when `use_ip_aliases` is true. Set to blank to have a range chosen with the default size. Set to /netmask (e.g. `/14`) to have a range chosen with a specific netmask. Set to a [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g. `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range to use."]
        pub cluster_ipv4_cidr_block: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterSecondaryRangeName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the secondary range to be used for the cluster CIDR block. The secondary range will be used for pod IP addresses. This must be an existing secondary range associated with the cluster subnetwork. This field is only applicable with use_ip_aliases is true and create_subnetwork is false."]
        pub cluster_secondary_range_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createSubnetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether a new subnetwork will be created automatically for the cluster. This field is only applicable when `use_ip_aliases` is true."]
        pub create_subnetwork: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodeIpv4Cidr")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is deprecated, use node_ipv4_cidr_block."]
        pub node_ipv4_cidr: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodeIpv4CidrBlock")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IP address range of the instance IPs in this cluster. This is applicable only if `create_subnetwork` is true. Set to blank to have a range chosen with the default size. Set to /netmask (e.g. `/14`) to have a range chosen with a specific netmask. Set to a [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g. `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range to use."]
        pub node_ipv4_cidr_block: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "servicesIpv4Cidr")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is deprecated, use services_ipv4_cidr_block."]
        pub services_ipv4_cidr: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "servicesIpv4CidrBlock")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IP address range of the services IPs in this cluster. If blank, a range will be automatically chosen with the default size. This field is only applicable when `use_ip_aliases` is true. Set to blank to have a range chosen with the default size. Set to /netmask (e.g. `/14`) to have a range chosen with a specific netmask. Set to a [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g. `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range to use."]
        pub services_ipv4_cidr_block: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "servicesSecondaryRangeName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the secondary range to be used as for the services CIDR block. The secondary range will be used for service ClusterIPs. This must be an existing secondary range associated with the cluster subnetwork. This field is only applicable with use_ip_aliases is true and create_subnetwork is false."]
        pub services_secondary_range_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subnetworkName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A custom subnetwork name to be used if `create_subnetwork` is true. If this field is empty, then an automatic name will be chosen for the new subnetwork."]
        pub subnetwork_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tpuIpv4CidrBlock")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IP address range of the Cloud TPUs in this cluster. If unspecified, a range will be automatically chosen with the default size. This field is only applicable when `use_ip_aliases` is true. If unspecified, the range will use the default size. Set to /netmask (e.g. `/14`) to have a range chosen with a specific netmask. Set to a [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g. `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range to use."]
        pub tpu_ipv4_cidr_block: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "useIpAliases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether alias IPs will be used for pod IPs in the cluster. This is used in conjunction with use_routes. It cannot be true if use_routes is true. If both use_ip_aliases and use_routes are false, then the server picks the default IP allocation mode"]
        pub use_ip_aliases: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "useRoutes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether routes will be used for pod IPs in the cluster. This is used in conjunction with use_ip_aliases. It cannot be true if use_ip_aliases is true. If both use_ip_aliases and use_routes are false, then the server picks the default IP allocation mode"]
        pub use_routes: ::std::option::Option<::std::primitive::bool>,
    }
    impl IpAllocationPolicy {
        pub fn builder() -> IpAllocationPolicyBuilder {
            IpAllocationPolicyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "IntraNodeVisibilityConfig contains the desired config of the intra-node visibility on this cluster."]
    pub struct IntraNodeVisibilityConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enables intra node visibility for this cluster."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl IntraNodeVisibilityConfig {
        pub fn builder() -> IntraNodeVisibilityConfigBuilder {
            IntraNodeVisibilityConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Jwk is a JSON Web Key as specified in RFC 7517"]
    pub struct Jwk {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alg")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Algorithm."]
        pub alg: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "crv")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Used for ECDSA keys."]
        pub crv: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "e")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Used for RSA keys."]
        pub e: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Key ID."]
        pub kid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kty")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Key Type."]
        pub kty: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "n")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Used for RSA keys."]
        pub n: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "use")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Permitted uses for the public keys."]
        pub _use: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "x")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Used for ECDSA keys."]
        pub x: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "y")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Used for ECDSA keys."]
        pub y: ::std::option::Option<::std::string::String>,
    }
    impl Jwk {
        pub fn builder() -> JwkBuilder {
            JwkBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for the Kubernetes Dashboard."]
    pub struct KubernetesDashboard {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the Kubernetes Dashboard is enabled for this cluster."]
        pub disabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl KubernetesDashboard {
        pub fn builder() -> KubernetesDashboardBuilder {
            KubernetesDashboardBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for the legacy Attribute Based Access Control authorization mode."]
    pub struct LegacyAbac {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the ABAC authorizer is enabled for this cluster. When enabled, identities in the system, including service accounts, nodes, and controllers, will have statically granted permissions beyond those provided by the RBAC configuration or IAM."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl LegacyAbac {
        pub fn builder() -> LegacyAbacBuilder {
            LegacyAbacBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parameters that can be configured on Linux nodes."]
    pub struct LinuxNodeConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sysctls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Linux kernel parameters to be applied to the nodes and all pods running on the nodes. The following parameters are supported. net.core.netdev_max_backlog net.core.rmem_max net.core.wmem_default net.core.wmem_max net.core.optmem_max net.core.somaxconn net.ipv4.tcp_rmem net.ipv4.tcp_wmem net.ipv4.tcp_tw_reuse"]
        pub sysctls:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl LinuxNodeConfig {
        pub fn builder() -> LinuxNodeConfigBuilder {
            LinuxNodeConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "ListClustersResponse is the result of ListClustersRequest."]
    pub struct ListClustersResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of clusters in the project in the specified zone, or across all ones."]
        pub clusters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Cluster>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "missingZones")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If any zones are listed here, the list of clusters returned may be missing those zones."]
        pub missing_zones: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ListClustersResponse {
        pub fn builder() -> ListClustersResponseBuilder {
            ListClustersResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "ListNodePoolsResponse is the result of ListNodePoolsRequest."]
    pub struct ListNodePoolsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodePools")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of node pools for a cluster."]
        pub node_pools: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NodePool>>>,
    }
    impl ListNodePoolsResponse {
        pub fn builder() -> ListNodePoolsResponseBuilder {
            ListNodePoolsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "ListOperationsResponse is the result of ListOperationsRequest."]
    pub struct ListOperationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "missingZones")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If any zones are listed here, the list of operations returned may be missing the operations from those zones."]
        pub missing_zones: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of operations in the project in the specified zone."]
        pub operations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Operation>>>,
    }
    impl ListOperationsResponse {
        pub fn builder() -> ListOperationsResponseBuilder {
            ListOperationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "ListUsableSubnetworksResponse is the response of ListUsableSubnetworksRequest."]
    pub struct ListUsableSubnetworksResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This token allows you to get the next page of results for list requests. If the number of results is larger than `page_size`, use the `next_page_token` as a value for the query parameter `page_token` in the next request. The value will become empty when there are no more pages."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subnetworks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of usable subnetworks in the specified network project."]
        pub subnetworks:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UsableSubnetwork>>>,
    }
    impl ListUsableSubnetworksResponse {
        pub fn builder() -> ListUsableSubnetworksResponseBuilder {
            ListUsableSubnetworksResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "MaintenancePolicy defines the maintenance policy to be used for the cluster."]
    pub struct MaintenancePolicy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A hash identifying the version of this policy, so that updates to fields of the policy won't accidentally undo intermediate changes (and so that users of the API unaware of some fields won't accidentally remove other fields). Make a `get()` request to the cluster to get the current resource version and include it with requests to set the policy."]
        pub resource_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "window")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the maintenance window in which maintenance may be performed."]
        pub window: ::std::option::Option<::std::boxed::Box<MaintenanceWindow>>,
    }
    impl MaintenancePolicy {
        pub fn builder() -> MaintenancePolicyBuilder {
            MaintenancePolicyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "MaintenanceWindow defines the maintenance window to be used for the cluster."]
    pub struct MaintenanceWindow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dailyMaintenanceWindow")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DailyMaintenanceWindow specifies a daily maintenance operation window."]
        pub daily_maintenance_window:
            ::std::option::Option<::std::boxed::Box<DailyMaintenanceWindow>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maintenanceExclusions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Exceptions to maintenance window. Non-emergency maintenance should not occur in these windows."]
        pub maintenance_exclusions: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<TimeWindow>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recurringWindow")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "RecurringWindow specifies some number of recurring time periods for maintenance to occur. The time windows may be overlapping. If no maintenance windows are set, maintenance can occur at any time."]
        pub recurring_window: ::std::option::Option<::std::boxed::Box<RecurringTimeWindow>>,
    }
    impl MaintenanceWindow {
        pub fn builder() -> MaintenanceWindowBuilder {
            MaintenanceWindowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The authentication information for accessing the master endpoint. Authentication can be done using HTTP basic auth or using client certificates."]
    pub struct MasterAuth {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientCertificate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] Base64-encoded public certificate used by clients to authenticate to the cluster endpoint."]
        pub client_certificate: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientCertificateConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for client certificate authentication on the cluster. For clusters before v1.12, if no configuration is specified, a client certificate is issued."]
        pub client_certificate_config:
            ::std::option::Option<::std::boxed::Box<ClientCertificateConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] Base64-encoded private key used by clients to authenticate to the cluster endpoint."]
        pub client_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterCaCertificate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] Base64-encoded public certificate that is the root of trust for the cluster."]
        pub cluster_ca_certificate: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "password")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The password to use for HTTP basic authentication to the master endpoint. Because the master endpoint is open to the Internet, you should create a strong password. If a password is provided for cluster creation, username must be non-empty. Warning: basic authentication is deprecated, and will be removed in GKE control plane versions 1.19 and newer. For a list of recommended authentication methods, see: https://cloud.google.com/kubernetes-engine/docs/how-to/api-server-authentication"]
        pub password: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "username")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The username to use for HTTP basic authentication to the master endpoint. For clusters v1.6.0 and later, basic authentication can be disabled by leaving username unspecified (or setting it to the empty string). Warning: basic authentication is deprecated, and will be removed in GKE control plane versions 1.19 and newer. For a list of recommended authentication methods, see: https://cloud.google.com/kubernetes-engine/docs/how-to/api-server-authentication"]
        pub username: ::std::option::Option<::std::string::String>,
    }
    impl MasterAuth {
        pub fn builder() -> MasterAuthBuilder {
            MasterAuthBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration options for the master authorized networks feature. Enabled master authorized networks will disallow all external traffic to access Kubernetes master through HTTPS except traffic from the given CIDR blocks, Google Compute Engine Public IPs and Google Prod IPs."]
    pub struct MasterAuthorizedNetworksConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cidrBlocks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "cidr_blocks define up to 50 external networks that could access Kubernetes master through HTTPS."]
        pub cidr_blocks: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CidrBlock>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not master authorized networks is enabled."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl MasterAuthorizedNetworksConfig {
        pub fn builder() -> MasterAuthorizedNetworksConfigBuilder {
            MasterAuthorizedNetworksConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Constraints applied to pods."]
    pub struct MaxPodsConstraint {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxPodsPerNode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Constraint enforced on the max num of pods per node."]
        pub max_pods_per_node: ::std::option::Option<::std::string::String>,
    }
    impl MaxPodsConstraint {
        pub fn builder() -> MaxPodsConstraintBuilder {
            MaxPodsConstraintBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Progress metric is (string, int|float|string) pair."]
    pub struct Metric {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "doubleValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For metrics with floating point value."]
        pub double_value: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For metrics with integer value."]
        pub int_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Metric name, e.g., \"nodes total\", \"percent done\"."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stringValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For metrics with custom values (ratios, visual progress, etc.)."]
        pub string_value: ::std::option::Option<::std::string::String>,
    }
    impl Metric {
        pub fn builder() -> MetricBuilder {
            MetricBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "NetworkConfig reports the relative names of network & subnetwork."]
    pub struct NetworkConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultSnatStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the cluster disables default in-node sNAT rules. In-node sNAT rules will be disabled when default_snat_status is disabled. When disabled is set to false, default IP masquerade rules will be applied to the nodes to prevent sNAT on cluster internal traffic."]
        pub default_snat_status: ::std::option::Option<::std::boxed::Box<DefaultSnatStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableIntraNodeVisibility")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether Intra-node visibility is enabled for this cluster. This makes same node pod to pod traffic visible for VPC network."]
        pub enable_intra_node_visibility: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "network")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The relative name of the Google Compute Engine network(https://cloud.google.com/compute/docs/networks-and-firewalls#networks) to which the cluster is connected. Example: projects/my-project/global/networks/my-network"]
        pub network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "privateIpv6GoogleAccess")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The desired state of IPv6 connectivity to Google Services. By default, no private IPv6 access to or from Google Services (all access will be via IPv4)"]
        pub private_ipv6_google_access:
            ::std::option::Option<NetworkConfigPrivateIpv6GoogleAccessEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subnetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The relative name of the Google Compute Engine [subnetwork](https://cloud.google.com/compute/docs/vpc) to which the cluster is connected. Example: projects/my-project/regions/us-central1/subnetworks/my-subnet"]
        pub subnetwork: ::std::option::Option<::std::string::String>,
    }
    impl NetworkConfig {
        pub fn builder() -> NetworkConfigBuilder {
            NetworkConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The desired state of IPv6 connectivity to Google Services. By default, no private IPv6 access to or from Google Services (all access will be via IPv4)"]
    pub enum NetworkConfigPrivateIpv6GoogleAccessEnum {
        #[serde(rename = "PRIVATE_IPV6_GOOGLE_ACCESS_UNSPECIFIED")]
        #[doc = "Default value. Same as DISABLED"]
        PrivateIpv6GoogleAccessUnspecified,
        #[serde(rename = "PRIVATE_IPV6_GOOGLE_ACCESS_DISABLED")]
        #[doc = "No private access to or from Google Services"]
        PrivateIpv6GoogleAccessDisabled,
        #[serde(rename = "PRIVATE_IPV6_GOOGLE_ACCESS_TO_GOOGLE")]
        #[doc = "Enables private IPv6 access to Google Services from GKE"]
        PrivateIpv6GoogleAccessToGoogle,
        #[serde(rename = "PRIVATE_IPV6_GOOGLE_ACCESS_BIDIRECTIONAL")]
        #[doc = "Enables priate IPv6 access to and from Google Services"]
        PrivateIpv6GoogleAccessBidirectional,
    }
    impl ::std::default::Default for NetworkConfigPrivateIpv6GoogleAccessEnum {
        fn default() -> Self {
            Self::PrivateIpv6GoogleAccessUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration options for the NetworkPolicy feature. https://kubernetes.io/docs/concepts/services-networking/networkpolicies/"]
    pub struct NetworkPolicy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether network policy is enabled on the cluster."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provider")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The selected network policy provider."]
        pub provider: ::std::option::Option<NetworkPolicyProviderEnum>,
    }
    impl NetworkPolicy {
        pub fn builder() -> NetworkPolicyBuilder {
            NetworkPolicyBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The selected network policy provider."]
    pub enum NetworkPolicyProviderEnum {
        #[serde(rename = "PROVIDER_UNSPECIFIED")]
        #[doc = "Not set"]
        ProviderUnspecified,
        #[serde(rename = "CALICO")]
        #[doc = "Tigera (Calico Felix)."]
        Calico,
    }
    impl ::std::default::Default for NetworkPolicyProviderEnum {
        fn default() -> Self {
            Self::ProviderUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for NetworkPolicy. This only tracks whether the addon is enabled or not on the Master, it does not track whether network policy is enabled for the nodes."]
    pub struct NetworkPolicyConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether NetworkPolicy is enabled for this cluster."]
        pub disabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl NetworkPolicyConfig {
        pub fn builder() -> NetworkPolicyConfigBuilder {
            NetworkPolicyConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parameters that describe the nodes in a cluster."]
    pub struct NodeConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accelerators")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of hardware accelerators to be attached to each node. See https://cloud.google.com/compute/docs/gpus for more information about support for GPUs."]
        pub accelerators:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AcceleratorConfig>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bootDiskKmsKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = " The Customer Managed Encryption Key used to encrypt the boot disk attached to each node in the node pool. This should be of the form projects/[KEY_PROJECT_ID]/locations/[LOCATION]/keyRings/[RING_NAME]/cryptoKeys/[KEY_NAME]. For more information about protecting resources with Cloud KMS Keys please see: https://cloud.google.com/compute/docs/disks/customer-managed-encryption"]
        pub boot_disk_kms_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diskSizeGb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Size of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB. If unspecified, the default disk size is 100GB."]
        pub disk_size_gb: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diskType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of the disk attached to each node (e.g. 'pd-standard', 'pd-ssd' or 'pd-balanced') If unspecified, the default disk type is 'pd-standard'"]
        pub disk_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The image type to use for this node. Note that for a given image type, the latest version of it will be used."]
        pub image_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kubeletConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Node kubelet configs."]
        pub kubelet_config: ::std::option::Option<::std::boxed::Box<NodeKubeletConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The map of Kubernetes labels (key/value pairs) to be applied to each node. These will added in addition to any default label(s) that Kubernetes may apply to the node. In case of conflict in label keys, the applied set may differ depending on the Kubernetes version -- it's best to assume the behavior is undefined and conflicts should be avoided. For more information, including usage and the valid values, see: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/"]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linuxNodeConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parameters that can be configured on Linux nodes."]
        pub linux_node_config: ::std::option::Option<::std::boxed::Box<LinuxNodeConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localSsdCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of local SSD disks to be attached to the node. The limit for this value is dependent upon the maximum number of disks available on a machine per zone. See: https://cloud.google.com/compute/docs/disks/local-ssd for more information."]
        pub local_ssd_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "machineType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of a Google Compute Engine [machine type](https://cloud.google.com/compute/docs/machine-types) If unspecified, the default machine type is `e2-medium`."]
        pub machine_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metadata key/value pairs assigned to instances in the cluster. Keys must conform to the regexp `[a-zA-Z0-9-_]+` and be less than 128 bytes in length. These are reflected as part of a URL in the metadata server. Additionally, to avoid ambiguity, keys must not conflict with any other metadata keys for the project or be one of the reserved keys: - \"cluster-location\" - \"cluster-name\" - \"cluster-uid\" - \"configure-sh\" - \"containerd-configure-sh\" - \"enable-os-login\" - \"gci-ensure-gke-docker\" - \"gci-metrics-enabled\" - \"gci-update-strategy\" - \"instance-template\" - \"kube-env\" - \"startup-script\" - \"user-data\" - \"disable-address-manager\" - \"windows-startup-script-ps1\" - \"common-psm1\" - \"k8s-node-setup-psm1\" - \"install-ssh-psm1\" - \"user-profile-psm1\" The following keys are reserved for Windows nodes: - \"serial-port-logging-enable\" Values are free-form strings, and only have meaning as interpreted by the image running in the instance. The only restriction placed on them is that each value's size must be less than or equal to 32 KB. The total size of all keys and values must be less than 512 KB."]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minCpuPlatform")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum CPU platform to be used by this instance. The instance may be scheduled on the specified or newer CPU platform. Applicable values are the friendly names of CPU platforms, such as `minCpuPlatform: \"Intel Haswell\"` or `minCpuPlatform: \"Intel Sandy Bridge\"`. For more information, read [how to specify min CPU platform](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform)"]
        pub min_cpu_platform: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodeGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Setting this field will assign instances of this pool to run on the specified node group. This is useful for running workloads on [sole tenant nodes](https://cloud.google.com/compute/docs/nodes/sole-tenant-nodes)."]
        pub node_group: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oauthScopes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of Google API scopes to be made available on all of the node VMs under the \"default\" service account. The following scopes are recommended, but not required, and by default are not included: * `https://www.googleapis.com/auth/compute` is required for mounting persistent storage on your nodes. * `https://www.googleapis.com/auth/devstorage.read_only` is required for communicating with **gcr.io** (the [Google Container Registry](https://cloud.google.com/container-registry/)). If unspecified, no scopes are added, unless Cloud Logging or Cloud Monitoring are enabled, in which case their required scopes will be added."]
        pub oauth_scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "preemptible")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the nodes are created as preemptible VM instances. See: https://cloud.google.com/compute/docs/instances/preemptible for more information about preemptible VM instances."]
        pub preemptible: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reservationAffinity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The optional reservation affinity. Setting this field will apply the specified [Zonal Compute Reservation](https://cloud.google.com/compute/docs/instances/reserving-zonal-resources) to this node pool."]
        pub reservation_affinity: ::std::option::Option<::std::boxed::Box<ReservationAffinity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sandboxConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sandbox configuration for this node."]
        pub sandbox_config: ::std::option::Option<::std::boxed::Box<SandboxConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceAccount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Google Cloud Platform Service Account to be used by the node VMs. Specify the email address of the Service Account; otherwise, if no Service Account is specified, the \"default\" service account is used."]
        pub service_account: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shieldedInstanceConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Shielded Instance options."]
        pub shielded_instance_config:
            ::std::option::Option<::std::boxed::Box<ShieldedInstanceConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of instance tags applied to all nodes. Tags are used to identify valid sources or targets for network firewalls and are specified by the client during cluster or node pool creation. Each tag within the list must comply with RFC1035."]
        pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "taints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of kubernetes taints to be applied to each node. For more information, including usage and the valid values, see: https://kubernetes.io/docs/concepts/configuration/taint-and-toleration/"]
        pub taints: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NodeTaint>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workloadMetadataConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The workload metadata configuration for this node."]
        pub workload_metadata_config:
            ::std::option::Option<::std::boxed::Box<WorkloadMetadataConfig>>,
    }
    impl NodeConfig {
        pub fn builder() -> NodeConfigBuilder {
            NodeConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Node kubelet configs."]
    pub struct NodeKubeletConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpuCfsQuota")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enable CPU CFS quota enforcement for containers that specify CPU limits. This option is enabled by default which makes kubelet use CFS quota (https://www.kernel.org/doc/Documentation/scheduler/sched-bwc.txt) to enforce container CPU limits. Otherwise, CPU limits will not be enforced at all. Disable this option to mitigate CPU throttling problems while still having your pods to be in Guaranteed QoS class by specifying the CPU limits. The default value is 'true' if unspecified."]
        pub cpu_cfs_quota: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpuCfsQuotaPeriod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set the CPU CFS quota period value 'cpu.cfs_period_us'. The string must be a sequence of decimal numbers, each with optional fraction and a unit suffix, such as \"300ms\". Valid time units are \"ns\", \"us\" (or \"µs\"), \"ms\", \"s\", \"m\", \"h\". The value must be a positive duration."]
        pub cpu_cfs_quota_period: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpuManagerPolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Control the CPU management policy on the node. See https://kubernetes.io/docs/tasks/administer-cluster/cpu-management-policies/ The following values are allowed. - \"none\": the default, which represents the existing scheduling behavior. - \"static\": allows pods with certain resource characteristics to be granted increased CPU affinity and exclusivity on the node. The default value is 'none' if unspecified."]
        pub cpu_manager_policy: ::std::option::Option<::std::string::String>,
    }
    impl NodeKubeletConfig {
        pub fn builder() -> NodeKubeletConfigBuilder {
            NodeKubeletConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "NodeManagement defines the set of node management services turned on for the node pool."]
    pub struct NodeManagement {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoRepair")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A flag that specifies whether the node auto-repair is enabled for the node pool. If enabled, the nodes in this node pool will be monitored and, if they fail health checks too many times, an automatic repair action will be triggered."]
        pub auto_repair: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoUpgrade")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A flag that specifies whether node auto-upgrade is enabled for the node pool. If enabled, node auto-upgrade helps keep the nodes in your node pool up to date with the latest release version of Kubernetes."]
        pub auto_upgrade: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "upgradeOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the Auto Upgrade knobs for the node pool."]
        pub upgrade_options: ::std::option::Option<::std::boxed::Box<AutoUpgradeOptions>>,
    }
    impl NodeManagement {
        pub fn builder() -> NodeManagementBuilder {
            NodeManagementBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "NodePool contains the name and configuration for a cluster's node pool. Node pools are a set of nodes (i.e. VM's), with a common configuration and specification, under the control of the cluster master. They may have a set of Kubernetes labels applied to them, which may be used to reference them during pod scheduling. They may also be resized up or down, to accommodate the workload."]
    pub struct NodePool {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoscaling")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Autoscaler configuration for this NodePool. Autoscaler is enabled only if a valid configuration is present."]
        pub autoscaling: ::std::option::Option<::std::boxed::Box<NodePoolAutoscaling>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conditions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Which conditions caused the current node pool state."]
        pub conditions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StatusCondition>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "config")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The node configuration of the pool."]
        pub config: ::std::option::Option<::std::boxed::Box<NodeConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "initialNodeCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The initial node count for the pool. You must ensure that your Compute Engine [resource quota](https://cloud.google.com/compute/quotas) is sufficient for this number of instances. You must also have available firewall and routes quota."]
        pub initial_node_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instanceGroupUrls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The resource URLs of the [managed instance groups](https://cloud.google.com/compute/docs/instance-groups/creating-groups-of-managed-instances) associated with this node pool."]
        pub instance_group_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available) in which the NodePool's nodes should be located. If this value is unspecified during node pool creation, the [Cluster.Locations](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters#Cluster.FIELDS.locations) value will be used, instead. Warning: changing node pool locations will result in nodes being added and/or removed."]
        pub locations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "management")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "NodeManagement configuration for this NodePool."]
        pub management: ::std::option::Option<::std::boxed::Box<NodeManagement>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxPodsConstraint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The constraint on the maximum number of pods that can be run simultaneously on a node in the node pool."]
        pub max_pods_constraint: ::std::option::Option<::std::boxed::Box<MaxPodsConstraint>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the node pool."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "podIpv4CidrSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The pod CIDR block size per node in this node pool."]
        pub pod_ipv4_cidr_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] Server-defined URL for the resource."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The status of the nodes in this pool instance."]
        pub status: ::std::option::Option<NodePoolStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statusMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] Deprecated. Use conditions instead. Additional information about the current status of this node pool instance, if available."]
        pub status_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "upgradeSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Upgrade settings control disruption and speed of the upgrade."]
        pub upgrade_settings: ::std::option::Option<::std::boxed::Box<UpgradeSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version of the Kubernetes of this node."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl NodePool {
        pub fn builder() -> NodePoolBuilder {
            NodePoolBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "[Output only] The status of the nodes in this pool instance."]
    pub enum NodePoolStatusEnum {
        #[serde(rename = "STATUS_UNSPECIFIED")]
        #[doc = "Not set."]
        StatusUnspecified,
        #[serde(rename = "PROVISIONING")]
        #[doc = "The PROVISIONING state indicates the node pool is being created."]
        Provisioning,
        #[serde(rename = "RUNNING")]
        #[doc = "The RUNNING state indicates the node pool has been created and is fully usable."]
        Running,
        #[serde(rename = "RUNNING_WITH_ERROR")]
        #[doc = "The RUNNING_WITH_ERROR state indicates the node pool has been created and is partially usable. Some error state has occurred and some functionality may be impaired. Customer may need to reissue a request or trigger a new update."]
        RunningWithError,
        #[serde(rename = "RECONCILING")]
        #[doc = "The RECONCILING state indicates that some work is actively being done on the node pool, such as upgrading node software. Details can be found in the `statusMessage` field."]
        Reconciling,
        #[serde(rename = "STOPPING")]
        #[doc = "The STOPPING state indicates the node pool is being deleted."]
        Stopping,
        #[serde(rename = "ERROR")]
        #[doc = "The ERROR state indicates the node pool may be unusable. Details can be found in the `statusMessage` field."]
        Error,
    }
    impl ::std::default::Default for NodePoolStatusEnum {
        fn default() -> Self {
            Self::StatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "NodePoolAutoscaling contains information required by cluster autoscaler to adjust the size of the node pool to the current cluster usage."]
    pub struct NodePoolAutoscaling {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoprovisioned")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Can this node pool be deleted automatically."]
        pub autoprovisioned: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Is autoscaling enabled for this node pool."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxNodeCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum number of nodes in the NodePool. Must be >= min_node_count. There has to enough quota to scale up the cluster."]
        pub max_node_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minNodeCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum number of nodes in the NodePool. Must be >= 1 and <= max_node_count."]
        pub min_node_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl NodePoolAutoscaling {
        pub fn builder() -> NodePoolAutoscalingBuilder {
            NodePoolAutoscalingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Kubernetes taint is comprised of three fields: key, value, and effect. Effect can only be one of three types: NoSchedule, PreferNoSchedule or NoExecute. See [here](https://kubernetes.io/docs/concepts/configuration/taint-and-toleration) for more information, including usage and the valid values."]
    pub struct NodeTaint {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "effect")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Effect for taint."]
        pub effect: ::std::option::Option<NodeTaintEffectEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Key for taint."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value for taint."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl NodeTaint {
        pub fn builder() -> NodeTaintBuilder {
            NodeTaintBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Effect for taint."]
    pub enum NodeTaintEffectEnum {
        #[serde(rename = "EFFECT_UNSPECIFIED")]
        #[doc = "Not set"]
        EffectUnspecified,
        #[serde(rename = "NO_SCHEDULE")]
        #[doc = "NoSchedule"]
        NoSchedule,
        #[serde(rename = "PREFER_NO_SCHEDULE")]
        #[doc = "PreferNoSchedule"]
        PreferNoSchedule,
        #[serde(rename = "NO_EXECUTE")]
        #[doc = "NoExecute"]
        NoExecute,
    }
    impl ::std::default::Default for NodeTaintEffectEnum {
        fn default() -> Self {
            Self::EffectUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "NotificationConfig is the configuration of notifications."]
    pub struct NotificationConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pubsub")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Notification config for Pub/Sub."]
        pub pubsub: ::std::option::Option<::std::boxed::Box<PubSub>>,
    }
    impl NotificationConfig {
        pub fn builder() -> NotificationConfigBuilder {
            NotificationConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This operation resource represents operations that may have happened or are happening on the cluster. All fields are output only."]
    pub struct Operation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterConditions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Which conditions caused the current cluster state."]
        pub cluster_conditions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StatusCondition>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detailed operation progress, if available."]
        pub detail: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The time the operation completed, in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) or [region](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) in which the cluster resides."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The server-assigned ID for the operation."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodepoolConditions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Which conditions caused the current node pool state."]
        pub nodepool_conditions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StatusCondition>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The operation type."]
        pub operation_type: ::std::option::Option<OperationOperationTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. [Output only] Progress information for an operation."]
        pub progress: ::std::option::Option<::std::boxed::Box<OperationProgress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Server-defined URL for the resource."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The time the operation started, in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current status of the operation."]
        pub status: ::std::option::Option<OperationStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statusMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. If an error has occurred, a textual description of the error."]
        pub status_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Server-defined URL for the target of the operation."]
        pub target_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the operation is taking place. This field is deprecated, use location instead."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl Operation {
        pub fn builder() -> OperationBuilder {
            OperationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The operation type."]
    pub enum OperationOperationTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "Not set."]
        TypeUnspecified,
        #[serde(rename = "CREATE_CLUSTER")]
        #[doc = "Cluster create."]
        CreateCluster,
        #[serde(rename = "DELETE_CLUSTER")]
        #[doc = "Cluster delete."]
        DeleteCluster,
        #[serde(rename = "UPGRADE_MASTER")]
        #[doc = "A master upgrade."]
        UpgradeMaster,
        #[serde(rename = "UPGRADE_NODES")]
        #[doc = "A node upgrade."]
        UpgradeNodes,
        #[serde(rename = "REPAIR_CLUSTER")]
        #[doc = "Cluster repair."]
        RepairCluster,
        #[serde(rename = "UPDATE_CLUSTER")]
        #[doc = "Cluster update."]
        UpdateCluster,
        #[serde(rename = "CREATE_NODE_POOL")]
        #[doc = "Node pool create."]
        CreateNodePool,
        #[serde(rename = "DELETE_NODE_POOL")]
        #[doc = "Node pool delete."]
        DeleteNodePool,
        #[serde(rename = "SET_NODE_POOL_MANAGEMENT")]
        #[doc = "Set node pool management."]
        SetNodePoolManagement,
        #[serde(rename = "AUTO_REPAIR_NODES")]
        #[doc = "Automatic node pool repair."]
        AutoRepairNodes,
        #[serde(rename = "AUTO_UPGRADE_NODES")]
        #[doc = "Automatic node upgrade."]
        AutoUpgradeNodes,
        #[serde(rename = "SET_LABELS")]
        #[doc = "Set labels."]
        SetLabels,
        #[serde(rename = "SET_MASTER_AUTH")]
        #[doc = "Set/generate master auth materials"]
        SetMasterAuth,
        #[serde(rename = "SET_NODE_POOL_SIZE")]
        #[doc = "Set node pool size."]
        SetNodePoolSize,
        #[serde(rename = "SET_NETWORK_POLICY")]
        #[doc = "Updates network policy for a cluster."]
        SetNetworkPolicy,
        #[serde(rename = "SET_MAINTENANCE_POLICY")]
        #[doc = "Set the maintenance policy."]
        SetMaintenancePolicy,
    }
    impl ::std::default::Default for OperationOperationTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The current status of the operation."]
    pub enum OperationStatusEnum {
        #[serde(rename = "STATUS_UNSPECIFIED")]
        #[doc = "Not set."]
        StatusUnspecified,
        #[serde(rename = "PENDING")]
        #[doc = "The operation has been created."]
        Pending,
        #[serde(rename = "RUNNING")]
        #[doc = "The operation is currently running."]
        Running,
        #[serde(rename = "DONE")]
        #[doc = "The operation is done, either cancelled or completed."]
        Done,
        #[serde(rename = "ABORTING")]
        #[doc = "The operation is aborting."]
        Aborting,
    }
    impl ::std::default::Default for OperationStatusEnum {
        fn default() -> Self {
            Self::StatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about operation (or operation stage) progress."]
    pub struct OperationProgress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Progress metric bundle, for example: metrics: [{name: \"nodes done\", int_value: 15}, {name: \"nodes total\", int_value: 32}] or metrics: [{name: \"progress\", double_value: 0.56}, {name: \"progress scale\", double_value: 1.0}]"]
        pub metrics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Metric>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A non-parameterized string describing an operation stage. Unset for single-stage operations."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Substages of an operation or a stage."]
        pub stages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OperationProgress>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of an operation stage. Unset for single-stage operations."]
        pub status: ::std::option::Option<OperationProgressStatusEnum>,
    }
    impl OperationProgress {
        pub fn builder() -> OperationProgressBuilder {
            OperationProgressBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Status of an operation stage. Unset for single-stage operations."]
    pub enum OperationProgressStatusEnum {
        #[serde(rename = "STATUS_UNSPECIFIED")]
        #[doc = "Not set."]
        StatusUnspecified,
        #[serde(rename = "PENDING")]
        #[doc = "The operation has been created."]
        Pending,
        #[serde(rename = "RUNNING")]
        #[doc = "The operation is currently running."]
        Running,
        #[serde(rename = "DONE")]
        #[doc = "The operation is done, either cancelled or completed."]
        Done,
        #[serde(rename = "ABORTING")]
        #[doc = "The operation is aborting."]
        Aborting,
    }
    impl ::std::default::Default for OperationProgressStatusEnum {
        fn default() -> Self {
            Self::StatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration options for private clusters."]
    pub struct PrivateClusterConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enablePrivateEndpoint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the master's internal IP address is used as the cluster endpoint."]
        pub enable_private_endpoint: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enablePrivateNodes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether nodes have internal IP addresses only. If enabled, all nodes are given only RFC 1918 private addresses and communicate with the master via private networking."]
        pub enable_private_nodes: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "masterGlobalAccessConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Controls master global access settings."]
        pub master_global_access_config:
            ::std::option::Option<::std::boxed::Box<PrivateClusterMasterGlobalAccessConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "masterIpv4CidrBlock")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IP range in CIDR notation to use for the hosted master network. This range will be used for assigning internal IP addresses to the master or set of masters, as well as the ILB VIP. This range must not overlap with any other ranges in use within the cluster's network."]
        pub master_ipv4_cidr_block: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "peeringName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The peering name in the customer VPC used by this cluster."]
        pub peering_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "privateEndpoint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The internal IP address of this cluster's master endpoint."]
        pub private_endpoint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publicEndpoint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The external IP address of this cluster's master endpoint."]
        pub public_endpoint: ::std::option::Option<::std::string::String>,
    }
    impl PrivateClusterConfig {
        pub fn builder() -> PrivateClusterConfigBuilder {
            PrivateClusterConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for controlling master global access settings."]
    pub struct PrivateClusterMasterGlobalAccessConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whenever master is accessible globally or not."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl PrivateClusterMasterGlobalAccessConfig {
        pub fn builder() -> PrivateClusterMasterGlobalAccessConfigBuilder {
            PrivateClusterMasterGlobalAccessConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Pub/Sub specific notification config."]
    pub struct PubSub {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enable notifications for Pub/Sub."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The desired Pub/Sub topic to which notifications will be sent by GKE. Format is `projects/{project}/topics/{topic}`."]
        pub topic: ::std::option::Option<::std::string::String>,
    }
    impl PubSub {
        pub fn builder() -> PubSubBuilder {
            PubSubBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents an arbitrary window of time that recurs."]
    pub struct RecurringTimeWindow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recurrence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An RRULE (https://tools.ietf.org/html/rfc5545#section-3.8.5.3) for how this window reccurs. They go on for the span of time between the start and end time. For example, to have something repeat every weekday, you'd use: `FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR` To repeat some window daily (equivalent to the DailyMaintenanceWindow): `FREQ=DAILY` For the first weekend of every month: `FREQ=MONTHLY;BYSETPOS=1;BYDAY=SA,SU` This specifies how frequently the window starts. Eg, if you wanted to have a 9-5 UTC-4 window every weekday, you'd use something like: ``` start time = 2019-01-01T09:00:00-0400 end time = 2019-01-01T17:00:00-0400 recurrence = FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR ``` Windows can span multiple days. Eg, to make the window encompass every weekend from midnight Saturday till the last minute of Sunday UTC: ``` start time = 2019-01-05T00:00:00Z end time = 2019-01-07T23:59:00Z recurrence = FREQ=WEEKLY;BYDAY=SA ``` Note the start and end time's specific dates are largely arbitrary except to specify duration of the window and when it first starts. The FREQ values of HOURLY, MINUTELY, and SECONDLY are not supported."]
        pub recurrence: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "window")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The window of the first recurrence."]
        pub window: ::std::option::Option<::std::boxed::Box<TimeWindow>>,
    }
    impl RecurringTimeWindow {
        pub fn builder() -> RecurringTimeWindowBuilder {
            RecurringTimeWindowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "ReleaseChannel indicates which release channel a cluster is subscribed to. Release channels are arranged in order of risk. When a cluster is subscribed to a release channel, Google maintains both the master version and the node version. Node auto-upgrade defaults to true and cannot be disabled."]
    pub struct ReleaseChannel {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "channel specifies which release channel the cluster is subscribed to."]
        pub channel: ::std::option::Option<ReleaseChannelChannelEnum>,
    }
    impl ReleaseChannel {
        pub fn builder() -> ReleaseChannelBuilder {
            ReleaseChannelBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "channel specifies which release channel the cluster is subscribed to."]
    pub enum ReleaseChannelChannelEnum {
        #[serde(rename = "UNSPECIFIED")]
        #[doc = "No channel specified."]
        Unspecified,
        #[serde(rename = "RAPID")]
        #[doc = "RAPID channel is offered on an early access basis for customers who want to test new releases. WARNING: Versions available in the RAPID Channel may be subject to unresolved issues with no known workaround and are not subject to any SLAs."]
        Rapid,
        #[serde(rename = "REGULAR")]
        #[doc = "Clusters subscribed to REGULAR receive versions that are considered GA quality. REGULAR is intended for production users who want to take advantage of new features."]
        Regular,
        #[serde(rename = "STABLE")]
        #[doc = "Clusters subscribed to STABLE receive versions that are known to be stable and reliable in production."]
        Stable,
    }
    impl ::std::default::Default for ReleaseChannelChannelEnum {
        fn default() -> Self {
            Self::Unspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "ReleaseChannelConfig exposes configuration for a release channel."]
    pub struct ReleaseChannelConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The release channel this configuration applies to."]
        pub channel: ::std::option::Option<ReleaseChannelConfigChannelEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The default version for newly created clusters on the channel."]
        pub default_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "validVersions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of valid versions for the channel."]
        pub valid_versions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ReleaseChannelConfig {
        pub fn builder() -> ReleaseChannelConfigBuilder {
            ReleaseChannelConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The release channel this configuration applies to."]
    pub enum ReleaseChannelConfigChannelEnum {
        #[serde(rename = "UNSPECIFIED")]
        #[doc = "No channel specified."]
        Unspecified,
        #[serde(rename = "RAPID")]
        #[doc = "RAPID channel is offered on an early access basis for customers who want to test new releases. WARNING: Versions available in the RAPID Channel may be subject to unresolved issues with no known workaround and are not subject to any SLAs."]
        Rapid,
        #[serde(rename = "REGULAR")]
        #[doc = "Clusters subscribed to REGULAR receive versions that are considered GA quality. REGULAR is intended for production users who want to take advantage of new features."]
        Regular,
        #[serde(rename = "STABLE")]
        #[doc = "Clusters subscribed to STABLE receive versions that are known to be stable and reliable in production."]
        Stable,
    }
    impl ::std::default::Default for ReleaseChannelConfigChannelEnum {
        fn default() -> Self {
            Self::Unspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "[ReservationAffinity](https://cloud.google.com/compute/docs/instances/reserving-zonal-resources) is the configuration of desired reservation which instances could take capacity from."]
    pub struct ReservationAffinity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumeReservationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Corresponds to the type of reservation consumption."]
        pub consume_reservation_type:
            ::std::option::Option<ReservationAffinityConsumeReservationTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Corresponds to the label key of a reservation resource. To target a SPECIFIC_RESERVATION by name, specify \"googleapis.com/reservation-name\" as the key and specify the name of your reservation as its value."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "values")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Corresponds to the label value(s) of reservation resource(s)."]
        pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ReservationAffinity {
        pub fn builder() -> ReservationAffinityBuilder {
            ReservationAffinityBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Corresponds to the type of reservation consumption."]
    pub enum ReservationAffinityConsumeReservationTypeEnum {
        #[serde(rename = "UNSPECIFIED")]
        #[doc = "Default value. This should not be used."]
        Unspecified,
        #[serde(rename = "NO_RESERVATION")]
        #[doc = "Do not consume from any reserved capacity."]
        NoReservation,
        #[serde(rename = "ANY_RESERVATION")]
        #[doc = "Consume any reservation available."]
        AnyReservation,
        #[serde(rename = "SPECIFIC_RESERVATION")]
        #[doc = "Must consume from a specific reservation. Must specify key value fields for specifying the reservations."]
        SpecificReservation,
    }
    impl ::std::default::Default for ReservationAffinityConsumeReservationTypeEnum {
        fn default() -> Self {
            Self::Unspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains information about amount of some resource in the cluster. For memory, value should be in GB."]
    pub struct ResourceLimit {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maximum")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum amount of the resource in the cluster."]
        pub maximum: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minimum")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum amount of the resource in the cluster."]
        pub minimum: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name \"cpu\", \"memory\" or gpu-specific string."]
        pub resource_type: ::std::option::Option<::std::string::String>,
    }
    impl ResourceLimit {
        pub fn builder() -> ResourceLimitBuilder {
            ResourceLimitBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for exporting cluster resource usages."]
    pub struct ResourceUsageExportConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bigqueryDestination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration to use BigQuery as usage export destination."]
        pub bigquery_destination: ::std::option::Option<::std::boxed::Box<BigQueryDestination>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consumptionMeteringConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration to enable resource consumption metering."]
        pub consumption_metering_config:
            ::std::option::Option<::std::boxed::Box<ConsumptionMeteringConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableNetworkEgressMetering")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to enable network egress metering for this cluster. If enabled, a daemonset will be created in the cluster to meter network egress traffic."]
        pub enable_network_egress_metering: ::std::option::Option<::std::primitive::bool>,
    }
    impl ResourceUsageExportConfig {
        pub fn builder() -> ResourceUsageExportConfigBuilder {
            ResourceUsageExportConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "RollbackNodePoolUpgradeRequest rollbacks the previously Aborted or Failed NodePool upgrade. This will be an no-op if the last upgrade successfully completed."]
    pub struct RollbackNodePoolUpgradeRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the cluster to rollback. This field has been deprecated and replaced by the name field."]
        pub cluster_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name (project, location, cluster, node pool id) of the node poll to rollback upgrade. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodePoolId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the node pool to rollback. This field has been deprecated and replaced by the name field."]
        pub node_pool_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl RollbackNodePoolUpgradeRequest {
        pub fn builder() -> RollbackNodePoolUpgradeRequestBuilder {
            RollbackNodePoolUpgradeRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SandboxConfig contains configurations of the sandbox to use for the node."]
    pub struct SandboxConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of the sandbox to use for the node."]
        pub _type: ::std::option::Option<SandboxConfigTypeEnum>,
    }
    impl SandboxConfig {
        pub fn builder() -> SandboxConfigBuilder {
            SandboxConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Type of the sandbox to use for the node."]
    pub enum SandboxConfigTypeEnum {
        #[serde(rename = "UNSPECIFIED")]
        #[doc = "Default value. This should not be used."]
        Unspecified,
        #[serde(rename = "GVISOR")]
        #[doc = "Run sandbox using gvisor."]
        Gvisor,
    }
    impl ::std::default::Default for SandboxConfigTypeEnum {
        fn default() -> Self {
            Self::Unspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Kubernetes Engine service configuration."]
    pub struct ServerConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of release channel configurations."]
        pub channels:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ReleaseChannelConfig>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultClusterVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Version of Kubernetes the service deploys by default."]
        pub default_cluster_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultImageType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Default image type."]
        pub default_image_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "validImageTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of valid image types."]
        pub valid_image_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "validMasterVersions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of valid master versions, in descending order."]
        pub valid_master_versions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "validNodeVersions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of valid node upgrade target versions, in descending order."]
        pub valid_node_versions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ServerConfig {
        pub fn builder() -> ServerConfigBuilder {
            ServerConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SetAddonsConfigRequest sets the addons associated with the cluster."]
    pub struct SetAddonsConfigRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addonsConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The desired configurations for the various addons available to run in the cluster."]
        pub addons_config: ::std::option::Option<::std::boxed::Box<AddonsConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."]
        pub cluster_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name (project, location, cluster) of the cluster to set addons. Specified in the format `projects/*/locations/*/clusters/*`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl SetAddonsConfigRequest {
        pub fn builder() -> SetAddonsConfigRequestBuilder {
            SetAddonsConfigRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SetLabelsRequest sets the Google Cloud Platform labels on a Google Container Engine cluster, which will in turn set them for Google Compute Engine resources used by that cluster"]
    pub struct SetLabelsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the cluster. This field has been deprecated and replaced by the name field."]
        pub cluster_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelFingerprint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The fingerprint of the previous set of labels for this resource, used to detect conflicts. The fingerprint is initially generated by Kubernetes Engine and changes after every request to modify or update labels. You must always provide an up-to-date fingerprint hash when updating or changing labels. Make a `get()` request to the resource to get the latest fingerprint."]
        pub label_fingerprint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name (project, location, cluster id) of the cluster to set labels. Specified in the format `projects/*/locations/*/clusters/*`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://developers.google.com/console/help/new/#projectnumber). This field has been deprecated and replaced by the name field."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The labels to set for that cluster."]
        pub resource_labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl SetLabelsRequest {
        pub fn builder() -> SetLabelsRequestBuilder {
            SetLabelsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SetLegacyAbacRequest enables or disables the ABAC authorization mechanism for a cluster."]
    pub struct SetLegacyAbacRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the cluster to update. This field has been deprecated and replaced by the name field."]
        pub cluster_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Whether ABAC authorization will be enabled in the cluster."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name (project, location, cluster id) of the cluster to set legacy abac. Specified in the format `projects/*/locations/*/clusters/*`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl SetLegacyAbacRequest {
        pub fn builder() -> SetLegacyAbacRequestBuilder {
            SetLegacyAbacRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SetLocationsRequest sets the locations of the cluster."]
    pub struct SetLocationsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."]
        pub cluster_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The desired list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available) in which the cluster's nodes should be located. Changing the locations a cluster is in will result in nodes being either created or removed from the cluster, depending on whether locations are being added or removed. This list must always include the cluster's primary zone."]
        pub locations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name (project, location, cluster) of the cluster to set locations. Specified in the format `projects/*/locations/*/clusters/*`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl SetLocationsRequest {
        pub fn builder() -> SetLocationsRequestBuilder {
            SetLocationsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SetLoggingServiceRequest sets the logging service of a cluster."]
    pub struct SetLoggingServiceRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."]
        pub cluster_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "loggingService")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The logging service the cluster should use to write logs. Currently available options: * `logging.googleapis.com/kubernetes` - The Cloud Logging service with a Kubernetes-native resource model * `logging.googleapis.com` - The legacy Cloud Logging service (no longer available as of GKE 1.15). * `none` - no logs will be exported from the cluster. If left as an empty string,`logging.googleapis.com/kubernetes` will be used for GKE 1.14+ or `logging.googleapis.com` for earlier versions."]
        pub logging_service: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name (project, location, cluster) of the cluster to set logging. Specified in the format `projects/*/locations/*/clusters/*`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl SetLoggingServiceRequest {
        pub fn builder() -> SetLoggingServiceRequestBuilder {
            SetLoggingServiceRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SetMaintenancePolicyRequest sets the maintenance policy for a cluster."]
    pub struct SetMaintenancePolicyRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name of the cluster to update."]
        pub cluster_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maintenancePolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The maintenance policy to be set for the cluster. An empty field clears the existing maintenance policy."]
        pub maintenance_policy: ::std::option::Option<::std::boxed::Box<MaintenancePolicy>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name (project, location, cluster id) of the cluster to set maintenance policy. Specified in the format `projects/*/locations/*/clusters/*`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840)."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl SetMaintenancePolicyRequest {
        pub fn builder() -> SetMaintenancePolicyRequestBuilder {
            SetMaintenancePolicyRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SetMasterAuthRequest updates the admin password of a cluster."]
    pub struct SetMasterAuthRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "action")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The exact form of action to be taken on the master auth."]
        pub action: ::std::option::Option<SetMasterAuthRequestActionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."]
        pub cluster_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name (project, location, cluster) of the cluster to set auth. Specified in the format `projects/*/locations/*/clusters/*`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "update")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A description of the update."]
        pub update: ::std::option::Option<::std::boxed::Box<MasterAuth>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl SetMasterAuthRequest {
        pub fn builder() -> SetMasterAuthRequestBuilder {
            SetMasterAuthRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The exact form of action to be taken on the master auth."]
    pub enum SetMasterAuthRequestActionEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Operation is unknown and will error out."]
        Unknown,
        #[serde(rename = "SET_PASSWORD")]
        #[doc = "Set the password to a user generated value."]
        SetPassword,
        #[serde(rename = "GENERATE_PASSWORD")]
        #[doc = "Generate a new password and set it to that."]
        GeneratePassword,
        #[serde(rename = "SET_USERNAME")]
        #[doc = "Set the username. If an empty username is provided, basic authentication is disabled for the cluster. If a non-empty username is provided, basic authentication is enabled, with either a provided password or a generated one."]
        SetUsername,
    }
    impl ::std::default::Default for SetMasterAuthRequestActionEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SetMonitoringServiceRequest sets the monitoring service of a cluster."]
    pub struct SetMonitoringServiceRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."]
        pub cluster_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "monitoringService")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The monitoring service the cluster should use to write metrics. Currently available options: * \"monitoring.googleapis.com/kubernetes\" - The Cloud Monitoring service with a Kubernetes-native resource model * `monitoring.googleapis.com` - The legacy Cloud Monitoring service (no longer available as of GKE 1.15). * `none` - No metrics will be exported from the cluster. If left as an empty string,`monitoring.googleapis.com/kubernetes` will be used for GKE 1.14+ or `monitoring.googleapis.com` for earlier versions."]
        pub monitoring_service: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name (project, location, cluster) of the cluster to set monitoring. Specified in the format `projects/*/locations/*/clusters/*`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl SetMonitoringServiceRequest {
        pub fn builder() -> SetMonitoringServiceRequestBuilder {
            SetMonitoringServiceRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SetNetworkPolicyRequest enables/disables network policy for a cluster."]
    pub struct SetNetworkPolicyRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the cluster. This field has been deprecated and replaced by the name field."]
        pub cluster_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name (project, location, cluster id) of the cluster to set networking policy. Specified in the format `projects/*/locations/*/clusters/*`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "networkPolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Configuration options for the NetworkPolicy feature."]
        pub network_policy: ::std::option::Option<::std::boxed::Box<NetworkPolicy>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://developers.google.com/console/help/new/#projectnumber). This field has been deprecated and replaced by the name field."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl SetNetworkPolicyRequest {
        pub fn builder() -> SetNetworkPolicyRequestBuilder {
            SetNetworkPolicyRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SetNodePoolAutoscalingRequest sets the autoscaler settings of a node pool."]
    pub struct SetNodePoolAutoscalingRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoscaling")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Autoscaling configuration for the node pool."]
        pub autoscaling: ::std::option::Option<::std::boxed::Box<NodePoolAutoscaling>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."]
        pub cluster_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name (project, location, cluster, node pool) of the node pool to set autoscaler settings. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodePoolId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the node pool to upgrade. This field has been deprecated and replaced by the name field."]
        pub node_pool_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl SetNodePoolAutoscalingRequest {
        pub fn builder() -> SetNodePoolAutoscalingRequestBuilder {
            SetNodePoolAutoscalingRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SetNodePoolManagementRequest sets the node management properties of a node pool."]
    pub struct SetNodePoolManagementRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the cluster to update. This field has been deprecated and replaced by the name field."]
        pub cluster_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "management")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. NodeManagement configuration for the node pool."]
        pub management: ::std::option::Option<::std::boxed::Box<NodeManagement>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name (project, location, cluster, node pool id) of the node pool to set management properties. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodePoolId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the node pool to update. This field has been deprecated and replaced by the name field."]
        pub node_pool_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl SetNodePoolManagementRequest {
        pub fn builder() -> SetNodePoolManagementRequestBuilder {
            SetNodePoolManagementRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SetNodePoolSizeRequest sets the size of a node pool."]
    pub struct SetNodePoolSizeRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the cluster to update. This field has been deprecated and replaced by the name field."]
        pub cluster_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name (project, location, cluster, node pool id) of the node pool to set size. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodeCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The desired node count for the pool."]
        pub node_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodePoolId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the node pool to update. This field has been deprecated and replaced by the name field."]
        pub node_pool_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl SetNodePoolSizeRequest {
        pub fn builder() -> SetNodePoolSizeRequestBuilder {
            SetNodePoolSizeRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A set of Shielded Instance options."]
    pub struct ShieldedInstanceConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableIntegrityMonitoring")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines whether the instance has integrity monitoring enabled. Enables monitoring and attestation of the boot integrity of the instance. The attestation is performed against the integrity policy baseline. This baseline is initially derived from the implicitly trusted boot image when the instance is created."]
        pub enable_integrity_monitoring: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableSecureBoot")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines whether the instance has Secure Boot enabled. Secure Boot helps ensure that the system only runs authentic software by verifying the digital signature of all boot components, and halting the boot process if signature verification fails."]
        pub enable_secure_boot: ::std::option::Option<::std::primitive::bool>,
    }
    impl ShieldedInstanceConfig {
        pub fn builder() -> ShieldedInstanceConfigBuilder {
            ShieldedInstanceConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration of Shielded Nodes feature."]
    pub struct ShieldedNodes {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether Shielded Nodes features are enabled on all nodes in this cluster."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl ShieldedNodes {
        pub fn builder() -> ShieldedNodesBuilder {
            ShieldedNodesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "StartIPRotationRequest creates a new IP for the cluster and then performs a node upgrade on each node pool to point to the new IP."]
    pub struct StartIpRotationRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the cluster. This field has been deprecated and replaced by the name field."]
        pub cluster_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name (project, location, cluster id) of the cluster to start IP rotation. Specified in the format `projects/*/locations/*/clusters/*`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://developers.google.com/console/help/new/#projectnumber). This field has been deprecated and replaced by the name field."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rotateCredentials")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to rotate credentials during IP rotation."]
        pub rotate_credentials: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl StartIpRotationRequest {
        pub fn builder() -> StartIpRotationRequestBuilder {
            StartIpRotationRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "StatusCondition describes why a cluster or a node pool has a certain status (e.g., ERROR or DEGRADED)."]
    pub struct StatusCondition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Machine-friendly representation of the condition"]
        pub code: ::std::option::Option<StatusConditionCodeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human-friendly representation of the condition"]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl StatusCondition {
        pub fn builder() -> StatusConditionBuilder {
            StatusConditionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Machine-friendly representation of the condition"]
    pub enum StatusConditionCodeEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "UNKNOWN indicates a generic condition."]
        Unknown,
        #[serde(rename = "GCE_STOCKOUT")]
        #[doc = "GCE_STOCKOUT indicates that Google Compute Engine resources are temporarily unavailable."]
        GceStockout,
        #[serde(rename = "GKE_SERVICE_ACCOUNT_DELETED")]
        #[doc = "GKE_SERVICE_ACCOUNT_DELETED indicates that the user deleted their robot service account."]
        GkeServiceAccountDeleted,
        #[serde(rename = "GCE_QUOTA_EXCEEDED")]
        #[doc = "Google Compute Engine quota was exceeded."]
        GceQuotaExceeded,
        #[serde(rename = "SET_BY_OPERATOR")]
        #[doc = "Cluster state was manually changed by an SRE due to a system logic error."]
        SetByOperator,
        #[serde(rename = "CLOUD_KMS_KEY_ERROR")]
        #[doc = "Unable to perform an encrypt operation against the CloudKMS key used for etcd level encryption. More codes TBA"]
        CloudKmsKeyError,
    }
    impl ::std::default::Default for StatusConditionCodeEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents an arbitrary window of time."]
    pub struct TimeWindow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time that the window ends. The end time should take place after the start time."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time that the window first starts."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl TimeWindow {
        pub fn builder() -> TimeWindowBuilder {
            TimeWindowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "UpdateClusterRequest updates the settings of a cluster."]
    pub struct UpdateClusterRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."]
        pub cluster_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name (project, location, cluster) of the cluster to update. Specified in the format `projects/*/locations/*/clusters/*`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "update")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A description of the update."]
        pub update: ::std::option::Option<::std::boxed::Box<ClusterUpdate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl UpdateClusterRequest {
        pub fn builder() -> UpdateClusterRequestBuilder {
            UpdateClusterRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "UpdateMasterRequest updates the master of the cluster."]
    pub struct UpdateMasterRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."]
        pub cluster_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "masterVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The Kubernetes version to change the master to. Users may specify either explicit versions offered by Kubernetes Engine or version aliases, which have the following behavior: - \"latest\": picks the highest valid Kubernetes version - \"1.X\": picks the highest valid patch+gke.N patch in the 1.X version - \"1.X.Y\": picks the highest valid gke.N patch in the 1.X.Y version - \"1.X.Y-gke.N\": picks an explicit Kubernetes version - \"-\": picks the default Kubernetes version"]
        pub master_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name (project, location, cluster) of the cluster to update. Specified in the format `projects/*/locations/*/clusters/*`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl UpdateMasterRequest {
        pub fn builder() -> UpdateMasterRequestBuilder {
            UpdateMasterRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "UpdateNodePoolRequests update a node pool's image and/or version."]
    pub struct UpdateNodePoolRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."]
        pub cluster_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The desired image type for the node pool."]
        pub image_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kubeletConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Node kubelet configs."]
        pub kubelet_config: ::std::option::Option<::std::boxed::Box<NodeKubeletConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linuxNodeConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parameters that can be configured on Linux nodes."]
        pub linux_node_config: ::std::option::Option<::std::boxed::Box<LinuxNodeConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The desired list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available) in which the node pool's nodes should be located. Changing the locations for a node pool will result in nodes being either created or removed from the node pool, depending on whether locations are being added or removed."]
        pub locations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name (project, location, cluster, node pool) of the node pool to update. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodePoolId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the node pool to upgrade. This field has been deprecated and replaced by the name field."]
        pub node_pool_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodeVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The Kubernetes version to change the nodes to (typically an upgrade). Users may specify either explicit versions offered by Kubernetes Engine or version aliases, which have the following behavior: - \"latest\": picks the highest valid Kubernetes version - \"1.X\": picks the highest valid patch+gke.N patch in the 1.X version - \"1.X.Y\": picks the highest valid gke.N patch in the 1.X.Y version - \"1.X.Y-gke.N\": picks an explicit Kubernetes version - \"-\": picks the Kubernetes master version"]
        pub node_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "upgradeSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Upgrade settings control disruption and speed of the upgrade."]
        pub upgrade_settings: ::std::option::Option<::std::boxed::Box<UpgradeSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workloadMetadataConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The desired workload metadata config for the node pool."]
        pub workload_metadata_config:
            ::std::option::Option<::std::boxed::Box<WorkloadMetadataConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
        pub zone: ::std::option::Option<::std::string::String>,
    }
    impl UpdateNodePoolRequest {
        pub fn builder() -> UpdateNodePoolRequestBuilder {
            UpdateNodePoolRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "UpgradeEvent is a notification sent to customers by the cluster server when a resource is upgrading."]
    pub struct UpgradeEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current version before the upgrade."]
        pub current_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The operation associated with this upgrade."]
        pub operation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationStartTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when the operation was started."]
        pub operation_start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional relative path to the resource. For example in node pool upgrades, the relative path of the node pool."]
        pub resource: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource type that is upgrading."]
        pub resource_type: ::std::option::Option<UpgradeEventResourceTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target version for the upgrade."]
        pub target_version: ::std::option::Option<::std::string::String>,
    }
    impl UpgradeEvent {
        pub fn builder() -> UpgradeEventBuilder {
            UpgradeEventBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The resource type that is upgrading."]
    pub enum UpgradeEventResourceTypeEnum {
        #[serde(rename = "UPGRADE_RESOURCE_TYPE_UNSPECIFIED")]
        #[doc = "Default value. This shouldn't be used."]
        UpgradeResourceTypeUnspecified,
        #[serde(rename = "MASTER")]
        #[doc = "Master / control plane"]
        Master,
        #[serde(rename = "NODE_POOL")]
        #[doc = "Node pool"]
        NodePool,
    }
    impl ::std::default::Default for UpgradeEventResourceTypeEnum {
        fn default() -> Self {
            Self::UpgradeResourceTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "These upgrade settings control the level of parallelism and the level of disruption caused by an upgrade. maxUnavailable controls the number of nodes that can be simultaneously unavailable. maxSurge controls the number of additional nodes that can be added to the node pool temporarily for the time of the upgrade to increase the number of available nodes. (maxUnavailable + maxSurge) determines the level of parallelism (how many nodes are being upgraded at the same time). Note: upgrades inevitably introduce some disruption since workloads need to be moved from old nodes to new, upgraded ones. Even if maxUnavailable=0, this holds true. (Disruption stays within the limits of PodDisruptionBudget, if it is configured.) Consider a hypothetical node pool with 5 nodes having maxSurge=2, maxUnavailable=1. This means the upgrade process upgrades 3 nodes simultaneously. It creates 2 additional (upgraded) nodes, then it brings down 3 old (not yet upgraded) nodes at the same time. This ensures that there are always at least 4 nodes available."]
    pub struct UpgradeSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxSurge")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of nodes that can be created beyond the current size of the node pool during the upgrade process."]
        pub max_surge: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxUnavailable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of nodes that can be simultaneously unavailable during the upgrade process. A node is considered available if its status is Ready."]
        pub max_unavailable: ::std::option::Option<::std::primitive::i64>,
    }
    impl UpgradeSettings {
        pub fn builder() -> UpgradeSettingsBuilder {
            UpgradeSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "UsableSubnetwork resource returns the subnetwork name, its associated network and the primary CIDR range."]
    pub struct UsableSubnetwork {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipCidrRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range of internal addresses that are owned by this subnetwork."]
        pub ip_cidr_range: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "network")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Network Name. Example: projects/my-project/global/networks/my-network"]
        pub network: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secondaryIpRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Secondary IP ranges."]
        pub secondary_ip_ranges: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<UsableSubnetworkSecondaryRange>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statusMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A human readable status message representing the reasons for cases where the caller cannot use the secondary ranges under the subnet. For example if the secondary_ip_ranges is empty due to a permission issue, an insufficient permission message will be given by status_message."]
        pub status_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subnetwork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Subnetwork Name. Example: projects/my-project/regions/us-central1/subnetworks/my-subnet"]
        pub subnetwork: ::std::option::Option<::std::string::String>,
    }
    impl UsableSubnetwork {
        pub fn builder() -> UsableSubnetworkBuilder {
            UsableSubnetworkBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Secondary IP range of a usable subnetwork."]
    pub struct UsableSubnetworkSecondaryRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipCidrRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range of IP addresses belonging to this subnetwork secondary range."]
        pub ip_cidr_range: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rangeName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name associated with this subnetwork secondary range, used when adding an alias IP range to a VM instance."]
        pub range_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is to determine the status of the secondary range programmably."]
        pub status: ::std::option::Option<UsableSubnetworkSecondaryRangeStatusEnum>,
    }
    impl UsableSubnetworkSecondaryRange {
        pub fn builder() -> UsableSubnetworkSecondaryRangeBuilder {
            UsableSubnetworkSecondaryRangeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "This field is to determine the status of the secondary range programmably."]
    pub enum UsableSubnetworkSecondaryRangeStatusEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "UNKNOWN is the zero value of the Status enum. It's not a valid status."]
        Unknown,
        #[serde(rename = "UNUSED")]
        #[doc = "UNUSED denotes that this range is unclaimed by any cluster."]
        Unused,
        #[serde(rename = "IN_USE_SERVICE")]
        #[doc = "IN_USE_SERVICE denotes that this range is claimed by a cluster for services. It cannot be used for other clusters."]
        InUseService,
        #[serde(rename = "IN_USE_SHAREABLE_POD")]
        #[doc = "IN_USE_SHAREABLE_POD denotes this range was created by the network admin and is currently claimed by a cluster for pods. It can only be used by other clusters as a pod range."]
        InUseShareablePod,
        #[serde(rename = "IN_USE_MANAGED_POD")]
        #[doc = "IN_USE_MANAGED_POD denotes this range was created by GKE and is claimed for pods. It cannot be used for other clusters."]
        InUseManagedPod,
    }
    impl ::std::default::Default for UsableSubnetworkSecondaryRangeStatusEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "VerticalPodAutoscaling contains global, per-cluster information required by Vertical Pod Autoscaler to automatically adjust the resources of pods controlled by it."]
    pub struct VerticalPodAutoscaling {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enables vertical pod autoscaling."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl VerticalPodAutoscaling {
        pub fn builder() -> VerticalPodAutoscalingBuilder {
            VerticalPodAutoscalingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for the use of Kubernetes Service Accounts in GCP IAM policies."]
    pub struct WorkloadIdentityConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workloadPool")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The workload pool to attach all Kubernetes service accounts to."]
        pub workload_pool: ::std::option::Option<::std::string::String>,
    }
    impl WorkloadIdentityConfig {
        pub fn builder() -> WorkloadIdentityConfigBuilder {
            WorkloadIdentityConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "WorkloadMetadataConfig defines the metadata configuration to expose to workloads on the node pool."]
    pub struct WorkloadMetadataConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mode is the configuration for how to expose metadata to workloads running on the node pool."]
        pub mode: ::std::option::Option<WorkloadMetadataConfigModeEnum>,
    }
    impl WorkloadMetadataConfig {
        pub fn builder() -> WorkloadMetadataConfigBuilder {
            WorkloadMetadataConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Mode is the configuration for how to expose metadata to workloads running on the node pool."]
    pub enum WorkloadMetadataConfigModeEnum {
        #[serde(rename = "MODE_UNSPECIFIED")]
        #[doc = "Not set."]
        ModeUnspecified,
        #[serde(rename = "GCE_METADATA")]
        #[doc = "Expose all Compute Engine metadata to pods."]
        GceMetadata,
        #[serde(rename = "GKE_METADATA")]
        #[doc = "Run the GKE Metadata Server on this node. The GKE Metadata Server exposes a metadata API to workloads that is compatible with the V1 Compute Metadata APIs exposed by the Compute Engine and App Engine Metadata Servers. This feature can only be enabled if Workload Identity is enabled at the cluster level."]
        GkeMetadata,
    }
    impl ::std::default::Default for WorkloadMetadataConfigModeEnum {
        fn default() -> Self {
            Self::ModeUnspecified
        }
    }
}
