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
        serde_json::from_str(&"json").unwrap()
    }
    pub fn pretty_print() -> ::std::primitive::bool {
        serde_json::from_str(&"false").unwrap()
    }
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Data format for the response."]
pub enum QueryParametersAltEnum {
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
    pub mod data {
        pub mod resources {
            pub mod ga {
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
                            #[serde(rename = "dimensions")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A comma-separated list of Analytics dimensions. E.g., 'ga:browser,ga:city'."]
                            pub dimensions: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "end-date")]
                            #[doc = "End date for fetching Analytics data. Request can should specify an end date formatted as YYYY-MM-DD, or as a relative date (e.g., today, yesterday, or 7daysAgo). The default value is yesterday."]
                            pub end_date: ::std::string::String,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "filters")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A comma-separated list of dimension or metric filters to be applied to Analytics data."]
                            pub filters: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "ids")]
                            #[doc = "Unique table ID for retrieving Analytics data. Table ID is of the form ga:XXXX, where XXXX is the Analytics view (profile) ID."]
                            pub ids: ::std::string::String,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "include-empty-rows")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The response will include empty rows if this parameter is set to true, the default is true"]
                            pub include_empty_rows: ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "max-results")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of entries to include in this feed."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "metrics")]
                            #[doc = "A comma-separated list of Analytics metrics. E.g., 'ga:sessions,ga:pageviews'. At least one metric must be specified."]
                            pub metrics: ::std::string::String,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "output")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The selected format for the response. Default format is JSON."]
                            pub output: ::std::option::Option<QueryParametersOutputEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "samplingLevel")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The desired sampling level."]
                            pub sampling_level:
                                ::std::option::Option<QueryParametersSamplingLevelEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "segment")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An Analytics segment to be applied to data."]
                            pub segment: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "sort")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A comma-separated list of dimensions or metrics that determine the sort order for Analytics data."]
                            pub sort: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "start-date")]
                            #[doc = "Start date for fetching Analytics data. Requests can specify a start date formatted as YYYY-MM-DD, or as a relative date (e.g., today, yesterday, or 7daysAgo). The default value is 7daysAgo."]
                            pub start_date: ::std::string::String,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "start-index")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An index of the first entity to retrieve. Use this parameter as a pagination mechanism along with the max-results parameter."]
                            pub start_index: ::std::option::Option<::std::primitive::i64>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "The selected format for the response. Default format is JSON."]
                        pub enum QueryParametersOutputEnum {
                            #[serde(rename = "dataTable")]
                            #[doc = "Returns the response in Google Charts Data Table format. This is useful in creating visualization using Google Charts."]
                            DataTable,
                            #[serde(rename = "json")]
                            #[doc = "Returns the response in standard JSON format."]
                            Json,
                        }
                        impl ::std::default::Default for QueryParametersOutputEnum {
                            fn default() -> Self {
                                Self::DataTable
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "The desired sampling level."]
                        pub enum QueryParametersSamplingLevelEnum {
                            #[serde(rename = "DEFAULT")]
                            #[doc = "Returns response with a sample size that balances speed and accuracy."]
                            Default,
                            #[serde(rename = "FASTER")]
                            #[doc = "Returns a fast response with a smaller sample size."]
                            Faster,
                            #[serde(rename = "HIGHER_PRECISION")]
                            #[doc = "Returns a more accurate response using a large sample size, but this may result in the response being slower."]
                            HigherPrecision,
                        }
                        impl ::std::default::Default for QueryParametersSamplingLevelEnum {
                            fn default() -> Self {
                                Self::Default
                            }
                        }
                    }
                }
            }
            pub mod mcf {
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
                            #[serde(rename = "dimensions")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A comma-separated list of Multi-Channel Funnels dimensions. E.g., 'mcf:source,mcf:medium'."]
                            pub dimensions: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "end-date")]
                            #[doc = "End date for fetching Analytics data. Requests can specify a start date formatted as YYYY-MM-DD, or as a relative date (e.g., today, yesterday, or 7daysAgo). The default value is 7daysAgo."]
                            pub end_date: ::std::string::String,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "filters")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A comma-separated list of dimension or metric filters to be applied to the Analytics data."]
                            pub filters: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "ids")]
                            #[doc = "Unique table ID for retrieving Analytics data. Table ID is of the form ga:XXXX, where XXXX is the Analytics view (profile) ID."]
                            pub ids: ::std::string::String,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "max-results")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of entries to include in this feed."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "metrics")]
                            #[doc = "A comma-separated list of Multi-Channel Funnels metrics. E.g., 'mcf:totalConversions,mcf:totalConversionValue'. At least one metric must be specified."]
                            pub metrics: ::std::string::String,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "samplingLevel")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The desired sampling level."]
                            pub sampling_level:
                                ::std::option::Option<QueryParametersSamplingLevelEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "sort")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A comma-separated list of dimensions or metrics that determine the sort order for the Analytics data."]
                            pub sort: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "start-date")]
                            #[doc = "Start date for fetching Analytics data. Requests can specify a start date formatted as YYYY-MM-DD, or as a relative date (e.g., today, yesterday, or 7daysAgo). The default value is 7daysAgo."]
                            pub start_date: ::std::string::String,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "start-index")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An index of the first entity to retrieve. Use this parameter as a pagination mechanism along with the max-results parameter."]
                            pub start_index: ::std::option::Option<::std::primitive::i64>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "The desired sampling level."]
                        pub enum QueryParametersSamplingLevelEnum {
                            #[serde(rename = "DEFAULT")]
                            #[doc = "Returns response with a sample size that balances speed and accuracy."]
                            Default,
                            #[serde(rename = "FASTER")]
                            #[doc = "Returns a fast response with a smaller sample size."]
                            Faster,
                            #[serde(rename = "HIGHER_PRECISION")]
                            #[doc = "Returns a more accurate response using a large sample size, but this may result in the response being slower."]
                            HigherPrecision,
                        }
                        impl ::std::default::Default for QueryParametersSamplingLevelEnum {
                            fn default() -> Self {
                                Self::Default
                            }
                        }
                    }
                }
            }
            pub mod realtime {
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
                            #[serde(rename = "dimensions")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A comma-separated list of real time dimensions. E.g., 'rt:medium,rt:city'."]
                            pub dimensions: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "filters")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A comma-separated list of dimension or metric filters to be applied to real time data."]
                            pub filters: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "ids")]
                            #[doc = "Unique table ID for retrieving real time data. Table ID is of the form ga:XXXX, where XXXX is the Analytics view (profile) ID."]
                            pub ids: ::std::string::String,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "max-results")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of entries to include in this feed."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "metrics")]
                            #[doc = "A comma-separated list of real time metrics. E.g., 'rt:activeUsers'. At least one metric must be specified."]
                            pub metrics: ::std::string::String,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "sort")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A comma-separated list of dimensions or metrics that determine the sort order for real time data."]
                            pub sort: ::std::option::Option<::std::string::String>,
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
    pub mod management {
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "max-results")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of account summaries to include in this response, where the largest acceptable value is 1000."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "start-index")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An index of the first entity to retrieve. Use this parameter as a pagination mechanism along with the max-results parameter."]
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
            pub mod account_user_links {
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
                            #[serde(rename = "max-results")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of account-user links to include in this response."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "start-index")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An index of the first account-user link to retrieve. Use this parameter as a pagination mechanism along with the max-results parameter."]
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "max-results")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of accounts to include in this response."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "start-index")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An index of the first account to retrieve. Use this parameter as a pagination mechanism along with the max-results parameter."]
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
            pub mod custom_data_sources {
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
                            #[serde(rename = "max-results")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of custom data sources to include in this response."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "start-index")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A 1-based index of the first custom data source to retrieve. Use this parameter as a pagination mechanism along with the max-results parameter."]
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
            pub mod custom_dimensions {
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
                            #[serde(rename = "max-results")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of custom dimensions to include in this response."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "start-index")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An index of the first entity to retrieve. Use this parameter as a pagination mechanism along with the max-results parameter."]
                            pub start_index: ::std::option::Option<::std::primitive::i64>,
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
                                default = "{ query_parameters_defaults :: ignore_custom_data_source_links () }",
                                setter(into)
                            )]
                            #[serde(rename = "ignoreCustomDataSourceLinks")]
                            #[serde(
                                default = "query_parameters_defaults :: ignore_custom_data_source_links"
                            )]
                            #[doc = "Force the update and ignore any warnings related to the custom dimension being linked to a custom data source / data set."]
                            pub ignore_custom_data_source_links: ::std::primitive::bool,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        mod query_parameters_defaults {
                            pub fn ignore_custom_data_source_links() -> ::std::primitive::bool {
                                serde_json::from_str(&"false").unwrap()
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
                                default = "{ query_parameters_defaults :: ignore_custom_data_source_links () }",
                                setter(into)
                            )]
                            #[serde(rename = "ignoreCustomDataSourceLinks")]
                            #[serde(
                                default = "query_parameters_defaults :: ignore_custom_data_source_links"
                            )]
                            #[doc = "Force the update and ignore any warnings related to the custom dimension being linked to a custom data source / data set."]
                            pub ignore_custom_data_source_links: ::std::primitive::bool,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        mod query_parameters_defaults {
                            pub fn ignore_custom_data_source_links() -> ::std::primitive::bool {
                                serde_json::from_str(&"false").unwrap()
                            }
                        }
                    }
                }
            }
            pub mod custom_metrics {
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
                            #[serde(rename = "max-results")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of custom metrics to include in this response."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "start-index")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An index of the first entity to retrieve. Use this parameter as a pagination mechanism along with the max-results parameter."]
                            pub start_index: ::std::option::Option<::std::primitive::i64>,
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
                                default = "{ query_parameters_defaults :: ignore_custom_data_source_links () }",
                                setter(into)
                            )]
                            #[serde(rename = "ignoreCustomDataSourceLinks")]
                            #[serde(
                                default = "query_parameters_defaults :: ignore_custom_data_source_links"
                            )]
                            #[doc = "Force the update and ignore any warnings related to the custom metric being linked to a custom data source / data set."]
                            pub ignore_custom_data_source_links: ::std::primitive::bool,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        mod query_parameters_defaults {
                            pub fn ignore_custom_data_source_links() -> ::std::primitive::bool {
                                serde_json::from_str(&"false").unwrap()
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
                                default = "{ query_parameters_defaults :: ignore_custom_data_source_links () }",
                                setter(into)
                            )]
                            #[serde(rename = "ignoreCustomDataSourceLinks")]
                            #[serde(
                                default = "query_parameters_defaults :: ignore_custom_data_source_links"
                            )]
                            #[doc = "Force the update and ignore any warnings related to the custom metric being linked to a custom data source / data set."]
                            pub ignore_custom_data_source_links: ::std::primitive::bool,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        mod query_parameters_defaults {
                            pub fn ignore_custom_data_source_links() -> ::std::primitive::bool {
                                serde_json::from_str(&"false").unwrap()
                            }
                        }
                    }
                }
            }
            pub mod experiments {
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
                            #[serde(rename = "max-results")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of experiments to include in this response."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "start-index")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An index of the first experiment to retrieve. Use this parameter as a pagination mechanism along with the max-results parameter."]
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
            pub mod filters {
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
                            #[serde(rename = "max-results")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of filters to include in this response."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "start-index")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An index of the first entity to retrieve. Use this parameter as a pagination mechanism along with the max-results parameter."]
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
            pub mod goals {
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
                            #[serde(rename = "max-results")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of goals to include in this response."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "start-index")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An index of the first goal to retrieve. Use this parameter as a pagination mechanism along with the max-results parameter."]
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
            pub mod profile_filter_links {
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
                            #[serde(rename = "max-results")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of profile filter links to include in this response."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "start-index")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An index of the first entity to retrieve. Use this parameter as a pagination mechanism along with the max-results parameter."]
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
            pub mod profile_user_links {
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
                            #[serde(rename = "max-results")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of profile-user links to include in this response."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "start-index")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An index of the first profile-user link to retrieve. Use this parameter as a pagination mechanism along with the max-results parameter."]
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
            pub mod profiles {
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
                            #[serde(rename = "max-results")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of views (profiles) to include in this response."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "start-index")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An index of the first entity to retrieve. Use this parameter as a pagination mechanism along with the max-results parameter."]
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
            pub mod remarketing_audience {
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
                            #[serde(rename = "max-results")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of remarketing audiences to include in this response."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "start-index")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An index of the first entity to retrieve. Use this parameter as a pagination mechanism along with the max-results parameter."]
                            pub start_index: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ query_parameters_defaults :: _type () }",
                                setter(into)
                            )]
                            #[serde(rename = "type")]
                            #[serde(default = "query_parameters_defaults :: _type")]
                            pub _type: ::std::string::String,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        mod query_parameters_defaults {
                            pub fn _type() -> ::std::string::String {
                                String::from("all")
                            }
                        }
                    }
                }
            }
            pub mod segments {
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
                            #[serde(rename = "max-results")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of segments to include in this response."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "start-index")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An index of the first segment to retrieve. Use this parameter as a pagination mechanism along with the max-results parameter."]
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
            pub mod unsampled_reports {
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
                            #[serde(rename = "max-results")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of unsampled reports to include in this response."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "start-index")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An index of the first unsampled report to retrieve. Use this parameter as a pagination mechanism along with the max-results parameter."]
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
            pub mod uploads {
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
                            #[serde(rename = "max-results")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of uploads to include in this response."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "start-index")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A 1-based index of the first upload to retrieve. Use this parameter as a pagination mechanism along with the max-results parameter."]
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
            pub mod web_property_ad_words_links {
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
                            #[serde(rename = "max-results")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of webProperty-Google Ads links to include in this response."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "start-index")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An index of the first webProperty-Google Ads link to retrieve. Use this parameter as a pagination mechanism along with the max-results parameter."]
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
            pub mod webproperties {
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
                            #[serde(rename = "max-results")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of web properties to include in this response."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "start-index")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An index of the first entity to retrieve. Use this parameter as a pagination mechanism along with the max-results parameter."]
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
            pub mod webproperty_user_links {
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
                            #[serde(rename = "max-results")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of webProperty-user Links to include in this response."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "start-index")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An index of the first webProperty-user link to retrieve. Use this parameter as a pagination mechanism along with the max-results parameter."]
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
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for Analytics account entry."]
    pub struct Account {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "childLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Child link for an account entry. Points to the list of web properties for this account."]
        pub child_link: ::std::option::Option<AccountChildLink>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "created")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time the account was created."]
        pub created: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ account_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "account_defaults :: kind")]
        #[doc = "Resource type for Analytics account."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Permissions the user has for this account."]
        pub permissions: ::std::option::Option<AccountPermissions>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link for this account."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "starred")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether this account is starred or not."]
        pub starred: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time the account was last modified."]
        pub updated: ::std::option::Option<::std::string::String>,
    }
    impl Account {
        pub fn builder() -> AccountBuilder {
            AccountBuilder::default()
        }
    }
    mod account_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#account")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Child link for an account entry. Points to the list of web properties for this account."]
    pub struct AccountChildLink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "href")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to the list of web properties for this account."]
        pub href: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ account_child_link_defaults :: _type () }", setter(into))]
        #[serde(rename = "type")]
        #[serde(default = "account_child_link_defaults :: _type")]
        #[doc = "Type of the child link. Its value is \"analytics#webproperties\"."]
        pub _type: ::std::string::String,
    }
    impl AccountChildLink {
        pub fn builder() -> AccountChildLinkBuilder {
            AccountChildLinkBuilder::default()
        }
    }
    mod account_child_link_defaults {
        pub fn _type() -> ::std::string::String {
            String::from("analytics#webproperties")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Permissions the user has for this account."]
    pub struct AccountPermissions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "effective")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All the permissions that the user has for this account. These include any implied permissions (e.g., EDIT implies VIEW)."]
        pub effective: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl AccountPermissions {
        pub fn builder() -> AccountPermissionsBuilder {
            AccountPermissionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for a linked account."]
    pub struct AccountRef {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "href")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link for this account."]
        pub href: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ account_ref_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "account_ref_defaults :: kind")]
        #[doc = "Analytics account reference."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account name."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl AccountRef {
        pub fn builder() -> AccountRefBuilder {
            AccountRefBuilder::default()
        }
    }
    mod account_ref_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#accountRef")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An AccountSummary collection lists a summary of accounts, properties and views (profiles) to which the user has access. Each resource in the collection corresponds to a single AccountSummary."]
    pub struct AccountSummaries {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of AccountSummaries."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountSummary>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemsPerPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter."]
        pub items_per_page: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ account_summaries_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "account_summaries_defaults :: kind")]
        #[doc = "Collection type."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to next page for this AccountSummary collection."]
        pub next_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previousLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to previous page for this AccountSummary collection."]
        pub previous_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of results for the query, regardless of the number of results in the response."]
        pub total_results: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "username")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email ID of the authenticated user"]
        pub username: ::std::option::Option<::std::string::String>,
    }
    impl AccountSummaries {
        pub fn builder() -> AccountSummariesBuilder {
            AccountSummariesBuilder::default()
        }
    }
    mod account_summaries_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#accountSummaries")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for an Analytics AccountSummary. An AccountSummary is a lightweight tree comprised of properties/profiles."]
    pub struct AccountSummary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ account_summary_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "account_summary_defaults :: kind")]
        #[doc = "Resource type for Analytics AccountSummary."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "starred")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether this account is starred or not."]
        pub starred: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of web properties under this account."]
        pub web_properties:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WebPropertySummary>>>,
    }
    impl AccountSummary {
        pub fn builder() -> AccountSummaryBuilder {
            AccountSummaryBuilder::default()
        }
    }
    mod account_summary_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#accountSummary")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for an Analytics account ticket. The account ticket consists of the ticket ID and the basic information for the account, property and profile."]
    pub struct AccountTicket {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "account")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account for this ticket."]
        pub account: ::std::option::Option<::std::boxed::Box<Account>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account ticket ID used to access the account ticket."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ account_ticket_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "account_ticket_defaults :: kind")]
        #[doc = "Resource type for account ticket."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "View (Profile) for the account."]
        pub profile: ::std::option::Option<::std::boxed::Box<Profile>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "redirectUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Redirect URI where the user will be sent after accepting Terms of Service. Must be configured in APIs console as a callback URL."]
        pub redirect_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webproperty")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web property for the account."]
        pub webproperty: ::std::option::Option<::std::boxed::Box<Webproperty>>,
    }
    impl AccountTicket {
        pub fn builder() -> AccountTicketBuilder {
            AccountTicketBuilder::default()
        }
    }
    mod account_ticket_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#accountTicket")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for an Analytics account tree requests. The account tree request is used in the provisioning api to create an account, property, and view (profile). It contains the basic information required to make these fields."]
    pub struct AccountTreeRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub account_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ account_tree_request_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "account_tree_request_defaults :: kind")]
        #[doc = "Resource type for account ticket."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub profile_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timezone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub timezone: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webpropertyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub webproperty_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "websiteUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub website_url: ::std::option::Option<::std::string::String>,
    }
    impl AccountTreeRequest {
        pub fn builder() -> AccountTreeRequestBuilder {
            AccountTreeRequestBuilder::default()
        }
    }
    mod account_tree_request_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#accountTreeRequest")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for an Analytics account tree response. The account tree response is used in the provisioning api to return the result of creating an account, property, and view (profile)."]
    pub struct AccountTreeResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "account")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The account created."]
        pub account: ::std::option::Option<::std::boxed::Box<Account>>,
        #[builder(
            default = "{ account_tree_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "account_tree_response_defaults :: kind")]
        #[doc = "Resource type for account ticket."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "View (Profile) for the account."]
        pub profile: ::std::option::Option<::std::boxed::Box<Profile>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webproperty")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web property for the account."]
        pub webproperty: ::std::option::Option<::std::boxed::Box<Webproperty>>,
    }
    impl AccountTreeResponse {
        pub fn builder() -> AccountTreeResponseBuilder {
            AccountTreeResponseBuilder::default()
        }
    }
    mod account_tree_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#accountTreeResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An account collection provides a list of Analytics accounts to which a user has access. The account collection is the entry point to all management information. Each resource in the collection corresponds to a single Analytics account."]
    pub struct Accounts {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of accounts."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Account>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemsPerPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of entries the response can contain, regardless of the actual number of entries returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter."]
        pub items_per_page: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ accounts_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "accounts_defaults :: kind")]
        #[doc = "Collection type."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Next link for this account collection."]
        pub next_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previousLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Previous link for this account collection."]
        pub previous_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The starting index of the entries, which is 1 by default or otherwise specified by the start-index query parameter."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of results for the query, regardless of the number of results in the response."]
        pub total_results: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "username")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email ID of the authenticated user"]
        pub username: ::std::option::Option<::std::string::String>,
    }
    impl Accounts {
        pub fn builder() -> AccountsBuilder {
            AccountsBuilder::default()
        }
    }
    mod accounts_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#accounts")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for an Google Ads account."]
    pub struct AdWordsAccount {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoTaggingEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if auto-tagging is enabled on the Google Ads account. Read-only after the insert operation."]
        pub auto_tagging_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Customer ID. This field is required when creating a Google Ads link."]
        pub customer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ad_words_account_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "ad_words_account_defaults :: kind")]
        #[doc = "Resource type for Google Ads account."]
        pub kind: ::std::string::String,
    }
    impl AdWordsAccount {
        pub fn builder() -> AdWordsAccountBuilder {
            AdWordsAccountBuilder::default()
        }
    }
    mod ad_words_account_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#adWordsAccount")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request template for the delete upload data request."]
    pub struct AnalyticsDataimportDeleteUploadDataRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customDataImportUids")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of upload UIDs."]
        pub custom_data_import_uids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl AnalyticsDataimportDeleteUploadDataRequest {
        pub fn builder() -> AnalyticsDataimportDeleteUploadDataRequestBuilder {
            AnalyticsDataimportDeleteUploadDataRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for a metadata column."]
    pub struct Column {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Map of attribute name and value for this column."]
        pub attributes:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Column id."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ column_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "column_defaults :: kind")]
        #[doc = "Resource type for Analytics column."]
        pub kind: ::std::string::String,
    }
    impl Column {
        pub fn builder() -> ColumnBuilder {
            ColumnBuilder::default()
        }
    }
    mod column_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#column")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Lists columns (dimensions and metrics) for a particular report type."]
    pub struct Columns {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributeNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of attributes names returned by columns."]
        pub attribute_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of collection. This etag can be compared with the last response etag to check if response has changed."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of columns for a report type."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Column>>>,
        #[builder(default = "{ columns_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "columns_defaults :: kind")]
        #[doc = "Collection type."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of columns returned in the response."]
        pub total_results: ::std::option::Option<::std::primitive::i64>,
    }
    impl Columns {
        pub fn builder() -> ColumnsBuilder {
            ColumnsBuilder::default()
        }
    }
    mod columns_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#columns")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for an Analytics custom data source."]
    pub struct CustomDataSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account ID to which this custom data source belongs."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "childLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub child_link: ::std::option::Option<CustomDataSourceChildLink>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "created")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time this custom data source was created."]
        pub created: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of custom data source."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom data source ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importBehavior")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub import_behavior: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ custom_data_source_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "custom_data_source_defaults :: kind")]
        #[doc = "Resource type for Analytics custom data source."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of this custom data source."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parent link for this custom data source. Points to the web property to which this custom data source belongs."]
        pub parent_link: ::std::option::Option<CustomDataSourceParentLink>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profilesLinked")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "IDs of views (profiles) linked to the custom data source."]
        pub profiles_linked: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schema")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Collection of schema headers of the custom data source."]
        pub schema: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link for this Analytics custom data source."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of the custom data source."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time this custom data source was last modified."]
        pub updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uploadType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Upload type of the custom data source."]
        pub upload_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web property ID of the form UA-XXXXX-YY to which this custom data source belongs."]
        pub web_property_id: ::std::option::Option<::std::string::String>,
    }
    impl CustomDataSource {
        pub fn builder() -> CustomDataSourceBuilder {
            CustomDataSourceBuilder::default()
        }
    }
    mod custom_data_source_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#customDataSource")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct CustomDataSourceChildLink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "href")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to the list of daily uploads for this custom data source. Link to the list of uploads for this custom data source."]
        pub href: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value is \"analytics#dailyUploads\". Value is \"analytics#uploads\"."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl CustomDataSourceChildLink {
        pub fn builder() -> CustomDataSourceChildLinkBuilder {
            CustomDataSourceChildLinkBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parent link for this custom data source. Points to the web property to which this custom data source belongs."]
    pub struct CustomDataSourceParentLink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "href")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to the web property to which this custom data source belongs."]
        pub href: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ custom_data_source_parent_link_defaults :: _type () }",
            setter(into)
        )]
        #[serde(rename = "type")]
        #[serde(default = "custom_data_source_parent_link_defaults :: _type")]
        #[doc = "Value is \"analytics#webproperty\"."]
        pub _type: ::std::string::String,
    }
    impl CustomDataSourceParentLink {
        pub fn builder() -> CustomDataSourceParentLinkBuilder {
            CustomDataSourceParentLinkBuilder::default()
        }
    }
    mod custom_data_source_parent_link_defaults {
        pub fn _type() -> ::std::string::String {
            String::from("analytics#webproperty")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Lists Analytics custom data sources to which the user has access. Each resource in the collection corresponds to a single Analytics custom data source."]
    pub struct CustomDataSources {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Collection of custom data sources."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomDataSource>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemsPerPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter."]
        pub items_per_page: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ custom_data_sources_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "custom_data_sources_defaults :: kind")]
        #[doc = "Collection type."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to next page for this custom data source collection."]
        pub next_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previousLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to previous page for this custom data source collection."]
        pub previous_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of results for the query, regardless of the number of results in the response."]
        pub total_results: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "username")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email ID of the authenticated user"]
        pub username: ::std::option::Option<::std::string::String>,
    }
    impl CustomDataSources {
        pub fn builder() -> CustomDataSourcesBuilder {
            CustomDataSourcesBuilder::default()
        }
    }
    mod custom_data_sources_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#customDataSources")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for Analytics Custom Dimension."]
    pub struct CustomDimension {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account ID."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "active")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Boolean indicating whether the custom dimension is active."]
        pub active: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "created")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time the custom dimension was created."]
        pub created: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom dimension ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "index")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Index of the custom dimension."]
        pub index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ custom_dimension_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "custom_dimension_defaults :: kind")]
        #[doc = "Kind value for a custom dimension. Set to \"analytics#customDimension\". It is a read-only field."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the custom dimension."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parent link for the custom dimension. Points to the property to which the custom dimension belongs."]
        pub parent_link: ::std::option::Option<CustomDimensionParentLink>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scope")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Scope of the custom dimension: HIT, SESSION, USER or PRODUCT."]
        pub scope: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link for the custom dimension"]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time the custom dimension was last modified."]
        pub updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Property ID."]
        pub web_property_id: ::std::option::Option<::std::string::String>,
    }
    impl CustomDimension {
        pub fn builder() -> CustomDimensionBuilder {
            CustomDimensionBuilder::default()
        }
    }
    mod custom_dimension_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#customDimension")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parent link for the custom dimension. Points to the property to which the custom dimension belongs."]
    pub struct CustomDimensionParentLink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "href")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to the property to which the custom dimension belongs."]
        pub href: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ custom_dimension_parent_link_defaults :: _type () }",
            setter(into)
        )]
        #[serde(rename = "type")]
        #[serde(default = "custom_dimension_parent_link_defaults :: _type")]
        #[doc = "Type of the parent link. Set to \"analytics#webproperty\"."]
        pub _type: ::std::string::String,
    }
    impl CustomDimensionParentLink {
        pub fn builder() -> CustomDimensionParentLinkBuilder {
            CustomDimensionParentLinkBuilder::default()
        }
    }
    mod custom_dimension_parent_link_defaults {
        pub fn _type() -> ::std::string::String {
            String::from("analytics#webproperty")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A custom dimension collection lists Analytics custom dimensions to which the user has access. Each resource in the collection corresponds to a single Analytics custom dimension."]
    pub struct CustomDimensions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Collection of custom dimensions."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomDimension>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemsPerPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter."]
        pub items_per_page: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ custom_dimensions_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "custom_dimensions_defaults :: kind")]
        #[doc = "Collection type."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to next page for this custom dimension collection."]
        pub next_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previousLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to previous page for this custom dimension collection."]
        pub previous_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of results for the query, regardless of the number of results in the response."]
        pub total_results: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "username")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email ID of the authenticated user"]
        pub username: ::std::option::Option<::std::string::String>,
    }
    impl CustomDimensions {
        pub fn builder() -> CustomDimensionsBuilder {
            CustomDimensionsBuilder::default()
        }
    }
    mod custom_dimensions_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#customDimensions")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for Analytics Custom Metric."]
    pub struct CustomMetric {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account ID."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "active")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Boolean indicating whether the custom metric is active."]
        pub active: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "created")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time the custom metric was created."]
        pub created: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom metric ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "index")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Index of the custom metric."]
        pub index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ custom_metric_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "custom_metric_defaults :: kind")]
        #[doc = "Kind value for a custom metric. Set to \"analytics#customMetric\". It is a read-only field."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "max_value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Max value of custom metric."]
        pub max_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "min_value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Min value of custom metric."]
        pub min_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the custom metric."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parent link for the custom metric. Points to the property to which the custom metric belongs."]
        pub parent_link: ::std::option::Option<CustomMetricParentLink>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scope")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Scope of the custom metric: HIT or PRODUCT."]
        pub scope: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link for the custom metric"]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data type of custom metric."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time the custom metric was last modified."]
        pub updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Property ID."]
        pub web_property_id: ::std::option::Option<::std::string::String>,
    }
    impl CustomMetric {
        pub fn builder() -> CustomMetricBuilder {
            CustomMetricBuilder::default()
        }
    }
    mod custom_metric_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#customMetric")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parent link for the custom metric. Points to the property to which the custom metric belongs."]
    pub struct CustomMetricParentLink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "href")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to the property to which the custom metric belongs."]
        pub href: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ custom_metric_parent_link_defaults :: _type () }",
            setter(into)
        )]
        #[serde(rename = "type")]
        #[serde(default = "custom_metric_parent_link_defaults :: _type")]
        #[doc = "Type of the parent link. Set to \"analytics#webproperty\"."]
        pub _type: ::std::string::String,
    }
    impl CustomMetricParentLink {
        pub fn builder() -> CustomMetricParentLinkBuilder {
            CustomMetricParentLinkBuilder::default()
        }
    }
    mod custom_metric_parent_link_defaults {
        pub fn _type() -> ::std::string::String {
            String::from("analytics#webproperty")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A custom metric collection lists Analytics custom metrics to which the user has access. Each resource in the collection corresponds to a single Analytics custom metric."]
    pub struct CustomMetrics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Collection of custom metrics."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomMetric>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemsPerPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter."]
        pub items_per_page: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ custom_metrics_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "custom_metrics_defaults :: kind")]
        #[doc = "Collection type."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to next page for this custom metric collection."]
        pub next_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previousLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to previous page for this custom metric collection."]
        pub previous_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of results for the query, regardless of the number of results in the response."]
        pub total_results: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "username")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email ID of the authenticated user"]
        pub username: ::std::option::Option<::std::string::String>,
    }
    impl CustomMetrics {
        pub fn builder() -> CustomMetricsBuilder {
            CustomMetricsBuilder::default()
        }
    }
    mod custom_metrics_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#customMetrics")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for Analytics Entity Google Ads Link."]
    pub struct EntityAdWordsLink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adWordsAccounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of Google Ads client accounts. These cannot be MCC accounts. This field is required when creating a Google Ads link. It cannot be empty."]
        pub ad_words_accounts:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AdWordsAccount>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web property being linked."]
        pub entity: ::std::option::Option<EntityAdWordsLinkEntity>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entity Google Ads link ID"]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ entity_ad_words_link_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "entity_ad_words_link_defaults :: kind")]
        #[doc = "Resource type for entity Google Ads link."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the link. This field is required when creating a Google Ads link."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "IDs of linked Views (Profiles) represented as strings."]
        pub profile_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL link for this Google Analytics - Google Ads link."]
        pub self_link: ::std::option::Option<::std::string::String>,
    }
    impl EntityAdWordsLink {
        pub fn builder() -> EntityAdWordsLinkBuilder {
            EntityAdWordsLinkBuilder::default()
        }
    }
    mod entity_ad_words_link_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#entityAdWordsLink")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Web property being linked."]
    pub struct EntityAdWordsLinkEntity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webPropertyRef")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub web_property_ref: ::std::option::Option<::std::boxed::Box<WebPropertyRef>>,
    }
    impl EntityAdWordsLinkEntity {
        pub fn builder() -> EntityAdWordsLinkEntityBuilder {
            EntityAdWordsLinkEntityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An entity Google Ads link collection provides a list of GA-Google Ads links Each resource in this collection corresponds to a single link."]
    pub struct EntityAdWordsLinks {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of entity Google Ads links."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EntityAdWordsLink>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemsPerPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of entries the response can contain, regardless of the actual number of entries returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter."]
        pub items_per_page: ::std::option::Option<::std::primitive::i64>,
        #[builder(
            default = "{ entity_ad_words_links_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "entity_ad_words_links_defaults :: kind")]
        #[doc = "Collection type."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Next link for this Google Ads link collection."]
        pub next_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previousLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Previous link for this Google Ads link collection."]
        pub previous_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The starting index of the entries, which is 1 by default or otherwise specified by the start-index query parameter."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of results for the query, regardless of the number of results in the response."]
        pub total_results: ::std::option::Option<::std::primitive::i64>,
    }
    impl EntityAdWordsLinks {
        pub fn builder() -> EntityAdWordsLinksBuilder {
            EntityAdWordsLinksBuilder::default()
        }
    }
    mod entity_ad_words_links_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#entityAdWordsLinks")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for an Analytics Entity-User Link. Returns permissions that a user has for an entity."]
    pub struct EntityUserLink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entity for this link. It can be an account, a web property, or a view (profile)."]
        pub entity: ::std::option::Option<EntityUserLinkEntity>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entity user link ID"]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ entity_user_link_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "entity_user_link_defaults :: kind")]
        #[doc = "Resource type for entity user link."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Permissions the user has for this entity."]
        pub permissions: ::std::option::Option<EntityUserLinkPermissions>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Self link for this resource."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userRef")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User reference."]
        pub user_ref: ::std::option::Option<::std::boxed::Box<UserRef>>,
    }
    impl EntityUserLink {
        pub fn builder() -> EntityUserLinkBuilder {
            EntityUserLinkBuilder::default()
        }
    }
    mod entity_user_link_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#entityUserLink")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Entity for this link. It can be an account, a web property, or a view (profile)."]
    pub struct EntityUserLinkEntity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountRef")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account for this link."]
        pub account_ref: ::std::option::Option<::std::boxed::Box<AccountRef>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileRef")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "View (Profile) for this link."]
        pub profile_ref: ::std::option::Option<::std::boxed::Box<ProfileRef>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webPropertyRef")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web property for this link."]
        pub web_property_ref: ::std::option::Option<::std::boxed::Box<WebPropertyRef>>,
    }
    impl EntityUserLinkEntity {
        pub fn builder() -> EntityUserLinkEntityBuilder {
            EntityUserLinkEntityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Permissions the user has for this entity."]
    pub struct EntityUserLinkPermissions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "effective")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Effective permissions represent all the permissions that a user has for this entity. These include any implied permissions (e.g., EDIT implies VIEW) or inherited permissions from the parent entity. Effective permissions are read-only."]
        pub effective: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "local")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Permissions that a user has been assigned at this very level. Does not include any implied or inherited permissions. Local permissions are modifiable."]
        pub local: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl EntityUserLinkPermissions {
        pub fn builder() -> EntityUserLinkPermissionsBuilder {
            EntityUserLinkPermissionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An entity user link collection provides a list of Analytics ACL links Each resource in this collection corresponds to a single link."]
    pub struct EntityUserLinks {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of entity user links."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EntityUserLink>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemsPerPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of entries the response can contain, regardless of the actual number of entries returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter."]
        pub items_per_page: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ entity_user_links_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "entity_user_links_defaults :: kind")]
        #[doc = "Collection type."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Next link for this account collection."]
        pub next_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previousLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Previous link for this account collection."]
        pub previous_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The starting index of the entries, which is 1 by default or otherwise specified by the start-index query parameter."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of results for the query, regardless of the number of results in the response."]
        pub total_results: ::std::option::Option<::std::primitive::i64>,
    }
    impl EntityUserLinks {
        pub fn builder() -> EntityUserLinksBuilder {
            EntityUserLinksBuilder::default()
        }
    }
    mod entity_user_links_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#entityUserLinks")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for Analytics experiment resource."]
    pub struct Experiment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account ID to which this experiment belongs. This field is read-only."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "created")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time the experiment was created. This field is read-only."]
        pub created: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Notes about this experiment."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "editableInGaUi")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, the end user will be able to edit the experiment via the Google Analytics user interface."]
        pub editable_in_ga_ui: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ending time of the experiment (the time the status changed from RUNNING to ENDED). This field is present only if the experiment has ended. This field is read-only."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "equalWeighting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Boolean specifying whether to distribute traffic evenly across all variations. If the value is False, content experiments follows the default behavior of adjusting traffic dynamically based on variation performance. Optional -- defaults to False. This field may not be changed for an experiment whose status is ENDED."]
        pub equal_weighting: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Experiment ID. Required for patch and update. Disallowed for create."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "internalWebPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Internal ID for the web property to which this experiment belongs. This field is read-only."]
        pub internal_web_property_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ experiment_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "experiment_defaults :: kind")]
        #[doc = "Resource type for an Analytics experiment. This field is read-only."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minimumExperimentLengthInDays")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An integer number in [3, 90]. Specifies the minimum length of the experiment. Can be changed for a running experiment. This field may not be changed for an experiments whose status is ENDED."]
        pub minimum_experiment_length_in_days: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Experiment name. This field may not be changed for an experiment whose status is ENDED. This field is required when creating an experiment."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectiveMetric")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metric that the experiment is optimizing. Valid values: \"ga:goal(n)Completions\", \"ga:adsenseAdsClicks\", \"ga:adsenseAdsViewed\", \"ga:adsenseRevenue\", \"ga:bounces\", \"ga:pageviews\", \"ga:sessionDuration\", \"ga:transactions\", \"ga:transactionRevenue\". This field is required if status is \"RUNNING\" and servingFramework is one of \"REDIRECT\" or \"API\"."]
        pub objective_metric: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "optimizationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the objectiveMetric should be minimized or maximized. Possible values: \"MAXIMUM\", \"MINIMUM\". Optional--defaults to \"MAXIMUM\". Cannot be specified without objectiveMetric. Cannot be modified when status is \"RUNNING\" or \"ENDED\"."]
        pub optimization_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parent link for an experiment. Points to the view (profile) to which this experiment belongs."]
        pub parent_link: ::std::option::Option<ExperimentParentLink>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "View (Profile) ID to which this experiment belongs. This field is read-only."]
        pub profile_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reasonExperimentEnded")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Why the experiment ended. Possible values: \"STOPPED_BY_USER\", \"WINNER_FOUND\", \"EXPERIMENT_EXPIRED\", \"ENDED_WITH_NO_WINNER\", \"GOAL_OBJECTIVE_CHANGED\". \"ENDED_WITH_NO_WINNER\" means that the experiment didn't expire but no winner was projected to be found. If the experiment status is changed via the API to ENDED this field is set to STOPPED_BY_USER. This field is read-only."]
        pub reason_experiment_ended: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rewriteVariationUrlsAsOriginal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Boolean specifying whether variations URLS are rewritten to match those of the original. This field may not be changed for an experiments whose status is ENDED."]
        pub rewrite_variation_urls_as_original: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link for this experiment. This field is read-only."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "servingFramework")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The framework used to serve the experiment variations and evaluate the results. One of:  \n- REDIRECT: Google Analytics redirects traffic to different variation pages, reports the chosen variation and evaluates the results.\n- API: Google Analytics chooses and reports the variation to serve and evaluates the results; the caller is responsible for serving the selected variation.\n- EXTERNAL: The variations will be served externally and the chosen variation reported to Google Analytics. The caller is responsible for serving the selected variation and evaluating the results."]
        pub serving_framework: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The snippet of code to include on the control page(s). This field is read-only."]
        pub snippet: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The starting time of the experiment (the time the status changed from READY_TO_RUN to RUNNING). This field is present only if the experiment has started. This field is read-only."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Experiment status. Possible values: \"DRAFT\", \"READY_TO_RUN\", \"RUNNING\", \"ENDED\". Experiments can be created in the \"DRAFT\", \"READY_TO_RUN\" or \"RUNNING\" state. This field is required when creating an experiment."]
        pub status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trafficCoverage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A floating-point number in (0, 1]. Specifies the fraction of the traffic that participates in the experiment. Can be changed for a running experiment. This field may not be changed for an experiments whose status is ENDED."]
        pub traffic_coverage: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time the experiment was last modified. This field is read-only."]
        pub updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "variations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Array of variations. The first variation in the array is the original. The number of variations may not change once an experiment is in the RUNNING state. At least two variations are required before status can be set to RUNNING."]
        pub variations: ::std::option::Option<::std::vec::Vec<ExperimentVariations>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web property ID to which this experiment belongs. The web property ID is of the form UA-XXXXX-YY. This field is read-only."]
        pub web_property_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "winnerConfidenceLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A floating-point number in (0, 1). Specifies the necessary confidence level to choose a winner. This field may not be changed for an experiments whose status is ENDED."]
        pub winner_confidence_level: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "winnerFound")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Boolean specifying whether a winner has been found for this experiment. This field is read-only."]
        pub winner_found: ::std::option::Option<::std::primitive::bool>,
    }
    impl Experiment {
        pub fn builder() -> ExperimentBuilder {
            ExperimentBuilder::default()
        }
    }
    mod experiment_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#experiment")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parent link for an experiment. Points to the view (profile) to which this experiment belongs."]
    pub struct ExperimentParentLink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "href")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to the view (profile) to which this experiment belongs. This field is read-only."]
        pub href: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ experiment_parent_link_defaults :: _type () }",
            setter(into)
        )]
        #[serde(rename = "type")]
        #[serde(default = "experiment_parent_link_defaults :: _type")]
        #[doc = "Value is \"analytics#profile\". This field is read-only."]
        pub _type: ::std::string::String,
    }
    impl ExperimentParentLink {
        pub fn builder() -> ExperimentParentLinkBuilder {
            ExperimentParentLinkBuilder::default()
        }
    }
    mod experiment_parent_link_defaults {
        pub fn _type() -> ::std::string::String {
            String::from("analytics#profile")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ExperimentVariations {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the variation. This field is required when creating an experiment. This field may not be changed for an experiment whose status is ENDED."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of the variation. Possible values: \"ACTIVE\", \"INACTIVE\". INACTIVE variations are not served. This field may not be changed for an experiment whose status is ENDED."]
        pub status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the variation. This field may not be changed for an experiment whose status is RUNNING or ENDED."]
        pub url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "weight")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Weight that this variation should receive. Only present if the experiment is running. This field is read-only."]
        pub weight: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "won")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the experiment has ended and this variation performed (statistically) significantly better than the original. This field is read-only."]
        pub won: ::std::option::Option<::std::primitive::bool>,
    }
    impl ExperimentVariations {
        pub fn builder() -> ExperimentVariationsBuilder {
            ExperimentVariationsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An experiment collection lists Analytics experiments to which the user has access. Each view (profile) can have a set of experiments. Each resource in the Experiment collection corresponds to a single Analytics experiment."]
    pub struct Experiments {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of experiments."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Experiment>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemsPerPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter."]
        pub items_per_page: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ experiments_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "experiments_defaults :: kind")]
        #[doc = "Collection type."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to next page for this experiment collection."]
        pub next_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previousLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to previous page for this experiment collection."]
        pub previous_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of results for the query, regardless of the number of resources in the result."]
        pub total_results: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "username")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email ID of the authenticated user"]
        pub username: ::std::option::Option<::std::string::String>,
    }
    impl Experiments {
        pub fn builder() -> ExperimentsBuilder {
            ExperimentsBuilder::default()
        }
    }
    mod experiments_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#experiments")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for an Analytics account filter."]
    pub struct Filter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account ID to which this filter belongs."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "advancedDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details for the filter of the type ADVANCED."]
        pub advanced_details: ::std::option::Option<FilterAdvancedDetails>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "created")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time this filter was created."]
        pub created: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludeDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details for the filter of the type EXCLUDE."]
        pub exclude_details: ::std::option::Option<::std::boxed::Box<FilterExpression>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Filter ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details for the filter of the type INCLUDE."]
        pub include_details: ::std::option::Option<::std::boxed::Box<FilterExpression>>,
        #[builder(default = "{ filter_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "filter_defaults :: kind")]
        #[doc = "Resource type for Analytics filter."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lowercaseDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details for the filter of the type LOWER."]
        pub lowercase_details: ::std::option::Option<FilterLowercaseDetails>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of this filter."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parent link for this filter. Points to the account to which this filter belongs."]
        pub parent_link: ::std::option::Option<FilterParentLink>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "searchAndReplaceDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details for the filter of the type SEARCH_AND_REPLACE."]
        pub search_and_replace_details: ::std::option::Option<FilterSearchAndReplaceDetails>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link for this filter."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of this filter. Possible values are INCLUDE, EXCLUDE, LOWERCASE, UPPERCASE, SEARCH_AND_REPLACE and ADVANCED."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time this filter was last modified."]
        pub updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uppercaseDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details for the filter of the type UPPER."]
        pub uppercase_details: ::std::option::Option<FilterUppercaseDetails>,
    }
    impl Filter {
        pub fn builder() -> FilterBuilder {
            FilterBuilder::default()
        }
    }
    mod filter_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#filter")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details for the filter of the type ADVANCED."]
    pub struct FilterAdvancedDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "caseSensitive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates if the filter expressions are case sensitive."]
        pub case_sensitive: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "extractA")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Expression to extract from field A."]
        pub extract_a: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "extractB")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Expression to extract from field B."]
        pub extract_b: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldA")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Field A."]
        pub field_a: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldAIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Index of the custom dimension. Required if field is a CUSTOM_DIMENSION."]
        pub field_a_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldARequired")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates if field A is required to match."]
        pub field_a_required: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldB")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Field B."]
        pub field_b: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldBIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Index of the custom dimension. Required if field is a CUSTOM_DIMENSION."]
        pub field_b_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldBRequired")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates if field B is required to match."]
        pub field_b_required: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputConstructor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Expression used to construct the output value."]
        pub output_constructor: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputToField")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output field."]
        pub output_to_field: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputToFieldIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Index of the custom dimension. Required if field is a CUSTOM_DIMENSION."]
        pub output_to_field_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "overrideOutputField")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates if the existing value of the output field, if any, should be overridden by the output expression."]
        pub override_output_field: ::std::option::Option<::std::primitive::bool>,
    }
    impl FilterAdvancedDetails {
        pub fn builder() -> FilterAdvancedDetailsBuilder {
            FilterAdvancedDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details for the filter of the type LOWER."]
    pub struct FilterLowercaseDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "field")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Field to use in the filter."]
        pub field: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Index of the custom dimension. Required if field is a CUSTOM_DIMENSION."]
        pub field_index: ::std::option::Option<::std::primitive::i64>,
    }
    impl FilterLowercaseDetails {
        pub fn builder() -> FilterLowercaseDetailsBuilder {
            FilterLowercaseDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parent link for this filter. Points to the account to which this filter belongs."]
    pub struct FilterParentLink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "href")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to the account to which this filter belongs."]
        pub href: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ filter_parent_link_defaults :: _type () }", setter(into))]
        #[serde(rename = "type")]
        #[serde(default = "filter_parent_link_defaults :: _type")]
        #[doc = "Value is \"analytics#account\"."]
        pub _type: ::std::string::String,
    }
    impl FilterParentLink {
        pub fn builder() -> FilterParentLinkBuilder {
            FilterParentLinkBuilder::default()
        }
    }
    mod filter_parent_link_defaults {
        pub fn _type() -> ::std::string::String {
            String::from("analytics#account")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details for the filter of the type SEARCH_AND_REPLACE."]
    pub struct FilterSearchAndReplaceDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "caseSensitive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines if the filter is case sensitive."]
        pub case_sensitive: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "field")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Field to use in the filter."]
        pub field: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Index of the custom dimension. Required if field is a CUSTOM_DIMENSION."]
        pub field_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replaceString")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Term to replace the search term with."]
        pub replace_string: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "searchString")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Term to search."]
        pub search_string: ::std::option::Option<::std::string::String>,
    }
    impl FilterSearchAndReplaceDetails {
        pub fn builder() -> FilterSearchAndReplaceDetailsBuilder {
            FilterSearchAndReplaceDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details for the filter of the type UPPER."]
    pub struct FilterUppercaseDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "field")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Field to use in the filter."]
        pub field: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Index of the custom dimension. Required if field is a CUSTOM_DIMENSION."]
        pub field_index: ::std::option::Option<::std::primitive::i64>,
    }
    impl FilterUppercaseDetails {
        pub fn builder() -> FilterUppercaseDetailsBuilder {
            FilterUppercaseDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for an Analytics filter expression."]
    pub struct FilterExpression {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "caseSensitive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines if the filter is case sensitive."]
        pub case_sensitive: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expressionValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Filter expression value"]
        pub expression_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "field")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Field to filter. Possible values:  \n- Content and Traffic  \n- PAGE_REQUEST_URI, \n- PAGE_HOSTNAME, \n- PAGE_TITLE, \n- REFERRAL, \n- COST_DATA_URI (Campaign target URL), \n- HIT_TYPE, \n- INTERNAL_SEARCH_TERM, \n- INTERNAL_SEARCH_TYPE, \n- SOURCE_PROPERTY_TRACKING_ID,   \n- Campaign or AdGroup  \n- CAMPAIGN_SOURCE, \n- CAMPAIGN_MEDIUM, \n- CAMPAIGN_NAME, \n- CAMPAIGN_AD_GROUP, \n- CAMPAIGN_TERM, \n- CAMPAIGN_CONTENT, \n- CAMPAIGN_CODE, \n- CAMPAIGN_REFERRAL_PATH,   \n- E-Commerce  \n- TRANSACTION_COUNTRY, \n- TRANSACTION_REGION, \n- TRANSACTION_CITY, \n- TRANSACTION_AFFILIATION (Store or order location), \n- ITEM_NAME, \n- ITEM_CODE, \n- ITEM_VARIATION, \n- TRANSACTION_ID, \n- TRANSACTION_CURRENCY_CODE, \n- PRODUCT_ACTION_TYPE,   \n- Audience/Users  \n- BROWSER, \n- BROWSER_VERSION, \n- BROWSER_SIZE, \n- PLATFORM, \n- PLATFORM_VERSION, \n- LANGUAGE, \n- SCREEN_RESOLUTION, \n- SCREEN_COLORS, \n- JAVA_ENABLED (Boolean Field), \n- FLASH_VERSION, \n- GEO_SPEED (Connection speed), \n- VISITOR_TYPE, \n- GEO_ORGANIZATION (ISP organization), \n- GEO_DOMAIN, \n- GEO_IP_ADDRESS, \n- GEO_IP_VERSION,   \n- Location  \n- GEO_COUNTRY, \n- GEO_REGION, \n- GEO_CITY,   \n- Event  \n- EVENT_CATEGORY, \n- EVENT_ACTION, \n- EVENT_LABEL,   \n- Other  \n- CUSTOM_FIELD_1, \n- CUSTOM_FIELD_2, \n- USER_DEFINED_VALUE,   \n- Application  \n- APP_ID, \n- APP_INSTALLER_ID, \n- APP_NAME, \n- APP_VERSION, \n- SCREEN, \n- IS_APP (Boolean Field), \n- IS_FATAL_EXCEPTION (Boolean Field), \n- EXCEPTION_DESCRIPTION,   \n- Mobile device  \n- IS_MOBILE (Boolean Field, Deprecated. Use DEVICE_CATEGORY=mobile), \n- IS_TABLET (Boolean Field, Deprecated. Use DEVICE_CATEGORY=tablet), \n- DEVICE_CATEGORY, \n- MOBILE_HAS_QWERTY_KEYBOARD (Boolean Field), \n- MOBILE_HAS_NFC_SUPPORT (Boolean Field), \n- MOBILE_HAS_CELLULAR_RADIO (Boolean Field), \n- MOBILE_HAS_WIFI_SUPPORT (Boolean Field), \n- MOBILE_BRAND_NAME, \n- MOBILE_MODEL_NAME, \n- MOBILE_MARKETING_NAME, \n- MOBILE_POINTING_METHOD,   \n- Social  \n- SOCIAL_NETWORK, \n- SOCIAL_ACTION, \n- SOCIAL_ACTION_TARGET,   \n- Custom dimension  \n- CUSTOM_DIMENSION (See accompanying field index),"]
        pub field: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Index of the custom dimension. Set only if the field is a is CUSTOM_DIMENSION."]
        pub field_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ filter_expression_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "filter_expression_defaults :: kind")]
        #[doc = "Kind value for filter expression"]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Match type for this filter. Possible values are BEGINS_WITH, EQUAL, ENDS_WITH, CONTAINS, or MATCHES. GEO_DOMAIN, GEO_IP_ADDRESS, PAGE_REQUEST_URI, or PAGE_HOSTNAME filters can use any match type; all other filters must use MATCHES."]
        pub match_type: ::std::option::Option<::std::string::String>,
    }
    impl FilterExpression {
        pub fn builder() -> FilterExpressionBuilder {
            FilterExpressionBuilder::default()
        }
    }
    mod filter_expression_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#filterExpression")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for a profile filter link."]
    pub struct FilterRef {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account ID to which this filter belongs."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "href")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link for this filter."]
        pub href: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Filter ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ filter_ref_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "filter_ref_defaults :: kind")]
        #[doc = "Kind value for filter reference."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of this filter."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl FilterRef {
        pub fn builder() -> FilterRefBuilder {
            FilterRefBuilder::default()
        }
    }
    mod filter_ref_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#filterRef")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A filter collection lists filters created by users in an Analytics account. Each resource in the collection corresponds to a filter."]
    pub struct Filters {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of filters."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Filter>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemsPerPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1,000 with a value of 1000 by default, or otherwise specified by the max-results query parameter."]
        pub items_per_page: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ filters_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "filters_defaults :: kind")]
        #[doc = "Collection type."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to next page for this filter collection."]
        pub next_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previousLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to previous page for this filter collection."]
        pub previous_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of results for the query, regardless of the number of results in the response."]
        pub total_results: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "username")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email ID of the authenticated user"]
        pub username: ::std::option::Option<::std::string::String>,
    }
    impl Filters {
        pub fn builder() -> FiltersBuilder {
            FiltersBuilder::default()
        }
    }
    mod filters_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#filters")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Analytics data for a given view (profile)."]
    pub struct GaData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnHeaders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Column headers that list dimension names followed by the metric names. The order of dimensions and metrics is same as specified in the request."]
        pub column_headers: ::std::option::Option<::std::vec::Vec<GaDataColumnHeaders>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containsSampledData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines if Analytics data contains samples."]
        pub contains_sampled_data: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataLastRefreshed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last refreshed time in seconds for Analytics data."]
        pub data_last_refreshed: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataTable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub data_table: ::std::option::Option<GaDataDataTable>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique ID for this data response."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemsPerPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of rows the response can contain, regardless of the actual number of rows returned. Its value ranges from 1 to 10,000 with a value of 1000 by default, or otherwise specified by the max-results query parameter."]
        pub items_per_page: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ga_data_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "ga_data_defaults :: kind")]
        #[doc = "Resource type."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to next page for this Analytics data query."]
        pub next_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previousLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to previous page for this Analytics data query."]
        pub previous_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information for the view (profile), for which the Analytics data was requested."]
        pub profile_info: ::std::option::Option<GaDataProfileInfo>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "query")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Analytics data request query parameters."]
        pub query: ::std::option::Option<GaDataQuery>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Analytics data rows, where each row contains a list of dimension values followed by the metric values. The order of dimensions and metrics is same as specified in the request."]
        pub rows: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sampleSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of samples used to calculate the result."]
        pub sample_size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sampleSpace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total size of the sample space from which the samples were selected."]
        pub sample_space: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to this page."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of rows for the query, regardless of the number of rows in the response."]
        pub total_results: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalsForAllResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total values for the requested metrics over all the results, not just the results returned in this response. The order of the metric totals is same as the metric order specified in the request."]
        pub totals_for_all_results:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl GaData {
        pub fn builder() -> GaDataBuilder {
            GaDataBuilder::default()
        }
    }
    mod ga_data_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#gaData")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GaDataColumnHeaders {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Column Type. Either DIMENSION or METRIC."]
        pub column_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data type. Dimension column headers have only STRING as the data type. Metric column headers have data types for metric values such as INTEGER, DOUBLE, CURRENCY etc."]
        pub data_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Column name."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GaDataColumnHeaders {
        pub fn builder() -> GaDataColumnHeadersBuilder {
            GaDataColumnHeadersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GaDataDataTable {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cols")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub cols: ::std::option::Option<::std::vec::Vec<GaDataDataTableCols>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub rows: ::std::option::Option<::std::vec::Vec<GaDataDataTableRows>>,
    }
    impl GaDataDataTable {
        pub fn builder() -> GaDataDataTableBuilder {
            GaDataDataTableBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GaDataDataTableCols {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "label")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl GaDataDataTableCols {
        pub fn builder() -> GaDataDataTableColsBuilder {
            GaDataDataTableColsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GaDataDataTableRows {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "c")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub c: ::std::option::Option<::std::vec::Vec<GaDataDataTableRowsC>>,
    }
    impl GaDataDataTableRows {
        pub fn builder() -> GaDataDataTableRowsBuilder {
            GaDataDataTableRowsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GaDataDataTableRowsC {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "v")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub v: ::std::option::Option<::std::string::String>,
    }
    impl GaDataDataTableRowsC {
        pub fn builder() -> GaDataDataTableRowsCBuilder {
            GaDataDataTableRowsCBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information for the view (profile), for which the Analytics data was requested."]
    pub struct GaDataProfileInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account ID to which this view (profile) belongs."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "internalWebPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Internal ID for the web property to which this view (profile) belongs."]
        pub internal_web_property_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "View (Profile) ID."]
        pub profile_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "View (Profile) name."]
        pub profile_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Table ID for view (profile)."]
        pub table_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web Property ID to which this view (profile) belongs."]
        pub web_property_id: ::std::option::Option<::std::string::String>,
    }
    impl GaDataProfileInfo {
        pub fn builder() -> GaDataProfileInfoBuilder {
            GaDataProfileInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Analytics data request query parameters."]
    pub struct GaDataQuery {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of analytics dimensions."]
        pub dimensions: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "end-date")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "End date."]
        pub end_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Comma-separated list of dimension or metric filters."]
        pub filters: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ids")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique table ID."]
        pub ids: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "max-results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum results per page."]
        pub max_results: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of analytics metrics."]
        pub metrics: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "samplingLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Desired sampling level"]
        pub sampling_level: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Analytics advanced segment."]
        pub segment: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sort")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of dimensions or metrics based on which Analytics data is sorted."]
        pub sort: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "start-date")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Start date."]
        pub start_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "start-index")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Start index."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
    }
    impl GaDataQuery {
        pub fn builder() -> GaDataQueryBuilder {
            GaDataQueryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for Analytics goal resource."]
    pub struct Goal {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account ID to which this goal belongs."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "active")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines whether this goal is active."]
        pub active: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "created")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time this goal was created."]
        pub created: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details for the goal of the type EVENT."]
        pub event_details: ::std::option::Option<GoalEventDetails>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Goal ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "internalWebPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Internal ID for the web property to which this goal belongs."]
        pub internal_web_property_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ goal_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "goal_defaults :: kind")]
        #[doc = "Resource type for an Analytics goal."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Goal name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parent link for a goal. Points to the view (profile) to which this goal belongs."]
        pub parent_link: ::std::option::Option<GoalParentLink>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "View (Profile) ID to which this goal belongs."]
        pub profile_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link for this goal."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Goal type. Possible values are URL_DESTINATION, VISIT_TIME_ON_SITE, VISIT_NUM_PAGES, AND EVENT."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time this goal was last modified."]
        pub updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "urlDestinationDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details for the goal of the type URL_DESTINATION."]
        pub url_destination_details: ::std::option::Option<GoalUrlDestinationDetails>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Goal value."]
        pub value: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitNumPagesDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details for the goal of the type VISIT_NUM_PAGES."]
        pub visit_num_pages_details: ::std::option::Option<GoalVisitNumPagesDetails>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitTimeOnSiteDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details for the goal of the type VISIT_TIME_ON_SITE."]
        pub visit_time_on_site_details: ::std::option::Option<GoalVisitTimeOnSiteDetails>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web property ID to which this goal belongs. The web property ID is of the form UA-XXXXX-YY."]
        pub web_property_id: ::std::option::Option<::std::string::String>,
    }
    impl Goal {
        pub fn builder() -> GoalBuilder {
            GoalBuilder::default()
        }
    }
    mod goal_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#goal")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details for the goal of the type EVENT."]
    pub struct GoalEventDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventConditions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of event conditions."]
        pub event_conditions:
            ::std::option::Option<::std::vec::Vec<GoalEventDetailsEventConditions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "useEventValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines if the event value should be used as the value for this goal."]
        pub use_event_value: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoalEventDetails {
        pub fn builder() -> GoalEventDetailsBuilder {
            GoalEventDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoalEventDetailsEventConditions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "comparisonType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of comparison. Possible values are LESS_THAN, GREATER_THAN or EQUAL."]
        pub comparison_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "comparisonValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value used for this comparison."]
        pub comparison_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expression")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Expression used for this match."]
        pub expression: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of the match to be performed. Possible values are REGEXP, BEGINS_WITH, or EXACT."]
        pub match_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of this event condition. Possible values are CATEGORY, ACTION, LABEL, or VALUE."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl GoalEventDetailsEventConditions {
        pub fn builder() -> GoalEventDetailsEventConditionsBuilder {
            GoalEventDetailsEventConditionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parent link for a goal. Points to the view (profile) to which this goal belongs."]
    pub struct GoalParentLink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "href")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to the view (profile) to which this goal belongs."]
        pub href: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ goal_parent_link_defaults :: _type () }", setter(into))]
        #[serde(rename = "type")]
        #[serde(default = "goal_parent_link_defaults :: _type")]
        #[doc = "Value is \"analytics#profile\"."]
        pub _type: ::std::string::String,
    }
    impl GoalParentLink {
        pub fn builder() -> GoalParentLinkBuilder {
            GoalParentLinkBuilder::default()
        }
    }
    mod goal_parent_link_defaults {
        pub fn _type() -> ::std::string::String {
            String::from("analytics#profile")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details for the goal of the type URL_DESTINATION."]
    pub struct GoalUrlDestinationDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "caseSensitive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines if the goal URL must exactly match the capitalization of visited URLs."]
        pub case_sensitive: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firstStepRequired")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines if the first step in this goal is required."]
        pub first_step_required: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Match type for the goal URL. Possible values are HEAD, EXACT, or REGEX."]
        pub match_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "steps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of steps configured for this goal funnel."]
        pub steps: ::std::option::Option<::std::vec::Vec<GoalUrlDestinationDetailsSteps>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL for this goal."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl GoalUrlDestinationDetails {
        pub fn builder() -> GoalUrlDestinationDetailsBuilder {
            GoalUrlDestinationDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoalUrlDestinationDetailsSteps {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Step name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "number")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Step number."]
        pub number: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL for this step."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl GoalUrlDestinationDetailsSteps {
        pub fn builder() -> GoalUrlDestinationDetailsStepsBuilder {
            GoalUrlDestinationDetailsStepsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details for the goal of the type VISIT_NUM_PAGES."]
    pub struct GoalVisitNumPagesDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "comparisonType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of comparison. Possible values are LESS_THAN, GREATER_THAN, or EQUAL."]
        pub comparison_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "comparisonValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value used for this comparison."]
        pub comparison_value: ::std::option::Option<::std::string::String>,
    }
    impl GoalVisitNumPagesDetails {
        pub fn builder() -> GoalVisitNumPagesDetailsBuilder {
            GoalVisitNumPagesDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details for the goal of the type VISIT_TIME_ON_SITE."]
    pub struct GoalVisitTimeOnSiteDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "comparisonType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of comparison. Possible values are LESS_THAN or GREATER_THAN."]
        pub comparison_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "comparisonValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value used for this comparison."]
        pub comparison_value: ::std::option::Option<::std::string::String>,
    }
    impl GoalVisitTimeOnSiteDetails {
        pub fn builder() -> GoalVisitTimeOnSiteDetailsBuilder {
            GoalVisitTimeOnSiteDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A goal collection lists Analytics goals to which the user has access. Each view (profile) can have a set of goals. Each resource in the Goal collection corresponds to a single Analytics goal."]
    pub struct Goals {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of goals."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Goal>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemsPerPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter."]
        pub items_per_page: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ goals_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "goals_defaults :: kind")]
        #[doc = "Collection type."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to next page for this goal collection."]
        pub next_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previousLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to previous page for this goal collection."]
        pub previous_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of results for the query, regardless of the number of resources in the result."]
        pub total_results: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "username")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email ID of the authenticated user"]
        pub username: ::std::option::Option<::std::string::String>,
    }
    impl Goals {
        pub fn builder() -> GoalsBuilder {
            GoalsBuilder::default()
        }
    }
    mod goals_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#goals")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for a hash Client Id request resource."]
    pub struct HashClientIdRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub client_id: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ hash_client_id_request_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "hash_client_id_request_defaults :: kind")]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub web_property_id: ::std::option::Option<::std::string::String>,
    }
    impl HashClientIdRequest {
        pub fn builder() -> HashClientIdRequestBuilder {
            HashClientIdRequestBuilder::default()
        }
    }
    mod hash_client_id_request_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#hashClientIdRequest")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for a hash Client Id response resource."]
    pub struct HashClientIdResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub client_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hashedClientId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub hashed_client_id: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ hash_client_id_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "hash_client_id_response_defaults :: kind")]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub web_property_id: ::std::option::Option<::std::string::String>,
    }
    impl HashClientIdResponse {
        pub fn builder() -> HashClientIdResponseBuilder {
            HashClientIdResponseBuilder::default()
        }
    }
    mod hash_client_id_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#hashClientIdResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for an Analytics Remarketing Include Conditions."]
    pub struct IncludeConditions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "daysToLookBack")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The look-back window lets you specify a time frame for evaluating the behavior that qualifies users for your audience. For example, if your filters include users from Central Asia, and Transactions Greater than 2, and you set the look-back window to 14 days, then any user from Central Asia whose cumulative transactions exceed 2 during the last 14 days is added to the audience."]
        pub days_to_look_back: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isSmartList")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Boolean indicating whether this segment is a smart list. https://support.google.com/analytics/answer/4628577"]
        pub is_smart_list: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ include_conditions_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "include_conditions_defaults :: kind")]
        #[doc = "Resource type for include conditions."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "membershipDurationDays")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of days (in the range 1 to 540) a user remains in the audience."]
        pub membership_duration_days: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The segment condition that will cause a user to be added to an audience."]
        pub segment: ::std::option::Option<::std::string::String>,
    }
    impl IncludeConditions {
        pub fn builder() -> IncludeConditionsBuilder {
            IncludeConditionsBuilder::default()
        }
    }
    mod include_conditions_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#includeConditions")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for an Analytics Remarketing Audience Foreign Link."]
    pub struct LinkedForeignAccount {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account ID to which this linked foreign account belongs."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eligibleForSearch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Boolean indicating whether this is eligible for search."]
        pub eligible_for_search: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entity ad account link ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "internalWebPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Internal ID for the web property to which this linked foreign account belongs."]
        pub internal_web_property_id: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ linked_foreign_account_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "linked_foreign_account_defaults :: kind")]
        #[doc = "Resource type for linked foreign account."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linkedAccountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The foreign account ID. For example the an Google Ads `linkedAccountId` has the following format XXX-XXX-XXXX."]
        pub linked_account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "remarketingAudienceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Remarketing audience ID to which this linked foreign account belongs."]
        pub remarketing_audience_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of this foreign account link."]
        pub status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the foreign account. For example, `ADWORDS_LINKS`, `DBM_LINKS`, `MCC_LINKS` or `OPTIMIZE`."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web property ID of the form UA-XXXXX-YY to which this linked foreign account belongs."]
        pub web_property_id: ::std::option::Option<::std::string::String>,
    }
    impl LinkedForeignAccount {
        pub fn builder() -> LinkedForeignAccountBuilder {
            LinkedForeignAccountBuilder::default()
        }
    }
    mod linked_foreign_account_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#linkedForeignAccount")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Multi-Channel Funnels data for a given view (profile)."]
    pub struct McfData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnHeaders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Column headers that list dimension names followed by the metric names. The order of dimensions and metrics is same as specified in the request."]
        pub column_headers: ::std::option::Option<::std::vec::Vec<McfDataColumnHeaders>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containsSampledData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines if the Analytics data contains sampled data."]
        pub contains_sampled_data: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique ID for this data response."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemsPerPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of rows the response can contain, regardless of the actual number of rows returned. Its value ranges from 1 to 10,000 with a value of 1000 by default, or otherwise specified by the max-results query parameter."]
        pub items_per_page: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ mcf_data_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "mcf_data_defaults :: kind")]
        #[doc = "Resource type."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to next page for this Analytics data query."]
        pub next_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previousLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to previous page for this Analytics data query."]
        pub previous_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information for the view (profile), for which the Analytics data was requested."]
        pub profile_info: ::std::option::Option<McfDataProfileInfo>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "query")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Analytics data request query parameters."]
        pub query: ::std::option::Option<McfDataQuery>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Analytics data rows, where each row contains a list of dimension values followed by the metric values. The order of dimensions and metrics is same as specified in the request."]
        pub rows: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<McfDataRows>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sampleSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of samples used to calculate the result."]
        pub sample_size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sampleSpace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total size of the sample space from which the samples were selected."]
        pub sample_space: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to this page."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of rows for the query, regardless of the number of rows in the response."]
        pub total_results: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalsForAllResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total values for the requested metrics over all the results, not just the results returned in this response. The order of the metric totals is same as the metric order specified in the request."]
        pub totals_for_all_results:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl McfData {
        pub fn builder() -> McfDataBuilder {
            McfDataBuilder::default()
        }
    }
    mod mcf_data_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#mcfData")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct McfDataColumnHeaders {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Column Type. Either DIMENSION or METRIC."]
        pub column_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data type. Dimension and metric values data types such as INTEGER, DOUBLE, CURRENCY, MCF_SEQUENCE etc."]
        pub data_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Column name."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl McfDataColumnHeaders {
        pub fn builder() -> McfDataColumnHeadersBuilder {
            McfDataColumnHeadersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information for the view (profile), for which the Analytics data was requested."]
    pub struct McfDataProfileInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account ID to which this view (profile) belongs."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "internalWebPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Internal ID for the web property to which this view (profile) belongs."]
        pub internal_web_property_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "View (Profile) ID."]
        pub profile_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "View (Profile) name."]
        pub profile_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Table ID for view (profile)."]
        pub table_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web Property ID to which this view (profile) belongs."]
        pub web_property_id: ::std::option::Option<::std::string::String>,
    }
    impl McfDataProfileInfo {
        pub fn builder() -> McfDataProfileInfoBuilder {
            McfDataProfileInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Analytics data request query parameters."]
    pub struct McfDataQuery {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of analytics dimensions."]
        pub dimensions: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "end-date")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "End date."]
        pub end_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Comma-separated list of dimension or metric filters."]
        pub filters: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ids")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique table ID."]
        pub ids: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "max-results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum results per page."]
        pub max_results: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of analytics metrics."]
        pub metrics: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "samplingLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Desired sampling level"]
        pub sampling_level: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Analytics advanced segment."]
        pub segment: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sort")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of dimensions or metrics based on which Analytics data is sorted."]
        pub sort: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "start-date")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Start date."]
        pub start_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "start-index")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Start index."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
    }
    impl McfDataQuery {
        pub fn builder() -> McfDataQueryBuilder {
            McfDataQueryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A union object representing a dimension or metric value. Only one of \"primitiveValue\" or \"conversionPathValue\" attribute will be populated."]
    pub struct McfDataRows {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conversionPathValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A conversion path dimension value, containing a list of interactions with their attributes."]
        pub conversion_path_value:
            ::std::option::Option<::std::vec::Vec<McfDataRowsConversionPathValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primitiveValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A primitive dimension value. A primitive metric value."]
        pub primitive_value: ::std::option::Option<::std::string::String>,
    }
    impl McfDataRows {
        pub fn builder() -> McfDataRowsBuilder {
            McfDataRowsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct McfDataRowsConversionPathValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "interactionType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of an interaction on conversion path. Such as CLICK, IMPRESSION etc."]
        pub interaction_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nodeValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Node value of an interaction on conversion path. Such as source, medium etc."]
        pub node_value: ::std::option::Option<::std::string::String>,
    }
    impl McfDataRowsConversionPathValue {
        pub fn builder() -> McfDataRowsConversionPathValueBuilder {
            McfDataRowsConversionPathValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for an Analytics view (profile)."]
    pub struct Profile {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account ID to which this view (profile) belongs."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "botFilteringEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether bot filtering is enabled for this view (profile)."]
        pub bot_filtering_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "childLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Child link for this view (profile). Points to the list of goals for this view (profile)."]
        pub child_link: ::std::option::Option<ProfileChildLink>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "created")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time this view (profile) was created."]
        pub created: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currency")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The currency type associated with this view (profile), defaults to USD. The supported values are:\nUSD, JPY, EUR, GBP, AUD, KRW, BRL, CNY, DKK, RUB, SEK, NOK, PLN, TRY, TWD, HKD, THB, IDR, ARS, MXN, VND, PHP, INR, CHF, CAD, CZK, NZD, HUF, BGN, LTL, ZAR, UAH, AED, BOB, CLP, COP, EGP, HRK, ILS, MAD, MYR, PEN, PKR, RON, RSD, SAR, SGD, VEF, LVL"]
        pub currency: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Default page for this view (profile)."]
        pub default_page: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eCommerceTracking")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether ecommerce tracking is enabled for this view (profile)."]
        pub e_commerce_tracking: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enhancedECommerceTracking")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether enhanced ecommerce tracking is enabled for this view (profile). This property can only be enabled if ecommerce tracking is enabled."]
        pub enhanced_e_commerce_tracking: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludeQueryParameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The query parameters that are excluded from this view (profile)."]
        pub exclude_query_parameters: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "View (Profile) ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "internalWebPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Internal ID for the web property to which this view (profile) belongs."]
        pub internal_web_property_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ profile_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "profile_defaults :: kind")]
        #[doc = "Resource type for Analytics view (profile)."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of this view (profile)."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parent link for this view (profile). Points to the web property to which this view (profile) belongs."]
        pub parent_link: ::std::option::Option<ProfileParentLink>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Permissions the user has for this view (profile)."]
        pub permissions: ::std::option::Option<ProfilePermissions>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link for this view (profile)."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "siteSearchCategoryParameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Site search category parameters for this view (profile)."]
        pub site_search_category_parameters: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "siteSearchQueryParameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The site search query parameters for this view (profile)."]
        pub site_search_query_parameters: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "starred")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether this view (profile) is starred or not."]
        pub starred: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stripSiteSearchCategoryParameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not Analytics will strip search category parameters from the URLs in your reports."]
        pub strip_site_search_category_parameters: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stripSiteSearchQueryParameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not Analytics will strip search query parameters from the URLs in your reports."]
        pub strip_site_search_query_parameters: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timezone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time zone for which this view (profile) has been configured. Time zones are identified by strings from the TZ database."]
        pub timezone: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "View (Profile) type. Supported types: WEB or APP."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time this view (profile) was last modified."]
        pub updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web property ID of the form UA-XXXXX-YY to which this view (profile) belongs."]
        pub web_property_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "websiteUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Website URL for this view (profile)."]
        pub website_url: ::std::option::Option<::std::string::String>,
    }
    impl Profile {
        pub fn builder() -> ProfileBuilder {
            ProfileBuilder::default()
        }
    }
    mod profile_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#profile")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Child link for this view (profile). Points to the list of goals for this view (profile)."]
    pub struct ProfileChildLink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "href")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to the list of goals for this view (profile)."]
        pub href: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ profile_child_link_defaults :: _type () }", setter(into))]
        #[serde(rename = "type")]
        #[serde(default = "profile_child_link_defaults :: _type")]
        #[doc = "Value is \"analytics#goals\"."]
        pub _type: ::std::string::String,
    }
    impl ProfileChildLink {
        pub fn builder() -> ProfileChildLinkBuilder {
            ProfileChildLinkBuilder::default()
        }
    }
    mod profile_child_link_defaults {
        pub fn _type() -> ::std::string::String {
            String::from("analytics#goals")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parent link for this view (profile). Points to the web property to which this view (profile) belongs."]
    pub struct ProfileParentLink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "href")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to the web property to which this view (profile) belongs."]
        pub href: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ profile_parent_link_defaults :: _type () }", setter(into))]
        #[serde(rename = "type")]
        #[serde(default = "profile_parent_link_defaults :: _type")]
        #[doc = "Value is \"analytics#webproperty\"."]
        pub _type: ::std::string::String,
    }
    impl ProfileParentLink {
        pub fn builder() -> ProfileParentLinkBuilder {
            ProfileParentLinkBuilder::default()
        }
    }
    mod profile_parent_link_defaults {
        pub fn _type() -> ::std::string::String {
            String::from("analytics#webproperty")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Permissions the user has for this view (profile)."]
    pub struct ProfilePermissions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "effective")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All the permissions that the user has for this view (profile). These include any implied permissions (e.g., EDIT implies VIEW) or inherited permissions from the parent web property."]
        pub effective: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ProfilePermissions {
        pub fn builder() -> ProfilePermissionsBuilder {
            ProfilePermissionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for an Analytics profile filter link."]
    pub struct ProfileFilterLink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filterRef")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Filter for this link."]
        pub filter_ref: ::std::option::Option<::std::boxed::Box<FilterRef>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Profile filter link ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ profile_filter_link_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "profile_filter_link_defaults :: kind")]
        #[doc = "Resource type for Analytics filter."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileRef")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "View (Profile) for this link."]
        pub profile_ref: ::std::option::Option<::std::boxed::Box<ProfileRef>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rank")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The rank of this profile filter link relative to the other filters linked to the same profile.\nFor readonly (i.e., list and get) operations, the rank always starts at 1.\nFor write (i.e., create, update, or delete) operations, you may specify a value between 0 and 255 inclusively, [0, 255]. In order to insert a link at the end of the list, either don't specify a rank or set a rank to a number greater than the largest rank in the list. In order to insert a link to the beginning of the list specify a rank that is less than or equal to 1. The new link will move all existing filters with the same or lower rank down the list. After the link is inserted/updated/deleted all profile filter links will be renumbered starting at 1."]
        pub rank: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link for this profile filter link."]
        pub self_link: ::std::option::Option<::std::string::String>,
    }
    impl ProfileFilterLink {
        pub fn builder() -> ProfileFilterLinkBuilder {
            ProfileFilterLinkBuilder::default()
        }
    }
    mod profile_filter_link_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#profileFilterLink")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A profile filter link collection lists profile filter links between profiles and filters. Each resource in the collection corresponds to a profile filter link."]
    pub struct ProfileFilterLinks {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of profile filter links."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProfileFilterLink>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemsPerPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1,000 with a value of 1000 by default, or otherwise specified by the max-results query parameter."]
        pub items_per_page: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ profile_filter_links_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "profile_filter_links_defaults :: kind")]
        #[doc = "Collection type."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to next page for this profile filter link collection."]
        pub next_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previousLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to previous page for this profile filter link collection."]
        pub previous_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of results for the query, regardless of the number of results in the response."]
        pub total_results: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "username")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email ID of the authenticated user"]
        pub username: ::std::option::Option<::std::string::String>,
    }
    impl ProfileFilterLinks {
        pub fn builder() -> ProfileFilterLinksBuilder {
            ProfileFilterLinksBuilder::default()
        }
    }
    mod profile_filter_links_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#profileFilterLinks")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for a linked view (profile)."]
    pub struct ProfileRef {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account ID to which this view (profile) belongs."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "href")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link for this view (profile)."]
        pub href: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "View (Profile) ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "internalWebPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Internal ID for the web property to which this view (profile) belongs."]
        pub internal_web_property_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ profile_ref_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "profile_ref_defaults :: kind")]
        #[doc = "Analytics view (profile) reference."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of this view (profile)."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web property ID of the form UA-XXXXX-YY to which this view (profile) belongs."]
        pub web_property_id: ::std::option::Option<::std::string::String>,
    }
    impl ProfileRef {
        pub fn builder() -> ProfileRefBuilder {
            ProfileRefBuilder::default()
        }
    }
    mod profile_ref_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#profileRef")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for an Analytics ProfileSummary. ProfileSummary returns basic information (i.e., summary) for a profile."]
    pub struct ProfileSummary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "View (profile) ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ profile_summary_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "profile_summary_defaults :: kind")]
        #[doc = "Resource type for Analytics ProfileSummary."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "View (profile) name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "starred")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether this view (profile) is starred or not."]
        pub starred: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "View (Profile) type. Supported types: WEB or APP."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl ProfileSummary {
        pub fn builder() -> ProfileSummaryBuilder {
            ProfileSummaryBuilder::default()
        }
    }
    mod profile_summary_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#profileSummary")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A view (profile) collection lists Analytics views (profiles) to which the user has access. Each resource in the collection corresponds to a single Analytics view (profile)."]
    pub struct Profiles {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of views (profiles)."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Profile>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemsPerPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter."]
        pub items_per_page: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ profiles_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "profiles_defaults :: kind")]
        #[doc = "Collection type."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to next page for this view (profile) collection."]
        pub next_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previousLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to previous page for this view (profile) collection."]
        pub previous_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of results for the query, regardless of the number of results in the response."]
        pub total_results: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "username")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email ID of the authenticated user"]
        pub username: ::std::option::Option<::std::string::String>,
    }
    impl Profiles {
        pub fn builder() -> ProfilesBuilder {
            ProfilesBuilder::default()
        }
    }
    mod profiles_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#profiles")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Real time data for a given view (profile)."]
    pub struct RealtimeData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnHeaders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Column headers that list dimension names followed by the metric names. The order of dimensions and metrics is same as specified in the request."]
        pub column_headers: ::std::option::Option<::std::vec::Vec<RealtimeDataColumnHeaders>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique ID for this data response."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ realtime_data_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "realtime_data_defaults :: kind")]
        #[doc = "Resource type."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information for the view (profile), for which the real time data was requested."]
        pub profile_info: ::std::option::Option<RealtimeDataProfileInfo>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "query")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Real time data request query parameters."]
        pub query: ::std::option::Option<RealtimeDataQuery>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Real time data rows, where each row contains a list of dimension values followed by the metric values. The order of dimensions and metrics is same as specified in the request."]
        pub rows: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to this page."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of rows for the query, regardless of the number of rows in the response."]
        pub total_results: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalsForAllResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total values for the requested metrics over all the results, not just the results returned in this response. The order of the metric totals is same as the metric order specified in the request."]
        pub totals_for_all_results:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl RealtimeData {
        pub fn builder() -> RealtimeDataBuilder {
            RealtimeDataBuilder::default()
        }
    }
    mod realtime_data_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#realtimeData")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct RealtimeDataColumnHeaders {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Column Type. Either DIMENSION or METRIC."]
        pub column_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data type. Dimension column headers have only STRING as the data type. Metric column headers have data types for metric values such as INTEGER, DOUBLE, CURRENCY etc."]
        pub data_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Column name."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl RealtimeDataColumnHeaders {
        pub fn builder() -> RealtimeDataColumnHeadersBuilder {
            RealtimeDataColumnHeadersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information for the view (profile), for which the real time data was requested."]
    pub struct RealtimeDataProfileInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account ID to which this view (profile) belongs."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "internalWebPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Internal ID for the web property to which this view (profile) belongs."]
        pub internal_web_property_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "View (Profile) ID."]
        pub profile_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "View (Profile) name."]
        pub profile_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Table ID for view (profile)."]
        pub table_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web Property ID to which this view (profile) belongs."]
        pub web_property_id: ::std::option::Option<::std::string::String>,
    }
    impl RealtimeDataProfileInfo {
        pub fn builder() -> RealtimeDataProfileInfoBuilder {
            RealtimeDataProfileInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Real time data request query parameters."]
    pub struct RealtimeDataQuery {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of real time dimensions."]
        pub dimensions: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Comma-separated list of dimension or metric filters."]
        pub filters: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ids")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique table ID."]
        pub ids: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "max-results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum results per page."]
        pub max_results: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of real time metrics."]
        pub metrics: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sort")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of dimensions or metrics based on which real time data is sorted."]
        pub sort: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl RealtimeDataQuery {
        pub fn builder() -> RealtimeDataQueryBuilder {
            RealtimeDataQueryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for an Analytics remarketing audience."]
    pub struct RemarketingAudience {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account ID to which this remarketing audience belongs."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audienceDefinition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The simple audience definition that will cause a user to be added to an audience."]
        pub audience_definition: ::std::option::Option<RemarketingAudienceAudienceDefinition>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audienceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of audience, either SIMPLE or STATE_BASED."]
        pub audience_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "created")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time this remarketing audience was created."]
        pub created: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The description of this remarketing audience."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Remarketing Audience ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "internalWebPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Internal ID for the web property to which this remarketing audience belongs."]
        pub internal_web_property_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ remarketing_audience_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "remarketing_audience_defaults :: kind")]
        #[doc = "Collection type."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linkedAdAccounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The linked ad accounts associated with this remarketing audience. A remarketing audience can have only one linkedAdAccount currently."]
        pub linked_ad_accounts:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LinkedForeignAccount>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linkedViews")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The views (profiles) that this remarketing audience is linked to."]
        pub linked_views: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of this remarketing audience."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateBasedAudienceDefinition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A state based audience definition that will cause a user to be added or removed from an audience."]
        pub state_based_audience_definition:
            ::std::option::Option<RemarketingAudienceStateBasedAudienceDefinition>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time this remarketing audience was last modified."]
        pub updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web property ID of the form UA-XXXXX-YY to which this remarketing audience belongs."]
        pub web_property_id: ::std::option::Option<::std::string::String>,
    }
    impl RemarketingAudience {
        pub fn builder() -> RemarketingAudienceBuilder {
            RemarketingAudienceBuilder::default()
        }
    }
    mod remarketing_audience_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#remarketingAudience")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The simple audience definition that will cause a user to be added to an audience."]
    pub struct RemarketingAudienceAudienceDefinition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeConditions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines the conditions to include users to the audience."]
        pub include_conditions: ::std::option::Option<::std::boxed::Box<IncludeConditions>>,
    }
    impl RemarketingAudienceAudienceDefinition {
        pub fn builder() -> RemarketingAudienceAudienceDefinitionBuilder {
            RemarketingAudienceAudienceDefinitionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A state based audience definition that will cause a user to be added or removed from an audience."]
    pub struct RemarketingAudienceStateBasedAudienceDefinition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludeConditions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines the conditions to exclude users from the audience."]
        pub exclude_conditions:
            ::std::option::Option<RemarketingAudienceStateBasedAudienceDefinitionExcludeConditions>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeConditions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines the conditions to include users to the audience."]
        pub include_conditions: ::std::option::Option<::std::boxed::Box<IncludeConditions>>,
    }
    impl RemarketingAudienceStateBasedAudienceDefinition {
        pub fn builder() -> RemarketingAudienceStateBasedAudienceDefinitionBuilder {
            RemarketingAudienceStateBasedAudienceDefinitionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines the conditions to exclude users from the audience."]
    pub struct RemarketingAudienceStateBasedAudienceDefinitionExcludeConditions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exclusionDuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to make the exclusion TEMPORARY or PERMANENT."]
        pub exclusion_duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The segment condition that will cause a user to be removed from an audience."]
        pub segment: ::std::option::Option<::std::string::String>,
    }
    impl RemarketingAudienceStateBasedAudienceDefinitionExcludeConditions {
        pub fn builder() -> RemarketingAudienceStateBasedAudienceDefinitionExcludeConditionsBuilder
        {
            RemarketingAudienceStateBasedAudienceDefinitionExcludeConditionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A remarketing audience collection lists Analytics remarketing audiences to which the user has access. Each resource in the collection corresponds to a single Analytics remarketing audience."]
    pub struct RemarketingAudiences {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of remarketing audiences."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RemarketingAudience>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemsPerPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter."]
        pub items_per_page: ::std::option::Option<::std::primitive::i64>,
        #[builder(
            default = "{ remarketing_audiences_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "remarketing_audiences_defaults :: kind")]
        #[doc = "Collection type."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to next page for this remarketing audience collection."]
        pub next_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previousLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to previous page for this view (profile) collection."]
        pub previous_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of results for the query, regardless of the number of results in the response."]
        pub total_results: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "username")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email ID of the authenticated user"]
        pub username: ::std::option::Option<::std::string::String>,
    }
    impl RemarketingAudiences {
        pub fn builder() -> RemarketingAudiencesBuilder {
            RemarketingAudiencesBuilder::default()
        }
    }
    mod remarketing_audiences_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#remarketingAudiences")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for an Analytics segment."]
    pub struct Segment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "created")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time the segment was created."]
        pub created: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "definition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Segment definition."]
        pub definition: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Segment ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ segment_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "segment_defaults :: kind")]
        #[doc = "Resource type for Analytics segment."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Segment name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segmentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Segment ID. Can be used with the 'segment' parameter in Core Reporting API."]
        pub segment_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link for this segment."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type for a segment. Possible values are \"BUILT_IN\" or \"CUSTOM\"."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time the segment was last modified."]
        pub updated: ::std::option::Option<::std::string::String>,
    }
    impl Segment {
        pub fn builder() -> SegmentBuilder {
            SegmentBuilder::default()
        }
    }
    mod segment_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#segment")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An segment collection lists Analytics segments that the user has access to. Each resource in the collection corresponds to a single Analytics segment."]
    pub struct Segments {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of segments."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Segment>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemsPerPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter."]
        pub items_per_page: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ segments_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "segments_defaults :: kind")]
        #[doc = "Collection type for segments."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to next page for this segment collection."]
        pub next_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previousLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to previous page for this segment collection."]
        pub previous_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of results for the query, regardless of the number of results in the response."]
        pub total_results: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "username")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email ID of the authenticated user"]
        pub username: ::std::option::Option<::std::string::String>,
    }
    impl Segments {
        pub fn builder() -> SegmentsBuilder {
            SegmentsBuilder::default()
        }
    }
    mod segments_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#segments")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for Analytics unsampled report resource."]
    pub struct UnsampledReport {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account ID to which this unsampled report belongs."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloudStorageDownloadDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Download details for a file stored in Google Cloud Storage."]
        pub cloud_storage_download_details:
            ::std::option::Option<UnsampledReportCloudStorageDownloadDetails>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "created")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time this unsampled report was created."]
        pub created: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dimensions for the unsampled report."]
        pub dimensions: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "downloadType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of download you need to use for the report data file. Possible values include `GOOGLE_DRIVE` and `GOOGLE_CLOUD_STORAGE`. If the value is `GOOGLE_DRIVE`, see the `driveDownloadDetails` field. If the value is `GOOGLE_CLOUD_STORAGE`, see the `cloudStorageDownloadDetails` field."]
        pub download_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "driveDownloadDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Download details for a file stored in Google Drive."]
        pub drive_download_details: ::std::option::Option<UnsampledReportDriveDownloadDetails>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "end-date")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end date for the unsampled report."]
        pub end_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The filters for the unsampled report."]
        pub filters: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unsampled report ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ unsampled_report_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "unsampled_report_defaults :: kind")]
        #[doc = "Resource type for an Analytics unsampled report."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metrics for the unsampled report."]
        pub metrics: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "View (Profile) ID to which this unsampled report belongs."]
        pub profile_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The segment for the unsampled report."]
        pub segment: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link for this unsampled report."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "start-date")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start date for the unsampled report."]
        pub start_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of this unsampled report. Possible values are PENDING, COMPLETED, or FAILED."]
        pub status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title of the unsampled report."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time this unsampled report was last modified."]
        pub updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web property ID to which this unsampled report belongs. The web property ID is of the form UA-XXXXX-YY."]
        pub web_property_id: ::std::option::Option<::std::string::String>,
    }
    impl UnsampledReport {
        pub fn builder() -> UnsampledReportBuilder {
            UnsampledReportBuilder::default()
        }
    }
    mod unsampled_report_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#unsampledReport")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Download details for a file stored in Google Cloud Storage."]
    pub struct UnsampledReportCloudStorageDownloadDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Id of the bucket the file object is stored in."]
        pub bucket_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Id of the file object containing the report data."]
        pub object_id: ::std::option::Option<::std::string::String>,
    }
    impl UnsampledReportCloudStorageDownloadDetails {
        pub fn builder() -> UnsampledReportCloudStorageDownloadDetailsBuilder {
            UnsampledReportCloudStorageDownloadDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Download details for a file stored in Google Drive."]
    pub struct UnsampledReportDriveDownloadDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "documentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Id of the document/file containing the report data."]
        pub document_id: ::std::option::Option<::std::string::String>,
    }
    impl UnsampledReportDriveDownloadDetails {
        pub fn builder() -> UnsampledReportDriveDownloadDetailsBuilder {
            UnsampledReportDriveDownloadDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An unsampled report collection lists Analytics unsampled reports to which the user has access. Each view (profile) can have a set of unsampled reports. Each resource in the unsampled report collection corresponds to a single Analytics unsampled report."]
    pub struct UnsampledReports {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of unsampled reports."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UnsampledReport>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemsPerPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter."]
        pub items_per_page: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ unsampled_reports_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "unsampled_reports_defaults :: kind")]
        #[doc = "Collection type."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to next page for this unsampled report collection."]
        pub next_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previousLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to previous page for this unsampled report collection."]
        pub previous_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of results for the query, regardless of the number of resources in the result."]
        pub total_results: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "username")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email ID of the authenticated user"]
        pub username: ::std::option::Option<::std::string::String>,
    }
    impl UnsampledReports {
        pub fn builder() -> UnsampledReportsBuilder {
            UnsampledReportsBuilder::default()
        }
    }
    mod unsampled_reports_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#unsampledReports")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata returned for an upload operation."]
    pub struct Upload {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account Id to which this upload belongs."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customDataSourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom data source Id to which this data import belongs."]
        pub custom_data_source_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data import errors collection."]
        pub errors: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique ID for this upload."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ upload_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "upload_defaults :: kind")]
        #[doc = "Resource type for Analytics upload."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Upload status. Possible values: PENDING, COMPLETED, FAILED, DELETING, DELETED."]
        pub status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uploadTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time this file is uploaded."]
        pub upload_time: ::std::option::Option<::std::string::String>,
    }
    impl Upload {
        pub fn builder() -> UploadBuilder {
            UploadBuilder::default()
        }
    }
    mod upload_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#upload")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Upload collection lists Analytics uploads to which the user has access. Each custom data source can have a set of uploads. Each resource in the upload collection corresponds to a single Analytics data upload."]
    pub struct Uploads {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of uploads."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Upload>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemsPerPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter."]
        pub items_per_page: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ uploads_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "uploads_defaults :: kind")]
        #[doc = "Collection type."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to next page for this upload collection."]
        pub next_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previousLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to previous page for this upload collection."]
        pub previous_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of results for the query, regardless of the number of resources in the result."]
        pub total_results: ::std::option::Option<::std::primitive::i64>,
    }
    impl Uploads {
        pub fn builder() -> UploadsBuilder {
            UploadsBuilder::default()
        }
    }
    mod uploads_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#uploads")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for a user deletion request resource."]
    pub struct UserDeletionRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deletionRequestTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This marks the point in time for which all user data before should be deleted"]
        pub deletion_request_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firebaseProjectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Firebase Project Id"]
        pub firebase_project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User ID."]
        pub id: ::std::option::Option<UserDeletionRequestId>,
        #[builder(
            default = "{ user_deletion_request_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "user_deletion_request_defaults :: kind")]
        #[doc = "Value is \"analytics#userDeletionRequest\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "propertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Property ID"]
        pub property_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web property ID of the form UA-XXXXX-YY."]
        pub web_property_id: ::std::option::Option<::std::string::String>,
    }
    impl UserDeletionRequest {
        pub fn builder() -> UserDeletionRequestBuilder {
            UserDeletionRequestBuilder::default()
        }
    }
    mod user_deletion_request_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#userDeletionRequest")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "User ID."]
    pub struct UserDeletionRequestId {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of user"]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The User's id"]
        pub user_id: ::std::option::Option<::std::string::String>,
    }
    impl UserDeletionRequestId {
        pub fn builder() -> UserDeletionRequestIdBuilder {
            UserDeletionRequestIdBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for a user reference."]
    pub struct UserRef {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email ID of this user."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ user_ref_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "user_ref_defaults :: kind")]
        pub kind: ::std::string::String,
    }
    impl UserRef {
        pub fn builder() -> UserRefBuilder {
            UserRefBuilder::default()
        }
    }
    mod user_ref_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#userRef")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for a web property reference."]
    pub struct WebPropertyRef {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account ID to which this web property belongs."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "href")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link for this web property."]
        pub href: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web property ID of the form UA-XXXXX-YY."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "internalWebPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Internal ID for this web property."]
        pub internal_web_property_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ web_property_ref_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "web_property_ref_defaults :: kind")]
        #[doc = "Analytics web property reference."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of this web property."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl WebPropertyRef {
        pub fn builder() -> WebPropertyRefBuilder {
            WebPropertyRefBuilder::default()
        }
    }
    mod web_property_ref_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#webPropertyRef")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for an Analytics WebPropertySummary. WebPropertySummary returns basic information (i.e., summary) for a web property."]
    pub struct WebPropertySummary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web property ID of the form UA-XXXXX-YY."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "internalWebPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Internal ID for this web property."]
        pub internal_web_property_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ web_property_summary_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "web_property_summary_defaults :: kind")]
        #[doc = "Resource type for Analytics WebPropertySummary."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "level")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Level for this web property. Possible values are STANDARD or PREMIUM."]
        pub level: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web property name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profiles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of profiles under this web property."]
        pub profiles: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProfileSummary>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "starred")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether this web property is starred or not."]
        pub starred: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "websiteUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Website url for this web property."]
        pub website_url: ::std::option::Option<::std::string::String>,
    }
    impl WebPropertySummary {
        pub fn builder() -> WebPropertySummaryBuilder {
            WebPropertySummaryBuilder::default()
        }
    }
    mod web_property_summary_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#webPropertySummary")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A web property collection lists Analytics web properties to which the user has access. Each resource in the collection corresponds to a single Analytics web property."]
    pub struct Webproperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of web properties."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Webproperty>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemsPerPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter."]
        pub items_per_page: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ webproperties_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "webproperties_defaults :: kind")]
        #[doc = "Collection type."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to next page for this web property collection."]
        pub next_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previousLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to previous page for this web property collection."]
        pub previous_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of results for the query, regardless of the number of results in the response."]
        pub total_results: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "username")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email ID of the authenticated user"]
        pub username: ::std::option::Option<::std::string::String>,
    }
    impl Webproperties {
        pub fn builder() -> WebpropertiesBuilder {
            WebpropertiesBuilder::default()
        }
    }
    mod webproperties_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#webproperties")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for an Analytics web property."]
    pub struct Webproperty {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account ID to which this web property belongs."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "childLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Child link for this web property. Points to the list of views (profiles) for this web property."]
        pub child_link: ::std::option::Option<WebpropertyChildLink>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "created")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time this web property was created."]
        pub created: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataRetentionResetOnNewActivity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set to true to reset the retention period of the user identifier with each new event from that user (thus setting the expiration date to current time plus retention period).\nSet to false to delete data associated with the user identifier automatically after the rentention period.\nThis property cannot be set on insert."]
        pub data_retention_reset_on_new_activity: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataRetentionTtl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The length of time for which user and event data is retained.\nThis property cannot be set on insert."]
        pub data_retention_ttl: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultProfileId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Default view (profile) ID."]
        pub default_profile_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web property ID of the form UA-XXXXX-YY."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "industryVertical")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The industry vertical/category selected for this web property."]
        pub industry_vertical: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "internalWebPropertyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Internal ID for this web property."]
        pub internal_web_property_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ webproperty_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "webproperty_defaults :: kind")]
        #[doc = "Resource type for Analytics WebProperty."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "level")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Level for this web property. Possible values are STANDARD or PREMIUM."]
        pub level: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of this web property."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parent link for this web property. Points to the account to which this web property belongs."]
        pub parent_link: ::std::option::Option<WebpropertyParentLink>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Permissions the user has for this web property."]
        pub permissions: ::std::option::Option<WebpropertyPermissions>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "View (Profile) count for this web property."]
        pub profile_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link for this web property."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "starred")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether this web property is starred or not."]
        pub starred: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time this web property was last modified."]
        pub updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "websiteUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Website url for this web property."]
        pub website_url: ::std::option::Option<::std::string::String>,
    }
    impl Webproperty {
        pub fn builder() -> WebpropertyBuilder {
            WebpropertyBuilder::default()
        }
    }
    mod webproperty_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("analytics#webproperty")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Child link for this web property. Points to the list of views (profiles) for this web property."]
    pub struct WebpropertyChildLink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "href")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to the list of views (profiles) for this web property."]
        pub href: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ webproperty_child_link_defaults :: _type () }",
            setter(into)
        )]
        #[serde(rename = "type")]
        #[serde(default = "webproperty_child_link_defaults :: _type")]
        #[doc = "Type of the parent link. Its value is \"analytics#profiles\"."]
        pub _type: ::std::string::String,
    }
    impl WebpropertyChildLink {
        pub fn builder() -> WebpropertyChildLinkBuilder {
            WebpropertyChildLinkBuilder::default()
        }
    }
    mod webproperty_child_link_defaults {
        pub fn _type() -> ::std::string::String {
            String::from("analytics#profiles")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parent link for this web property. Points to the account to which this web property belongs."]
    pub struct WebpropertyParentLink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "href")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to the account for this web property."]
        pub href: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ webproperty_parent_link_defaults :: _type () }",
            setter(into)
        )]
        #[serde(rename = "type")]
        #[serde(default = "webproperty_parent_link_defaults :: _type")]
        #[doc = "Type of the parent link. Its value is \"analytics#account\"."]
        pub _type: ::std::string::String,
    }
    impl WebpropertyParentLink {
        pub fn builder() -> WebpropertyParentLinkBuilder {
            WebpropertyParentLinkBuilder::default()
        }
    }
    mod webproperty_parent_link_defaults {
        pub fn _type() -> ::std::string::String {
            String::from("analytics#account")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Permissions the user has for this web property."]
    pub struct WebpropertyPermissions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "effective")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All the permissions that the user has for this web property. These include any implied permissions (e.g., EDIT implies VIEW) or inherited permissions from the parent account."]
        pub effective: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl WebpropertyPermissions {
        pub fn builder() -> WebpropertyPermissionsBuilder {
            WebpropertyPermissionsBuilder::default()
        }
    }
}
