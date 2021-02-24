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
            pub mod scan_configs {
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
                            #[doc = "The maximum number of ScanConfigs to return, can be limited by server. If not specified or not positive, the implementation will select a reasonable value."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A token identifying a page of results to be returned. This should be a `next_page_token` value returned from a previous List request. If unspecified, the first page of results is returned."]
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
                            #[doc = "Required. The update mask applies to the resource. For the `FieldMask` definition, see https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask"]
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
                    pub mod scan_runs {
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
                                    #[doc = "The maximum number of ScanRuns to return, can be limited by server. If not specified or not positive, the implementation will select a reasonable value."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A token identifying a page of results to be returned. This should be a `next_page_token` value returned from a previous List request. If unspecified, the first page of results is returned."]
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
                            pub mod crawled_urls {
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
                                            #[doc = "The maximum number of CrawledUrls to return, can be limited by server. If not specified or not positive, the implementation will select a reasonable value."]
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
                                            #[doc = "A token identifying a page of results to be returned. This should be a `next_page_token` value returned from a previous List request. If unspecified, the first page of results is returned."]
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
                            pub mod findings {
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
                                            #[doc = "The filter expression. The expression must be in the format: . Supported field: 'finding_type'. Supported operator: '='."]
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
                                            #[doc = "The maximum number of Findings to return, can be limited by server. If not specified or not positive, the implementation will select a reasonable value."]
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
                                            #[doc = "A token identifying a page of results to be returned. This should be a `next_page_token` value returned from a previous List request. If unspecified, the first page of results is returned."]
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
    #[doc = "Scan authentication configuration."]
    pub struct Authentication {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customAccount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Authentication using a custom account."]
        pub custom_account: ::std::option::Option<::std::boxed::Box<CustomAccount>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "googleAccount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Authentication using a Google account."]
        pub google_account: ::std::option::Option<::std::boxed::Box<GoogleAccount>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iapCredential")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Authentication using Identity-Aware-Proxy (IAP)."]
        pub iap_credential: ::std::option::Option<::std::boxed::Box<IapCredential>>,
    }
    impl Authentication {
        pub fn builder() -> AuthenticationBuilder {
            AuthenticationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A CrawledUrl resource represents a URL that was crawled during a ScanRun. Web Security Scanner Service crawls the web applications, following all links within the scope of sites, to find the URLs to test against."]
    pub struct CrawledUrl {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "body")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The body of the request that was used to visit the URL."]
        pub body: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "httpMethod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The http method of the request that was used to visit the URL, in uppercase."]
        pub http_method: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The URL that was crawled."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl CrawledUrl {
        pub fn builder() -> CrawledUrlBuilder {
            CrawledUrlBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes authentication configuration that uses a custom account."]
    pub struct CustomAccount {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "loginUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The login form URL of the website."]
        pub login_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "password")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Input only. The password of the custom account. The credential is stored encrypted and not returned in any response nor included in audit logs."]
        pub password: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "username")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The user name of the custom account."]
        pub username: ::std::option::Option<::std::string::String>,
    }
    impl CustomAccount {
        pub fn builder() -> CustomAccountBuilder {
            CustomAccountBuilder::default()
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
    #[doc = "A Finding resource represents a vulnerability instance identified during a ScanRun."]
    pub struct Finding {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "body")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The body of the request that triggered the vulnerability."]
        pub body: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The description of the vulnerability."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "finalUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The URL where the browser lands when the vulnerability is detected."]
        pub final_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "findingType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The type of the Finding. Detailed and up-to-date information on findings can be found here: https://cloud.google.com/security-command-center/docs/how-to-remediate-web-security-scanner-findings"]
        pub finding_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "form")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. An addon containing information reported for a vulnerability with an HTML form, if any."]
        pub form: ::std::option::Option<::std::boxed::Box<Form>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frameUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. If the vulnerability was originated from nested IFrame, the immediate parent IFrame is reported."]
        pub frame_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fuzzedUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The URL produced by the server-side fuzzer and used in the request that triggered the vulnerability."]
        pub fuzzed_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "httpMethod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The http method of the request that triggered the vulnerability, in uppercase."]
        pub http_method: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The resource name of the Finding. The name follows the format of 'projects/{projectId}/scanConfigs/{scanConfigId}/scanruns/{scanRunId}/findings/{findingId}'. The finding IDs are generated by the system."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outdatedLibrary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. An addon containing information about outdated libraries."]
        pub outdated_library: ::std::option::Option<::std::boxed::Box<OutdatedLibrary>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reproductionUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The URL containing human-readable payload that user can leverage to reproduce the vulnerability."]
        pub reproduction_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "severity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The severity level of the reported vulnerability."]
        pub severity: ::std::option::Option<FindingSeverityEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trackingId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The tracking ID uniquely identifies a vulnerability instance across multiple ScanRuns."]
        pub tracking_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "violatingResource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. An addon containing detailed information regarding any resource causing the vulnerability such as JavaScript sources, image, audio files, etc."]
        pub violating_resource: ::std::option::Option<::std::boxed::Box<ViolatingResource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vulnerableHeaders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. An addon containing information about vulnerable or missing HTTP headers."]
        pub vulnerable_headers: ::std::option::Option<::std::boxed::Box<VulnerableHeaders>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vulnerableParameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. An addon containing information about request parameters which were found to be vulnerable."]
        pub vulnerable_parameters: ::std::option::Option<::std::boxed::Box<VulnerableParameters>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "xss")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. An addon containing information reported for an XSS, if any."]
        pub xss: ::std::option::Option<::std::boxed::Box<Xss>>,
    }
    impl Finding {
        pub fn builder() -> FindingBuilder {
            FindingBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The severity level of the reported vulnerability."]
    pub enum FindingSeverityEnum {
        #[serde(rename = "SEVERITY_UNSPECIFIED")]
        #[doc = "No severity specified. The default value."]
        SeverityUnspecified,
        #[serde(rename = "CRITICAL")]
        #[doc = "Critical severity."]
        Critical,
        #[serde(rename = "HIGH")]
        #[doc = "High severity."]
        High,
        #[serde(rename = "MEDIUM")]
        #[doc = "Medium severity."]
        Medium,
        #[serde(rename = "LOW")]
        #[doc = "Low severity."]
        Low,
    }
    impl ::std::default::Default for FindingSeverityEnum {
        fn default() -> Self {
            Self::SeverityUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A FindingTypeStats resource represents stats regarding a specific FindingType of Findings under a given ScanRun."]
    pub struct FindingTypeStats {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "findingCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The count of findings belonging to this finding type."]
        pub finding_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "findingType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The finding type associated with the stats."]
        pub finding_type: ::std::option::Option<::std::string::String>,
    }
    impl FindingTypeStats {
        pub fn builder() -> FindingTypeStatsBuilder {
            FindingTypeStatsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "! Information about a vulnerability with an HTML."]
    pub struct Form {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actionUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "! The URI where to send the form when it's submitted."]
        pub action_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "! The names of form fields related to the vulnerability."]
        pub fields: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl Form {
        pub fn builder() -> FormBuilder {
            FormBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes authentication configuration that uses a Google account."]
    pub struct GoogleAccount {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "password")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Input only. The password of the Google account. The credential is stored encrypted and not returned in any response nor included in audit logs."]
        pub password: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "username")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The user name of the Google account."]
        pub username: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAccount {
        pub fn builder() -> GoogleAccountBuilder {
            GoogleAccountBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a HTTP Header."]
    pub struct Header {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Header name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Header value."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl Header {
        pub fn builder() -> HeaderBuilder {
            HeaderBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes authentication configuration for Identity-Aware-Proxy (IAP)."]
    pub struct IapCredential {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iapTestServiceAccountInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Authentication configuration when Web-Security-Scanner service account is added in Identity-Aware-Proxy (IAP) access policies."]
        pub iap_test_service_account_info:
            ::std::option::Option<::std::boxed::Box<IapTestServiceAccountInfo>>,
    }
    impl IapCredential {
        pub fn builder() -> IapCredentialBuilder {
            IapCredentialBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes authentication configuration when Web-Security-Scanner service account is added in Identity-Aware-Proxy (IAP) access policies."]
    pub struct IapTestServiceAccountInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetAudienceClientId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Describes OAuth2 client id of resources protected by Identity-Aware-Proxy (IAP)."]
        pub target_audience_client_id: ::std::option::Option<::std::string::String>,
    }
    impl IapTestServiceAccountInfo {
        pub fn builder() -> IapTestServiceAccountInfoBuilder {
            IapTestServiceAccountInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for the `ListCrawledUrls` method."]
    pub struct ListCrawledUrlsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "crawledUrls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of CrawledUrls returned."]
        pub crawled_urls: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CrawledUrl>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListCrawledUrlsResponse {
        pub fn builder() -> ListCrawledUrlsResponseBuilder {
            ListCrawledUrlsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for the `ListFindingTypeStats` method."]
    pub struct ListFindingTypeStatsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "findingTypeStats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of FindingTypeStats returned."]
        pub finding_type_stats:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FindingTypeStats>>>,
    }
    impl ListFindingTypeStatsResponse {
        pub fn builder() -> ListFindingTypeStatsResponseBuilder {
            ListFindingTypeStatsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for the `ListFindings` method."]
    pub struct ListFindingsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "findings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of Findings returned."]
        pub findings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Finding>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListFindingsResponse {
        pub fn builder() -> ListFindingsResponseBuilder {
            ListFindingsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for the `ListScanConfigs` method."]
    pub struct ListScanConfigsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scanConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of ScanConfigs returned."]
        pub scan_configs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ScanConfig>>>,
    }
    impl ListScanConfigsResponse {
        pub fn builder() -> ListScanConfigsResponseBuilder {
            ListScanConfigsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for the `ListScanRuns` method."]
    pub struct ListScanRunsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scanRuns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of ScanRuns returned."]
        pub scan_runs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ScanRun>>>,
    }
    impl ListScanRunsResponse {
        pub fn builder() -> ListScanRunsResponseBuilder {
            ListScanRunsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information reported for an outdated library."]
    pub struct OutdatedLibrary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "learnMoreUrls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URLs to learn more information about the vulnerabilities in the library."]
        pub learn_more_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "libraryName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the outdated library."]
        pub library_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version number."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl OutdatedLibrary {
        pub fn builder() -> OutdatedLibraryBuilder {
            OutdatedLibraryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A ScanConfig resource contains the configurations to launch a scan."]
    pub struct ScanConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authentication")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The authentication configuration. If specified, service will use the authentication configuration during scanning."]
        pub authentication: ::std::option::Option<::std::boxed::Box<Authentication>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blacklistPatterns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The excluded URL patterns as described in https://cloud.google.com/security-command-center/docs/how-to-use-web-security-scanner#excluding_urls"]
        pub blacklist_patterns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The user provided display name of the ScanConfig."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exportToSecurityCommandCenter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Controls export of scan configurations and results to Security Command Center."]
        pub export_to_security_command_center:
            ::std::option::Option<ScanConfigExportToSecurityCommandCenterEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "managedScan")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the scan config is managed by Web Security Scanner, output only."]
        pub managed_scan: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxQps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum QPS during scanning. A valid value ranges from 5 to 20 inclusively. If the field is unspecified or its value is set 0, server will default to 15. Other values outside of [5, 20] range will be rejected with INVALID_ARGUMENT error."]
        pub max_qps: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the ScanConfig. The name follows the format of 'projects/{projectId}/scanConfigs/{scanConfigId}'. The ScanConfig IDs are generated by the system."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "riskLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The risk level selected for the scan"]
        pub risk_level: ::std::option::Option<ScanConfigRiskLevelEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schedule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The schedule of the ScanConfig."]
        pub schedule: ::std::option::Option<::std::boxed::Box<Schedule>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startingUrls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The starting URLs from which the scanner finds site pages."]
        pub starting_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "staticIpScan")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the scan configuration has enabled static IP address scan feature. If enabled, the scanner will access applications from static IP addresses."]
        pub static_ip_scan: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userAgent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user agent used during scanning."]
        pub user_agent: ::std::option::Option<ScanConfigUserAgentEnum>,
    }
    impl ScanConfig {
        pub fn builder() -> ScanConfigBuilder {
            ScanConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Controls export of scan configurations and results to Security Command Center."]
    pub enum ScanConfigExportToSecurityCommandCenterEnum {
        #[serde(rename = "EXPORT_TO_SECURITY_COMMAND_CENTER_UNSPECIFIED")]
        #[doc = "Use default, which is ENABLED."]
        ExportToSecurityCommandCenterUnspecified,
        #[serde(rename = "ENABLED")]
        #[doc = "Export results of this scan to Security Command Center."]
        Enabled,
        #[serde(rename = "DISABLED")]
        #[doc = "Do not export results of this scan to Security Command Center."]
        Disabled,
    }
    impl ::std::default::Default for ScanConfigExportToSecurityCommandCenterEnum {
        fn default() -> Self {
            Self::ExportToSecurityCommandCenterUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The risk level selected for the scan"]
    pub enum ScanConfigRiskLevelEnum {
        #[serde(rename = "RISK_LEVEL_UNSPECIFIED")]
        #[doc = "Use default, which is NORMAL."]
        RiskLevelUnspecified,
        #[serde(rename = "NORMAL")]
        #[doc = "Normal scanning (Recommended)"]
        Normal,
        #[serde(rename = "LOW")]
        #[doc = "Lower impact scanning"]
        Low,
    }
    impl ::std::default::Default for ScanConfigRiskLevelEnum {
        fn default() -> Self {
            Self::RiskLevelUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The user agent used during scanning."]
    pub enum ScanConfigUserAgentEnum {
        #[serde(rename = "USER_AGENT_UNSPECIFIED")]
        #[doc = "The user agent is unknown. Service will default to CHROME_LINUX."]
        UserAgentUnspecified,
        #[serde(rename = "CHROME_LINUX")]
        #[doc = "Chrome on Linux. This is the service default if unspecified."]
        ChromeLinux,
        #[serde(rename = "CHROME_ANDROID")]
        #[doc = "Chrome on Android."]
        ChromeAndroid,
        #[serde(rename = "SAFARI_IPHONE")]
        #[doc = "Safari on IPhone."]
        SafariIphone,
    }
    impl ::std::default::Default for ScanConfigUserAgentEnum {
        fn default() -> Self {
            Self::UserAgentUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines a custom error message used by CreateScanConfig and UpdateScanConfig APIs when scan configuration validation fails. It is also reported as part of a ScanRunErrorTrace message if scan validation fails due to a scan configuration error."]
    pub struct ScanConfigError {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Indicates the reason code for a configuration failure."]
        pub code: ::std::option::Option<ScanConfigErrorCodeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Indicates the full name of the ScanConfig field that triggers this error, for example \"scan_config.max_qps\". This field is provided for troubleshooting purposes only and its actual value can change in the future."]
        pub field_name: ::std::option::Option<::std::string::String>,
    }
    impl ScanConfigError {
        pub fn builder() -> ScanConfigErrorBuilder {
            ScanConfigErrorBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Indicates the reason code for a configuration failure."]
    pub enum ScanConfigErrorCodeEnum {
        #[serde(rename = "CODE_UNSPECIFIED")]
        #[doc = "There is no error."]
        CodeUnspecified,
        #[serde(rename = "OK")]
        #[doc = "There is no error."]
        Ok,
        #[serde(rename = "INTERNAL_ERROR")]
        #[doc = "Indicates an internal server error. Please DO NOT USE THIS ERROR CODE unless the root cause is truly unknown."]
        InternalError,
        #[serde(rename = "APPENGINE_API_BACKEND_ERROR")]
        #[doc = "One of the seed URLs is an App Engine URL but we cannot validate the scan settings due to an App Engine API backend error."]
        AppengineApiBackendError,
        #[serde(rename = "APPENGINE_API_NOT_ACCESSIBLE")]
        #[doc = "One of the seed URLs is an App Engine URL but we cannot access the App Engine API to validate scan settings."]
        AppengineApiNotAccessible,
        #[serde(rename = "APPENGINE_DEFAULT_HOST_MISSING")]
        #[doc = "One of the seed URLs is an App Engine URL but the Default Host of the App Engine is not set."]
        AppengineDefaultHostMissing,
        #[serde(rename = "CANNOT_USE_GOOGLE_COM_ACCOUNT")]
        #[doc = "Google corporate accounts can not be used for scanning."]
        CannotUseGoogleComAccount,
        #[serde(rename = "CANNOT_USE_OWNER_ACCOUNT")]
        #[doc = "The account of the scan creator can not be used for scanning."]
        CannotUseOwnerAccount,
        #[serde(rename = "COMPUTE_API_BACKEND_ERROR")]
        #[doc = "This scan targets Compute Engine, but we cannot validate scan settings due to a Compute Engine API backend error."]
        ComputeApiBackendError,
        #[serde(rename = "COMPUTE_API_NOT_ACCESSIBLE")]
        #[doc = "This scan targets Compute Engine, but we cannot access the Compute Engine API to validate the scan settings."]
        ComputeApiNotAccessible,
        #[serde(rename = "CUSTOM_LOGIN_URL_DOES_NOT_BELONG_TO_CURRENT_PROJECT")]
        #[doc = "The Custom Login URL does not belong to the current project."]
        CustomLoginUrlDoesNotBelongToCurrentProject,
        #[serde(rename = "CUSTOM_LOGIN_URL_MALFORMED")]
        #[doc = "The Custom Login URL is malformed (can not be parsed)."]
        CustomLoginUrlMalformed,
        #[serde(rename = "CUSTOM_LOGIN_URL_MAPPED_TO_NON_ROUTABLE_ADDRESS")]
        #[doc = "The Custom Login URL is mapped to a non-routable IP address in DNS."]
        CustomLoginUrlMappedToNonRoutableAddress,
        #[serde(rename = "CUSTOM_LOGIN_URL_MAPPED_TO_UNRESERVED_ADDRESS")]
        #[doc = "The Custom Login URL is mapped to an IP address which is not reserved for the current project."]
        CustomLoginUrlMappedToUnreservedAddress,
        #[serde(rename = "CUSTOM_LOGIN_URL_HAS_NON_ROUTABLE_IP_ADDRESS")]
        #[doc = "The Custom Login URL has a non-routable IP address."]
        CustomLoginUrlHasNonRoutableIpAddress,
        #[serde(rename = "CUSTOM_LOGIN_URL_HAS_UNRESERVED_IP_ADDRESS")]
        #[doc = "The Custom Login URL has an IP address which is not reserved for the current project."]
        CustomLoginUrlHasUnreservedIpAddress,
        #[serde(rename = "DUPLICATE_SCAN_NAME")]
        #[doc = "Another scan with the same name (case-sensitive) already exists."]
        DuplicateScanName,
        #[serde(rename = "INVALID_FIELD_VALUE")]
        #[doc = "A field is set to an invalid value."]
        InvalidFieldValue,
        #[serde(rename = "FAILED_TO_AUTHENTICATE_TO_TARGET")]
        #[doc = "There was an error trying to authenticate to the scan target."]
        FailedToAuthenticateToTarget,
        #[serde(rename = "FINDING_TYPE_UNSPECIFIED")]
        #[doc = "Finding type value is not specified in the list findings request."]
        FindingTypeUnspecified,
        #[serde(rename = "FORBIDDEN_TO_SCAN_COMPUTE")]
        #[doc = "Scan targets Compute Engine, yet current project was not whitelisted for Google Compute Engine Scanning Alpha access."]
        ForbiddenToScanCompute,
        #[serde(rename = "FORBIDDEN_UPDATE_TO_MANAGED_SCAN")]
        #[doc = "User tries to update managed scan"]
        ForbiddenUpdateToManagedScan,
        #[serde(rename = "MALFORMED_FILTER")]
        #[doc = "The supplied filter is malformed. For example, it can not be parsed, does not have a filter type in expression, or the same filter type appears more than once."]
        MalformedFilter,
        #[serde(rename = "MALFORMED_RESOURCE_NAME")]
        #[doc = "The supplied resource name is malformed (can not be parsed)."]
        MalformedResourceName,
        #[serde(rename = "PROJECT_INACTIVE")]
        #[doc = "The current project is not in an active state."]
        ProjectInactive,
        #[serde(rename = "REQUIRED_FIELD")]
        #[doc = "A required field is not set."]
        RequiredField,
        #[serde(rename = "RESOURCE_NAME_INCONSISTENT")]
        #[doc = "Project id, scanconfig id, scanrun id, or finding id are not consistent with each other in resource name."]
        ResourceNameInconsistent,
        #[serde(rename = "SCAN_ALREADY_RUNNING")]
        #[doc = "The scan being requested to start is already running."]
        ScanAlreadyRunning,
        #[serde(rename = "SCAN_NOT_RUNNING")]
        #[doc = "The scan that was requested to be stopped is not running."]
        ScanNotRunning,
        #[serde(rename = "SEED_URL_DOES_NOT_BELONG_TO_CURRENT_PROJECT")]
        #[doc = "One of the seed URLs does not belong to the current project."]
        SeedUrlDoesNotBelongToCurrentProject,
        #[serde(rename = "SEED_URL_MALFORMED")]
        #[doc = "One of the seed URLs is malformed (can not be parsed)."]
        SeedUrlMalformed,
        #[serde(rename = "SEED_URL_MAPPED_TO_NON_ROUTABLE_ADDRESS")]
        #[doc = "One of the seed URLs is mapped to a non-routable IP address in DNS."]
        SeedUrlMappedToNonRoutableAddress,
        #[serde(rename = "SEED_URL_MAPPED_TO_UNRESERVED_ADDRESS")]
        #[doc = "One of the seed URLs is mapped to an IP address which is not reserved for the current project."]
        SeedUrlMappedToUnreservedAddress,
        #[serde(rename = "SEED_URL_HAS_NON_ROUTABLE_IP_ADDRESS")]
        #[doc = "One of the seed URLs has on-routable IP address."]
        SeedUrlHasNonRoutableIpAddress,
        #[serde(rename = "SEED_URL_HAS_UNRESERVED_IP_ADDRESS")]
        #[doc = "One of the seed URLs has an IP address that is not reserved for the current project."]
        SeedUrlHasUnreservedIpAddress,
        #[serde(rename = "SERVICE_ACCOUNT_NOT_CONFIGURED")]
        #[doc = "The Web Security Scanner service account is not configured under the project."]
        ServiceAccountNotConfigured,
        #[serde(rename = "TOO_MANY_SCANS")]
        #[doc = "A project has reached the maximum number of scans."]
        TooManyScans,
        #[serde(rename = "UNABLE_TO_RESOLVE_PROJECT_INFO")]
        #[doc = "Resolving the details of the current project fails."]
        UnableToResolveProjectInfo,
        #[serde(rename = "UNSUPPORTED_BLACKLIST_PATTERN_FORMAT")]
        #[doc = "One or more blacklist patterns were in the wrong format."]
        UnsupportedBlacklistPatternFormat,
        #[serde(rename = "UNSUPPORTED_FILTER")]
        #[doc = "The supplied filter is not supported."]
        UnsupportedFilter,
        #[serde(rename = "UNSUPPORTED_FINDING_TYPE")]
        #[doc = "The supplied finding type is not supported. For example, we do not provide findings of the given finding type."]
        UnsupportedFindingType,
        #[serde(rename = "UNSUPPORTED_URL_SCHEME")]
        #[doc = "The URL scheme of one or more of the supplied URLs is not supported."]
        UnsupportedUrlScheme,
    }
    impl ::std::default::Default for ScanConfigErrorCodeEnum {
        fn default() -> Self {
            Self::CodeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A ScanRun is a output-only resource representing an actual run of the scan. Next id: 12"]
    pub struct ScanRun {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time at which the ScanRun reached termination state - that the ScanRun is either finished or stopped by user."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorTrace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. If result_state is an ERROR, this field provides the primary reason for scan's termination and more details, if such are available."]
        pub error_trace: ::std::option::Option<::std::boxed::Box<ScanRunErrorTrace>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The execution state of the ScanRun."]
        pub execution_state: ::std::option::Option<ScanRunExecutionStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hasVulnerabilities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Whether the scan run has found any vulnerabilities."]
        pub has_vulnerabilities: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The resource name of the ScanRun. The name follows the format of 'projects/{projectId}/scanConfigs/{scanConfigId}/scanRuns/{scanRunId}'. The ScanRun IDs are generated by the system."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressPercent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The percentage of total completion ranging from 0 to 100. If the scan is in queue, the value is 0. If the scan is running, the value ranges from 0 to 100. If the scan is finished, the value is 100."]
        pub progress_percent: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resultState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The result state of the ScanRun. This field is only available after the execution state reaches \"FINISHED\"."]
        pub result_state: ::std::option::Option<ScanRunResultStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time at which the ScanRun started."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "urlsCrawledCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The number of URLs crawled during this ScanRun. If the scan is in progress, the value represents the number of URLs crawled up to now."]
        pub urls_crawled_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "urlsTestedCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The number of URLs tested during this ScanRun. If the scan is in progress, the value represents the number of URLs tested up to now. The number of URLs tested is usually larger than the number URLS crawled because typically a crawled URL is tested with multiple test payloads."]
        pub urls_tested_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warningTraces")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A list of warnings, if such are encountered during this scan run."]
        pub warning_traces:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ScanRunWarningTrace>>>,
    }
    impl ScanRun {
        pub fn builder() -> ScanRunBuilder {
            ScanRunBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The execution state of the ScanRun."]
    pub enum ScanRunExecutionStateEnum {
        #[serde(rename = "EXECUTION_STATE_UNSPECIFIED")]
        #[doc = "Represents an invalid state caused by internal server error. This value should never be returned."]
        ExecutionStateUnspecified,
        #[serde(rename = "QUEUED")]
        #[doc = "The scan is waiting in the queue."]
        Queued,
        #[serde(rename = "SCANNING")]
        #[doc = "The scan is in progress."]
        Scanning,
        #[serde(rename = "FINISHED")]
        #[doc = "The scan is either finished or stopped by user."]
        Finished,
    }
    impl ::std::default::Default for ScanRunExecutionStateEnum {
        fn default() -> Self {
            Self::ExecutionStateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The result state of the ScanRun. This field is only available after the execution state reaches \"FINISHED\"."]
    pub enum ScanRunResultStateEnum {
        #[serde(rename = "RESULT_STATE_UNSPECIFIED")]
        #[doc = "Default value. This value is returned when the ScanRun is not yet finished."]
        ResultStateUnspecified,
        #[serde(rename = "SUCCESS")]
        #[doc = "The scan finished without errors."]
        Success,
        #[serde(rename = "ERROR")]
        #[doc = "The scan finished with errors."]
        Error,
        #[serde(rename = "KILLED")]
        #[doc = "The scan was terminated by user."]
        Killed,
    }
    impl ::std::default::Default for ScanRunResultStateEnum {
        fn default() -> Self {
            Self::ResultStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. Defines an error trace message for a ScanRun."]
    pub struct ScanRunErrorTrace {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Indicates the error reason code."]
        pub code: ::std::option::Option<ScanRunErrorTraceCodeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mostCommonHttpErrorCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. If the scan encounters TOO_MANY_HTTP_ERRORS, this field indicates the most common HTTP error code, if such is available. For example, if this code is 404, the scan has encountered too many NOT_FOUND responses."]
        pub most_common_http_error_code: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scanConfigError")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. If the scan encounters SCAN_CONFIG_ISSUE error, this field has the error message encountered during scan configuration validation that is performed before each scan run."]
        pub scan_config_error: ::std::option::Option<::std::boxed::Box<ScanConfigError>>,
    }
    impl ScanRunErrorTrace {
        pub fn builder() -> ScanRunErrorTraceBuilder {
            ScanRunErrorTraceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Indicates the error reason code."]
    pub enum ScanRunErrorTraceCodeEnum {
        #[serde(rename = "CODE_UNSPECIFIED")]
        #[doc = "Default value is never used."]
        CodeUnspecified,
        #[serde(rename = "INTERNAL_ERROR")]
        #[doc = "Indicates that the scan run failed due to an internal server error."]
        InternalError,
        #[serde(rename = "SCAN_CONFIG_ISSUE")]
        #[doc = "Indicates a scan configuration error, usually due to outdated ScanConfig settings, such as starting_urls or the DNS configuration."]
        ScanConfigIssue,
        #[serde(rename = "AUTHENTICATION_CONFIG_ISSUE")]
        #[doc = "Indicates an authentication error, usually due to outdated ScanConfig authentication settings."]
        AuthenticationConfigIssue,
        #[serde(rename = "TIMED_OUT_WHILE_SCANNING")]
        #[doc = "Indicates a scan operation timeout, usually caused by a very large site."]
        TimedOutWhileScanning,
        #[serde(rename = "TOO_MANY_REDIRECTS")]
        #[doc = "Indicates that a scan encountered excessive redirects, either to authentication or some other page outside of the scan scope."]
        TooManyRedirects,
        #[serde(rename = "TOO_MANY_HTTP_ERRORS")]
        #[doc = "Indicates that a scan encountered numerous errors from the web site pages. When available, most_common_http_error_code field indicates the most common HTTP error code encountered during the scan."]
        TooManyHttpErrors,
    }
    impl ::std::default::Default for ScanRunErrorTraceCodeEnum {
        fn default() -> Self {
            Self::CodeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. Defines a warning trace message for ScanRun. Warning traces provide customers with useful information that helps make the scanning process more effective."]
    pub struct ScanRunWarningTrace {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Indicates the warning code."]
        pub code: ::std::option::Option<ScanRunWarningTraceCodeEnum>,
    }
    impl ScanRunWarningTrace {
        pub fn builder() -> ScanRunWarningTraceBuilder {
            ScanRunWarningTraceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Indicates the warning code."]
    pub enum ScanRunWarningTraceCodeEnum {
        #[serde(rename = "CODE_UNSPECIFIED")]
        #[doc = "Default value is never used."]
        CodeUnspecified,
        #[serde(rename = "INSUFFICIENT_CRAWL_RESULTS")]
        #[doc = "Indicates that a scan discovered an unexpectedly low number of URLs. This is sometimes caused by complex navigation features or by using a single URL for numerous pages."]
        InsufficientCrawlResults,
        #[serde(rename = "TOO_MANY_CRAWL_RESULTS")]
        #[doc = "Indicates that a scan discovered too many URLs to test, or excessive redundant URLs."]
        TooManyCrawlResults,
        #[serde(rename = "TOO_MANY_FUZZ_TASKS")]
        #[doc = "Indicates that too many tests have been generated for the scan. Customer should try reducing the number of starting URLs, increasing the QPS rate, or narrowing down the scope of the scan using the excluded patterns."]
        TooManyFuzzTasks,
        #[serde(rename = "BLOCKED_BY_IAP")]
        #[doc = "Indicates that a scan is blocked by IAP."]
        BlockedByIap,
        #[serde(rename = "NO_STARTING_URL_FOUND_FOR_MANAGED_SCAN")]
        #[doc = "Indicates that no seeds is found for a scan"]
        NoStartingUrlFoundForManagedScan,
    }
    impl ::std::default::Default for ScanRunWarningTraceCodeEnum {
        fn default() -> Self {
            Self::CodeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Scan schedule configuration."]
    pub struct Schedule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intervalDurationDays")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The duration of time between executions in days."]
        pub interval_duration_days: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scheduleTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A timestamp indicates when the next run will be scheduled. The value is refreshed by the server after each run. If unspecified, it will default to current server time, which means the scan will be scheduled to start immediately."]
        pub schedule_time: ::std::option::Option<::std::string::String>,
    }
    impl Schedule {
        pub fn builder() -> ScheduleBuilder {
            ScheduleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for the `StartScanRun` method."]
    pub struct StartScanRunRequest {}
    impl StartScanRunRequest {
        pub fn builder() -> StartScanRunRequestBuilder {
            StartScanRunRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for the `StopScanRun` method."]
    pub struct StopScanRunRequest {}
    impl StopScanRunRequest {
        pub fn builder() -> StopScanRunRequestBuilder {
            StopScanRunRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information regarding any resource causing the vulnerability such as JavaScript sources, image, audio files, etc."]
    pub struct ViolatingResource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The MIME type of this resource."]
        pub content_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL of this violating resource."]
        pub resource_url: ::std::option::Option<::std::string::String>,
    }
    impl ViolatingResource {
        pub fn builder() -> ViolatingResourceBuilder {
            ViolatingResourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about vulnerable or missing HTTP Headers."]
    pub struct VulnerableHeaders {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of vulnerable headers."]
        pub headers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Header>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "missingHeaders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of missing headers."]
        pub missing_headers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Header>>>,
    }
    impl VulnerableHeaders {
        pub fn builder() -> VulnerableHeadersBuilder {
            VulnerableHeadersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about vulnerable request parameters."]
    pub struct VulnerableParameters {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameterNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The vulnerable parameter names."]
        pub parameter_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl VulnerableParameters {
        pub fn builder() -> VulnerableParametersBuilder {
            VulnerableParametersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information reported for an XSS."]
    pub struct Xss {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attackVector")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The attack vector of the payload triggering this XSS."]
        pub attack_vector: ::std::option::Option<XssAttackVectorEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An error message generated by a javascript breakage."]
        pub error_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stackTraces")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Stack traces leading to the point where the XSS occurred."]
        pub stack_traces: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storedXssSeedingUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reproduction url for the seeding POST request of a Stored XSS."]
        pub stored_xss_seeding_url: ::std::option::Option<::std::string::String>,
    }
    impl Xss {
        pub fn builder() -> XssBuilder {
            XssBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The attack vector of the payload triggering this XSS."]
    pub enum XssAttackVectorEnum {
        #[serde(rename = "ATTACK_VECTOR_UNSPECIFIED")]
        #[doc = "Unknown attack vector."]
        AttackVectorUnspecified,
        #[serde(rename = "LOCAL_STORAGE")]
        #[doc = "The attack comes from fuzzing the browser's localStorage."]
        LocalStorage,
        #[serde(rename = "SESSION_STORAGE")]
        #[doc = "The attack comes from fuzzing the browser's sessionStorage."]
        SessionStorage,
        #[serde(rename = "WINDOW_NAME")]
        #[doc = "The attack comes from fuzzing the window's name property."]
        WindowName,
        #[serde(rename = "REFERRER")]
        #[doc = "The attack comes from fuzzing the referrer property."]
        Referrer,
        #[serde(rename = "FORM_INPUT")]
        #[doc = "The attack comes from fuzzing an input element."]
        FormInput,
        #[serde(rename = "COOKIE")]
        #[doc = "The attack comes from fuzzing the browser's cookies."]
        Cookie,
        #[serde(rename = "POST_MESSAGE")]
        #[doc = "The attack comes from hijacking the post messaging mechanism."]
        PostMessage,
        #[serde(rename = "GET_PARAMETERS")]
        #[doc = "The attack comes from fuzzing parameters in the url."]
        GetParameters,
        #[serde(rename = "URL_FRAGMENT")]
        #[doc = "The attack comes from fuzzing the fragment in the url."]
        UrlFragment,
        #[serde(rename = "HTML_COMMENT")]
        #[doc = "The attack comes from fuzzing the HTML comments."]
        HtmlComment,
        #[serde(rename = "POST_PARAMETERS")]
        #[doc = "The attack comes from fuzzing the POST parameters."]
        PostParameters,
        #[serde(rename = "PROTOCOL")]
        #[doc = "The attack comes from fuzzing the protocol."]
        Protocol,
        #[serde(rename = "STORED_XSS")]
        #[doc = "The attack comes from the server side and is stored."]
        StoredXss,
        #[serde(rename = "SAME_ORIGIN")]
        #[doc = "The attack is a Same-Origin Method Execution attack via a GET parameter."]
        SameOrigin,
        #[serde(rename = "USER_CONTROLLABLE_URL")]
        #[doc = "The attack payload is received from a third-party host via a URL that is user-controllable"]
        UserControllableUrl,
    }
    impl ::std::default::Default for XssAttackVectorEnum {
        fn default() -> Self {
            Self::AttackVectorUnspecified
        }
    }
}
