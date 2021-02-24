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
    pub mod billing_accounts {
        pub mod resources {
            pub mod locations {
                pub mod resources {
                    pub mod insight_types {
                        pub mod resources {
                            pub mod insights {
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
                                            #[doc = "Optional. Filter expression to restrict the insights returned. Supported filter fields: state Eg: `state:\"DISMISSED\" or state:\"ACTIVE\""]
                                            pub filter:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Optional. The maximum number of results to return from this request. Non-positive values are ignored. If not specified, the server will determine the number of results to return."]
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
                                            #[doc = "Optional. If present, retrieves the next batch of results from the preceding call to this method. `page_token` must be the value of `next_page_token` from the previous response. The values of other method parameters must be identical to those in the previous call."]
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
                        }
                    }
                    pub mod recommenders {
                        pub mod resources {
                            pub mod recommendations {
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
                                            #[doc = "Filter expression to restrict the recommendations returned. Supported filter fields: state_info.state Eg: `state_info.state:\"DISMISSED\" or state_info.state:\"FAILED\""]
                                            pub filter:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Optional. The maximum number of results to return from this request. Non-positive values are ignored. If not specified, the server will determine the number of results to return."]
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
                                            #[doc = "Optional. If present, retrieves the next batch of results from the preceding call to this method. `page_token` must be the value of `next_page_token` from the previous response. The values of other method parameters must be identical to those in the previous call."]
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
                        }
                    }
                }
            }
        }
    }
    pub mod folders {
        pub mod resources {
            pub mod locations {
                pub mod resources {
                    pub mod insight_types {
                        pub mod resources {
                            pub mod insights {
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
                                            #[doc = "Optional. Filter expression to restrict the insights returned. Supported filter fields: state Eg: `state:\"DISMISSED\" or state:\"ACTIVE\""]
                                            pub filter:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Optional. The maximum number of results to return from this request. Non-positive values are ignored. If not specified, the server will determine the number of results to return."]
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
                                            #[doc = "Optional. If present, retrieves the next batch of results from the preceding call to this method. `page_token` must be the value of `next_page_token` from the previous response. The values of other method parameters must be identical to those in the previous call."]
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
                        }
                    }
                    pub mod recommenders {
                        pub mod resources {
                            pub mod recommendations {
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
                                            #[doc = "Filter expression to restrict the recommendations returned. Supported filter fields: state_info.state Eg: `state_info.state:\"DISMISSED\" or state_info.state:\"FAILED\""]
                                            pub filter:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Optional. The maximum number of results to return from this request. Non-positive values are ignored. If not specified, the server will determine the number of results to return."]
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
                                            #[doc = "Optional. If present, retrieves the next batch of results from the preceding call to this method. `page_token` must be the value of `next_page_token` from the previous response. The values of other method parameters must be identical to those in the previous call."]
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
                        }
                    }
                }
            }
        }
    }
    pub mod organizations {
        pub mod resources {
            pub mod locations {
                pub mod resources {
                    pub mod insight_types {
                        pub mod resources {
                            pub mod insights {
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
                                            #[doc = "Optional. Filter expression to restrict the insights returned. Supported filter fields: state Eg: `state:\"DISMISSED\" or state:\"ACTIVE\""]
                                            pub filter:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Optional. The maximum number of results to return from this request. Non-positive values are ignored. If not specified, the server will determine the number of results to return."]
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
                                            #[doc = "Optional. If present, retrieves the next batch of results from the preceding call to this method. `page_token` must be the value of `next_page_token` from the previous response. The values of other method parameters must be identical to those in the previous call."]
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
                        }
                    }
                    pub mod recommenders {
                        pub mod resources {
                            pub mod recommendations {
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
                                            #[doc = "Filter expression to restrict the recommendations returned. Supported filter fields: state_info.state Eg: `state_info.state:\"DISMISSED\" or state_info.state:\"FAILED\""]
                                            pub filter:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Optional. The maximum number of results to return from this request. Non-positive values are ignored. If not specified, the server will determine the number of results to return."]
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
                                            #[doc = "Optional. If present, retrieves the next batch of results from the preceding call to this method. `page_token` must be the value of `next_page_token` from the previous response. The values of other method parameters must be identical to those in the previous call."]
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
                        }
                    }
                }
            }
        }
    }
    pub mod projects {
        pub mod resources {
            pub mod locations {
                pub mod resources {
                    pub mod insight_types {
                        pub mod resources {
                            pub mod insights {
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
                                            #[doc = "Optional. Filter expression to restrict the insights returned. Supported filter fields: state Eg: `state:\"DISMISSED\" or state:\"ACTIVE\""]
                                            pub filter:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Optional. The maximum number of results to return from this request. Non-positive values are ignored. If not specified, the server will determine the number of results to return."]
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
                                            #[doc = "Optional. If present, retrieves the next batch of results from the preceding call to this method. `page_token` must be the value of `next_page_token` from the previous response. The values of other method parameters must be identical to those in the previous call."]
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
                        }
                    }
                    pub mod recommenders {
                        pub mod resources {
                            pub mod recommendations {
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
                                            #[doc = "Filter expression to restrict the recommendations returned. Supported filter fields: state_info.state Eg: `state_info.state:\"DISMISSED\" or state_info.state:\"FAILED\""]
                                            pub filter:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Optional. The maximum number of results to return from this request. Non-positive values are ignored. If not specified, the server will determine the number of results to return."]
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
                                            #[doc = "Optional. If present, retrieves the next batch of results from the preceding call to this method. `page_token` must be the value of `next_page_token` from the previous response. The values of other method parameters must be identical to those in the previous call."]
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
    #[doc = "Contains metadata about how much money a recommendation can save or incur."]
    pub struct GoogleCloudRecommenderV1CostProjection {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cost")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An approximate projection on amount saved or amount incurred. Negative cost units indicate cost savings and positive cost units indicate increase. See google.type.Money documentation for positive/negative units."]
        pub cost: ::std::option::Option<::std::boxed::Box<GoogleTypeMoney>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Duration for which this cost applies."]
        pub duration: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRecommenderV1CostProjection {
        pub fn builder() -> GoogleCloudRecommenderV1CostProjectionBuilder {
            GoogleCloudRecommenderV1CostProjectionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains the impact a recommendation can have for a given category."]
    pub struct GoogleCloudRecommenderV1Impact {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "category")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Category that is being targeted."]
        pub category: ::std::option::Option<GoogleCloudRecommenderV1ImpactCategoryEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "costProjection")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Use with CategoryType.COST"]
        pub cost_projection:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRecommenderV1CostProjection>>,
    }
    impl GoogleCloudRecommenderV1Impact {
        pub fn builder() -> GoogleCloudRecommenderV1ImpactBuilder {
            GoogleCloudRecommenderV1ImpactBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Category that is being targeted."]
    pub enum GoogleCloudRecommenderV1ImpactCategoryEnum {
        #[serde(rename = "CATEGORY_UNSPECIFIED")]
        #[doc = "Default unspecified category. Don't use directly."]
        CategoryUnspecified,
        #[serde(rename = "COST")]
        #[doc = "Indicates a potential increase or decrease in cost."]
        Cost,
        #[serde(rename = "SECURITY")]
        #[doc = "Indicates a potential increase or decrease in security."]
        Security,
        #[serde(rename = "PERFORMANCE")]
        #[doc = "Indicates a potential increase or decrease in performance."]
        Performance,
        #[serde(rename = "MANAGEABILITY")]
        #[doc = "Indicates a potential increase or decrease in manageability."]
        Manageability,
    }
    impl ::std::default::Default for GoogleCloudRecommenderV1ImpactCategoryEnum {
        fn default() -> Self {
            Self::CategoryUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An insight along with the information used to derive the insight. The insight may have associated recomendations as well."]
    pub struct GoogleCloudRecommenderV1Insight {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "associatedRecommendations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Recommendations derived from this insight."]
        pub associated_recommendations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudRecommenderV1InsightRecommendationReference>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "category")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Category being targeted by the insight."]
        pub category: ::std::option::Option<GoogleCloudRecommenderV1InsightCategoryEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A struct of custom fields to explain the insight. Example: \"grantedPermissionsCount\": \"1000\""]
        pub content:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Free-form human readable summary in English. The maximum length is 500 characters."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fingerprint of the Insight. Provides optimistic locking when updating states."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insightSubtype")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Insight subtype. Insight content schema will be stable for a given subtype."]
        pub insight_subtype: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastRefreshTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp of the latest data used to generate the insight."]
        pub last_refresh_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the insight."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "observationPeriod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Observation period that led to the insight. The source data used to generate the insight ends at last_refresh_time and begins at (last_refresh_time - observation_period)."]
        pub observation_period: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information state and metadata."]
        pub state_info:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRecommenderV1InsightStateInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetResources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fully qualified resource names that this insight is targeting."]
        pub target_resources: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudRecommenderV1Insight {
        pub fn builder() -> GoogleCloudRecommenderV1InsightBuilder {
            GoogleCloudRecommenderV1InsightBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Category being targeted by the insight."]
    pub enum GoogleCloudRecommenderV1InsightCategoryEnum {
        #[serde(rename = "CATEGORY_UNSPECIFIED")]
        #[doc = "Unspecified category."]
        CategoryUnspecified,
        #[serde(rename = "COST")]
        #[doc = "The insight is related to cost."]
        Cost,
        #[serde(rename = "SECURITY")]
        #[doc = "The insight is related to security."]
        Security,
        #[serde(rename = "PERFORMANCE")]
        #[doc = "The insight is related to performance."]
        Performance,
        #[serde(rename = "MANAGEABILITY")]
        #[doc = "This insight is related to manageability."]
        Manageability,
    }
    impl ::std::default::Default for GoogleCloudRecommenderV1InsightCategoryEnum {
        fn default() -> Self {
            Self::CategoryUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Reference to an associated recommendation."]
    pub struct GoogleCloudRecommenderV1InsightRecommendationReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recommendation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Recommendation resource name, e.g. projects/[PROJECT_NUMBER]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]/recommendations/[RECOMMENDATION_ID]"]
        pub recommendation: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRecommenderV1InsightRecommendationReference {
        pub fn builder() -> GoogleCloudRecommenderV1InsightRecommendationReferenceBuilder {
            GoogleCloudRecommenderV1InsightRecommendationReferenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information related to insight state."]
    pub struct GoogleCloudRecommenderV1InsightStateInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Insight state."]
        pub state: ::std::option::Option<GoogleCloudRecommenderV1InsightStateInfoStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A map of metadata for the state, provided by user or automations systems."]
        pub state_metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl GoogleCloudRecommenderV1InsightStateInfo {
        pub fn builder() -> GoogleCloudRecommenderV1InsightStateInfoBuilder {
            GoogleCloudRecommenderV1InsightStateInfoBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Insight state."]
    pub enum GoogleCloudRecommenderV1InsightStateInfoStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Unspecified state."]
        StateUnspecified,
        #[serde(rename = "ACTIVE")]
        #[doc = "Insight is active. Content for ACTIVE insights can be updated by Google. ACTIVE insights can be marked DISMISSED OR ACCEPTED."]
        Active,
        #[serde(rename = "ACCEPTED")]
        #[doc = "Some action has been taken based on this insight. Insights become accepted when a recommendation derived from the insight has been marked CLAIMED, SUCCEEDED, or FAILED. ACTIVE insights can also be marked ACCEPTED explicitly. Content for ACCEPTED insights is immutable. ACCEPTED insights can only be marked ACCEPTED (which may update state metadata)."]
        Accepted,
        #[serde(rename = "DISMISSED")]
        #[doc = "Insight is dismissed. Content for DISMISSED insights can be updated by Google. DISMISSED insights can be marked as ACTIVE."]
        Dismissed,
    }
    impl ::std::default::Default for GoogleCloudRecommenderV1InsightStateInfoStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to the `ListInsights` method."]
    pub struct GoogleCloudRecommenderV1ListInsightsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insights")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of insights for the `parent` resource."]
        pub insights: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudRecommenderV1Insight>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token that can be used to request the next page of results. This field is empty if there are no additional results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRecommenderV1ListInsightsResponse {
        pub fn builder() -> GoogleCloudRecommenderV1ListInsightsResponseBuilder {
            GoogleCloudRecommenderV1ListInsightsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to the `ListRecommendations` method."]
    pub struct GoogleCloudRecommenderV1ListRecommendationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token that can be used to request the next page of results. This field is empty if there are no additional results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recommendations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of recommendations for the `parent` resource."]
        pub recommendations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudRecommenderV1Recommendation>>,
        >,
    }
    impl GoogleCloudRecommenderV1ListRecommendationsResponse {
        pub fn builder() -> GoogleCloudRecommenderV1ListRecommendationsResponseBuilder {
            GoogleCloudRecommenderV1ListRecommendationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for the `MarkInsightAccepted` method."]
    pub struct GoogleCloudRecommenderV1MarkInsightAcceptedRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Fingerprint of the Insight. Provides optimistic locking."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. State properties user wish to include with this state. Full replace of the current state_metadata."]
        pub state_metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl GoogleCloudRecommenderV1MarkInsightAcceptedRequest {
        pub fn builder() -> GoogleCloudRecommenderV1MarkInsightAcceptedRequestBuilder {
            GoogleCloudRecommenderV1MarkInsightAcceptedRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for the `MarkRecommendationClaimed` Method."]
    pub struct GoogleCloudRecommenderV1MarkRecommendationClaimedRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Fingerprint of the Recommendation. Provides optimistic locking."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "State properties to include with this state. Overwrites any existing `state_metadata`. Keys must match the regex /^a-z0-9{0,62}$/. Values must match the regex /^[a-zA-Z0-9_./-]{0,255}$/."]
        pub state_metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl GoogleCloudRecommenderV1MarkRecommendationClaimedRequest {
        pub fn builder() -> GoogleCloudRecommenderV1MarkRecommendationClaimedRequestBuilder {
            GoogleCloudRecommenderV1MarkRecommendationClaimedRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for the `MarkRecommendationFailed` Method."]
    pub struct GoogleCloudRecommenderV1MarkRecommendationFailedRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Fingerprint of the Recommendation. Provides optimistic locking."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "State properties to include with this state. Overwrites any existing `state_metadata`. Keys must match the regex /^a-z0-9{0,62}$/. Values must match the regex /^[a-zA-Z0-9_./-]{0,255}$/."]
        pub state_metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl GoogleCloudRecommenderV1MarkRecommendationFailedRequest {
        pub fn builder() -> GoogleCloudRecommenderV1MarkRecommendationFailedRequestBuilder {
            GoogleCloudRecommenderV1MarkRecommendationFailedRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for the `MarkRecommendationSucceeded` Method."]
    pub struct GoogleCloudRecommenderV1MarkRecommendationSucceededRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Fingerprint of the Recommendation. Provides optimistic locking."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "State properties to include with this state. Overwrites any existing `state_metadata`. Keys must match the regex /^a-z0-9{0,62}$/. Values must match the regex /^[a-zA-Z0-9_./-]{0,255}$/."]
        pub state_metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl GoogleCloudRecommenderV1MarkRecommendationSucceededRequest {
        pub fn builder() -> GoogleCloudRecommenderV1MarkRecommendationSucceededRequestBuilder {
            GoogleCloudRecommenderV1MarkRecommendationSucceededRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains an operation for a resource loosely based on the JSON-PATCH format with support for: * Custom filters for describing partial array patch. * Extended path values for describing nested arrays. * Custom fields for describing the resource for which the operation is being described. * Allows extension to custom operations not natively supported by RFC6902. See https://tools.ietf.org/html/rfc6902 for details on the original RFC."]
    pub struct GoogleCloudRecommenderV1Operation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "action")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of this operation. Contains one of 'and', 'remove', 'replace', 'move', 'copy', 'test' and custom operations. This field is case-insensitive and always populated."]
        pub action: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Path to the target field being operated on. If the operation is at the resource level, then path should be \"/\". This field is always populated."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pathFilters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set of filters to apply if `path` refers to array elements or nested array elements in order to narrow down to a single unique element that is being tested/modified. This is intended to be an exact match per filter. To perform advanced matching, use path_value_matchers. * Example: { \"/versions/*/name\" : \"it-123\" \"/versions/*/targetSize/percent\": 20 } * Example: { \"/bindings/*/role\": \"roles/owner\" \"/bindings/*/condition\" : null } * Example: { \"/bindings/*/role\": \"roles/owner\" \"/bindings/*/members/*\" : [\"x@example.com\", \"y@example.com\"] } When both path_filters and path_value_matchers are set, an implicit AND must be performed."]
        pub path_filters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pathValueMatchers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Similar to path_filters, this contains set of filters to apply if `path` field referes to array elements. This is meant to support value matching beyond exact match. To perform exact match, use path_filters. When both path_filters and path_value_matchers are set, an implicit AND must be performed."]
        pub path_value_matchers: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                ::std::boxed::Box<GoogleCloudRecommenderV1ValueMatcher>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains the fully qualified resource name. This field is always populated. ex: //cloudresourcemanager.googleapis.com/projects/foo."]
        pub resource: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of GCP resource being modified/tested. This field is always populated. Example: cloudresourcemanager.googleapis.com/Project, compute.googleapis.com/Instance"]
        pub resource_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourcePath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Can be set with action 'copy' or 'move' to indicate the source field within resource or source_resource, ignored if provided for other operation types."]
        pub source_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceResource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Can be set with action 'copy' to copy resource configuration across different resources of the same type. Example: A resource clone can be done via action = 'copy', path = \"/\", from = \"/\", source_resource = and resource_name = . This field is empty for all other values of `action`."]
        pub source_resource: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value for the `path` field. Will be set for actions:'add'/'replace'. Maybe set for action: 'test'. Either this or `value_matcher` will be set for 'test' operation. An exact match must be performed."]
        pub value: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueMatcher")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Can be set for action 'test' for advanced matching for the value of 'path' field. Either this or `value` will be set for 'test' operation."]
        pub value_matcher:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRecommenderV1ValueMatcher>>,
    }
    impl GoogleCloudRecommenderV1Operation {
        pub fn builder() -> GoogleCloudRecommenderV1OperationBuilder {
            GoogleCloudRecommenderV1OperationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Group of operations that need to be performed atomically."]
    pub struct GoogleCloudRecommenderV1OperationGroup {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of operations across one or more resources that belong to this group. Loosely based on RFC6902 and should be performed in the order they appear."]
        pub operations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudRecommenderV1Operation>>,
        >,
    }
    impl GoogleCloudRecommenderV1OperationGroup {
        pub fn builder() -> GoogleCloudRecommenderV1OperationGroupBuilder {
            GoogleCloudRecommenderV1OperationGroupBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A recommendation along with a suggested action. E.g., a rightsizing recommendation for an underutilized VM, IAM role recommendations, etc"]
    pub struct GoogleCloudRecommenderV1Recommendation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalImpact")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional set of additional impact that this recommendation may have when trying to optimize for the primary category. These may be positive or negative."]
        pub additional_impact: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudRecommenderV1Impact>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "associatedInsights")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Insights that led to this recommendation."]
        pub associated_insights: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudRecommenderV1RecommendationInsightReference>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Content of the recommendation describing recommended changes to resources."]
        pub content:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRecommenderV1RecommendationContent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Free-form human readable summary in English. The maximum length is 500 characters."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fingerprint of the Recommendation. Provides optimistic locking when updating states."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastRefreshTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Last time this recommendation was refreshed by the system that created it in the first place."]
        pub last_refresh_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of recommendation."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primaryImpact")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The primary impact that this recommendation can have while trying to optimize for one category."]
        pub primary_impact:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRecommenderV1Impact>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recommenderSubtype")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains an identifier for a subtype of recommendations produced for the same recommender. Subtype is a function of content and impact, meaning a new subtype might be added when significant changes to `content` or `primary_impact.category` are introduced. See the Recommenders section to see a list of subtypes for a given Recommender. Examples: For recommender = \"google.iam.policy.Recommender\", recommender_subtype can be one of \"REMOVE_ROLE\"/\"REPLACE_ROLE\""]
        pub recommender_subtype: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information for state. Contains state and metadata."]
        pub state_info: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudRecommenderV1RecommendationStateInfo>,
        >,
    }
    impl GoogleCloudRecommenderV1Recommendation {
        pub fn builder() -> GoogleCloudRecommenderV1RecommendationBuilder {
            GoogleCloudRecommenderV1RecommendationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains what resources are changing and how they are changing."]
    pub struct GoogleCloudRecommenderV1RecommendationContent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationGroups")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Operations to one or more Google Cloud resources grouped in such a way that, all operations within one group are expected to be performed atomically and in an order."]
        pub operation_groups: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudRecommenderV1OperationGroup>>,
        >,
    }
    impl GoogleCloudRecommenderV1RecommendationContent {
        pub fn builder() -> GoogleCloudRecommenderV1RecommendationContentBuilder {
            GoogleCloudRecommenderV1RecommendationContentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Reference to an associated insight."]
    pub struct GoogleCloudRecommenderV1RecommendationInsightReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insight")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Insight resource name, e.g. projects/[PROJECT_NUMBER]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]/insights/[INSIGHT_ID]"]
        pub insight: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRecommenderV1RecommendationInsightReference {
        pub fn builder() -> GoogleCloudRecommenderV1RecommendationInsightReferenceBuilder {
            GoogleCloudRecommenderV1RecommendationInsightReferenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information for state. Contains state and metadata."]
    pub struct GoogleCloudRecommenderV1RecommendationStateInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state of the recommendation, Eg ACTIVE, SUCCEEDED, FAILED."]
        pub state: ::std::option::Option<GoogleCloudRecommenderV1RecommendationStateInfoStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A map of metadata for the state, provided by user or automations systems."]
        pub state_metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl GoogleCloudRecommenderV1RecommendationStateInfo {
        pub fn builder() -> GoogleCloudRecommenderV1RecommendationStateInfoBuilder {
            GoogleCloudRecommenderV1RecommendationStateInfoBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The state of the recommendation, Eg ACTIVE, SUCCEEDED, FAILED."]
    pub enum GoogleCloudRecommenderV1RecommendationStateInfoStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Default state. Don't use directly."]
        StateUnspecified,
        #[serde(rename = "ACTIVE")]
        #[doc = "Recommendation is active and can be applied. Recommendations content can be updated by Google. ACTIVE recommendations can be marked as CLAIMED, SUCCEEDED, or FAILED."]
        Active,
        #[serde(rename = "CLAIMED")]
        #[doc = "Recommendation is in claimed state. Recommendations content is immutable and cannot be updated by Google. CLAIMED recommendations can be marked as CLAIMED, SUCCEEDED, or FAILED."]
        Claimed,
        #[serde(rename = "SUCCEEDED")]
        #[doc = "Recommendation is in succeeded state. Recommendations content is immutable and cannot be updated by Google. SUCCEEDED recommendations can be marked as SUCCEEDED, or FAILED."]
        Succeeded,
        #[serde(rename = "FAILED")]
        #[doc = "Recommendation is in failed state. Recommendations content is immutable and cannot be updated by Google. FAILED recommendations can be marked as SUCCEEDED, or FAILED."]
        Failed,
        #[serde(rename = "DISMISSED")]
        #[doc = "Recommendation is in dismissed state. Recommendation content can be updated by Google. DISMISSED recommendations can be marked as ACTIVE."]
        Dismissed,
    }
    impl ::std::default::Default for GoogleCloudRecommenderV1RecommendationStateInfoStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains various matching options for values for a GCP resource field."]
    pub struct GoogleCloudRecommenderV1ValueMatcher {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchesPattern")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "To be used for full regex matching. The regular expression is using the Google RE2 syntax (https://github.com/google/re2/wiki/Syntax), so to be used with RE2::FullMatch"]
        pub matches_pattern: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRecommenderV1ValueMatcher {
        pub fn builder() -> GoogleCloudRecommenderV1ValueMatcherBuilder {
            GoogleCloudRecommenderV1ValueMatcherBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents an amount of money with its currency type."]
    pub struct GoogleTypeMoney {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currencyCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The three-letter currency code defined in ISO 4217."]
        pub currency_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nanos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000."]
        pub nanos: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "units")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The whole units of the amount. For example if `currencyCode` is `\"USD\"`, then 1 unit is one US dollar."]
        pub units: ::std::option::Option<::std::string::String>,
    }
    impl GoogleTypeMoney {
        pub fn builder() -> GoogleTypeMoneyBuilder {
            GoogleTypeMoneyBuilder::default()
        }
    }
}
