#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An alias to a repo revision."]
pub struct AliasContext {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The alias kind."]
    pub kind: ::std::option::Option<AliasContextKindEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The alias name."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The alias kind."]
pub enum AliasContextKindEnum {
    #[serde(rename = "ANY")]
    #[doc = "Do not use."]
    Any,
    #[serde(rename = "FIXED")]
    #[doc = "Git tag"]
    Fixed,
    #[serde(rename = "MOVABLE")]
    #[doc = "Git branch"]
    Movable,
    #[serde(rename = "OTHER")]
    #[doc = "OTHER is used to specify non-standard aliases, those not of the kinds above. For example, if a Git repo has a ref named \"refs/foo/bar\", it is considered to be of kind OTHER."]
    Other,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "------------------------------------------------------------------------------ ## Breakpoint (the resource) Represents the breakpoint specification, status and results."]
pub struct Breakpoint {
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Action that the agent should perform when the code at the breakpoint location is hit."]
    pub action: ::std::option::Option<BreakpointActionEnum>,
    #[serde(rename = "canaryExpireTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The deadline for the breakpoint to stay in CANARY_ACTIVE state. The value is meaningless when the breakpoint is not in CANARY_ACTIVE state."]
    pub canary_expire_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Condition that triggers the breakpoint. The condition is a compound boolean expression composed using expressions in a programming language at the source location."]
    pub condition: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time this breakpoint was created by the server in seconds resolution."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "evaluatedExpressions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Values of evaluated expressions at breakpoint time. The evaluated expressions appear in exactly the same order they are listed in the `expressions` field. The `name` field holds the original expression text, the `value` or `members` field holds the result of the evaluated expression. If the expression cannot be evaluated, the `status` inside the `Variable` will indicate an error and contain the error text."]
    pub evaluated_expressions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Variable>>>,
    #[serde(rename = "expressions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of read-only expressions to evaluate at the breakpoint location. The expressions are composed using expressions in the programming language at the source location. If the breakpoint action is `LOG`, the evaluated expressions are included in log statements."]
    pub expressions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "finalTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time this breakpoint was finalized as seen by the server in seconds resolution."]
    pub final_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Breakpoint identifier, unique in the scope of the debuggee."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isFinalState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When true, indicates that this is a final result and the breakpoint state will not change from here on."]
    pub is_final_state: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A set of custom breakpoint properties, populated by the agent, to be displayed to the user."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Breakpoint source location."]
    pub location: ::std::option::Option<::std::boxed::Box<SourceLocation>>,
    #[serde(rename = "logLevel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the severity of the log. Only relevant when action is `LOG`."]
    pub log_level: ::std::option::Option<BreakpointLogLevelEnum>,
    #[serde(rename = "logMessageFormat")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only relevant when action is `LOG`. Defines the message to log when the breakpoint hits. The message may include parameter placeholders `$0`, `$1`, etc. These placeholders are replaced with the evaluated value of the appropriate expression. Expressions not referenced in `log_message_format` are not logged. Example: `Message received, id = $0, count = $1` with `expressions` = `[ message.id, message.count ]`."]
    pub log_message_format: ::std::option::Option<::std::string::String>,
    #[serde(rename = "stackFrames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The stack at breakpoint time, where stack_frames[0] represents the most recently entered function."]
    pub stack_frames: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StackFrame>>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current state of the breakpoint."]
    pub state: ::std::option::Option<BreakpointStateEnum>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Breakpoint status. The status includes an error flag and a human readable message. This field is usually unset. The message can be either informational or an error message. Regardless, clients should always display the text message back to the user. Error status indicates complete failure of the breakpoint. Example (non-final state): `Still loading symbols...` Examples (final state): * `Invalid line number` referring to location * `Field f not found in class C` referring to condition"]
    pub status: ::std::option::Option<::std::boxed::Box<StatusMessage>>,
    #[serde(rename = "userEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "E-mail address of the user that created this breakpoint"]
    pub user_email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "variableTable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `variable_table` exists to aid with computation, memory and network traffic optimization. It enables storing a variable once and reference it from multiple variables, including variables stored in the `variable_table` itself. For example, the same `this` object, which may appear at many levels of the stack, can have all of its data stored once in this table. The stack frame variables then would hold only a reference to it. The variable `var_table_index` field is an index into this repeated field. The stored objects are nameless and get their name from the referencing variable. The effective variable is a merge of the referencing variable and the referenced variable."]
    pub variable_table: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Variable>>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Action that the agent should perform when the code at the breakpoint location is hit."]
pub enum BreakpointActionEnum {
    #[serde(rename = "CAPTURE")]
    #[doc = "Capture stack frame and variables and update the breakpoint. The data is only captured once. After that the breakpoint is set in a final state."]
    Capture,
    #[serde(rename = "LOG")]
    #[doc = "Log each breakpoint hit. The breakpoint remains active until deleted or expired."]
    Log,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Indicates the severity of the log. Only relevant when action is `LOG`."]
pub enum BreakpointLogLevelEnum {
    #[serde(rename = "INFO")]
    #[doc = "Information log message."]
    Info,
    #[serde(rename = "WARNING")]
    #[doc = "Warning log message."]
    Warning,
    #[serde(rename = "ERROR")]
    #[doc = "Error log message."]
    Error,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The current state of the breakpoint."]
pub enum BreakpointStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Breakpoint state UNSPECIFIED."]
    StateUnspecified,
    #[serde(rename = "STATE_CANARY_PENDING_AGENTS")]
    #[doc = "Enabling canary but no agents are available."]
    StateCanaryPendingAgents,
    #[serde(rename = "STATE_CANARY_ACTIVE")]
    #[doc = "Enabling canary and successfully assigning canary agents."]
    StateCanaryActive,
    #[serde(rename = "STATE_ROLLING_TO_ALL")]
    #[doc = "Breakpoint rolling out to all agents."]
    StateRollingToAll,
    #[serde(rename = "STATE_IS_FINAL")]
    #[doc = "Breakpoint is hit/complete/failed."]
    StateIsFinal,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A CloudRepoSourceContext denotes a particular revision in a cloud repo (a repo hosted by the Google Cloud Platform)."]
pub struct CloudRepoSourceContext {
    #[serde(rename = "aliasContext")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An alias, which may be a branch or tag."]
    pub alias_context: ::std::option::Option<::std::boxed::Box<AliasContext>>,
    #[serde(rename = "aliasName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of an alias (branch, tag, etc.)."]
    pub alias_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "repoId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the repo."]
    pub repo_id: ::std::option::Option<::std::boxed::Box<RepoId>>,
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A revision ID."]
    pub revision_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A CloudWorkspaceId is a unique identifier for a cloud workspace. A cloud workspace is a place associated with a repo where modified files can be stored before they are committed."]
pub struct CloudWorkspaceId {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique name of the workspace within the repo. This is the name chosen by the client in the Source API's CreateWorkspace method."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "repoId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the repo containing the workspace."]
    pub repo_id: ::std::option::Option<::std::boxed::Box<RepoId>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A CloudWorkspaceSourceContext denotes a workspace at a particular snapshot."]
pub struct CloudWorkspaceSourceContext {
    #[serde(rename = "snapshotId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the snapshot. An empty snapshot_id refers to the most recent snapshot."]
    pub snapshot_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "workspaceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the workspace."]
    pub workspace_id: ::std::option::Option<::std::boxed::Box<CloudWorkspaceId>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the debugged application. The application may include one or more replicated processes executing the same code. Each of these processes is attached with a debugger agent, carrying out the debugging commands. Agents attached to the same debuggee identify themselves as such by using exactly the same Debuggee message value when registering."]
pub struct Debuggee {
    #[serde(rename = "agentVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Version ID of the agent. Schema: `domain/language-platform/vmajor.minor` (for example `google.com/java-gcp/v1.1`)."]
    pub agent_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "canaryMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Used when setting breakpoint canary for this debuggee."]
    pub canary_mode: ::std::option::Option<DebuggeeCanaryModeEnum>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Human readable description of the debuggee. Including a human-readable project name, environment name and version information is recommended."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "extSourceContexts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "References to the locations and revisions of the source code used in the deployed application."]
    pub ext_source_contexts:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ExtendedSourceContext>>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier for the debuggee generated by the controller service."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isDisabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set to `true`, indicates that the agent should disable itself and detach from the debuggee."]
    pub is_disabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "isInactive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set to `true`, indicates that Controller service does not detect any activity from the debuggee agents and the application is possibly stopped."]
    pub is_inactive: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A set of custom debuggee properties, populated by the agent, to be displayed to the user."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "project")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Project the debuggee is associated with. Use project number or id when registering a Google Cloud Platform project."]
    pub project: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sourceContexts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "References to the locations and revisions of the source code used in the deployed application."]
    pub source_contexts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SourceContext>>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Human readable message to be displayed to the user about this debuggee. Absence of this field indicates no status. The message can be either informational or an error status."]
    pub status: ::std::option::Option<::std::boxed::Box<StatusMessage>>,
    #[serde(rename = "uniquifier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Uniquifier to further distinguish the application. It is possible that different applications might have identical values in the debuggee message, thus, incorrectly identified as a single application by the Controller service. This field adds salt to further distinguish the application. Agents should consider seeding this field with value that identifies the code, binary, configuration and environment."]
    pub uniquifier: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Used when setting breakpoint canary for this debuggee."]
pub enum DebuggeeCanaryModeEnum {
    #[serde(rename = "CANARY_MODE_UNSPECIFIED")]
    #[doc = "CANARY_MODE_UNSPECIFIED is equivalent to CANARY_MODE_ALWAYS_DISABLED so that if the debuggee is not configured to use the canary feature, the feature will be disabled."]
    CanaryModeUnspecified,
    #[serde(rename = "CANARY_MODE_ALWAYS_ENABLED")]
    #[doc = "Always enable breakpoint canary regardless of the value of breakpoint's canary option."]
    CanaryModeAlwaysEnabled,
    #[serde(rename = "CANARY_MODE_ALWAYS_DISABLED")]
    #[doc = "Always disable breakpoint canary regardless of the value of breakpoint's canary option."]
    CanaryModeAlwaysDisabled,
    #[serde(rename = "CANARY_MODE_DEFAULT_ENABLED")]
    #[doc = "Depends on the breakpoint's canary option. Enable canary by default if the breakpoint's canary option is not specified."]
    CanaryModeDefaultEnabled,
    #[serde(rename = "CANARY_MODE_DEFAULT_DISABLED")]
    #[doc = "Depends on the breakpoint's canary option. Disable canary by default if the breakpoint's canary option is not specified."]
    CanaryModeDefaultDisabled,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An ExtendedSourceContext is a SourceContext combined with additional details describing the context."]
pub struct ExtendedSourceContext {
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Any source context."]
    pub context: ::std::option::Option<::std::boxed::Box<SourceContext>>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Labels with user defined metadata."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a message with parameters."]
pub struct FormatMessage {
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Format template for the message. The `format` uses placeholders `$0`, `$1`, etc. to reference parameters. `$$` can be used to denote the `$` character. Examples: * `Failed to load '$0' which helps debug $1 the first time it is loaded. Again, $0 is very important.` * `Please pay $$10 to use $0 instead of $1.`"]
    pub format: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional parameters to be embedded into the message."]
    pub parameters: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A SourceContext referring to a Gerrit project."]
pub struct GerritSourceContext {
    #[serde(rename = "aliasContext")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An alias, which may be a branch or tag."]
    pub alias_context: ::std::option::Option<::std::boxed::Box<AliasContext>>,
    #[serde(rename = "aliasName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of an alias (branch, tag, etc.)."]
    pub alias_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gerritProject")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full project name within the host. Projects may be nested, so \"project/subproject\" is a valid project name. The \"repo name\" is hostURI/project."]
    pub gerrit_project: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hostUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI of a running Gerrit instance."]
    pub host_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A revision (commit) ID."]
    pub revision_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for getting breakpoint information."]
pub struct GetBreakpointResponse {
    #[serde(rename = "breakpoint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Complete breakpoint state. The fields `id` and `location` are guaranteed to be set."]
    pub breakpoint: ::std::option::Option<::std::boxed::Box<Breakpoint>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A GitSourceContext denotes a particular revision in a third party Git repository (e.g. GitHub)."]
pub struct GitSourceContext {
    #[serde(rename = "revisionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Git commit hash. required."]
    pub revision_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Git repository URL."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for listing active breakpoints."]
pub struct ListActiveBreakpointsResponse {
    #[serde(rename = "breakpoints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of all active breakpoints. The fields `id` and `location` are guaranteed to be set on each breakpoint."]
    pub breakpoints: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Breakpoint>>>,
    #[serde(rename = "nextWaitToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token that can be used in the next method call to block until the list of breakpoints changes."]
    pub next_wait_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "waitExpired")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set to `true`, indicates that there is no change to the list of active breakpoints and the server-selected timeout has expired. The `breakpoints` field would be empty and should be ignored."]
    pub wait_expired: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for listing breakpoints."]
pub struct ListBreakpointsResponse {
    #[serde(rename = "breakpoints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of breakpoints matching the request. The fields `id` and `location` are guaranteed to be set on each breakpoint. The fields: `stack_frames`, `evaluated_expressions` and `variable_table` are cleared on each breakpoint regardless of its status."]
    pub breakpoints: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Breakpoint>>>,
    #[serde(rename = "nextWaitToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A wait token that can be used in the next call to `list` (REST) or `ListBreakpoints` (RPC) to block until the list of breakpoints has changes."]
    pub next_wait_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for listing debuggees."]
pub struct ListDebuggeesResponse {
    #[serde(rename = "debuggees")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of debuggees accessible to the calling user. The fields `debuggee.id` and `description` are guaranteed to be set. The `description` field is a human readable field provided by agents and can be displayed to users."]
    pub debuggees: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Debuggee>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Selects a repo using a Google Cloud Platform project ID (e.g. winged-cargo-31) and a repo name within that project."]
pub struct ProjectRepoId {
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the project."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "repoName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the repo. Leave empty for the default repo."]
    pub repo_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to register a debuggee."]
pub struct RegisterDebuggeeRequest {
    #[serde(rename = "debuggee")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Debuggee information to register. The fields `project`, `uniquifier`, `description` and `agent_version` of the debuggee must be set."]
    pub debuggee: ::std::option::Option<::std::boxed::Box<Debuggee>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for registering a debuggee."]
pub struct RegisterDebuggeeResponse {
    #[serde(rename = "agentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A unique ID generated for the agent. Each RegisterDebuggee request will generate a new agent ID."]
    pub agent_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "debuggee")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Debuggee resource. The field `id` is guaranteed to be set (in addition to the echoed fields). If the field `is_disabled` is set to `true`, the agent should disable itself by removing all breakpoints and detaching from the application. It should however continue to poll `RegisterDebuggee` until reenabled."]
    pub debuggee: ::std::option::Option<::std::boxed::Box<Debuggee>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A unique identifier for a cloud repo."]
pub struct RepoId {
    #[serde(rename = "projectRepoId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A combination of a project ID and a repo name."]
    pub project_repo_id: ::std::option::Option<::std::boxed::Box<ProjectRepoId>>,
    #[serde(rename = "uid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A server-assigned, globally unique identifier."]
    pub uid: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for setting a breakpoint."]
pub struct SetBreakpointResponse {
    #[serde(rename = "breakpoint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Breakpoint resource. The field `id` is guaranteed to be set (in addition to the echoed fields)."]
    pub breakpoint: ::std::option::Option<::std::boxed::Box<Breakpoint>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A SourceContext is a reference to a tree of files. A SourceContext together with a path point to a unique revision of a single file or directory."]
pub struct SourceContext {
    #[serde(rename = "cloudRepo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A SourceContext referring to a revision in a cloud repo."]
    pub cloud_repo: ::std::option::Option<::std::boxed::Box<CloudRepoSourceContext>>,
    #[serde(rename = "cloudWorkspace")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A SourceContext referring to a snapshot in a cloud workspace."]
    pub cloud_workspace: ::std::option::Option<::std::boxed::Box<CloudWorkspaceSourceContext>>,
    #[serde(rename = "gerrit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A SourceContext referring to a Gerrit project."]
    pub gerrit: ::std::option::Option<::std::boxed::Box<GerritSourceContext>>,
    #[serde(rename = "git")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A SourceContext referring to any third party Git repo (e.g. GitHub)."]
    pub git: ::std::option::Option<::std::boxed::Box<GitSourceContext>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a location in the source code."]
pub struct SourceLocation {
    #[serde(rename = "column")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Column within a line. The first column in a line as the value `1`. Agents that do not support setting breakpoints on specific columns ignore this field."]
    pub column: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "line")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Line inside the file. The first line in the file has the value `1`."]
    pub line: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Path to the source file within the source context of the target binary."]
    pub path: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a stack frame context."]
pub struct StackFrame {
    #[serde(rename = "arguments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set of arguments passed to this function. Note that this might not be populated for all stack frames."]
    pub arguments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Variable>>>,
    #[serde(rename = "function")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Demangled function name at the call site."]
    pub function: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locals")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set of local variables at the stack frame location. Note that this might not be populated for all stack frames."]
    pub locals: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Variable>>>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Source location of the call site."]
    pub location: ::std::option::Option<::std::boxed::Box<SourceLocation>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a contextual status message. The message can indicate an error or informational status, and refer to specific parts of the containing object. For example, the `Breakpoint.status` field can indicate an error referring to the `BREAKPOINT_SOURCE_LOCATION` with the message `Location not found`."]
pub struct StatusMessage {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status message text."]
    pub description: ::std::option::Option<::std::boxed::Box<FormatMessage>>,
    #[serde(rename = "isError")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Distinguishes errors from informational messages."]
    pub is_error: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "refersTo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reference to which the message applies."]
    pub refers_to: ::std::option::Option<StatusMessageRefersToEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Reference to which the message applies."]
pub enum StatusMessageRefersToEnum {
    #[serde(rename = "UNSPECIFIED")]
    #[doc = "Status doesn't refer to any particular input."]
    Unspecified,
    #[serde(rename = "BREAKPOINT_SOURCE_LOCATION")]
    #[doc = "Status applies to the breakpoint and is related to its location."]
    BreakpointSourceLocation,
    #[serde(rename = "BREAKPOINT_CONDITION")]
    #[doc = "Status applies to the breakpoint and is related to its condition."]
    BreakpointCondition,
    #[serde(rename = "BREAKPOINT_EXPRESSION")]
    #[doc = "Status applies to the breakpoint and is related to its expressions."]
    BreakpointExpression,
    #[serde(rename = "BREAKPOINT_AGE")]
    #[doc = "Status applies to the breakpoint and is related to its age."]
    BreakpointAge,
    #[serde(rename = "BREAKPOINT_CANARY_FAILED")]
    #[doc = "Status applies to the breakpoint when the breakpoint failed to exit the canary state."]
    BreakpointCanaryFailed,
    #[serde(rename = "VARIABLE_NAME")]
    #[doc = "Status applies to the entire variable."]
    VariableName,
    #[serde(rename = "VARIABLE_VALUE")]
    #[doc = "Status applies to variable value (variable name is valid)."]
    VariableValue,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to update an active breakpoint."]
pub struct UpdateActiveBreakpointRequest {
    #[serde(rename = "breakpoint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Updated breakpoint information. The field `id` must be set. The agent must echo all Breakpoint specification fields in the update."]
    pub breakpoint: ::std::option::Option<::std::boxed::Box<Breakpoint>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for updating an active breakpoint. The message is defined to allow future extensions."]
pub struct UpdateActiveBreakpointResponse {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a variable or an argument possibly of a compound object type. Note how the following variables are represented: 1) A simple variable: int x = 5 { name: \"x\", value: \"5\", type: \"int\" } // Captured variable 2) A compound object: struct T { int m1; int m2; }; T x = { 3, 7 }; { // Captured variable name: \"x\", type: \"T\", members { name: \"m1\", value: \"3\", type: \"int\" }, members { name: \"m2\", value: \"7\", type: \"int\" } } 3) A pointer where the pointee was captured: T x = { 3, 7 }; T* p = &x; { // Captured variable name: \"p\", type: \"T*\", value: \"0x00500500\", members { name: \"m1\", value: \"3\", type: \"int\" }, members { name: \"m2\", value: \"7\", type: \"int\" } } 4) A pointer where the pointee was not captured: T* p = new T; { // Captured variable name: \"p\", type: \"T*\", value: \"0x00400400\" status { is_error: true, description { format: \"unavailable\" } } } The status should describe the reason for the missing value, such as ``, ``, ``. Note that a null pointer should not have members. 5) An unnamed value: int* p = new int(7); { // Captured variable name: \"p\", value: \"0x00500500\", type: \"int*\", members { value: \"7\", type: \"int\" } } 6) An unnamed pointer where the pointee was not captured: int* p = new int(7); int** pp = &p; { // Captured variable name: \"pp\", value: \"0x00500500\", type: \"int**\", members { value: \"0x00400400\", type: \"int*\" status { is_error: true, description: { format: \"unavailable\" } } } } } To optimize computation, memory and network traffic, variables that repeat in the output multiple times can be stored once in a shared variable table and be referenced using the `var_table_index` field. The variables stored in the shared table are nameless and are essentially a partition of the complete variable. To reconstruct the complete variable, merge the referencing variable with the referenced variable. When using the shared variable table, the following variables: T x = { 3, 7 }; T* p = &x; T& r = x; { name: \"x\", var_table_index: 3, type: \"T\" } // Captured variables { name: \"p\", value \"0x00500500\", type=\"T*\", var_table_index: 3 } { name: \"r\", type=\"T&\", var_table_index: 3 } { // Shared variable table entry #3: members { name: \"m1\", value: \"3\", type: \"int\" }, members { name: \"m2\", value: \"7\", type: \"int\" } } Note that the pointer address is stored with the referencing variable and not with the referenced variable. This allows the referenced variable to be shared between pointers and references. The type field is optional. The debugger agent may or may not support it."]
pub struct Variable {
    #[serde(rename = "members")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Members contained or pointed to by the variable."]
    pub members: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Variable>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the variable, if any."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status associated with the variable. This field will usually stay unset. A status of a single variable only applies to that variable or expression. The rest of breakpoint data still remains valid. Variables might be reported in error state even when breakpoint is not in final state. The message may refer to variable name with `refers_to` set to `VARIABLE_NAME`. Alternatively `refers_to` will be set to `VARIABLE_VALUE`. In either case variable value and members will be unset. Example of error message applied to name: `Invalid expression syntax`. Example of information message applied to value: `Not captured`. Examples of error message applied to value: * `Malformed string`, * `Field f not found in class C` * `Null pointer dereference`"]
    pub status: ::std::option::Option<::std::boxed::Box<StatusMessage>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Variable type (e.g. `MyClass`). If the variable is split with `var_table_index`, `type` goes next to `value`. The interpretation of a type is agent specific. It is recommended to include the dynamic type rather than a static type of an object."]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Simple value of the variable."]
    pub value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "varTableIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reference to a variable in the shared variable table. More than one variable can reference the same variable in the table. The `var_table_index` field is an index into `variable_table` in Breakpoint."]
    pub var_table_index: ::std::option::Option<::std::primitive::i64>,
}
