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
    pub mod apps {
        pub mod methods {
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
                    #[doc = "Standard field mask for the set of fields to be updated."]
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
            pub mod authorized_certificates {
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
                            #[doc = "Controls the set of fields returned in the GET response."]
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
                        #[doc = "Controls the set of fields returned in the GET response."]
                        pub enum QueryParametersViewEnum {
                            #[serde(rename = "BASIC_CERTIFICATE")]
                            #[doc = "Basic certificate information, including applicable domains and expiration date."]
                            BasicCertificate,
                            #[serde(rename = "FULL_CERTIFICATE")]
                            #[doc = "The information from BASIC_CERTIFICATE, plus detailed information on the domain mappings that have this certificate mapped."]
                            FullCertificate,
                        }
                        impl ::std::default::Default for QueryParametersViewEnum {
                            fn default() -> Self {
                                Self::BasicCertificate
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
                            #[doc = "Maximum results to return per page."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Continuation token for fetching the next page of results."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "view")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Controls the set of fields returned in the LIST response."]
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
                        #[doc = "Controls the set of fields returned in the LIST response."]
                        pub enum QueryParametersViewEnum {
                            #[serde(rename = "BASIC_CERTIFICATE")]
                            #[doc = "Basic certificate information, including applicable domains and expiration date."]
                            BasicCertificate,
                            #[serde(rename = "FULL_CERTIFICATE")]
                            #[doc = "The information from BASIC_CERTIFICATE, plus detailed information on the domain mappings that have this certificate mapped."]
                            FullCertificate,
                        }
                        impl ::std::default::Default for QueryParametersViewEnum {
                            fn default() -> Self {
                                Self::BasicCertificate
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
                            #[doc = "Standard field mask for the set of fields to be updated. Updates are only supported on the certificate_raw_data and display_name fields."]
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
            pub mod authorized_domains {
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
                            #[doc = "Maximum results to return per page."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Continuation token for fetching the next page of results."]
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
            pub mod domain_mappings {
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
                            #[serde(rename = "overrideStrategy")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Whether the domain creation should override any existing mappings for this domain. By default, overrides are rejected."]
                            pub override_strategy:
                                ::std::option::Option<QueryParametersOverrideStrategyEnum>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Whether the domain creation should override any existing mappings for this domain. By default, overrides are rejected."]
                        pub enum QueryParametersOverrideStrategyEnum {
                            #[serde(rename = "UNSPECIFIED_DOMAIN_OVERRIDE_STRATEGY")]
                            #[doc = "Strategy unspecified. Defaults to STRICT."]
                            UnspecifiedDomainOverrideStrategy,
                            #[serde(rename = "STRICT")]
                            #[doc = "Overrides not allowed. If a mapping already exists for the specified domain, the request will return an ALREADY_EXISTS (409)."]
                            Strict,
                            #[serde(rename = "OVERRIDE")]
                            #[doc = "Overrides allowed. If a mapping already exists for the specified domain, the request will overwrite it. Note that this might stop another Google product from serving. For example, if the domain is mapped to another App Engine application, that app will no longer serve from that domain."]
                            Override,
                        }
                        impl ::std::default::Default for QueryParametersOverrideStrategyEnum {
                            fn default() -> Self {
                                Self::UnspecifiedDomainOverrideStrategy
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
                            #[doc = "Maximum results to return per page."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Continuation token for fetching the next page of results."]
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
                            #[doc = "Standard field mask for the set of fields to be updated."]
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
            pub mod firewall {
                pub mod resources {
                    pub mod ingress_rules {
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
                                    #[serde(rename = "matchingAddress")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A valid IP Address. If set, only rules matching this address will be returned. The first returned rule will be the rule that fires on requests from this IP."]
                                    pub matching_address:
                                        ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Maximum results to return per page."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Continuation token for fetching the next page of results."]
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
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Standard field mask for the set of fields to be updated."]
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
            }
            pub mod operations {
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
            }
            pub mod services {
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
                            #[doc = "Maximum results to return per page."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Continuation token for fetching the next page of results."]
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
                            #[serde(rename = "migrateTraffic")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Set to true to gradually shift traffic to one or more versions that you specify. By default, traffic is shifted immediately. For gradual traffic migration, the target versions must be located within instances that are configured for both warmup requests (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#InboundServiceType) and automatic scaling (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#AutomaticScaling). You must specify the shardBy (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services#ShardBy) field in the Service resource. Gradual traffic migration is not supported in the App Engine flexible environment. For examples, see Migrating and Splitting Traffic (https://cloud.google.com/appengine/docs/admin-api/migrating-splitting-traffic)."]
                            pub migrate_traffic: ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Standard field mask for the set of fields to be updated."]
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
                    pub mod versions {
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
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Controls the set of fields returned in the Get response."]
                                    pub view: ::std::option::Option<QueryParametersViewEnum>,
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
                                #[doc = "Controls the set of fields returned in the Get response."]
                                pub enum QueryParametersViewEnum {
                                    #[serde(rename = "BASIC")]
                                    #[doc = "Basic version information including scaling and inbound services, but not detailed deployment information."]
                                    Basic,
                                    #[serde(rename = "FULL")]
                                    #[doc = "The information from BASIC, plus detailed information about the deployment. This format is required when creating resources, but is not returned in Get or List by default."]
                                    Full,
                                }
                                impl ::std::default::Default for QueryParametersViewEnum {
                                    fn default() -> Self {
                                        Self::Basic
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
                                    #[doc = "Maximum results to return per page."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Continuation token for fetching the next page of results."]
                                    pub page_token: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "view")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Controls the set of fields returned in the List response."]
                                    pub view: ::std::option::Option<QueryParametersViewEnum>,
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
                                #[doc = "Controls the set of fields returned in the List response."]
                                pub enum QueryParametersViewEnum {
                                    #[serde(rename = "BASIC")]
                                    #[doc = "Basic version information including scaling and inbound services, but not detailed deployment information."]
                                    Basic,
                                    #[serde(rename = "FULL")]
                                    #[doc = "The information from BASIC, plus detailed information about the deployment. This format is required when creating resources, but is not returned in Get or List by default."]
                                    Full,
                                }
                                impl ::std::default::Default for QueryParametersViewEnum {
                                    fn default() -> Self {
                                        Self::Basic
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
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Standard field mask for the set of fields to be updated."]
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
                            pub mod instances {
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
                                            #[doc = "Maximum results to return per page."]
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
                                            #[doc = "Continuation token for fetching the next page of results."]
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
    #[doc = "Google Cloud Endpoints (https://cloud.google.com/appengine/docs/python/endpoints/) configuration for API handlers."]
    pub struct ApiConfigHandler {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authFailAction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Action to take when users access resources that require authentication. Defaults to redirect."]
        pub auth_fail_action: ::std::option::Option<ApiConfigHandlerAuthFailActionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "login")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Level of login required to access this resource. Defaults to optional."]
        pub login: ::std::option::Option<ApiConfigHandlerLoginEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "script")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Path to the script from the application root directory."]
        pub script: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "securityLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Security (HTTPS) enforcement for this URL."]
        pub security_level: ::std::option::Option<ApiConfigHandlerSecurityLevelEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL to serve the endpoint at."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl ApiConfigHandler {
        pub fn builder() -> ApiConfigHandlerBuilder {
            ApiConfigHandlerBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Action to take when users access resources that require authentication. Defaults to redirect."]
    pub enum ApiConfigHandlerAuthFailActionEnum {
        #[serde(rename = "AUTH_FAIL_ACTION_UNSPECIFIED")]
        #[doc = "Not specified. AUTH_FAIL_ACTION_REDIRECT is assumed."]
        AuthFailActionUnspecified,
        #[serde(rename = "AUTH_FAIL_ACTION_REDIRECT")]
        #[doc = "Redirects user to \"accounts.google.com\". The user is redirected back to the application URL after signing in or creating an account."]
        AuthFailActionRedirect,
        #[serde(rename = "AUTH_FAIL_ACTION_UNAUTHORIZED")]
        #[doc = "Rejects request with a 401 HTTP status code and an error message."]
        AuthFailActionUnauthorized,
    }
    impl ::std::default::Default for ApiConfigHandlerAuthFailActionEnum {
        fn default() -> Self {
            Self::AuthFailActionUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Level of login required to access this resource. Defaults to optional."]
    pub enum ApiConfigHandlerLoginEnum {
        #[serde(rename = "LOGIN_UNSPECIFIED")]
        #[doc = "Not specified. LOGIN_OPTIONAL is assumed."]
        LoginUnspecified,
        #[serde(rename = "LOGIN_OPTIONAL")]
        #[doc = "Does not require that the user is signed in."]
        LoginOptional,
        #[serde(rename = "LOGIN_ADMIN")]
        #[doc = "If the user is not signed in, the auth_fail_action is taken. In addition, if the user is not an administrator for the application, they are given an error message regardless of auth_fail_action. If the user is an administrator, the handler proceeds."]
        LoginAdmin,
        #[serde(rename = "LOGIN_REQUIRED")]
        #[doc = "If the user has signed in, the handler proceeds normally. Otherwise, the auth_fail_action is taken."]
        LoginRequired,
    }
    impl ::std::default::Default for ApiConfigHandlerLoginEnum {
        fn default() -> Self {
            Self::LoginUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Security (HTTPS) enforcement for this URL."]
    pub enum ApiConfigHandlerSecurityLevelEnum {
        #[serde(rename = "SECURE_UNSPECIFIED")]
        #[doc = "Not specified."]
        SecureUnspecified,
        #[serde(rename = "SECURE_DEFAULT")]
        #[doc = "Both HTTP and HTTPS requests with URLs that match the handler succeed without redirects. The application can examine the request to determine which protocol was used, and respond accordingly."]
        SecureDefault,
        #[serde(rename = "SECURE_NEVER")]
        #[doc = "Requests for a URL that match this handler that use HTTPS are automatically redirected to the HTTP equivalent URL."]
        SecureNever,
        #[serde(rename = "SECURE_OPTIONAL")]
        #[doc = "Both HTTP and HTTPS requests with URLs that match the handler succeed without redirects. The application can examine the request to determine which protocol was used and respond accordingly."]
        SecureOptional,
        #[serde(rename = "SECURE_ALWAYS")]
        #[doc = "Requests for a URL that match this handler that do not use HTTPS are automatically redirected to the HTTPS URL with the same path. Query parameters are reserved for the redirect."]
        SecureAlways,
    }
    impl ::std::default::Default for ApiConfigHandlerSecurityLevelEnum {
        fn default() -> Self {
            Self::SecureUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Uses Google Cloud Endpoints to handle requests."]
    pub struct ApiEndpointHandler {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scriptPath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Path to the script from the application root directory."]
        pub script_path: ::std::option::Option<::std::string::String>,
    }
    impl ApiEndpointHandler {
        pub fn builder() -> ApiEndpointHandlerBuilder {
            ApiEndpointHandlerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An Application resource contains the top-level configuration of an App Engine application."]
    pub struct Application {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authDomain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Apps authentication domain that controls which users can access this application.Defaults to open access for any Google Account."]
        pub auth_domain: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "codeBucket")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage bucket that can be used for storing files associated with this application. This bucket is associated with the application and can be used by the gcloud deployment commands.@OutputOnly"]
        pub code_bucket: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "databaseType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the Cloud Firestore or Cloud Datastore database associated with this application."]
        pub database_type: ::std::option::Option<ApplicationDatabaseTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultBucket")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage bucket that can be used by this application to store content.@OutputOnly"]
        pub default_bucket: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultCookieExpiration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cookie expiration policy for this application."]
        pub default_cookie_expiration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultHostname")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hostname used to reach this application, as resolved by App Engine.@OutputOnly"]
        pub default_hostname: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dispatchRules")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "HTTP path dispatch rules for requests to the application that do not explicitly target a service or version. Rules are order-dependent. Up to 20 dispatch rules can be supported."]
        pub dispatch_rules:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UrlDispatchRule>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "featureSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The feature specific settings to be used in the application."]
        pub feature_settings: ::std::option::Option<::std::boxed::Box<FeatureSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcrDomain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Google Container Registry domain used for storing managed build docker images for this application."]
        pub gcr_domain: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iap")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub iap: ::std::option::Option<::std::boxed::Box<IdentityAwareProxy>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier of the Application resource. This identifier is equivalent to the project ID of the Google Cloud Platform project where you want to deploy your application. Example: myapp."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location from which this application runs. Application instances run out of the data centers in the specified location, which is also where all of the application's end user content is stored.Defaults to us-central.View the list of supported locations (https://cloud.google.com/appengine/docs/locations)."]
        pub location_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Full path to the Application resource in the API. Example: apps/myapp.@OutputOnly"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "servingStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serving status of this application."]
        pub serving_status: ::std::option::Option<ApplicationServingStatusEnum>,
    }
    impl Application {
        pub fn builder() -> ApplicationBuilder {
            ApplicationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the Cloud Firestore or Cloud Datastore database associated with this application."]
    pub enum ApplicationDatabaseTypeEnum {
        #[serde(rename = "DATABASE_TYPE_UNSPECIFIED")]
        #[doc = "Database type is unspecified."]
        DatabaseTypeUnspecified,
        #[serde(rename = "CLOUD_DATASTORE")]
        #[doc = "Cloud Datastore"]
        CloudDatastore,
        #[serde(rename = "CLOUD_FIRESTORE")]
        #[doc = "Cloud Firestore Native"]
        CloudFirestore,
        #[serde(rename = "CLOUD_DATASTORE_COMPATIBILITY")]
        #[doc = "Cloud Firestore in Datastore Mode"]
        CloudDatastoreCompatibility,
    }
    impl ::std::default::Default for ApplicationDatabaseTypeEnum {
        fn default() -> Self {
            Self::DatabaseTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Serving status of this application."]
    pub enum ApplicationServingStatusEnum {
        #[serde(rename = "UNSPECIFIED")]
        #[doc = "Serving status is unspecified."]
        Unspecified,
        #[serde(rename = "SERVING")]
        #[doc = "Application is serving."]
        Serving,
        #[serde(rename = "USER_DISABLED")]
        #[doc = "Application has been disabled by the user."]
        UserDisabled,
        #[serde(rename = "SYSTEM_DISABLED")]
        #[doc = "Application has been disabled by the system."]
        SystemDisabled,
    }
    impl ::std::default::Default for ApplicationServingStatusEnum {
        fn default() -> Self {
            Self::Unspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An SSL certificate that a user has been authorized to administer. A user is authorized to administer any certificate that applies to one of their authorized domains."]
    pub struct AuthorizedCertificate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "certificateRawData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The SSL certificate serving the AuthorizedCertificate resource. This must be obtained independently from a certificate authority."]
        pub certificate_raw_data: ::std::option::Option<::std::boxed::Box<CertificateRawData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user-specified display name of the certificate. This is not guaranteed to be unique. Example: My Certificate."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domainMappingsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Aggregate count of the domain mappings with this certificate mapped. This count includes domain mappings on applications for which the user does not have VIEWER permissions.Only returned by GET or LIST requests when specifically requested by the view=FULL_CERTIFICATE option.@OutputOnly"]
        pub domain_mappings_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domainNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Topmost applicable domains of this certificate. This certificate applies to these domains and their subdomains. Example: example.com.@OutputOnly"]
        pub domain_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expireTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when this certificate expires. To update the renewal time on this certificate, upload an SSL certificate with a different expiration time using AuthorizedCertificates.UpdateAuthorizedCertificate.@OutputOnly"]
        pub expire_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Relative name of the certificate. This is a unique value autogenerated on AuthorizedCertificate resource creation. Example: 12345.@OutputOnly"]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "managedCertificate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Only applicable if this certificate is managed by App Engine. Managed certificates are tied to the lifecycle of a DomainMapping and cannot be updated or deleted via the AuthorizedCertificates API. If this certificate is manually administered by the user, this field will be empty.@OutputOnly"]
        pub managed_certificate: ::std::option::Option<::std::boxed::Box<ManagedCertificate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Full path to the AuthorizedCertificate resource in the API. Example: apps/myapp/authorizedCertificates/12345.@OutputOnly"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visibleDomainMappings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full paths to user visible Domain Mapping resources that have this certificate mapped. Example: apps/myapp/domainMappings/example.com.This may not represent the full list of mapped domain mappings if the user does not have VIEWER permissions on all of the applications that have this certificate mapped. See domain_mappings_count for a complete count.Only returned by GET or LIST requests when specifically requested by the view=FULL_CERTIFICATE option.@OutputOnly"]
        pub visible_domain_mappings: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl AuthorizedCertificate {
        pub fn builder() -> AuthorizedCertificateBuilder {
            AuthorizedCertificateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A domain that a user has been authorized to administer. To authorize use of a domain, verify ownership via Webmaster Central (https://www.google.com/webmasters/verification/home)."]
    pub struct AuthorizedDomain {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fully qualified domain name of the domain authorized for use. Example: example.com."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Full path to the AuthorizedDomain resource in the API. Example: apps/myapp/authorizedDomains/example.com.@OutputOnly"]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl AuthorizedDomain {
        pub fn builder() -> AuthorizedDomainBuilder {
            AuthorizedDomainBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Automatic scaling is based on request rate, response latencies, and other application metrics."]
    pub struct AutomaticScaling {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "coolDownPeriod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time period that the Autoscaler (https://cloud.google.com/compute/docs/autoscaler/) should wait before it starts collecting information from a new instance. This prevents the autoscaler from collecting information when the instance is initializing, during which the collected usage would not be reliable. Only applicable in the App Engine flexible environment."]
        pub cool_down_period: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpuUtilization")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Target scaling by CPU usage."]
        pub cpu_utilization: ::std::option::Option<::std::boxed::Box<CpuUtilization>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diskUtilization")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Target scaling by disk usage."]
        pub disk_utilization: ::std::option::Option<::std::boxed::Box<DiskUtilization>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxConcurrentRequests")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of concurrent requests an automatic scaling instance can accept before the scheduler spawns a new instance.Defaults to a runtime-specific value."]
        pub max_concurrent_requests: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxIdleInstances")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum number of idle instances that should be maintained for this version."]
        pub max_idle_instances: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxPendingLatency")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum amount of time that a request should wait in the pending queue before starting a new instance to handle it."]
        pub max_pending_latency: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxTotalInstances")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum number of instances that should be started to handle requests for this version."]
        pub max_total_instances: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minIdleInstances")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum number of idle instances that should be maintained for this version. Only applicable for the default version of a service."]
        pub min_idle_instances: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minPendingLatency")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum amount of time a request should wait in the pending queue before starting a new instance to handle it."]
        pub min_pending_latency: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minTotalInstances")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum number of running instances that should be maintained for this version."]
        pub min_total_instances: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "networkUtilization")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Target scaling by network usage."]
        pub network_utilization: ::std::option::Option<::std::boxed::Box<NetworkUtilization>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestUtilization")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Target scaling by request utilization."]
        pub request_utilization: ::std::option::Option<::std::boxed::Box<RequestUtilization>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "standardSchedulerSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Scheduler settings for standard environment."]
        pub standard_scheduler_settings:
            ::std::option::Option<::std::boxed::Box<StandardSchedulerSettings>>,
    }
    impl AutomaticScaling {
        pub fn builder() -> AutomaticScalingBuilder {
            AutomaticScalingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A service with basic scaling will create an instance when the application receives a request. The instance will be turned down when the app becomes idle. Basic scaling is ideal for work that is intermittent or driven by user activity."]
    pub struct BasicScaling {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "idleTimeout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Duration of time after the last request that an instance must wait before the instance is shut down."]
        pub idle_timeout: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxInstances")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum number of instances to create for this version."]
        pub max_instances: ::std::option::Option<::std::primitive::i64>,
    }
    impl BasicScaling {
        pub fn builder() -> BasicScalingBuilder {
            BasicScalingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for Firewall.BatchUpdateIngressRules."]
    pub struct BatchUpdateIngressRulesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ingressRules")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of FirewallRules to replace the existing set."]
        pub ingress_rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FirewallRule>>>,
    }
    impl BatchUpdateIngressRulesRequest {
        pub fn builder() -> BatchUpdateIngressRulesRequestBuilder {
            BatchUpdateIngressRulesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for Firewall.UpdateAllIngressRules."]
    pub struct BatchUpdateIngressRulesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ingressRules")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full list of ingress FirewallRules for this application."]
        pub ingress_rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FirewallRule>>>,
    }
    impl BatchUpdateIngressRulesResponse {
        pub fn builder() -> BatchUpdateIngressRulesResponseBuilder {
            BatchUpdateIngressRulesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An SSL certificate obtained from a certificate authority."]
    pub struct CertificateRawData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "privateKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unencrypted PEM encoded RSA private key. This field is set once on certificate creation and then encrypted. The key size must be 2048 bits or fewer. Must include the header and footer. Example: -----BEGIN RSA PRIVATE KEY----- -----END RSA PRIVATE KEY----- @InputOnly"]
        pub private_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publicCertificate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "PEM encoded x.509 public key certificate. This field is set once on certificate creation. Must include the header and footer. Example: -----BEGIN CERTIFICATE----- -----END CERTIFICATE----- "]
        pub public_certificate: ::std::option::Option<::std::string::String>,
    }
    impl CertificateRawData {
        pub fn builder() -> CertificateRawDataBuilder {
            CertificateRawDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Options for the build operations performed as a part of the version deployment. Only applicable for App Engine flexible environment when creating a version using source code directly."]
    pub struct CloudBuildOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appYamlPath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Path to the yaml file used in deployment, used to determine runtime configuration details.Required for flexible environment builds.See https://cloud.google.com/appengine/docs/standard/python/config/appref for more details."]
        pub app_yaml_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloudBuildTimeout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Cloud Build timeout used as part of any dependent builds performed by version creation. Defaults to 10 minutes."]
        pub cloud_build_timeout: ::std::option::Option<::std::string::String>,
    }
    impl CloudBuildOptions {
        pub fn builder() -> CloudBuildOptionsBuilder {
            CloudBuildOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Docker image that is used to create a container and start a VM instance for the version that you deploy. Only applicable for instances running in the App Engine flexible environment."]
    pub struct ContainerInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URI to the hosted container image in Google Container Registry. The URI must be fully qualified and include a tag or digest. Examples: \"gcr.io/my-project/image:tag\" or \"gcr.io/my-project/image@digest\""]
        pub image: ::std::option::Option<::std::string::String>,
    }
    impl ContainerInfo {
        pub fn builder() -> ContainerInfoBuilder {
            ContainerInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Target scaling by CPU usage."]
    pub struct CpuUtilization {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aggregationWindowLength")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Period of time over which CPU utilization is calculated."]
        pub aggregation_window_length: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetUtilization")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Target CPU utilization ratio to maintain when scaling. Must be between 0 and 1."]
        pub target_utilization: ::std::option::Option<::std::primitive::f64>,
    }
    impl CpuUtilization {
        pub fn builder() -> CpuUtilizationBuilder {
            CpuUtilizationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for the given google.longrunning.Operation during a google.appengine.v1.CreateVersionRequest."]
    pub struct CreateVersionMetadataV1 {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloudBuildId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Cloud Build ID if one was created as part of the version create. @OutputOnly"]
        pub cloud_build_id: ::std::option::Option<::std::string::String>,
    }
    impl CreateVersionMetadataV1 {
        pub fn builder() -> CreateVersionMetadataV1Builder {
            CreateVersionMetadataV1Builder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for the given google.longrunning.Operation during a google.appengine.v1alpha.CreateVersionRequest."]
    pub struct CreateVersionMetadataV1Alpha {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloudBuildId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Cloud Build ID if one was created as part of the version create. @OutputOnly"]
        pub cloud_build_id: ::std::option::Option<::std::string::String>,
    }
    impl CreateVersionMetadataV1Alpha {
        pub fn builder() -> CreateVersionMetadataV1AlphaBuilder {
            CreateVersionMetadataV1AlphaBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for the given google.longrunning.Operation during a google.appengine.v1beta.CreateVersionRequest."]
    pub struct CreateVersionMetadataV1Beta {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloudBuildId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Cloud Build ID if one was created as part of the version create. @OutputOnly"]
        pub cloud_build_id: ::std::option::Option<::std::string::String>,
    }
    impl CreateVersionMetadataV1Beta {
        pub fn builder() -> CreateVersionMetadataV1BetaBuilder {
            CreateVersionMetadataV1BetaBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for Instances.DebugInstance."]
    pub struct DebugInstanceRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sshKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Public SSH key to add to the instance. Examples: [USERNAME]:ssh-rsa [KEY_VALUE] [USERNAME] [USERNAME]:ssh-rsa [KEY_VALUE] google-ssh {\"userName\":\"[USERNAME]\",\"expireOn\":\"[EXPIRE_TIME]\"}For more information, see Adding and Removing SSH Keys (https://cloud.google.com/compute/docs/instances/adding-removing-ssh-keys)."]
        pub ssh_key: ::std::option::Option<::std::string::String>,
    }
    impl DebugInstanceRequest {
        pub fn builder() -> DebugInstanceRequestBuilder {
            DebugInstanceRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Code and application artifacts used to deploy a version to App Engine."]
    pub struct Deployment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloudBuildOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Options for any Google Cloud Build builds created as a part of this deployment.These options will only be used if a new build is created, such as when deploying to the App Engine flexible environment using files or zip."]
        pub cloud_build_options: ::std::option::Option<::std::boxed::Box<CloudBuildOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "container")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Docker image for the container that runs the version. Only applicable for instances running in the App Engine flexible environment."]
        pub container: ::std::option::Option<::std::boxed::Box<ContainerInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "files")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Manifest of the files stored in Google Cloud Storage that are included as part of this version. All files must be readable using the credentials supplied with this call."]
        pub files: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<FileInfo>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zip")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The zip file for this deployment, if this is a zip deployment."]
        pub zip: ::std::option::Option<::std::boxed::Box<ZipInfo>>,
    }
    impl Deployment {
        pub fn builder() -> DeploymentBuilder {
            DeploymentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Target scaling by disk usage. Only applicable in the App Engine flexible environment."]
    pub struct DiskUtilization {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetReadBytesPerSecond")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Target bytes read per second."]
        pub target_read_bytes_per_second: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetReadOpsPerSecond")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Target ops read per seconds."]
        pub target_read_ops_per_second: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetWriteBytesPerSecond")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Target bytes written per second."]
        pub target_write_bytes_per_second: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetWriteOpsPerSecond")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Target ops written per second."]
        pub target_write_ops_per_second: ::std::option::Option<::std::primitive::i64>,
    }
    impl DiskUtilization {
        pub fn builder() -> DiskUtilizationBuilder {
            DiskUtilizationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A domain serving an App Engine application."]
    pub struct DomainMapping {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Relative name of the domain serving the application. Example: example.com."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Full path to the DomainMapping resource in the API. Example: apps/myapp/domainMapping/example.com.@OutputOnly"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceRecords")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource records required to configure this domain mapping. These records must be added to the domain's DNS configuration in order to serve the application via this domain mapping.@OutputOnly"]
        pub resource_records:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ResourceRecord>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sslSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "SSL configuration for this domain. If unconfigured, this domain will not serve with SSL."]
        pub ssl_settings: ::std::option::Option<::std::boxed::Box<SslSettings>>,
    }
    impl DomainMapping {
        pub fn builder() -> DomainMappingBuilder {
            DomainMappingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for Empty is empty JSON object {}."]
    pub struct Empty {}
    impl Empty {
        pub fn builder() -> EmptyBuilder {
            EmptyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Cloud Endpoints (https://cloud.google.com/endpoints) configuration. The Endpoints API Service provides tooling for serving Open API and gRPC endpoints via an NGINX proxy. Only valid for App Engine Flexible environment deployments.The fields here refer to the name and configuration ID of a \"service\" resource in the Service Management API (https://cloud.google.com/service-management/overview)."]
    pub struct EndpointsApiService {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "configId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Endpoints service configuration ID as specified by the Service Management API. For example \"2016-09-19r1\".By default, the rollout strategy for Endpoints is RolloutStrategy.FIXED. This means that Endpoints starts up with a particular configuration ID. When a new configuration is rolled out, Endpoints must be given the new configuration ID. The config_id field is used to give the configuration ID and is required in this case.Endpoints also has a rollout strategy called RolloutStrategy.MANAGED. When using this, Endpoints fetches the latest configuration and does not need the configuration ID. In this case, config_id must be omitted."]
        pub config_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disableTraceSampling")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enable or disable trace sampling. By default, this is set to false for enabled."]
        pub disable_trace_sampling: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Endpoints service name which is the name of the \"service\" resource in the Service Management API. For example \"myapi.endpoints.myproject.cloud.goog\""]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rolloutStrategy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Endpoints rollout strategy. If FIXED, config_id must be specified. If MANAGED, config_id must be omitted."]
        pub rollout_strategy: ::std::option::Option<EndpointsApiServiceRolloutStrategyEnum>,
    }
    impl EndpointsApiService {
        pub fn builder() -> EndpointsApiServiceBuilder {
            EndpointsApiServiceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Endpoints rollout strategy. If FIXED, config_id must be specified. If MANAGED, config_id must be omitted."]
    pub enum EndpointsApiServiceRolloutStrategyEnum {
        #[serde(rename = "UNSPECIFIED_ROLLOUT_STRATEGY")]
        #[doc = "Not specified. Defaults to FIXED."]
        UnspecifiedRolloutStrategy,
        #[serde(rename = "FIXED")]
        #[doc = "Endpoints service configuration ID will be fixed to the configuration ID specified by config_id."]
        Fixed,
        #[serde(rename = "MANAGED")]
        #[doc = "Endpoints service configuration ID will be updated with each rollout."]
        Managed,
    }
    impl ::std::default::Default for EndpointsApiServiceRolloutStrategyEnum {
        fn default() -> Self {
            Self::UnspecifiedRolloutStrategy
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The entrypoint for the application."]
    pub struct Entrypoint {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shell")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The format should be a shell command that can be fed to bash -c."]
        pub shell: ::std::option::Option<::std::string::String>,
    }
    impl Entrypoint {
        pub fn builder() -> EntrypointBuilder {
            EntrypointBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Custom static error page to be served when an error occurs."]
    pub struct ErrorHandler {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Error condition this handler applies to."]
        pub error_code: ::std::option::Option<ErrorHandlerErrorCodeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "MIME type of file. Defaults to text/html."]
        pub mime_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "staticFile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Static file content to be served for this error."]
        pub static_file: ::std::option::Option<::std::string::String>,
    }
    impl ErrorHandler {
        pub fn builder() -> ErrorHandlerBuilder {
            ErrorHandlerBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Error condition this handler applies to."]
    pub enum ErrorHandlerErrorCodeEnum {
        #[serde(rename = "ERROR_CODE_UNSPECIFIED")]
        #[doc = "Not specified. ERROR_CODE_DEFAULT is assumed."]
        ErrorCodeUnspecified,
        #[serde(rename = "ERROR_CODE_DEFAULT")]
        #[doc = "All other error types."]
        ErrorCodeDefault,
        #[serde(rename = "ERROR_CODE_OVER_QUOTA")]
        #[doc = "Application has exceeded a resource quota."]
        ErrorCodeOverQuota,
        #[serde(rename = "ERROR_CODE_DOS_API_DENIAL")]
        #[doc = "Client blocked by the application's Denial of Service protection configuration."]
        ErrorCodeDosApiDenial,
        #[serde(rename = "ERROR_CODE_TIMEOUT")]
        #[doc = "Deadline reached before the application responds."]
        ErrorCodeTimeout,
    }
    impl ::std::default::Default for ErrorHandlerErrorCodeEnum {
        fn default() -> Self {
            Self::ErrorCodeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The feature specific settings to be used in the application. These define behaviors that are user configurable."]
    pub struct FeatureSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "splitHealthChecks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Boolean value indicating if split health checks should be used instead of the legacy health checks. At an app.yaml level, this means defaulting to 'readiness_check' and 'liveness_check' values instead of 'health_check' ones. Once the legacy 'health_check' behavior is deprecated, and this value is always true, this setting can be removed."]
        pub split_health_checks: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "useContainerOptimizedOs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, use Container-Optimized OS (https://cloud.google.com/container-optimized-os/) base image for VMs, rather than a base Debian image."]
        pub use_container_optimized_os: ::std::option::Option<::std::primitive::bool>,
    }
    impl FeatureSettings {
        pub fn builder() -> FeatureSettingsBuilder {
            FeatureSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Single source file that is part of the version to be deployed. Each source file that is deployed must be specified separately."]
    pub struct FileInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The MIME type of the file.Defaults to the value from Google Cloud Storage."]
        pub mime_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sha1Sum")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The SHA1 hash of the file, in hex."]
        pub sha1_sum: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL source to use to fetch this file. Must be a URL to a resource in Google Cloud Storage in the form 'http(s)://storage.googleapis.com//'."]
        pub source_url: ::std::option::Option<::std::string::String>,
    }
    impl FileInfo {
        pub fn builder() -> FileInfoBuilder {
            FileInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single firewall rule that is evaluated against incoming traffic and provides an action to take on matched requests."]
    pub struct FirewallRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "action")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The action to take on matched requests."]
        pub action: ::std::option::Option<FirewallRuleActionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional string description of this rule. This field has a maximum length of 100 characters."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "priority")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A positive integer between 1, Int32.MaxValue-1 that defines the order of rule evaluation. Rules with the lowest priority are evaluated first.A default rule at priority Int32.MaxValue matches all IPv4 and IPv6 traffic when no previous rule matches. Only the action of this rule can be modified by the user."]
        pub priority: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "IP address or range, defined using CIDR notation, of requests that this rule applies to. You can use the wildcard character \"*\" to match all IPs equivalent to \"0/0\" and \"::/0\" together. Examples: 192.168.1.1 or 192.168.0.0/16 or 2001:db8::/32 or 2001:0db8:0000:0042:0000:8a2e:0370:7334. Truncation will be silently performed on addresses which are not properly truncated. For example, 1.2.3.4/24 is accepted as the same address as 1.2.3.0/24. Similarly, for IPv6, 2001:db8::1/32 is accepted as the same address as 2001:db8::/32."]
        pub source_range: ::std::option::Option<::std::string::String>,
    }
    impl FirewallRule {
        pub fn builder() -> FirewallRuleBuilder {
            FirewallRuleBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The action to take on matched requests."]
    pub enum FirewallRuleActionEnum {
        #[serde(rename = "UNSPECIFIED_ACTION")]
        #[doc = ""]
        UnspecifiedAction,
        #[serde(rename = "ALLOW")]
        #[doc = "Matching requests are allowed."]
        Allow,
        #[serde(rename = "DENY")]
        #[doc = "Matching requests are denied."]
        Deny,
    }
    impl ::std::default::Default for FirewallRuleActionEnum {
        fn default() -> Self {
            Self::UnspecifiedAction
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Health checking configuration for VM instances. Unhealthy instances are killed and replaced with new instances. Only applicable for instances in App Engine flexible environment."]
    pub struct HealthCheck {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "checkInterval")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Interval between health checks."]
        pub check_interval: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disableHealthCheck")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to explicitly disable health checks for this instance."]
        pub disable_health_check: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "healthyThreshold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of consecutive successful health checks required before receiving traffic."]
        pub healthy_threshold: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "host")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Host header to send when performing an HTTP health check. Example: \"myapp.appspot.com\""]
        pub host: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "restartThreshold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of consecutive failed health checks required before an instance is restarted."]
        pub restart_threshold: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time before the health check is considered failed."]
        pub timeout: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unhealthyThreshold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of consecutive failed health checks required before removing traffic."]
        pub unhealthy_threshold: ::std::option::Option<::std::primitive::i64>,
    }
    impl HealthCheck {
        pub fn builder() -> HealthCheckBuilder {
            HealthCheckBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Identity-Aware Proxy"]
    pub struct IdentityAwareProxy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the serving infrastructure will authenticate and authorize all incoming requests.If true, the oauth2_client_id and oauth2_client_secret fields must be non-empty."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oauth2ClientId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OAuth2 client ID to use for the authentication flow."]
        pub oauth2_client_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oauth2ClientSecret")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OAuth2 client secret to use for the authentication flow.For security reasons, this value cannot be retrieved via the API. Instead, the SHA-256 hash of the value is returned in the oauth2_client_secret_sha256 field.@InputOnly"]
        pub oauth2_client_secret: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oauth2ClientSecretSha256")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hex-encoded SHA-256 hash of the client secret.@OutputOnly"]
        pub oauth2_client_secret_sha256: ::std::option::Option<::std::string::String>,
    }
    impl IdentityAwareProxy {
        pub fn builder() -> IdentityAwareProxyBuilder {
            IdentityAwareProxyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An Instance resource is the computing unit that App Engine uses to automatically scale an application."]
    pub struct Instance {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appEngineRelease")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. App Engine release this instance is running on."]
        pub app_engine_release: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "availability")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Availability of the instance."]
        pub availability: ::std::option::Option<InstanceAvailabilityEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "averageLatency")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Average latency (ms) over the last minute."]
        pub average_latency: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Number of errors since this instance was started."]
        pub errors: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Relative name of the instance within the version. Example: instance-1."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "memoryUsage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Total memory in use (bytes)."]
        pub memory_usage: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Full path to the Instance resource in the API. Example: apps/myapp/services/default/versions/v1/instances/instance-1."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "qps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Average queries per second (QPS) over the last minute."]
        pub qps: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requests")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Number of requests since this instance was started."]
        pub requests: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Time that this instance was started.@OutputOnly"]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vmDebugEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Whether this instance is in debug mode. Only applicable for instances in App Engine flexible environment."]
        pub vm_debug_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vmId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Virtual machine ID of this instance. Only applicable for instances in App Engine flexible environment."]
        pub vm_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vmIp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The IP address of this instance. Only applicable for instances in App Engine flexible environment."]
        pub vm_ip: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vmLiveness")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The liveness health check of this instance. Only applicable for instances in App Engine flexible environment."]
        pub vm_liveness: ::std::option::Option<InstanceVmLivenessEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vmName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Name of the virtual machine where this instance lives. Only applicable for instances in App Engine flexible environment."]
        pub vm_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vmStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Status of the virtual machine where this instance lives. Only applicable for instances in App Engine flexible environment."]
        pub vm_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vmZoneName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Zone where the virtual machine is located. Only applicable for instances in App Engine flexible environment."]
        pub vm_zone_name: ::std::option::Option<::std::string::String>,
    }
    impl Instance {
        pub fn builder() -> InstanceBuilder {
            InstanceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Availability of the instance."]
    pub enum InstanceAvailabilityEnum {
        #[serde(rename = "UNSPECIFIED")]
        #[doc = ""]
        Unspecified,
        #[serde(rename = "RESIDENT")]
        #[doc = ""]
        Resident,
        #[serde(rename = "DYNAMIC")]
        #[doc = ""]
        Dynamic,
    }
    impl ::std::default::Default for InstanceAvailabilityEnum {
        fn default() -> Self {
            Self::Unspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The liveness health check of this instance. Only applicable for instances in App Engine flexible environment."]
    pub enum InstanceVmLivenessEnum {
        #[serde(rename = "LIVENESS_STATE_UNSPECIFIED")]
        #[doc = "There is no liveness health check for the instance. Only applicable for instances in App Engine standard environment."]
        LivenessStateUnspecified,
        #[serde(rename = "UNKNOWN")]
        #[doc = "The health checking system is aware of the instance but its health is not known at the moment."]
        Unknown,
        #[serde(rename = "HEALTHY")]
        #[doc = "The instance is reachable i.e. a connection to the application health checking endpoint can be established, and conforms to the requirements defined by the health check."]
        Healthy,
        #[serde(rename = "UNHEALTHY")]
        #[doc = "The instance is reachable, but does not conform to the requirements defined by the health check."]
        Unhealthy,
        #[serde(rename = "DRAINING")]
        #[doc = "The instance is being drained. The existing connections to the instance have time to complete, but the new ones are being refused."]
        Draining,
        #[serde(rename = "TIMEOUT")]
        #[doc = "The instance is unreachable i.e. a connection to the application health checking endpoint cannot be established, or the server does not respond within the specified timeout."]
        Timeout,
    }
    impl ::std::default::Default for InstanceVmLivenessEnum {
        fn default() -> Self {
            Self::LivenessStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Third-party Python runtime library that is required by the application."]
    pub struct Library {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the library. Example: \"django\"."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Version of the library to select, or \"latest\"."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl Library {
        pub fn builder() -> LibraryBuilder {
            LibraryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for AuthorizedCertificates.ListAuthorizedCertificates."]
    pub struct ListAuthorizedCertificatesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "certificates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The SSL certificates the user is authorized to administer."]
        pub certificates:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuthorizedCertificate>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token for fetching the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListAuthorizedCertificatesResponse {
        pub fn builder() -> ListAuthorizedCertificatesResponseBuilder {
            ListAuthorizedCertificatesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for AuthorizedDomains.ListAuthorizedDomains."]
    pub struct ListAuthorizedDomainsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domains")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The authorized domains belonging to the user."]
        pub domains: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuthorizedDomain>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token for fetching the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListAuthorizedDomainsResponse {
        pub fn builder() -> ListAuthorizedDomainsResponseBuilder {
            ListAuthorizedDomainsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for DomainMappings.ListDomainMappings."]
    pub struct ListDomainMappingsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domainMappings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The domain mappings for the application."]
        pub domain_mappings:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DomainMapping>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token for fetching the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListDomainMappingsResponse {
        pub fn builder() -> ListDomainMappingsResponseBuilder {
            ListDomainMappingsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for Firewall.ListIngressRules."]
    pub struct ListIngressRulesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ingressRules")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ingress FirewallRules for this application."]
        pub ingress_rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FirewallRule>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token for fetching the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListIngressRulesResponse {
        pub fn builder() -> ListIngressRulesResponseBuilder {
            ListIngressRulesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for Instances.ListInstances."]
    pub struct ListInstancesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instances")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The instances belonging to the requested version."]
        pub instances: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Instance>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token for fetching the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListInstancesResponse {
        pub fn builder() -> ListInstancesResponseBuilder {
            ListInstancesResponseBuilder::default()
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
    #[doc = "The response message for Operations.ListOperations."]
    pub struct ListOperationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The standard List next-page token."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of operations that matches the specified filter in the request."]
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
    #[doc = "Response message for Services.ListServices."]
    pub struct ListServicesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token for fetching the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "services")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The services belonging to the requested application."]
        pub services: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Service>>>,
    }
    impl ListServicesResponse {
        pub fn builder() -> ListServicesResponseBuilder {
            ListServicesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for Versions.ListVersions."]
    pub struct ListVersionsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token for fetching the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The versions belonging to the requested service."]
        pub versions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Version>>>,
    }
    impl ListVersionsResponse {
        pub fn builder() -> ListVersionsResponseBuilder {
            ListVersionsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Health checking configuration for VM instances. Unhealthy instances are killed and replaced with new instances."]
    pub struct LivenessCheck {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "checkInterval")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Interval between health checks."]
        pub check_interval: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "failureThreshold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of consecutive failed checks required before considering the VM unhealthy."]
        pub failure_threshold: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "host")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Host header to send when performing a HTTP Liveness check. Example: \"myapp.appspot.com\""]
        pub host: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "initialDelay")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The initial delay before starting to execute the checks."]
        pub initial_delay: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request path."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "successThreshold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of consecutive successful checks required before considering the VM healthy."]
        pub success_threshold: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time before the check is considered failed."]
        pub timeout: ::std::option::Option<::std::string::String>,
    }
    impl LivenessCheck {
        pub fn builder() -> LivenessCheckBuilder {
            LivenessCheckBuilder::default()
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
        #[doc = "Cross-service attributes for the location. For example {\"cloud.googleapis.com/region\": \"us-east1\"} "]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The canonical id for this location. For example: \"us-east1\"."]
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
        #[doc = "Resource name for the location, which may vary between implementations. For example: \"projects/example-project/locations/us-east1\""]
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
    #[doc = "Metadata for the given google.cloud.location.Location."]
    pub struct LocationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "flexibleEnvironmentAvailable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "App Engine flexible environment is available in the given location.@OutputOnly"]
        pub flexible_environment_available: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "searchApiAvailable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Search API (https://cloud.google.com/appengine/docs/standard/python/search) is available in the given location."]
        pub search_api_available: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "standardEnvironmentAvailable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "App Engine standard environment is available in the given location.@OutputOnly"]
        pub standard_environment_available: ::std::option::Option<::std::primitive::bool>,
    }
    impl LocationMetadata {
        pub fn builder() -> LocationMetadataBuilder {
            LocationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A certificate managed by App Engine."]
    pub struct ManagedCertificate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastRenewalTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time at which the certificate was last renewed. The renewal process is fully managed. Certificate renewal will automatically occur before the certificate expires. Renewal errors can be tracked via ManagementStatus.@OutputOnly"]
        pub last_renewal_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of certificate management. Refers to the most recent certificate acquisition or renewal attempt.@OutputOnly"]
        pub status: ::std::option::Option<ManagedCertificateStatusEnum>,
    }
    impl ManagedCertificate {
        pub fn builder() -> ManagedCertificateBuilder {
            ManagedCertificateBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Status of certificate management. Refers to the most recent certificate acquisition or renewal attempt.@OutputOnly"]
    pub enum ManagedCertificateStatusEnum {
        #[serde(rename = "MANAGEMENT_STATUS_UNSPECIFIED")]
        #[doc = ""]
        ManagementStatusUnspecified,
        #[serde(rename = "OK")]
        #[doc = "Certificate was successfully obtained and inserted into the serving system."]
        Ok,
        #[serde(rename = "PENDING")]
        #[doc = "Certificate is under active attempts to acquire or renew."]
        Pending,
        #[serde(rename = "FAILED_RETRYING_NOT_VISIBLE")]
        #[doc = "Most recent renewal failed due to an invalid DNS setup and will be retried. Renewal attempts will continue to fail until the certificate domain's DNS configuration is fixed. The last successfully provisioned certificate may still be serving."]
        FailedRetryingNotVisible,
        #[serde(rename = "FAILED_PERMANENT")]
        #[doc = "All renewal attempts have been exhausted, likely due to an invalid DNS setup."]
        FailedPermanent,
        #[serde(rename = "FAILED_RETRYING_CAA_FORBIDDEN")]
        #[doc = "Most recent renewal failed due to an explicit CAA record that does not include one of the in-use CAs (Google CA and Let's Encrypt). Renewals will continue to fail until the CAA is reconfigured. The last successfully provisioned certificate may still be serving."]
        FailedRetryingCaaForbidden,
        #[serde(rename = "FAILED_RETRYING_CAA_CHECKING")]
        #[doc = "Most recent renewal failed due to a CAA retrieval failure. This means that the domain's DNS provider does not properly handle CAA records, failing requests for CAA records when no CAA records are defined. Renewals will continue to fail until the DNS provider is changed or a CAA record is added for the given domain. The last successfully provisioned certificate may still be serving."]
        FailedRetryingCaaChecking,
    }
    impl ::std::default::Default for ManagedCertificateStatusEnum {
        fn default() -> Self {
            Self::ManagementStatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A service with manual scaling runs continuously, allowing you to perform complex initialization and rely on the state of its memory over time."]
    pub struct ManualScaling {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instances")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of instances to assign to the service at the start. This number can later be altered by using the Modules API (https://cloud.google.com/appengine/docs/python/modules/functions) set_num_instances() function."]
        pub instances: ::std::option::Option<::std::primitive::i64>,
    }
    impl ManualScaling {
        pub fn builder() -> ManualScalingBuilder {
            ManualScalingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Extra network settings. Only applicable in the App Engine flexible environment."]
    pub struct Network {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "forwardedPorts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of ports, or port pairs, to forward from the virtual machine to the application container. Only applicable in the App Engine flexible environment."]
        pub forwarded_ports: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instanceTag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tag to apply to the instance during creation. Only applicable in the App Engine flexible environment."]
        pub instance_tag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Compute Engine network where the virtual machines are created. Specify the short name, not the resource path.Defaults to default."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sessionAffinity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enable session affinity. Only applicable in the App Engine flexible environment."]
        pub session_affinity: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subnetworkName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Platform sub-network where the virtual machines are created. Specify the short name, not the resource path.If a subnetwork name is specified, a network name will also be required unless it is for the default network. If the network that the instance is being created in is a Legacy network, then the IP address is allocated from the IPv4Range. If the network that the instance is being created in is an auto Subnet Mode Network, then only network name should be specified (not the subnetwork_name) and the IP address is created from the IPCidrRange of the subnetwork that exists in that zone for that network. If the network that the instance is being created in is a custom Subnet Mode Network, then the subnetwork_name must be specified and the IP address is created from the IPCidrRange of the subnetwork.If specified, the subnetwork must exist in the same region as the App Engine flexible environment application."]
        pub subnetwork_name: ::std::option::Option<::std::string::String>,
    }
    impl Network {
        pub fn builder() -> NetworkBuilder {
            NetworkBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A NetworkSettings resource is a container for ingress settings for a version or service."]
    pub struct NetworkSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ingressTrafficAllowed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ingress settings for version or service."]
        pub ingress_traffic_allowed:
            ::std::option::Option<NetworkSettingsIngressTrafficAllowedEnum>,
    }
    impl NetworkSettings {
        pub fn builder() -> NetworkSettingsBuilder {
            NetworkSettingsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The ingress settings for version or service."]
    pub enum NetworkSettingsIngressTrafficAllowedEnum {
        #[serde(rename = "INGRESS_TRAFFIC_ALLOWED_UNSPECIFIED")]
        #[doc = "Unspecified"]
        IngressTrafficAllowedUnspecified,
        #[serde(rename = "INGRESS_TRAFFIC_ALLOWED_ALL")]
        #[doc = "Allow HTTP traffic from public and private sources."]
        IngressTrafficAllowedAll,
        #[serde(rename = "INGRESS_TRAFFIC_ALLOWED_INTERNAL_ONLY")]
        #[doc = "Allow HTTP traffic from only private VPC sources."]
        IngressTrafficAllowedInternalOnly,
        #[serde(rename = "INGRESS_TRAFFIC_ALLOWED_INTERNAL_AND_LB")]
        #[doc = "Allow HTTP traffic from private VPC sources and through load balancers."]
        IngressTrafficAllowedInternalAndLb,
    }
    impl ::std::default::Default for NetworkSettingsIngressTrafficAllowedEnum {
        fn default() -> Self {
            Self::IngressTrafficAllowedUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Target scaling by network usage. Only applicable in the App Engine flexible environment."]
    pub struct NetworkUtilization {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetReceivedBytesPerSecond")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Target bytes received per second."]
        pub target_received_bytes_per_second: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetReceivedPacketsPerSecond")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Target packets received per second."]
        pub target_received_packets_per_second: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetSentBytesPerSecond")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Target bytes sent per second."]
        pub target_sent_bytes_per_second: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetSentPacketsPerSecond")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Target packets sent per second."]
        pub target_sent_packets_per_second: ::std::option::Option<::std::primitive::i64>,
    }
    impl NetworkUtilization {
        pub fn builder() -> NetworkUtilizationBuilder {
            NetworkUtilizationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This resource represents a long-running operation that is the result of a network API call."]
    pub struct Operation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "done")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available."]
        pub done: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error result of the operation in case of failure or cancellation."]
        pub error: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any."]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the name should be a resource name ending with operations/{unique_id}."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "response")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The normal response of the operation in case of success. If the original method returns no data on success, such as Delete, the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type XxxResponse, where Xxx is the original method name. For example, if the original method name is TakeSnapshot(), the inferred response type is TakeSnapshotResponse."]
        pub response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl Operation {
        pub fn builder() -> OperationBuilder {
            OperationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for the given google.longrunning.Operation."]
    pub struct OperationMetadataV1 {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createVersionMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub create_version_metadata:
            ::std::option::Option<::std::boxed::Box<CreateVersionMetadataV1>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time that this operation completed.@OutputOnly"]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ephemeralMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ephemeral message that may change every time the operation is polled. @OutputOnly"]
        pub ephemeral_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time that this operation was created.@OutputOnly"]
        pub insert_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "API method that initiated this operation. Example: google.appengine.v1.Versions.CreateVersion.@OutputOnly"]
        pub method: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "target")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the resource that this operation is acting on. Example: apps/myapp/services/default.@OutputOnly"]
        pub target: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "user")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User who requested this operation.@OutputOnly"]
        pub user: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warning")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Durable messages that persist on every operation poll. @OutputOnly"]
        pub warning: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl OperationMetadataV1 {
        pub fn builder() -> OperationMetadataV1Builder {
            OperationMetadataV1Builder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for the given google.longrunning.Operation."]
    pub struct OperationMetadataV1Alpha {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createVersionMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub create_version_metadata:
            ::std::option::Option<::std::boxed::Box<CreateVersionMetadataV1Alpha>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time that this operation completed.@OutputOnly"]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ephemeralMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ephemeral message that may change every time the operation is polled. @OutputOnly"]
        pub ephemeral_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time that this operation was created.@OutputOnly"]
        pub insert_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "API method that initiated this operation. Example: google.appengine.v1alpha.Versions.CreateVersion.@OutputOnly"]
        pub method: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "target")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the resource that this operation is acting on. Example: apps/myapp/services/default.@OutputOnly"]
        pub target: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "user")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User who requested this operation.@OutputOnly"]
        pub user: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warning")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Durable messages that persist on every operation poll. @OutputOnly"]
        pub warning: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl OperationMetadataV1Alpha {
        pub fn builder() -> OperationMetadataV1AlphaBuilder {
            OperationMetadataV1AlphaBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for the given google.longrunning.Operation."]
    pub struct OperationMetadataV1Beta {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createVersionMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub create_version_metadata:
            ::std::option::Option<::std::boxed::Box<CreateVersionMetadataV1Beta>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time that this operation completed.@OutputOnly"]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ephemeralMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ephemeral message that may change every time the operation is polled. @OutputOnly"]
        pub ephemeral_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time that this operation was created.@OutputOnly"]
        pub insert_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "API method that initiated this operation. Example: google.appengine.v1beta.Versions.CreateVersion.@OutputOnly"]
        pub method: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "target")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the resource that this operation is acting on. Example: apps/myapp/services/default.@OutputOnly"]
        pub target: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "user")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User who requested this operation.@OutputOnly"]
        pub user: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warning")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Durable messages that persist on every operation poll. @OutputOnly"]
        pub warning: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl OperationMetadataV1Beta {
        pub fn builder() -> OperationMetadataV1BetaBuilder {
            OperationMetadataV1BetaBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Readiness checking configuration for VM instances. Unhealthy instances are removed from traffic rotation."]
    pub struct ReadinessCheck {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appStartTimeout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A maximum time limit on application initialization, measured from moment the application successfully replies to a healthcheck until it is ready to serve traffic."]
        pub app_start_timeout: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "checkInterval")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Interval between health checks."]
        pub check_interval: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "failureThreshold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of consecutive failed checks required before removing traffic."]
        pub failure_threshold: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "host")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Host header to send when performing a HTTP Readiness check. Example: \"myapp.appspot.com\""]
        pub host: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request path."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "successThreshold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of consecutive successful checks required before receiving traffic."]
        pub success_threshold: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time before the check is considered failed."]
        pub timeout: ::std::option::Option<::std::string::String>,
    }
    impl ReadinessCheck {
        pub fn builder() -> ReadinessCheckBuilder {
            ReadinessCheckBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for 'Applications.RepairApplication'."]
    pub struct RepairApplicationRequest {}
    impl RepairApplicationRequest {
        pub fn builder() -> RepairApplicationRequestBuilder {
            RepairApplicationRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Target scaling by request utilization. Only applicable in the App Engine flexible environment."]
    pub struct RequestUtilization {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetConcurrentRequests")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Target number of concurrent requests."]
        pub target_concurrent_requests: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetRequestCountPerSecond")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Target requests per second."]
        pub target_request_count_per_second: ::std::option::Option<::std::primitive::i64>,
    }
    impl RequestUtilization {
        pub fn builder() -> RequestUtilizationBuilder {
            RequestUtilizationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A DNS resource record."]
    pub struct ResourceRecord {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Relative name of the object affected by this record. Only applicable for CNAME records. Example: 'www'."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rrdata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data for this record. Values vary by record type, as defined in RFC 1035 (section 5) and RFC 1034 (section 3.6.1)."]
        pub rrdata: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource record type. Example: AAAA."]
        pub _type: ::std::option::Option<ResourceRecordTypeEnum>,
    }
    impl ResourceRecord {
        pub fn builder() -> ResourceRecordBuilder {
            ResourceRecordBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Resource record type. Example: AAAA."]
    pub enum ResourceRecordTypeEnum {
        #[serde(rename = "RECORD_TYPE_UNSPECIFIED")]
        #[doc = "An unknown resource record."]
        RecordTypeUnspecified,
        #[serde(rename = "A")]
        #[doc = "An A resource record. Data is an IPv4 address."]
        A,
        #[serde(rename = "AAAA")]
        #[doc = "An AAAA resource record. Data is an IPv6 address."]
        Aaaa,
        #[serde(rename = "CNAME")]
        #[doc = "A CNAME resource record. Data is a domain name to be aliased."]
        Cname,
    }
    impl ::std::default::Default for ResourceRecordTypeEnum {
        fn default() -> Self {
            Self::RecordTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Machine resources for a version."]
    pub struct Resources {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpu")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of CPU cores needed."]
        pub cpu: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diskGb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Disk size (GB) needed."]
        pub disk_gb: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kmsKeyReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the encryption key that is stored in Google Cloud KMS. Only should be used by Cloud Composer to encrypt the vm disk"]
        pub kms_key_reference: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "memoryGb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Memory (GB) needed."]
        pub memory_gb: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User specified volumes."]
        pub volumes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Volume>>>,
    }
    impl Resources {
        pub fn builder() -> ResourcesBuilder {
            ResourcesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Executes a script to handle the request that matches the URL pattern."]
    pub struct ScriptHandler {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scriptPath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Path to the script from the application root directory."]
        pub script_path: ::std::option::Option<::std::string::String>,
    }
    impl ScriptHandler {
        pub fn builder() -> ScriptHandlerBuilder {
            ScriptHandlerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Service resource is a logical component of an application that can share state and communicate in a secure fashion with other services. For example, an application that handles customer requests might include separate services to handle tasks such as backend data analysis or API requests from mobile devices. Each service has a collection of versions that define a specific set of code used to implement the functionality of that service."]
    pub struct Service {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Relative name of the service within the application. Example: default.@OutputOnly"]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Full path to the Service resource in the API. Example: apps/myapp/services/default.@OutputOnly"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "networkSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ingress settings for this service. Will apply to all versions."]
        pub network_settings: ::std::option::Option<::std::boxed::Box<NetworkSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "split")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mapping that defines fractional HTTP traffic diversion to different versions within the service."]
        pub split: ::std::option::Option<::std::boxed::Box<TrafficSplit>>,
    }
    impl Service {
        pub fn builder() -> ServiceBuilder {
            ServiceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SSL configuration for a DomainMapping resource."]
    pub struct SslSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "certificateId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the AuthorizedCertificate resource configuring SSL for the application. Clearing this field will remove SSL support.By default, a managed certificate is automatically created for every domain mapping. To omit SSL support or to configure SSL manually, specify SslManagementType.MANUAL on a CREATE or UPDATE request. You must be authorized to administer the AuthorizedCertificate resource to manually map it to a DomainMapping resource. Example: 12345."]
        pub certificate_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pendingManagedCertificateId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the managed AuthorizedCertificate resource currently being provisioned, if applicable. Until the new managed certificate has been successfully provisioned, the previous SSL state will be preserved. Once the provisioning process completes, the certificate_id field will reflect the new managed certificate and this field will be left empty. To remove SSL support while there is still a pending managed certificate, clear the certificate_id field with an UpdateDomainMappingRequest.@OutputOnly"]
        pub pending_managed_certificate_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sslManagementType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "SSL management type for this domain. If AUTOMATIC, a managed certificate is automatically provisioned. If MANUAL, certificate_id must be manually specified in order to configure SSL for this domain."]
        pub ssl_management_type: ::std::option::Option<SslSettingsSslManagementTypeEnum>,
    }
    impl SslSettings {
        pub fn builder() -> SslSettingsBuilder {
            SslSettingsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "SSL management type for this domain. If AUTOMATIC, a managed certificate is automatically provisioned. If MANUAL, certificate_id must be manually specified in order to configure SSL for this domain."]
    pub enum SslSettingsSslManagementTypeEnum {
        #[serde(rename = "SSL_MANAGEMENT_TYPE_UNSPECIFIED")]
        #[doc = "Defaults to AUTOMATIC."]
        SslManagementTypeUnspecified,
        #[serde(rename = "AUTOMATIC")]
        #[doc = "SSL support for this domain is configured automatically. The mapped SSL certificate will be automatically renewed."]
        Automatic,
        #[serde(rename = "MANUAL")]
        #[doc = "SSL support for this domain is configured manually by the user. Either the domain has no SSL support or a user-obtained SSL certificate has been explictly mapped to this domain."]
        Manual,
    }
    impl ::std::default::Default for SslSettingsSslManagementTypeEnum {
        fn default() -> Self {
            Self::SslManagementTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Scheduler settings for standard environment."]
    pub struct StandardSchedulerSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxInstances")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum number of instances to run for this version. Set to zero to disable max_instances configuration."]
        pub max_instances: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minInstances")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum number of instances to run for this version. Set to zero to disable min_instances configuration."]
        pub min_instances: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetCpuUtilization")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Target CPU utilization ratio to maintain when scaling."]
        pub target_cpu_utilization: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetThroughputUtilization")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Target throughput utilization ratio to maintain when scaling"]
        pub target_throughput_utilization: ::std::option::Option<::std::primitive::f64>,
    }
    impl StandardSchedulerSettings {
        pub fn builder() -> StandardSchedulerSettingsBuilder {
            StandardSchedulerSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Files served directly to the user for a given URL, such as images, CSS stylesheets, or JavaScript source files. Static file handlers describe which files in the application directory are static files, and which URLs serve them."]
    pub struct StaticFilesHandler {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "applicationReadable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether files should also be uploaded as code data. By default, files declared in static file handlers are uploaded as static data and are only served to end users; they cannot be read by the application. If enabled, uploads are charged against both your code and static data storage resource quotas."]
        pub application_readable: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expiration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time a static file served by this handler should be cached by web proxies and browsers."]
        pub expiration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "httpHeaders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "HTTP headers to use for all responses from these URLs."]
        pub http_headers:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "MIME type used to serve all files served by this handler.Defaults to file-specific MIME types, which are derived from each file's filename extension."]
        pub mime_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Path to the static files matched by the URL pattern, from the application root directory. The path can refer to text matched in groupings in the URL pattern."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requireMatchingFile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this handler should match the request if the file referenced by the handler does not exist."]
        pub require_matching_file: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uploadPathRegex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Regular expression that matches the file paths for all files that should be referenced by this handler."]
        pub upload_path_regex: ::std::option::Option<::std::string::String>,
    }
    impl StaticFilesHandler {
        pub fn builder() -> StaticFilesHandlerBuilder {
            StaticFilesHandlerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Status type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by gRPC (https://github.com/grpc). Each Status message contains three pieces of data: error code, error message, and error details.You can find out more about this error model and how to work with it in the API Design Guide (https://cloud.google.com/apis/design/errors)."]
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
    #[doc = "Traffic routing configuration for versions within a single service. Traffic splits define how traffic directed to the service is assigned to versions."]
    pub struct TrafficSplit {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allocations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mapping from version IDs within the service to fractional (0.000, 1] allocations of traffic for that version. Each version can be specified only once, but some versions in the service may not have any traffic allocation. Services that have traffic allocated cannot be deleted until either the service is deleted or their traffic allocation is removed. Allocations must sum to 1. Up to two decimal place precision is supported for IP-based splits and up to three decimal places is supported for cookie-based splits."]
        pub allocations:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::primitive::f64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shardBy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mechanism used to determine which version a request is sent to. The traffic selection algorithm will be stable for either type until allocations are changed."]
        pub shard_by: ::std::option::Option<TrafficSplitShardByEnum>,
    }
    impl TrafficSplit {
        pub fn builder() -> TrafficSplitBuilder {
            TrafficSplitBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Mechanism used to determine which version a request is sent to. The traffic selection algorithm will be stable for either type until allocations are changed."]
    pub enum TrafficSplitShardByEnum {
        #[serde(rename = "UNSPECIFIED")]
        #[doc = "Diversion method unspecified."]
        Unspecified,
        #[serde(rename = "COOKIE")]
        #[doc = "Diversion based on a specially named cookie, \"GOOGAPPUID.\" The cookie must be set by the application itself or no diversion will occur."]
        Cookie,
        #[serde(rename = "IP")]
        #[doc = "Diversion based on applying the modulus operation to a fingerprint of the IP address."]
        Ip,
        #[serde(rename = "RANDOM")]
        #[doc = "Diversion based on weighted random assignment. An incoming request is randomly routed to a version in the traffic split, with probability proportional to the version's traffic share."]
        Random,
    }
    impl ::std::default::Default for TrafficSplitShardByEnum {
        fn default() -> Self {
            Self::Unspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Rules to match an HTTP request and dispatch that request to a service."]
    pub struct UrlDispatchRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Domain name to match against. The wildcard \"*\" is supported if specified before a period: \"*.\".Defaults to matching all domains: \"*\"."]
        pub domain: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pathname within the host. Must start with a \"/\". A single \"*\" can be included at the end of the path.The sum of the lengths of the domain and path may not exceed 100 characters."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "service")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource ID of a service in this application that should serve the matched request. The service must already exist. Example: default."]
        pub service: ::std::option::Option<::std::string::String>,
    }
    impl UrlDispatchRule {
        pub fn builder() -> UrlDispatchRuleBuilder {
            UrlDispatchRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "URL pattern and description of how the URL should be handled. App Engine can handle URLs by executing application code or by serving static files uploaded with the version, such as images, CSS, or JavaScript."]
    pub struct UrlMap {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiEndpoint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uses API Endpoints to handle requests."]
        pub api_endpoint: ::std::option::Option<::std::boxed::Box<ApiEndpointHandler>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authFailAction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Action to take when users access resources that require authentication. Defaults to redirect."]
        pub auth_fail_action: ::std::option::Option<UrlMapAuthFailActionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "login")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Level of login required to access this resource. Not supported for Node.js in the App Engine standard environment."]
        pub login: ::std::option::Option<UrlMapLoginEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "redirectHttpResponseCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "30x code to use when performing redirects for the secure field. Defaults to 302."]
        pub redirect_http_response_code: ::std::option::Option<UrlMapRedirectHttpResponseCodeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "script")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Executes a script to handle the requests that match this URL pattern. Only the auto value is supported for Node.js in the App Engine standard environment, for example \"script\": \"auto\"."]
        pub script: ::std::option::Option<::std::boxed::Box<ScriptHandler>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "securityLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Security (HTTPS) enforcement for this URL."]
        pub security_level: ::std::option::Option<UrlMapSecurityLevelEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "staticFiles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Returns the contents of a file, such as an image, as the response."]
        pub static_files: ::std::option::Option<::std::boxed::Box<StaticFilesHandler>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "urlRegex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL prefix. Uses regular expression syntax, which means regexp special characters must be escaped, but should not contain groupings. All URLs that begin with this prefix are handled by this handler, using the portion of the URL after the prefix as part of the file path."]
        pub url_regex: ::std::option::Option<::std::string::String>,
    }
    impl UrlMap {
        pub fn builder() -> UrlMapBuilder {
            UrlMapBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Action to take when users access resources that require authentication. Defaults to redirect."]
    pub enum UrlMapAuthFailActionEnum {
        #[serde(rename = "AUTH_FAIL_ACTION_UNSPECIFIED")]
        #[doc = "Not specified. AUTH_FAIL_ACTION_REDIRECT is assumed."]
        AuthFailActionUnspecified,
        #[serde(rename = "AUTH_FAIL_ACTION_REDIRECT")]
        #[doc = "Redirects user to \"accounts.google.com\". The user is redirected back to the application URL after signing in or creating an account."]
        AuthFailActionRedirect,
        #[serde(rename = "AUTH_FAIL_ACTION_UNAUTHORIZED")]
        #[doc = "Rejects request with a 401 HTTP status code and an error message."]
        AuthFailActionUnauthorized,
    }
    impl ::std::default::Default for UrlMapAuthFailActionEnum {
        fn default() -> Self {
            Self::AuthFailActionUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Level of login required to access this resource. Not supported for Node.js in the App Engine standard environment."]
    pub enum UrlMapLoginEnum {
        #[serde(rename = "LOGIN_UNSPECIFIED")]
        #[doc = "Not specified. LOGIN_OPTIONAL is assumed."]
        LoginUnspecified,
        #[serde(rename = "LOGIN_OPTIONAL")]
        #[doc = "Does not require that the user is signed in."]
        LoginOptional,
        #[serde(rename = "LOGIN_ADMIN")]
        #[doc = "If the user is not signed in, the auth_fail_action is taken. In addition, if the user is not an administrator for the application, they are given an error message regardless of auth_fail_action. If the user is an administrator, the handler proceeds."]
        LoginAdmin,
        #[serde(rename = "LOGIN_REQUIRED")]
        #[doc = "If the user has signed in, the handler proceeds normally. Otherwise, the auth_fail_action is taken."]
        LoginRequired,
    }
    impl ::std::default::Default for UrlMapLoginEnum {
        fn default() -> Self {
            Self::LoginUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "30x code to use when performing redirects for the secure field. Defaults to 302."]
    pub enum UrlMapRedirectHttpResponseCodeEnum {
        #[serde(rename = "REDIRECT_HTTP_RESPONSE_CODE_UNSPECIFIED")]
        #[doc = "Not specified. 302 is assumed."]
        RedirectHttpResponseCodeUnspecified,
        #[serde(rename = "REDIRECT_HTTP_RESPONSE_CODE_301")]
        #[doc = "301 Moved Permanently code."]
        RedirectHttpResponseCode301,
        #[serde(rename = "REDIRECT_HTTP_RESPONSE_CODE_302")]
        #[doc = "302 Moved Temporarily code."]
        RedirectHttpResponseCode302,
        #[serde(rename = "REDIRECT_HTTP_RESPONSE_CODE_303")]
        #[doc = "303 See Other code."]
        RedirectHttpResponseCode303,
        #[serde(rename = "REDIRECT_HTTP_RESPONSE_CODE_307")]
        #[doc = "307 Temporary Redirect code."]
        RedirectHttpResponseCode307,
    }
    impl ::std::default::Default for UrlMapRedirectHttpResponseCodeEnum {
        fn default() -> Self {
            Self::RedirectHttpResponseCodeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Security (HTTPS) enforcement for this URL."]
    pub enum UrlMapSecurityLevelEnum {
        #[serde(rename = "SECURE_UNSPECIFIED")]
        #[doc = "Not specified."]
        SecureUnspecified,
        #[serde(rename = "SECURE_DEFAULT")]
        #[doc = "Both HTTP and HTTPS requests with URLs that match the handler succeed without redirects. The application can examine the request to determine which protocol was used, and respond accordingly."]
        SecureDefault,
        #[serde(rename = "SECURE_NEVER")]
        #[doc = "Requests for a URL that match this handler that use HTTPS are automatically redirected to the HTTP equivalent URL."]
        SecureNever,
        #[serde(rename = "SECURE_OPTIONAL")]
        #[doc = "Both HTTP and HTTPS requests with URLs that match the handler succeed without redirects. The application can examine the request to determine which protocol was used and respond accordingly."]
        SecureOptional,
        #[serde(rename = "SECURE_ALWAYS")]
        #[doc = "Requests for a URL that match this handler that do not use HTTPS are automatically redirected to the HTTPS URL with the same path. Query parameters are reserved for the redirect."]
        SecureAlways,
    }
    impl ::std::default::Default for UrlMapSecurityLevelEnum {
        fn default() -> Self {
            Self::SecureUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Version resource is a specific set of source code and configuration files that are deployed into a service."]
    pub struct Version {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serving configuration for Google Cloud Endpoints (https://cloud.google.com/appengine/docs/python/endpoints/).Only returned in GET requests if view=FULL is set."]
        pub api_config: ::std::option::Option<::std::boxed::Box<ApiConfigHandler>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "automaticScaling")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Automatic scaling is based on request rate, response latencies, and other application metrics. Instances are dynamically created and destroyed as needed in order to handle traffic."]
        pub automatic_scaling: ::std::option::Option<::std::boxed::Box<AutomaticScaling>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicScaling")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A service with basic scaling will create an instance when the application receives a request. The instance will be turned down when the app becomes idle. Basic scaling is ideal for work that is intermittent or driven by user activity."]
        pub basic_scaling: ::std::option::Option<::std::boxed::Box<BasicScaling>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "betaSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata settings that are supplied to this version to enable beta runtime features."]
        pub beta_settings:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buildEnvVariables")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Environment variables available to the build environment.Only returned in GET requests if view=FULL is set."]
        pub build_env_variables:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time that this version was created.@OutputOnly"]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdBy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email address of the user who created this version.@OutputOnly"]
        pub created_by: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultExpiration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Duration that static files should be cached by web proxies and browsers. Only applicable if the corresponding StaticFilesHandler (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#StaticFilesHandler) does not specify its own expiration time.Only returned in GET requests if view=FULL is set."]
        pub default_expiration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deployment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Code and application artifacts that make up this version.Only returned in GET requests if view=FULL is set."]
        pub deployment: ::std::option::Option<::std::boxed::Box<Deployment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diskUsageBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total size in bytes of all the files that are included in this version and currently hosted on the App Engine disk.@OutputOnly"]
        pub disk_usage_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endpointsApiService")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cloud Endpoints configuration.If endpoints_api_service is set, the Cloud Endpoints Extensible Service Proxy will be provided to serve the API implemented by the app."]
        pub endpoints_api_service: ::std::option::Option<::std::boxed::Box<EndpointsApiService>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entrypoint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entrypoint for the application."]
        pub entrypoint: ::std::option::Option<::std::boxed::Box<Entrypoint>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "env")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "App Engine execution environment for this version.Defaults to standard."]
        pub env: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "envVariables")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Environment variables available to the application.Only returned in GET requests if view=FULL is set."]
        pub env_variables:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorHandlers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom static error pages. Limited to 10KB per page.Only returned in GET requests if view=FULL is set."]
        pub error_handlers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ErrorHandler>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "handlers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An ordered list of URL-matching patterns that should be applied to incoming requests. The first matching URL handles the request and other request handlers are not attempted.Only returned in GET requests if view=FULL is set."]
        pub handlers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UrlMap>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "healthCheck")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configures health checking for instances. Unhealthy instances are stopped and replaced with new instances. Only applicable in the App Engine flexible environment.Only returned in GET requests if view=FULL is set."]
        pub health_check: ::std::option::Option<::std::boxed::Box<HealthCheck>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Relative name of the version within the service. Example: v1. Version names can contain only lowercase letters, numbers, or hyphens. Reserved names: \"default\", \"latest\", and any name with the prefix \"ah-\"."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inboundServices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Before an application can receive email or XMPP messages, the application must be configured to enable the service."]
        pub inbound_services: ::std::option::Option<::std::vec::Vec<VersionInboundServicesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instanceClass")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Instance class that is used to run this version. Valid values are: AutomaticScaling: F1, F2, F4, F4_1G ManualScaling or BasicScaling: B1, B2, B4, B8, B4_1GDefaults to F1 for AutomaticScaling and B1 for ManualScaling or BasicScaling."]
        pub instance_class: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "libraries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for third-party Python runtime libraries that are required by the application.Only returned in GET requests if view=FULL is set."]
        pub libraries: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Library>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "livenessCheck")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configures liveness health checking for instances. Unhealthy instances are stopped and replaced with new instancesOnly returned in GET requests if view=FULL is set."]
        pub liveness_check: ::std::option::Option<::std::boxed::Box<LivenessCheck>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "manualScaling")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A service with manual scaling runs continuously, allowing you to perform complex initialization and rely on the state of its memory over time. Manually scaled versions are sometimes referred to as \"backends\"."]
        pub manual_scaling: ::std::option::Option<::std::boxed::Box<ManualScaling>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Full path to the Version resource in the API. Example: apps/myapp/services/default/versions/v1.@OutputOnly"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "network")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Extra network settings. Only applicable in the App Engine flexible environment."]
        pub network: ::std::option::Option<::std::boxed::Box<Network>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nobuildFilesRegex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Files that match this pattern will not be built into this version. Only applicable for Go runtimes.Only returned in GET requests if view=FULL is set."]
        pub nobuild_files_regex: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "readinessCheck")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configures readiness health checking for instances. Unhealthy instances are not put into the backend traffic rotation.Only returned in GET requests if view=FULL is set."]
        pub readiness_check: ::std::option::Option<::std::boxed::Box<ReadinessCheck>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Machine resources for this version. Only applicable in the App Engine flexible environment."]
        pub resources: ::std::option::Option<::std::boxed::Box<Resources>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "runtime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Desired runtime. Example: python27."]
        pub runtime: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "runtimeApiVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version of the API in the given runtime environment. Please see the app.yaml reference for valid values at https://cloud.google.com/appengine/docs/standard//config/appref"]
        pub runtime_api_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "runtimeChannel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The channel of the runtime to use. Only available for some runtimes. Defaults to the default channel."]
        pub runtime_channel: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "runtimeMainExecutablePath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The path or name of the app's main executable."]
        pub runtime_main_executable_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "servingStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Current serving status of this version. Only the versions with a SERVING status create instances and can be billed.SERVING_STATUS_UNSPECIFIED is an invalid value. Defaults to SERVING."]
        pub serving_status: ::std::option::Option<VersionServingStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threadsafe")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether multiple requests can be dispatched to this version at once."]
        pub threadsafe: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serving URL for this version. Example: \"https://myversion-dot-myservice-dot-myapp.appspot.com\"@OutputOnly"]
        pub version_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vm")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to deploy this version in a container on a virtual machine."]
        pub vm: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vpcAccessConnector")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enables VPC connectivity for standard apps."]
        pub vpc_access_connector: ::std::option::Option<::std::boxed::Box<VpcAccessConnector>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zones")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Google Compute Engine zones that are supported by this version in the App Engine flexible environment. Deprecated."]
        pub zones: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl Version {
        pub fn builder() -> VersionBuilder {
            VersionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum VersionInboundServicesEnum {
        #[serde(rename = "INBOUND_SERVICE_UNSPECIFIED")]
        #[doc = "Not specified."]
        InboundServiceUnspecified,
        #[serde(rename = "INBOUND_SERVICE_MAIL")]
        #[doc = "Allows an application to receive mail."]
        InboundServiceMail,
        #[serde(rename = "INBOUND_SERVICE_MAIL_BOUNCE")]
        #[doc = "Allows an application to receive email-bound notifications."]
        InboundServiceMailBounce,
        #[serde(rename = "INBOUND_SERVICE_XMPP_ERROR")]
        #[doc = "Allows an application to receive error stanzas."]
        InboundServiceXmppError,
        #[serde(rename = "INBOUND_SERVICE_XMPP_MESSAGE")]
        #[doc = "Allows an application to receive instant messages."]
        InboundServiceXmppMessage,
        #[serde(rename = "INBOUND_SERVICE_XMPP_SUBSCRIBE")]
        #[doc = "Allows an application to receive user subscription POSTs."]
        InboundServiceXmppSubscribe,
        #[serde(rename = "INBOUND_SERVICE_XMPP_PRESENCE")]
        #[doc = "Allows an application to receive a user's chat presence."]
        InboundServiceXmppPresence,
        #[serde(rename = "INBOUND_SERVICE_CHANNEL_PRESENCE")]
        #[doc = "Registers an application for notifications when a client connects or disconnects from a channel."]
        InboundServiceChannelPresence,
        #[serde(rename = "INBOUND_SERVICE_WARMUP")]
        #[doc = "Enables warmup requests."]
        InboundServiceWarmup,
    }
    impl ::std::default::Default for VersionInboundServicesEnum {
        fn default() -> Self {
            Self::InboundServiceUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Current serving status of this version. Only the versions with a SERVING status create instances and can be billed.SERVING_STATUS_UNSPECIFIED is an invalid value. Defaults to SERVING."]
    pub enum VersionServingStatusEnum {
        #[serde(rename = "SERVING_STATUS_UNSPECIFIED")]
        #[doc = "Not specified."]
        ServingStatusUnspecified,
        #[serde(rename = "SERVING")]
        #[doc = "Currently serving. Instances are created according to the scaling settings of the version."]
        Serving,
        #[serde(rename = "STOPPED")]
        #[doc = "Disabled. No instances will be created and the scaling settings are ignored until the state of the version changes to SERVING."]
        Stopped,
    }
    impl ::std::default::Default for VersionServingStatusEnum {
        fn default() -> Self {
            Self::ServingStatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Volumes mounted within the app container. Only applicable in the App Engine flexible environment."]
    pub struct Volume {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique name for the volume."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sizeGb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Volume size in gigabytes."]
        pub size_gb: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Underlying volume type, e.g. 'tmpfs'."]
        pub volume_type: ::std::option::Option<::std::string::String>,
    }
    impl Volume {
        pub fn builder() -> VolumeBuilder {
            VolumeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "VPC access connector specification."]
    pub struct VpcAccessConnector {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Full Serverless VPC Access Connector name e.g. /projects/my-project/locations/us-central1/connectors/c1."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl VpcAccessConnector {
        pub fn builder() -> VpcAccessConnectorBuilder {
            VpcAccessConnectorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The zip file information for a zip deployment."]
    pub struct ZipInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filesCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An estimate of the number of files in a zip for a zip deployment. If set, must be greater than or equal to the actual number of files. Used for optimizing performance; if not provided, deployment may be slow."]
        pub files_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL of the zip file to deploy from. Must be a URL to a resource in Google Cloud Storage in the form 'http(s)://storage.googleapis.com//'."]
        pub source_url: ::std::option::Option<::std::string::String>,
    }
    impl ZipInfo {
        pub fn builder() -> ZipInfoBuilder {
            ZipInfoBuilder::default()
        }
    }
}
