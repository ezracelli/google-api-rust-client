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
            pub mod data_sources {
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
                            #[doc = "Page size. The default page size is the maximum value of 1000 results."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Pagination token, which can be used to request a specific page of `ListDataSourcesRequest` list results. For multiple-page results, `ListDataSourcesResponse` outputs a `next_page` token, which can be used as the `page_token` value to request the next page of list results."]
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
            pub mod locations {
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
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The standard list filter."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The standard list page size."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The standard list page token."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
                pub mod resources {
                    pub mod data_sources {
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
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Page size. The default page size is the maximum value of 1000 results."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Pagination token, which can be used to request a specific page of `ListDataSourcesRequest` list results. For multiple-page results, `ListDataSourcesResponse` outputs a `next_page` token, which can be used as the `page_token` value to request the next page of list results."]
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
                    pub mod transfer_configs {
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
                                    #[serde(rename = "authorizationCode")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional OAuth2 authorization code to use with this transfer configuration. This is required if new credentials are needed, as indicated by `CheckValidCreds`. In order to obtain authorization_code, please make a request to https://www.gstatic.com/bigquerydatatransfer/oauthz/auth?client_id=&scope=&redirect_uri= * client_id should be OAuth client_id of BigQuery DTS API for the given data source returned by ListDataSources method. * data_source_scopes are the scopes returned by ListDataSources method. * redirect_uri is an optional parameter. If not specified, then authorization code is posted to the opener of authorization flow window. Otherwise it will be sent to the redirect uri. A special value of urn:ietf:wg:oauth:2.0:oob means that authorization code should be returned in the title bar of the browser, with the page text prompting the user to copy the code and paste it in the application."]
                                    pub authorization_code:
                                        ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "serviceAccountName")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional service account name. If this field is set, transfer config will be created with this service account credentials. It requires that requesting user calling this API has permissions to act as this service account."]
                                    pub service_account_name:
                                        ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "versionInfo")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional version info. If users want to find a very recent access token, that is, immediately after approving access, users have to set the version_info claim in the token request. To obtain the version_info, users must use the \"none+gsession\" response type. which be return a version_info back in the authorization response which be be put in a JWT claim in the token request."]
                                    pub version_info: ::std::option::Option<::std::string::String>,
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
                                    #[serde(rename = "dataSourceIds")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "When specified, only configurations of requested data sources are returned."]
                                    pub data_source_ids:
                                        ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Page size. The default page size is the maximum value of 1000 results."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Pagination token, which can be used to request a specific page of `ListTransfersRequest` list results. For multiple-page results, `ListTransfersResponse` outputs a `next_page` token, which can be used as the `page_token` value to request the next page of list results."]
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
                                    #[serde(rename = "authorizationCode")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional OAuth2 authorization code to use with this transfer configuration. If it is provided, the transfer configuration will be associated with the authorizing user. In order to obtain authorization_code, please make a request to https://www.gstatic.com/bigquerydatatransfer/oauthz/auth?client_id=&scope=&redirect_uri= * client_id should be OAuth client_id of BigQuery DTS API for the given data source returned by ListDataSources method. * data_source_scopes are the scopes returned by ListDataSources method. * redirect_uri is an optional parameter. If not specified, then authorization code is posted to the opener of authorization flow window. Otherwise it will be sent to the redirect uri. A special value of urn:ietf:wg:oauth:2.0:oob means that authorization code should be returned in the title bar of the browser, with the page text prompting the user to copy the code and paste it in the application."]
                                    pub authorization_code:
                                        ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "serviceAccountName")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional service account name. If this field is set and \"service_account_name\" is set in update_mask, transfer config will be updated to use this service account credentials. It requires that requesting user calling this API has permissions to act as this service account."]
                                    pub service_account_name:
                                        ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "updateMask")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Required. Required list of fields to be updated in this request."]
                                    pub update_mask: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "versionInfo")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional version info. If users want to find a very recent access token, that is, immediately after approving access, users have to set the version_info claim in the token request. To obtain the version_info, users must use the \"none+gsession\" response type. which be return a version_info back in the authorization response which be be put in a JWT claim in the token request."]
                                    pub version_info: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                        }
                        pub mod resources {
                            pub mod runs {
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
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Page size. The default page size is the maximum value of 1000 results."]
                                            pub page_size:
                                                ::std::option::Option<::std::primitive::i64>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Pagination token, which can be used to request a specific page of `ListTransferRunsRequest` list results. For multiple-page results, `ListTransferRunsResponse` outputs a `next_page` token, which can be used as the `page_token` value to request the next page of list results."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "runAttempt")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Indicates how run attempts are to be pulled."]
                                            pub run_attempt: ::std::option::Option<
                                                QueryParametersRunAttemptEnum,
                                            >,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "states")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "When specified, only transfer runs with requested states are returned."]
                                            pub states:
                                                ::std::option::Option<QueryParametersStatesEnum>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                        #[derive(
                                            Debug,
                                            PartialEq,
                                            Copy,
                                            Clone,
                                            serde :: Serialize,
                                            serde :: Deserialize,
                                        )]
                                        #[doc = "Indicates how run attempts are to be pulled."]
                                        pub enum QueryParametersRunAttemptEnum {
                                            #[serde(rename = "RUN_ATTEMPT_UNSPECIFIED")]
                                            #[doc = "All runs should be returned."]
                                            RunAttemptUnspecified,
                                            #[serde(rename = "LATEST")]
                                            #[doc = "Only latest run per day should be returned."]
                                            Latest,
                                        }
                                        impl ::std::default::Default for QueryParametersRunAttemptEnum {
                                            fn default() -> Self {
                                                Self::RunAttemptUnspecified
                                            }
                                        }
                                        #[derive(
                                            Debug,
                                            PartialEq,
                                            Copy,
                                            Clone,
                                            serde :: Serialize,
                                            serde :: Deserialize,
                                        )]
                                        #[doc = "When specified, only transfer runs with requested states are returned."]
                                        pub enum QueryParametersStatesEnum {
                                            #[serde(rename = "TRANSFER_STATE_UNSPECIFIED")]
                                            #[doc = "State placeholder."]
                                            TransferStateUnspecified,
                                            #[serde(rename = "PENDING")]
                                            #[doc = "Data transfer is scheduled and is waiting to be picked up by data transfer backend."]
                                            Pending,
                                            #[serde(rename = "RUNNING")]
                                            #[doc = "Data transfer is in progress."]
                                            Running,
                                            #[serde(rename = "SUCCEEDED")]
                                            #[doc = "Data transfer completed successfully."]
                                            Succeeded,
                                            #[serde(rename = "FAILED")]
                                            #[doc = "Data transfer failed."]
                                            Failed,
                                            #[serde(rename = "CANCELLED")]
                                            #[doc = "Data transfer is cancelled."]
                                            Cancelled,
                                        }
                                        impl ::std::default::Default for QueryParametersStatesEnum {
                                            fn default() -> Self {
                                                Self::TransferStateUnspecified
                                            }
                                        }
                                    }
                                }
                                pub mod resources {
                                    pub mod transfer_logs {
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
                                                    #[serde(rename = "messageTypes")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "Message types to return. If not populated - INFO, WARNING and ERROR messages are returned."]
                                                    pub message_types: ::std::option::Option<
                                                        QueryParametersMessageTypesEnum,
                                                    >,
                                                    #[builder(
                                                        default = "{ ::std::default::Default::default() }",
                                                        setter(into)
                                                    )]
                                                    #[serde(rename = "pageSize")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "Page size. The default page size is the maximum value of 1000 results."]
                                                    pub page_size: ::std::option::Option<
                                                        ::std::primitive::i64,
                                                    >,
                                                    #[builder(
                                                        default = "{ ::std::default::Default::default() }",
                                                        setter(into)
                                                    )]
                                                    #[serde(rename = "pageToken")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "Pagination token, which can be used to request a specific page of `ListTransferLogsRequest` list results. For multiple-page results, `ListTransferLogsResponse` outputs a `next_page` token, which can be used as the `page_token` value to request the next page of list results."]
                                                    pub page_token: ::std::option::Option<
                                                        ::std::string::String,
                                                    >,
                                                }
                                                impl QueryParameters {
                                                    pub fn builder() -> QueryParametersBuilder {
                                                        QueryParametersBuilder::default()
                                                    }
                                                }
                                                #[derive(
                                                    Debug,
                                                    PartialEq,
                                                    Copy,
                                                    Clone,
                                                    serde :: Serialize,
                                                    serde :: Deserialize,
                                                )]
                                                #[doc = "Message types to return. If not populated - INFO, WARNING and ERROR messages are returned."]
                                                pub enum QueryParametersMessageTypesEnum {
                                                    #[serde(
                                                        rename = "MESSAGE_SEVERITY_UNSPECIFIED"
                                                    )]
                                                    #[doc = "No severity specified."]
                                                    MessageSeverityUnspecified,
                                                    #[serde(rename = "INFO")]
                                                    #[doc = "Informational message."]
                                                    Info,
                                                    #[serde(rename = "WARNING")]
                                                    #[doc = "Warning message."]
                                                    Warning,
                                                    #[serde(rename = "ERROR")]
                                                    #[doc = "Error message."]
                                                    Error,
                                                }
                                                impl ::std::default::Default for QueryParametersMessageTypesEnum {
                                                    fn default() -> Self {
                                                        Self::MessageSeverityUnspecified
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
            }
            pub mod transfer_configs {
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
                            #[serde(rename = "authorizationCode")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional OAuth2 authorization code to use with this transfer configuration. This is required if new credentials are needed, as indicated by `CheckValidCreds`. In order to obtain authorization_code, please make a request to https://www.gstatic.com/bigquerydatatransfer/oauthz/auth?client_id=&scope=&redirect_uri= * client_id should be OAuth client_id of BigQuery DTS API for the given data source returned by ListDataSources method. * data_source_scopes are the scopes returned by ListDataSources method. * redirect_uri is an optional parameter. If not specified, then authorization code is posted to the opener of authorization flow window. Otherwise it will be sent to the redirect uri. A special value of urn:ietf:wg:oauth:2.0:oob means that authorization code should be returned in the title bar of the browser, with the page text prompting the user to copy the code and paste it in the application."]
                            pub authorization_code: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "serviceAccountName")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional service account name. If this field is set, transfer config will be created with this service account credentials. It requires that requesting user calling this API has permissions to act as this service account."]
                            pub service_account_name: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "versionInfo")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional version info. If users want to find a very recent access token, that is, immediately after approving access, users have to set the version_info claim in the token request. To obtain the version_info, users must use the \"none+gsession\" response type. which be return a version_info back in the authorization response which be be put in a JWT claim in the token request."]
                            pub version_info: ::std::option::Option<::std::string::String>,
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
                            #[serde(rename = "dataSourceIds")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "When specified, only configurations of requested data sources are returned."]
                            pub data_source_ids: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Page size. The default page size is the maximum value of 1000 results."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Pagination token, which can be used to request a specific page of `ListTransfersRequest` list results. For multiple-page results, `ListTransfersResponse` outputs a `next_page` token, which can be used as the `page_token` value to request the next page of list results."]
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
                            #[serde(rename = "authorizationCode")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional OAuth2 authorization code to use with this transfer configuration. If it is provided, the transfer configuration will be associated with the authorizing user. In order to obtain authorization_code, please make a request to https://www.gstatic.com/bigquerydatatransfer/oauthz/auth?client_id=&scope=&redirect_uri= * client_id should be OAuth client_id of BigQuery DTS API for the given data source returned by ListDataSources method. * data_source_scopes are the scopes returned by ListDataSources method. * redirect_uri is an optional parameter. If not specified, then authorization code is posted to the opener of authorization flow window. Otherwise it will be sent to the redirect uri. A special value of urn:ietf:wg:oauth:2.0:oob means that authorization code should be returned in the title bar of the browser, with the page text prompting the user to copy the code and paste it in the application."]
                            pub authorization_code: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "serviceAccountName")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional service account name. If this field is set and \"service_account_name\" is set in update_mask, transfer config will be updated to use this service account credentials. It requires that requesting user calling this API has permissions to act as this service account."]
                            pub service_account_name: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. Required list of fields to be updated in this request."]
                            pub update_mask: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "versionInfo")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional version info. If users want to find a very recent access token, that is, immediately after approving access, users have to set the version_info claim in the token request. To obtain the version_info, users must use the \"none+gsession\" response type. which be return a version_info back in the authorization response which be be put in a JWT claim in the token request."]
                            pub version_info: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
                pub mod resources {
                    pub mod runs {
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
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Page size. The default page size is the maximum value of 1000 results."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Pagination token, which can be used to request a specific page of `ListTransferRunsRequest` list results. For multiple-page results, `ListTransferRunsResponse` outputs a `next_page` token, which can be used as the `page_token` value to request the next page of list results."]
                                    pub page_token: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "runAttempt")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Indicates how run attempts are to be pulled."]
                                    pub run_attempt:
                                        ::std::option::Option<QueryParametersRunAttemptEnum>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "states")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "When specified, only transfer runs with requested states are returned."]
                                    pub states: ::std::option::Option<QueryParametersStatesEnum>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                                #[derive(
                                    Debug,
                                    PartialEq,
                                    Copy,
                                    Clone,
                                    serde :: Serialize,
                                    serde :: Deserialize,
                                )]
                                #[doc = "Indicates how run attempts are to be pulled."]
                                pub enum QueryParametersRunAttemptEnum {
                                    #[serde(rename = "RUN_ATTEMPT_UNSPECIFIED")]
                                    #[doc = "All runs should be returned."]
                                    RunAttemptUnspecified,
                                    #[serde(rename = "LATEST")]
                                    #[doc = "Only latest run per day should be returned."]
                                    Latest,
                                }
                                impl ::std::default::Default for QueryParametersRunAttemptEnum {
                                    fn default() -> Self {
                                        Self::RunAttemptUnspecified
                                    }
                                }
                                #[derive(
                                    Debug,
                                    PartialEq,
                                    Copy,
                                    Clone,
                                    serde :: Serialize,
                                    serde :: Deserialize,
                                )]
                                #[doc = "When specified, only transfer runs with requested states are returned."]
                                pub enum QueryParametersStatesEnum {
                                    #[serde(rename = "TRANSFER_STATE_UNSPECIFIED")]
                                    #[doc = "State placeholder."]
                                    TransferStateUnspecified,
                                    #[serde(rename = "PENDING")]
                                    #[doc = "Data transfer is scheduled and is waiting to be picked up by data transfer backend."]
                                    Pending,
                                    #[serde(rename = "RUNNING")]
                                    #[doc = "Data transfer is in progress."]
                                    Running,
                                    #[serde(rename = "SUCCEEDED")]
                                    #[doc = "Data transfer completed successfully."]
                                    Succeeded,
                                    #[serde(rename = "FAILED")]
                                    #[doc = "Data transfer failed."]
                                    Failed,
                                    #[serde(rename = "CANCELLED")]
                                    #[doc = "Data transfer is cancelled."]
                                    Cancelled,
                                }
                                impl ::std::default::Default for QueryParametersStatesEnum {
                                    fn default() -> Self {
                                        Self::TransferStateUnspecified
                                    }
                                }
                            }
                        }
                        pub mod resources {
                            pub mod transfer_logs {
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
                                            #[serde(rename = "messageTypes")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Message types to return. If not populated - INFO, WARNING and ERROR messages are returned."]
                                            pub message_types: ::std::option::Option<
                                                QueryParametersMessageTypesEnum,
                                            >,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Page size. The default page size is the maximum value of 1000 results."]
                                            pub page_size:
                                                ::std::option::Option<::std::primitive::i64>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Pagination token, which can be used to request a specific page of `ListTransferLogsRequest` list results. For multiple-page results, `ListTransferLogsResponse` outputs a `next_page` token, which can be used as the `page_token` value to request the next page of list results."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                        #[derive(
                                            Debug,
                                            PartialEq,
                                            Copy,
                                            Clone,
                                            serde :: Serialize,
                                            serde :: Deserialize,
                                        )]
                                        #[doc = "Message types to return. If not populated - INFO, WARNING and ERROR messages are returned."]
                                        pub enum QueryParametersMessageTypesEnum {
                                            #[serde(rename = "MESSAGE_SEVERITY_UNSPECIFIED")]
                                            #[doc = "No severity specified."]
                                            MessageSeverityUnspecified,
                                            #[serde(rename = "INFO")]
                                            #[doc = "Informational message."]
                                            Info,
                                            #[serde(rename = "WARNING")]
                                            #[doc = "Warning message."]
                                            Warning,
                                            #[serde(rename = "ERROR")]
                                            #[doc = "Error message."]
                                            Error,
                                        }
                                        impl ::std::default::Default for QueryParametersMessageTypesEnum {
                                            fn default() -> Self {
                                                Self::MessageSeverityUnspecified
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
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to determine whether the user has valid credentials. This method is used to limit the number of OAuth popups in the user interface. The user id is inferred from the API call context. If the data source has the Google+ authorization type, this method returns false, as it cannot be determined whether the credentials are already valid merely based on the user id."]
    pub struct CheckValidCredsRequest {}
    impl CheckValidCredsRequest {
        pub fn builder() -> CheckValidCredsRequestBuilder {
            CheckValidCredsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A response indicating whether the credentials exist and are valid."]
    pub struct CheckValidCredsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hasValidCreds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set to `true`, the credentials exist and are valid."]
        pub has_valid_creds: ::std::option::Option<::std::primitive::bool>,
    }
    impl CheckValidCredsResponse {
        pub fn builder() -> CheckValidCredsResponseBuilder {
            CheckValidCredsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents data source metadata. Metadata is sufficient to render UI and request proper OAuth tokens."]
    pub struct DataSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authorizationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates the type of authorization."]
        pub authorization_type: ::std::option::Option<DataSourceAuthorizationTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data source client id which should be used to receive refresh token."]
        pub client_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataRefreshType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies whether the data source supports automatic data refresh for the past few days, and how it's supported. For some data sources, data might not be complete until a few days later, so it's useful to refresh data automatically."]
        pub data_refresh_type: ::std::option::Option<DataSourceDataRefreshTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data source id."]
        pub data_source_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultDataRefreshWindowDays")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Default data refresh window on days. Only meaningful when `data_refresh_type` = `SLIDING_WINDOW`."]
        pub default_data_refresh_window_days: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultSchedule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Default data transfer schedule. Examples of valid schedules include: `1st,3rd monday of month 15:30`, `every wed,fri of jan,jun 13:15`, and `first sunday of quarter 00:00`."]
        pub default_schedule: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User friendly data source description string."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User friendly data source name."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "helpUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Url for the help document for this data source."]
        pub help_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "manualRunsDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Disables backfilling and manual run scheduling for the data source."]
        pub manual_runs_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minimumScheduleInterval")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minimum interval for scheduler to schedule runs."]
        pub minimum_schedule_interval: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Data source resource name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data source parameters."]
        pub parameters:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataSourceParameter>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scopes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Api auth scopes for which refresh token needs to be obtained. These are scopes needed by a data source to prepare data and ingest them into BigQuery, e.g., https://www.googleapis.com/auth/bigquery"]
        pub scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "supportsCustomSchedule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies whether the data source supports a user defined schedule, or operates on the default schedule. When set to `true`, user can override default schedule."]
        pub supports_custom_schedule: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "supportsMultipleTransfers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This field has no effect."]
        pub supports_multiple_transfers: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transferType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This field has no effect."]
        pub transfer_type: ::std::option::Option<DataSourceTransferTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateDeadlineSeconds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of seconds to wait for an update from the data source before the Data Transfer Service marks the transfer as FAILED."]
        pub update_deadline_seconds: ::std::option::Option<::std::primitive::i64>,
    }
    impl DataSource {
        pub fn builder() -> DataSourceBuilder {
            DataSourceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Indicates the type of authorization."]
    pub enum DataSourceAuthorizationTypeEnum {
        #[serde(rename = "AUTHORIZATION_TYPE_UNSPECIFIED")]
        #[doc = "Type unspecified."]
        AuthorizationTypeUnspecified,
        #[serde(rename = "AUTHORIZATION_CODE")]
        #[doc = "Use OAuth 2 authorization codes that can be exchanged for a refresh token on the backend."]
        AuthorizationCode,
        #[serde(rename = "GOOGLE_PLUS_AUTHORIZATION_CODE")]
        #[doc = "Return an authorization code for a given Google+ page that can then be exchanged for a refresh token on the backend."]
        GooglePlusAuthorizationCode,
        #[serde(rename = "FIRST_PARTY_OAUTH")]
        #[doc = "Use First Party OAuth based on Loas Owned Clients. First Party OAuth doesn't require a refresh token to get an offline access token. Instead, it uses a client-signed JWT assertion to retrieve an access token."]
        FirstPartyOauth,
    }
    impl ::std::default::Default for DataSourceAuthorizationTypeEnum {
        fn default() -> Self {
            Self::AuthorizationTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies whether the data source supports automatic data refresh for the past few days, and how it's supported. For some data sources, data might not be complete until a few days later, so it's useful to refresh data automatically."]
    pub enum DataSourceDataRefreshTypeEnum {
        #[serde(rename = "DATA_REFRESH_TYPE_UNSPECIFIED")]
        #[doc = "The data source won't support data auto refresh, which is default value."]
        DataRefreshTypeUnspecified,
        #[serde(rename = "SLIDING_WINDOW")]
        #[doc = "The data source supports data auto refresh, and runs will be scheduled for the past few days. Does not allow custom values to be set for each transfer config."]
        SlidingWindow,
        #[serde(rename = "CUSTOM_SLIDING_WINDOW")]
        #[doc = "The data source supports data auto refresh, and runs will be scheduled for the past few days. Allows custom values to be set for each transfer config."]
        CustomSlidingWindow,
    }
    impl ::std::default::Default for DataSourceDataRefreshTypeEnum {
        fn default() -> Self {
            Self::DataRefreshTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Deprecated. This field has no effect."]
    pub enum DataSourceTransferTypeEnum {
        #[serde(rename = "TRANSFER_TYPE_UNSPECIFIED")]
        #[doc = "Invalid or Unknown transfer type placeholder."]
        TransferTypeUnspecified,
        #[serde(rename = "BATCH")]
        #[doc = "Batch data transfer."]
        Batch,
        #[serde(rename = "STREAMING")]
        #[doc = "Streaming data transfer. Streaming data source currently doesn't support multiple transfer configs per project."]
        Streaming,
    }
    impl ::std::default::Default for DataSourceTransferTypeEnum {
        fn default() -> Self {
            Self::TransferTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a data source parameter with validation rules, so that parameters can be rendered in the UI. These parameters are given to us by supported data sources, and include all needed information for rendering and validation. Thus, whoever uses this api can decide to generate either generic ui, or custom data source specific forms."]
    pub struct DataSourceParameter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowedValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All possible values for the parameter."]
        pub allowed_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deprecated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, it should not be used in new transfers, and it should not be visible to users."]
        pub deprecated: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parameter description."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parameter display name in the user interface."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This field has no effect."]
        pub fields: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataSourceParameter>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "immutable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cannot be changed after initial creation."]
        pub immutable: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For integer and double values specifies maxminum allowed value."]
        pub max_value: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For integer and double values specifies minimum allowed value."]
        pub min_value: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paramId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parameter identifier."]
        pub param_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recurse")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This field has no effect."]
        pub recurse: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "repeated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This field has no effect."]
        pub repeated: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "required")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Is parameter required."]
        pub required: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parameter type."]
        pub _type: ::std::option::Option<DataSourceParameterTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "validationDescription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the requirements for this field, in case the user input does not fulfill the regex pattern or min/max values."]
        pub validation_description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "validationHelpUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL to a help document to further explain the naming requirements."]
        pub validation_help_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "validationRegex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Regular expression which can be used for parameter validation."]
        pub validation_regex: ::std::option::Option<::std::string::String>,
    }
    impl DataSourceParameter {
        pub fn builder() -> DataSourceParameterBuilder {
            DataSourceParameterBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Parameter type."]
    pub enum DataSourceParameterTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "Type unspecified."]
        TypeUnspecified,
        #[serde(rename = "STRING")]
        #[doc = "String parameter."]
        String,
        #[serde(rename = "INTEGER")]
        #[doc = "Integer parameter (64-bits). Will be serialized to json as string."]
        Integer,
        #[serde(rename = "DOUBLE")]
        #[doc = "Double precision floating point parameter."]
        Double,
        #[serde(rename = "BOOLEAN")]
        #[doc = "Boolean parameter."]
        Boolean,
        #[serde(rename = "RECORD")]
        #[doc = "Deprecated. This field has no effect."]
        Record,
        #[serde(rename = "PLUS_PAGE")]
        #[doc = "Page ID for a Google+ Page."]
        PlusPage,
    }
    impl ::std::default::Default for DataSourceParameterTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents preferences for sending email notifications for transfer run events."]
    pub struct EmailPreferences {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableFailureEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, email notifications will be sent on transfer run failures."]
        pub enable_failure_email: ::std::option::Option<::std::primitive::bool>,
    }
    impl EmailPreferences {
        pub fn builder() -> EmailPreferencesBuilder {
            EmailPreferencesBuilder::default()
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
    #[doc = "Returns list of supported data sources and their metadata."]
    pub struct ListDataSourcesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of supported data sources and their transfer settings."]
        pub data_sources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataSource>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The next-pagination token. For multiple-page list results, this token can be used as the `ListDataSourcesRequest.page_token` to request the next page of list results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListDataSourcesResponse {
        pub fn builder() -> ListDataSourcesResponseBuilder {
            ListDataSourcesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Locations.ListLocations."]
    pub struct ListLocationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of locations that matches the specified filter in the request."]
        pub locations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Location>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The standard List next-page token."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListLocationsResponse {
        pub fn builder() -> ListLocationsResponseBuilder {
            ListLocationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The returned list of pipelines in the project."]
    pub struct ListTransferConfigsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The next-pagination token. For multiple-page list results, this token can be used as the `ListTransferConfigsRequest.page_token` to request the next page of list results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transferConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The stored pipeline transfer configurations."]
        pub transfer_configs:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TransferConfig>>>,
    }
    impl ListTransferConfigsResponse {
        pub fn builder() -> ListTransferConfigsResponseBuilder {
            ListTransferConfigsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The returned list transfer run messages."]
    pub struct ListTransferLogsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The next-pagination token. For multiple-page list results, this token can be used as the `GetTransferRunLogRequest.page_token` to request the next page of list results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transferMessages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The stored pipeline transfer messages."]
        pub transfer_messages:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TransferMessage>>>,
    }
    impl ListTransferLogsResponse {
        pub fn builder() -> ListTransferLogsResponseBuilder {
            ListTransferLogsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The returned list of pipelines in the project."]
    pub struct ListTransferRunsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The next-pagination token. For multiple-page list results, this token can be used as the `ListTransferRunsRequest.page_token` to request the next page of list results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transferRuns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The stored pipeline transfer runs."]
        pub transfer_runs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TransferRun>>>,
    }
    impl ListTransferRunsResponse {
        pub fn builder() -> ListTransferRunsResponseBuilder {
            ListTransferRunsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A resource that represents Google Cloud Platform location."]
    pub struct Location {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The friendly name for this location, typically a nearby city name. For example, \"Tokyo\"."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cross-service attributes for the location. For example {\"cloud.googleapis.com/region\": \"us-east1\"}"]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The canonical id for this location. For example: `\"us-east1\"`."]
        pub location_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Service-specific metadata. For example the available capacity at the given location."]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name for the location, which may vary between implementations. For example: `\"projects/example-project/locations/us-east1\"`"]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl Location {
        pub fn builder() -> LocationBuilder {
            LocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Options customizing the data transfer schedule."]
    pub struct ScheduleOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disableAutoScheduling")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, automatic scheduling of data transfer runs for this configuration will be disabled. The runs can be started on ad-hoc basis using StartManualTransferRuns API. When automatic scheduling is disabled, the TransferConfig.schedule field will be ignored."]
        pub disable_auto_scheduling: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines time to stop scheduling transfer runs. A transfer run cannot be scheduled at or after the end time. The end time can be changed at any moment. The time when a data transfer can be trigerred manually is not limited by this option."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies time to start scheduling transfer runs. The first run will be scheduled at or after the start time according to a recurrence pattern defined in the schedule string. The start time can be changed at any moment. The time when a data transfer can be trigerred manually is not limited by this option."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl ScheduleOptions {
        pub fn builder() -> ScheduleOptionsBuilder {
            ScheduleOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to schedule transfer runs for a time range."]
    pub struct ScheduleTransferRunsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. End time of the range of transfer runs. For example, `\"2017-05-30T00:00:00+00:00\"`."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Start time of the range of transfer runs. For example, `\"2017-05-25T00:00:00+00:00\"`."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl ScheduleTransferRunsRequest {
        pub fn builder() -> ScheduleTransferRunsRequestBuilder {
            ScheduleTransferRunsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A response to schedule transfer runs for a time range."]
    pub struct ScheduleTransferRunsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "runs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The transfer runs that were scheduled."]
        pub runs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TransferRun>>>,
    }
    impl ScheduleTransferRunsResponse {
        pub fn builder() -> ScheduleTransferRunsResponseBuilder {
            ScheduleTransferRunsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to start manual transfer runs."]
    pub struct StartManualTransferRunsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestedRunTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specific run_time for a transfer run to be started. The requested_run_time must not be in the future."]
        pub requested_run_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestedTimeRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time range for the transfer runs that should be started."]
        pub requested_time_range: ::std::option::Option<::std::boxed::Box<TimeRange>>,
    }
    impl StartManualTransferRunsRequest {
        pub fn builder() -> StartManualTransferRunsRequestBuilder {
            StartManualTransferRunsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A response to start manual transfer runs."]
    pub struct StartManualTransferRunsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "runs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The transfer runs that were created."]
        pub runs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TransferRun>>>,
    }
    impl StartManualTransferRunsResponse {
        pub fn builder() -> StartManualTransferRunsResponseBuilder {
            StartManualTransferRunsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
    pub struct Status {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        pub code: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "details")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of messages that carry the error details. There is a common set of message types for APIs to use."]
        pub details: ::std::option::Option<
            ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client."]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl Status {
        pub fn builder() -> StatusBuilder {
            StatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A specification for a time range, this will request transfer runs with run_time between start_time (inclusive) and end_time (exclusive)."]
    pub struct TimeRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "End time of the range of transfer runs. For example, `\"2017-05-30T00:00:00+00:00\"`. The end_time must not be in the future. Creates transfer runs where run_time is in the range between start_time (inclusive) and end_time (exclusive)."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Start time of the range of transfer runs. For example, `\"2017-05-25T00:00:00+00:00\"`. The start_time must be strictly less than the end_time. Creates transfer runs where run_time is in the range between start_time (inclusive) and end_time (exclusive)."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl TimeRange {
        pub fn builder() -> TimeRangeBuilder {
            TimeRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a data transfer configuration. A transfer configuration contains all metadata needed to perform a data transfer. For example, `destination_dataset_id` specifies where data should be stored. When a new transfer configuration is created, the specified `destination_dataset_id` is created when needed and shared with the appropriate data source service account."]
    pub struct TransferConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataRefreshWindowDays")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of days to look back to automatically refresh the data. For example, if `data_refresh_window_days = 10`, then every day BigQuery reingests data for [today-10, today-1], rather than ingesting data for just [today-1]. Only valid if the data source supports the feature. Set the value to 0 to use the default value."]
        pub data_refresh_window_days: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data source id. Cannot be changed once data transfer is created."]
        pub data_source_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datasetRegion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Region in which BigQuery dataset is located."]
        pub dataset_region: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationDatasetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The BigQuery target dataset id."]
        pub destination_dataset_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Is this config disabled. When set to true, no runs are scheduled for a given transfer."]
        pub disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User specified display name for the data transfer."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emailPreferences")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email notifications will be sent according to these preferences to the email address of the user who owns this transfer config."]
        pub email_preferences: ::std::option::Option<::std::boxed::Box<EmailPreferences>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the transfer config. Transfer config names have the form of `projects/{project_id}/locations/{region}/transferConfigs/{config_id}`. The name is automatically generated based on the config_id specified in CreateTransferConfigRequest along with project_id and region. If config_id is not provided, usually a uuid, even though it is not guaranteed or required, will be generated for config_id."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextRunTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Next time when data transfer will run."]
        pub next_run_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notificationPubsubTopic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pub/Sub topic where notifications will be sent after transfer runs associated with this transfer config finish."]
        pub notification_pubsub_topic: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "params")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data transfer specific parameters."]
        pub params:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schedule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data transfer schedule. If the data source does not support a custom schedule, this should be empty. If it is empty, the default value for the data source will be used. The specified times are in UTC. Examples of valid format: `1st,3rd monday of month 15:30`, `every wed,fri of jan,jun 13:15`, and `first sunday of quarter 00:00`. See more explanation about the format here: https://cloud.google.com/appengine/docs/flexible/python/scheduling-jobs-with-cron-yaml#the_schedule_format NOTE: the granularity should be at least 8 hours, or less frequent."]
        pub schedule: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scheduleOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Options customizing the data transfer schedule."]
        pub schedule_options: ::std::option::Option<::std::boxed::Box<ScheduleOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. State of the most recently updated transfer run."]
        pub state: ::std::option::Option<TransferConfigStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Data transfer modification time. Ignored by server on input."]
        pub update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Unique ID of the user on whose behalf transfer is done."]
        pub user_id: ::std::option::Option<::std::string::String>,
    }
    impl TransferConfig {
        pub fn builder() -> TransferConfigBuilder {
            TransferConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. State of the most recently updated transfer run."]
    pub enum TransferConfigStateEnum {
        #[serde(rename = "TRANSFER_STATE_UNSPECIFIED")]
        #[doc = "State placeholder."]
        TransferStateUnspecified,
        #[serde(rename = "PENDING")]
        #[doc = "Data transfer is scheduled and is waiting to be picked up by data transfer backend."]
        Pending,
        #[serde(rename = "RUNNING")]
        #[doc = "Data transfer is in progress."]
        Running,
        #[serde(rename = "SUCCEEDED")]
        #[doc = "Data transfer completed successfully."]
        Succeeded,
        #[serde(rename = "FAILED")]
        #[doc = "Data transfer failed."]
        Failed,
        #[serde(rename = "CANCELLED")]
        #[doc = "Data transfer is cancelled."]
        Cancelled,
    }
    impl ::std::default::Default for TransferConfigStateEnum {
        fn default() -> Self {
            Self::TransferStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a user facing message for a particular data transfer run."]
    pub struct TransferMessage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messageText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Message text."]
        pub message_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messageTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time when message was logged."]
        pub message_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "severity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Message severity."]
        pub severity: ::std::option::Option<TransferMessageSeverityEnum>,
    }
    impl TransferMessage {
        pub fn builder() -> TransferMessageBuilder {
            TransferMessageBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Message severity."]
    pub enum TransferMessageSeverityEnum {
        #[serde(rename = "MESSAGE_SEVERITY_UNSPECIFIED")]
        #[doc = "No severity specified."]
        MessageSeverityUnspecified,
        #[serde(rename = "INFO")]
        #[doc = "Informational message."]
        Info,
        #[serde(rename = "WARNING")]
        #[doc = "Warning message."]
        Warning,
        #[serde(rename = "ERROR")]
        #[doc = "Error message."]
        Error,
    }
    impl ::std::default::Default for TransferMessageSeverityEnum {
        fn default() -> Self {
            Self::MessageSeverityUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a data transfer run."]
    pub struct TransferRun {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Data source id."]
        pub data_source_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationDatasetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The BigQuery target dataset id."]
        pub destination_dataset_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emailPreferences")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Email notifications will be sent according to these preferences to the email address of the user who owns the transfer config this run was derived from."]
        pub email_preferences: ::std::option::Option<::std::boxed::Box<EmailPreferences>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time when transfer run ended. Parameter ignored by server for input requests."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of the transfer run."]
        pub error_status: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the transfer run. Transfer run names have the form `projects/{project_id}/locations/{location}/transferConfigs/{config_id}/runs/{run_id}`. The name is ignored when creating a transfer run."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notificationPubsubTopic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Pub/Sub topic where a notification will be sent after this transfer run finishes"]
        pub notification_pubsub_topic: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "params")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Data transfer specific parameters."]
        pub params:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "runTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For batch transfer runs, specifies the date and time of the data should be ingested."]
        pub run_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schedule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Describes the schedule of this transfer run if it was created as part of a regular schedule. For batch transfer runs that are scheduled manually, this is empty. NOTE: the system might choose to delay the schedule depending on the current load, so `schedule_time` doesn't always match this."]
        pub schedule: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scheduleTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum time after which a transfer run can be started."]
        pub schedule_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time when transfer run was started. Parameter ignored by server for input requests."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data transfer run state. Ignored for input requests."]
        pub state: ::std::option::Option<TransferRunStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Last time the data transfer run state was updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Unique ID of the user on whose behalf transfer is done."]
        pub user_id: ::std::option::Option<::std::string::String>,
    }
    impl TransferRun {
        pub fn builder() -> TransferRunBuilder {
            TransferRunBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Data transfer run state. Ignored for input requests."]
    pub enum TransferRunStateEnum {
        #[serde(rename = "TRANSFER_STATE_UNSPECIFIED")]
        #[doc = "State placeholder."]
        TransferStateUnspecified,
        #[serde(rename = "PENDING")]
        #[doc = "Data transfer is scheduled and is waiting to be picked up by data transfer backend."]
        Pending,
        #[serde(rename = "RUNNING")]
        #[doc = "Data transfer is in progress."]
        Running,
        #[serde(rename = "SUCCEEDED")]
        #[doc = "Data transfer completed successfully."]
        Succeeded,
        #[serde(rename = "FAILED")]
        #[doc = "Data transfer failed."]
        Failed,
        #[serde(rename = "CANCELLED")]
        #[doc = "Data transfer is cancelled."]
        Cancelled,
    }
    impl ::std::default::Default for TransferRunStateEnum {
        fn default() -> Self {
            Self::TransferStateUnspecified
        }
    }
}
