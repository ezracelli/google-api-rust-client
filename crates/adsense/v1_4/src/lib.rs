#[derive(
    Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
)]
pub struct QueryParameters {
    #[builder(default = "{ query_parameters_defaults :: alt () }", setter(into))]
    #[serde(rename = "alt")]
    #[serde(default = "query_parameters_defaults :: alt")]
    #[doc = "Data format for the response."]
    pub alt: QueryParametersAltEnum,
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
    #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
    pub quota_user: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "userIp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Please use quotaUser instead."]
    pub user_ip: ::std::option::Option<::std::string::String>,
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
#[doc = "Data format for the response."]
pub enum QueryParametersAltEnum {
    #[serde(rename = "csv")]
    #[doc = "Responses with Content-Type of text/csv"]
    Csv,
    #[serde(rename = "json")]
    #[doc = "Responses with Content-Type of application/json"]
    Json,
}
impl ::std::default::Default for QueryParametersAltEnum {
    fn default() -> Self {
        Self::Json
    }
}
pub mod resources {
    pub mod accounts {
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
                    #[serde(rename = "tree")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether the tree of sub accounts should be returned."]
                    pub tree: ::std::option::Option<::std::primitive::bool>,
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
                    #[doc = "The maximum number of accounts to include in the response, used for paging."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A continuation token, used to page through accounts. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
            pub mod adclients {
                pub mod methods {
                    pub mod get_ad_code {
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
                            #[serde(rename = "tagPartner")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Tag partner to include in the ad code snippet."]
                            pub tag_partner: ::std::option::Option<::std::string::String>,
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
                            #[serde(rename = "maxResults")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of ad clients to include in the response, used for paging."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A continuation token, used to page through ad clients. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
            pub mod adunits {
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
                            #[serde(rename = "includeInactive")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Whether to include inactive ad units. Default: true."]
                            pub include_inactive: ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "maxResults")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of ad units to include in the response, used for paging."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A continuation token, used to page through ad units. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
                    pub mod customchannels {
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
                                    #[serde(rename = "maxResults")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The maximum number of custom channels to include in the response, used for paging."]
                                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A continuation token, used to page through custom channels. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
            pub mod alerts {
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
                            #[serde(rename = "locale")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The locale to use for translating alert messages. The account locale will be used if this is not supplied. The AdSense default (English) will be used if the supplied locale is invalid or unsupported."]
                            pub locale: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod customchannels {
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
                            #[serde(rename = "maxResults")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of custom channels to include in the response, used for paging."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A continuation token, used to page through custom channels. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
                    pub mod adunits {
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
                                    #[serde(rename = "includeInactive")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Whether to include inactive ad units. Default: true."]
                                    pub include_inactive:
                                        ::std::option::Option<::std::primitive::bool>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "maxResults")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The maximum number of ad units to include in the response, used for paging."]
                                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A continuation token, used to page through ad units. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
            pub mod reports {
                pub mod methods {
                    pub mod generate {
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
                            #[serde(rename = "currency")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional currency to use when reporting on monetary metrics. Defaults to the account's currency if not set."]
                            pub currency: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "dimension")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Dimensions to base the report on."]
                            pub dimension: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "endDate")]
                            #[doc = "End of the date range to report on in \"YYYY-MM-DD\" format, inclusive."]
                            pub end_date: ::std::string::String,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "filter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Filters to be run on the report."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "locale")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional locale to use for translating report output to a local language. Defaults to \"en_US\" if not specified."]
                            pub locale: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "maxResults")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of rows of report data to return."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "metric")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Numeric columns to include in the report."]
                            pub metric: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "sort")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The name of a dimension or metric to sort the resulting report on, optionally prefixed with \"+\" to sort ascending or \"-\" to sort descending. If no prefix is specified, the column is sorted ascending."]
                            pub sort: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "startDate")]
                            #[doc = "Start of the date range to report on in \"YYYY-MM-DD\" format, inclusive."]
                            pub start_date: ::std::string::String,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "startIndex")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Index of the first row of report data to return."]
                            pub start_index: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "useTimezoneReporting")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Whether the report should be generated in the AdSense account's local timezone. If false default PST/PDT timezone will be used."]
                            pub use_timezone_reporting:
                                ::std::option::Option<::std::primitive::bool>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
                pub mod resources {
                    pub mod saved {
                        pub mod methods {
                            pub mod generate {
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
                                    #[doc = "Optional locale to use for translating report output to a local language. Defaults to \"en_US\" if not specified."]
                                    pub locale: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "maxResults")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The maximum number of rows of report data to return."]
                                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "startIndex")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Index of the first row of report data to return."]
                                    pub start_index: ::std::option::Option<::std::primitive::i64>,
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
                                    #[serde(rename = "maxResults")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The maximum number of saved reports to include in the response, used for paging."]
                                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A continuation token, used to page through saved reports. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
            pub mod savedadstyles {
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
                            #[serde(rename = "maxResults")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of saved ad styles to include in the response, used for paging."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A continuation token, used to page through saved ad styles. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
            pub mod urlchannels {
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
                            #[serde(rename = "maxResults")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of URL channels to include in the response, used for paging."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A continuation token, used to page through URL channels. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
    pub mod adclients {
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
                    #[doc = "The maximum number of ad clients to include in the response, used for paging."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A continuation token, used to page through ad clients. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
    pub mod adunits {
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
                    #[serde(rename = "includeInactive")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to include inactive ad units. Default: true."]
                    pub include_inactive: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of ad units to include in the response, used for paging."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A continuation token, used to page through ad units. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
            pub mod customchannels {
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
                            #[serde(rename = "maxResults")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of custom channels to include in the response, used for paging."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A continuation token, used to page through custom channels. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
    pub mod alerts {
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
                    #[serde(rename = "locale")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The locale to use for translating alert messages. The account locale will be used if this is not supplied. The AdSense default (English) will be used if the supplied locale is invalid or unsupported."]
                    pub locale: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod customchannels {
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
                    #[doc = "The maximum number of custom channels to include in the response, used for paging."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A continuation token, used to page through custom channels. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
            pub mod adunits {
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
                            #[serde(rename = "includeInactive")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Whether to include inactive ad units. Default: true."]
                            pub include_inactive: ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "maxResults")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of ad units to include in the response, used for paging."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A continuation token, used to page through ad units. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
    pub mod reports {
        pub mod methods {
            pub mod generate {
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
                    #[serde(rename = "accountId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Accounts upon which to report."]
                    pub account_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "currency")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional currency to use when reporting on monetary metrics. Defaults to the account's currency if not set."]
                    pub currency: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "dimension")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Dimensions to base the report on."]
                    pub dimension: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "endDate")]
                    #[doc = "End of the date range to report on in \"YYYY-MM-DD\" format, inclusive."]
                    pub end_date: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "filter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filters to be run on the report."]
                    pub filter: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "locale")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional locale to use for translating report output to a local language. Defaults to \"en_US\" if not specified."]
                    pub locale: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of rows of report data to return."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "metric")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Numeric columns to include in the report."]
                    pub metric: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sort")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The name of a dimension or metric to sort the resulting report on, optionally prefixed with \"+\" to sort ascending or \"-\" to sort descending. If no prefix is specified, the column is sorted ascending."]
                    pub sort: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "startDate")]
                    #[doc = "Start of the date range to report on in \"YYYY-MM-DD\" format, inclusive."]
                    pub start_date: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "startIndex")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Index of the first row of report data to return."]
                    pub start_index: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "useTimezoneReporting")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether the report should be generated in the AdSense account's local timezone. If false default PST/PDT timezone will be used."]
                    pub use_timezone_reporting: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
        pub mod resources {
            pub mod saved {
                pub mod methods {
                    pub mod generate {
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
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional locale to use for translating report output to a local language. Defaults to \"en_US\" if not specified."]
                            pub locale: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "maxResults")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of rows of report data to return."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "startIndex")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Index of the first row of report data to return."]
                            pub start_index: ::std::option::Option<::std::primitive::i64>,
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
                            #[serde(rename = "maxResults")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of saved reports to include in the response, used for paging."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A continuation token, used to page through saved reports. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
    pub mod savedadstyles {
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
                    #[doc = "The maximum number of saved ad styles to include in the response, used for paging."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A continuation token, used to page through saved ad styles. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
    pub mod urlchannels {
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
                    #[doc = "The maximum number of URL channels to include in the response, used for paging."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A continuation token, used to page through URL channels. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Account {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creation_time")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier of this account."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ account_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "account_defaults :: kind")]
        #[doc = "Kind of resource this is, in this case adsense#account."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of this account."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "premium")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this account is premium."]
        pub premium: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subAccounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sub accounts of the this account."]
        pub sub_accounts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Account>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timezone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "AdSense timezone of this account."]
        pub timezone: ::std::option::Option<::std::string::String>,
    }
    impl Account {
        pub fn builder() -> AccountBuilder {
            AccountBuilder::default()
        }
    }
    mod account_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsense#account\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Accounts {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of this response for caching purposes."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The accounts returned in this list response."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Account>>>,
        #[builder(default = "{ accounts_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "accounts_defaults :: kind")]
        #[doc = "Kind of list this is, in this case adsense#accounts."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token used to page through accounts. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl Accounts {
        pub fn builder() -> AccountsBuilder {
            AccountsBuilder::default()
        }
    }
    mod accounts_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsense#accounts\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AdClient {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "arcOptIn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this ad client is opted in to ARC."]
        pub arc_opt_in: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier of this ad client."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ad_client_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "ad_client_defaults :: kind")]
        #[doc = "Kind of resource this is, in this case adsense#adClient."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This ad client's product code, which corresponds to the PRODUCT_CODE report dimension."]
        pub product_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "supportsReporting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this ad client supports being reported on."]
        pub supports_reporting: ::std::option::Option<::std::primitive::bool>,
    }
    impl AdClient {
        pub fn builder() -> AdClientBuilder {
            AdClientBuilder::default()
        }
    }
    mod ad_client_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsense#adClient\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AdClients {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of this response for caching purposes."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ad clients returned in this list response."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AdClient>>>,
        #[builder(default = "{ ad_clients_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "ad_clients_defaults :: kind")]
        #[doc = "Kind of list this is, in this case adsense#adClients."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token used to page through ad clients. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl AdClients {
        pub fn builder() -> AdClientsBuilder {
            AdClientsBuilder::default()
        }
    }
    mod ad_clients_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsense#adClients\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AdCode {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Auto ad code snippet. The ad code snippet."]
        pub ad_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ampBody")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The AMP Auto ad code snippet that goes in the body of an AMP page."]
        pub amp_body: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ampHead")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The AMP Auto ad code snippet that goes in the head of an AMP page."]
        pub amp_head: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ad_code_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "ad_code_defaults :: kind")]
        #[doc = "Kind this is, in this case adsense#adCode."]
        pub kind: ::std::string::String,
    }
    impl AdCode {
        pub fn builder() -> AdCodeBuilder {
            AdCodeBuilder::default()
        }
    }
    mod ad_code_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsense#adCode\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AdStyle {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The colors which are included in the style. These are represented as six hexadecimal characters, similar to HTML color codes, but without the leading hash."]
        pub colors: ::std::option::Option<AdStyleColors>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "corners")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The style of the corners in the ad (deprecated: never populated, ignored)."]
        pub corners: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "font")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The font which is included in the style."]
        pub font: ::std::option::Option<AdStyleFont>,
        #[builder(default = "{ ad_style_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "ad_style_defaults :: kind")]
        #[doc = "Kind this is, in this case adsense#adStyle."]
        pub kind: ::std::string::String,
    }
    impl AdStyle {
        pub fn builder() -> AdStyleBuilder {
            AdStyleBuilder::default()
        }
    }
    mod ad_style_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsense#adStyle\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The colors which are included in the style. These are represented as six hexadecimal characters, similar to HTML color codes, but without the leading hash."]
    pub struct AdStyleColors {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "background")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the ad background."]
        pub background: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "border")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the ad border."]
        pub border: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the ad text."]
        pub text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the ad title."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the ad url."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl AdStyleColors {
        pub fn builder() -> AdStyleColorsBuilder {
            AdStyleColorsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The font which is included in the style."]
    pub struct AdStyleFont {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "family")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The family of the font."]
        pub family: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size of the font."]
        pub size: ::std::option::Option<::std::string::String>,
    }
    impl AdStyleFont {
        pub fn builder() -> AdStyleFontBuilder {
            AdStyleFontBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AdUnit {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identity code of this ad unit, not necessarily unique across ad clients."]
        pub code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentAdsSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Settings specific to content ads (AFC) and highend mobile content ads (AFMC - deprecated)."]
        pub content_ads_settings: ::std::option::Option<AdUnitContentAdsSettings>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom style information specific to this ad unit."]
        pub custom_style: ::std::option::Option<::std::boxed::Box<AdStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "feedAdsSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Settings specific to feed ads (AFF) - deprecated."]
        pub feed_ads_settings: ::std::option::Option<AdUnitFeedAdsSettings>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier of this ad unit. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ad_unit_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "ad_unit_defaults :: kind")]
        #[doc = "Kind of resource this is, in this case adsense#adUnit."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mobileContentAdsSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Settings specific to WAP mobile content ads (AFMC) - deprecated."]
        pub mobile_content_ads_settings: ::std::option::Option<AdUnitMobileContentAdsSettings>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of this ad unit."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "savedStyleId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the saved ad style which holds this ad unit's style information."]
        pub saved_style_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of this ad unit. Possible values are:\nNEW: Indicates that the ad unit was created within the last seven days and does not yet have any activity associated with it.\n\nACTIVE: Indicates that there has been activity on this ad unit in the last seven days.\n\nINACTIVE: Indicates that there has been no activity on this ad unit in the last seven days."]
        pub status: ::std::option::Option<::std::string::String>,
    }
    impl AdUnit {
        pub fn builder() -> AdUnitBuilder {
            AdUnitBuilder::default()
        }
    }
    mod ad_unit_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsense#adUnit\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Settings specific to content ads (AFC) and highend mobile content ads (AFMC - deprecated)."]
    pub struct AdUnitContentAdsSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backupOption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The backup option to be used in instances where no ad is available."]
        pub backup_option: ::std::option::Option<AdUnitContentAdsSettingsBackupOption>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Size of this ad unit."]
        pub size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of this ad unit."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl AdUnitContentAdsSettings {
        pub fn builder() -> AdUnitContentAdsSettingsBuilder {
            AdUnitContentAdsSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The backup option to be used in instances where no ad is available."]
    pub struct AdUnitContentAdsSettingsBackupOption {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "color")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Color to use when type is set to COLOR."]
        pub color: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of the backup option. Possible values are BLANK, COLOR and URL."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL to use when type is set to URL."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl AdUnitContentAdsSettingsBackupOption {
        pub fn builder() -> AdUnitContentAdsSettingsBackupOptionBuilder {
            AdUnitContentAdsSettingsBackupOptionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Settings specific to feed ads (AFF) - deprecated."]
    pub struct AdUnitFeedAdsSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adPosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The position of the ads relative to the feed entries."]
        pub ad_position: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frequency")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The frequency at which ads should appear in the feed (i.e. every N entries)."]
        pub frequency: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minimumWordCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minimum length an entry should be in order to have attached ads."]
        pub minimum_word_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of ads which should appear."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl AdUnitFeedAdsSettings {
        pub fn builder() -> AdUnitFeedAdsSettingsBuilder {
            AdUnitFeedAdsSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Settings specific to WAP mobile content ads (AFMC) - deprecated."]
    pub struct AdUnitMobileContentAdsSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "markupLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The markup language to use for this ad unit."]
        pub markup_language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scriptingLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The scripting language to use for this ad unit."]
        pub scripting_language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Size of this ad unit."]
        pub size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of this ad unit."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl AdUnitMobileContentAdsSettings {
        pub fn builder() -> AdUnitMobileContentAdsSettingsBuilder {
            AdUnitMobileContentAdsSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AdUnits {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of this response for caching purposes."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ad units returned in this list response."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AdUnit>>>,
        #[builder(default = "{ ad_units_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "ad_units_defaults :: kind")]
        #[doc = "Kind of list this is, in this case adsense#adUnits."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token used to page through ad units. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl AdUnits {
        pub fn builder() -> AdUnitsBuilder {
            AdUnitsBuilder::default()
        }
    }
    mod ad_units_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsense#adUnits\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AdsenseReportsGenerateResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "averages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The averages of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty."]
        pub averages: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The requested end date in yyyy-mm-dd format."]
        pub end_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The header information of the columns requested in the report. This is a list of headers; one for each dimension in the request, followed by one for each metric in the request."]
        pub headers: ::std::option::Option<::std::vec::Vec<AdsenseReportsGenerateResponseHeaders>>,
        #[builder(
            default = "{ adsense_reports_generate_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "adsense_reports_generate_response_defaults :: kind")]
        #[doc = "Kind this is, in this case adsense#report."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The output rows of the report. Each row is a list of cells; one for each dimension in the request, followed by one for each metric in the request. The dimension cells contain strings, and the metric cells contain numbers."]
        pub rows: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The requested start date in yyyy-mm-dd format."]
        pub start_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalMatchedRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of rows matched by the report request. Fewer rows may be returned in the response due to being limited by the row count requested or the report row limit."]
        pub total_matched_rows: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totals")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The totals of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty."]
        pub totals: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warnings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Any warnings associated with generation of the report."]
        pub warnings: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl AdsenseReportsGenerateResponse {
        pub fn builder() -> AdsenseReportsGenerateResponseBuilder {
            AdsenseReportsGenerateResponseBuilder::default()
        }
    }
    mod adsense_reports_generate_response_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsense#report\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AdsenseReportsGenerateResponseHeaders {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currency")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The currency of this column. Only present if the header type is METRIC_CURRENCY."]
        pub currency: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the header."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the header; one of DIMENSION, METRIC_TALLY, METRIC_RATIO, or METRIC_CURRENCY."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl AdsenseReportsGenerateResponseHeaders {
        pub fn builder() -> AdsenseReportsGenerateResponseHeadersBuilder {
            AdsenseReportsGenerateResponseHeadersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Alert {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier of this alert. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isDismissible")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this alert can be dismissed."]
        pub is_dismissible: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ alert_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "alert_defaults :: kind")]
        #[doc = "Kind of resource this is, in this case adsense#alert."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The localized alert message."]
        pub message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "severity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Severity of this alert. Possible values: INFO, WARNING, SEVERE."]
        pub severity: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of this alert. Possible values: SELF_HOLD, MIGRATED_TO_BILLING3, ADDRESS_PIN_VERIFICATION, PHONE_PIN_VERIFICATION, CORPORATE_ENTITY, GRAYLISTED_PUBLISHER, API_HOLD."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl Alert {
        pub fn builder() -> AlertBuilder {
            AlertBuilder::default()
        }
    }
    mod alert_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsense#alert\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Alerts {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The alerts returned in this list response."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Alert>>>,
        #[builder(default = "{ alerts_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "alerts_defaults :: kind")]
        #[doc = "Kind of list this is, in this case adsense#alerts."]
        pub kind: ::std::string::String,
    }
    impl Alerts {
        pub fn builder() -> AlertsBuilder {
            AlertsBuilder::default()
        }
    }
    mod alerts_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsense#alerts\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct CustomChannel {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Code of this custom channel, not necessarily unique across ad clients."]
        pub code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier of this custom channel. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ custom_channel_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "custom_channel_defaults :: kind")]
        #[doc = "Kind of resource this is, in this case adsense#customChannel."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of this custom channel."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetingInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The targeting information of this custom channel, if activated."]
        pub targeting_info: ::std::option::Option<CustomChannelTargetingInfo>,
    }
    impl CustomChannel {
        pub fn builder() -> CustomChannelBuilder {
            CustomChannelBuilder::default()
        }
    }
    mod custom_channel_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsense#customChannel\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The targeting information of this custom channel, if activated."]
    pub struct CustomChannelTargetingInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adsAppearOn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name used to describe this channel externally."]
        pub ads_appear_on: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The external description of the channel."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The locations in which ads appear. (Only valid for content and mobile content ads (deprecated)). Acceptable values for content ads are: TOP_LEFT, TOP_CENTER, TOP_RIGHT, MIDDLE_LEFT, MIDDLE_CENTER, MIDDLE_RIGHT, BOTTOM_LEFT, BOTTOM_CENTER, BOTTOM_RIGHT, MULTIPLE_LOCATIONS. Acceptable values for mobile content ads (deprecated) are: TOP, MIDDLE, BOTTOM, MULTIPLE_LOCATIONS."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "siteLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language of the sites ads will be displayed on."]
        pub site_language: ::std::option::Option<::std::string::String>,
    }
    impl CustomChannelTargetingInfo {
        pub fn builder() -> CustomChannelTargetingInfoBuilder {
            CustomChannelTargetingInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct CustomChannels {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of this response for caching purposes."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The custom channels returned in this list response."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomChannel>>>,
        #[builder(default = "{ custom_channels_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "custom_channels_defaults :: kind")]
        #[doc = "Kind of list this is, in this case adsense#customChannels."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token used to page through custom channels. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl CustomChannels {
        pub fn builder() -> CustomChannelsBuilder {
            CustomChannelsBuilder::default()
        }
    }
    mod custom_channels_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsense#customChannels\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Metadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub items:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ReportingMetadataEntry>>>,
        #[builder(default = "{ metadata_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "metadata_defaults :: kind")]
        #[doc = "Kind of list this is, in this case adsense#metadata."]
        pub kind: ::std::string::String,
    }
    impl Metadata {
        pub fn builder() -> MetadataBuilder {
            MetadataBuilder::default()
        }
    }
    mod metadata_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsense#metadata\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Payment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier of this Payment."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ payment_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "payment_defaults :: kind")]
        #[doc = "Kind of resource this is, in this case adsense#payment."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paymentAmount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount to be paid."]
        pub payment_amount: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paymentAmountCurrencyCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The currency code for the amount to be paid."]
        pub payment_amount_currency_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paymentDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date this payment was/will be credited to the user, or none if the payment threshold has not been met."]
        pub payment_date: ::std::option::Option<::std::string::String>,
    }
    impl Payment {
        pub fn builder() -> PaymentBuilder {
            PaymentBuilder::default()
        }
    }
    mod payment_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsense#payment\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Payments {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of Payments for the account. One or both of a) the account's most recent payment; and b) the account's upcoming payment."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Payment>>>,
        #[builder(default = "{ payments_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "payments_defaults :: kind")]
        #[doc = "Kind of list this is, in this case adsense#payments."]
        pub kind: ::std::string::String,
    }
    impl Payments {
        pub fn builder() -> PaymentsBuilder {
            PaymentsBuilder::default()
        }
    }
    mod payments_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsense#payments\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ReportingMetadataEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "compatibleDimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For metrics this is a list of dimension IDs which the metric is compatible with, for dimensions it is a list of compatibility groups the dimension belongs to."]
        pub compatible_dimensions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "compatibleMetrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The names of the metrics the dimension or metric this reporting metadata entry describes is compatible with."]
        pub compatible_metrics: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier of this reporting metadata entry, corresponding to the name of the appropriate dimension or metric."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ reporting_metadata_entry_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "reporting_metadata_entry_defaults :: kind")]
        #[doc = "Kind of resource this is, in this case adsense#reportingMetadataEntry."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requiredDimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The names of the dimensions which the dimension or metric this reporting metadata entry describes requires to also be present in order for the report to be valid. Omitting these will not cause an error or warning, but may result in data which cannot be correctly interpreted."]
        pub required_dimensions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requiredMetrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The names of the metrics which the dimension or metric this reporting metadata entry describes requires to also be present in order for the report to be valid. Omitting these will not cause an error or warning, but may result in data which cannot be correctly interpreted."]
        pub required_metrics: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "supportedProducts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The codes of the projects supported by the dimension or metric this reporting metadata entry describes."]
        pub supported_products: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ReportingMetadataEntry {
        pub fn builder() -> ReportingMetadataEntryBuilder {
            ReportingMetadataEntryBuilder::default()
        }
    }
    mod reporting_metadata_entry_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsense#reportingMetadataEntry\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct SavedAdStyle {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The AdStyle itself."]
        pub ad_style: ::std::option::Option<::std::boxed::Box<AdStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier of this saved ad style. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ saved_ad_style_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "saved_ad_style_defaults :: kind")]
        #[doc = "Kind of resource this is, in this case adsense#savedAdStyle."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user selected name of this SavedAdStyle."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl SavedAdStyle {
        pub fn builder() -> SavedAdStyleBuilder {
            SavedAdStyleBuilder::default()
        }
    }
    mod saved_ad_style_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsense#savedAdStyle\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct SavedAdStyles {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of this response for caching purposes."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The saved ad styles returned in this list response."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SavedAdStyle>>>,
        #[builder(default = "{ saved_ad_styles_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "saved_ad_styles_defaults :: kind")]
        #[doc = "Kind of list this is, in this case adsense#savedAdStyles."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token used to page through ad units. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl SavedAdStyles {
        pub fn builder() -> SavedAdStylesBuilder {
            SavedAdStylesBuilder::default()
        }
    }
    mod saved_ad_styles_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsense#savedAdStyles\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct SavedReport {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier of this saved report."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ saved_report_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "saved_report_defaults :: kind")]
        #[doc = "Kind of resource this is, in this case adsense#savedReport."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This saved report's name."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl SavedReport {
        pub fn builder() -> SavedReportBuilder {
            SavedReportBuilder::default()
        }
    }
    mod saved_report_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsense#savedReport\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct SavedReports {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of this response for caching purposes."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The saved reports returned in this list response."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SavedReport>>>,
        #[builder(default = "{ saved_reports_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "saved_reports_defaults :: kind")]
        #[doc = "Kind of list this is, in this case adsense#savedReports."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token used to page through saved reports. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl SavedReports {
        pub fn builder() -> SavedReportsBuilder {
            SavedReportsBuilder::default()
        }
    }
    mod saved_reports_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsense#savedReports\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct UrlChannel {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier of this URL channel. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ url_channel_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "url_channel_defaults :: kind")]
        #[doc = "Kind of resource this is, in this case adsense#urlChannel."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "urlPattern")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL Pattern of this URL channel. Does not include \"http://\" or \"https://\". Example: www.example.com/home"]
        pub url_pattern: ::std::option::Option<::std::string::String>,
    }
    impl UrlChannel {
        pub fn builder() -> UrlChannelBuilder {
            UrlChannelBuilder::default()
        }
    }
    mod url_channel_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsense#urlChannel\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct UrlChannels {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of this response for caching purposes."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL channels returned in this list response."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UrlChannel>>>,
        #[builder(default = "{ url_channels_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "url_channels_defaults :: kind")]
        #[doc = "Kind of list this is, in this case adsense#urlChannels."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token used to page through URL channels. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl UrlChannels {
        pub fn builder() -> UrlChannelsBuilder {
            UrlChannelsBuilder::default()
        }
    }
    mod url_channels_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsense#urlChannels\"").unwrap()
        }
    }
}
