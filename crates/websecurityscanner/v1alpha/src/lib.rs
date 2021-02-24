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
                                            #[doc = "Required. The filter expression. The expression must be in the format: . Supported field: 'finding_type'. Supported operator: '='."]
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
        #[doc = "The body of the request that triggered the vulnerability."]
        pub body: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The description of the vulnerability."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "finalUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL where the browser lands when the vulnerability is detected."]
        pub final_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "findingType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the Finding."]
        pub finding_type: ::std::option::Option<FindingFindingTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frameUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the vulnerability was originated from nested IFrame, the immediate parent IFrame is reported."]
        pub frame_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fuzzedUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL produced by the server-side fuzzer and used in the request that triggered the vulnerability."]
        pub fuzzed_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "httpMethod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The http method of the request that triggered the vulnerability, in uppercase."]
        pub http_method: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the Finding. The name follows the format of 'projects/{projectId}/scanConfigs/{scanConfigId}/scanruns/{scanRunId}/findings/{findingId}'. The finding IDs are generated by the system."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outdatedLibrary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An addon containing information about outdated libraries."]
        pub outdated_library: ::std::option::Option<::std::boxed::Box<OutdatedLibrary>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reproductionUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL containing human-readable payload that user can leverage to reproduce the vulnerability."]
        pub reproduction_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trackingId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The tracking ID uniquely identifies a vulnerability instance across multiple ScanRuns."]
        pub tracking_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "violatingResource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An addon containing detailed information regarding any resource causing the vulnerability such as JavaScript sources, image, audio files, etc."]
        pub violating_resource: ::std::option::Option<::std::boxed::Box<ViolatingResource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vulnerableHeaders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An addon containing information about vulnerable or missing HTTP headers."]
        pub vulnerable_headers: ::std::option::Option<::std::boxed::Box<VulnerableHeaders>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vulnerableParameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An addon containing information about request parameters which were found to be vulnerable."]
        pub vulnerable_parameters: ::std::option::Option<::std::boxed::Box<VulnerableParameters>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "xss")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An addon containing information reported for an XSS, if any."]
        pub xss: ::std::option::Option<::std::boxed::Box<Xss>>,
    }
    impl Finding {
        pub fn builder() -> FindingBuilder {
            FindingBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the Finding."]
    pub enum FindingFindingTypeEnum {
        #[serde(rename = "FINDING_TYPE_UNSPECIFIED")]
        #[doc = "The invalid finding type."]
        FindingTypeUnspecified,
        #[serde(rename = "MIXED_CONTENT")]
        #[doc = "A page that was served over HTTPS also resources over HTTP. A man-in-the-middle attacker could tamper with the HTTP resource and gain full access to the website that loads the resource or to monitor the actions taken by the user."]
        MixedContent,
        #[serde(rename = "OUTDATED_LIBRARY")]
        #[doc = "The version of an included library is known to contain a security issue. The scanner checks the version of library in use against a known list of vulnerable libraries. False positives are possible if the version detection fails or if the library has been manually patched."]
        OutdatedLibrary,
        #[serde(rename = "ROSETTA_FLASH")]
        #[doc = "This type of vulnerability occurs when the value of a request parameter is reflected at the beginning of the response, for example, in requests using JSONP. Under certain circumstances, an attacker may be able to supply an alphanumeric-only Flash file in the vulnerable parameter causing the browser to execute the Flash file as if it originated on the vulnerable server."]
        RosettaFlash,
        #[serde(rename = "XSS_CALLBACK")]
        #[doc = "A cross-site scripting (XSS) bug is found via JavaScript callback. For detailed explanations on XSS, see https://www.google.com/about/appsecurity/learning/xss/."]
        XssCallback,
        #[serde(rename = "XSS_ERROR")]
        #[doc = "A potential cross-site scripting (XSS) bug due to JavaScript breakage. In some circumstances, the application under test might modify the test string before it is parsed by the browser. When the browser attempts to runs this modified test string, it will likely break and throw a JavaScript execution error, thus an injection issue is occurring. However, it may not be exploitable. Manual verification is needed to see if the test string modifications can be evaded and confirm that the issue is in fact an XSS vulnerability. For detailed explanations on XSS, see https://www.google.com/about/appsecurity/learning/xss/."]
        XssError,
        #[serde(rename = "CLEAR_TEXT_PASSWORD")]
        #[doc = "An application appears to be transmitting a password field in clear text. An attacker can eavesdrop network traffic and sniff the password field."]
        ClearTextPassword,
        #[serde(rename = "INVALID_CONTENT_TYPE")]
        #[doc = "An application returns sensitive content with an invalid content type, or without an 'X-Content-Type-Options: nosniff' header."]
        InvalidContentType,
        #[serde(rename = "XSS_ANGULAR_CALLBACK")]
        #[doc = "A cross-site scripting (XSS) vulnerability in AngularJS module that occurs when a user-provided string is interpolated by Angular."]
        XssAngularCallback,
        #[serde(rename = "INVALID_HEADER")]
        #[doc = "A malformed or invalid valued header."]
        InvalidHeader,
        #[serde(rename = "MISSPELLED_SECURITY_HEADER_NAME")]
        #[doc = "Misspelled security header name."]
        MisspelledSecurityHeaderName,
        #[serde(rename = "MISMATCHING_SECURITY_HEADER_VALUES")]
        #[doc = "Mismatching values in a duplicate security header."]
        MismatchingSecurityHeaderValues,
    }
    impl ::std::default::Default for FindingFindingTypeEnum {
        fn default() -> Self {
            Self::FindingTypeUnspecified
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
        #[doc = "The count of findings belonging to this finding type."]
        pub finding_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "findingType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The finding type associated with the stats."]
        pub finding_type: ::std::option::Option<FindingTypeStatsFindingTypeEnum>,
    }
    impl FindingTypeStats {
        pub fn builder() -> FindingTypeStatsBuilder {
            FindingTypeStatsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The finding type associated with the stats."]
    pub enum FindingTypeStatsFindingTypeEnum {
        #[serde(rename = "FINDING_TYPE_UNSPECIFIED")]
        #[doc = "The invalid finding type."]
        FindingTypeUnspecified,
        #[serde(rename = "MIXED_CONTENT")]
        #[doc = "A page that was served over HTTPS also resources over HTTP. A man-in-the-middle attacker could tamper with the HTTP resource and gain full access to the website that loads the resource or to monitor the actions taken by the user."]
        MixedContent,
        #[serde(rename = "OUTDATED_LIBRARY")]
        #[doc = "The version of an included library is known to contain a security issue. The scanner checks the version of library in use against a known list of vulnerable libraries. False positives are possible if the version detection fails or if the library has been manually patched."]
        OutdatedLibrary,
        #[serde(rename = "ROSETTA_FLASH")]
        #[doc = "This type of vulnerability occurs when the value of a request parameter is reflected at the beginning of the response, for example, in requests using JSONP. Under certain circumstances, an attacker may be able to supply an alphanumeric-only Flash file in the vulnerable parameter causing the browser to execute the Flash file as if it originated on the vulnerable server."]
        RosettaFlash,
        #[serde(rename = "XSS_CALLBACK")]
        #[doc = "A cross-site scripting (XSS) bug is found via JavaScript callback. For detailed explanations on XSS, see https://www.google.com/about/appsecurity/learning/xss/."]
        XssCallback,
        #[serde(rename = "XSS_ERROR")]
        #[doc = "A potential cross-site scripting (XSS) bug due to JavaScript breakage. In some circumstances, the application under test might modify the test string before it is parsed by the browser. When the browser attempts to runs this modified test string, it will likely break and throw a JavaScript execution error, thus an injection issue is occurring. However, it may not be exploitable. Manual verification is needed to see if the test string modifications can be evaded and confirm that the issue is in fact an XSS vulnerability. For detailed explanations on XSS, see https://www.google.com/about/appsecurity/learning/xss/."]
        XssError,
        #[serde(rename = "CLEAR_TEXT_PASSWORD")]
        #[doc = "An application appears to be transmitting a password field in clear text. An attacker can eavesdrop network traffic and sniff the password field."]
        ClearTextPassword,
        #[serde(rename = "INVALID_CONTENT_TYPE")]
        #[doc = "An application returns sensitive content with an invalid content type, or without an 'X-Content-Type-Options: nosniff' header."]
        InvalidContentType,
        #[serde(rename = "XSS_ANGULAR_CALLBACK")]
        #[doc = "A cross-site scripting (XSS) vulnerability in AngularJS module that occurs when a user-provided string is interpolated by Angular."]
        XssAngularCallback,
        #[serde(rename = "INVALID_HEADER")]
        #[doc = "A malformed or invalid valued header."]
        InvalidHeader,
        #[serde(rename = "MISSPELLED_SECURITY_HEADER_NAME")]
        #[doc = "Misspelled security header name."]
        MisspelledSecurityHeaderName,
        #[serde(rename = "MISMATCHING_SECURITY_HEADER_VALUES")]
        #[doc = "Mismatching values in a duplicate security header."]
        MismatchingSecurityHeaderValues,
    }
    impl ::std::default::Default for FindingTypeStatsFindingTypeEnum {
        fn default() -> Self {
            Self::FindingTypeUnspecified
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
    #[doc = "A ScanConfig resource contains the configurations to launch a scan. next id: 12"]
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
        #[serde(rename = "latestRun")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Latest ScanRun if available."]
        pub latest_run: ::std::option::Option<::std::boxed::Box<ScanRun>>,
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
        #[serde(rename = "targetPlatforms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set of Google Cloud platforms targeted by the scan. If empty, APP_ENGINE will be used as a default."]
        pub target_platforms: ::std::option::Option<::std::vec::Vec<ScanConfigTargetPlatformsEnum>>,
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
    pub enum ScanConfigTargetPlatformsEnum {
        #[serde(rename = "TARGET_PLATFORM_UNSPECIFIED")]
        #[doc = "The target platform is unknown. Requests with this enum value will be rejected with INVALID_ARGUMENT error."]
        TargetPlatformUnspecified,
        #[serde(rename = "APP_ENGINE")]
        #[doc = "Google App Engine service."]
        AppEngine,
        #[serde(rename = "COMPUTE")]
        #[doc = "Google Compute Engine service."]
        Compute,
    }
    impl ::std::default::Default for ScanConfigTargetPlatformsEnum {
        fn default() -> Self {
            Self::TargetPlatformUnspecified
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
    #[doc = "A ScanRun is a output-only resource representing an actual run of the scan."]
    pub struct ScanRun {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the ScanRun reached termination state - that the ScanRun is either finished or stopped by user."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The execution state of the ScanRun."]
        pub execution_state: ::std::option::Option<ScanRunExecutionStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hasVulnerabilities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the scan run has found any vulnerabilities."]
        pub has_vulnerabilities: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the ScanRun. The name follows the format of 'projects/{projectId}/scanConfigs/{scanConfigId}/scanRuns/{scanRunId}'. The ScanRun IDs are generated by the system."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressPercent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The percentage of total completion ranging from 0 to 100. If the scan is in queue, the value is 0. If the scan is running, the value ranges from 0 to 100. If the scan is finished, the value is 100."]
        pub progress_percent: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resultState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result state of the ScanRun. This field is only available after the execution state reaches \"FINISHED\"."]
        pub result_state: ::std::option::Option<ScanRunResultStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the ScanRun started."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "urlsCrawledCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of URLs crawled during this ScanRun. If the scan is in progress, the value represents the number of URLs crawled up to now."]
        pub urls_crawled_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "urlsTestedCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of URLs tested during this ScanRun. If the scan is in progress, the value represents the number of URLs tested up to now. The number of URLs tested is usually larger than the number URLS crawled because typically a crawled URL is tested with multiple test payloads."]
        pub urls_tested_count: ::std::option::Option<::std::string::String>,
    }
    impl ScanRun {
        pub fn builder() -> ScanRunBuilder {
            ScanRunBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The execution state of the ScanRun."]
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
    #[doc = "The result state of the ScanRun. This field is only available after the execution state reaches \"FINISHED\"."]
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
        #[serde(rename = "errorMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An error message generated by a javascript breakage."]
        pub error_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stackTraces")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Stack traces leading to the point where the XSS occurred."]
        pub stack_traces: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl Xss {
        pub fn builder() -> XssBuilder {
            XssBuilder::default()
        }
    }
}
