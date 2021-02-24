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
    pub mod bidders {
        pub mod resources {
            pub mod creatives {
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
                            #[doc = "Query string to filter creatives. If no filter is specified, all active creatives will be returned. Example: 'accountId=12345 AND (dealsStatus:DISAPPROVED AND disapprovalReason:UNACCEPTABLE_CONTENT) OR declaredAttributes:IS_COOKIE_TARGETED'"]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Requested page size. The server may return fewer creatives than requested (due to timeout constraint) even if more are available via another call. If unspecified, server will pick an appropriate default. Acceptable values are 1 to 1000, inclusive."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListCreativesResponse.nextPageToken returned from the previous call to the 'ListCreatives' method."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "view")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Controls the amount of information included in the response. By default only creativeServingDecision is included. To retrieve the entire creative resource (including the declared fields and the creative content) specify the view as \"FULL\"."]
                            pub view: ::std::option::Option<QueryParametersViewEnum>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Controls the amount of information included in the response. By default only creativeServingDecision is included. To retrieve the entire creative resource (including the declared fields and the creative content) specify the view as \"FULL\"."]
                        pub enum QueryParametersViewEnum {
                            #[serde(rename = "CREATIVE_VIEW_UNSPECIFIED")]
                            #[doc = "Not specified, equivalent to SERVING_DECISION_ONLY."]
                            CreativeViewUnspecified,
                            #[serde(rename = "SERVING_DECISION_ONLY")]
                            #[doc = "Only creativeServingDecision is included in the response."]
                            ServingDecisionOnly,
                            #[serde(rename = "FULL")]
                            #[doc = "The entire creative resource (including the declared fields and the creative content) is included in the response."]
                            Full,
                        }
                        impl ::std::default::Default for QueryParametersViewEnum {
                            fn default() -> Self {
                                Self::CreativeViewUnspecified
                            }
                        }
                    }
                }
            }
            pub mod pretargeting_configs {
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
                            #[doc = "The maximum number of pretargeting configurations to return. If unspecified, at most 10 pretargeting configurations will be returned. The maximum value is 100; values above 100 will be coerced to 100."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A token identifying a page of results the server should return. This value is received from a previous `ListPretargetingConfigs` call in ListPretargetingConfigsResponse.nextPageToken."]
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
                            #[doc = "Field mask to use for partial in-place updates."]
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
    pub mod buyers {
        pub mod resources {
            pub mod creatives {
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
                            #[serde(rename = "view")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Controls the amount of information included in the response. By default only creativeServingDecision is included. To retrieve the entire creative resource (including the declared fields and the creative content) specify the view as \"FULL\"."]
                            pub view: ::std::option::Option<QueryParametersViewEnum>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Controls the amount of information included in the response. By default only creativeServingDecision is included. To retrieve the entire creative resource (including the declared fields and the creative content) specify the view as \"FULL\"."]
                        pub enum QueryParametersViewEnum {
                            #[serde(rename = "CREATIVE_VIEW_UNSPECIFIED")]
                            #[doc = "Not specified, equivalent to SERVING_DECISION_ONLY."]
                            CreativeViewUnspecified,
                            #[serde(rename = "SERVING_DECISION_ONLY")]
                            #[doc = "Only creativeServingDecision is included in the response."]
                            ServingDecisionOnly,
                            #[serde(rename = "FULL")]
                            #[doc = "The entire creative resource (including the declared fields and the creative content) is included in the response."]
                            Full,
                        }
                        impl ::std::default::Default for QueryParametersViewEnum {
                            fn default() -> Self {
                                Self::CreativeViewUnspecified
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
                            #[serde(rename = "filter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Query string to filter creatives. If no filter is specified, all active creatives will be returned. Example: 'accountId=12345 AND (dealsStatus:DISAPPROVED AND disapprovalReason:UNACCEPTABLE_CONTENT) OR declaredAttributes:IS_COOKIE_TARGETED'"]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Requested page size. The server may return fewer creatives than requested (due to timeout constraint) even if more are available via another call. If unspecified, server will pick an appropriate default. Acceptable values are 1 to 1000, inclusive."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListCreativesResponse.nextPageToken returned from the previous call to the 'ListCreatives' method."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "view")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Controls the amount of information included in the response. By default only creativeServingDecision is included. To retrieve the entire creative resource (including the declared fields and the creative content) specify the view as \"FULL\"."]
                            pub view: ::std::option::Option<QueryParametersViewEnum>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Controls the amount of information included in the response. By default only creativeServingDecision is included. To retrieve the entire creative resource (including the declared fields and the creative content) specify the view as \"FULL\"."]
                        pub enum QueryParametersViewEnum {
                            #[serde(rename = "CREATIVE_VIEW_UNSPECIFIED")]
                            #[doc = "Not specified, equivalent to SERVING_DECISION_ONLY."]
                            CreativeViewUnspecified,
                            #[serde(rename = "SERVING_DECISION_ONLY")]
                            #[doc = "Only creativeServingDecision is included in the response."]
                            ServingDecisionOnly,
                            #[serde(rename = "FULL")]
                            #[doc = "The entire creative resource (including the declared fields and the creative content) is included in the response."]
                            Full,
                        }
                        impl ::std::default::Default for QueryParametersViewEnum {
                            fn default() -> Self {
                                Self::CreativeViewUnspecified
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
                            #[doc = "Field mask to use for partial in-place updates."]
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
            pub mod user_lists {
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
                            #[doc = "The number of results to return per page."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Continuation page token (as received from a previous response)."]
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
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to activate a pretargeting configuration. Sets the configuration's state to ACTIVE."]
    pub struct ActivatePretargetingConfigRequest {}
    impl ActivatePretargetingConfigRequest {
        pub fn builder() -> ActivatePretargetingConfigRequestBuilder {
            ActivatePretargetingConfigRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to start targeting the provided app IDs in a specific pretargeting configuration. The pretargeting configuration itself specifies how these apps are targeted. in PretargetingConfig.appTargeting.mobileAppTargeting."]
    pub struct AddTargetedAppsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of app IDs to target in the pretargeting configuration. These values will be added to the list of targeted app IDs in PretargetingConfig.appTargeting.mobileAppTargeting.values."]
        pub app_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetingMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The targeting mode that should be applied to the list of app IDs. If there are existing targeted app IDs, must be equal to the existing PretargetingConfig.appTargeting.mobileAppTargeting.targetingMode or a 400 bad request error will be returned."]
        pub targeting_mode: ::std::option::Option<AddTargetedAppsRequestTargetingModeEnum>,
    }
    impl AddTargetedAppsRequest {
        pub fn builder() -> AddTargetedAppsRequestBuilder {
            AddTargetedAppsRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The targeting mode that should be applied to the list of app IDs. If there are existing targeted app IDs, must be equal to the existing PretargetingConfig.appTargeting.mobileAppTargeting.targetingMode or a 400 bad request error will be returned."]
    pub enum AddTargetedAppsRequestTargetingModeEnum {
        #[serde(rename = "TARGETING_MODE_UNSPECIFIED")]
        #[doc = "Placeholder for undefined targeting mode."]
        TargetingModeUnspecified,
        #[serde(rename = "INCLUSIVE")]
        #[doc = "The inclusive list type. Inventory must match an item in this list to be targeted."]
        Inclusive,
        #[serde(rename = "EXCLUSIVE")]
        #[doc = "The exclusive list type. Inventory must not match any item in this list to be targeted."]
        Exclusive,
    }
    impl ::std::default::Default for AddTargetedAppsRequestTargetingModeEnum {
        fn default() -> Self {
            Self::TargetingModeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to start targeting the provided publishers in a specific pretargeting configuration. The pretargeting configuration itself specifies how these publishers are targeted in PretargetingConfig.publisherTargeting."]
    pub struct AddTargetedPublishersRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publisherIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of publisher IDs to target in the pretargeting configuration. These values will be added to the list of targeted publisher IDs in PretargetingConfig.publisherTargeting.values. Publishers are identified by their publisher ID from ads.txt / app-ads.txt. See https://iabtechlab.com/ads-txt/ and https://iabtechlab.com/app-ads-txt/ for more details."]
        pub publisher_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetingMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The targeting mode that should be applied to the list of publisher IDs. If are existing publisher IDs, must be equal to the existing PretargetingConfig.publisherTargeting.targetingMode or a 400 bad request error will be returned."]
        pub targeting_mode: ::std::option::Option<AddTargetedPublishersRequestTargetingModeEnum>,
    }
    impl AddTargetedPublishersRequest {
        pub fn builder() -> AddTargetedPublishersRequestBuilder {
            AddTargetedPublishersRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The targeting mode that should be applied to the list of publisher IDs. If are existing publisher IDs, must be equal to the existing PretargetingConfig.publisherTargeting.targetingMode or a 400 bad request error will be returned."]
    pub enum AddTargetedPublishersRequestTargetingModeEnum {
        #[serde(rename = "TARGETING_MODE_UNSPECIFIED")]
        #[doc = "Placeholder for undefined targeting mode."]
        TargetingModeUnspecified,
        #[serde(rename = "INCLUSIVE")]
        #[doc = "The inclusive list type. Inventory must match an item in this list to be targeted."]
        Inclusive,
        #[serde(rename = "EXCLUSIVE")]
        #[doc = "The exclusive list type. Inventory must not match any item in this list to be targeted."]
        Exclusive,
    }
    impl ::std::default::Default for AddTargetedPublishersRequestTargetingModeEnum {
        fn default() -> Self {
            Self::TargetingModeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to start targeting the provided sites in a specific pretargeting configuration. The pretargeting configuration itself specifies how these sites are targeted in PretargetingConfig.webTargeting."]
    pub struct AddTargetedSitesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sites")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of site URLs to target in the pretargeting configuration. These values will be added to the list of targeted URLs in PretargetingConfig.webTargeting.values."]
        pub sites: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetingMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The targeting mode that should be applied to the list of site URLs. If there are existing targeted sites, must be equal to the existing PretargetingConfig.webTargeting.targetingMode or a 400 bad request error will be returned."]
        pub targeting_mode: ::std::option::Option<AddTargetedSitesRequestTargetingModeEnum>,
    }
    impl AddTargetedSitesRequest {
        pub fn builder() -> AddTargetedSitesRequestBuilder {
            AddTargetedSitesRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The targeting mode that should be applied to the list of site URLs. If there are existing targeted sites, must be equal to the existing PretargetingConfig.webTargeting.targetingMode or a 400 bad request error will be returned."]
    pub enum AddTargetedSitesRequestTargetingModeEnum {
        #[serde(rename = "TARGETING_MODE_UNSPECIFIED")]
        #[doc = "Placeholder for undefined targeting mode."]
        TargetingModeUnspecified,
        #[serde(rename = "INCLUSIVE")]
        #[doc = "The inclusive list type. Inventory must match an item in this list to be targeted."]
        Inclusive,
        #[serde(rename = "EXCLUSIVE")]
        #[doc = "The exclusive list type. Inventory must not match any item in this list to be targeted."]
        Exclusive,
    }
    impl ::std::default::Default for AddTargetedSitesRequestTargetingModeEnum {
        fn default() -> Self {
            Self::TargetingModeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected advertiser and brand information."]
    pub struct AdvertiserAndBrand {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "advertiserId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "See https://storage.googleapis.com/adx-rtb-dictionaries/advertisers.txt for the list of possible values. Can be used to filter the response of the creatives.list method."]
        pub advertiser_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "advertiserName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Advertiser name. Can be used to filter the response of the creatives.list method."]
        pub advertiser_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "brandId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected brand ID or zero if no brand has been detected. See https://storage.googleapis.com/adx-rtb-dictionaries/brands.txt for the list of possible values. Can be used to filter the response of the creatives.list method."]
        pub brand_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "brandName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Brand name. Can be used to filter the response of the creatives.list method."]
        pub brand_name: ::std::option::Option<::std::string::String>,
    }
    impl AdvertiserAndBrand {
        pub fn builder() -> AdvertiserAndBrandBuilder {
            AdvertiserAndBrandBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A subset of app inventory to target. Bid requests that match criteria in at least one of the specified dimensions will be sent."]
    pub struct AppTargeting {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mobileAppCategoryTargeting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Lists of included and excluded mobile app categories as defined in https://developers.google.com/adwords/api/docs/appendix/mobileappcategories.csv."]
        pub mobile_app_category_targeting:
            ::std::option::Option<::std::boxed::Box<NumericTargetingDimension>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mobileAppTargeting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Targeted app IDs. App IDs can refer to those found in an app store or ones that are not published in an app store. A maximum of 30,000 app IDs can be targeted."]
        pub mobile_app_targeting:
            ::std::option::Option<::std::boxed::Box<StringTargetingDimension>>,
    }
    impl AppTargeting {
        pub fn builder() -> AppTargetingBuilder {
            AppTargetingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to close a specified user list."]
    pub struct CloseUserListRequest {}
    impl CloseUserListRequest {
        pub fn builder() -> CloseUserListRequestBuilder {
            CloseUserListRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A creative and its classification data."]
    pub struct Creative {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. ID of the buyer account that this creative is owned by. Can be used to filter the response of the creatives.list method with equality and inequality check."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adChoicesDestinationUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The link to AdChoices destination page. This is only supported for native ads."]
        pub ad_choices_destination_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "advertiserName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the company being advertised in the creative. Can be used to filter the response of the creatives.list method."]
        pub advertiser_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "agencyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The agency ID for this creative."]
        pub agency_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiUpdateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The last update timestamp of the creative via API."]
        pub api_update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creativeFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The format of this creative. Can be used to filter the response of the creatives.list method."]
        pub creative_format: ::std::option::Option<CreativeCreativeFormatEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creativeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Buyer-specific creative ID that references this creative in bid responses. This field is Ignored in update operations. Can be used to filter the response of the creatives.list method. The maximum length of the creative ID is 128 bytes."]
        pub creative_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creativeServingDecision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Top level status and detected attributes of a creative (for example domain, language, advertiser, product category, etc.) that affect whether (status) and where (context) a creative will be allowed to serve."]
        pub creative_serving_decision:
            ::std::option::Option<::std::boxed::Box<CreativeServingDecision>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dealIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. IDs of all of the deals with which this creative has been used in bidding. Can be used to filter the response of the creatives.list method."]
        pub deal_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "declaredAttributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All declared attributes for the ads that may be shown from this creative. Can be used to filter the response of the creatives.list method. If the `excluded_attribute` field of a [bid request](https://developers.google.com/authorized-buyers/rtb/downloads/realtime-bidding-proto\") contains one of the attributes that were declared or detected for a given creative, and a bid is submitted with that creative, the bid will be filtered before the auction."]
        pub declared_attributes:
            ::std::option::Option<::std::vec::Vec<CreativeDeclaredAttributesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "declaredClickThroughUrls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of declared destination URLs for the creative. Can be used to filter the response of the creatives.list method."]
        pub declared_click_through_urls:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "declaredRestrictedCategories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All declared restricted categories for the ads that may be shown from this creative. Can be used to filter the response of the creatives.list method."]
        pub declared_restricted_categories:
            ::std::option::Option<::std::vec::Vec<CreativeDeclaredRestrictedCategoriesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "declaredVendorIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "IDs for the declared ad technology vendors that may be used by this creative. See https://storage.googleapis.com/adx-rtb-dictionaries/vendors.txt for possible values. Can be used to filter the response of the creatives.list method."]
        pub declared_vendor_ids: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "html")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An HTML creative."]
        pub html: ::std::option::Option<::std::boxed::Box<HtmlContent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "impressionTrackingUrls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of URLs to be called to record an impression."]
        pub impression_tracking_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Name of the creative. Follows the pattern `buyers/{buyer}/creatives/{creative}`, where `{buyer}` represents the account ID of the buyer who owns the creative, and `{creative}` is the buyer-specific creative ID that references this creative in the bid response."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "native")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A native creative."]
        pub native: ::std::option::Option<::std::boxed::Box<NativeContent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "restrictedCategories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All restricted categories for the ads that may be shown from this creative."]
        pub restricted_categories:
            ::std::option::Option<::std::vec::Vec<CreativeRestrictedCategoriesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The version of this creative. Version for a new creative is 1 and it increments during subsequent creative updates."]
        pub version: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "video")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A video creative."]
        pub video: ::std::option::Option<::std::boxed::Box<VideoContent>>,
    }
    impl Creative {
        pub fn builder() -> CreativeBuilder {
            CreativeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The format of this creative. Can be used to filter the response of the creatives.list method."]
    pub enum CreativeCreativeFormatEnum {
        #[serde(rename = "CREATIVE_FORMAT_UNSPECIFIED")]
        #[doc = "The format is unknown."]
        CreativeFormatUnspecified,
        #[serde(rename = "HTML")]
        #[doc = "HTML creative."]
        Html,
        #[serde(rename = "VIDEO")]
        #[doc = "Video creative."]
        Video,
        #[serde(rename = "NATIVE")]
        #[doc = "Native creative."]
        Native,
    }
    impl ::std::default::Default for CreativeCreativeFormatEnum {
        fn default() -> Self {
            Self::CreativeFormatUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum CreativeDeclaredAttributesEnum {
        #[serde(rename = "ATTRIBUTE_UNSPECIFIED")]
        #[doc = "Do not use. This is a placeholder value only."]
        AttributeUnspecified,
        #[serde(rename = "IMAGE_RICH_MEDIA")]
        #[doc = "The creative is of type image/rich media. For pretargeting."]
        ImageRichMedia,
        #[serde(rename = "ADOBE_FLASH_FLV")]
        #[doc = "The creative is of video type Adobe Flash FLV. For pretargeting."]
        AdobeFlashFlv,
        #[serde(rename = "IS_TAGGED")]
        #[doc = "The creative is tagged."]
        IsTagged,
        #[serde(rename = "IS_COOKIE_TARGETED")]
        #[doc = "The creative is cookie targeted."]
        IsCookieTargeted,
        #[serde(rename = "IS_USER_INTEREST_TARGETED")]
        #[doc = "The creative is user interest targeted."]
        IsUserInterestTargeted,
        #[serde(rename = "EXPANDING_DIRECTION_NONE")]
        #[doc = "The creative does not expand."]
        ExpandingDirectionNone,
        #[serde(rename = "EXPANDING_DIRECTION_UP")]
        #[doc = "The creative expands up."]
        ExpandingDirectionUp,
        #[serde(rename = "EXPANDING_DIRECTION_DOWN")]
        #[doc = "The creative expands down."]
        ExpandingDirectionDown,
        #[serde(rename = "EXPANDING_DIRECTION_LEFT")]
        #[doc = "The creative expands left."]
        ExpandingDirectionLeft,
        #[serde(rename = "EXPANDING_DIRECTION_RIGHT")]
        #[doc = "The creative expands right."]
        ExpandingDirectionRight,
        #[serde(rename = "EXPANDING_DIRECTION_UP_LEFT")]
        #[doc = "The creative expands up and left."]
        ExpandingDirectionUpLeft,
        #[serde(rename = "EXPANDING_DIRECTION_UP_RIGHT")]
        #[doc = "The creative expands up and right."]
        ExpandingDirectionUpRight,
        #[serde(rename = "EXPANDING_DIRECTION_DOWN_LEFT")]
        #[doc = "The creative expands down and left."]
        ExpandingDirectionDownLeft,
        #[serde(rename = "EXPANDING_DIRECTION_DOWN_RIGHT")]
        #[doc = "The creative expands down and right."]
        ExpandingDirectionDownRight,
        #[serde(rename = "CREATIVE_TYPE_HTML")]
        #[doc = "The creative type is HTML."]
        CreativeTypeHtml,
        #[serde(rename = "CREATIVE_TYPE_VAST_VIDEO")]
        #[doc = "The creative type is VAST video."]
        CreativeTypeVastVideo,
        #[serde(rename = "EXPANDING_DIRECTION_UP_OR_DOWN")]
        #[doc = "The creative expands up or down."]
        ExpandingDirectionUpOrDown,
        #[serde(rename = "EXPANDING_DIRECTION_LEFT_OR_RIGHT")]
        #[doc = "The creative expands left or right."]
        ExpandingDirectionLeftOrRight,
        #[serde(rename = "EXPANDING_DIRECTION_ANY_DIAGONAL")]
        #[doc = "The creative expands on any diagonal."]
        ExpandingDirectionAnyDiagonal,
        #[serde(rename = "EXPANDING_ACTION_ROLLOVER_TO_EXPAND")]
        #[doc = "The creative expands when rolled over."]
        ExpandingActionRolloverToExpand,
        #[serde(rename = "INSTREAM_VAST_VIDEO_TYPE_VPAID_FLASH")]
        #[doc = "The instream vast video type is vpaid flash."]
        InstreamVastVideoTypeVpaidFlash,
        #[serde(rename = "RICH_MEDIA_CAPABILITY_TYPE_MRAID")]
        #[doc = "The creative is MRAID."]
        RichMediaCapabilityTypeMraid,
        #[serde(rename = "RICH_MEDIA_CAPABILITY_TYPE_FLASH")]
        #[doc = "The creative is Flash."]
        RichMediaCapabilityTypeFlash,
        #[serde(rename = "RICH_MEDIA_CAPABILITY_TYPE_HTML5")]
        #[doc = "The creative is HTML5."]
        RichMediaCapabilityTypeHtml5,
        #[serde(rename = "SKIPPABLE_INSTREAM_VIDEO")]
        #[doc = "The creative has an instream VAST video type of skippable instream video. For pretargeting."]
        SkippableInstreamVideo,
        #[serde(rename = "RICH_MEDIA_CAPABILITY_TYPE_SSL")]
        #[doc = "The creative is SSL."]
        RichMediaCapabilityTypeSsl,
        #[serde(rename = "RICH_MEDIA_CAPABILITY_TYPE_NON_SSL")]
        #[doc = "The creative is non-SSL."]
        RichMediaCapabilityTypeNonSsl,
        #[serde(rename = "RICH_MEDIA_CAPABILITY_TYPE_INTERSTITIAL")]
        #[doc = "The creative is an interstitial."]
        RichMediaCapabilityTypeInterstitial,
        #[serde(rename = "NON_SKIPPABLE_INSTREAM_VIDEO")]
        #[doc = "The creative has an instream VAST video type of non-skippable instream video. For pretargeting."]
        NonSkippableInstreamVideo,
        #[serde(rename = "NATIVE_ELIGIBILITY_ELIGIBLE")]
        #[doc = "The creative is eligible for native."]
        NativeEligibilityEligible,
        #[serde(rename = "NON_VPAID")]
        #[doc = "The creative has an instream VAST video type of non-VPAID. For pretargeting."]
        NonVpaid,
        #[serde(rename = "NATIVE_ELIGIBILITY_NOT_ELIGIBLE")]
        #[doc = "The creative is not eligible for native."]
        NativeEligibilityNotEligible,
        #[serde(rename = "ANY_INTERSTITIAL")]
        #[doc = "The creative has an interstitial size of any interstitial. For pretargeting."]
        AnyInterstitial,
        #[serde(rename = "NON_INTERSTITIAL")]
        #[doc = "The creative has an interstitial size of non interstitial. For pretargeting."]
        NonInterstitial,
        #[serde(rename = "IN_BANNER_VIDEO")]
        #[doc = "The video type is in-banner video."]
        InBannerVideo,
        #[serde(rename = "RENDERING_SIZELESS_ADX")]
        #[doc = "The creative can dynamically resize to fill a variety of slot sizes."]
        RenderingSizelessAdx,
        #[serde(rename = "OMSDK_1_0")]
        #[doc = "The open measurement SDK is supported."]
        Omsdk10,
    }
    impl ::std::default::Default for CreativeDeclaredAttributesEnum {
        fn default() -> Self {
            Self::AttributeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum CreativeDeclaredRestrictedCategoriesEnum {
        #[serde(rename = "RESTRICTED_CATEGORY_UNSPECIFIED")]
        #[doc = "Default value that should never be used."]
        RestrictedCategoryUnspecified,
        #[serde(rename = "ALCOHOL")]
        #[doc = "The alcohol restricted category."]
        Alcohol,
    }
    impl ::std::default::Default for CreativeDeclaredRestrictedCategoriesEnum {
        fn default() -> Self {
            Self::RestrictedCategoryUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum CreativeRestrictedCategoriesEnum {
        #[serde(rename = "RESTRICTED_CATEGORY_UNSPECIFIED")]
        #[doc = "Default value that should never be used."]
        RestrictedCategoryUnspecified,
        #[serde(rename = "ALCOHOL")]
        #[doc = "The alcohol restricted category."]
        Alcohol,
    }
    impl ::std::default::Default for CreativeRestrictedCategoriesEnum {
        fn default() -> Self {
            Self::RestrictedCategoryUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The dimensions of a creative. This applies to only HTML and Native creatives."]
    pub struct CreativeDimensions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The height of the creative in pixels."]
        pub height: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The width of the creative in pixels."]
        pub width: ::std::option::Option<::std::string::String>,
    }
    impl CreativeDimensions {
        pub fn builder() -> CreativeDimensionsBuilder {
            CreativeDimensionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Top level status and detected attributes of a creative."]
    pub struct CreativeServingDecision {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "chinaPolicyCompliance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The policy compliance of this creative in China. When approved or disapproved, this applies to both deals and open auction in China. When pending review, this creative is allowed to serve for deals but not for open auction."]
        pub china_policy_compliance: ::std::option::Option<::std::boxed::Box<PolicyCompliance>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dealsPolicyCompliance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Policy compliance of this creative when bidding on Programmatic Guaranteed and Preferred Deals (outside of Russia and China)."]
        pub deals_policy_compliance: ::std::option::Option<::std::boxed::Box<PolicyCompliance>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedAdvertisers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected advertisers and brands."]
        pub detected_advertisers:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AdvertiserAndBrand>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedAttributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Publisher-excludable attributes that were detected for this creative. Can be used to filter the response of the creatives.list method. If the `excluded_attribute` field of a [bid request](https://developers.google.com/authorized-buyers/rtb/downloads/realtime-bidding-proto) contains one of the attributes that were declared or detected for a given creative, and a bid is submitted with that creative, the bid will be filtered before the auction."]
        pub detected_attributes:
            ::std::option::Option<::std::vec::Vec<CreativeServingDecisionDetectedAttributesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedClickThroughUrls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of detected destination URLs for the creative. Can be used to filter the response of the creatives.list method."]
        pub detected_click_through_urls:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedDomains")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The detected domains for this creative."]
        pub detected_domains: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The detected languages for this creative. The order is arbitrary. The codes are 2 or 5 characters and are documented at https://developers.google.com/adwords/api/docs/appendix/languagecodes. Can be used to filter the response of the creatives.list method."]
        pub detected_languages: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedProductCategories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected product categories, if any. See the ad-product-categories.txt file in the technical documentation for a list of IDs. Can be used to filter the response of the creatives.list method."]
        pub detected_product_categories:
            ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedSensitiveCategories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected sensitive categories, if any. Can be used to filter the response of the creatives.list method. See the ad-sensitive-categories.txt file in the technical documentation for a list of IDs. You should use these IDs along with the excluded-sensitive-category field in the bid request to filter your bids."]
        pub detected_sensitive_categories:
            ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedVendorIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "IDs of the ad technology vendors that were detected to be used by this creative. See https://storage.googleapis.com/adx-rtb-dictionaries/vendors.txt for possible values. Can be used to filter the response of the creatives.list method. If the `allowed_vendor_type` field of a [bid request](https://developers.google.com/authorized-buyers/rtb/downloads/realtime-bidding-proto) does not contain one of the vendor type IDs that were declared or detected for a given creative, and a bid is submitted with that creative, the bid will be filtered before the auction."]
        pub detected_vendor_ids: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastStatusUpdate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last time the creative status was updated. Can be used to filter the response of the creatives.list method."]
        pub last_status_update: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "networkPolicyCompliance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Policy compliance of this creative when bidding in open auction, private auction, or auction packages (outside of Russia and China)."]
        pub network_policy_compliance: ::std::option::Option<::std::boxed::Box<PolicyCompliance>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "platformPolicyCompliance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Policy compliance of this creative when bidding in Open Bidding (outside of Russia and China). For the list of platform policies, see: https://support.google.com/platformspolicy/answer/3013851."]
        pub platform_policy_compliance: ::std::option::Option<::std::boxed::Box<PolicyCompliance>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "russiaPolicyCompliance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The policy compliance of this creative in Russia. When approved or disapproved, this applies to both deals and open auction in Russia. When pending review, this creative is allowed to serve for deals but not for open auction."]
        pub russia_policy_compliance: ::std::option::Option<::std::boxed::Box<PolicyCompliance>>,
    }
    impl CreativeServingDecision {
        pub fn builder() -> CreativeServingDecisionBuilder {
            CreativeServingDecisionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum CreativeServingDecisionDetectedAttributesEnum {
        #[serde(rename = "ATTRIBUTE_UNSPECIFIED")]
        #[doc = "Do not use. This is a placeholder value only."]
        AttributeUnspecified,
        #[serde(rename = "IMAGE_RICH_MEDIA")]
        #[doc = "The creative is of type image/rich media. For pretargeting."]
        ImageRichMedia,
        #[serde(rename = "ADOBE_FLASH_FLV")]
        #[doc = "The creative is of video type Adobe Flash FLV. For pretargeting."]
        AdobeFlashFlv,
        #[serde(rename = "IS_TAGGED")]
        #[doc = "The creative is tagged."]
        IsTagged,
        #[serde(rename = "IS_COOKIE_TARGETED")]
        #[doc = "The creative is cookie targeted."]
        IsCookieTargeted,
        #[serde(rename = "IS_USER_INTEREST_TARGETED")]
        #[doc = "The creative is user interest targeted."]
        IsUserInterestTargeted,
        #[serde(rename = "EXPANDING_DIRECTION_NONE")]
        #[doc = "The creative does not expand."]
        ExpandingDirectionNone,
        #[serde(rename = "EXPANDING_DIRECTION_UP")]
        #[doc = "The creative expands up."]
        ExpandingDirectionUp,
        #[serde(rename = "EXPANDING_DIRECTION_DOWN")]
        #[doc = "The creative expands down."]
        ExpandingDirectionDown,
        #[serde(rename = "EXPANDING_DIRECTION_LEFT")]
        #[doc = "The creative expands left."]
        ExpandingDirectionLeft,
        #[serde(rename = "EXPANDING_DIRECTION_RIGHT")]
        #[doc = "The creative expands right."]
        ExpandingDirectionRight,
        #[serde(rename = "EXPANDING_DIRECTION_UP_LEFT")]
        #[doc = "The creative expands up and left."]
        ExpandingDirectionUpLeft,
        #[serde(rename = "EXPANDING_DIRECTION_UP_RIGHT")]
        #[doc = "The creative expands up and right."]
        ExpandingDirectionUpRight,
        #[serde(rename = "EXPANDING_DIRECTION_DOWN_LEFT")]
        #[doc = "The creative expands down and left."]
        ExpandingDirectionDownLeft,
        #[serde(rename = "EXPANDING_DIRECTION_DOWN_RIGHT")]
        #[doc = "The creative expands down and right."]
        ExpandingDirectionDownRight,
        #[serde(rename = "CREATIVE_TYPE_HTML")]
        #[doc = "The creative type is HTML."]
        CreativeTypeHtml,
        #[serde(rename = "CREATIVE_TYPE_VAST_VIDEO")]
        #[doc = "The creative type is VAST video."]
        CreativeTypeVastVideo,
        #[serde(rename = "EXPANDING_DIRECTION_UP_OR_DOWN")]
        #[doc = "The creative expands up or down."]
        ExpandingDirectionUpOrDown,
        #[serde(rename = "EXPANDING_DIRECTION_LEFT_OR_RIGHT")]
        #[doc = "The creative expands left or right."]
        ExpandingDirectionLeftOrRight,
        #[serde(rename = "EXPANDING_DIRECTION_ANY_DIAGONAL")]
        #[doc = "The creative expands on any diagonal."]
        ExpandingDirectionAnyDiagonal,
        #[serde(rename = "EXPANDING_ACTION_ROLLOVER_TO_EXPAND")]
        #[doc = "The creative expands when rolled over."]
        ExpandingActionRolloverToExpand,
        #[serde(rename = "INSTREAM_VAST_VIDEO_TYPE_VPAID_FLASH")]
        #[doc = "The instream vast video type is vpaid flash."]
        InstreamVastVideoTypeVpaidFlash,
        #[serde(rename = "RICH_MEDIA_CAPABILITY_TYPE_MRAID")]
        #[doc = "The creative is MRAID."]
        RichMediaCapabilityTypeMraid,
        #[serde(rename = "RICH_MEDIA_CAPABILITY_TYPE_FLASH")]
        #[doc = "The creative is Flash."]
        RichMediaCapabilityTypeFlash,
        #[serde(rename = "RICH_MEDIA_CAPABILITY_TYPE_HTML5")]
        #[doc = "The creative is HTML5."]
        RichMediaCapabilityTypeHtml5,
        #[serde(rename = "SKIPPABLE_INSTREAM_VIDEO")]
        #[doc = "The creative has an instream VAST video type of skippable instream video. For pretargeting."]
        SkippableInstreamVideo,
        #[serde(rename = "RICH_MEDIA_CAPABILITY_TYPE_SSL")]
        #[doc = "The creative is SSL."]
        RichMediaCapabilityTypeSsl,
        #[serde(rename = "RICH_MEDIA_CAPABILITY_TYPE_NON_SSL")]
        #[doc = "The creative is non-SSL."]
        RichMediaCapabilityTypeNonSsl,
        #[serde(rename = "RICH_MEDIA_CAPABILITY_TYPE_INTERSTITIAL")]
        #[doc = "The creative is an interstitial."]
        RichMediaCapabilityTypeInterstitial,
        #[serde(rename = "NON_SKIPPABLE_INSTREAM_VIDEO")]
        #[doc = "The creative has an instream VAST video type of non-skippable instream video. For pretargeting."]
        NonSkippableInstreamVideo,
        #[serde(rename = "NATIVE_ELIGIBILITY_ELIGIBLE")]
        #[doc = "The creative is eligible for native."]
        NativeEligibilityEligible,
        #[serde(rename = "NON_VPAID")]
        #[doc = "The creative has an instream VAST video type of non-VPAID. For pretargeting."]
        NonVpaid,
        #[serde(rename = "NATIVE_ELIGIBILITY_NOT_ELIGIBLE")]
        #[doc = "The creative is not eligible for native."]
        NativeEligibilityNotEligible,
        #[serde(rename = "ANY_INTERSTITIAL")]
        #[doc = "The creative has an interstitial size of any interstitial. For pretargeting."]
        AnyInterstitial,
        #[serde(rename = "NON_INTERSTITIAL")]
        #[doc = "The creative has an interstitial size of non interstitial. For pretargeting."]
        NonInterstitial,
        #[serde(rename = "IN_BANNER_VIDEO")]
        #[doc = "The video type is in-banner video."]
        InBannerVideo,
        #[serde(rename = "RENDERING_SIZELESS_ADX")]
        #[doc = "The creative can dynamically resize to fill a variety of slot sizes."]
        RenderingSizelessAdx,
        #[serde(rename = "OMSDK_1_0")]
        #[doc = "The open measurement SDK is supported."]
        Omsdk10,
    }
    impl ::std::default::Default for CreativeServingDecisionDetectedAttributesEnum {
        fn default() -> Self {
            Self::AttributeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values * A month and day value, with a zero year, such as an anniversary * A year on its own, with zero month and day values * A year and month value, with a zero day, such as a credit card expiration date Related types are google.type.TimeOfDay and `google.protobuf.Timestamp`."]
    pub struct Date {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "day")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
        pub day: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "month")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
        pub month: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "year")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
        pub year: ::std::option::Option<::std::primitive::i64>,
    }
    impl Date {
        pub fn builder() -> DateBuilder {
            DateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Evidence that the creative's destination URL was not crawlable by Google."]
    pub struct DestinationNotCrawlableEvidence {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "crawlTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Approximate time of the crawl."]
        pub crawl_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "crawledUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Destination URL that was attempted to be crawled."]
        pub crawled_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reason of destination not crawlable."]
        pub reason: ::std::option::Option<DestinationNotCrawlableEvidenceReasonEnum>,
    }
    impl DestinationNotCrawlableEvidence {
        pub fn builder() -> DestinationNotCrawlableEvidenceBuilder {
            DestinationNotCrawlableEvidenceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Reason of destination not crawlable."]
    pub enum DestinationNotCrawlableEvidenceReasonEnum {
        #[serde(rename = "REASON_UNSPECIFIED")]
        #[doc = "Default value that should never be used."]
        ReasonUnspecified,
        #[serde(rename = "UNREACHABLE_ROBOTS")]
        #[doc = "Site's robots exclusion file (e.g., robots.txt) was unreachable."]
        UnreachableRobots,
        #[serde(rename = "TIMEOUT_ROBOTS")]
        #[doc = "Timed out reading site's robots exclusion file (e.g., robots.txt)."]
        TimeoutRobots,
        #[serde(rename = "ROBOTED_DENIED")]
        #[doc = "Crawler was disallowed by the site's robots exclusion file (e.g., robots.txt)."]
        RobotedDenied,
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown reason."]
        Unknown,
    }
    impl ::std::default::Default for DestinationNotCrawlableEvidenceReasonEnum {
        fn default() -> Self {
            Self::ReasonUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Evidence of the creative's destination URL not functioning properly or having been incorrectly set up."]
    pub struct DestinationNotWorkingEvidence {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dnsError")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DNS lookup errors."]
        pub dns_error: ::std::option::Option<DestinationNotWorkingEvidenceDnsErrorEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expandedUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full non-working URL."]
        pub expanded_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "httpError")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "HTTP error code (e.g. 404 or 5xx)"]
        pub http_error: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "invalidPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page was crawled successfully, but was detected as either a page with no content or an error page."]
        pub invalid_page: ::std::option::Option<DestinationNotWorkingEvidenceInvalidPageEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastCheckTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Approximate time when the ad destination was last checked."]
        pub last_check_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "platform")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Platform of the non-working URL."]
        pub platform: ::std::option::Option<DestinationNotWorkingEvidencePlatformEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "redirectionError")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "HTTP redirect chain error."]
        pub redirection_error:
            ::std::option::Option<DestinationNotWorkingEvidenceRedirectionErrorEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "urlRejected")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rejected because of malformed URLs or invalid requests."]
        pub url_rejected: ::std::option::Option<DestinationNotWorkingEvidenceUrlRejectedEnum>,
    }
    impl DestinationNotWorkingEvidence {
        pub fn builder() -> DestinationNotWorkingEvidenceBuilder {
            DestinationNotWorkingEvidenceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "DNS lookup errors."]
    pub enum DestinationNotWorkingEvidenceDnsErrorEnum {
        #[serde(rename = "DNS_ERROR_UNSPECIFIED")]
        #[doc = "Default value that should never be used."]
        DnsErrorUnspecified,
        #[serde(rename = "ERROR_DNS")]
        #[doc = "DNS name was not found."]
        ErrorDns,
        #[serde(rename = "GOOGLE_CRAWLER_DNS_ISSUE")]
        #[doc = "An internal issue occurred when Google's crawler tried to resolve the DNS entry. This is a Google-internal issue and may not be the result of an issue with the landing page."]
        GoogleCrawlerDnsIssue,
    }
    impl ::std::default::Default for DestinationNotWorkingEvidenceDnsErrorEnum {
        fn default() -> Self {
            Self::DnsErrorUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Page was crawled successfully, but was detected as either a page with no content or an error page."]
    pub enum DestinationNotWorkingEvidenceInvalidPageEnum {
        #[serde(rename = "INVALID_PAGE_UNSPECIFIED")]
        #[doc = "Default value that should never be used."]
        InvalidPageUnspecified,
        #[serde(rename = "EMPTY_OR_ERROR_PAGE")]
        #[doc = "Page was empty or had an error."]
        EmptyOrErrorPage,
    }
    impl ::std::default::Default for DestinationNotWorkingEvidenceInvalidPageEnum {
        fn default() -> Self {
            Self::InvalidPageUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Platform of the non-working URL."]
    pub enum DestinationNotWorkingEvidencePlatformEnum {
        #[serde(rename = "PLATFORM_UNSPECIFIED")]
        #[doc = "Default value that should never be used."]
        PlatformUnspecified,
        #[serde(rename = "PERSONAL_COMPUTER")]
        #[doc = "The personal computer platform."]
        PersonalComputer,
        #[serde(rename = "ANDROID")]
        #[doc = "The Android platform."]
        Android,
        #[serde(rename = "IOS")]
        #[doc = "The iOS platform."]
        Ios,
    }
    impl ::std::default::Default for DestinationNotWorkingEvidencePlatformEnum {
        fn default() -> Self {
            Self::PlatformUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "HTTP redirect chain error."]
    pub enum DestinationNotWorkingEvidenceRedirectionErrorEnum {
        #[serde(rename = "REDIRECTION_ERROR_UNSPECIFIED")]
        #[doc = "Default value that should never be used."]
        RedirectionErrorUnspecified,
        #[serde(rename = "TOO_MANY_REDIRECTS")]
        #[doc = "Too many redirect hops."]
        TooManyRedirects,
        #[serde(rename = "INVALID_REDIRECT")]
        #[doc = "Got a redirect but it was invalid."]
        InvalidRedirect,
        #[serde(rename = "EMPTY_REDIRECT")]
        #[doc = "Got a redirect but it was empty."]
        EmptyRedirect,
        #[serde(rename = "REDIRECT_ERROR_UNKNOWN")]
        #[doc = "Unknown redirect error."]
        RedirectErrorUnknown,
    }
    impl ::std::default::Default for DestinationNotWorkingEvidenceRedirectionErrorEnum {
        fn default() -> Self {
            Self::RedirectionErrorUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Rejected because of malformed URLs or invalid requests."]
    pub enum DestinationNotWorkingEvidenceUrlRejectedEnum {
        #[serde(rename = "URL_REJECTED_UNSPECIFIED")]
        #[doc = "Default value that should never be used."]
        UrlRejectedUnspecified,
        #[serde(rename = "BAD_REQUEST")]
        #[doc = "URL rejected because of a malformed request."]
        BadRequest,
        #[serde(rename = "MALFORMED_URL")]
        #[doc = "URL rejected because of a malformed URL."]
        MalformedUrl,
        #[serde(rename = "URL_REJECTED_UNKNOWN")]
        #[doc = "URL rejected because of unknown reason."]
        UrlRejectedUnknown,
    }
    impl ::std::default::Default for DestinationNotWorkingEvidenceUrlRejectedEnum {
        fn default() -> Self {
            Self::UrlRejectedUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The full landing page URL of the destination."]
    pub struct DestinationUrlEvidence {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full landing page URL of the destination."]
        pub destination_url: ::std::option::Option<::std::string::String>,
    }
    impl DestinationUrlEvidence {
        pub fn builder() -> DestinationUrlEvidenceBuilder {
            DestinationUrlEvidenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Number of HTTP calls made by a creative, broken down by domain."]
    pub struct DomainCallEvidence {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topHttpCallDomains")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Breakdown of the most frequent domains called via HTTP by the creative."]
        pub top_http_call_domains:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DomainCalls>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalHttpCallCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of HTTP calls made by the creative, including but not limited to the number of calls in the top_http_call_domains."]
        pub total_http_call_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl DomainCallEvidence {
        pub fn builder() -> DomainCallEvidenceBuilder {
            DomainCallEvidenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The number of HTTP calls made to the given domain."]
    pub struct DomainCalls {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The domain name."]
        pub domain: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "httpCallCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of HTTP calls made to the domain."]
        pub http_call_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl DomainCalls {
        pub fn builder() -> DomainCallsBuilder {
            DomainCallsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Total download size and URL-level download size breakdown for resources in a creative."]
    pub struct DownloadSizeEvidence {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topUrlDownloadSizeBreakdowns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Download size broken down by URLs with the top download size."]
        pub top_url_download_size_breakdowns:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UrlDownloadSize>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalDownloadSizeKb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total download size (in kilobytes) for all the resources in the creative."]
        pub total_download_size_kb: ::std::option::Option<::std::primitive::i64>,
    }
    impl DownloadSizeEvidence {
        pub fn builder() -> DownloadSizeEvidenceBuilder {
            DownloadSizeEvidenceBuilder::default()
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
    #[doc = "Response for a request to get remarketing tag."]
    pub struct GetRemarketingTagResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A HTML tag that can be placed on the advertiser's page to add users to a user list. For more information and code samples on using snippet on your website refer to [Tag your site for remarketing]( https://support.google.com/google-ads/answer/2476688)."]
        pub snippet: ::std::option::Option<::std::string::String>,
    }
    impl GetRemarketingTagResponse {
        pub fn builder() -> GetRemarketingTagResponseBuilder {
            GetRemarketingTagResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "HTML content for a creative."]
    pub struct HtmlContent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The height of the HTML snippet in pixels. Can be used to filter the response of the creatives.list method."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The HTML snippet that displays the ad when inserted in the web page."]
        pub snippet: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The width of the HTML snippet in pixels. Can be used to filter the response of the creatives.list method."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl HtmlContent {
        pub fn builder() -> HtmlContentBuilder {
            HtmlContentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "HTTP calls made by a creative that resulted in policy violations."]
    pub struct HttpCallEvidence {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "urls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URLs of HTTP calls made by the creative."]
        pub urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl HttpCallEvidence {
        pub fn builder() -> HttpCallEvidenceBuilder {
            HttpCallEvidenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Evidence for HTTP cookie-related policy violations."]
    pub struct HttpCookieEvidence {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cookieNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Names of cookies that violate Google policies. For TOO_MANY_COOKIES policy, this will be the cookie names of top domains with the largest number of cookies. For other policies, this will be all the cookie names that violate the policy."]
        pub cookie_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxCookieCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The largest number of cookies set by a creative. If this field is set, cookie_names above will be set to the cookie names of top domains with the largest number of cookies. This field will only be set for TOO_MANY_COOKIES policy."]
        pub max_cookie_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl HttpCookieEvidence {
        pub fn builder() -> HttpCookieEvidenceBuilder {
            HttpCookieEvidenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An image resource. You may provide a larger image than was requested, so long as the aspect ratio is preserved."]
    pub struct Image {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image height in pixels."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the image."]
        pub url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image width in pixels."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl Image {
        pub fn builder() -> ImageBuilder {
            ImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A response for listing creatives."]
    pub struct ListCreativesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creatives")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of creatives."]
        pub creatives: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Creative>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results. Pass this value in the ListCreativesRequest.pageToken field in the subsequent call to the `ListCreatives` method to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListCreativesResponse {
        pub fn builder() -> ListCreativesResponseBuilder {
            ListCreativesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A response containing pretargeting configurations."]
    pub struct ListPretargetingConfigsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token which can be passed to a subsequent call to the `ListPretargetingConfigs` method to retrieve the next page of results in ListPretargetingConfigsRequest.pageToken."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pretargetingConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of pretargeting configurations."]
        pub pretargeting_configs:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PretargetingConfig>>>,
    }
    impl ListPretargetingConfigsResponse {
        pub fn builder() -> ListPretargetingConfigsResponseBuilder {
            ListPretargetingConfigsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The list user list response."]
    pub struct ListUserListsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The continuation page token to send back to the server in a subsequent request. Due to a currently known issue, it is recommended that the caller keep invoking the list method till the time a next page token is not returned (even if the result set is empty)."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userLists")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of user lists from the search."]
        pub user_lists: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UserList>>>,
    }
    impl ListUserListsResponse {
        pub fn builder() -> ListUserListsResponseBuilder {
            ListUserListsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about each media file in the VAST."]
    pub struct MediaFile {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bitrate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Bitrate of the video file, in Kbps. Can be used to filter the response of the creatives.list method."]
        pub bitrate: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The MIME type of this media file. Can be used to filter the response of the creatives.list method."]
        pub mime_type: ::std::option::Option<MediaFileMimeTypeEnum>,
    }
    impl MediaFile {
        pub fn builder() -> MediaFileBuilder {
            MediaFileBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The MIME type of this media file. Can be used to filter the response of the creatives.list method."]
    pub enum MediaFileMimeTypeEnum {
        #[serde(rename = "VIDEO_MIME_TYPE_UNSPECIFIED")]
        #[doc = "Default value that should never be used."]
        VideoMimeTypeUnspecified,
        #[serde(rename = "MIME_VIDEO_XFLV")]
        #[doc = "Flash container."]
        MimeVideoXflv,
        #[serde(rename = "MIME_VIDEO_WEBM")]
        #[doc = "WebM container assuming VP9 codec."]
        MimeVideoWebm,
        #[serde(rename = "MIME_VIDEO_MP4")]
        #[doc = "MPEG-4 container typically with H.264 codec."]
        MimeVideoMp4,
        #[serde(rename = "MIME_VIDEO_OGG")]
        #[doc = "Ogg container assuming Theora codec."]
        MimeVideoOgg,
        #[serde(rename = "MIME_VIDEO_YT_HOSTED")]
        #[doc = "Video files hosted on YouTube."]
        MimeVideoYtHosted,
        #[serde(rename = "MIME_VIDEO_X_MS_WMV")]
        #[doc = "Windows Media Video Codec."]
        MimeVideoXMsWmv,
        #[serde(rename = "MIME_VIDEO_3GPP")]
        #[doc = "3GPP container format used on 3G phones."]
        MimeVideo3Gpp,
        #[serde(rename = "MIME_VIDEO_MOV")]
        #[doc = "Quicktime container format."]
        MimeVideoMov,
        #[serde(rename = "MIME_APPLICATION_SWF")]
        #[doc = "Shockwave Flash (used for VPAID ads)."]
        MimeApplicationSwf,
        #[serde(rename = "MIME_APPLICATION_SURVEY")]
        #[doc = "Properties of VAST served by consumer survey."]
        MimeApplicationSurvey,
        #[serde(rename = "MIME_APPLICATION_JAVASCRIPT")]
        #[doc = "JavaScript (used for VPAID ads)."]
        MimeApplicationJavascript,
        #[serde(rename = "MIME_APPLICATION_SILVERLIGHT")]
        #[doc = "Silverlight (used for VPAID ads)."]
        MimeApplicationSilverlight,
        #[serde(rename = "MIME_APPLICATION_MPEGURL")]
        #[doc = "HLS/M3U8."]
        MimeApplicationMpegurl,
        #[serde(rename = "MIME_APPLICATION_MPEGDASH")]
        #[doc = "DASH."]
        MimeApplicationMpegdash,
        #[serde(rename = "MIME_AUDIO_MP4A")]
        #[doc = "MPEG-4 audio format."]
        MimeAudioMp4A,
        #[serde(rename = "MIME_AUDIO_MP3")]
        #[doc = "MPEG-3 audio format."]
        MimeAudioMp3,
        #[serde(rename = "MIME_AUDIO_OGG")]
        #[doc = "Ogg audio format"]
        MimeAudioOgg,
    }
    impl ::std::default::Default for MediaFileMimeTypeEnum {
        fn default() -> Self {
            Self::VideoMimeTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Native content for a creative."]
    pub struct NativeContent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "advertiserName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the advertiser or sponsor, to be displayed in the ad creative."]
        pub advertiser_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appIcon")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The app icon, for app download ads."]
        pub app_icon: ::std::option::Option<::std::boxed::Box<Image>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "body")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A long description of the ad."]
        pub body: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "callToAction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A label for the button that the user is supposed to click."]
        pub call_to_action: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clickLinkUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL that the browser/SDK will load when the user clicks the ad."]
        pub click_link_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clickTrackingUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL to use for click tracking."]
        pub click_tracking_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headline")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short title for the ad."]
        pub headline: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A large image."]
        pub image: ::std::option::Option<::std::boxed::Box<Image>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A smaller image, for the advertiser's logo."]
        pub logo: ::std::option::Option<::std::boxed::Box<Image>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "priceDisplayText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The price of the promoted app including currency info."]
        pub price_display_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "starRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The app rating in the app store. Must be in the range [0-5]."]
        pub star_rating: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL to fetch a native video ad."]
        pub video_url: ::std::option::Option<::std::string::String>,
    }
    impl NativeContent {
        pub fn builder() -> NativeContentBuilder {
            NativeContentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Generic targeting used for targeting dimensions that contain a list of included and excluded numeric IDs used in app, user list, geo, and vertical id targeting."]
    pub struct NumericTargetingDimension {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludedIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IDs excluded in a configuration."]
        pub excluded_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includedIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IDs included in a configuration."]
        pub included_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl NumericTargetingDimension {
        pub fn builder() -> NumericTargetingDimensionBuilder {
            NumericTargetingDimensionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to open a specified user list."]
    pub struct OpenUserListRequest {}
    impl OpenUserListRequest {
        pub fn builder() -> OpenUserListRequestBuilder {
            OpenUserListRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Policy compliance of the creative for a transaction type or a region."]
    pub struct PolicyCompliance {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serving status for the given transaction type (e.g., open auction, deals) or region (e.g., China, Russia). Can be used to filter the response of the creatives.list method."]
        pub status: ::std::option::Option<PolicyComplianceStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Topics related to the policy compliance for this transaction type (e.g., open auction, deals) or region (e.g., China, Russia). Topics may be present only if status is DISAPPROVED."]
        pub topics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PolicyTopicEntry>>>,
    }
    impl PolicyCompliance {
        pub fn builder() -> PolicyComplianceBuilder {
            PolicyComplianceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Serving status for the given transaction type (e.g., open auction, deals) or region (e.g., China, Russia). Can be used to filter the response of the creatives.list method."]
    pub enum PolicyComplianceStatusEnum {
        #[serde(rename = "STATUS_UNSPECIFIED")]
        #[doc = "Default value that should never be used."]
        StatusUnspecified,
        #[serde(rename = "PENDING_REVIEW")]
        #[doc = "Creative is pending review."]
        PendingReview,
        #[serde(rename = "DISAPPROVED")]
        #[doc = "Creative cannot serve."]
        Disapproved,
        #[serde(rename = "APPROVED")]
        #[doc = "Creative is approved."]
        Approved,
    }
    impl ::std::default::Default for PolicyComplianceStatusEnum {
        fn default() -> Self {
            Self::StatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Each policy topic entry will represent a violation of a policy topic for a creative, with the policy topic information and optional evidence for the policy violation."]
    pub struct PolicyTopicEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "evidences")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pieces of evidence associated with this policy topic entry."]
        pub evidences:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PolicyTopicEvidence>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "helpCenterUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL of the help center article describing this policy topic."]
        pub help_center_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policyTopic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Policy topic this entry refers to. For example, \"ALCOHOL\", \"TRADEMARKS_IN_AD_TEXT\", or \"DESTINATION_NOT_WORKING\". The set of possible policy topics is not fixed for a particular API version and may change at any time. Can be used to filter the response of the creatives.list method"]
        pub policy_topic: ::std::option::Option<::std::string::String>,
    }
    impl PolicyTopicEntry {
        pub fn builder() -> PolicyTopicEntryBuilder {
            PolicyTopicEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Evidence associated with a policy topic entry."]
    pub struct PolicyTopicEvidence {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationNotCrawlable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creative's destination URL was not crawlable by Google."]
        pub destination_not_crawlable:
            ::std::option::Option<::std::boxed::Box<DestinationNotCrawlableEvidence>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationNotWorking")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creative's destination URL did not function properly or was incorrectly set up."]
        pub destination_not_working:
            ::std::option::Option<::std::boxed::Box<DestinationNotWorkingEvidence>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL of the actual landing page."]
        pub destination_url: ::std::option::Option<::std::boxed::Box<DestinationUrlEvidence>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domainCall")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of HTTP calls made by the creative, broken down by domain."]
        pub domain_call: ::std::option::Option<::std::boxed::Box<DomainCallEvidence>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "downloadSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total download size and URL-level download size breakdown for resources in a creative."]
        pub download_size: ::std::option::Option<::std::boxed::Box<DownloadSizeEvidence>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "httpCall")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "HTTP calls made by the creative that resulted in policy violations."]
        pub http_call: ::std::option::Option<::std::boxed::Box<HttpCallEvidence>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "httpCookie")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Evidence for HTTP cookie-related policy violations."]
        pub http_cookie: ::std::option::Option<::std::boxed::Box<HttpCookieEvidence>>,
    }
    impl PolicyTopicEvidence {
        pub fn builder() -> PolicyTopicEvidenceBuilder {
            PolicyTopicEvidenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Pretargeting configuration: a set of targeting dimensions applied at the pretargeting stage of the RTB funnel. These control which inventory a bidder will receive bid requests for."]
    pub struct PretargetingConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowedUserTargetingModes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Targeting modes included by this configuration. A bid request must allow all the specified targeting modes. An unset value allows all bid requests to be sent, regardless of which targeting modes they allow."]
        pub allowed_user_targeting_modes:
            ::std::option::Option<::std::vec::Vec<PretargetingConfigAllowedUserTargetingModesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appTargeting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Targeting on a subset of app inventory. If APP is listed in targeted_environments, the specified targeting is applied. A maximum of 30,000 app IDs can be targeted. An unset value for targeting allows all app-based bid requests to be sent. Apps can either be targeting positively (bid requests will be sent only if the destination app is listed in the targeting dimension) or negatively (bid requests will be sent only if the destination app is not listed in the targeting dimension)."]
        pub app_targeting: ::std::option::Option<::std::boxed::Box<AppTargeting>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "billingId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The identifier that corresponds to this pretargeting configuration that helps buyers track and attribute their spend across their own arbitrary divisions. If a bid request matches more than one configuration, the buyer chooses which billing_id to attribute each of their bids."]
        pub billing_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The diplay name associated with this configuration. This name must be unique among all the pretargeting configurations a bidder has."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludedContentLabelIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sensitive content category label IDs excluded in this configuration. Bid requests for inventory with any of the specified content label IDs will not be sent. Refer to this file https://storage.googleapis.com/adx-rtb-dictionaries/content-labels.txt for category IDs."]
        pub excluded_content_label_ids:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "geoTargeting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The geos included or excluded in this configuration defined in https://storage.googleapis.com/adx-rtb-dictionaries/geo-table.csv"]
        pub geo_targeting: ::std::option::Option<::std::boxed::Box<NumericTargetingDimension>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includedCreativeDimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Creative dimensions included by this configuration. Only bid requests eligible for at least one of the specified creative dimensions will be sent. An unset value allows all bid requests to be sent, regardless of creative dimension."]
        pub included_creative_dimensions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CreativeDimensions>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includedEnvironments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Environments that are being included. Bid requests will not be sent for a given environment if it is not included. Further restrictions can be applied to included environments to target only a subset of its inventory. An unset value includes all environments."]
        pub included_environments:
            ::std::option::Option<::std::vec::Vec<PretargetingConfigIncludedEnvironmentsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includedFormats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Creative formats included by this configuration. Only bid requests eligible for at least one of the specified creative formats will be sent. An unset value will allow all bid requests to be sent, regardless of format."]
        pub included_formats:
            ::std::option::Option<::std::vec::Vec<PretargetingConfigIncludedFormatsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The languages included in this configuration, represented by their language code. See https://developers.google.com/adwords/api/docs/appendix/languagecodes."]
        pub included_languages: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includedMobileOperatingSystemIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The mobile operating systems included in this configuration as defined in https://storage.googleapis.com/adx-rtb-dictionaries/mobile-os.csv"]
        pub included_mobile_operating_system_ids:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includedPlatforms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The platforms included by this configration. Bid requests for devices with the specified platform types will be sent. An unset value allows all bid requests to be sent, regardless of platform."]
        pub included_platforms:
            ::std::option::Option<::std::vec::Vec<PretargetingConfigIncludedPlatformsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includedUserIdTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User identifier types included in this configuration. At least one of the user identifier types specified in this list must be available for the bid request to be sent."]
        pub included_user_id_types:
            ::std::option::Option<::std::vec::Vec<PretargetingConfigIncludedUserIdTypesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "interstitialTargeting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The interstitial targeting specified for this configuration. The unset value will allow bid requests to be sent regardless of whether they are for interstitials or not."]
        pub interstitial_targeting:
            ::std::option::Option<PretargetingConfigInterstitialTargetingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "invalidGeoIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Existing included or excluded geos that are invalid. Previously targeted geos may become invalid due to privacy restrictions."]
        pub invalid_geo_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maximumQps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum QPS threshold for this configuration. The bidder should receive no more than this number of bid requests matching this configuration per second across all their bidding endpoints among all trading locations. Further information available at https://developers.google.com/authorized-buyers/rtb/peer-guide"]
        pub maximum_qps: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minimumViewabilityDecile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The targeted minimum viewability decile, ranging in values [0, 10]. A value of 5 means that the configuration will only match adslots for which we predict at least 50% viewability. Values > 10 will be rounded down to 10. An unset value or a value of 0 indicates that bid requests will be sent regardless of viewability."]
        pub minimum_viewability_decile: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Name of the pretargeting configuration that must follow the pattern `bidders/{bidder_account_id}/pretargetingConfigs/{config_id}`"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publisherTargeting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Targeting on a subset of publisher inventory. Publishers can either be targeted positively (bid requests will be sent only if the publisher is listed in the targeting dimension) or negatively (bid requests will be sent only if the publisher is not listed in the targeting dimension). A maximum of 10,000 publisher IDs can be targeted. Publisher IDs are found in [ads.txt](https://iabtechlab.com/ads-txt/) / [app-ads.txt](https://iabtechlab.com/app-ads-txt/) and in bid requests in the `BidRequest.publisher_id` field on the [Google RTB protocol](https://developers.google.com/authorized-buyers/rtb/downloads/realtime-bidding-proto) or the `BidRequest.site.publisher.id` / `BidRequest.app.publisher.id` field on the [OpenRTB protocol](https://developers.google.com/authorized-buyers/rtb/downloads/openrtb-adx-proto)."]
        pub publisher_targeting: ::std::option::Option<::std::boxed::Box<StringTargetingDimension>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The state of this pretargeting configuration."]
        pub state: ::std::option::Option<PretargetingConfigStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userListTargeting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The remarketing lists included or excluded in this configuration as defined in UserList."]
        pub user_list_targeting:
            ::std::option::Option<::std::boxed::Box<NumericTargetingDimension>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verticalTargeting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The verticals included or excluded in this configuration as defined in https://developers.google.com/authorized-buyers/rtb/downloads/publisher-verticals"]
        pub vertical_targeting: ::std::option::Option<::std::boxed::Box<NumericTargetingDimension>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webTargeting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Targeting on a subset of site inventory. If WEB is listed in included_environments, the specified targeting is applied. A maximum of 50,000 site URLs can be targeted. An unset value for targeting allows all web-based bid requests to be sent. Sites can either be targeting positively (bid requests will be sent only if the destination site is listed in the targeting dimension) or negatively (bid requests will be sent only if the destination site is not listed in the pretargeting configuration)."]
        pub web_targeting: ::std::option::Option<::std::boxed::Box<StringTargetingDimension>>,
    }
    impl PretargetingConfig {
        pub fn builder() -> PretargetingConfigBuilder {
            PretargetingConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum PretargetingConfigAllowedUserTargetingModesEnum {
        #[serde(rename = "USER_TARGETING_MODE_UNSPECIFIED")]
        #[doc = "Placeholder for undefined user targeting mode."]
        UserTargetingModeUnspecified,
        #[serde(rename = "REMARKETING_ADS")]
        #[doc = "Remarketing ads are allowed to serve."]
        RemarketingAds,
        #[serde(rename = "INTEREST_BASED_TARGETING")]
        #[doc = "Ads based on user interest category targeting are allowed to serve."]
        InterestBasedTargeting,
    }
    impl ::std::default::Default for PretargetingConfigAllowedUserTargetingModesEnum {
        fn default() -> Self {
            Self::UserTargetingModeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum PretargetingConfigIncludedEnvironmentsEnum {
        #[serde(rename = "ENVIRONMENT_UNSPECIFIED")]
        #[doc = "Placeholder for unspecified environment. This value should not be used."]
        EnvironmentUnspecified,
        #[serde(rename = "APP")]
        #[doc = "App environment."]
        App,
        #[serde(rename = "WEB")]
        #[doc = "Web environment."]
        Web,
    }
    impl ::std::default::Default for PretargetingConfigIncludedEnvironmentsEnum {
        fn default() -> Self {
            Self::EnvironmentUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum PretargetingConfigIncludedFormatsEnum {
        #[serde(rename = "CREATIVE_FORMAT_UNSPECIFIED")]
        #[doc = "Placeholder for undefined creative format. This value should not be used."]
        CreativeFormatUnspecified,
        #[serde(rename = "HTML")]
        #[doc = "HTML and AMPHTML creatives."]
        Html,
        #[serde(rename = "VAST")]
        #[doc = "VAST video or audio creative."]
        Vast,
        #[serde(rename = "NATIVE")]
        #[doc = "Native creative, including standard and video native ads."]
        Native,
    }
    impl ::std::default::Default for PretargetingConfigIncludedFormatsEnum {
        fn default() -> Self {
            Self::CreativeFormatUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum PretargetingConfigIncludedPlatformsEnum {
        #[serde(rename = "PLATFORM_UNSPECIFIED")]
        #[doc = "Placeholder for an undefined platform. This value should not be used."]
        PlatformUnspecified,
        #[serde(rename = "PERSONAL_COMPUTER")]
        #[doc = "The personal computer platform."]
        PersonalComputer,
        #[serde(rename = "PHONE")]
        #[doc = "The mobile platform."]
        Phone,
        #[serde(rename = "TABLET")]
        #[doc = "The tablet platform."]
        Tablet,
        #[serde(rename = "CONNECTED_TV")]
        #[doc = "The connected TV platform."]
        ConnectedTv,
    }
    impl ::std::default::Default for PretargetingConfigIncludedPlatformsEnum {
        fn default() -> Self {
            Self::PlatformUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum PretargetingConfigIncludedUserIdTypesEnum {
        #[serde(rename = "USER_ID_TYPE_UNSPECIFIED")]
        #[doc = "Placeholder for unspecified user identifier."]
        UserIdTypeUnspecified,
        #[serde(rename = "HOSTED_MATCH_DATA")]
        #[doc = "Hosted match data, referring to hosted_match_data in the bid request."]
        HostedMatchData,
        #[serde(rename = "GOOGLE_COOKIE")]
        #[doc = "Google cookie, referring to google_user_id in the bid request."]
        GoogleCookie,
        #[serde(rename = "DEVICE_ID")]
        #[doc = "Mobile device advertising ID."]
        DeviceId,
    }
    impl ::std::default::Default for PretargetingConfigIncludedUserIdTypesEnum {
        fn default() -> Self {
            Self::UserIdTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The interstitial targeting specified for this configuration. The unset value will allow bid requests to be sent regardless of whether they are for interstitials or not."]
    pub enum PretargetingConfigInterstitialTargetingEnum {
        #[serde(rename = "INTERSTITIAL_TARGETING_UNSPECIFIED")]
        #[doc = "Unspecified interstitial targeting. Represents an interstitial-agnostic selection."]
        InterstitialTargetingUnspecified,
        #[serde(rename = "ONLY_INTERSTITIAL_REQUESTS")]
        #[doc = "Only bid requests for interstitial inventory should be sent."]
        OnlyInterstitialRequests,
        #[serde(rename = "ONLY_NON_INTERSTITIAL_REQUESTS")]
        #[doc = "Only bid requests for non-interstitial inventory should be sent."]
        OnlyNonInterstitialRequests,
    }
    impl ::std::default::Default for PretargetingConfigInterstitialTargetingEnum {
        fn default() -> Self {
            Self::InterstitialTargetingUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The state of this pretargeting configuration."]
    pub enum PretargetingConfigStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Placeholder for undefined state."]
        StateUnspecified,
        #[serde(rename = "ACTIVE")]
        #[doc = "This pretargeting configuration is actively being used to filter bid requests."]
        Active,
        #[serde(rename = "SUSPENDED")]
        #[doc = "This pretargeting configuration is suspended and not used in serving."]
        Suspended,
    }
    impl ::std::default::Default for PretargetingConfigStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to stop targeting the provided apps in a specific pretargeting configuration. The pretargeting configuration itself specifies how these apps are targeted. in PretargetingConfig.appTargeting.mobileAppTargeting."]
    pub struct RemoveTargetedAppsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of app IDs to stop targeting in the pretargeting configuration. These values will be removed from the list of targeted app IDs in PretargetingConfig.appTargeting.mobileAppTargeting.values."]
        pub app_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl RemoveTargetedAppsRequest {
        pub fn builder() -> RemoveTargetedAppsRequestBuilder {
            RemoveTargetedAppsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to stop targeting publishers in a specific configuration. The pretargeting configuration itself specifies how these publishers are targeted in PretargetingConfig.publisherTargeting."]
    pub struct RemoveTargetedPublishersRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publisherIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of publisher IDs to stop targeting in the pretargeting configuration. These values will be removed from the list of targeted publisher IDs in PretargetingConfig.publisherTargeting.values. Publishers are identified by their publisher ID from ads.txt / app-ads.txt. See https://iabtechlab.com/ads-txt/ and https://iabtechlab.com/app-ads-txt/ for more details."]
        pub publisher_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl RemoveTargetedPublishersRequest {
        pub fn builder() -> RemoveTargetedPublishersRequestBuilder {
            RemoveTargetedPublishersRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to stop targeting sites in a specific pretargeting configuration. The pretargeting configuration itself specifies how these sites are targeted in PretargetingConfig.webTargeting."]
    pub struct RemoveTargetedSitesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sites")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of site URLs to stop targeting in the pretargeting configuration. These values will be removed from the list of targeted URLs in PretargetingConfig.webTargeting.values."]
        pub sites: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl RemoveTargetedSitesRequest {
        pub fn builder() -> RemoveTargetedSitesRequestBuilder {
            RemoveTargetedSitesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Generic targeting with string values used in app, website and publisher targeting."]
    pub struct StringTargetingDimension {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetingMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How the items in this list should be targeted."]
        pub targeting_mode: ::std::option::Option<StringTargetingDimensionTargetingModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "values")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The values specified."]
        pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl StringTargetingDimension {
        pub fn builder() -> StringTargetingDimensionBuilder {
            StringTargetingDimensionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "How the items in this list should be targeted."]
    pub enum StringTargetingDimensionTargetingModeEnum {
        #[serde(rename = "TARGETING_MODE_UNSPECIFIED")]
        #[doc = "Placeholder for undefined targeting mode."]
        TargetingModeUnspecified,
        #[serde(rename = "INCLUSIVE")]
        #[doc = "The inclusive list type. Inventory must match an item in this list to be targeted."]
        Inclusive,
        #[serde(rename = "EXCLUSIVE")]
        #[doc = "The exclusive list type. Inventory must not match any item in this list to be targeted."]
        Exclusive,
    }
    impl ::std::default::Default for StringTargetingDimensionTargetingModeEnum {
        fn default() -> Self {
            Self::TargetingModeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to suspend a pretargeting configuration. Sets the configuration's state to SUSPENDED."]
    pub struct SuspendPretargetingConfigRequest {}
    impl SuspendPretargetingConfigRequest {
        pub fn builder() -> SuspendPretargetingConfigRequestBuilder {
            SuspendPretargetingConfigRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The URL-level breakdown for the download size."]
    pub struct UrlDownloadSize {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "downloadSizeKb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Download size of the URL in kilobytes."]
        pub download_size_kb: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The normalized URL with query parameters and fragment removed."]
        pub normalized_url: ::std::option::Option<::std::string::String>,
    }
    impl UrlDownloadSize {
        pub fn builder() -> UrlDownloadSizeBuilder {
            UrlDownloadSizeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the URL restriction (for the URL captured by the pixel callback) for a user list."]
    pub struct UrlRestriction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "End date (if specified) of the URL restriction. End date should be later than the start date for the date range to be valid."]
        pub end_date: ::std::option::Option<::std::boxed::Box<Date>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "restrictionType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The restriction type for the specified URL."]
        pub restriction_type: ::std::option::Option<UrlRestrictionRestrictionTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Start date (if specified) of the URL restriction."]
        pub start_date: ::std::option::Option<::std::boxed::Box<Date>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The URL to use for applying the restriction on the user list."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl UrlRestriction {
        pub fn builder() -> UrlRestrictionBuilder {
            UrlRestrictionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The restriction type for the specified URL."]
    pub enum UrlRestrictionRestrictionTypeEnum {
        #[serde(rename = "RESTRICTION_TYPE_UNSPECIFIED")]
        #[doc = "Default value that should never be used."]
        RestrictionTypeUnspecified,
        #[serde(rename = "CONTAINS")]
        #[doc = "The tag URL (as recorded by the pixel callback) contains the specified URL."]
        Contains,
        #[serde(rename = "EQUALS")]
        #[doc = "The tag URL (as recorded by the pixel callback) exactly matches the specified URL."]
        Equals,
        #[serde(rename = "STARTS_WITH")]
        #[doc = "The tag URL (as recorded by the pixel callback) starts with the specified URL."]
        StartsWith,
        #[serde(rename = "ENDS_WITH")]
        #[doc = "The tag URL (as recorded by the pixel callback) ends with the specified URL."]
        EndsWith,
        #[serde(rename = "DOES_NOT_EQUAL")]
        #[doc = "The tag URL (as recorded by the pixel callback) does not equal the specified URL."]
        DoesNotEqual,
        #[serde(rename = "DOES_NOT_CONTAIN")]
        #[doc = "The tag URL (as recorded by the pixel callback) does not contain the specified URL."]
        DoesNotContain,
        #[serde(rename = "DOES_NOT_START_WITH")]
        #[doc = "The tag URL (as recorded by the pixel callback) does not start with the specified URL."]
        DoesNotStartWith,
        #[serde(rename = "DOES_NOT_END_WITH")]
        #[doc = "The tag URL (as recorded by the pixel callback) does not end with the specified URL."]
        DoesNotEndWith,
    }
    impl ::std::default::Default for UrlRestrictionRestrictionTypeEnum {
        fn default() -> Self {
            Self::RestrictionTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents an Authorized Buyers user list. Authorized Buyers can create/update/list user lists. Once a user list is created in the system, Authorized Buyers can add users to the user list using the bulk uploader API. Alternatively, users can be added by hosting a tag on the advertiser's page."]
    pub struct UserList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The description for the user list."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Display name of the user list. This must be unique across all user lists for a given account."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "membershipDurationDays")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The number of days a user's cookie stays on the user list. The field must be between 0 and 540 inclusive."]
        pub membership_duration_days: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Name of the user list that must follow the pattern `buyers/{buyer}/userLists/{user_list}`, where `{buyer}` represents the account ID of the buyer who owns the user list. For a bidder accessing user lists on behalf of a child seat buyer, `{buyer}` represents the account ID of the child seat buyer. `{user_list}` is an int64 identifier assigned by Google to uniquely identify a user list."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The status of the user list. A new user list starts out as open."]
        pub status: ::std::option::Option<UserListStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "urlRestriction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The URL restriction for the user list."]
        pub url_restriction: ::std::option::Option<::std::boxed::Box<UrlRestriction>>,
    }
    impl UserList {
        pub fn builder() -> UserListBuilder {
            UserListBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The status of the user list. A new user list starts out as open."]
    pub enum UserListStatusEnum {
        #[serde(rename = "STATUS_UNSPECIFIED")]
        #[doc = "Default value that should never be used."]
        StatusUnspecified,
        #[serde(rename = "OPEN")]
        #[doc = "New users can be added to the user list."]
        Open,
        #[serde(rename = "CLOSED")]
        #[doc = "New users cannot be added to the user list."]
        Closed,
    }
    impl ::std::default::Default for UserListStatusEnum {
        fn default() -> Self {
            Self::StatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video content for a creative."]
    pub struct VideoContent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Video metadata."]
        pub video_metadata: ::std::option::Option<::std::boxed::Box<VideoMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL to fetch a video ad."]
        pub video_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoVastXml")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The contents of a VAST document for a video ad. This document should conform to the VAST 2.0 or 3.0 standard."]
        pub video_vast_xml: ::std::option::Option<::std::string::String>,
    }
    impl VideoContent {
        pub fn builder() -> VideoContentBuilder {
            VideoContentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video metadata for a creative."]
    pub struct VideoMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The duration of the ad. Can be used to filter the response of the creatives.list method."]
        pub duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isValidVast")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Is this a valid VAST ad? Can be used to filter the response of the creatives.list method."]
        pub is_valid_vast: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isVpaid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Is this a VPAID ad? Can be used to filter the response of the creatives.list method."]
        pub is_vpaid: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mediaFiles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of all media files declared in the VAST. If there are multiple VASTs in a wrapper chain, this includes the media files from the deepest one in the chain."]
        pub media_files: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MediaFile>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "skipOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minimum duration that the user has to watch before being able to skip this ad. If the field is not set, the ad is not skippable. If the field is set, the ad is skippable. Can be used to filter the response of the creatives.list method."]
        pub skip_offset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vastVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum VAST version across all wrapped VAST documents. Can be used to filter the response of the creatives.list method."]
        pub vast_version: ::std::option::Option<VideoMetadataVastVersionEnum>,
    }
    impl VideoMetadata {
        pub fn builder() -> VideoMetadataBuilder {
            VideoMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The maximum VAST version across all wrapped VAST documents. Can be used to filter the response of the creatives.list method."]
    pub enum VideoMetadataVastVersionEnum {
        #[serde(rename = "VAST_VERSION_UNSPECIFIED")]
        #[doc = "Default value that should never be used."]
        VastVersionUnspecified,
        #[serde(rename = "VAST_VERSION_1_0")]
        #[doc = "VAST 1.0"]
        VastVersion10,
        #[serde(rename = "VAST_VERSION_2_0")]
        #[doc = "VAST 2.0"]
        VastVersion20,
        #[serde(rename = "VAST_VERSION_3_0")]
        #[doc = "VAST 3.0"]
        VastVersion30,
        #[serde(rename = "VAST_VERSION_4_0")]
        #[doc = "VAST 4.0"]
        VastVersion40,
    }
    impl ::std::default::Default for VideoMetadataVastVersionEnum {
        fn default() -> Self {
            Self::VastVersionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to receive push notifications when any of the creatives belonging to the bidder changes status."]
    pub struct WatchCreativesRequest {}
    impl WatchCreativesRequest {
        pub fn builder() -> WatchCreativesRequestBuilder {
            WatchCreativesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A response for the request to receive push notification when a bidder's creatives change status."]
    pub struct WatchCreativesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subscription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Pub/Sub subscription that can be used to pull creative status notifications. This would be of the format `projects/{project_id}/subscriptions/{subscription_id}`. Subscription is created with pull delivery. All service accounts belonging to the bidder will have read access to this subscription. Subscriptions that are inactive for more than 90 days will be disabled. Please use watchCreatives to re-enable the subscription."]
        pub subscription: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Pub/Sub topic that will be used to publish creative serving status notifications. This would be of the format `projects/{project_id}/topics/{topic_id}`."]
        pub topic: ::std::option::Option<::std::string::String>,
    }
    impl WatchCreativesResponse {
        pub fn builder() -> WatchCreativesResponseBuilder {
            WatchCreativesResponseBuilder::default()
        }
    }
}
