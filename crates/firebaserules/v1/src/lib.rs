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
            pub mod releases {
                pub mod methods {
                    pub mod get_executable {
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
                            #[serde(rename = "executableVersion")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The requested runtime executable version. Defaults to FIREBASE_RULES_EXECUTABLE_V1."]
                            pub executable_version:
                                ::std::option::Option<QueryParametersExecutableVersionEnum>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "The requested runtime executable version. Defaults to FIREBASE_RULES_EXECUTABLE_V1."]
                        pub enum QueryParametersExecutableVersionEnum {
                            #[serde(rename = "RELEASE_EXECUTABLE_VERSION_UNSPECIFIED")]
                            #[doc = "Executable format unspecified. Defaults to FIREBASE_RULES_EXECUTABLE_V1"]
                            ReleaseExecutableVersionUnspecified,
                            #[serde(rename = "FIREBASE_RULES_EXECUTABLE_V1")]
                            #[doc = "Firebase Rules syntax 'rules2' executable versions: Custom AST for use with Java clients."]
                            FirebaseRulesExecutableV1,
                            #[serde(rename = "FIREBASE_RULES_EXECUTABLE_V2")]
                            #[doc = "CEL-based executable for use with C++ clients."]
                            FirebaseRulesExecutableV2,
                        }
                        impl ::std::default::Default for QueryParametersExecutableVersionEnum {
                            fn default() -> Self {
                                Self::ReleaseExecutableVersionUnspecified
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
                            #[doc = "`Release` filter. The list method supports filters with restrictions on the `Release.name`, `Release.ruleset_name`, and `Release.test_suite_name`. Example 1: A filter of 'name=prod*' might return `Release`s with names within 'projects/foo' prefixed with 'prod': Name | Ruleset Name ------------------------------|------------- projects/foo/releases/prod | projects/foo/rulesets/uuid1234 projects/foo/releases/prod/v1 | projects/foo/rulesets/uuid1234 projects/foo/releases/prod/v2 | projects/foo/rulesets/uuid8888 Example 2: A filter of `name=prod* ruleset_name=uuid1234` would return only `Release` instances for 'projects/foo' with names prefixed with 'prod' referring to the same `Ruleset` name of 'uuid1234': Name | Ruleset Name ------------------------------|------------- projects/foo/releases/prod | projects/foo/rulesets/1234 projects/foo/releases/prod/v1 | projects/foo/rulesets/1234 In the examples, the filter parameters refer to the search filters are relative to the project. Fully qualified prefixed may also be used. e.g. `test_suite_name=projects/foo/testsuites/uuid1`"]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Page size to load. Maximum of 100. Defaults to 10. Note: `page_size` is just a hint and the service may choose to load fewer than `page_size` results due to the size of the output. To traverse all of the releases, the caller should iterate until the `page_token` on the response is empty."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Next page token for the next batch of `Release` instances."]
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
            pub mod rulesets {
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
                            #[doc = "`Ruleset` filter. The list method supports filters with restrictions on `Ruleset.name`. Filters on `Ruleset.create_time` should use the `date` function which parses strings that conform to the RFC 3339 date/time specifications. Example: `create_time > date(\"2017-01-01T00:00:00Z\") AND name=UUID-*`"]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Page size to load. Maximum of 100. Defaults to 10. Note: `page_size` is just a hint and the service may choose to load less than `page_size` due to the size of the output. To traverse all of the releases, caller should iterate until the `page_token` is empty."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Next page token for loading the next batch of `Ruleset` instances."]
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
    #[doc = "Arg matchers for the mock function."]
    pub struct Arg {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "anyValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Argument matches any value provided."]
        pub any_value: ::std::option::Option<::std::boxed::Box<Empty>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exactValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Argument exactly matches value provided."]
        pub exact_value: ::std::option::Option<::serde_json::Value>,
    }
    impl Arg {
        pub fn builder() -> ArgBuilder {
            ArgBuilder::default()
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
    #[doc = "Describes where in a file an expression is found and what it was evaluated to over the course of its use."]
    pub struct ExpressionReport {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "children")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Subexpressions"]
        pub children: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ExpressionReport>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourcePosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Position of expression in original rules source."]
        pub source_position: ::std::option::Option<::std::boxed::Box<SourcePosition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "values")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Values that this expression evaluated to when encountered."]
        pub values: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ValueCount>>>,
    }
    impl ExpressionReport {
        pub fn builder() -> ExpressionReportBuilder {
            ExpressionReportBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "`File` containing source content."]
    pub struct File {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Textual Content."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fingerprint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fingerprint (e.g. github sha) associated with the `File`."]
        pub fingerprint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "File name."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl File {
        pub fn builder() -> FileBuilder {
            FileBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a service-defined function call that was invoked during test execution."]
    pub struct FunctionCall {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "args")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The arguments that were provided to the function."]
        pub args: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "function")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the function invoked."]
        pub function: ::std::option::Option<::std::string::String>,
    }
    impl FunctionCall {
        pub fn builder() -> FunctionCallBuilder {
            FunctionCallBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Mock function definition. Mocks must refer to a function declared by the target service. The type of the function args and result will be inferred at test time. If either the arg or result values are not compatible with function type declaration, the request will be considered invalid. More than one `FunctionMock` may be provided for a given function name so long as the `Arg` matchers are distinct. There may be only one function for a given overload where all `Arg` values are `Arg.any_value`."]
    pub struct FunctionMock {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "args")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of `Arg` values to match. The order in which the arguments are provided is the order in which they must appear in the function invocation."]
        pub args: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Arg>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "function")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the function. The function name must match one provided by a service declaration."]
        pub function: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "result")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The mock result of the function call."]
        pub result: ::std::option::Option<::std::boxed::Box<Result>>,
    }
    impl FunctionMock {
        pub fn builder() -> FunctionMockBuilder {
            FunctionMockBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for FirebaseRulesService.GetReleaseExecutable"]
    pub struct GetReleaseExecutableResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Executable view of the `Ruleset` referenced by the `Release`."]
        pub executable: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executableVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Rules runtime version of the executable."]
        pub executable_version:
            ::std::option::Option<GetReleaseExecutableResponseExecutableVersionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "language")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "`Language` used to generate the executable bytes."]
        pub language: ::std::option::Option<GetReleaseExecutableResponseLanguageEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rulesetName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "`Ruleset` name associated with the `Release` executable."]
        pub ruleset_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "syncTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional, indicates the freshness of the result. The response is guaranteed to be the latest within an interval up to the sync_time (inclusive)."]
        pub sync_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp for the most recent `Release.update_time`."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GetReleaseExecutableResponse {
        pub fn builder() -> GetReleaseExecutableResponseBuilder {
            GetReleaseExecutableResponseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The Rules runtime version of the executable."]
    pub enum GetReleaseExecutableResponseExecutableVersionEnum {
        #[serde(rename = "RELEASE_EXECUTABLE_VERSION_UNSPECIFIED")]
        #[doc = "Executable format unspecified. Defaults to FIREBASE_RULES_EXECUTABLE_V1"]
        ReleaseExecutableVersionUnspecified,
        #[serde(rename = "FIREBASE_RULES_EXECUTABLE_V1")]
        #[doc = "Firebase Rules syntax 'rules2' executable versions: Custom AST for use with Java clients."]
        FirebaseRulesExecutableV1,
        #[serde(rename = "FIREBASE_RULES_EXECUTABLE_V2")]
        #[doc = "CEL-based executable for use with C++ clients."]
        FirebaseRulesExecutableV2,
    }
    impl ::std::default::Default for GetReleaseExecutableResponseExecutableVersionEnum {
        fn default() -> Self {
            Self::ReleaseExecutableVersionUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "`Language` used to generate the executable bytes."]
    pub enum GetReleaseExecutableResponseLanguageEnum {
        #[serde(rename = "LANGUAGE_UNSPECIFIED")]
        #[doc = "Language unspecified. Defaults to FIREBASE_RULES."]
        LanguageUnspecified,
        #[serde(rename = "FIREBASE_RULES")]
        #[doc = "Firebase Rules language."]
        FirebaseRules,
        #[serde(rename = "EVENT_FLOW_TRIGGERS")]
        #[doc = "Event Flow triggers."]
        EventFlowTriggers,
    }
    impl ::std::default::Default for GetReleaseExecutableResponseLanguageEnum {
        fn default() -> Self {
            Self::LanguageUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Issues include warnings, errors, and deprecation notices."]
    pub struct Issue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Short error description."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "severity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The severity of the issue."]
        pub severity: ::std::option::Option<IssueSeverityEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourcePosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Position of the issue in the `Source`."]
        pub source_position: ::std::option::Option<::std::boxed::Box<SourcePosition>>,
    }
    impl Issue {
        pub fn builder() -> IssueBuilder {
            IssueBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The severity of the issue."]
    pub enum IssueSeverityEnum {
        #[serde(rename = "SEVERITY_UNSPECIFIED")]
        #[doc = "An unspecified severity."]
        SeverityUnspecified,
        #[serde(rename = "DEPRECATION")]
        #[doc = "Deprecation issue for statements and method that may no longer be supported or maintained."]
        Deprecation,
        #[serde(rename = "WARNING")]
        #[doc = "Warnings such as: unused variables."]
        Warning,
        #[serde(rename = "ERROR")]
        #[doc = "Errors such as: unmatched curly braces or variable redefinition."]
        Error,
    }
    impl ::std::default::Default for IssueSeverityEnum {
        fn default() -> Self {
            Self::SeverityUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for FirebaseRulesService.ListReleases."]
    pub struct ListReleasesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The pagination token to retrieve the next page of results. If the value is empty, no further results remain."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "releases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of `Release` instances."]
        pub releases: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Release>>>,
    }
    impl ListReleasesResponse {
        pub fn builder() -> ListReleasesResponseBuilder {
            ListReleasesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for FirebaseRulesService.ListRulesets."]
    pub struct ListRulesetsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The pagination token to retrieve the next page of results. If the value is empty, no further results remain."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rulesets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of `Ruleset` instances."]
        pub rulesets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Ruleset>>>,
    }
    impl ListRulesetsResponse {
        pub fn builder() -> ListRulesetsResponseBuilder {
            ListRulesetsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for a Ruleset."]
    pub struct Metadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "services")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Services that this ruleset has declarations for (e.g., \"cloud.firestore\"). There may be 0+ of these."]
        pub services: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl Metadata {
        pub fn builder() -> MetadataBuilder {
            MetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "`Release` is a named reference to a `Ruleset`. Once a `Release` refers to a `Ruleset`, rules-enabled services will be able to enforce the `Ruleset`."]
    pub struct Release {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time the release was created. Output only."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name for the `Release`. `Release` names may be structured `app1/prod/v2` or flat `app1_prod_v2` which affords developers a great deal of flexibility in mapping the name to the style that best fits their existing development practices. For example, a name could refer to an environment, an app, a version, or some combination of three. In the table below, for the project name `projects/foo`, the following relative release paths show how flat and structured names might be chosen to match a desired development / deployment strategy. Use Case | Flat Name | Structured Name -------------|---------------------|---------------- Environments | releases/qa | releases/qa Apps | releases/app1_qa | releases/app1/qa Versions | releases/app1_v2_qa | releases/app1/v2/qa The delimiter between the release name path elements can be almost anything and it should work equally well with the release name list filter, but in many ways the structured paths provide a clearer picture of the relationship between `Release` instances. Format: `projects/{project_id}/releases/{release_id}`"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rulesetName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the `Ruleset` referred to by this `Release`. The `Ruleset` must exist the `Release` to be created."]
        pub ruleset_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time the release was updated. Output only."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl Release {
        pub fn builder() -> ReleaseBuilder {
            ReleaseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Possible result values from the function mock invocation."]
    pub struct Result {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "undefined")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result is undefined, meaning the result could not be computed."]
        pub undefined: ::std::option::Option<::std::boxed::Box<Empty>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result is an actual value. The type of the value must match that of the type declared by the service."]
        pub value: ::std::option::Option<::serde_json::Value>,
    }
    impl Result {
        pub fn builder() -> ResultBuilder {
            ResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "`Ruleset` is an immutable copy of `Source` with a globally unique identifier and a creation time."]
    pub struct Ruleset {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time the `Ruleset` was created. Output only."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metadata for this ruleset. Output only."]
        pub metadata: ::std::option::Option<::std::boxed::Box<Metadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the `Ruleset`. The ruleset_id is auto generated by the service. Format: `projects/{project_id}/rulesets/{ruleset_id}` Output only."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "`Source` for the `Ruleset`."]
        pub source: ::std::option::Option<::std::boxed::Box<Source>>,
    }
    impl Ruleset {
        pub fn builder() -> RulesetBuilder {
            RulesetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "`Source` is one or more `File` messages comprising a logical set of rules."]
    pub struct Source {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "files")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "`File` set constituting the `Source` bundle."]
        pub files: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<File>>>,
    }
    impl Source {
        pub fn builder() -> SourceBuilder {
            SourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Position in the `Source` content including its line, column number, and an index of the `File` in the `Source` message. Used for debug purposes."]
    pub struct SourcePosition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "column")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "First column on the source line associated with the source fragment."]
        pub column: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Start position relative to the beginning of the file."]
        pub current_offset: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "End position relative to the beginning of the file."]
        pub end_offset: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the `File`."]
        pub file_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "line")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Line number of the source fragment. 1-based."]
        pub line: ::std::option::Option<::std::primitive::i64>,
    }
    impl SourcePosition {
        pub fn builder() -> SourcePositionBuilder {
            SourcePositionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "`TestCase` messages provide the request context and an expectation as to whether the given context will be allowed or denied. Test cases may specify the `request`, `resource`, and `function_mocks` to mock a function call to a service-provided function. The `request` object represents context present at request-time. The `resource` is the value of the target resource as it appears in persistent storage before the request is executed."]
    pub struct TestCase {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expectation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Test expectation."]
        pub expectation: ::std::option::Option<TestCaseExpectationEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expressionReportLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies what should be included in the response."]
        pub expression_report_level: ::std::option::Option<TestCaseExpressionReportLevelEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "functionMocks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional function mocks for service-defined functions. If not set, any service defined function is expected to return an error, which may or may not influence the test outcome."]
        pub function_mocks: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FunctionMock>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pathEncoding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies whether paths (such as request.path) are encoded and how."]
        pub path_encoding: ::std::option::Option<TestCasePathEncodingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "request")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Request context. The exact format of the request context is service-dependent. See the appropriate service documentation for information about the supported fields and types on the request. Minimally, all services support the following fields and types: Request field | Type ---------------|----------------- auth.uid | `string` auth.token | `map` headers | `map` method | `string` params | `map` path | `string` time | `google.protobuf.Timestamp` If the request value is not well-formed for the service, the request will be rejected as an invalid argument."]
        pub request: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional resource value as it appears in persistent storage before the request is fulfilled. The resource type depends on the `request.path` value."]
        pub resource: ::std::option::Option<::serde_json::Value>,
    }
    impl TestCase {
        pub fn builder() -> TestCaseBuilder {
            TestCaseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Test expectation."]
    pub enum TestCaseExpectationEnum {
        #[serde(rename = "EXPECTATION_UNSPECIFIED")]
        #[doc = "Unspecified expectation."]
        ExpectationUnspecified,
        #[serde(rename = "ALLOW")]
        #[doc = "Expect an allowed result."]
        Allow,
        #[serde(rename = "DENY")]
        #[doc = "Expect a denied result."]
        Deny,
    }
    impl ::std::default::Default for TestCaseExpectationEnum {
        fn default() -> Self {
            Self::ExpectationUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies what should be included in the response."]
    pub enum TestCaseExpressionReportLevelEnum {
        #[serde(rename = "LEVEL_UNSPECIFIED")]
        #[doc = "No level has been specified. Defaults to \"NONE\" behavior."]
        LevelUnspecified,
        #[serde(rename = "NONE")]
        #[doc = "Do not include any additional information."]
        None,
        #[serde(rename = "FULL")]
        #[doc = "Include detailed reporting on expressions evaluated."]
        Full,
        #[serde(rename = "VISITED")]
        #[doc = "Only include the expressions that were visited during evaluation."]
        Visited,
    }
    impl ::std::default::Default for TestCaseExpressionReportLevelEnum {
        fn default() -> Self {
            Self::LevelUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies whether paths (such as request.path) are encoded and how."]
    pub enum TestCasePathEncodingEnum {
        #[serde(rename = "ENCODING_UNSPECIFIED")]
        #[doc = "No encoding has been specified. Defaults to \"URL_ENCODED\" behavior."]
        EncodingUnspecified,
        #[serde(rename = "URL_ENCODED")]
        #[doc = "Treats path segments as URL encoded but with non-encoded separators (\"/\"). This is the default behavior."]
        UrlEncoded,
        #[serde(rename = "PLAIN")]
        #[doc = "Treats total path as non-URL encoded e.g. raw."]
        Plain,
    }
    impl ::std::default::Default for TestCasePathEncodingEnum {
        fn default() -> Self {
            Self::EncodingUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Test result message containing the state of the test as well as a description and source position for test failures."]
    pub struct TestResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "debugMessages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Debug messages related to test execution issues encountered during evaluation. Debug messages may be related to too many or too few invocations of function mocks or to runtime errors that occur during evaluation. For example: ```Unable to read variable [name: \"resource\"]```"]
        pub debug_messages: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorPosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Position in the `Source` or `Ruleset` where the principle runtime error occurs. Evaluation of an expression may result in an error. Rules are deny by default, so a `DENY` expectation when an error is generated is valid. When there is a `DENY` with an error, the `SourcePosition` is returned. E.g. `error_position { line: 19 column: 37 }`"]
        pub error_position: ::std::option::Option<::std::boxed::Box<SourcePosition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expressionReports")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The mapping from expression in the ruleset AST to the values they were evaluated to. Partially-nested to mirror AST structure. Note that this field is actually tracking expressions and not permission statements in contrast to the \"visited_expressions\" field above. Literal expressions are omitted."]
        pub expression_reports:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ExpressionReport>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "functionCalls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of function calls made to service-defined methods. Function calls are included in the order in which they are encountered during evaluation, are provided for both mocked and unmocked functions, and included on the response regardless of the test `state`."]
        pub function_calls: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FunctionCall>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "State of the test."]
        pub state: ::std::option::Option<TestResultStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitedExpressions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of visited permission expressions for a given test. This returns the positions and evaluation results of all visited permission expressions which were relevant to the test case, e.g. ``` match /path { allow read if: } ``` For a detailed report of the intermediate evaluation states, see the `expression_reports` field"]
        pub visited_expressions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<VisitedExpression>>>,
    }
    impl TestResult {
        pub fn builder() -> TestResultBuilder {
            TestResultBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "State of the test."]
    pub enum TestResultStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Test state is not set."]
        StateUnspecified,
        #[serde(rename = "SUCCESS")]
        #[doc = "Test is a success."]
        Success,
        #[serde(rename = "FAILURE")]
        #[doc = "Test is a failure."]
        Failure,
    }
    impl ::std::default::Default for TestResultStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request for FirebaseRulesService.TestRuleset."]
    pub struct TestRulesetRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional `Source` to be checked for correctness. This field must not be set when the resource name refers to a `Ruleset`."]
        pub source: ::std::option::Option<::std::boxed::Box<Source>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testSuite")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Inline `TestSuite` to run."]
        pub test_suite: ::std::option::Option<::std::boxed::Box<TestSuite>>,
    }
    impl TestRulesetRequest {
        pub fn builder() -> TestRulesetRequestBuilder {
            TestRulesetRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for FirebaseRulesService.TestRuleset."]
    pub struct TestRulesetResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "issues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Syntactic and semantic `Source` issues of varying severity. Issues of `ERROR` severity will prevent tests from executing."]
        pub issues: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Issue>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of test results given the test cases in the `TestSuite`. The results will appear in the same order as the test cases appear in the `TestSuite`."]
        pub test_results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TestResult>>>,
    }
    impl TestRulesetResponse {
        pub fn builder() -> TestRulesetResponseBuilder {
            TestRulesetResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "`TestSuite` is a collection of `TestCase` instances that validate the logical correctness of a `Ruleset`. The `TestSuite` may be referenced in-line within a `TestRuleset` invocation or as part of a `Release` object as a pre-release check."]
    pub struct TestSuite {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testCases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Collection of test cases associated with the `TestSuite`."]
        pub test_cases: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TestCase>>>,
    }
    impl TestSuite {
        pub fn builder() -> TestSuiteBuilder {
            TestSuiteBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request for FirebaseRulesService.UpdateReleasePatch."]
    pub struct UpdateReleaseRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "release")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "`Release` to update."]
        pub release: ::std::option::Option<::std::boxed::Box<Release>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateMask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies which fields to update."]
        pub update_mask: ::std::option::Option<::std::string::String>,
    }
    impl UpdateReleaseRequest {
        pub fn builder() -> UpdateReleaseRequestBuilder {
            UpdateReleaseRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Tuple for how many times an Expression was evaluated to a particular ExpressionValue."]
    pub struct ValueCount {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of times that expression returned."]
        pub count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The return value of the expression"]
        pub value: ::std::option::Option<::serde_json::Value>,
    }
    impl ValueCount {
        pub fn builder() -> ValueCountBuilder {
            ValueCountBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Store the position and access outcome for an expression visited in rules."]
    pub struct VisitedExpression {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourcePosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Position in the `Source` or `Ruleset` where an expression was visited."]
        pub source_position: ::std::option::Option<::std::boxed::Box<SourcePosition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The evaluated value for the visited expression, e.g. true/false"]
        pub value: ::std::option::Option<::serde_json::Value>,
    }
    impl VisitedExpression {
        pub fn builder() -> VisitedExpressionBuilder {
            VisitedExpressionBuilder::default()
        }
    }
}
