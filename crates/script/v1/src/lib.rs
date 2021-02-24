#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The Content resource."]
pub struct Content {
    #[serde(rename = "files")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of script project files. One of the files is a script manifest; it must be named \"appsscript\", must have type of JSON, and include the manifest configurations for the project."]
    pub files: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<File>>>,
    #[serde(rename = "scriptId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The script project's Drive ID."]
    pub script_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to create a script project."]
pub struct CreateProjectRequest {
    #[serde(rename = "parentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Drive ID of a parent file that the created script project is bound to. This is usually the ID of a Google Doc, Google Sheet, Google Form, or Google Slides file. If not set, a standalone script project is created."]
    pub parent_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title for the project."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Representation of a single script deployment."]
pub struct Deployment {
    #[serde(rename = "deploymentConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The deployment configuration."]
    pub deployment_config: ::std::option::Option<::std::boxed::Box<DeploymentConfig>>,
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The deployment ID for this deployment."]
    pub deployment_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entryPoints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The deployment's entry points."]
    pub entry_points: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EntryPoint>>>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Last modified date time stamp."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata the defines how a deployment is configured."]
pub struct DeploymentConfig {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The description for this deployment."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "manifestFileName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The manifest file name for this deployment."]
    pub manifest_file_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scriptId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The script project's Drive ID."]
    pub script_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "versionNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version number on which this deployment is based."]
    pub version_number: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A configuration that defines how a deployment is accessed externally."]
pub struct EntryPoint {
    #[serde(rename = "addOn")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Add-on properties."]
    pub add_on: ::std::option::Option<::std::boxed::Box<GoogleAppsScriptTypeAddOnEntryPoint>>,
    #[serde(rename = "entryPointType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the entry point."]
    pub entry_point_type: ::std::option::Option<EntryPointEntryPointTypeEnum>,
    #[serde(rename = "executionApi")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An entry point specification for Apps Script API execution calls."]
    pub execution_api:
        ::std::option::Option<::std::boxed::Box<GoogleAppsScriptTypeExecutionApiEntryPoint>>,
    #[serde(rename = "webApp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An entry point specification for web apps."]
    pub web_app: ::std::option::Option<::std::boxed::Box<GoogleAppsScriptTypeWebAppEntryPoint>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the entry point."]
pub enum EntryPointEntryPointTypeEnum {
    #[serde(rename = "ENTRY_POINT_TYPE_UNSPECIFIED")]
    #[doc = "An unspecified entry point."]
    EntryPointTypeUnspecified,
    #[serde(rename = "WEB_APP")]
    #[doc = "A web application entry point."]
    WebApp,
    #[serde(rename = "EXECUTION_API")]
    #[doc = "An API executable entry point."]
    ExecutionApi,
    #[serde(rename = "ADD_ON")]
    #[doc = "An Add-On entry point."]
    AddOn,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for executing or debugging a function in an Apps Script project."]
pub struct ExecuteStreamResponse {
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of the execution."]
    pub result: ::std::option::Option<::std::boxed::Box<ScriptExecutionResult>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An object that provides information about the nature of an error resulting from an attempted execution of a script function using the Apps Script API. If a run call succeeds but the script function (or Apps Script itself) throws an exception, the response body's error field contains a Status object. The `Status` object's `details` field contains an array with a single one of these `ExecutionError` objects."]
pub struct ExecutionError {
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The error message thrown by Apps Script, usually localized into the user's language."]
    pub error_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "errorType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The error type, for example `TypeError` or `ReferenceError`. If the error type is unavailable, this field is not included."]
    pub error_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scriptStackTraceElements")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An array of objects that provide a stack trace through the script to show where the execution failed, with the deepest call first."]
    pub script_stack_trace_elements:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ScriptStackTraceElement>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request to run the function in a script. The script is identified by the specified `script_id`. Executing a function on a script returns results based on the implementation of the script."]
pub struct ExecutionRequest {
    #[serde(rename = "devMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If `true` and the user is an owner of the script, the script runs at the most recently saved version rather than the version deployed for use with the Apps Script API. Optional; default is `false`."]
    pub dev_mode: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "function")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the function to execute in the given script. The name does not include parentheses or parameters. It can reference a function in an included library such as `Library.libFunction1`."]
    pub function: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The parameters to be passed to the function being executed. The object type for each parameter should match the expected type in Apps Script. Parameters cannot be Apps Script-specific object types (such as a `Document` or a `Calendar`); they can only be primitive types such as `string`, `number`, `array`, `object`, or `boolean`. Optional."]
    pub parameters: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
    #[serde(rename = "sessionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "*Deprecated*. For use with Android add-ons only. An ID that represents the user's current session in the Android app for Google Docs or Sheets, included as extra data in the [Intent](https://developer.android.com/guide/components/intents-filters.html) that launches the add-on. When an Android add-on is run with a session state, it gains the privileges of a [bound](https://developers.google.com/apps-script/guides/bound) script—that is, it can access information like the user's current cursor position (in Docs) or selected cell (in Sheets). To retrieve the state, call `Intent.getStringExtra(\"com.google.android.apps.docs.addons.SessionState\")`. Optional."]
    pub session_state: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An object that provides the return value of a function executed using the Apps Script API. If the script function returns successfully, the response body's response field contains this `ExecutionResponse` object."]
pub struct ExecutionResponse {
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The return value of the script function. The type matches the object type returned in Apps Script. Functions called using the Apps Script API cannot return Apps Script-specific objects (such as a `Document` or a `Calendar`); they can only return primitive types such as a `string`, `number`, `array`, `object`, or `boolean`."]
    pub result: ::std::option::Option<::serde_json::Value>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An individual file within a script project. A file is a third-party source code created by one or more developers. It can be a server-side JS code, HTML, or a configuration file. Each script project can contain multiple files."]
pub struct File {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creation date timestamp. This read-only field is only visible to users who have WRITER permission for the script project."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "functionSet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The defined set of functions in the script file, if any."]
    pub function_set: ::std::option::Option<::std::boxed::Box<GoogleAppsScriptTypeFunctionSet>>,
    #[serde(rename = "lastModifyUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user who modified the file most recently. This read-only field is only visible to users who have WRITER permission for the script project."]
    pub last_modify_user: ::std::option::Option<::std::boxed::Box<GoogleAppsScriptTypeUser>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the file. The file extension is not part of the file name, which can be identified from the type field."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The file content."]
    pub source: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the file."]
    pub _type: ::std::option::Option<FileTypeEnum>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Last modified date timestamp. This read-only field is only visible to users who have WRITER permission for the script project."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the file."]
pub enum FileTypeEnum {
    #[serde(rename = "ENUM_TYPE_UNSPECIFIED")]
    #[doc = "Undetermined file type; never actually used."]
    EnumTypeUnspecified,
    #[serde(rename = "SERVER_JS")]
    #[doc = "An Apps Script server-side code file."]
    ServerJs,
    #[serde(rename = "HTML")]
    #[doc = "A file containing client-side HTML."]
    Html,
    #[serde(rename = "JSON")]
    #[doc = "A file in JSON format. This type is only used for the script project's manifest. The manifest file content must match the structure of a valid [ScriptManifest](/apps-script/concepts/manifests)"]
    Json,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An add-on entry point."]
pub struct GoogleAppsScriptTypeAddOnEntryPoint {
    #[serde(rename = "addOnType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The add-on's required list of supported container types."]
    pub add_on_type: ::std::option::Option<GoogleAppsScriptTypeAddOnEntryPointAddOnTypeEnum>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The add-on's optional description."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "helpUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The add-on's optional help URL."]
    pub help_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "postInstallTipUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The add-on's required post install tip URL."]
    pub post_install_tip_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reportIssueUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The add-on's optional report issue URL."]
    pub report_issue_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The add-on's required title."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The add-on's required list of supported container types."]
pub enum GoogleAppsScriptTypeAddOnEntryPointAddOnTypeEnum {
    #[serde(rename = "UNKNOWN_ADDON_TYPE")]
    #[doc = "Default value, unknown add-on type."]
    UnknownAddonType,
    #[serde(rename = "GMAIL")]
    #[doc = "Add-on type for Gmail."]
    Gmail,
    #[serde(rename = "DATA_STUDIO")]
    #[doc = "Add-on type for Data Studio."]
    DataStudio,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "API executable entry point configuration."]
pub struct GoogleAppsScriptTypeExecutionApiConfig {
    #[serde(rename = "access")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Who has permission to run the API executable."]
    pub access: ::std::option::Option<GoogleAppsScriptTypeExecutionApiConfigAccessEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Who has permission to run the API executable."]
pub enum GoogleAppsScriptTypeExecutionApiConfigAccessEnum {
    #[serde(rename = "UNKNOWN_ACCESS")]
    #[doc = "Default value, should not be used."]
    UnknownAccess,
    #[serde(rename = "MYSELF")]
    #[doc = "Only the user who deployed the web app or executable can access it. Note that this is not necessarily the owner of the script project."]
    Myself,
    #[serde(rename = "DOMAIN")]
    #[doc = "Only users in the same domain as the user who deployed the web app or executable can access it."]
    Domain,
    #[serde(rename = "ANYONE")]
    #[doc = "Any logged in user can access the web app or executable."]
    Anyone,
    #[serde(rename = "ANYONE_ANONYMOUS")]
    #[doc = "Any user, logged in or not, can access the web app or executable."]
    AnyoneAnonymous,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An API executable entry point."]
pub struct GoogleAppsScriptTypeExecutionApiEntryPoint {
    #[serde(rename = "entryPointConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The entry point's configuration."]
    pub entry_point_config:
        ::std::option::Option<::std::boxed::Box<GoogleAppsScriptTypeExecutionApiConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a function in a script project."]
pub struct GoogleAppsScriptTypeFunction {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The function name in the script project."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A set of functions. No duplicates are permitted."]
pub struct GoogleAppsScriptTypeFunctionSet {
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of functions composing the set."]
    pub values:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleAppsScriptTypeFunction>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Representation of a single script process execution that was started from the script editor, a trigger, an application, or using the Apps Script API. This is distinct from the `Operation` resource, which only represents executions started via the Apps Script API."]
pub struct GoogleAppsScriptTypeProcess {
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Duration the execution spent executing."]
    pub duration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "functionName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the function the started the execution."]
    pub function_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "processStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The executions status."]
    pub process_status: ::std::option::Option<GoogleAppsScriptTypeProcessProcessStatusEnum>,
    #[serde(rename = "processType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The executions type."]
    pub process_type: ::std::option::Option<GoogleAppsScriptTypeProcessProcessTypeEnum>,
    #[serde(rename = "projectName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the script being executed."]
    pub project_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time the execution started."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userAccessLevel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The executing users access level to the script."]
    pub user_access_level: ::std::option::Option<GoogleAppsScriptTypeProcessUserAccessLevelEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The executions status."]
pub enum GoogleAppsScriptTypeProcessProcessStatusEnum {
    #[serde(rename = "PROCESS_STATUS_UNSPECIFIED")]
    #[doc = "Unspecified status."]
    ProcessStatusUnspecified,
    #[serde(rename = "RUNNING")]
    #[doc = "The process is currently running."]
    Running,
    #[serde(rename = "PAUSED")]
    #[doc = "The process has paused."]
    Paused,
    #[serde(rename = "COMPLETED")]
    #[doc = "The process has completed."]
    Completed,
    #[serde(rename = "CANCELED")]
    #[doc = "The process was cancelled."]
    Canceled,
    #[serde(rename = "FAILED")]
    #[doc = "The process failed."]
    Failed,
    #[serde(rename = "TIMED_OUT")]
    #[doc = "The process timed out."]
    TimedOut,
    #[serde(rename = "UNKNOWN")]
    #[doc = "Process status unknown."]
    Unknown,
    #[serde(rename = "DELAYED")]
    #[doc = "The process is delayed, waiting for quota."]
    Delayed,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The executions type."]
pub enum GoogleAppsScriptTypeProcessProcessTypeEnum {
    #[serde(rename = "PROCESS_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified type."]
    ProcessTypeUnspecified,
    #[serde(rename = "ADD_ON")]
    #[doc = "The process was started from an add-on entry point."]
    AddOn,
    #[serde(rename = "EXECUTION_API")]
    #[doc = "The process was started using the Apps Script API."]
    ExecutionApi,
    #[serde(rename = "TIME_DRIVEN")]
    #[doc = "The process was started from a time-based trigger."]
    TimeDriven,
    #[serde(rename = "TRIGGER")]
    #[doc = "The process was started from an event-based trigger."]
    Trigger,
    #[serde(rename = "WEBAPP")]
    #[doc = "The process was started from a web app entry point."]
    Webapp,
    #[serde(rename = "EDITOR")]
    #[doc = "The process was started using the Apps Script IDE."]
    Editor,
    #[serde(rename = "SIMPLE_TRIGGER")]
    #[doc = "The process was started from a G Suite simple trigger."]
    SimpleTrigger,
    #[serde(rename = "MENU")]
    #[doc = "The process was started from a G Suite menu item."]
    Menu,
    #[serde(rename = "BATCH_TASK")]
    #[doc = "The process was started as a task in a batch job."]
    BatchTask,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The executing users access level to the script."]
pub enum GoogleAppsScriptTypeProcessUserAccessLevelEnum {
    #[serde(rename = "USER_ACCESS_LEVEL_UNSPECIFIED")]
    #[doc = "User access level unspecified"]
    UserAccessLevelUnspecified,
    #[serde(rename = "NONE")]
    #[doc = "The user has no access."]
    None,
    #[serde(rename = "READ")]
    #[doc = "The user has read-only access."]
    Read,
    #[serde(rename = "WRITE")]
    #[doc = "The user has write access."]
    Write,
    #[serde(rename = "OWNER")]
    #[doc = "The user is an owner."]
    Owner,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A simple user profile resource."]
pub struct GoogleAppsScriptTypeUser {
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user's domain."]
    pub domain: ::std::option::Option<::std::string::String>,
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user's identifying email address."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user's display name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "photoUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user's photo."]
    pub photo_url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Web app entry point configuration."]
pub struct GoogleAppsScriptTypeWebAppConfig {
    #[serde(rename = "access")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Who has permission to run the web app."]
    pub access: ::std::option::Option<GoogleAppsScriptTypeWebAppConfigAccessEnum>,
    #[serde(rename = "executeAs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Who to execute the web app as."]
    pub execute_as: ::std::option::Option<GoogleAppsScriptTypeWebAppConfigExecuteAsEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Who has permission to run the web app."]
pub enum GoogleAppsScriptTypeWebAppConfigAccessEnum {
    #[serde(rename = "UNKNOWN_ACCESS")]
    #[doc = "Default value, should not be used."]
    UnknownAccess,
    #[serde(rename = "MYSELF")]
    #[doc = "Only the user who deployed the web app or executable can access it. Note that this is not necessarily the owner of the script project."]
    Myself,
    #[serde(rename = "DOMAIN")]
    #[doc = "Only users in the same domain as the user who deployed the web app or executable can access it."]
    Domain,
    #[serde(rename = "ANYONE")]
    #[doc = "Any logged in user can access the web app or executable."]
    Anyone,
    #[serde(rename = "ANYONE_ANONYMOUS")]
    #[doc = "Any user, logged in or not, can access the web app or executable."]
    AnyoneAnonymous,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Who to execute the web app as."]
pub enum GoogleAppsScriptTypeWebAppConfigExecuteAsEnum {
    #[serde(rename = "UNKNOWN_EXECUTE_AS")]
    #[doc = "Default value, should not be used."]
    UnknownExecuteAs,
    #[serde(rename = "USER_ACCESSING")]
    #[doc = "The script runs as the user accessing the web app."]
    UserAccessing,
    #[serde(rename = "USER_DEPLOYING")]
    #[doc = "The script runs as the user who deployed the web app. Note that this is not necessarily the owner of the script project."]
    UserDeploying,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A web application entry point."]
pub struct GoogleAppsScriptTypeWebAppEntryPoint {
    #[serde(rename = "entryPointConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The entry point's configuration."]
    pub entry_point_config:
        ::std::option::Option<::std::boxed::Box<GoogleAppsScriptTypeWebAppConfig>>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL for the web application."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response with the list of deployments for the specified Apps Script project."]
pub struct ListDeploymentsResponse {
    #[serde(rename = "deployments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of deployments."]
    pub deployments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Deployment>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token that can be used in the next call to get the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response with the list of Process resources."]
pub struct ListScriptProcessesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token for the next page of results. If empty, there are no more pages remaining."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "processes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of processes matching request parameters."]
    pub processes:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleAppsScriptTypeProcess>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response with the list of Process resources."]
pub struct ListUserProcessesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token for the next page of results. If empty, there are no more pages remaining."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "processes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of processes matching request parameters."]
    pub processes:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleAppsScriptTypeProcess>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`ListValue` is a wrapper around a repeated field of values."]
pub struct ListValue {
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Repeated field of dynamically typed values."]
    pub values: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Value>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response with the list of the versions for the specified script project."]
pub struct ListVersionsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token use to fetch the next page of records. if not exist in the response, that means no more versions to list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "versions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of versions."]
    pub versions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Version>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Resource containing usage stats for a given script, based on the supplied filter and mask present in the request."]
pub struct Metrics {
    #[serde(rename = "activeUsers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of active users."]
    pub active_users: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricsValue>>>,
    #[serde(rename = "failedExecutions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of failed executions."]
    pub failed_executions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricsValue>>>,
    #[serde(rename = "totalExecutions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of total executions."]
    pub total_executions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricsValue>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metrics value that holds number of executions counted."]
pub struct MetricsValue {
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required field indicating the end time of the interval."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required field indicating the start time of the interval."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the number of executions counted."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A representation of an execution of an Apps Script function started with run. The execution response does not arrive until the function finishes executing. The maximum execution runtime is listed in the [Apps Script quotas guide](/apps-script/guides/services/quotas#current_limitations). After execution has started, it can have one of four outcomes: - If the script function returns successfully, the response field contains an ExecutionResponse object with the function's return value in the object's `result` field. - If the script function (or Apps Script itself) throws an exception, the error field contains a Status object. The `Status` object's `details` field contains an array with a single ExecutionError object that provides information about the nature of the error. - If the execution has not yet completed, the done field is `false` and the neither the `response` nor `error` fields are present. - If the `run` call itself fails (for example, because of a malformed request or an authorization error), the method returns an HTTP response code in the 4XX range with a different format for the response body. Client libraries automatically convert a 4XX response into an exception class. "]
pub struct Operation {
    #[serde(rename = "done")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field indicates whether the script execution has completed. A completed execution has a populated `response` field containing the ExecutionResponse from function that was executed."]
    pub done: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If a `run` call succeeds but the script function (or Apps Script itself) throws an exception, this field contains a Status object. The `Status` object's `details` field contains an array with a single ExecutionError object that provides information about the nature of the error."]
    pub error: ::std::option::Option<::std::boxed::Box<Status>>,
    #[serde(rename = "response")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the script function returns successfully, this field contains an ExecutionResponse object with the function's return value."]
    pub response: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The script project resource."]
pub struct Project {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When the script was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creator")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User who originally created the script."]
    pub creator: ::std::option::Option<::std::boxed::Box<GoogleAppsScriptTypeUser>>,
    #[serde(rename = "lastModifyUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User who last modified the script."]
    pub last_modify_user: ::std::option::Option<::std::boxed::Box<GoogleAppsScriptTypeUser>>,
    #[serde(rename = "parentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The parent's Drive ID that the script will be attached to. This is usually the ID of a Google Document or Google Sheet. This filed is optional, and if not set, a stand-alone script will be created."]
    pub parent_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scriptId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The script project's Drive ID."]
    pub script_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title for the project."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When the script was last updated."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The result of an execution."]
pub struct ScriptExecutionResult {
    #[serde(rename = "returnValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The returned value of the execution."]
    pub return_value: ::std::option::Option<::std::boxed::Box<Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A stack trace through the script that shows where the execution failed."]
pub struct ScriptStackTraceElement {
    #[serde(rename = "function")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the function that failed."]
    pub function: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lineNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The line number where the script failed."]
    pub line_number: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "If a `run` call succeeds but the script function (or Apps Script itself) throws an exception, the response body's error field contains this `Status` object."]
pub struct Status {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status code. For this API, this value either: - 10, indicating a `SCRIPT_TIMEOUT` error, - 3, indicating an `INVALID_ARGUMENT` error, or - 1, indicating a `CANCELLED` execution. "]
    pub code: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An array that contains a single ExecutionError object that provides information about the nature of the error."]
    pub details: ::std::option::Option<
        ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    >,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A developer-facing error message, which is in English. Any user-facing error message is localized and sent in the details field, or localized by the client."]
    pub message: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`Struct` represents a structured data value, consisting of fields which map to dynamically typed values."]
pub struct Struct {
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unordered map of dynamically typed values."]
    pub fields:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<Value>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request with deployment information to update an existing deployment."]
pub struct UpdateDeploymentRequest {
    #[serde(rename = "deploymentConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The deployment configuration."]
    pub deployment_config: ::std::option::Option<::std::boxed::Box<DeploymentConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`Value` represents a dynamically typed value which is the outcome of an executed script."]
pub struct Value {
    #[serde(rename = "boolValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents a boolean value."]
    pub bool_value: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "bytesValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents raw byte values."]
    pub bytes_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dateValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents a date in ms since the epoch."]
    pub date_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "listValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents a repeated `Value`."]
    pub list_value: ::std::option::Option<::std::boxed::Box<ListValue>>,
    #[serde(rename = "nullValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents a null value."]
    pub null_value: ::std::option::Option<ValueNullValueEnum>,
    #[serde(rename = "numberValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents a double value."]
    pub number_value: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "protoValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents a structured proto value."]
    pub proto_value:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "stringValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents a string value."]
    pub string_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "structValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents a structured value."]
    pub struct_value: ::std::option::Option<::std::boxed::Box<Struct>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Represents a null value."]
pub enum ValueNullValueEnum {
    #[serde(rename = "NULL_VALUE")]
    #[doc = "Null value."]
    NullValue,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A resource representing a script project version. A version is a \"snapshot\" of a script project and is similar to a read-only branched release. When creating deployments, the version to use must be specified."]
pub struct Version {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When the version was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The description for this version."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scriptId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The script project's Drive ID."]
    pub script_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "versionNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The incremental ID that is created by Apps Script when a version is created. This is system assigned number and is immutable once created."]
    pub version_number: ::std::option::Option<::std::primitive::i64>,
}
