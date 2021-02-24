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
                    #[serde(rename = "filterAdClientId")]
                    #[doc = "Ad clients to list accounts for."]
                    pub filter_ad_client_id: ::std::string::String,
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
                            #[serde(rename = "hostCustomChannelId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Host custom channel to attach to the ad code."]
                            pub host_custom_channel_id:
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
                            #[serde(rename = "adUnitId")]
                            #[doc = "Ad unit to get."]
                            pub ad_unit_id: ::std::string::String,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
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
    pub mod associationsessions {
        pub mod methods {
            pub mod start {
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
                    #[serde(rename = "callbackUrl")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The URL to redirect the user to once association is completed. It receives a token parameter that can then be used to retrieve the associated account."]
                    pub callback_url: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "productCode")]
                    #[doc = "Products to associate with the user."]
                    pub product_code: QueryParametersProductCodeEnum,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userLocale")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The preferred locale of the user."]
                    pub user_locale: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "websiteLocale")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The locale of the user's hosted website."]
                    pub website_locale: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "websiteUrl")]
                    #[doc = "The URL of the user's hosted website."]
                    pub website_url: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Products to associate with the user."]
                pub enum QueryParametersProductCodeEnum {
                    #[serde(rename = "AFC")]
                    #[doc = "AdSense For Content"]
                    Afc,
                    #[serde(rename = "AFG")]
                    #[doc = "AdSense For Games"]
                    Afg,
                    #[serde(rename = "AFMC")]
                    #[doc = "AdSense For Mobile Content - deprecated"]
                    Afmc,
                    #[serde(rename = "AFS")]
                    #[doc = "AdSense For Search - deprecated"]
                    Afs,
                    #[serde(rename = "AFV")]
                    #[doc = "AdSense For Video"]
                    Afv,
                }
                impl ::std::default::Default for QueryParametersProductCodeEnum {
                    fn default() -> Self {
                        Self::Afc
                    }
                }
            }
            pub mod verify {
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
                    #[serde(rename = "token")]
                    #[doc = "The token returned to the association callback URL."]
                    pub token: ::std::string::String,
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
                    #[serde(rename = "customChannelId")]
                    #[doc = "Custom channel to get."]
                    pub custom_channel_id: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier of this account."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ account_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "account_defaults :: kind")]
        #[doc = "Kind of resource this is, in this case adsensehost#account."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of this account."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Approval status of this account. One of: PENDING, APPROVED, DISABLED."]
        pub status: ::std::option::Option<::std::string::String>,
    }
    impl Account {
        pub fn builder() -> AccountBuilder {
            AccountBuilder::default()
        }
    }
    mod account_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsensehost#account\"").unwrap()
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
        #[doc = "Kind of list this is, in this case adsensehost#accounts."]
        pub kind: ::std::string::String,
    }
    impl Accounts {
        pub fn builder() -> AccountsBuilder {
            AccountsBuilder::default()
        }
    }
    mod accounts_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsensehost#accounts\"").unwrap()
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
        #[doc = "Kind of resource this is, in this case adsensehost#adClient."]
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
            serde_json::from_str(&"\"adsensehost#adClient\"").unwrap()
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
        #[doc = "Kind of list this is, in this case adsensehost#adClients."]
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
            serde_json::from_str(&"\"adsensehost#adClients\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AdCode {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ad code snippet."]
        pub ad_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ad_code_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "ad_code_defaults :: kind")]
        #[doc = "Kind this is, in this case adsensehost#adCode."]
        pub kind: ::std::string::String,
    }
    impl AdCode {
        pub fn builder() -> AdCodeBuilder {
            AdCodeBuilder::default()
        }
    }
    mod ad_code_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsensehost#adCode\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AdStyle {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The colors included in the style. These are represented as six hexadecimal characters, similar to HTML color codes, but without the leading hash."]
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
        #[doc = "Kind this is, in this case adsensehost#adStyle."]
        pub kind: ::std::string::String,
    }
    impl AdStyle {
        pub fn builder() -> AdStyleBuilder {
            AdStyleBuilder::default()
        }
    }
    mod ad_style_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsensehost#adStyle\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The colors included in the style. These are represented as six hexadecimal characters, similar to HTML color codes, but without the leading hash."]
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
        #[doc = "The family of the font. Possible values are: ACCOUNT_DEFAULT_FAMILY, ADSENSE_DEFAULT_FAMILY, ARIAL, TIMES and VERDANA."]
        pub family: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size of the font. Possible values are: ACCOUNT_DEFAULT_SIZE, ADSENSE_DEFAULT_SIZE, SMALL, MEDIUM and LARGE."]
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
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier of this ad unit. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ad_unit_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "ad_unit_defaults :: kind")]
        #[doc = "Kind of resource this is, in this case adsensehost#adUnit."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mobileContentAdsSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Settings specific to WAP mobile content ads (AFMC - deprecated)."]
        pub mobile_content_ads_settings: ::std::option::Option<AdUnitMobileContentAdsSettings>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of this ad unit."]
        pub name: ::std::option::Option<::std::string::String>,
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
            serde_json::from_str(&"\"adsensehost#adUnit\"").unwrap()
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
        #[doc = "Size of this ad unit. Size values are in the form SIZE_{width}_{height}."]
        pub size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of this ad unit. Possible values are TEXT, TEXT_IMAGE, IMAGE and LINK."]
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
        #[doc = "Color to use when type is set to COLOR. These are represented as six hexadecimal characters, similar to HTML color codes, but without the leading hash."]
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
    #[doc = "Settings specific to WAP mobile content ads (AFMC - deprecated)."]
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
        #[doc = "Kind of list this is, in this case adsensehost#adUnits."]
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
            serde_json::from_str(&"\"adsensehost#adUnits\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AssociationSession {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hosted account id of the associated publisher after association. Present if status is ACCEPTED."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier of this association session."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ association_session_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "association_session_defaults :: kind")]
        #[doc = "Kind of resource this is, in this case adsensehost#associationSession."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productCodes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The products to associate with the user. Options: AFC, AFG, AFV, AFS (deprecated), AFMC (deprecated)"]
        pub product_codes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "redirectUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Redirect URL of this association session. Used to redirect users into the AdSense association flow."]
        pub redirect_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of the completed association, available once the association callback token has been verified. One of ACCEPTED, REJECTED, or ERROR."]
        pub status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userLocale")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The preferred locale of the user themselves when going through the AdSense association flow."]
        pub user_locale: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "websiteLocale")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The locale of the user's hosted website."]
        pub website_locale: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "websiteUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the user's hosted website."]
        pub website_url: ::std::option::Option<::std::string::String>,
    }
    impl AssociationSession {
        pub fn builder() -> AssociationSessionBuilder {
            AssociationSessionBuilder::default()
        }
    }
    mod association_session_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsensehost#associationSession\"").unwrap()
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
        #[doc = "Kind of resource this is, in this case adsensehost#customChannel."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of this custom channel."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl CustomChannel {
        pub fn builder() -> CustomChannelBuilder {
            CustomChannelBuilder::default()
        }
    }
    mod custom_channel_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsensehost#customChannel\"").unwrap()
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
        #[doc = "Kind of list this is, in this case adsensehost#customChannels."]
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
            serde_json::from_str(&"\"adsensehost#customChannels\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Report {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "averages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The averages of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty."]
        pub averages: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The header information of the columns requested in the report. This is a list of headers; one for each dimension in the request, followed by one for each metric in the request."]
        pub headers: ::std::option::Option<::std::vec::Vec<ReportHeaders>>,
        #[builder(default = "{ report_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "report_defaults :: kind")]
        #[doc = "Kind this is, in this case adsensehost#report."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The output rows of the report. Each row is a list of cells; one for each dimension in the request, followed by one for each metric in the request. The dimension cells contain strings, and the metric cells contain numbers."]
        pub rows: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
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
    impl Report {
        pub fn builder() -> ReportBuilder {
            ReportBuilder::default()
        }
    }
    mod report_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"adsensehost#report\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ReportHeaders {
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
    impl ReportHeaders {
        pub fn builder() -> ReportHeadersBuilder {
            ReportHeadersBuilder::default()
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
        #[doc = "Kind of resource this is, in this case adsensehost#urlChannel."]
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
            serde_json::from_str(&"\"adsensehost#urlChannel\"").unwrap()
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
        #[doc = "Kind of list this is, in this case adsensehost#urlChannels."]
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
            serde_json::from_str(&"\"adsensehost#urlChannels\"").unwrap()
        }
    }
}
