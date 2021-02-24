#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Arg matchers for the mock function."]
pub struct Arg {
    #[serde(rename = "anyValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Argument matches any value provided."]
    pub any_value: ::std::option::Option<::std::boxed::Box<Empty>>,
    #[serde(rename = "exactValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Argument exactly matches value provided."]
    pub exact_value: ::std::option::Option<::serde_json::Value>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes where in a file an expression is found and what it was evaluated to over the course of its use."]
pub struct ExpressionReport {
    #[serde(rename = "children")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subexpressions"]
    pub children: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ExpressionReport>>>,
    #[serde(rename = "sourcePosition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Position of expression in original rules source."]
    pub source_position: ::std::option::Option<::std::boxed::Box<SourcePosition>>,
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Values that this expression evaluated to when encountered."]
    pub values: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ValueCount>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`File` containing source content."]
pub struct File {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Textual Content."]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Fingerprint (e.g. github sha) associated with the `File`."]
    pub fingerprint: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "File name."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a service-defined function call that was invoked during test execution."]
pub struct FunctionCall {
    #[serde(rename = "args")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The arguments that were provided to the function."]
    pub args: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
    #[serde(rename = "function")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the function invoked."]
    pub function: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Mock function definition. Mocks must refer to a function declared by the target service. The type of the function args and result will be inferred at test time. If either the arg or result values are not compatible with function type declaration, the request will be considered invalid. More than one `FunctionMock` may be provided for a given function name so long as the `Arg` matchers are distinct. There may be only one function for a given overload where all `Arg` values are `Arg.any_value`."]
pub struct FunctionMock {
    #[serde(rename = "args")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of `Arg` values to match. The order in which the arguments are provided is the order in which they must appear in the function invocation."]
    pub args: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Arg>>>,
    #[serde(rename = "function")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the function. The function name must match one provided by a service declaration."]
    pub function: ::std::option::Option<::std::string::String>,
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The mock result of the function call."]
    pub result: ::std::option::Option<::std::boxed::Box<Result>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for FirebaseRulesService.GetReleaseExecutable"]
pub struct GetReleaseExecutableResponse {
    #[serde(rename = "executable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Executable view of the `Ruleset` referenced by the `Release`."]
    pub executable: ::std::option::Option<::std::string::String>,
    #[serde(rename = "executableVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Rules runtime version of the executable."]
    pub executable_version:
        ::std::option::Option<GetReleaseExecutableResponseExecutableVersionEnum>,
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "`Language` used to generate the executable bytes."]
    pub language: ::std::option::Option<GetReleaseExecutableResponseLanguageEnum>,
    #[serde(rename = "rulesetName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "`Ruleset` name associated with the `Release` executable."]
    pub ruleset_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "syncTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional, indicates the freshness of the result. The response is guaranteed to be the latest within an interval up to the sync_time (inclusive)."]
    pub sync_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp for the most recent `Release.update_time`."]
    pub update_time: ::std::option::Option<::std::string::String>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Issues include warnings, errors, and deprecation notices."]
pub struct Issue {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Short error description."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The severity of the issue."]
    pub severity: ::std::option::Option<IssueSeverityEnum>,
    #[serde(rename = "sourcePosition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Position of the issue in the `Source`."]
    pub source_position: ::std::option::Option<::std::boxed::Box<SourcePosition>>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for FirebaseRulesService.ListReleases."]
pub struct ListReleasesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pagination token to retrieve the next page of results. If the value is empty, no further results remain."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "releases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of `Release` instances."]
    pub releases: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Release>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for FirebaseRulesService.ListRulesets."]
pub struct ListRulesetsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pagination token to retrieve the next page of results. If the value is empty, no further results remain."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rulesets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of `Ruleset` instances."]
    pub rulesets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Ruleset>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for a Ruleset."]
pub struct Metadata {
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Services that this ruleset has declarations for (e.g., \"cloud.firestore\"). There may be 0+ of these."]
    pub services: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`Release` is a named reference to a `Ruleset`. Once a `Release` refers to a `Ruleset`, rules-enabled services will be able to enforce the `Ruleset`."]
pub struct Release {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time the release was created. Output only."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name for the `Release`. `Release` names may be structured `app1/prod/v2` or flat `app1_prod_v2` which affords developers a great deal of flexibility in mapping the name to the style that best fits their existing development practices. For example, a name could refer to an environment, an app, a version, or some combination of three. In the table below, for the project name `projects/foo`, the following relative release paths show how flat and structured names might be chosen to match a desired development / deployment strategy. Use Case | Flat Name | Structured Name -------------|---------------------|---------------- Environments | releases/qa | releases/qa Apps | releases/app1_qa | releases/app1/qa Versions | releases/app1_v2_qa | releases/app1/v2/qa The delimiter between the release name path elements can be almost anything and it should work equally well with the release name list filter, but in many ways the structured paths provide a clearer picture of the relationship between `Release` instances. Format: `projects/{project_id}/releases/{release_id}`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rulesetName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the `Ruleset` referred to by this `Release`. The `Ruleset` must exist the `Release` to be created."]
    pub ruleset_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time the release was updated. Output only."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Possible result values from the function mock invocation."]
pub struct Result {
    #[serde(rename = "undefined")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result is undefined, meaning the result could not be computed."]
    pub undefined: ::std::option::Option<::std::boxed::Box<Empty>>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result is an actual value. The type of the value must match that of the type declared by the service."]
    pub value: ::std::option::Option<::serde_json::Value>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`Ruleset` is an immutable copy of `Source` with a globally unique identifier and a creation time."]
pub struct Ruleset {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time the `Ruleset` was created. Output only."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The metadata for this ruleset. Output only."]
    pub metadata: ::std::option::Option<::std::boxed::Box<Metadata>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the `Ruleset`. The ruleset_id is auto generated by the service. Format: `projects/{project_id}/rulesets/{ruleset_id}` Output only."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "`Source` for the `Ruleset`."]
    pub source: ::std::option::Option<::std::boxed::Box<Source>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`Source` is one or more `File` messages comprising a logical set of rules."]
pub struct Source {
    #[serde(rename = "files")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "`File` set constituting the `Source` bundle."]
    pub files: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<File>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Position in the `Source` content including its line, column number, and an index of the `File` in the `Source` message. Used for debug purposes."]
pub struct SourcePosition {
    #[serde(rename = "column")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "First column on the source line associated with the source fragment."]
    pub column: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "currentOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Start position relative to the beginning of the file."]
    pub current_offset: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "endOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "End position relative to the beginning of the file."]
    pub end_offset: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "fileName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the `File`."]
    pub file_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "line")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Line number of the source fragment. 1-based."]
    pub line: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`TestCase` messages provide the request context and an expectation as to whether the given context will be allowed or denied. Test cases may specify the `request`, `resource`, and `function_mocks` to mock a function call to a service-provided function. The `request` object represents context present at request-time. The `resource` is the value of the target resource as it appears in persistent storage before the request is executed."]
pub struct TestCase {
    #[serde(rename = "expectation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Test expectation."]
    pub expectation: ::std::option::Option<TestCaseExpectationEnum>,
    #[serde(rename = "expressionReportLevel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies what should be included in the response."]
    pub expression_report_level: ::std::option::Option<TestCaseExpressionReportLevelEnum>,
    #[serde(rename = "functionMocks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional function mocks for service-defined functions. If not set, any service defined function is expected to return an error, which may or may not influence the test outcome."]
    pub function_mocks: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FunctionMock>>>,
    #[serde(rename = "pathEncoding")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies whether paths (such as request.path) are encoded and how."]
    pub path_encoding: ::std::option::Option<TestCasePathEncodingEnum>,
    #[serde(rename = "request")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Request context. The exact format of the request context is service-dependent. See the appropriate service documentation for information about the supported fields and types on the request. Minimally, all services support the following fields and types: Request field | Type ---------------|----------------- auth.uid | `string` auth.token | `map` headers | `map` method | `string` params | `map` path | `string` time | `google.protobuf.Timestamp` If the request value is not well-formed for the service, the request will be rejected as an invalid argument."]
    pub request: ::std::option::Option<::serde_json::Value>,
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional resource value as it appears in persistent storage before the request is fulfilled. The resource type depends on the `request.path` value."]
    pub resource: ::std::option::Option<::serde_json::Value>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Test result message containing the state of the test as well as a description and source position for test failures."]
pub struct TestResult {
    #[serde(rename = "debugMessages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Debug messages related to test execution issues encountered during evaluation. Debug messages may be related to too many or too few invocations of function mocks or to runtime errors that occur during evaluation. For example: ```Unable to read variable [name: \"resource\"]```"]
    pub debug_messages: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "errorPosition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Position in the `Source` or `Ruleset` where the principle runtime error occurs. Evaluation of an expression may result in an error. Rules are deny by default, so a `DENY` expectation when an error is generated is valid. When there is a `DENY` with an error, the `SourcePosition` is returned. E.g. `error_position { line: 19 column: 37 }`"]
    pub error_position: ::std::option::Option<::std::boxed::Box<SourcePosition>>,
    #[serde(rename = "expressionReports")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The mapping from expression in the ruleset AST to the values they were evaluated to. Partially-nested to mirror AST structure. Note that this field is actually tracking expressions and not permission statements in contrast to the \"visited_expressions\" field above. Literal expressions are omitted."]
    pub expression_reports:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ExpressionReport>>>,
    #[serde(rename = "functionCalls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of function calls made to service-defined methods. Function calls are included in the order in which they are encountered during evaluation, are provided for both mocked and unmocked functions, and included on the response regardless of the test `state`."]
    pub function_calls: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FunctionCall>>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "State of the test."]
    pub state: ::std::option::Option<TestResultStateEnum>,
    #[serde(rename = "visitedExpressions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of visited permission expressions for a given test. This returns the positions and evaluation results of all visited permission expressions which were relevant to the test case, e.g. ``` match /path { allow read if: } ``` For a detailed report of the intermediate evaluation states, see the `expression_reports` field"]
    pub visited_expressions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<VisitedExpression>>>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for FirebaseRulesService.TestRuleset."]
pub struct TestRulesetRequest {
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional `Source` to be checked for correctness. This field must not be set when the resource name refers to a `Ruleset`."]
    pub source: ::std::option::Option<::std::boxed::Box<Source>>,
    #[serde(rename = "testSuite")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inline `TestSuite` to run."]
    pub test_suite: ::std::option::Option<::std::boxed::Box<TestSuite>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for FirebaseRulesService.TestRuleset."]
pub struct TestRulesetResponse {
    #[serde(rename = "issues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Syntactic and semantic `Source` issues of varying severity. Issues of `ERROR` severity will prevent tests from executing."]
    pub issues: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Issue>>>,
    #[serde(rename = "testResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of test results given the test cases in the `TestSuite`. The results will appear in the same order as the test cases appear in the `TestSuite`."]
    pub test_results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TestResult>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`TestSuite` is a collection of `TestCase` instances that validate the logical correctness of a `Ruleset`. The `TestSuite` may be referenced in-line within a `TestRuleset` invocation or as part of a `Release` object as a pre-release check."]
pub struct TestSuite {
    #[serde(rename = "testCases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Collection of test cases associated with the `TestSuite`."]
    pub test_cases: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TestCase>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for FirebaseRulesService.UpdateReleasePatch."]
pub struct UpdateReleaseRequest {
    #[serde(rename = "release")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "`Release` to update."]
    pub release: ::std::option::Option<::std::boxed::Box<Release>>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies which fields to update."]
    pub update_mask: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Tuple for how many times an Expression was evaluated to a particular ExpressionValue."]
pub struct ValueCount {
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount of times that expression returned."]
    pub count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The return value of the expression"]
    pub value: ::std::option::Option<::serde_json::Value>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Store the position and access outcome for an expression visited in rules."]
pub struct VisitedExpression {
    #[serde(rename = "sourcePosition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Position in the `Source` or `Ruleset` where an expression was visited."]
    pub source_position: ::std::option::Option<::std::boxed::Box<SourcePosition>>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The evaluated value for the visited expression, e.g. true/false"]
    pub value: ::std::option::Option<::serde_json::Value>,
}
