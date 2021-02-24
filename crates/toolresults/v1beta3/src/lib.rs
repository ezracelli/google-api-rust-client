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
    pub mod projects {
        pub mod resources {
            pub mod histories {
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
                            #[serde(rename = "requestId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A unique request ID for server to detect duplicated requests. For example, a UUID. Optional, but strongly recommended."]
                            pub request_id: ::std::option::Option<::std::string::String>,
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
                            #[serde(rename = "filterByName")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If set, only return histories with the given name. Optional."]
                            pub filter_by_name: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of Histories to fetch. Default value: 20. The server will use this default if the field is not set or has a value of 0. Any value greater than 100 will be treated as 100. Optional."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A continuation token to resume the query at the next item. Optional."]
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
                    pub mod executions {
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
                                    #[serde(rename = "requestId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A unique request ID for server to detect duplicated requests. For example, a UUID. Optional, but strongly recommended."]
                                    pub request_id: ::std::option::Option<::std::string::String>,
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
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The maximum number of Executions to fetch. Default value: 25. The server will use this default if the field is not set or has a value of 0. Optional."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A continuation token to resume the query at the next item. Optional."]
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
                                    #[serde(rename = "requestId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A unique request ID for server to detect duplicated requests. For example, a UUID. Optional, but strongly recommended."]
                                    pub request_id: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                        }
                        pub mod resources {
                            pub mod environments {
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
                                            #[doc = "The maximum number of Environments to fetch. Default value: 25. The server will use this default if the field is not set or has a value of 0."]
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
                                            #[doc = "A continuation token to resume the query at the next item."]
                                            pub page_token:
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
                            pub mod steps {
                                pub mod methods {
                                    pub mod accessibility_clusters {
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
                                            #[serde(rename = "locale")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The accepted format is the canonical Unicode format with hyphen as a delimiter. Language must be lowercase, Language Script - Capitalized, Region - UPPERCASE. See http://www.unicode.org/reports/tr35/#Unicode_locale_identifier for details. Required."]
                                            pub locale:
                                                ::std::option::Option<::std::string::String>,
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
                                            #[serde(rename = "requestId")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "A unique request ID for server to detect duplicated requests. For example, a UUID. Optional, but strongly recommended."]
                                            pub request_id:
                                                ::std::option::Option<::std::string::String>,
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
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The maximum number of Steps to fetch. Default value: 25. The server will use this default if the field is not set or has a value of 0. Optional."]
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
                                            #[doc = "A continuation token to resume the query at the next item. Optional."]
                                            pub page_token:
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
                                            #[serde(rename = "requestId")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "A unique request ID for server to detect duplicated requests. For example, a UUID. Optional, but strongly recommended."]
                                            pub request_id:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                }
                                pub mod resources {
                                    pub mod perf_sample_series {
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
                                                    #[doc = "Specify one or more PerfMetricType values such as CPU to filter the result"]
                                                    pub filter: ::std::option::Option<
                                                        QueryParametersFilterEnum,
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
                                                #[doc = "Specify one or more PerfMetricType values such as CPU to filter the result"]
                                                pub enum QueryParametersFilterEnum {
                                                    #[serde(rename = "perfMetricTypeUnspecified")]
                                                    #[doc = ""]
                                                    PerfMetricTypeUnspecified,
                                                    #[serde(rename = "memory")]
                                                    #[doc = ""]
                                                    Memory,
                                                    #[serde(rename = "cpu")]
                                                    #[doc = ""]
                                                    Cpu,
                                                    #[serde(rename = "network")]
                                                    #[doc = ""]
                                                    Network,
                                                    #[serde(rename = "graphics")]
                                                    #[doc = ""]
                                                    Graphics,
                                                }
                                                impl ::std::default::Default for QueryParametersFilterEnum {
                                                    fn default() -> Self {
                                                        Self::PerfMetricTypeUnspecified
                                                    }
                                                }
                                            }
                                        }
                                        pub mod resources {
                                            pub mod samples {
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
                                                            #[doc = "The default page size is 500 samples, and the maximum size is 5000. If the page_size is greater than 5000, the effective page size will be 5000"]
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
                                                            #[doc = "Optional, the next_page_token returned in the previous response"]
                                                            pub page_token: ::std::option::Option<
                                                                ::std::string::String,
                                                            >,
                                                        }
                                                        impl QueryParameters {
                                                            pub fn builder(
                                                            ) -> QueryParametersBuilder
                                                            {
                                                                QueryParametersBuilder::default()
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    pub mod test_cases {
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
                                                    #[doc = "The maximum number of TestCases to fetch. Default value: 100. The server will use this default if the field is not set or has a value of 0. Optional."]
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
                                                    #[doc = "A continuation token to resume the query at the next item. Optional."]
                                                    pub page_token: ::std::option::Option<
                                                        ::std::string::String,
                                                    >,
                                                }
                                                impl QueryParameters {
                                                    pub fn builder() -> QueryParametersBuilder {
                                                        QueryParametersBuilder::default()
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    pub mod thumbnails {
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
                                                    #[doc = "The maximum number of thumbnails to fetch. Default value: 50. The server will use this default if the field is not set or has a value of 0. Optional."]
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
                                                    #[doc = "A continuation token to resume the query at the next item. Optional."]
                                                    pub page_token: ::std::option::Option<
                                                        ::std::string::String,
                                                    >,
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
            }
        }
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional details for an ANR crash."]
    pub struct Anr {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stackTrace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The stack trace of the ANR crash. Optional."]
        pub stack_trace: ::std::option::Option<::std::boxed::Box<StackTrace>>,
    }
    impl Anr {
        pub fn builder() -> AnrBuilder {
            AnrBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Android app information."]
    pub struct AndroidAppInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the app. Optional"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The package name of the app. Required."]
        pub package_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The internal version code of the app. Optional."]
        pub version_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version name of the app. Optional."]
        pub version_name: ::std::option::Option<::std::string::String>,
    }
    impl AndroidAppInfo {
        pub fn builder() -> AndroidAppInfoBuilder {
            AndroidAppInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A test of an Android application that can control an Android component independently of its normal lifecycle. See for more information on types of Android tests."]
    pub struct AndroidInstrumentationTest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testPackageId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The java package for the test to be executed. Required"]
        pub test_package_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testRunnerClass")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The InstrumentationTestRunner class. Required"]
        pub test_runner_class: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testTargets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Each target must be fully qualified with the package name or class name, in one of these formats: - \"package package_name\" - \"class package_name.class_name\" - \"class package_name.class_name#method_name\" If empty, all targets in the module will be run."]
        pub test_targets: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "useOrchestrator")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The flag indicates whether Android Test Orchestrator will be used to run test or not."]
        pub use_orchestrator: ::std::option::Option<::std::primitive::bool>,
    }
    impl AndroidInstrumentationTest {
        pub fn builder() -> AndroidInstrumentationTestBuilder {
            AndroidInstrumentationTestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A test of an android application that explores the application on a virtual or physical Android device, finding culprits and crashes as it goes."]
    pub struct AndroidRoboTest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appInitialActivity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The initial activity that should be used to start the app. Optional"]
        pub app_initial_activity: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bootstrapPackageId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The java package for the bootstrap. Optional"]
        pub bootstrap_package_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bootstrapRunnerClass")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The runner class for the bootstrap. Optional"]
        pub bootstrap_runner_class: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxDepth")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The max depth of the traversal stack Robo can explore. Optional"]
        pub max_depth: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxSteps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The max number of steps/actions Robo can execute. Default is no limit (0). Optional"]
        pub max_steps: ::std::option::Option<::std::primitive::i64>,
    }
    impl AndroidRoboTest {
        pub fn builder() -> AndroidRoboTestBuilder {
            AndroidRoboTestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An Android mobile test specification."]
    pub struct AndroidTest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "androidAppInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about the application under test."]
        pub android_app_info: ::std::option::Option<::std::boxed::Box<AndroidAppInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "androidInstrumentationTest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An Android instrumentation test."]
        pub android_instrumentation_test:
            ::std::option::Option<::std::boxed::Box<AndroidInstrumentationTest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "androidRoboTest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An Android robo test."]
        pub android_robo_test: ::std::option::Option<::std::boxed::Box<AndroidRoboTest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "androidTestLoop")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An Android test loop."]
        pub android_test_loop: ::std::option::Option<::std::boxed::Box<AndroidTestLoop>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testTimeout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Max time a test is allowed to run before it is automatically cancelled."]
        pub test_timeout: ::std::option::Option<::std::boxed::Box<Duration>>,
    }
    impl AndroidTest {
        pub fn builder() -> AndroidTestBuilder {
            AndroidTestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Test Loops are tests that can be launched by the app itself, determining when to run by listening for an intent."]
    pub struct AndroidTestLoop {}
    impl AndroidTestLoop {
        pub fn builder() -> AndroidTestLoopBuilder {
            AndroidTestLoopBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = " `Any` contains an arbitrary serialized protocol buffer message along with a URL that describes the type of the serialized message. Protobuf library provides support to pack/unpack Any values in the form of utility functions or additional generated methods of the Any type. Example 1: Pack and unpack a message in C++. Foo foo = ...; Any any; any.PackFrom(foo); ... if (any.UnpackTo(&foo)) { ... } Example 2: Pack and unpack a message in Java. Foo foo = ...; Any any = Any.pack(foo); ... if (any.is(Foo.class)) { foo = any.unpack(Foo.class); } Example 3: Pack and unpack a message in Python. foo = Foo(...) any = Any() any.Pack(foo) ... if any.Is(Foo.DESCRIPTOR): any.Unpack(foo) ... Example 4: Pack and unpack a message in Go foo := &pb.Foo{...} any, err := ptypes.MarshalAny(foo) ... foo := &pb.Foo{} if err := ptypes.UnmarshalAny(any, foo); err != nil { ... } The pack methods provided by protobuf library will by default use 'type.googleapis.com/full.type.name' as the type URL and the unpack methods only use the fully qualified type name after the last '/' in the type URL, for example \"foo.bar.com/x/y.z\" will yield type name \"y.z\". # JSON The JSON representation of an `Any` value uses the regular representation of the deserialized, embedded message, with an additional field `@type` which contains the type URL. Example: package google.profile; message Person { string first_name = 1; string last_name = 2; } { \"@type\": \"type.googleapis.com/google.profile.Person\", \"firstName\": , \"lastName\": } If the embedded message type is well-known and has a custom JSON representation, that representation will be embedded adding a field `value` which holds the custom JSON in addition to the `@type` field. Example (for message google.protobuf.Duration): { \"@type\": \"type.googleapis.com/google.protobuf.Duration\", \"value\": \"1.212s\" }"]
    pub struct Any {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "typeUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A URL/resource name that uniquely identifies the type of the serialized protocol buffer message. This string must contain at least one \"/\" character. The last segment of the URL's path must represent the fully qualified name of the type (as in `path/google.protobuf.Duration`). The name should be in a canonical form (e.g., leading \".\" is not accepted). In practice, teams usually precompile into the binary all types that they expect it to use in the context of Any. However, for URLs which use the scheme `http`, `https`, or no scheme, one can optionally set up a type server that maps type URLs to message definitions as follows: * If no scheme is provided, `https` is assumed. * An HTTP GET on the URL must yield a google.protobuf.Type value in binary format, or produce an error. * Applications are allowed to cache lookup results based on the URL, or have them precompiled into a binary to avoid any lookup. Therefore, binary compatibility needs to be preserved on changes to types. (Use versioned type names to manage breaking changes.) Note: this functionality is not currently available in the official protobuf release, and it is not used for type URLs beginning with type.googleapis.com. Schemes other than `http`, `https` (or the empty scheme) might be used with implementation specific semantics."]
        pub type_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Must be a valid serialized protocol buffer of the above specified type."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl Any {
        pub fn builder() -> AnyBuilder {
            AnyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AppStartTime {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullyDrawnTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The time from app start to reaching the developer-reported \"fully drawn\" time. This is only stored if the app includes a call to Activity.reportFullyDrawn(). See https://developer.android.com/topic/performance/launch-time.html#time-full"]
        pub fully_drawn_time: ::std::option::Option<::std::boxed::Box<Duration>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "initialDisplayTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time from app start to the first displayed activity being drawn, as reported in Logcat. See https://developer.android.com/topic/performance/launch-time.html#time-initial"]
        pub initial_display_time: ::std::option::Option<::std::boxed::Box<Duration>>,
    }
    impl AppStartTime {
        pub fn builder() -> AppStartTimeBuilder {
            AppStartTimeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A suggestion to use deep links for a Robo run."]
    pub struct AvailableDeepLinks {}
    impl AvailableDeepLinks {
        pub fn builder() -> AvailableDeepLinksBuilder {
            AvailableDeepLinksBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Encapsulates the metadata for basic sample series represented by a line chart"]
    pub struct BasicPerfSampleSeries {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "perfMetricType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub perf_metric_type: ::std::option::Option<BasicPerfSampleSeriesPerfMetricTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "perfUnit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub perf_unit: ::std::option::Option<BasicPerfSampleSeriesPerfUnitEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sampleSeriesLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub sample_series_label: ::std::option::Option<BasicPerfSampleSeriesSampleSeriesLabelEnum>,
    }
    impl BasicPerfSampleSeries {
        pub fn builder() -> BasicPerfSampleSeriesBuilder {
            BasicPerfSampleSeriesBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum BasicPerfSampleSeriesPerfMetricTypeEnum {
        #[serde(rename = "perfMetricTypeUnspecified")]
        #[doc = ""]
        PerfMetricTypeUnspecified,
        #[serde(rename = "memory")]
        #[doc = ""]
        Memory,
        #[serde(rename = "cpu")]
        #[doc = ""]
        Cpu,
        #[serde(rename = "network")]
        #[doc = ""]
        Network,
        #[serde(rename = "graphics")]
        #[doc = ""]
        Graphics,
    }
    impl ::std::default::Default for BasicPerfSampleSeriesPerfMetricTypeEnum {
        fn default() -> Self {
            Self::PerfMetricTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum BasicPerfSampleSeriesPerfUnitEnum {
        #[serde(rename = "perfUnitUnspecified")]
        #[doc = ""]
        PerfUnitUnspecified,
        #[serde(rename = "kibibyte")]
        #[doc = ""]
        Kibibyte,
        #[serde(rename = "percent")]
        #[doc = ""]
        Percent,
        #[serde(rename = "bytesPerSecond")]
        #[doc = ""]
        BytesPerSecond,
        #[serde(rename = "framesPerSecond")]
        #[doc = ""]
        FramesPerSecond,
        #[serde(rename = "byte")]
        #[doc = ""]
        Byte,
    }
    impl ::std::default::Default for BasicPerfSampleSeriesPerfUnitEnum {
        fn default() -> Self {
            Self::PerfUnitUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum BasicPerfSampleSeriesSampleSeriesLabelEnum {
        #[serde(rename = "sampleSeriesTypeUnspecified")]
        #[doc = ""]
        SampleSeriesTypeUnspecified,
        #[serde(rename = "memoryRssPrivate")]
        #[doc = "Memory sample series"]
        MemoryRssPrivate,
        #[serde(rename = "memoryRssShared")]
        #[doc = ""]
        MemoryRssShared,
        #[serde(rename = "memoryRssTotal")]
        #[doc = ""]
        MemoryRssTotal,
        #[serde(rename = "memoryTotal")]
        #[doc = ""]
        MemoryTotal,
        #[serde(rename = "cpuUser")]
        #[doc = "CPU sample series"]
        CpuUser,
        #[serde(rename = "cpuKernel")]
        #[doc = ""]
        CpuKernel,
        #[serde(rename = "cpuTotal")]
        #[doc = ""]
        CpuTotal,
        #[serde(rename = "ntBytesTransferred")]
        #[doc = "Network sample series"]
        NtBytesTransferred,
        #[serde(rename = "ntBytesReceived")]
        #[doc = ""]
        NtBytesReceived,
        #[serde(rename = "networkSent")]
        #[doc = ""]
        NetworkSent,
        #[serde(rename = "networkReceived")]
        #[doc = ""]
        NetworkReceived,
        #[serde(rename = "graphicsFrameRate")]
        #[doc = "Graphics sample series"]
        GraphicsFrameRate,
    }
    impl ::std::default::Default for BasicPerfSampleSeriesSampleSeriesLabelEnum {
        fn default() -> Self {
            Self::SampleSeriesTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request must provide up to a maximum of 5000 samples to be created; a larger sample size will cause an INVALID_ARGUMENT error"]
    pub struct BatchCreatePerfSamplesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "perfSamples")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of PerfSamples to create should not include existing timestamps"]
        pub perf_samples: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PerfSample>>>,
    }
    impl BatchCreatePerfSamplesRequest {
        pub fn builder() -> BatchCreatePerfSamplesRequestBuilder {
            BatchCreatePerfSamplesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct BatchCreatePerfSamplesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "perfSamples")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub perf_samples: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PerfSample>>>,
    }
    impl BatchCreatePerfSamplesResponse {
        pub fn builder() -> BatchCreatePerfSamplesResponseBuilder {
            BatchCreatePerfSamplesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A warning that Robo encountered a screen that was mostly blank; this may indicate a problem with the app."]
    pub struct BlankScreen {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "screenId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The screen id of the element"]
        pub screen_id: ::std::option::Option<::std::string::String>,
    }
    impl BlankScreen {
        pub fn builder() -> BlankScreenBuilder {
            BlankScreenBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct CpuInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpuProcessor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "description of the device processor ie '1.8 GHz hexa core 64-bit ARMv8-A'"]
        pub cpu_processor: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpuSpeedInGhz")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "the CPU clock speed in GHz"]
        pub cpu_speed_in_ghz: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numberOfCores")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "the number of CPU cores"]
        pub number_of_cores: ::std::option::Option<::std::primitive::i64>,
    }
    impl CpuInfo {
        pub fn builder() -> CpuInfoBuilder {
            CpuInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Crash dialog was detected during the test execution"]
    pub struct CrashDialogError {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "crashPackage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the package that caused the dialog."]
        pub crash_package: ::std::option::Option<::std::string::String>,
    }
    impl CrashDialogError {
        pub fn builder() -> CrashDialogErrorBuilder {
            CrashDialogErrorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A warning that device ran out of memory"]
    pub struct DeviceOutOfMemory {}
    impl DeviceOutOfMemory {
        pub fn builder() -> DeviceOutOfMemoryBuilder {
            DeviceOutOfMemoryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = " A Duration represents a signed, fixed-length span of time represented as a count of seconds and fractions of seconds at nanosecond resolution. It is independent of any calendar and concepts like \"day\" or \"month\". It is related to Timestamp in that the difference between two Timestamp values is a Duration and it can be added or subtracted from a Timestamp. Range is approximately +-10,000 years."]
    pub struct Duration {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nanos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Signed fractions of a second at nanosecond resolution of the span of time. Durations less than one second are represented with a 0 `seconds` field and a positive or negative `nanos` field. For durations of one second or more, a non-zero value for the `nanos` field must be of the same sign as the `seconds` field. Must be from -999,999,999 to +999,999,999 inclusive."]
        pub nanos: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "seconds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Signed seconds of the span of time. Must be from -315,576,000,000 to +315,576,000,000 inclusive. Note: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
        pub seconds: ::std::option::Option<::std::string::String>,
    }
    impl Duration {
        pub fn builder() -> DurationBuilder {
            DurationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional details about encountered login screens."]
    pub struct EncounteredLoginScreen {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "distinctScreens")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of encountered distinct login screens."]
        pub distinct_screens: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "screenIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Subset of login screens."]
        pub screen_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl EncounteredLoginScreen {
        pub fn builder() -> EncounteredLoginScreenBuilder {
            EncounteredLoginScreenBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional details about encountered screens with elements that are not Android UI widgets."]
    pub struct EncounteredNonAndroidUiWidgetScreen {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "distinctScreens")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of encountered distinct screens with non Android UI widgets."]
        pub distinct_screens: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "screenIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Subset of screens which contain non Android UI widgets."]
        pub screen_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl EncounteredNonAndroidUiWidgetScreen {
        pub fn builder() -> EncounteredNonAndroidUiWidgetScreenBuilder {
            EncounteredNonAndroidUiWidgetScreenBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An Environment represents the set of test runs (Steps) from the parent Execution that are configured with the same set of dimensions (Model, Version, Locale, and Orientation). Multiple such runs occur particularly because of features like sharding (splitting up a test suite to run in parallel across devices) and reruns (running a test multiple times to check for different outcomes)."]
    pub struct Environment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "completionTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time when the Environment status was set to complete. This value will be set automatically when state transitions to COMPLETE."]
        pub completion_time: ::std::option::Option<::std::boxed::Box<Timestamp>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time when the Environment was created."]
        pub creation_time: ::std::option::Option<::std::boxed::Box<Timestamp>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Dimension values describing the environment. Dimension values always consist of \"Model\", \"Version\", \"Locale\", and \"Orientation\". - In response: always set - In create request: always set - In update request: never set"]
        pub dimension_value: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<EnvironmentDimensionValueEntry>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short human-readable name to display in the UI. Maximum of 100 characters. For example: Nexus 5, API 27."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environmentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. An Environment id."]
        pub environment_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environmentResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Merged result of the environment."]
        pub environment_result: ::std::option::Option<::std::boxed::Box<MergedResult>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. An Execution id."]
        pub execution_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "historyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A History id."]
        pub history_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A Project id."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resultsStorage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location where output files are stored in the user bucket."]
        pub results_storage: ::std::option::Option<::std::boxed::Box<ResultsStorage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shardSummaries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Summaries of shards. Only one shard will present unless sharding feature is enabled in TestExecutionService."]
        pub shard_summaries:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ShardSummary>>>,
    }
    impl Environment {
        pub fn builder() -> EnvironmentBuilder {
            EnvironmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct EnvironmentDimensionValueEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl EnvironmentDimensionValueEntry {
        pub fn builder() -> EnvironmentDimensionValueEntryBuilder {
            EnvironmentDimensionValueEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An Execution represents a collection of Steps. For instance, it could represent: - a mobile test executed across a range of device configurations - a jenkins job with a build step followed by a test step The maximum size of an execution message is 1 MiB. An Execution can be updated until its state is set to COMPLETE at which point it becomes immutable."]
    pub struct Execution {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "completionTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when the Execution status transitioned to COMPLETE. This value will be set automatically when state transitions to COMPLETE. - In response: set if the execution state is COMPLETE. - In create/update request: never set"]
        pub completion_time: ::std::option::Option<::std::boxed::Box<Timestamp>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when the Execution was created. This value will be set automatically when CreateExecution is called. - In response: always set - In create/update request: never set"]
        pub creation_time: ::std::option::Option<::std::boxed::Box<Timestamp>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionDefinitions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dimensions along which different steps in this execution may vary. This must remain fixed over the life of the execution. Returns INVALID_ARGUMENT if this field is set in an update request. Returns INVALID_ARGUMENT if the same name occurs in more than one dimension_definition. Returns INVALID_ARGUMENT if the size of the list is over 100. - In response: present if set by create - In create request: optional - In update request: never set"]
        pub dimension_definitions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MatrixDimensionDefinition>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique identifier within a History for this Execution. Returns INVALID_ARGUMENT if this field is set or overwritten by the caller. - In response always set - In create/update request: never set"]
        pub execution_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outcome")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Classify the result, for example into SUCCESS or FAILURE - In response: present if set by create/update request - In create/update request: optional"]
        pub outcome: ::std::option::Option<::std::boxed::Box<Outcome>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "specification")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Lightweight information about execution request. - In response: present if set by create - In create: optional - In update: optional"]
        pub specification: ::std::option::Option<::std::boxed::Box<Specification>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The initial state is IN_PROGRESS. The only legal state transitions is from IN_PROGRESS to COMPLETE. A PRECONDITION_FAILED will be returned if an invalid transition is requested. The state can only be set to COMPLETE once. A FAILED_PRECONDITION will be returned if the state is set to COMPLETE multiple times. If the state is set to COMPLETE, all the in-progress steps within the execution will be set as COMPLETE. If the outcome of the step is not set, the outcome will be set to INCONCLUSIVE. - In response always set - In create/update request: optional"]
        pub state: ::std::option::Option<ExecutionStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testExecutionMatrixId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "TestExecution Matrix ID that the TestExecutionService uses. - In response: present if set by create - In create: optional - In update: never set"]
        pub test_execution_matrix_id: ::std::option::Option<::std::string::String>,
    }
    impl Execution {
        pub fn builder() -> ExecutionBuilder {
            ExecutionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The initial state is IN_PROGRESS. The only legal state transitions is from IN_PROGRESS to COMPLETE. A PRECONDITION_FAILED will be returned if an invalid transition is requested. The state can only be set to COMPLETE once. A FAILED_PRECONDITION will be returned if the state is set to COMPLETE multiple times. If the state is set to COMPLETE, all the in-progress steps within the execution will be set as COMPLETE. If the outcome of the step is not set, the outcome will be set to INCONCLUSIVE. - In response always set - In create/update request: optional"]
    pub enum ExecutionStateEnum {
        #[serde(rename = "unknownState")]
        #[doc = "Should never be in this state. Exists for proto deserialization backward compatibility."]
        UnknownState,
        #[serde(rename = "pending")]
        #[doc = "The Execution/Step is created, ready to run, but not running yet. If an Execution/Step is created without initial state, it is assumed that the Execution/Step is in PENDING state."]
        Pending,
        #[serde(rename = "inProgress")]
        #[doc = "The Execution/Step is in progress."]
        InProgress,
        #[serde(rename = "complete")]
        #[doc = "The finalized, immutable state. Steps/Executions in this state cannot be modified."]
        Complete,
    }
    impl ::std::default::Default for ExecutionStateEnum {
        fn default() -> Self {
            Self::UnknownState
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Failed to install the APK."]
    pub struct FailedToInstall {}
    impl FailedToInstall {
        pub fn builder() -> FailedToInstallBuilder {
            FailedToInstallBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details for an outcome with a FAILURE outcome summary."]
    pub struct FailureDetail {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "crashed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the failure was severe because the system (app) under test crashed."]
        pub crashed: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceOutOfMemory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the device ran out of memory during a test, causing the test to crash."]
        pub device_out_of_memory: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "failedRoboscript")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the Roboscript failed to complete successfully, e.g., because a Roboscript action or assertion failed or a Roboscript action could not be matched during the entire crawl."]
        pub failed_roboscript: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notInstalled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If an app is not installed and thus no test can be run with the app. This might be caused by trying to run a test on an unsupported platform."]
        pub not_installed: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "otherNativeCrash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If a native process (including any other than the app) crashed."]
        pub other_native_crash: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timedOut")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the test overran some time limit, and that is why it failed."]
        pub timed_out: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unableToCrawl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the robo was unable to crawl the app; perhaps because the app did not start."]
        pub unable_to_crawl: ::std::option::Option<::std::primitive::bool>,
    }
    impl FailureDetail {
        pub fn builder() -> FailureDetailBuilder {
            FailureDetailBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional details for a fatal exception."]
    pub struct FatalException {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stackTrace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The stack trace of the fatal exception. Optional."]
        pub stack_trace: ::std::option::Option<::std::boxed::Box<StackTrace>>,
    }
    impl FatalException {
        pub fn builder() -> FatalExceptionBuilder {
            FatalExceptionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A reference to a file."]
    pub struct FileReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI of a file stored in Google Cloud Storage. For example: http://storage.googleapis.com/mybucket/path/to/test.xml or in gsutil format: gs://mybucket/path/to/test.xml with version-specific info, gs://mybucket/path/to/test.xml#1360383693690000 An INVALID_ARGUMENT error will be returned if the URI format is not supported. - In response: always set - In create/update request: always set"]
        pub file_uri: ::std::option::Option<::std::string::String>,
    }
    impl FileReference {
        pub fn builder() -> FileReferenceBuilder {
            FileReferenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Graphics statistics for the App. The information is collected from 'adb shell dumpsys graphicsstats'. For more info see: https://developer.android.com/training/testing/performance.html Statistics will only be present for API 23+."]
    pub struct GraphicsStats {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buckets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Histogram of frame render times. There should be 154 buckets ranging from [5ms, 6ms) to [4950ms, infinity)"]
        pub buckets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GraphicsStatsBucket>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "highInputLatencyCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total \"high input latency\" events."]
        pub high_input_latency_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jankyFrames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total frames with slow render time. Should be <= total_frames."]
        pub janky_frames: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "missedVsyncCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total \"missed vsync\" events."]
        pub missed_vsync_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "p50Millis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "50th percentile frame render time in milliseconds."]
        pub p50_millis: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "p90Millis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "90th percentile frame render time in milliseconds."]
        pub p90_millis: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "p95Millis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "95th percentile frame render time in milliseconds."]
        pub p95_millis: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "p99Millis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "99th percentile frame render time in milliseconds."]
        pub p99_millis: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "slowBitmapUploadCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total \"slow bitmap upload\" events."]
        pub slow_bitmap_upload_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "slowDrawCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total \"slow draw\" events."]
        pub slow_draw_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "slowUiThreadCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total \"slow UI thread\" events."]
        pub slow_ui_thread_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalFrames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total frames rendered by package."]
        pub total_frames: ::std::option::Option<::std::string::String>,
    }
    impl GraphicsStats {
        pub fn builder() -> GraphicsStatsBuilder {
            GraphicsStatsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GraphicsStatsBucket {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frameCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of frames in the bucket."]
        pub frame_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "renderMillis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Lower bound of render time in milliseconds."]
        pub render_millis: ::std::option::Option<::std::string::String>,
    }
    impl GraphicsStatsBucket {
        pub fn builder() -> GraphicsStatsBucketBuilder {
            GraphicsStatsBucketBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A History represents a sorted list of Executions ordered by the start_timestamp_millis field (descending). It can be used to group all the Executions of a continuous build. Note that the ordering only operates on one-dimension. If a repository has multiple branches, it means that multiple histories will need to be used in order to order Executions per branch."]
    pub struct History {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short human-readable (plain text) name to display in the UI. Maximum of 100 characters. - In response: present if set during create. - In create request: optional"]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "historyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique identifier within a project for this History. Returns INVALID_ARGUMENT if this field is set or overwritten by the caller. - In response always set - In create request: never set"]
        pub history_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A name to uniquely identify a history within a project. Maximum of 200 characters. - In response always set - In create request: always set"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testPlatform")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The platform of the test history. - In response: always set. Returns the platform of the last execution if unknown."]
        pub test_platform: ::std::option::Option<HistoryTestPlatformEnum>,
    }
    impl History {
        pub fn builder() -> HistoryBuilder {
            HistoryBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The platform of the test history. - In response: always set. Returns the platform of the last execution if unknown."]
    pub enum HistoryTestPlatformEnum {
        #[serde(rename = "unknownPlatform")]
        #[doc = ""]
        UnknownPlatform,
        #[serde(rename = "android")]
        #[doc = ""]
        Android,
        #[serde(rename = "ios")]
        #[doc = ""]
        Ios,
    }
    impl ::std::default::Default for HistoryTestPlatformEnum {
        fn default() -> Self {
            Self::UnknownPlatform
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An image, with a link to the main image and a thumbnail."]
    pub struct Image {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An error explaining why the thumbnail could not be rendered."]
        pub error: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceImage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reference to the full-size, original image. This is the same as the tool_outputs entry for the image under its Step. Always set."]
        pub source_image: ::std::option::Option<::std::boxed::Box<ToolOutputReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stepId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The step to which the image is attached. Always set."]
        pub step_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The thumbnail."]
        pub thumbnail: ::std::option::Option<::std::boxed::Box<Thumbnail>>,
    }
    impl Image {
        pub fn builder() -> ImageBuilder {
            ImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional details of in-app purchases encountered during the crawl."]
    pub struct InAppPurchasesFound {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inAppPurchasesFlowsExplored")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of in-app purchases flows explored: how many times the robo tries to buy a SKU."]
        pub in_app_purchases_flows_explored: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inAppPurchasesFlowsStarted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of in-app purchases flows started."]
        pub in_app_purchases_flows_started: ::std::option::Option<::std::primitive::i64>,
    }
    impl InAppPurchasesFound {
        pub fn builder() -> InAppPurchasesFoundBuilder {
            InAppPurchasesFoundBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details for an outcome with an INCONCLUSIVE outcome summary."]
    pub struct InconclusiveDetail {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "abortedByUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the end user aborted the test execution before a pass or fail could be determined. For example, the user pressed ctrl-c which sent a kill signal to the test runner while the test was running."]
        pub aborted_by_user: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hasErrorLogs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If results are being provided to the user in certain cases of infrastructure failures"]
        pub has_error_logs: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "infrastructureFailure")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the test runner could not determine success or failure because the test depends on a component other than the system under test which failed. For example, a mobile test requires provisioning a device where the test executes, and that provisioning can fail."]
        pub infrastructure_failure: ::std::option::Option<::std::primitive::bool>,
    }
    impl InconclusiveDetail {
        pub fn builder() -> InconclusiveDetailBuilder {
            InconclusiveDetailBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Step Id and outcome of each individual step that was run as a group with other steps with the same configuration."]
    pub struct IndividualOutcome {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "multistepNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique int given to each step. Ranges from 0(inclusive) to total number of steps(exclusive). The primary step is 0."]
        pub multistep_number: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outcomeSummary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub outcome_summary: ::std::option::Option<IndividualOutcomeOutcomeSummaryEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "runDuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How long it took for this step to run."]
        pub run_duration: ::std::option::Option<::std::boxed::Box<Duration>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stepId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub step_id: ::std::option::Option<::std::string::String>,
    }
    impl IndividualOutcome {
        pub fn builder() -> IndividualOutcomeBuilder {
            IndividualOutcomeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum IndividualOutcomeOutcomeSummaryEnum {
        #[serde(rename = "unset")]
        #[doc = "Do not use. For proto versioning only."]
        Unset,
        #[serde(rename = "success")]
        #[doc = "The test matrix run was successful, for instance: - All the test cases passed. - Robo did not detect a crash of the application under test."]
        Success,
        #[serde(rename = "failure")]
        #[doc = "A run failed, for instance: - One or more test case failed. - A test timed out. - The application under test crashed."]
        Failure,
        #[serde(rename = "inconclusive")]
        #[doc = "Something unexpected happened. The run should still be considered unsuccessful but this is likely a transient problem and re-running the test might be successful."]
        Inconclusive,
        #[serde(rename = "skipped")]
        #[doc = "All tests were skipped, for instance: - All device configurations were incompatible."]
        Skipped,
        #[serde(rename = "flaky")]
        #[doc = "A group of steps that were run with the same configuration had both failure and success outcomes."]
        Flaky,
    }
    impl ::std::default::Default for IndividualOutcomeOutcomeSummaryEnum {
        fn default() -> Self {
            Self::Unset
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A warning that Robo did not crawl potentially important parts of the app."]
    pub struct InsufficientCoverage {}
    impl InsufficientCoverage {
        pub fn builder() -> InsufficientCoverageBuilder {
            InsufficientCoverageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional details for an iOS app crash."]
    pub struct IosAppCrashed {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stackTrace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The stack trace, if one is available. Optional."]
        pub stack_trace: ::std::option::Option<::std::boxed::Box<StackTrace>>,
    }
    impl IosAppCrashed {
        pub fn builder() -> IosAppCrashedBuilder {
            IosAppCrashedBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "iOS app information"]
    pub struct IosAppInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the app. Required"]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl IosAppInfo {
        pub fn builder() -> IosAppInfoBuilder {
            IosAppInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Robo test for an iOS application."]
    pub struct IosRoboTest {}
    impl IosRoboTest {
        pub fn builder() -> IosRoboTestBuilder {
            IosRoboTestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A iOS mobile test specification"]
    pub struct IosTest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iosAppInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about the application under test."]
        pub ios_app_info: ::std::option::Option<::std::boxed::Box<IosAppInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iosRoboTest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An iOS Robo test."]
        pub ios_robo_test: ::std::option::Option<::std::boxed::Box<IosRoboTest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iosTestLoop")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An iOS test loop."]
        pub ios_test_loop: ::std::option::Option<::std::boxed::Box<IosTestLoop>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iosXcTest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An iOS XCTest."]
        pub ios_xc_test: ::std::option::Option<::std::boxed::Box<IosXcTest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testTimeout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Max time a test is allowed to run before it is automatically cancelled."]
        pub test_timeout: ::std::option::Option<::std::boxed::Box<Duration>>,
    }
    impl IosTest {
        pub fn builder() -> IosTestBuilder {
            IosTestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A game loop test of an iOS application."]
    pub struct IosTestLoop {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bundleId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Bundle ID of the app."]
        pub bundle_id: ::std::option::Option<::std::string::String>,
    }
    impl IosTestLoop {
        pub fn builder() -> IosTestLoopBuilder {
            IosTestLoopBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A test of an iOS application that uses the XCTest framework."]
    pub struct IosXcTest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bundleId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Bundle ID of the app."]
        pub bundle_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "xcodeVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Xcode version that the test was run with."]
        pub xcode_version: ::std::option::Option<::std::string::String>,
    }
    impl IosXcTest {
        pub fn builder() -> IosXcTestBuilder {
            IosXcTestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Failed to find the launcher activity of an app."]
    pub struct LauncherActivityNotFound {}
    impl LauncherActivityNotFound {
        pub fn builder() -> LauncherActivityNotFoundBuilder {
            LauncherActivityNotFoundBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for EnvironmentService.ListEnvironments."]
    pub struct ListEnvironmentsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Environments. Always set."]
        pub environments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Environment>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A Execution id Always set."]
        pub execution_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "historyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A History id. Always set."]
        pub history_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A continuation token to resume the query at the next item. Will only be set if there are more Environments to fetch."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A Project id. Always set."]
        pub project_id: ::std::option::Option<::std::string::String>,
    }
    impl ListEnvironmentsResponse {
        pub fn builder() -> ListEnvironmentsResponseBuilder {
            ListEnvironmentsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ListExecutionsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Executions. Always set."]
        pub executions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Execution>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A continuation token to resume the query at the next item. Will only be set if there are more Executions to fetch."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListExecutionsResponse {
        pub fn builder() -> ListExecutionsResponseBuilder {
            ListExecutionsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for HistoryService.List"]
    pub struct ListHistoriesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "histories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Histories."]
        pub histories: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<History>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A continuation token to resume the query at the next item. Will only be set if there are more histories to fetch. Tokens are valid for up to one hour from the time of the first list request. For instance, if you make a list request at 1PM and use the token from this first request 10 minutes later, the token from this second response will only be valid for 50 minutes."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListHistoriesResponse {
        pub fn builder() -> ListHistoriesResponseBuilder {
            ListHistoriesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ListPerfSampleSeriesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "perfSampleSeries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resulting PerfSampleSeries sorted by id"]
        pub perf_sample_series:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PerfSampleSeries>>>,
    }
    impl ListPerfSampleSeriesResponse {
        pub fn builder() -> ListPerfSampleSeriesResponseBuilder {
            ListPerfSampleSeriesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ListPerfSamplesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional, returned if result size exceeds the page size specified in the request (or the default page size, 500, if unspecified). It indicates the last sample timestamp to be used as page_token in subsequent request"]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "perfSamples")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub perf_samples: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PerfSample>>>,
    }
    impl ListPerfSamplesResponse {
        pub fn builder() -> ListPerfSamplesResponseBuilder {
            ListPerfSamplesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ListScreenshotClustersResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of clusters associated with an execution Always set"]
        pub clusters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ScreenshotCluster>>>,
    }
    impl ListScreenshotClustersResponse {
        pub fn builder() -> ListScreenshotClustersResponseBuilder {
            ListScreenshotClustersResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for AccessibilityService.ListStepAccessibilityClusters."]
    pub struct ListStepAccessibilityClustersResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A sequence of accessibility suggestions, grouped into clusters. Within the sequence, clusters that belong to the same SuggestionCategory should be adjacent. Within each category, clusters should be ordered by their SuggestionPriority (ERRORs first). The categories should be ordered by their highest priority cluster."]
        pub clusters:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SuggestionClusterProto>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A full resource name of the step. For example, projects/my-project/histories/bh.1234567890abcdef/executions/ 1234567890123456789/steps/bs.1234567890abcdef Always presents."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl ListStepAccessibilityClustersResponse {
        pub fn builder() -> ListStepAccessibilityClustersResponseBuilder {
            ListStepAccessibilityClustersResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A response containing the thumbnails in a step."]
    pub struct ListStepThumbnailsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A continuation token to resume the query at the next item. If set, indicates that there are more thumbnails to read, by calling list again with this value in the page_token field."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of image data. Images are returned in a deterministic order; they are ordered by these factors, in order of importance: * First, by their associated test case. Images without a test case are considered greater than images with one. * Second, by their creation time. Images without a creation time are greater than images with one. * Third, by the order in which they were added to the step (by calls to CreateStep or UpdateStep)."]
        pub thumbnails: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Image>>>,
    }
    impl ListStepThumbnailsResponse {
        pub fn builder() -> ListStepThumbnailsResponseBuilder {
            ListStepThumbnailsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for StepService.List."]
    pub struct ListStepsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A continuation token to resume the query at the next item. If set, indicates that there are more steps to read, by calling list again with this value in the page_token field."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "steps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Steps."]
        pub steps: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Step>>>,
    }
    impl ListStepsResponse {
        pub fn builder() -> ListStepsResponseBuilder {
            ListStepsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for StepService.ListTestCases."]
    pub struct ListTestCasesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testCases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of test cases."]
        pub test_cases: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TestCase>>>,
    }
    impl ListTestCasesResponse {
        pub fn builder() -> ListTestCasesResponseBuilder {
            ListTestCasesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A warning that there were issues in logcat collection."]
    pub struct LogcatCollectionError {}
    impl LogcatCollectionError {
        pub fn builder() -> LogcatCollectionErrorBuilder {
            LogcatCollectionErrorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "One dimension of the matrix of different runs of a step."]
    pub struct MatrixDimensionDefinition {}
    impl MatrixDimensionDefinition {
        pub fn builder() -> MatrixDimensionDefinitionBuilder {
            MatrixDimensionDefinitionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct MemoryInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "memoryCapInKibibyte")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum memory that can be allocated to the process in KiB"]
        pub memory_cap_in_kibibyte: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "memoryTotalInKibibyte")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total memory available on the device in KiB"]
        pub memory_total_in_kibibyte: ::std::option::Option<::std::string::String>,
    }
    impl MemoryInfo {
        pub fn builder() -> MemoryInfoBuilder {
            MemoryInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Merged test result for environment. If the environment has only one step (no reruns or shards), then the merged result is the same as the step result. If the environment has multiple shards and/or reruns, then the results of shards and reruns that belong to the same environment are merged into one environment result."]
    pub struct MergedResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outcome")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Outcome of the resource"]
        pub outcome: ::std::option::Option<::std::boxed::Box<Outcome>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "State of the resource"]
        pub state: ::std::option::Option<MergedResultStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testSuiteOverviews")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The combined and rolled-up result of each test suite that was run as part of this environment. Combining: When the test cases from a suite are run in different steps (sharding), the results are added back together in one overview. (e.g., if shard1 has 2 failures and shard2 has 1 failure than the overview failure_count = 3). Rollup: When test cases from the same suite are run multiple times (flaky), the results are combined (e.g., if testcase1.run1 fails, testcase1.run2 passes, and both testcase2.run1 and testcase2.run2 fail then the overview flaky_count = 1 and failure_count = 1)."]
        pub test_suite_overviews:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TestSuiteOverview>>>,
    }
    impl MergedResult {
        pub fn builder() -> MergedResultBuilder {
            MergedResultBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "State of the resource"]
    pub enum MergedResultStateEnum {
        #[serde(rename = "unknownState")]
        #[doc = "Should never be in this state. Exists for proto deserialization backward compatibility."]
        UnknownState,
        #[serde(rename = "pending")]
        #[doc = "The Execution/Step is created, ready to run, but not running yet. If an Execution/Step is created without initial state, it is assumed that the Execution/Step is in PENDING state."]
        Pending,
        #[serde(rename = "inProgress")]
        #[doc = "The Execution/Step is in progress."]
        InProgress,
        #[serde(rename = "complete")]
        #[doc = "The finalized, immutable state. Steps/Executions in this state cannot be modified."]
        Complete,
    }
    impl ::std::default::Default for MergedResultStateEnum {
        fn default() -> Self {
            Self::UnknownState
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details when multiple steps are run with the same configuration as a group."]
    pub struct MultiStep {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "multistepNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique int given to each step. Ranges from 0(inclusive) to total number of steps(exclusive). The primary step is 0."]
        pub multistep_number: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primaryStep")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Present if it is a primary (original) step."]
        pub primary_step: ::std::option::Option<::std::boxed::Box<PrimaryStep>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primaryStepId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Step Id of the primary (original) step, which might be this step."]
        pub primary_step_id: ::std::option::Option<::std::string::String>,
    }
    impl MultiStep {
        pub fn builder() -> MultiStepBuilder {
            MultiStepBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional details for a native crash."]
    pub struct NativeCrash {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stackTrace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The stack trace of the native crash. Optional."]
        pub stack_trace: ::std::option::Option<::std::boxed::Box<StackTrace>>,
    }
    impl NativeCrash {
        pub fn builder() -> NativeCrashBuilder {
            NativeCrashBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A non-sdk API and examples of it being called along with other metadata See https://developer.android.com/distribute/best-practices/develop/restrictions-non-sdk-interfaces"]
    pub struct NonSdkApi {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiSignature")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The signature of the Non-SDK API"]
        pub api_signature: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exampleStackTraces")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Example stack traces of this API being called."]
        pub example_stack_traces: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insights")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional debugging insights for non-SDK API violations."]
        pub insights: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NonSdkApiInsight>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "invocationCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of times this API was observed to have been called."]
        pub invocation_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "list")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Which list this API appears on"]
        pub list: ::std::option::Option<NonSdkApiListEnum>,
    }
    impl NonSdkApi {
        pub fn builder() -> NonSdkApiBuilder {
            NonSdkApiBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Which list this API appears on"]
    pub enum NonSdkApiListEnum {
        #[serde(rename = "NONE")]
        #[doc = ""]
        None,
        #[serde(rename = "WHITE")]
        #[doc = ""]
        White,
        #[serde(rename = "BLACK")]
        #[doc = ""]
        Black,
        #[serde(rename = "GREY")]
        #[doc = ""]
        Grey,
        #[serde(rename = "GREY_MAX_O")]
        #[doc = ""]
        GreyMaxO,
        #[serde(rename = "GREY_MAX_P")]
        #[doc = ""]
        GreyMaxP,
        #[serde(rename = "GREY_MAX_Q")]
        #[doc = ""]
        GreyMaxQ,
        #[serde(rename = "GREY_MAX_R")]
        #[doc = ""]
        GreyMaxR,
    }
    impl ::std::default::Default for NonSdkApiListEnum {
        fn default() -> Self {
            Self::None
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Non-SDK API insights (to address debugging solutions)."]
    pub struct NonSdkApiInsight {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exampleTraceMessages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional sample stack traces, for which this insight applies (there should be at least one)."]
        pub example_trace_messages: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matcherId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique ID, to be used for determining the effectiveness of this particular insight in the context of a matcher. (required)"]
        pub matcher_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pendingGoogleUpdateInsight")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An insight indicating that the hidden API usage originates from a Google-provided library."]
        pub pending_google_update_insight:
            ::std::option::Option<::std::boxed::Box<PendingGoogleUpdateInsight>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "upgradeInsight")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An insight indicating that the hidden API usage originates from the use of a library that needs to be upgraded."]
        pub upgrade_insight: ::std::option::Option<::std::boxed::Box<UpgradeInsight>>,
    }
    impl NonSdkApiInsight {
        pub fn builder() -> NonSdkApiInsightBuilder {
            NonSdkApiInsightBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional details for a non-sdk API usage violation."]
    pub struct NonSdkApiUsageViolation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiSignatures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Signatures of a subset of those hidden API's."]
        pub api_signatures: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uniqueApis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of unique hidden API's accessed."]
        pub unique_apis: ::std::option::Option<::std::primitive::i64>,
    }
    impl NonSdkApiUsageViolation {
        pub fn builder() -> NonSdkApiUsageViolationBuilder {
            NonSdkApiUsageViolationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains a summary and examples of non-sdk API usage violations."]
    pub struct NonSdkApiUsageViolationReport {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exampleApis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Examples of the detected API usages."]
        pub example_apis: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NonSdkApi>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minSdkVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum API level required for the application to run."]
        pub min_sdk_version: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetSdkVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the API Level on which the application is designed to run."]
        pub target_sdk_version: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uniqueApis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of unique Non-SDK API's accessed."]
        pub unique_apis: ::std::option::Option<::std::primitive::i64>,
    }
    impl NonSdkApiUsageViolationReport {
        pub fn builder() -> NonSdkApiUsageViolationReportBuilder {
            NonSdkApiUsageViolationReportBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Interprets a result so that humans and machines can act on it."]
    pub struct Outcome {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "failureDetail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "More information about a FAILURE outcome. Returns INVALID_ARGUMENT if this field is set but the summary is not FAILURE. Optional"]
        pub failure_detail: ::std::option::Option<::std::boxed::Box<FailureDetail>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inconclusiveDetail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "More information about an INCONCLUSIVE outcome. Returns INVALID_ARGUMENT if this field is set but the summary is not INCONCLUSIVE. Optional"]
        pub inconclusive_detail: ::std::option::Option<::std::boxed::Box<InconclusiveDetail>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "skippedDetail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "More information about a SKIPPED outcome. Returns INVALID_ARGUMENT if this field is set but the summary is not SKIPPED. Optional"]
        pub skipped_detail: ::std::option::Option<::std::boxed::Box<SkippedDetail>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "successDetail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "More information about a SUCCESS outcome. Returns INVALID_ARGUMENT if this field is set but the summary is not SUCCESS. Optional"]
        pub success_detail: ::std::option::Option<::std::boxed::Box<SuccessDetail>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "summary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The simplest way to interpret a result. Required"]
        pub summary: ::std::option::Option<OutcomeSummaryEnum>,
    }
    impl Outcome {
        pub fn builder() -> OutcomeBuilder {
            OutcomeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The simplest way to interpret a result. Required"]
    pub enum OutcomeSummaryEnum {
        #[serde(rename = "unset")]
        #[doc = "Do not use. For proto versioning only."]
        Unset,
        #[serde(rename = "success")]
        #[doc = "The test matrix run was successful, for instance: - All the test cases passed. - Robo did not detect a crash of the application under test."]
        Success,
        #[serde(rename = "failure")]
        #[doc = "A run failed, for instance: - One or more test case failed. - A test timed out. - The application under test crashed."]
        Failure,
        #[serde(rename = "inconclusive")]
        #[doc = "Something unexpected happened. The run should still be considered unsuccessful but this is likely a transient problem and re-running the test might be successful."]
        Inconclusive,
        #[serde(rename = "skipped")]
        #[doc = "All tests were skipped, for instance: - All device configurations were incompatible."]
        Skipped,
        #[serde(rename = "flaky")]
        #[doc = "A group of steps that were run with the same configuration had both failure and success outcomes."]
        Flaky,
    }
    impl ::std::default::Default for OutcomeSummaryEnum {
        fn default() -> Self {
            Self::Unset
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A warning that Robo encountered a screen that has overlapping clickable elements; this may indicate a potential UI issue."]
    pub struct OverlappingUiElements {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource names of the overlapping screen elements"]
        pub resource_name: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "screenId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The screen id of the elements"]
        pub screen_id: ::std::option::Option<::std::string::String>,
    }
    impl OverlappingUiElements {
        pub fn builder() -> OverlappingUiElementsBuilder {
            OverlappingUiElementsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This insight indicates that the hidden API usage originates from a Google-provided library. Users need not take any action."]
    pub struct PendingGoogleUpdateInsight {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nameOfGoogleLibrary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the Google-provided library with the non-SDK API dependency."]
        pub name_of_google_library: ::std::option::Option<::std::string::String>,
    }
    impl PendingGoogleUpdateInsight {
        pub fn builder() -> PendingGoogleUpdateInsightBuilder {
            PendingGoogleUpdateInsightBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Encapsulates performance environment info"]
    pub struct PerfEnvironment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpuInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "CPU related environment info"]
        pub cpu_info: ::std::option::Option<::std::boxed::Box<CpuInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "memoryInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Memory related environment info"]
        pub memory_info: ::std::option::Option<::std::boxed::Box<MemoryInfo>>,
    }
    impl PerfEnvironment {
        pub fn builder() -> PerfEnvironmentBuilder {
            PerfEnvironmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A summary of perf metrics collected and performance environment info"]
    pub struct PerfMetricsSummary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appStartTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub app_start_time: ::std::option::Option<::std::boxed::Box<AppStartTime>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A tool results execution ID. @OutputOnly"]
        pub execution_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "graphicsStats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Graphics statistics for the entire run. Statistics are reset at the beginning of the run and collected at the end of the run."]
        pub graphics_stats: ::std::option::Option<::std::boxed::Box<GraphicsStats>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "historyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A tool results history ID. @OutputOnly"]
        pub history_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "perfEnvironment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes the environment in which the performance metrics were collected"]
        pub perf_environment: ::std::option::Option<::std::boxed::Box<PerfEnvironment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "perfMetrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set of resource collected"]
        pub perf_metrics: ::std::option::Option<::std::vec::Vec<PerfMetricsSummaryPerfMetricsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cloud project @OutputOnly"]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stepId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A tool results step ID. @OutputOnly"]
        pub step_id: ::std::option::Option<::std::string::String>,
    }
    impl PerfMetricsSummary {
        pub fn builder() -> PerfMetricsSummaryBuilder {
            PerfMetricsSummaryBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum PerfMetricsSummaryPerfMetricsEnum {
        #[serde(rename = "perfMetricTypeUnspecified")]
        #[doc = ""]
        PerfMetricTypeUnspecified,
        #[serde(rename = "memory")]
        #[doc = ""]
        Memory,
        #[serde(rename = "cpu")]
        #[doc = ""]
        Cpu,
        #[serde(rename = "network")]
        #[doc = ""]
        Network,
        #[serde(rename = "graphics")]
        #[doc = ""]
        Graphics,
    }
    impl ::std::default::Default for PerfMetricsSummaryPerfMetricsEnum {
        fn default() -> Self {
            Self::PerfMetricTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Resource representing a single performance measure or data point"]
    pub struct PerfSample {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sampleTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp of collection."]
        pub sample_time: ::std::option::Option<::std::boxed::Box<Timestamp>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value observed"]
        pub value: ::std::option::Option<::std::primitive::f64>,
    }
    impl PerfSample {
        pub fn builder() -> PerfSampleBuilder {
            PerfSampleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Resource representing a collection of performance samples (or data points)"]
    pub struct PerfSampleSeries {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicPerfSampleSeries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Basic series represented by a line chart"]
        pub basic_perf_sample_series:
            ::std::option::Option<::std::boxed::Box<BasicPerfSampleSeries>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A tool results execution ID. @OutputOnly"]
        pub execution_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "historyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A tool results history ID. @OutputOnly"]
        pub history_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cloud project @OutputOnly"]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sampleSeriesId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A sample series id @OutputOnly"]
        pub sample_series_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stepId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A tool results step ID. @OutputOnly"]
        pub step_id: ::std::option::Option<::std::string::String>,
    }
    impl PerfSampleSeries {
        pub fn builder() -> PerfSampleSeriesBuilder {
            PerfSampleSeriesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A notification that Robo signed in with Google."]
    pub struct PerformedGoogleLogin {}
    impl PerformedGoogleLogin {
        pub fn builder() -> PerformedGoogleLoginBuilder {
            PerformedGoogleLoginBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A notification that Robo performed some monkey actions."]
    pub struct PerformedMonkeyActions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalActions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of monkey actions performed during the crawl."]
        pub total_actions: ::std::option::Option<::std::primitive::i64>,
    }
    impl PerformedMonkeyActions {
        pub fn builder() -> PerformedMonkeyActionsBuilder {
            PerformedMonkeyActionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Stores rollup test status of multiple steps that were run as a group and outcome of each individual step."]
    pub struct PrimaryStep {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "individualOutcome")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Step Id and outcome of each individual step."]
        pub individual_outcome:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<IndividualOutcome>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rollUp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rollup test status of multiple steps that were run with the same configuration as a group."]
        pub roll_up: ::std::option::Option<PrimaryStepRollUpEnum>,
    }
    impl PrimaryStep {
        pub fn builder() -> PrimaryStepBuilder {
            PrimaryStepBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Rollup test status of multiple steps that were run with the same configuration as a group."]
    pub enum PrimaryStepRollUpEnum {
        #[serde(rename = "unset")]
        #[doc = "Do not use. For proto versioning only."]
        Unset,
        #[serde(rename = "success")]
        #[doc = "The test matrix run was successful, for instance: - All the test cases passed. - Robo did not detect a crash of the application under test."]
        Success,
        #[serde(rename = "failure")]
        #[doc = "A run failed, for instance: - One or more test case failed. - A test timed out. - The application under test crashed."]
        Failure,
        #[serde(rename = "inconclusive")]
        #[doc = "Something unexpected happened. The run should still be considered unsuccessful but this is likely a transient problem and re-running the test might be successful."]
        Inconclusive,
        #[serde(rename = "skipped")]
        #[doc = "All tests were skipped, for instance: - All device configurations were incompatible."]
        Skipped,
        #[serde(rename = "flaky")]
        #[doc = "A group of steps that were run with the same configuration had both failure and success outcomes."]
        Flaky,
    }
    impl ::std::default::Default for PrimaryStepRollUpEnum {
        fn default() -> Self {
            Self::Unset
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Per-project settings for the Tool Results service."]
    pub struct ProjectSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultBucket")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the Google Cloud Storage bucket to which results are written. By default, this is unset. In update request: optional In response: optional"]
        pub default_bucket: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the project's settings. Always of the form: projects/{project-id}/settings In update request: never set In response: always set"]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl ProjectSettings {
        pub fn builder() -> ProjectSettingsBuilder {
            ProjectSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for StepService.PublishXunitXmlFiles."]
    pub struct PublishXunitXmlFilesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "xunitXmlFiles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URI of the Xunit XML files to publish. The maximum size of the file this reference is pointing to is 50MB. Required."]
        pub xunit_xml_files:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FileReference>>>,
    }
    impl PublishXunitXmlFilesRequest {
        pub fn builder() -> PublishXunitXmlFilesRequestBuilder {
            PublishXunitXmlFilesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A rectangular region."]
    pub struct RegionProto {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "heightPx")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The height, in pixels. Always set."]
        pub height_px: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "leftPx")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The left side of the rectangle, in pixels. Always set."]
        pub left_px: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topPx")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The top of the rectangle, in pixels. Always set."]
        pub top_px: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "widthPx")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The width, in pixels. Always set."]
        pub width_px: ::std::option::Option<::std::primitive::i64>,
    }
    impl RegionProto {
        pub fn builder() -> RegionProtoBuilder {
            RegionProtoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The storage for test results."]
    pub struct ResultsStorage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resultsStoragePath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The root directory for test results."]
        pub results_storage_path: ::std::option::Option<::std::boxed::Box<FileReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "xunitXmlFile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The path to the Xunit XML file."]
        pub xunit_xml_file: ::std::option::Option<::std::boxed::Box<FileReference>>,
    }
    impl ResultsStorage {
        pub fn builder() -> ResultsStorageBuilder {
            ResultsStorageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Execution stats for a user-provided Robo script."]
    pub struct RoboScriptExecution {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "successfulActions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of Robo script actions executed successfully."]
        pub successful_actions: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalActions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of actions in the Robo script."]
        pub total_actions: ::std::option::Option<::std::primitive::i64>,
    }
    impl RoboScriptExecution {
        pub fn builder() -> RoboScriptExecutionBuilder {
            RoboScriptExecutionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "IMPORTANT: It is unsafe to accept this message from an untrusted source, since it's trivial for an attacker to forge serialized messages that don't fulfill the type's safety contract -- for example, it could contain attacker controlled script. A system which receives a SafeHtmlProto implicitly trusts the producer of the SafeHtmlProto. So, it's generally safe to return this message in RPC responses, but generally unsafe to accept it in RPC requests."]
    pub struct SafeHtmlProto {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "privateDoNotAccessOrElseSafeHtmlWrappedValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "IMPORTANT: Never set or read this field, even from tests, it is private. See documentation at the top of .proto file for programming language packages with which to create or read this message."]
        pub private_do_not_access_or_else_safe_html_wrapped_value:
            ::std::option::Option<::std::string::String>,
    }
    impl SafeHtmlProto {
        pub fn builder() -> SafeHtmlProtoBuilder {
            SafeHtmlProtoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Screen {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "File reference of the png file. Required."]
        pub file_reference: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locale")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Locale of the device that the screenshot was taken on. Required."]
        pub locale: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "model")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Model of the device that the screenshot was taken on. Required."]
        pub model: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OS version of the device that the screenshot was taken on. Required."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl Screen {
        pub fn builder() -> ScreenBuilder {
            ScreenBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ScreenshotCluster {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "activity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A string that describes the activity of every screen in the cluster."]
        pub activity: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique identifier for the cluster. @OutputOnly"]
        pub cluster_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyScreen")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A singular screen that represents the cluster as a whole. This screen will act as the \"cover\" of the entire cluster. When users look at the clusters, only the key screen from each cluster will be shown. Which screen is the key screen is determined by the ClusteringAlgorithm"]
        pub key_screen: ::std::option::Option<::std::boxed::Box<Screen>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "screens")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Full list of screens."]
        pub screens: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Screen>>>,
    }
    impl ScreenshotCluster {
        pub fn builder() -> ScreenshotClusterBuilder {
            ScreenshotClusterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Result summary for a shard in an environment."]
    pub struct ShardSummary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "runs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Summaries of the steps belonging to the shard. With flaky_test_attempts enabled from TestExecutionService, more than one run (Step) can present. And the runs will be sorted by multistep_number."]
        pub runs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StepSummary>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shardResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Merged result of the shard."]
        pub shard_result: ::std::option::Option<::std::boxed::Box<MergedResult>>,
    }
    impl ShardSummary {
        pub fn builder() -> ShardSummaryBuilder {
            ShardSummaryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details for an outcome with a SKIPPED outcome summary."]
    pub struct SkippedDetail {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "incompatibleAppVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the App doesn't support the specific API level."]
        pub incompatible_app_version: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "incompatibleArchitecture")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the App doesn't run on the specific architecture, for example, x86."]
        pub incompatible_architecture: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "incompatibleDevice")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the requested OS version doesn't run on the specific device model."]
        pub incompatible_device: ::std::option::Option<::std::primitive::bool>,
    }
    impl SkippedDetail {
        pub fn builder() -> SkippedDetailBuilder {
            SkippedDetailBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The details about how to run the execution."]
    pub struct Specification {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "androidTest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An Android mobile test execution specification."]
        pub android_test: ::std::option::Option<::std::boxed::Box<AndroidTest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iosTest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An iOS mobile test execution specification."]
        pub ios_test: ::std::option::Option<::std::boxed::Box<IosTest>>,
    }
    impl Specification {
        pub fn builder() -> SpecificationBuilder {
            SpecificationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A stacktrace."]
    pub struct StackTrace {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exception")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The stack trace message. Required"]
        pub exception: ::std::option::Option<::std::string::String>,
    }
    impl StackTrace {
        pub fn builder() -> StackTraceBuilder {
            StackTraceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "User provided intent failed to resolve to an activity."]
    pub struct StartActivityNotFound {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "action")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub action: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl StartActivityNotFound {
        pub fn builder() -> StartActivityNotFoundBuilder {
            StartActivityNotFoundBuilder::default()
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
    #[doc = "A Step represents a single operation performed as part of Execution. A step can be used to represent the execution of a tool ( for example a test runner execution or an execution of a compiler). Steps can overlap (for instance two steps might have the same start time if some operations are done in parallel). Here is an example, let's consider that we have a continuous build is executing a test runner for each iteration. The workflow would look like: - user creates a Execution with id 1 - user creates an TestExecutionStep with id 100 for Execution 1 - user update TestExecutionStep with id 100 to add a raw xml log + the service parses the xml logs and returns a TestExecutionStep with updated TestResult(s). - user update the status of TestExecutionStep with id 100 to COMPLETE A Step can be updated until its state is set to COMPLETE at which points it becomes immutable. Next tag: 27"]
    pub struct Step {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "completionTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when the step status was set to complete. This value will be set automatically when state transitions to COMPLETE. - In response: set if the execution state is COMPLETE. - In create/update request: never set"]
        pub completion_time: ::std::option::Option<::std::boxed::Box<Timestamp>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when the step was created. - In response: always set - In create/update request: never set"]
        pub creation_time: ::std::option::Option<::std::boxed::Box<Timestamp>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A description of this tool For example: mvn clean package -D skipTests=true - In response: present if set by create/update request - In create/update request: optional"]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceUsageDuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How much the device resource is used to perform the test. This is the device usage used for billing purpose, which is different from the run_duration, for example, infrastructure failure won't be charged for device usage. PRECONDITION_FAILED will be returned if one attempts to set a device_usage on a step which already has this field set. - In response: present if previously set. - In create request: optional - In update request: optional"]
        pub device_usage_duration: ::std::option::Option<::std::boxed::Box<Duration>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensionValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the execution containing this step has any dimension_definition set, then this field allows the child to specify the values of the dimensions. The keys must exactly match the dimension_definition of the execution. For example, if the execution has `dimension_definition = ['attempt', 'device']` then a step must define values for those dimensions, eg. `dimension_value = ['attempt': '1', 'device': 'Nexus 6']` If a step does not participate in one dimension of the matrix, the value for that dimension should be empty string. For example, if one of the tests is executed by a runner which does not support retries, the step could have `dimension_value = ['attempt': '', 'device': 'Nexus 6']` If the step does not participate in any dimensions of the matrix, it may leave dimension_value unset. A PRECONDITION_FAILED will be returned if any of the keys do not exist in the dimension_definition of the execution. A PRECONDITION_FAILED will be returned if another step in this execution already has the same name and dimension_value, but differs on other data fields, for example, step field is different. A PRECONDITION_FAILED will be returned if dimension_value is set, and there is a dimension_definition in the execution which is not specified as one of the keys. - In response: present if set by create - In create request: optional - In update request: never set"]
        pub dimension_value:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StepDimensionValueEntry>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hasImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether any of the outputs of this step are images whose thumbnails can be fetched with ListThumbnails. - In response: always set - In create/update request: never set"]
        pub has_images: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Arbitrary user-supplied key/value pairs that are associated with the step. Users are responsible for managing the key namespace such that keys don't accidentally collide. An INVALID_ARGUMENT will be returned if the number of labels exceeds 100 or if the length of any of the keys or values exceeds 100 characters. - In response: always set - In create request: optional - In update request: optional; any new key/value pair will be added to the map, and any new value for an existing key will update that key's value"]
        pub labels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StepLabelsEntry>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "multiStep")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details when multiple steps are run with the same configuration as a group. These details can be used identify which group this step is part of. It also identifies the groups 'primary step' which indexes all the group members. - In response: present if previously set. - In create request: optional, set iff this step was performed more than once. - In update request: optional"]
        pub multi_step: ::std::option::Option<::std::boxed::Box<MultiStep>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short human-readable name to display in the UI. Maximum of 100 characters. For example: Clean build A PRECONDITION_FAILED will be returned upon creating a new step if it shares its name and dimension_value with an existing step. If two steps represent a similar action, but have different dimension values, they should share the same name. For instance, if the same set of tests is run on two different platforms, the two steps should have the same name. - In response: always set - In create request: always set - In update request: never set"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outcome")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Classification of the result, for example into SUCCESS or FAILURE - In response: present if set by create/update request - In create/update request: optional"]
        pub outcome: ::std::option::Option<::std::boxed::Box<Outcome>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "runDuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How long it took for this step to run. If unset, this is set to the difference between creation_time and completion_time when the step is set to the COMPLETE state. In some cases, it is appropriate to set this value separately: For instance, if a step is created, but the operation it represents is queued for a few minutes before it executes, it would be appropriate not to include the time spent queued in its run_duration. PRECONDITION_FAILED will be returned if one attempts to set a run_duration on a step which already has this field set. - In response: present if previously set; always present on COMPLETE step - In create request: optional - In update request: optional"]
        pub run_duration: ::std::option::Option<::std::boxed::Box<Duration>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The initial state is IN_PROGRESS. The only legal state transitions are * IN_PROGRESS -> COMPLETE A PRECONDITION_FAILED will be returned if an invalid transition is requested. It is valid to create Step with a state set to COMPLETE. The state can only be set to COMPLETE once. A PRECONDITION_FAILED will be returned if the state is set to COMPLETE multiple times. - In response: always set - In create/update request: optional"]
        pub state: ::std::option::Option<StepStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stepId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique identifier within a Execution for this Step. Returns INVALID_ARGUMENT if this field is set or overwritten by the caller. - In response: always set - In create/update request: never set"]
        pub step_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testExecutionStep")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An execution of a test runner."]
        pub test_execution_step: ::std::option::Option<::std::boxed::Box<TestExecutionStep>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "toolExecutionStep")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An execution of a tool (used for steps we don't explicitly support)."]
        pub tool_execution_step: ::std::option::Option<::std::boxed::Box<ToolExecutionStep>>,
    }
    impl Step {
        pub fn builder() -> StepBuilder {
            StepBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The initial state is IN_PROGRESS. The only legal state transitions are * IN_PROGRESS -> COMPLETE A PRECONDITION_FAILED will be returned if an invalid transition is requested. It is valid to create Step with a state set to COMPLETE. The state can only be set to COMPLETE once. A PRECONDITION_FAILED will be returned if the state is set to COMPLETE multiple times. - In response: always set - In create/update request: optional"]
    pub enum StepStateEnum {
        #[serde(rename = "unknownState")]
        #[doc = "Should never be in this state. Exists for proto deserialization backward compatibility."]
        UnknownState,
        #[serde(rename = "pending")]
        #[doc = "The Execution/Step is created, ready to run, but not running yet. If an Execution/Step is created without initial state, it is assumed that the Execution/Step is in PENDING state."]
        Pending,
        #[serde(rename = "inProgress")]
        #[doc = "The Execution/Step is in progress."]
        InProgress,
        #[serde(rename = "complete")]
        #[doc = "The finalized, immutable state. Steps/Executions in this state cannot be modified."]
        Complete,
    }
    impl ::std::default::Default for StepStateEnum {
        fn default() -> Self {
            Self::UnknownState
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct StepDimensionValueEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl StepDimensionValueEntry {
        pub fn builder() -> StepDimensionValueEntryBuilder {
            StepDimensionValueEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct StepLabelsEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl StepLabelsEntry {
        pub fn builder() -> StepLabelsEntryBuilder {
            StepLabelsEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Lightweight summary of a step within this execution."]
    pub struct StepSummary {}
    impl StepSummary {
        pub fn builder() -> StepSummaryBuilder {
            StepSummaryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details for an outcome with a SUCCESS outcome summary. LINT.IfChange"]
    pub struct SuccessDetail {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "otherNativeCrash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If a native process other than the app crashed."]
        pub other_native_crash: ::std::option::Option<::std::primitive::bool>,
    }
    impl SuccessDetail {
        pub fn builder() -> SuccessDetailBuilder {
            SuccessDetailBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A set of similar suggestions that we suspect are closely related. This proto and most of the nested protos are branched from foxandcrown.prelaunchreport.service.SuggestionClusterProto, replacing PLR's dependencies with FTL's."]
    pub struct SuggestionClusterProto {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "category")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Category in which these types of suggestions should appear. Always set."]
        pub category: ::std::option::Option<SuggestionClusterProtoCategoryEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "suggestions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A sequence of suggestions. All of the suggestions within a cluster must have the same SuggestionPriority and belong to the same SuggestionCategory. Suggestions with the same screenshot URL should be adjacent."]
        pub suggestions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SuggestionProto>>>,
    }
    impl SuggestionClusterProto {
        pub fn builder() -> SuggestionClusterProtoBuilder {
            SuggestionClusterProtoBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Category in which these types of suggestions should appear. Always set."]
    pub enum SuggestionClusterProtoCategoryEnum {
        #[serde(rename = "unknownCategory")]
        #[doc = ""]
        UnknownCategory,
        #[serde(rename = "contentLabeling")]
        #[doc = ""]
        ContentLabeling,
        #[serde(rename = "touchTargetSize")]
        #[doc = ""]
        TouchTargetSize,
        #[serde(rename = "lowContrast")]
        #[doc = ""]
        LowContrast,
        #[serde(rename = "implementation")]
        #[doc = ""]
        Implementation,
    }
    impl ::std::default::Default for SuggestionClusterProtoCategoryEnum {
        fn default() -> Self {
            Self::UnknownCategory
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct SuggestionProto {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "helpUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reference to a help center article concerning this type of suggestion. Always set."]
        pub help_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "longMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Message, in the user's language, explaining the suggestion, which may contain markup. Always set."]
        pub long_message: ::std::option::Option<::std::boxed::Box<SafeHtmlProto>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "priority")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Relative importance of a suggestion. Always set."]
        pub priority: ::std::option::Option<SuggestionProtoPriorityEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pseudoResourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A somewhat human readable identifier of the source view, if it does not have a resource_name. This is a path within the accessibility hierarchy, an element with resource name; similar to an XPath."]
        pub pseudo_resource_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "region")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Region within the screenshot that is relevant to this suggestion. Optional."]
        pub region: ::std::option::Option<::std::boxed::Box<RegionProto>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reference to a view element, identified by its resource name, if it has one."]
        pub resource_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "screenId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the screen for the suggestion. It is used for getting the corresponding screenshot path. For example, screen_id \"1\" corresponds to \"1.png\" file in GCS. Always set."]
        pub screen_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secondaryPriority")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Relative importance of a suggestion as compared with other suggestions that have the same priority and category. This is a meaningless value that can be used to order suggestions that are in the same category and have the same priority. The larger values have higher priority (i.e., are more important). Optional."]
        pub secondary_priority: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shortMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Concise message, in the user's language, representing the suggestion, which may contain markup. Always set."]
        pub short_message: ::std::option::Option<::std::boxed::Box<SafeHtmlProto>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "General title for the suggestion, in the user's language, without markup. Always set."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl SuggestionProto {
        pub fn builder() -> SuggestionProtoBuilder {
            SuggestionProtoBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Relative importance of a suggestion. Always set."]
    pub enum SuggestionProtoPriorityEnum {
        #[serde(rename = "unknownPriority")]
        #[doc = ""]
        UnknownPriority,
        #[serde(rename = "error")]
        #[doc = ""]
        Error,
        #[serde(rename = "warning")]
        #[doc = ""]
        Warning,
        #[serde(rename = "info")]
        #[doc = ""]
        Info,
    }
    impl ::std::default::Default for SuggestionProtoPriorityEnum {
        fn default() -> Self {
            Self::UnknownPriority
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TestCase {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "elapsedTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The elapsed run time of the test case. Required."]
        pub elapsed_time: ::std::option::Option<::std::boxed::Box<Duration>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end time of the test case."]
        pub end_time: ::std::option::Option<::std::boxed::Box<Timestamp>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "skippedMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Why the test case was skipped. Present only for skipped test case"]
        pub skipped_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stackTraces")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The stack trace details if the test case failed or encountered an error. The maximum size of the stack traces is 100KiB, beyond which the stack track will be truncated. Zero if the test case passed."]
        pub stack_traces: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StackTrace>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start time of the test case."]
        pub start_time: ::std::option::Option<::std::boxed::Box<Timestamp>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the test case. Required."]
        pub status: ::std::option::Option<TestCaseStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testCaseId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique identifier within a Step for this Test Case."]
        pub test_case_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testCaseReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Test case reference, e.g. name, class name and test suite name. Required."]
        pub test_case_reference: ::std::option::Option<::std::boxed::Box<TestCaseReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "toolOutputs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "References to opaque files of any format output by the tool execution. @OutputOnly"]
        pub tool_outputs:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ToolOutputReference>>>,
    }
    impl TestCase {
        pub fn builder() -> TestCaseBuilder {
            TestCaseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The status of the test case. Required."]
    pub enum TestCaseStatusEnum {
        #[serde(rename = "passed")]
        #[doc = "Test passed."]
        Passed,
        #[serde(rename = "failed")]
        #[doc = "Test failed."]
        Failed,
        #[serde(rename = "error")]
        #[doc = "Test encountered an error"]
        Error,
        #[serde(rename = "skipped")]
        #[doc = "Test skipped"]
        Skipped,
        #[serde(rename = "flaky")]
        #[doc = "Test flaked. Present only for rollup test cases; test cases from steps that were run with the same configuration had both failure and success outcomes."]
        Flaky,
    }
    impl ::std::default::Default for TestCaseStatusEnum {
        fn default() -> Self {
            Self::Passed
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A reference to a test case. Test case references are canonically ordered lexicographically by these three factors: * First, by test_suite_name. * Second, by class_name. * Third, by name."]
    pub struct TestCaseReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "className")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the class."]
        pub class_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the test case. Required."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testSuiteName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the test suite to which this test case belongs."]
        pub test_suite_name: ::std::option::Option<::std::string::String>,
    }
    impl TestCaseReference {
        pub fn builder() -> TestCaseReferenceBuilder {
            TestCaseReferenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A step that represents running tests. It accepts ant-junit xml files which will be parsed into structured test results by the service. Xml file paths are updated in order to append more files, however they can't be deleted. Users can also add test results manually by using the test_result field."]
    pub struct TestExecutionStep {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testIssues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Issues observed during the test execution. For example, if the mobile app under test crashed during the test, the error message and the stack trace content can be recorded here to assist debugging. - In response: present if set by create or update - In create/update request: optional"]
        pub test_issues: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TestIssue>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testSuiteOverviews")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of test suite overview contents. This could be parsed from xUnit XML log by server, or uploaded directly by user. This references should only be called when test suites are fully parsed or uploaded. The maximum allowed number of test suite overviews per step is 1000. - In response: always set - In create request: optional - In update request: never (use publishXunitXmlFiles custom method instead)"]
        pub test_suite_overviews:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TestSuiteOverview>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testTiming")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timing break down of the test execution. - In response: present if set by create or update - In create/update request: optional"]
        pub test_timing: ::std::option::Option<::std::boxed::Box<TestTiming>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "toolExecution")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents the execution of the test runner. The exit code of this tool will be used to determine if the test passed. - In response: always set - In create/update request: optional"]
        pub tool_execution: ::std::option::Option<::std::boxed::Box<ToolExecution>>,
    }
    impl TestExecutionStep {
        pub fn builder() -> TestExecutionStepBuilder {
            TestExecutionStepBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An issue detected occurring during a test execution."]
    pub struct TestIssue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "category")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Category of issue. Required."]
        pub category: ::std::option::Option<TestIssueCategoryEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A brief human-readable message describing the issue. Required."]
        pub error_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "severity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Severity of issue. Required."]
        pub severity: ::std::option::Option<TestIssueSeverityEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stackTrace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated in favor of stack trace fields inside specific warnings."]
        pub stack_trace: ::std::option::Option<::std::boxed::Box<StackTrace>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of issue. Required."]
        pub _type: ::std::option::Option<TestIssueTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warning")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Warning message with additional details of the issue. Should always be a message from com.google.devtools.toolresults.v1.warnings"]
        pub warning: ::std::option::Option<::std::boxed::Box<Any>>,
    }
    impl TestIssue {
        pub fn builder() -> TestIssueBuilder {
            TestIssueBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Category of issue. Required."]
    pub enum TestIssueCategoryEnum {
        #[serde(rename = "unspecifiedCategory")]
        #[doc = "Default unspecified category. Do not use. For versioning only."]
        UnspecifiedCategory,
        #[serde(rename = "common")]
        #[doc = "Issue is not specific to a particular test kind (e.g., a native crash)."]
        Common,
        #[serde(rename = "robo")]
        #[doc = "Issue is specific to Robo run."]
        Robo,
    }
    impl ::std::default::Default for TestIssueCategoryEnum {
        fn default() -> Self {
            Self::UnspecifiedCategory
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Severity of issue. Required."]
    pub enum TestIssueSeverityEnum {
        #[serde(rename = "unspecifiedSeverity")]
        #[doc = "Default unspecified severity. Do not use. For versioning only."]
        UnspecifiedSeverity,
        #[serde(rename = "info")]
        #[doc = "Non critical issue, providing users with some info about the test run."]
        Info,
        #[serde(rename = "suggestion")]
        #[doc = "Non critical issue, providing users with some hints on improving their testing experience, e.g., suggesting to use Game Loops."]
        Suggestion,
        #[serde(rename = "warning")]
        #[doc = "Potentially critical issue."]
        Warning,
        #[serde(rename = "severe")]
        #[doc = "Critical issue."]
        Severe,
    }
    impl ::std::default::Default for TestIssueSeverityEnum {
        fn default() -> Self {
            Self::UnspecifiedSeverity
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Type of issue. Required."]
    pub enum TestIssueTypeEnum {
        #[serde(rename = "unspecifiedType")]
        #[doc = "Default unspecified type. Do not use. For versioning only."]
        UnspecifiedType,
        #[serde(rename = "fatalException")]
        #[doc = "Issue is a fatal exception."]
        FatalException,
        #[serde(rename = "nativeCrash")]
        #[doc = "Issue is a native crash."]
        NativeCrash,
        #[serde(rename = "anr")]
        #[doc = "Issue is an ANR crash."]
        Anr,
        #[serde(rename = "unusedRoboDirective")]
        #[doc = "Issue is an unused robo directive."]
        UnusedRoboDirective,
        #[serde(rename = "compatibleWithOrchestrator")]
        #[doc = "Issue is a suggestion to use orchestrator."]
        CompatibleWithOrchestrator,
        #[serde(rename = "launcherActivityNotFound")]
        #[doc = "Issue with finding a launcher activity"]
        LauncherActivityNotFound,
        #[serde(rename = "startActivityNotFound")]
        #[doc = "Issue with resolving a user-provided intent to start an activity"]
        StartActivityNotFound,
        #[serde(rename = "incompleteRoboScriptExecution")]
        #[doc = "A Robo script was not fully executed."]
        IncompleteRoboScriptExecution,
        #[serde(rename = "completeRoboScriptExecution")]
        #[doc = "A Robo script was fully and successfully executed."]
        CompleteRoboScriptExecution,
        #[serde(rename = "failedToInstall")]
        #[doc = "The APK failed to install."]
        FailedToInstall,
        #[serde(rename = "availableDeepLinks")]
        #[doc = "The app-under-test has deep links, but none were provided to Robo."]
        AvailableDeepLinks,
        #[serde(rename = "nonSdkApiUsageViolation")]
        #[doc = "App accessed a non-sdk Api."]
        NonSdkApiUsageViolation,
        #[serde(rename = "nonSdkApiUsageReport")]
        #[doc = "App accessed a non-sdk Api (new detailed report)"]
        NonSdkApiUsageReport,
        #[serde(rename = "encounteredNonAndroidUiWidgetScreen")]
        #[doc = "Robo crawl encountered at least one screen with elements that are not Android UI widgets."]
        EncounteredNonAndroidUiWidgetScreen,
        #[serde(rename = "encounteredLoginScreen")]
        #[doc = "Robo crawl encountered at least one probable login screen."]
        EncounteredLoginScreen,
        #[serde(rename = "performedGoogleLogin")]
        #[doc = "Robo signed in with Google."]
        PerformedGoogleLogin,
        #[serde(rename = "iosException")]
        #[doc = "iOS App crashed with an exception."]
        IosException,
        #[serde(rename = "iosCrash")]
        #[doc = "iOS App crashed without an exception (e.g. killed)."]
        IosCrash,
        #[serde(rename = "performedMonkeyActions")]
        #[doc = "Robo crawl involved performing some monkey actions."]
        PerformedMonkeyActions,
        #[serde(rename = "usedRoboDirective")]
        #[doc = "Robo crawl used a Robo directive."]
        UsedRoboDirective,
        #[serde(rename = "usedRoboIgnoreDirective")]
        #[doc = "Robo crawl used a Robo directive to ignore an UI element."]
        UsedRoboIgnoreDirective,
        #[serde(rename = "insufficientCoverage")]
        #[doc = "Robo did not crawl some potentially important parts of the app."]
        InsufficientCoverage,
        #[serde(rename = "inAppPurchases")]
        #[doc = "Robo crawl involved some in-app purchases."]
        InAppPurchases,
        #[serde(rename = "crashDialogError")]
        #[doc = "Crash dialog was detected during the test execution"]
        CrashDialogError,
        #[serde(rename = "uiElementsTooDeep")]
        #[doc = "UI element depth is greater than the threshold"]
        UiElementsTooDeep,
        #[serde(rename = "blankScreen")]
        #[doc = "Blank screen is found in the Robo crawl"]
        BlankScreen,
        #[serde(rename = "overlappingUiElements")]
        #[doc = "Overlapping UI elements are found in the Robo crawl"]
        OverlappingUiElements,
        #[serde(rename = "unityException")]
        #[doc = "An uncaught Unity exception was detected (these don't crash apps)."]
        UnityException,
        #[serde(rename = "deviceOutOfMemory")]
        #[doc = "Device running out of memory was detected"]
        DeviceOutOfMemory,
        #[serde(rename = "logcatCollectionError")]
        #[doc = "Problems detected while collecting logcat"]
        LogcatCollectionError,
    }
    impl ::std::default::Default for TestIssueTypeEnum {
        fn default() -> Self {
            Self::UnspecifiedType
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A summary of a test suite result either parsed from XML or uploaded directly by a user. Note: the API related comments are for StepService only. This message is also being used in ExecutionService in a read only mode for the corresponding step."]
    pub struct TestSuiteOverview {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "elapsedTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Elapsed time of test suite."]
        pub elapsed_time: ::std::option::Option<::std::boxed::Box<Duration>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of test cases in error, typically set by the service by parsing the xml_source. - In create/response: always set - In update request: never"]
        pub error_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "failureCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of failed test cases, typically set by the service by parsing the xml_source. May also be set by the user. - In create/response: always set - In update request: never"]
        pub failure_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "flakyCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of flaky test cases, set by the service by rolling up flaky test attempts. Present only for rollup test suite overview at environment level. A step cannot have flaky test cases."]
        pub flaky_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the test suite. - In create/response: always set - In update request: never"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "skippedCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of test cases not run, typically set by the service by parsing the xml_source. - In create/response: always set - In update request: never"]
        pub skipped_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of test cases, typically set by the service by parsing the xml_source. - In create/response: always set - In update request: never"]
        pub total_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "xmlSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If this test suite was parsed from XML, this is the URI where the original XML file is stored. Note: Multiple test suites can share the same xml_source Returns INVALID_ARGUMENT if the uri format is not supported. - In create/response: optional - In update request: never"]
        pub xml_source: ::std::option::Option<::std::boxed::Box<FileReference>>,
    }
    impl TestSuiteOverview {
        pub fn builder() -> TestSuiteOverviewBuilder {
            TestSuiteOverviewBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Testing timing break down to know phases."]
    pub struct TestTiming {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testProcessDuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How long it took to run the test process. - In response: present if previously set. - In create/update request: optional"]
        pub test_process_duration: ::std::option::Option<::std::boxed::Box<Duration>>,
    }
    impl TestTiming {
        pub fn builder() -> TestTimingBuilder {
            TestTimingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single thumbnail, with its size and format."]
    pub struct Thumbnail {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The thumbnail's content type, i.e. \"image/png\". Always set."]
        pub content_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The thumbnail file itself. That is, the bytes here are precisely the bytes that make up the thumbnail file; they can be served as an image as-is (with the appropriate content type.) Always set."]
        pub data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "heightPx")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The height of the thumbnail, in pixels. Always set."]
        pub height_px: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "widthPx")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The width of the thumbnail, in pixels. Always set."]
        pub width_px: ::std::option::Option<::std::primitive::i64>,
    }
    impl Thumbnail {
        pub fn builder() -> ThumbnailBuilder {
            ThumbnailBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Timestamp represents a point in time independent of any time zone or local calendar, encoded as a count of seconds and fractions of seconds at nanosecond resolution. The count is relative to an epoch at UTC midnight on January 1, 1970, in the proleptic Gregorian calendar which extends the Gregorian calendar backwards to year one. All minutes are 60 seconds long. Leap seconds are \"smeared\" so that no leap second table is needed for interpretation, using a [24-hour linear smear](https://developers.google.com/time/smear). The range is from 0001-01-01T00:00:00Z to 9999-12-31T23:59:59.999999999Z. By restricting to that range, we ensure that we can convert to and from [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) date strings."]
    pub struct Timestamp {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nanos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Non-negative fractions of a second at nanosecond resolution. Negative second values with fractions must still have non-negative nanos values that count forward in time. Must be from 0 to 999,999,999 inclusive."]
        pub nanos: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "seconds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents seconds of UTC time since Unix epoch 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to 9999-12-31T23:59:59Z inclusive."]
        pub seconds: ::std::option::Option<::std::string::String>,
    }
    impl Timestamp {
        pub fn builder() -> TimestampBuilder {
            TimestampBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An execution of an arbitrary tool. It could be a test runner or a tool copying artifacts or deploying code."]
    pub struct ToolExecution {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commandLineArguments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full tokenized command line including the program name (equivalent to argv in a C program). - In response: present if set by create request - In create request: optional - In update request: never set"]
        pub command_line_arguments: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exitCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tool execution exit code. This field will be set once the tool has exited. - In response: present if set by create/update request - In create request: optional - In update request: optional, a FAILED_PRECONDITION error will be returned if an exit_code is already set."]
        pub exit_code: ::std::option::Option<::std::boxed::Box<ToolExitCode>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "toolLogs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "References to any plain text logs output the tool execution. This field can be set before the tool has exited in order to be able to have access to a live view of the logs while the tool is running. The maximum allowed number of tool logs per step is 1000. - In response: present if set by create/update request - In create request: optional - In update request: optional, any value provided will be appended to the existing list"]
        pub tool_logs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FileReference>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "toolOutputs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "References to opaque files of any format output by the tool execution. The maximum allowed number of tool outputs per step is 1000. - In response: present if set by create/update request - In create request: optional - In update request: optional, any value provided will be appended to the existing list"]
        pub tool_outputs:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ToolOutputReference>>>,
    }
    impl ToolExecution {
        pub fn builder() -> ToolExecutionBuilder {
            ToolExecutionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Generic tool step to be used for binaries we do not explicitly support. For example: running cp to copy artifacts from one location to another."]
    pub struct ToolExecutionStep {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "toolExecution")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A Tool execution. - In response: present if set by create/update request - In create/update request: optional"]
        pub tool_execution: ::std::option::Option<::std::boxed::Box<ToolExecution>>,
    }
    impl ToolExecutionStep {
        pub fn builder() -> ToolExecutionStepBuilder {
            ToolExecutionStepBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Exit code from a tool execution."]
    pub struct ToolExitCode {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "number")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tool execution exit code. A value of 0 means that the execution was successful. - In response: always set - In create/update request: always set"]
        pub number: ::std::option::Option<::std::primitive::i64>,
    }
    impl ToolExitCode {
        pub fn builder() -> ToolExitCodeBuilder {
            ToolExitCodeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A reference to a ToolExecution output file."]
    pub struct ToolOutputReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creation time of the file. - In response: present if set by create/update request - In create/update request: optional"]
        pub creation_time: ::std::option::Option<::std::boxed::Box<Timestamp>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "output")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A FileReference to an output file. - In response: always set - In create/update request: always set"]
        pub output: ::std::option::Option<::std::boxed::Box<FileReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testCase")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The test case to which this output file belongs. - In response: present if set by create/update request - In create/update request: optional"]
        pub test_case: ::std::option::Option<::std::boxed::Box<TestCaseReference>>,
    }
    impl ToolOutputReference {
        pub fn builder() -> ToolOutputReferenceBuilder {
            ToolOutputReferenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A warning that the screen hierarchy is deeper than the recommended threshold."]
    pub struct UiElementTooDeep {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "depth")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The depth of the screen element"]
        pub depth: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "screenId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The screen id of the element"]
        pub screen_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "screenStateId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The screen state id of the element"]
        pub screen_state_id: ::std::option::Option<::std::string::String>,
    }
    impl UiElementTooDeep {
        pub fn builder() -> UiElementTooDeepBuilder {
            UiElementTooDeepBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Default unspecified warning."]
    pub struct UnspecifiedWarning {}
    impl UnspecifiedWarning {
        pub fn builder() -> UnspecifiedWarningBuilder {
            UnspecifiedWarningBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional details of an unused robodirective."]
    pub struct UnusedRoboDirective {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the resource that was unused."]
        pub resource_name: ::std::option::Option<::std::string::String>,
    }
    impl UnusedRoboDirective {
        pub fn builder() -> UnusedRoboDirectiveBuilder {
            UnusedRoboDirectiveBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This insight is a recommendation to upgrade a given library to the specified version, in order to avoid dependencies on non-SDK APIs."]
    pub struct UpgradeInsight {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the package to be upgraded."]
        pub package_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "upgradeToVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The suggested version to upgrade to. Optional: In case we are not sure which version solves this problem"]
        pub upgrade_to_version: ::std::option::Option<::std::string::String>,
    }
    impl UpgradeInsight {
        pub fn builder() -> UpgradeInsightBuilder {
            UpgradeInsightBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional details of a used Robo directive."]
    pub struct UsedRoboDirective {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the resource that was used."]
        pub resource_name: ::std::option::Option<::std::string::String>,
    }
    impl UsedRoboDirective {
        pub fn builder() -> UsedRoboDirectiveBuilder {
            UsedRoboDirectiveBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional details of a used Robo directive with an ignore action. Note: This is a different scenario than unused directive."]
    pub struct UsedRoboIgnoreDirective {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the resource that was ignored."]
        pub resource_name: ::std::option::Option<::std::string::String>,
    }
    impl UsedRoboIgnoreDirective {
        pub fn builder() -> UsedRoboIgnoreDirectiveBuilder {
            UsedRoboIgnoreDirectiveBuilder::default()
        }
    }
}
