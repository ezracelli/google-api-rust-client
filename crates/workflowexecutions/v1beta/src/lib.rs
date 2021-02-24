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
            pub mod locations {
                pub mod resources {
                    pub mod workflows {
                        pub mod resources {
                            pub mod executions {
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
                                            #[doc = "Optional. A view defining which fields should be filled in the returned execution. The API will default to the FULL view."]
                                            pub view:
                                                ::std::option::Option<QueryParametersViewEnum>,
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
                                        #[doc = "Optional. A view defining which fields should be filled in the returned execution. The API will default to the FULL view."]
                                        pub enum QueryParametersViewEnum {
                                            #[serde(rename = "EXECUTION_VIEW_UNSPECIFIED")]
                                            #[doc = "The default / unset value."]
                                            ExecutionViewUnspecified,
                                            #[serde(rename = "BASIC")]
                                            #[doc = "Includes only basic metadata about the execution. Following fields are returned: name, start_time, end_time, state and workflow_revision_id."]
                                            Basic,
                                            #[serde(rename = "FULL")]
                                            #[doc = "Includes all data."]
                                            Full,
                                        }
                                        impl ::std::default::Default for QueryParametersViewEnum {
                                            fn default() -> Self {
                                                Self::ExecutionViewUnspecified
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
                                            #[doc = "Maximum number of executions to return per call. Max supported value depends on the selected Execution view: it's 10000 for BASIC and 100 for FULL. The default value used if the field is not specified is 100, regardless of the selected view. Values greater than the max value will be coerced down to it."]
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
                                            #[doc = "A page token, received from a previous `ListExecutions` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListExecutions` must match the call that provided the page token."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "view")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Optional. A view defining which fields should be filled in the returned executions. The API will default to the BASIC view."]
                                            pub view:
                                                ::std::option::Option<QueryParametersViewEnum>,
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
                                        #[doc = "Optional. A view defining which fields should be filled in the returned executions. The API will default to the BASIC view."]
                                        pub enum QueryParametersViewEnum {
                                            #[serde(rename = "EXECUTION_VIEW_UNSPECIFIED")]
                                            #[doc = "The default / unset value."]
                                            ExecutionViewUnspecified,
                                            #[serde(rename = "BASIC")]
                                            #[doc = "Includes only basic metadata about the execution. Following fields are returned: name, start_time, end_time, state and workflow_revision_id."]
                                            Basic,
                                            #[serde(rename = "FULL")]
                                            #[doc = "Includes all data."]
                                            Full,
                                        }
                                        impl ::std::default::Default for QueryParametersViewEnum {
                                            fn default() -> Self {
                                                Self::ExecutionViewUnspecified
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
    #[doc = "Request for the CancelExecution method."]
    pub struct CancelExecutionRequest {}
    impl CancelExecutionRequest {
        pub fn builder() -> CancelExecutionRequestBuilder {
            CancelExecutionRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Error describes why the execution was abnormally terminated."]
    pub struct Error {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "context")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human readable stack trace string."]
        pub context: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Error message and data returned represented as a JSON string."]
        pub payload: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stackTrace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Stack trace with detailed information of where error was generated."]
        pub stack_trace: ::std::option::Option<::std::boxed::Box<StackTrace>>,
    }
    impl Error {
        pub fn builder() -> ErrorBuilder {
            ErrorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A running instance of a [Workflow](/workflows/docs/reference/rest/v1beta/projects.locations.workflows)."]
    pub struct Execution {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "argument")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Input parameters of the execution represented as a JSON string. The size limit is 32KB. *Note*: If you are using the REST API directly to run your workflow, you must escape any JSON string value of `argument`. Example: `'{\"argument\":\"{\\\"firstName\\\":\\\"FIRST\\\",\\\"lastName\\\":\\\"LAST\\\"}\"}'`"]
        pub argument: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Marks the end of execution, successful or not."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The error which caused the execution to finish prematurely. The value is only present if the execution's state is `FAILED` or `CANCELLED`."]
        pub error: ::std::option::Option<::std::boxed::Box<Error>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The resource name of the execution. Format: projects/{project}/locations/{location}/workflows/{workflow}/executions/{execution}"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "result")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Output of the execution represented as a JSON string. The value can only be present if the execution's state is `SUCCEEDED`."]
        pub result: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Marks the beginning of execution."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Current state of the execution."]
        pub state: ::std::option::Option<ExecutionStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workflowRevisionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Revision of the workflow this execution is using."]
        pub workflow_revision_id: ::std::option::Option<::std::string::String>,
    }
    impl Execution {
        pub fn builder() -> ExecutionBuilder {
            ExecutionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Current state of the execution."]
    pub enum ExecutionStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Invalid state."]
        StateUnspecified,
        #[serde(rename = "ACTIVE")]
        #[doc = "The execution is in progress."]
        Active,
        #[serde(rename = "SUCCEEDED")]
        #[doc = "The execution finished successfully."]
        Succeeded,
        #[serde(rename = "FAILED")]
        #[doc = "The execution failed with an error."]
        Failed,
        #[serde(rename = "CANCELLED")]
        #[doc = "The execution was stopped intentionally."]
        Cancelled,
    }
    impl ::std::default::Default for ExecutionStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for the ListExecutions method."]
    pub struct ListExecutionsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The executions which match the request."]
        pub executions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Execution>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
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
    #[doc = "Position contains source position information about the stack trace element such as line number, column number and length of the code block in bytes."]
    pub struct Position {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "column")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source code column position (of the line) the current instruction was generated from."]
        pub column: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "length")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The length in bytes of text in this character group, e.g. digits of a number, string length, or AST (abstract syntax tree) node."]
        pub length: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "line")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source code line number the current instruction was generated from."]
        pub line: ::std::option::Option<::std::string::String>,
    }
    impl Position {
        pub fn builder() -> PositionBuilder {
            PositionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A collection of stack elements (frames) where an error occurred."]
    pub struct StackTrace {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "elements")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An array of Stack elements."]
        pub elements: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StackTraceElement>>>,
    }
    impl StackTrace {
        pub fn builder() -> StackTraceBuilder {
            StackTraceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single stack element (frame) where an error occurred. This field currently only exists in v1Beta. We will need to roll this change out to V1 after the feature is thoroughly tested. TODO(b/178540475)"]
    pub struct StackTraceElement {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "position")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source position information of the stacktrace element."]
        pub position: ::std::option::Option<::std::boxed::Box<Position>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "routine")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The routine where the error occurred."]
        pub routine: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "step")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The step the error occurred at."]
        pub step: ::std::option::Option<::std::string::String>,
    }
    impl StackTraceElement {
        pub fn builder() -> StackTraceElementBuilder {
            StackTraceElementBuilder::default()
        }
    }
}
