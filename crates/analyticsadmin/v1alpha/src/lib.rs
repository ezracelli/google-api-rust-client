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
    pub mod account_summaries {
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
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of AccountSummary resources to return. The service may return fewer than this value, even if there are additional pages. If unspecified, at most 50 resources will be returned. The maximum value is 200; (higher values will be coerced to the maximum)"]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A page token, received from a previous `ListAccountSummaries` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListAccountSummaries` must match the call that provided the page token."]
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
    pub mod accounts {
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
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of resources to return. The service may return fewer than this value, even if there are additional pages. If unspecified, at most 50 resources will be returned. The maximum value is 200; (higher values will be coerced to the maximum)"]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A page token, received from a previous `ListAccounts` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListAccounts` must match the call that provided the page token."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "showDeleted")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to include soft-deleted (ie: \"trashed\") Accounts in the results. Accounts can be inspected to determine whether they are deleted or not."]
                    pub show_deleted: ::std::option::Option<::std::primitive::bool>,
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
                    #[serde(rename = "updateMask")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. The list of fields to be updated. Field names must be in snake case (e.g., \"field_to_update\"). Omitted fields will not be updated. To replace the entire entity, use one path with the string \"*\" to match all fields."]
                    pub update_mask: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
        pub mod resources {
            pub mod user_links {
                pub mod methods {
                    pub mod batch_get {
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
                            #[serde(rename = "names")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. The names of the user links to retrieve. A maximum of 1000 user links can be retrieved in a batch. Format: accounts/{accountId}/userLinks/{userLinkId}"]
                            pub names: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
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
                            #[serde(rename = "notifyNewUser")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. If set, then email the new user notifying them that they've been granted permissions to the resource."]
                            pub notify_new_user: ::std::option::Option<::std::primitive::bool>,
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
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of user links to return. The service may return fewer than this value. If unspecified, at most 200 user links will be returned. The maximum value is 500; values above 500 will be coerced to 500."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A page token, received from a previous `ListUserLinks` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListUserLinks` must match the call that provided the page token."]
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
    pub mod properties {
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
                    #[doc = "Required. An expression for filtering the results of the request. Fields eligible for filtering are: `parent:`(The resource name of the parent account) or `firebase_project:`(The id or number of the linked firebase project). Some examples of filters: ``` | Filter | Description | |-----------------------------|-------------------------------------------| | parent:accounts/123 | The account with account id: 123. | | firebase_project:project-id | The firebase project with id: project-id. | | firebase_project:123 | The firebase project with number: 123. | ```"]
                    pub filter: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of resources to return. The service may return fewer than this value, even if there are additional pages. If unspecified, at most 50 resources will be returned. The maximum value is 200; (higher values will be coerced to the maximum)"]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A page token, received from a previous `ListProperties` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListProperties` must match the call that provided the page token."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "showDeleted")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to include soft-deleted (ie: \"trashed\") Properties in the results. Properties can be inspected to determine whether they are deleted or not."]
                    pub show_deleted: ::std::option::Option<::std::primitive::bool>,
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
                    #[serde(rename = "updateMask")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. The list of fields to be updated. Field names must be in snake case (e.g., \"field_to_update\"). Omitted fields will not be updated. To replace the entire entity, use one path with the string \"*\" to match all fields."]
                    pub update_mask: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
        pub mod resources {
            pub mod android_app_data_streams {
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
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of resources to return. If unspecified, at most 50 resources will be returned. The maximum value is 200; (higher values will be coerced to the maximum)"]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A page token, received from a previous call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListAndroidAppDataStreams` must match the call that provided the page token."]
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. The list of fields to be updated. Field names must be in snake case (e.g., \"field_to_update\"). Omitted fields will not be updated. To replace the entire entity, use one path with the string \"*\" to match all fields."]
                            pub update_mask: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod firebase_links {
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
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of resources to return. The service may return fewer than this value, even if there are additional pages. If unspecified, at most 50 resources will be returned. The maximum value is 200; (higher values will be coerced to the maximum)"]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A page token, received from a previous `ListFirebaseLinks` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListProperties` must match the call that provided the page token."]
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. The list of fields to be updated. Field names must be in snake case (e.g., \"field_to_update\"). Omitted fields will not be updated. To replace the entire entity, use one path with the string \"*\" to match all fields."]
                            pub update_mask: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod google_ads_links {
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
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of resources to return. If unspecified, at most 50 resources will be returned. The maximum value is 200 (higher values will be coerced to the maximum)."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A page token, received from a previous `ListGoogleAdsLinks` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListGoogleAdsLinks` must match the call that provided the page token."]
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. The list of fields to be updated. Field names must be in snake case (e.g., \"field_to_update\"). Omitted fields will not be updated. To replace the entire entity, use one path with the string \"*\" to match all fields."]
                            pub update_mask: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod ios_app_data_streams {
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
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of resources to return. If unspecified, at most 50 resources will be returned. The maximum value is 200; (higher values will be coerced to the maximum)"]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A page token, received from a previous `ListIosAppDataStreams` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListIosAppDataStreams` must match the call that provided the page token."]
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. The list of fields to be updated. Field names must be in snake case (e.g., \"field_to_update\"). Omitted fields will not be updated. To replace the entire entity, use one path with the string \"*\" to match all fields."]
                            pub update_mask: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod user_links {
                pub mod methods {
                    pub mod batch_get {
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
                            #[serde(rename = "names")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. The names of the user links to retrieve. A maximum of 1000 user links can be retrieved in a batch. Format: accounts/{accountId}/userLinks/{userLinkId}"]
                            pub names: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
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
                            #[serde(rename = "notifyNewUser")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. If set, then email the new user notifying them that they've been granted permissions to the resource."]
                            pub notify_new_user: ::std::option::Option<::std::primitive::bool>,
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
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of user links to return. The service may return fewer than this value. If unspecified, at most 200 user links will be returned. The maximum value is 500; values above 500 will be coerced to 500."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A page token, received from a previous `ListUserLinks` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListUserLinks` must match the call that provided the page token."]
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
            pub mod web_data_streams {
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
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of resources to return. If unspecified, at most 50 resources will be returned. The maximum value is 200; (higher values will be coerced to the maximum)"]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A page token, received from a previous `ListWebDataStreams` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListWebDataStreams` must match the call that provided the page token."]
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. The list of fields to be updated. Field names must be in snake case (e.g., \"field_to_update\"). Omitted fields will not be updated. To replace the entire entity, use one path with the string \"*\" to match all fields."]
                            pub update_mask: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                    pub mod update_enhanced_measurement_settings {
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
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. The list of fields to be updated. Field names must be in snake case (e.g., \"field_to_update\"). Omitted fields will not be updated. To replace the entire entity, use one path with the string \"*\" to match all fields."]
                            pub update_mask: ::std::option::Option<::std::string::String>,
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
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A resource message representing a Google Analytics account."]
    pub struct GoogleAnalyticsAdminV1alphaAccount {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time when this account was originally created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Indicates whether this Account is soft-deleted or not. Deleted accounts are excluded from List results unless specifically requested."]
        pub deleted: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Human-readable display name for this account."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Resource name of this account. Format: accounts/{account} Example: \"accounts/100\""]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "regionCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Country of business. Must be a Unicode CLDR region code."]
        pub region_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time when account payload fields were last updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAnalyticsAdminV1alphaAccount {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaAccountBuilder {
            GoogleAnalyticsAdminV1alphaAccountBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A virtual resource representing an overview of an account and all its child GA4 properties."]
    pub struct GoogleAnalyticsAdminV1alphaAccountSummary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "account")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name of account referred to by this account summary Format: accounts/{account_id} Example: \"accounts/1000\""]
        pub account: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display name for the account referred to in this account summary."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name for this account summary. Format: accountSummaries/{account_id} Example: \"accountSummaries/1000\""]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "propertySummaries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of summaries for child accounts of this account."]
        pub property_summaries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleAnalyticsAdminV1alphaPropertySummary>>,
        >,
    }
    impl GoogleAnalyticsAdminV1alphaAccountSummary {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaAccountSummaryBuilder {
            GoogleAnalyticsAdminV1alphaAccountSummaryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A resource message representing a Google Analytics Android app stream."]
    pub struct GoogleAnalyticsAdminV1alphaAndroidAppDataStream {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time when this stream was originally created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human-readable display name for the Data Stream. The max allowed display name length is 255 UTF-16 code units."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firebaseAppId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. ID of the corresponding Android app in Firebase, if any. This ID can change if the Android app is deleted and recreated."]
        pub firebase_app_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Resource name of this Data Stream. Format: properties/{property_id}/androidAppDataStreams/{stream_id} Example: \"properties/1000/androidAppDataStreams/2000\""]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Immutable. The package name for the app being measured. Example: \"com.example.myandroidapp\""]
        pub package_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time when stream payload fields were last updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAnalyticsAdminV1alphaAndroidAppDataStream {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaAndroidAppDataStreamBuilder {
            GoogleAnalyticsAdminV1alphaAndroidAppDataStreamBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Read-only resource used to summarize a principal's effective roles."]
    pub struct GoogleAnalyticsAdminV1alphaAuditUserLink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "directRoles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Roles directly assigned to this user for this entity. Format: predefinedRoles/read Excludes roles that are inherited from an account (if this is for a property), group, or organization admin role."]
        pub direct_roles: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "effectiveRoles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Union of all permissions a user has at this account or property (includes direct permissions, group-inherited permissions, etc.). Format: predefinedRoles/read"]
        pub effective_roles: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emailAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email address of the linked user"]
        pub email_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Example format: properties/1234/userLinks/5678"]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAnalyticsAdminV1alphaAuditUserLink {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaAuditUserLinkBuilder {
            GoogleAnalyticsAdminV1alphaAuditUserLinkBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for AuditUserLinks RPC."]
    pub struct GoogleAnalyticsAdminV1alphaAuditUserLinksRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of user links to return. The service may return fewer than this value. If unspecified, at most 1000 user links will be returned. The maximum value is 5000; values above 5000 will be coerced to 5000."]
        pub page_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A page token, received from a previous `AuditUserLinks` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `AuditUserLinks` must match the call that provided the page token."]
        pub page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAnalyticsAdminV1alphaAuditUserLinksRequest {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaAuditUserLinksRequestBuilder {
            GoogleAnalyticsAdminV1alphaAuditUserLinksRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for AuditUserLinks RPC."]
    pub struct GoogleAnalyticsAdminV1alphaAuditUserLinksResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userLinks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of AuditUserLinks. These will be ordered stably, but in an arbitrary order."]
        pub user_links: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleAnalyticsAdminV1alphaAuditUserLink>>,
        >,
    }
    impl GoogleAnalyticsAdminV1alphaAuditUserLinksResponse {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaAuditUserLinksResponseBuilder {
            GoogleAnalyticsAdminV1alphaAuditUserLinksResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for BatchCreateUserLinks RPC."]
    pub struct GoogleAnalyticsAdminV1alphaBatchCreateUserLinksRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notifyNewUsers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If set, then email the new users notifying them that they've been granted permissions to the resource. Regardless of whether this is set or not, notify_new_user field inside each individual request is ignored."]
        pub notify_new_users: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requests")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The requests specifying the user links to create. A maximum of 1000 user links can be created in a batch."]
        pub requests: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleAnalyticsAdminV1alphaCreateUserLinkRequest>>,
        >,
    }
    impl GoogleAnalyticsAdminV1alphaBatchCreateUserLinksRequest {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaBatchCreateUserLinksRequestBuilder {
            GoogleAnalyticsAdminV1alphaBatchCreateUserLinksRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for BatchCreateUserLinks RPC."]
    pub struct GoogleAnalyticsAdminV1alphaBatchCreateUserLinksResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userLinks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user links created."]
        pub user_links: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleAnalyticsAdminV1alphaUserLink>>,
        >,
    }
    impl GoogleAnalyticsAdminV1alphaBatchCreateUserLinksResponse {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaBatchCreateUserLinksResponseBuilder {
            GoogleAnalyticsAdminV1alphaBatchCreateUserLinksResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for BatchDeleteUserLinks RPC."]
    pub struct GoogleAnalyticsAdminV1alphaBatchDeleteUserLinksRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requests")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The requests specifying the user links to update. A maximum of 1000 user links can be updated in a batch."]
        pub requests: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleAnalyticsAdminV1alphaDeleteUserLinkRequest>>,
        >,
    }
    impl GoogleAnalyticsAdminV1alphaBatchDeleteUserLinksRequest {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaBatchDeleteUserLinksRequestBuilder {
            GoogleAnalyticsAdminV1alphaBatchDeleteUserLinksRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for BatchGetUserLinks RPC."]
    pub struct GoogleAnalyticsAdminV1alphaBatchGetUserLinksResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userLinks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The requested user links."]
        pub user_links: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleAnalyticsAdminV1alphaUserLink>>,
        >,
    }
    impl GoogleAnalyticsAdminV1alphaBatchGetUserLinksResponse {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaBatchGetUserLinksResponseBuilder {
            GoogleAnalyticsAdminV1alphaBatchGetUserLinksResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for BatchUpdateUserLinks RPC."]
    pub struct GoogleAnalyticsAdminV1alphaBatchUpdateUserLinksRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requests")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The requests specifying the user links to update. A maximum of 1000 user links can be updated in a batch."]
        pub requests: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleAnalyticsAdminV1alphaUpdateUserLinkRequest>>,
        >,
    }
    impl GoogleAnalyticsAdminV1alphaBatchUpdateUserLinksRequest {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaBatchUpdateUserLinksRequestBuilder {
            GoogleAnalyticsAdminV1alphaBatchUpdateUserLinksRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for BatchUpdateUserLinks RPC."]
    pub struct GoogleAnalyticsAdminV1alphaBatchUpdateUserLinksResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userLinks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user links updated."]
        pub user_links: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleAnalyticsAdminV1alphaUserLink>>,
        >,
    }
    impl GoogleAnalyticsAdminV1alphaBatchUpdateUserLinksResponse {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaBatchUpdateUserLinksResponseBuilder {
            GoogleAnalyticsAdminV1alphaBatchUpdateUserLinksResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for CreateUserLink RPC. Users can have multiple email addresses associated with their Google account, and one of these email addresses is the \"primary\" email address. Any of the email addresses associated with a Google account may be used for a new UserLink, but the returned UserLink will always contain the \"primary\" email address. As a result, the input and output email address for this request may differ."]
    pub struct GoogleAnalyticsAdminV1alphaCreateUserLinkRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notifyNewUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If set, then email the new user notifying them that they've been granted permissions to the resource."]
        pub notify_new_user: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Example format: accounts/1234"]
        pub parent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The user link to create."]
        pub user_link:
            ::std::option::Option<::std::boxed::Box<GoogleAnalyticsAdminV1alphaUserLink>>,
    }
    impl GoogleAnalyticsAdminV1alphaCreateUserLinkRequest {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaCreateUserLinkRequestBuilder {
            GoogleAnalyticsAdminV1alphaCreateUserLinkRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A resource message representing data sharing settings of a Google Analytics account."]
    pub struct GoogleAnalyticsAdminV1alphaDataSharingSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Resource name. Format: accounts/{account}/dataSharingSettings Example: \"accounts/1000/dataSharingSettings\""]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sharingWithGoogleAnySalesEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allows any of Google sales to access the data in order to suggest configuration changes to improve results."]
        pub sharing_with_google_any_sales_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sharingWithGoogleAssignedSalesEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allows Google sales teams that are assigned to the customer to access the data in order to suggest configuration changes to improve results. Sales team restrictions still apply when enabled."]
        pub sharing_with_google_assigned_sales_enabled:
            ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sharingWithGoogleProductsEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allows Google to use the data to improve other Google products or services."]
        pub sharing_with_google_products_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sharingWithGoogleSupportEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allows Google support to access the data in order to help troubleshoot issues."]
        pub sharing_with_google_support_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sharingWithOthersEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allows Google to share the data anonymously in aggregate form with others."]
        pub sharing_with_others_enabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleAnalyticsAdminV1alphaDataSharingSettings {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaDataSharingSettingsBuilder {
            GoogleAnalyticsAdminV1alphaDataSharingSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for DeleteUserLink RPC."]
    pub struct GoogleAnalyticsAdminV1alphaDeleteUserLinkRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Example format: accounts/1234/userLinks/5678"]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAnalyticsAdminV1alphaDeleteUserLinkRequest {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaDeleteUserLinkRequestBuilder {
            GoogleAnalyticsAdminV1alphaDeleteUserLinkRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Singleton resource under a WebDataStream, configuring measurement of additional site interactions and content."]
    pub struct GoogleAnalyticsAdminV1alphaEnhancedMeasurementSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileDownloadsEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If enabled, capture a file download event each time a link is clicked with a common document, compressed file, application, video, or audio extension."]
        pub file_downloads_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Resource name of this Data Stream. Format: properties/{property_id}/webDataStreams/{stream_id}/enhancedMeasurementSettings Example: \"properties/1000/webDataStreams/2000/enhancedMeasurementSettings\""]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outboundClicksEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If enabled, capture an outbound click event each time a visitor clicks a link that leads them away from your domain."]
        pub outbound_clicks_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageChangesEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If enabled, capture a page view event each time the website changes the browser history state."]
        pub page_changes_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageLoadsEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. If enabled, capture a page view event each time a page loads."]
        pub page_loads_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageViewsEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. If enabled, capture a page view event each time a page loads or the website changes the browser history state."]
        pub page_views_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scrollsEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If enabled, capture scroll events each time a visitor gets to the bottom of a page."]
        pub scrolls_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "searchQueryParameter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. URL query parameters to interpret as site search parameters. Max length is 1024 characters. Must not be empty."]
        pub search_query_parameter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "siteSearchEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If enabled, capture a view search results event each time a visitor performs a search on your site (based on a query parameter)."]
        pub site_search_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "streamEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether Enhanced Measurement Settings will be used to automatically measure interactions and content on this web stream. Changing this value does not affect the settings themselves, but determines whether they are respected."]
        pub stream_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uriQueryParameter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional URL query parameters. Max length is 1024 characters."]
        pub uri_query_parameter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoEngagementEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If enabled, capture video play, progress, and complete events as visitors view embedded videos on your site."]
        pub video_engagement_enabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleAnalyticsAdminV1alphaEnhancedMeasurementSettings {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaEnhancedMeasurementSettingsBuilder {
            GoogleAnalyticsAdminV1alphaEnhancedMeasurementSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A link between an GA4 property and a Firebase project."]
    pub struct GoogleAnalyticsAdminV1alphaFirebaseLink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time when this FirebaseLink was originally created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maximumUserAccess")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum user access to the GA4 property allowed to admins of the linked Firebase project."]
        pub maximum_user_access:
            ::std::option::Option<GoogleAnalyticsAdminV1alphaFirebaseLinkMaximumUserAccessEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Example format: properties/1234/firebaseLinks/5678"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "project")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Immutable. Firebase project resource name. When creating a FirebaseLink, you may provide this resource name using either a project number or project ID. Once this resource has been created, returned FirebaseLinks will always have a project_name that contains a project number. Format: 'projects/{project number}' Example: 'projects/1234'"]
        pub project: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAnalyticsAdminV1alphaFirebaseLink {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaFirebaseLinkBuilder {
            GoogleAnalyticsAdminV1alphaFirebaseLinkBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Maximum user access to the GA4 property allowed to admins of the linked Firebase project."]
    pub enum GoogleAnalyticsAdminV1alphaFirebaseLinkMaximumUserAccessEnum {
        #[serde(rename = "MAXIMUM_USER_ACCESS_UNSPECIFIED")]
        #[doc = "Unspecified maximum user access."]
        MaximumUserAccessUnspecified,
        #[serde(rename = "NO_ACCESS")]
        #[doc = "Firebase users have no access to the Analytics property."]
        NoAccess,
        #[serde(rename = "READ_AND_ANALYZE")]
        #[doc = "Firebase users have Read & Analyze access to the Analytics property."]
        ReadAndAnalyze,
        #[serde(rename = "EDITOR_WITHOUT_LINK_MANAGEMENT")]
        #[doc = "Firebase users have edit access to the Analytics property, but may not manage the Firebase link."]
        EditorWithoutLinkManagement,
        #[serde(rename = "EDITOR_INCLUDING_LINK_MANAGEMENT")]
        #[doc = "Firebase users have edit access to the Analytics property and may manage the Firebase link."]
        EditorIncludingLinkManagement,
    }
    impl ::std::default::Default for GoogleAnalyticsAdminV1alphaFirebaseLinkMaximumUserAccessEnum {
        fn default() -> Self {
            Self::MaximumUserAccessUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Read-only resource with the tag for sending data from a website to a WebDataStream."]
    pub struct GoogleAnalyticsAdminV1alphaGlobalSiteTag {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Resource name for this GlobalSiteTag resource. Format: properties/{propertyId}/globalSiteTag"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Immutable. JavaScript code snippet to be pasted as the first item into the head tag of every webpage to measure."]
        pub snippet: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAnalyticsAdminV1alphaGlobalSiteTag {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaGlobalSiteTagBuilder {
            GoogleAnalyticsAdminV1alphaGlobalSiteTagBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A link between an GA4 property and a Google Ads account."]
    pub struct GoogleAnalyticsAdminV1alphaGoogleAdsLink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adsPersonalizationEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enable personalized advertising features with this integration. Automatically publish my Google Analytics audience lists and Google Analytics remarketing events/parameters to the linked Google Ads account. If this field is not set on create/update it will be defaulted to true."]
        pub ads_personalization_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canManageClients")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. If true, this link is for a Google Ads manager account."]
        pub can_manage_clients: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time when this link was originally created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Immutable. Google Ads customer ID."]
        pub customer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emailAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Email address of the user that created the link. An empty string will be returned if the email address can't be retrieved."]
        pub email_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Format: properties/{propertyId}/googleAdsLinks/{googleAdsLinkId} Note: googleAdsLinkId is not the Google Ads customer ID."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time when this link was last updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAnalyticsAdminV1alphaGoogleAdsLink {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaGoogleAdsLinkBuilder {
            GoogleAnalyticsAdminV1alphaGoogleAdsLinkBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A resource message representing a Google Analytics IOS app stream."]
    pub struct GoogleAnalyticsAdminV1alphaIosAppDataStream {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bundleId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Immutable. The Apple App Store Bundle ID for the app Example: \"com.example.myiosapp\""]
        pub bundle_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time when this stream was originally created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human-readable display name for the Data Stream. The max allowed display name length is 255 UTF-16 code units."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firebaseAppId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. ID of the corresponding iOS app in Firebase, if any. This ID can change if the iOS app is deleted and recreated."]
        pub firebase_app_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Resource name of this Data Stream. Format: properties/{property_id}/iosAppDataStreams/{stream_id} Example: \"properties/1000/iosAppDataStreams/2000\""]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time when stream payload fields were last updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAnalyticsAdminV1alphaIosAppDataStream {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaIosAppDataStreamBuilder {
            GoogleAnalyticsAdminV1alphaIosAppDataStreamBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ListAccountSummaries RPC."]
    pub struct GoogleAnalyticsAdminV1alphaListAccountSummariesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountSummaries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account summaries of all accounts the caller has access to."]
        pub account_summaries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleAnalyticsAdminV1alphaAccountSummary>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAnalyticsAdminV1alphaListAccountSummariesResponse {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaListAccountSummariesResponseBuilder {
            GoogleAnalyticsAdminV1alphaListAccountSummariesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for ListAccounts RPC."]
    pub struct GoogleAnalyticsAdminV1alphaListAccountsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Results that were accessible to the caller."]
        pub accounts: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleAnalyticsAdminV1alphaAccount>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAnalyticsAdminV1alphaListAccountsResponse {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaListAccountsResponseBuilder {
            GoogleAnalyticsAdminV1alphaListAccountsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for ListAndroidDataStreams RPC."]
    pub struct GoogleAnalyticsAdminV1alphaListAndroidAppDataStreamsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "androidAppDataStreams")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Results that matched the filter criteria and were accessible to the caller."]
        pub android_app_data_streams: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleAnalyticsAdminV1alphaAndroidAppDataStream>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAnalyticsAdminV1alphaListAndroidAppDataStreamsResponse {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaListAndroidAppDataStreamsResponseBuilder {
            GoogleAnalyticsAdminV1alphaListAndroidAppDataStreamsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ListFirebaseLinks RPC"]
    pub struct GoogleAnalyticsAdminV1alphaListFirebaseLinksResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firebaseLinks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of FirebaseLinks. This will have at most one value."]
        pub firebase_links: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleAnalyticsAdminV1alphaFirebaseLink>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. Currently, Google Analytics supports only one FirebaseLink per property, so this will never be populated."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAnalyticsAdminV1alphaListFirebaseLinksResponse {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaListFirebaseLinksResponseBuilder {
            GoogleAnalyticsAdminV1alphaListFirebaseLinksResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ListGoogleAdsLinks RPC."]
    pub struct GoogleAnalyticsAdminV1alphaListGoogleAdsLinksResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "googleAdsLinks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of GoogleAdsLinks."]
        pub google_ads_links: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleAnalyticsAdminV1alphaGoogleAdsLink>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAnalyticsAdminV1alphaListGoogleAdsLinksResponse {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaListGoogleAdsLinksResponseBuilder {
            GoogleAnalyticsAdminV1alphaListGoogleAdsLinksResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for ListIosAppDataStreams RPC."]
    pub struct GoogleAnalyticsAdminV1alphaListIosAppDataStreamsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iosAppDataStreams")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Results that matched the filter criteria and were accessible to the caller."]
        pub ios_app_data_streams: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleAnalyticsAdminV1alphaIosAppDataStream>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAnalyticsAdminV1alphaListIosAppDataStreamsResponse {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaListIosAppDataStreamsResponseBuilder {
            GoogleAnalyticsAdminV1alphaListIosAppDataStreamsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ListProperties RPC."]
    pub struct GoogleAnalyticsAdminV1alphaListPropertiesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Results that matched the filter criteria and were accessible to the caller."]
        pub properties: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleAnalyticsAdminV1alphaProperty>>,
        >,
    }
    impl GoogleAnalyticsAdminV1alphaListPropertiesResponse {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaListPropertiesResponseBuilder {
            GoogleAnalyticsAdminV1alphaListPropertiesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ListUserLinks RPC."]
    pub struct GoogleAnalyticsAdminV1alphaListUserLinksResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userLinks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of UserLinks. These will be ordered stably, but in an arbitrary order."]
        pub user_links: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleAnalyticsAdminV1alphaUserLink>>,
        >,
    }
    impl GoogleAnalyticsAdminV1alphaListUserLinksResponse {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaListUserLinksResponseBuilder {
            GoogleAnalyticsAdminV1alphaListUserLinksResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for ListWebDataStreams RPC."]
    pub struct GoogleAnalyticsAdminV1alphaListWebDataStreamsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webDataStreams")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Results that matched the filter criteria and were accessible to the caller."]
        pub web_data_streams: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleAnalyticsAdminV1alphaWebDataStream>>,
        >,
    }
    impl GoogleAnalyticsAdminV1alphaListWebDataStreamsResponse {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaListWebDataStreamsResponseBuilder {
            GoogleAnalyticsAdminV1alphaListWebDataStreamsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A resource message representing a Google Analytics GA4 property."]
    pub struct GoogleAnalyticsAdminV1alphaProperty {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time when the entity was originally created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currencyCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The currency type used in reports involving monetary values. Format: https://en.wikipedia.org/wiki/ISO_4217 Examples: \"USD\", \"EUR\", \"JPY\""]
        pub currency_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Indicates whether this Property is soft-deleted or not. Deleted properties are excluded from List results unless specifically requested."]
        pub deleted: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Human-readable display name for this property. The max allowed display name length is 100 UTF-16 code units."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "industryCategory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Industry associated with this property Example: AUTOMOTIVE, FOOD_AND_DRINK"]
        pub industry_category:
            ::std::option::Option<GoogleAnalyticsAdminV1alphaPropertyIndustryCategoryEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Resource name of this property. Format: properties/{property_id} Example: \"properties/1000\""]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Immutable. Resource name of this property's logical parent. Note: The Property-Moving UI can be used to change the parent. Format: accounts/{account} Example: \"accounts/100\""]
        pub parent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reporting Time Zone, used as the day boundary for reports, regardless of where the data originates. If the time zone honors DST, Analytics will automatically adjust for the changes. NOTE: Changing the time zone only affects data going forward, and is not applied retroactively. Format: https://www.iana.org/time-zones Example: \"America/Los_Angeles\""]
        pub time_zone: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time when entity payload fields were last updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAnalyticsAdminV1alphaProperty {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaPropertyBuilder {
            GoogleAnalyticsAdminV1alphaPropertyBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Industry associated with this property Example: AUTOMOTIVE, FOOD_AND_DRINK"]
    pub enum GoogleAnalyticsAdminV1alphaPropertyIndustryCategoryEnum {
        #[serde(rename = "INDUSTRY_CATEGORY_UNSPECIFIED")]
        #[doc = "Industry category unspecified"]
        IndustryCategoryUnspecified,
        #[serde(rename = "AUTOMOTIVE")]
        #[doc = "Automotive"]
        Automotive,
        #[serde(rename = "BUSINESS_AND_INDUSTRIAL_MARKETS")]
        #[doc = "Business and industrial markets"]
        BusinessAndIndustrialMarkets,
        #[serde(rename = "FINANCE")]
        #[doc = "Finance"]
        Finance,
        #[serde(rename = "HEALTHCARE")]
        #[doc = "Healthcare"]
        Healthcare,
        #[serde(rename = "TECHNOLOGY")]
        #[doc = "Technology"]
        Technology,
        #[serde(rename = "TRAVEL")]
        #[doc = "Travel"]
        Travel,
        #[serde(rename = "OTHER")]
        #[doc = "Other"]
        Other,
        #[serde(rename = "ARTS_AND_ENTERTAINMENT")]
        #[doc = "Arts and entertainment"]
        ArtsAndEntertainment,
        #[serde(rename = "BEAUTY_AND_FITNESS")]
        #[doc = "Beauty and fitness"]
        BeautyAndFitness,
        #[serde(rename = "BOOKS_AND_LITERATURE")]
        #[doc = "Books and literature"]
        BooksAndLiterature,
        #[serde(rename = "FOOD_AND_DRINK")]
        #[doc = "Food and drink"]
        FoodAndDrink,
        #[serde(rename = "GAMES")]
        #[doc = "Games"]
        Games,
        #[serde(rename = "HOBBIES_AND_LEISURE")]
        #[doc = "Hobbies and leisure"]
        HobbiesAndLeisure,
        #[serde(rename = "HOME_AND_GARDEN")]
        #[doc = "Home and garden"]
        HomeAndGarden,
        #[serde(rename = "INTERNET_AND_TELECOM")]
        #[doc = "Internet and telecom"]
        InternetAndTelecom,
        #[serde(rename = "LAW_AND_GOVERNMENT")]
        #[doc = "Law and government"]
        LawAndGovernment,
        #[serde(rename = "NEWS")]
        #[doc = "News"]
        News,
        #[serde(rename = "ONLINE_COMMUNITIES")]
        #[doc = "Online communities"]
        OnlineCommunities,
        #[serde(rename = "PEOPLE_AND_SOCIETY")]
        #[doc = "People and society"]
        PeopleAndSociety,
        #[serde(rename = "PETS_AND_ANIMALS")]
        #[doc = "Pets and animals"]
        PetsAndAnimals,
        #[serde(rename = "REAL_ESTATE")]
        #[doc = "Real estate"]
        RealEstate,
        #[serde(rename = "REFERENCE")]
        #[doc = "Reference"]
        Reference,
        #[serde(rename = "SCIENCE")]
        #[doc = "Science"]
        Science,
        #[serde(rename = "SPORTS")]
        #[doc = "Sports"]
        Sports,
        #[serde(rename = "JOBS_AND_EDUCATION")]
        #[doc = "Jobs and education"]
        JobsAndEducation,
        #[serde(rename = "SHOPPING")]
        #[doc = "Shopping"]
        Shopping,
    }
    impl ::std::default::Default for GoogleAnalyticsAdminV1alphaPropertyIndustryCategoryEnum {
        fn default() -> Self {
            Self::IndustryCategoryUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A virtual resource representing metadata for an GA4 property."]
    pub struct GoogleAnalyticsAdminV1alphaPropertySummary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display name for the property referred to in this account summary."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name of property referred to by this property summary Format: properties/{property_id} Example: \"properties/1000\""]
        pub property: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAnalyticsAdminV1alphaPropertySummary {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaPropertySummaryBuilder {
            GoogleAnalyticsAdminV1alphaPropertySummaryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for ProvisionAccountTicket RPC."]
    pub struct GoogleAnalyticsAdminV1alphaProvisionAccountTicketRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "account")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The account to create."]
        pub account: ::std::option::Option<::std::boxed::Box<GoogleAnalyticsAdminV1alphaAccount>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "redirectUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Redirect URI where the user will be sent after accepting Terms of Service. Must be configured in Developers Console as a Redirect URI"]
        pub redirect_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAnalyticsAdminV1alphaProvisionAccountTicketRequest {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaProvisionAccountTicketRequestBuilder {
            GoogleAnalyticsAdminV1alphaProvisionAccountTicketRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ProvisionAccountTicket RPC."]
    pub struct GoogleAnalyticsAdminV1alphaProvisionAccountTicketResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountTicketId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The param to be passed in the ToS link."]
        pub account_ticket_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAnalyticsAdminV1alphaProvisionAccountTicketResponse {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaProvisionAccountTicketResponseBuilder {
            GoogleAnalyticsAdminV1alphaProvisionAccountTicketResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for UpdateUserLink RPC."]
    pub struct GoogleAnalyticsAdminV1alphaUpdateUserLinkRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The user link to update."]
        pub user_link:
            ::std::option::Option<::std::boxed::Box<GoogleAnalyticsAdminV1alphaUserLink>>,
    }
    impl GoogleAnalyticsAdminV1alphaUpdateUserLinkRequest {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaUpdateUserLinkRequestBuilder {
            GoogleAnalyticsAdminV1alphaUpdateUserLinkRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A resource message representing a user's permissions on an Account or Property resource."]
    pub struct GoogleAnalyticsAdminV1alphaUserLink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "directRoles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Roles directly assigned to this user for this account or property. Valid values: predefinedRoles/read predefinedRoles/collaborate predefinedRoles/edit predefinedRoles/manage-users Excludes roles that are inherited from a higher-level entity, group, or organization admin role. A UserLink that is updated to have an empty list of direct_roles will be deleted."]
        pub direct_roles: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emailAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email address of the user to link"]
        pub email_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Example format: properties/1234/userLinks/5678"]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAnalyticsAdminV1alphaUserLink {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaUserLinkBuilder {
            GoogleAnalyticsAdminV1alphaUserLinkBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A resource message representing a Google Analytics web stream."]
    pub struct GoogleAnalyticsAdminV1alphaWebDataStream {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time when this stream was originally created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Immutable. Domain name of the web app being measured, or empty. Example: \"http://www.google.com\", \"https://www.google.com\""]
        pub default_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Human-readable display name for the Data Stream. The max allowed display name length is 100 UTF-16 code units."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firebaseAppId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. ID of the corresponding web app in Firebase, if any. This ID can change if the web app is deleted and recreated."]
        pub firebase_app_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "measurementId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Analytics \"Measurement ID\", without the \"G-\" prefix. Example: \"G-1A2BCD345E\" would just be \"1A2BCD345E\""]
        pub measurement_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Resource name of this Data Stream. Format: properties/{property_id}/webDataStreams/{stream_id} Example: \"properties/1000/webDataStreams/2000\""]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time when stream payload fields were last updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAnalyticsAdminV1alphaWebDataStream {
        pub fn builder() -> GoogleAnalyticsAdminV1alphaWebDataStreamBuilder {
            GoogleAnalyticsAdminV1alphaWebDataStreamBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
    pub struct GoogleProtobufEmpty {}
    impl GoogleProtobufEmpty {
        pub fn builder() -> GoogleProtobufEmptyBuilder {
            GoogleProtobufEmptyBuilder::default()
        }
    }
}
