#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the CancelExecution method."]
pub struct CancelExecutionRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Error describes why the execution was abnormally terminated."]
pub struct Error {
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Human readable stack trace string."]
    pub context: ::std::option::Option<::std::string::String>,
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Error message and data returned represented as a JSON string."]
    pub payload: ::std::option::Option<::std::string::String>,
    #[serde(rename = "stackTrace")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Stack trace with detailed information of where error was generated."]
    pub stack_trace: ::std::option::Option<::std::boxed::Box<StackTrace>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A running instance of a [Workflow](/workflows/docs/reference/rest/v1beta/projects.locations.workflows)."]
pub struct Execution {
    #[serde(rename = "argument")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Input parameters of the execution represented as a JSON string. The size limit is 32KB. *Note*: If you are using the REST API directly to run your workflow, you must escape any JSON string value of `argument`. Example: `'{\"argument\":\"{\\\"firstName\\\":\\\"FIRST\\\",\\\"lastName\\\":\\\"LAST\\\"}\"}'`"]
    pub argument: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Marks the end of execution, successful or not."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The error which caused the execution to finish prematurely. The value is only present if the execution's state is `FAILED` or `CANCELLED`."]
    pub error: ::std::option::Option<::std::boxed::Box<Error>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the execution. Format: projects/{project}/locations/{location}/workflows/{workflow}/executions/{execution}"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Output of the execution represented as a JSON string. The value can only be present if the execution's state is `SUCCEEDED`."]
    pub result: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Marks the beginning of execution."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Current state of the execution."]
    pub state: ::std::option::Option<ExecutionStateEnum>,
    #[serde(rename = "workflowRevisionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Revision of the workflow this execution is using."]
    pub workflow_revision_id: ::std::option::Option<::std::string::String>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the ListExecutions method."]
pub struct ListExecutionsResponse {
    #[serde(rename = "executions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The executions which match the request."]
    pub executions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Execution>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Position contains source position information about the stack trace element such as line number, column number and length of the code block in bytes."]
pub struct Position {
    #[serde(rename = "column")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source code column position (of the line) the current instruction was generated from."]
    pub column: ::std::option::Option<::std::string::String>,
    #[serde(rename = "length")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The length in bytes of text in this character group, e.g. digits of a number, string length, or AST (abstract syntax tree) node."]
    pub length: ::std::option::Option<::std::string::String>,
    #[serde(rename = "line")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source code line number the current instruction was generated from."]
    pub line: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A collection of stack elements (frames) where an error occurred."]
pub struct StackTrace {
    #[serde(rename = "elements")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An array of Stack elements."]
    pub elements: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StackTraceElement>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single stack element (frame) where an error occurred. This field currently only exists in v1Beta. We will need to roll this change out to V1 after the feature is thoroughly tested. TODO(b/178540475)"]
pub struct StackTraceElement {
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source position information of the stacktrace element."]
    pub position: ::std::option::Option<::std::boxed::Box<Position>>,
    #[serde(rename = "routine")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The routine where the error occurred."]
    pub routine: ::std::option::Option<::std::string::String>,
    #[serde(rename = "step")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The step the error occurred at."]
    pub step: ::std::option::Option<::std::string::String>,
}
