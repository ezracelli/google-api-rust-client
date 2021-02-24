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
    pub mod processes {
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of returned processes per page of results. Defaults to 50."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token for continuing a previous list request on the next page. This should be set to the value of `nextPageToken` from a previous response."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProcessFilter.deploymentId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional field used to limit returned processes to those originating from projects with a specific deployment ID."]
                    pub user_process_filter_deployment_id:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProcessFilter.endTime")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional field used to limit returned processes to those that completed on or before the given timestamp."]
                    pub user_process_filter_end_time: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProcessFilter.functionName")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional field used to limit returned processes to those originating from a script function with the given function name."]
                    pub user_process_filter_function_name:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProcessFilter.projectName")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional field used to limit returned processes to those originating from projects with project names containing a specific string."]
                    pub user_process_filter_project_name:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProcessFilter.scriptId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional field used to limit returned processes to those originating from projects with a specific script ID."]
                    pub user_process_filter_script_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProcessFilter.startTime")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional field used to limit returned processes to those that were started on or after the given timestamp."]
                    pub user_process_filter_start_time:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProcessFilter.statuses")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional field used to limit returned processes to those having one of the specified process statuses."]
                    pub user_process_filter_statuses:
                        ::std::option::Option<QueryParametersUserProcessFilterStatusesEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProcessFilter.types")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional field used to limit returned processes to those having one of the specified process types."]
                    pub user_process_filter_types:
                        ::std::option::Option<QueryParametersUserProcessFilterTypesEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "userProcessFilter.userAccessLevels")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional field used to limit returned processes to those having one of the specified user access levels."]
                    pub user_process_filter_user_access_levels:
                        ::std::option::Option<QueryParametersUserProcessFilterUserAccessLevelsEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Optional field used to limit returned processes to those having one of the specified process statuses."]
                pub enum QueryParametersUserProcessFilterStatusesEnum {
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
                impl ::std::default::Default for QueryParametersUserProcessFilterStatusesEnum {
                    fn default() -> Self {
                        Self::ProcessStatusUnspecified
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Optional field used to limit returned processes to those having one of the specified process types."]
                pub enum QueryParametersUserProcessFilterTypesEnum {
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
                impl ::std::default::Default for QueryParametersUserProcessFilterTypesEnum {
                    fn default() -> Self {
                        Self::ProcessTypeUnspecified
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Optional field used to limit returned processes to those having one of the specified user access levels."]
                pub enum QueryParametersUserProcessFilterUserAccessLevelsEnum {
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
                impl ::std::default::Default for QueryParametersUserProcessFilterUserAccessLevelsEnum {
                    fn default() -> Self {
                        Self::UserAccessLevelUnspecified
                    }
                }
            }
            pub mod list_script_processes {
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
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of returned processes per page of results. Defaults to 50."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token for continuing a previous list request on the next page. This should be set to the value of `nextPageToken` from a previous response."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "scriptId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The script ID of the project whose processes are listed."]
                    pub script_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "scriptProcessFilter.deploymentId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional field used to limit returned processes to those originating from projects with a specific deployment ID."]
                    pub script_process_filter_deployment_id:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "scriptProcessFilter.endTime")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional field used to limit returned processes to those that completed on or before the given timestamp."]
                    pub script_process_filter_end_time:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "scriptProcessFilter.functionName")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional field used to limit returned processes to those originating from a script function with the given function name."]
                    pub script_process_filter_function_name:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "scriptProcessFilter.startTime")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional field used to limit returned processes to those that were started on or after the given timestamp."]
                    pub script_process_filter_start_time:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "scriptProcessFilter.statuses")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional field used to limit returned processes to those having one of the specified process statuses."]
                    pub script_process_filter_statuses:
                        ::std::option::Option<QueryParametersScriptProcessFilterStatusesEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "scriptProcessFilter.types")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional field used to limit returned processes to those having one of the specified process types."]
                    pub script_process_filter_types:
                        ::std::option::Option<QueryParametersScriptProcessFilterTypesEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "scriptProcessFilter.userAccessLevels")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional field used to limit returned processes to those having one of the specified user access levels."]
                    pub script_process_filter_user_access_levels: ::std::option::Option<
                        QueryParametersScriptProcessFilterUserAccessLevelsEnum,
                    >,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Optional field used to limit returned processes to those having one of the specified process statuses."]
                pub enum QueryParametersScriptProcessFilterStatusesEnum {
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
                impl ::std::default::Default for QueryParametersScriptProcessFilterStatusesEnum {
                    fn default() -> Self {
                        Self::ProcessStatusUnspecified
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Optional field used to limit returned processes to those having one of the specified process types."]
                pub enum QueryParametersScriptProcessFilterTypesEnum {
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
                impl ::std::default::Default for QueryParametersScriptProcessFilterTypesEnum {
                    fn default() -> Self {
                        Self::ProcessTypeUnspecified
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Optional field used to limit returned processes to those having one of the specified user access levels."]
                pub enum QueryParametersScriptProcessFilterUserAccessLevelsEnum {
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
                impl ::std::default::Default for QueryParametersScriptProcessFilterUserAccessLevelsEnum {
                    fn default() -> Self {
                        Self::UserAccessLevelUnspecified
                    }
                }
            }
        }
    }
    pub mod projects {
        pub mod methods {
            pub mod get_content {
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
                    #[serde(rename = "versionNumber")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The version number of the project to retrieve. If not provided, the project's HEAD version is returned."]
                    pub version_number: ::std::option::Option<::std::primitive::i64>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod get_metrics {
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
                    #[serde(rename = "metricsFilter.deploymentId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional field indicating a specific deployment to retrieve metrics from."]
                    pub metrics_filter_deployment_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "metricsGranularity")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required field indicating what granularity of metrics are returned."]
                    pub metrics_granularity:
                        ::std::option::Option<QueryParametersMetricsGranularityEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Required field indicating what granularity of metrics are returned."]
                pub enum QueryParametersMetricsGranularityEnum {
                    #[serde(rename = "UNSPECIFIED_GRANULARITY")]
                    #[doc = "Default metric granularity used to query no metrics."]
                    UnspecifiedGranularity,
                    #[serde(rename = "WEEKLY")]
                    #[doc = "Represents weekly metrics."]
                    Weekly,
                    #[serde(rename = "DAILY")]
                    #[doc = "Represents daily metrics over a period of 7 days."]
                    Daily,
                }
                impl ::std::default::Default for QueryParametersMetricsGranularityEnum {
                    fn default() -> Self {
                        Self::UnspecifiedGranularity
                    }
                }
            }
        }
        pub mod resources {
            pub mod deployments {
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
                            #[doc = "The maximum number of deployments on each returned page. Defaults to 50."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The token for continuing a previous list request on the next page. This should be set to the value of `nextPageToken` from a previous response."]
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
            pub mod versions {
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
                            #[doc = "The maximum number of versions on each returned page. Defaults to 50."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The token for continuing a previous list request on the next page. This should be set to the value of `nextPageToken` from a previous response."]
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
    #[doc = "The Content resource."]
    pub struct Content {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "files")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of script project files. One of the files is a script manifest; it must be named \"appsscript\", must have type of JSON, and include the manifest configurations for the project."]
        pub files: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<File>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scriptId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The script project's Drive ID."]
        pub script_id: ::std::option::Option<::std::string::String>,
    }
    impl Content {
        pub fn builder() -> ContentBuilder {
            ContentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to create a script project."]
    pub struct CreateProjectRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Drive ID of a parent file that the created script project is bound to. This is usually the ID of a Google Doc, Google Sheet, Google Form, or Google Slides file. If not set, a standalone script project is created."]
        pub parent_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title for the project."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl CreateProjectRequest {
        pub fn builder() -> CreateProjectRequestBuilder {
            CreateProjectRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Representation of a single script deployment."]
    pub struct Deployment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deploymentConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The deployment configuration."]
        pub deployment_config: ::std::option::Option<::std::boxed::Box<DeploymentConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deploymentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The deployment ID for this deployment."]
        pub deployment_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entryPoints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The deployment's entry points."]
        pub entry_points: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EntryPoint>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Last modified date time stamp."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl Deployment {
        pub fn builder() -> DeploymentBuilder {
            DeploymentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata the defines how a deployment is configured."]
    pub struct DeploymentConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The description for this deployment."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "manifestFileName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The manifest file name for this deployment."]
        pub manifest_file_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scriptId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The script project's Drive ID."]
        pub script_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version number on which this deployment is based."]
        pub version_number: ::std::option::Option<::std::primitive::i64>,
    }
    impl DeploymentConfig {
        pub fn builder() -> DeploymentConfigBuilder {
            DeploymentConfigBuilder::default()
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
    #[doc = "A configuration that defines how a deployment is accessed externally."]
    pub struct EntryPoint {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addOn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Add-on properties."]
        pub add_on: ::std::option::Option<::std::boxed::Box<GoogleAppsScriptTypeAddOnEntryPoint>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entryPointType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the entry point."]
        pub entry_point_type: ::std::option::Option<EntryPointEntryPointTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionApi")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An entry point specification for Apps Script API execution calls."]
        pub execution_api:
            ::std::option::Option<::std::boxed::Box<GoogleAppsScriptTypeExecutionApiEntryPoint>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webApp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An entry point specification for web apps."]
        pub web_app: ::std::option::Option<::std::boxed::Box<GoogleAppsScriptTypeWebAppEntryPoint>>,
    }
    impl EntryPoint {
        pub fn builder() -> EntryPointBuilder {
            EntryPointBuilder::default()
        }
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
    impl ::std::default::Default for EntryPointEntryPointTypeEnum {
        fn default() -> Self {
            Self::EntryPointTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for executing or debugging a function in an Apps Script project."]
    pub struct ExecuteStreamResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "result")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of the execution."]
        pub result: ::std::option::Option<::std::boxed::Box<ScriptExecutionResult>>,
    }
    impl ExecuteStreamResponse {
        pub fn builder() -> ExecuteStreamResponseBuilder {
            ExecuteStreamResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An object that provides information about the nature of an error resulting from an attempted execution of a script function using the Apps Script API. If a run call succeeds but the script function (or Apps Script itself) throws an exception, the response body's error field contains a Status object. The `Status` object's `details` field contains an array with a single one of these `ExecutionError` objects."]
    pub struct ExecutionError {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error message thrown by Apps Script, usually localized into the user's language."]
        pub error_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error type, for example `TypeError` or `ReferenceError`. If the error type is unavailable, this field is not included."]
        pub error_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scriptStackTraceElements")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An array of objects that provide a stack trace through the script to show where the execution failed, with the deepest call first."]
        pub script_stack_trace_elements:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ScriptStackTraceElement>>>,
    }
    impl ExecutionError {
        pub fn builder() -> ExecutionErrorBuilder {
            ExecutionErrorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to run the function in a script. The script is identified by the specified `script_id`. Executing a function on a script returns results based on the implementation of the script."]
    pub struct ExecutionRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "devMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If `true` and the user is an owner of the script, the script runs at the most recently saved version rather than the version deployed for use with the Apps Script API. Optional; default is `false`."]
        pub dev_mode: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "function")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the function to execute in the given script. The name does not include parentheses or parameters. It can reference a function in an included library such as `Library.libFunction1`."]
        pub function: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The parameters to be passed to the function being executed. The object type for each parameter should match the expected type in Apps Script. Parameters cannot be Apps Script-specific object types (such as a `Document` or a `Calendar`); they can only be primitive types such as `string`, `number`, `array`, `object`, or `boolean`. Optional."]
        pub parameters: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sessionState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "*Deprecated*. For use with Android add-ons only. An ID that represents the user's current session in the Android app for Google Docs or Sheets, included as extra data in the [Intent](https://developer.android.com/guide/components/intents-filters.html) that launches the add-on. When an Android add-on is run with a session state, it gains the privileges of a [bound](https://developers.google.com/apps-script/guides/bound) script—that is, it can access information like the user's current cursor position (in Docs) or selected cell (in Sheets). To retrieve the state, call `Intent.getStringExtra(\"com.google.android.apps.docs.addons.SessionState\")`. Optional."]
        pub session_state: ::std::option::Option<::std::string::String>,
    }
    impl ExecutionRequest {
        pub fn builder() -> ExecutionRequestBuilder {
            ExecutionRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An object that provides the return value of a function executed using the Apps Script API. If the script function returns successfully, the response body's response field contains this `ExecutionResponse` object."]
    pub struct ExecutionResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "result")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The return value of the script function. The type matches the object type returned in Apps Script. Functions called using the Apps Script API cannot return Apps Script-specific objects (such as a `Document` or a `Calendar`); they can only return primitive types such as a `string`, `number`, `array`, `object`, or `boolean`."]
        pub result: ::std::option::Option<::serde_json::Value>,
    }
    impl ExecutionResponse {
        pub fn builder() -> ExecutionResponseBuilder {
            ExecutionResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An individual file within a script project. A file is a third-party source code created by one or more developers. It can be a server-side JS code, HTML, or a configuration file. Each script project can contain multiple files."]
    pub struct File {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Creation date timestamp. This read-only field is only visible to users who have WRITER permission for the script project."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "functionSet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The defined set of functions in the script file, if any."]
        pub function_set: ::std::option::Option<::std::boxed::Box<GoogleAppsScriptTypeFunctionSet>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifyUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user who modified the file most recently. This read-only field is only visible to users who have WRITER permission for the script project."]
        pub last_modify_user: ::std::option::Option<::std::boxed::Box<GoogleAppsScriptTypeUser>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the file. The file extension is not part of the file name, which can be identified from the type field."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The file content."]
        pub source: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the file."]
        pub _type: ::std::option::Option<FileTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Last modified date timestamp. This read-only field is only visible to users who have WRITER permission for the script project."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl File {
        pub fn builder() -> FileBuilder {
            FileBuilder::default()
        }
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
    impl ::std::default::Default for FileTypeEnum {
        fn default() -> Self {
            Self::EnumTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An add-on entry point."]
    pub struct GoogleAppsScriptTypeAddOnEntryPoint {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addOnType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The add-on's required list of supported container types."]
        pub add_on_type: ::std::option::Option<GoogleAppsScriptTypeAddOnEntryPointAddOnTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The add-on's optional description."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "helpUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The add-on's optional help URL."]
        pub help_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postInstallTipUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The add-on's required post install tip URL."]
        pub post_install_tip_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reportIssueUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The add-on's optional report issue URL."]
        pub report_issue_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The add-on's required title."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAppsScriptTypeAddOnEntryPoint {
        pub fn builder() -> GoogleAppsScriptTypeAddOnEntryPointBuilder {
            GoogleAppsScriptTypeAddOnEntryPointBuilder::default()
        }
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
    impl ::std::default::Default for GoogleAppsScriptTypeAddOnEntryPointAddOnTypeEnum {
        fn default() -> Self {
            Self::UnknownAddonType
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "API executable entry point configuration."]
    pub struct GoogleAppsScriptTypeExecutionApiConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "access")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Who has permission to run the API executable."]
        pub access: ::std::option::Option<GoogleAppsScriptTypeExecutionApiConfigAccessEnum>,
    }
    impl GoogleAppsScriptTypeExecutionApiConfig {
        pub fn builder() -> GoogleAppsScriptTypeExecutionApiConfigBuilder {
            GoogleAppsScriptTypeExecutionApiConfigBuilder::default()
        }
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
    impl ::std::default::Default for GoogleAppsScriptTypeExecutionApiConfigAccessEnum {
        fn default() -> Self {
            Self::UnknownAccess
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An API executable entry point."]
    pub struct GoogleAppsScriptTypeExecutionApiEntryPoint {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entryPointConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entry point's configuration."]
        pub entry_point_config:
            ::std::option::Option<::std::boxed::Box<GoogleAppsScriptTypeExecutionApiConfig>>,
    }
    impl GoogleAppsScriptTypeExecutionApiEntryPoint {
        pub fn builder() -> GoogleAppsScriptTypeExecutionApiEntryPointBuilder {
            GoogleAppsScriptTypeExecutionApiEntryPointBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a function in a script project."]
    pub struct GoogleAppsScriptTypeFunction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The function name in the script project."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAppsScriptTypeFunction {
        pub fn builder() -> GoogleAppsScriptTypeFunctionBuilder {
            GoogleAppsScriptTypeFunctionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A set of functions. No duplicates are permitted."]
    pub struct GoogleAppsScriptTypeFunctionSet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "values")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of functions composing the set."]
        pub values:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleAppsScriptTypeFunction>>>,
    }
    impl GoogleAppsScriptTypeFunctionSet {
        pub fn builder() -> GoogleAppsScriptTypeFunctionSetBuilder {
            GoogleAppsScriptTypeFunctionSetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Representation of a single script process execution that was started from the script editor, a trigger, an application, or using the Apps Script API. This is distinct from the `Operation` resource, which only represents executions started via the Apps Script API."]
    pub struct GoogleAppsScriptTypeProcess {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Duration the execution spent executing."]
        pub duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "functionName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the function the started the execution."]
        pub function_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "processStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The executions status."]
        pub process_status: ::std::option::Option<GoogleAppsScriptTypeProcessProcessStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "processType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The executions type."]
        pub process_type: ::std::option::Option<GoogleAppsScriptTypeProcessProcessTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the script being executed."]
        pub project_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time the execution started."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userAccessLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The executing users access level to the script."]
        pub user_access_level:
            ::std::option::Option<GoogleAppsScriptTypeProcessUserAccessLevelEnum>,
    }
    impl GoogleAppsScriptTypeProcess {
        pub fn builder() -> GoogleAppsScriptTypeProcessBuilder {
            GoogleAppsScriptTypeProcessBuilder::default()
        }
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
    impl ::std::default::Default for GoogleAppsScriptTypeProcessProcessStatusEnum {
        fn default() -> Self {
            Self::ProcessStatusUnspecified
        }
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
    impl ::std::default::Default for GoogleAppsScriptTypeProcessProcessTypeEnum {
        fn default() -> Self {
            Self::ProcessTypeUnspecified
        }
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
    impl ::std::default::Default for GoogleAppsScriptTypeProcessUserAccessLevelEnum {
        fn default() -> Self {
            Self::UserAccessLevelUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A simple user profile resource."]
    pub struct GoogleAppsScriptTypeUser {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's domain."]
        pub domain: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's identifying email address."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's display name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "photoUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's photo."]
        pub photo_url: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAppsScriptTypeUser {
        pub fn builder() -> GoogleAppsScriptTypeUserBuilder {
            GoogleAppsScriptTypeUserBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Web app entry point configuration."]
    pub struct GoogleAppsScriptTypeWebAppConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "access")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Who has permission to run the web app."]
        pub access: ::std::option::Option<GoogleAppsScriptTypeWebAppConfigAccessEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executeAs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Who to execute the web app as."]
        pub execute_as: ::std::option::Option<GoogleAppsScriptTypeWebAppConfigExecuteAsEnum>,
    }
    impl GoogleAppsScriptTypeWebAppConfig {
        pub fn builder() -> GoogleAppsScriptTypeWebAppConfigBuilder {
            GoogleAppsScriptTypeWebAppConfigBuilder::default()
        }
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
    impl ::std::default::Default for GoogleAppsScriptTypeWebAppConfigAccessEnum {
        fn default() -> Self {
            Self::UnknownAccess
        }
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
    impl ::std::default::Default for GoogleAppsScriptTypeWebAppConfigExecuteAsEnum {
        fn default() -> Self {
            Self::UnknownExecuteAs
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A web application entry point."]
    pub struct GoogleAppsScriptTypeWebAppEntryPoint {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entryPointConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entry point's configuration."]
        pub entry_point_config:
            ::std::option::Option<::std::boxed::Box<GoogleAppsScriptTypeWebAppConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL for the web application."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAppsScriptTypeWebAppEntryPoint {
        pub fn builder() -> GoogleAppsScriptTypeWebAppEntryPointBuilder {
            GoogleAppsScriptTypeWebAppEntryPointBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response with the list of deployments for the specified Apps Script project."]
    pub struct ListDeploymentsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deployments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of deployments."]
        pub deployments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Deployment>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used in the next call to get the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListDeploymentsResponse {
        pub fn builder() -> ListDeploymentsResponseBuilder {
            ListDeploymentsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response with the list of Process resources."]
    pub struct ListScriptProcessesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token for the next page of results. If empty, there are no more pages remaining."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "processes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of processes matching request parameters."]
        pub processes:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleAppsScriptTypeProcess>>>,
    }
    impl ListScriptProcessesResponse {
        pub fn builder() -> ListScriptProcessesResponseBuilder {
            ListScriptProcessesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response with the list of Process resources."]
    pub struct ListUserProcessesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token for the next page of results. If empty, there are no more pages remaining."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "processes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of processes matching request parameters."]
        pub processes:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleAppsScriptTypeProcess>>>,
    }
    impl ListUserProcessesResponse {
        pub fn builder() -> ListUserProcessesResponseBuilder {
            ListUserProcessesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "`ListValue` is a wrapper around a repeated field of values."]
    pub struct ListValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "values")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Repeated field of dynamically typed values."]
        pub values: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Value>>>,
    }
    impl ListValue {
        pub fn builder() -> ListValueBuilder {
            ListValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response with the list of the versions for the specified script project."]
    pub struct ListVersionsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token use to fetch the next page of records. if not exist in the response, that means no more versions to list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of versions."]
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
    #[doc = "Resource containing usage stats for a given script, based on the supplied filter and mask present in the request."]
    pub struct Metrics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "activeUsers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of active users."]
        pub active_users: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricsValue>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "failedExecutions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of failed executions."]
        pub failed_executions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricsValue>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalExecutions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of total executions."]
        pub total_executions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricsValue>>>,
    }
    impl Metrics {
        pub fn builder() -> MetricsBuilder {
            MetricsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metrics value that holds number of executions counted."]
    pub struct MetricsValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required field indicating the end time of the interval."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required field indicating the start time of the interval."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates the number of executions counted."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl MetricsValue {
        pub fn builder() -> MetricsValueBuilder {
            MetricsValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A representation of an execution of an Apps Script function started with run. The execution response does not arrive until the function finishes executing. The maximum execution runtime is listed in the [Apps Script quotas guide](/apps-script/guides/services/quotas#current_limitations). After execution has started, it can have one of four outcomes: - If the script function returns successfully, the response field contains an ExecutionResponse object with the function's return value in the object's `result` field. - If the script function (or Apps Script itself) throws an exception, the error field contains a Status object. The `Status` object's `details` field contains an array with a single ExecutionError object that provides information about the nature of the error. - If the execution has not yet completed, the done field is `false` and the neither the `response` nor `error` fields are present. - If the `run` call itself fails (for example, because of a malformed request or an authorization error), the method returns an HTTP response code in the 4XX range with a different format for the response body. Client libraries automatically convert a 4XX response into an exception class. "]
    pub struct Operation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "done")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field indicates whether the script execution has completed. A completed execution has a populated `response` field containing the ExecutionResponse from function that was executed."]
        pub done: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If a `run` call succeeds but the script function (or Apps Script itself) throws an exception, this field contains a Status object. The `Status` object's `details` field contains an array with a single ExecutionError object that provides information about the nature of the error."]
        pub error: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "response")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the script function returns successfully, this field contains an ExecutionResponse object with the function's return value."]
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
    #[doc = "The script project resource."]
    pub struct Project {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the script was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creator")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User who originally created the script."]
        pub creator: ::std::option::Option<::std::boxed::Box<GoogleAppsScriptTypeUser>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifyUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User who last modified the script."]
        pub last_modify_user: ::std::option::Option<::std::boxed::Box<GoogleAppsScriptTypeUser>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The parent's Drive ID that the script will be attached to. This is usually the ID of a Google Document or Google Sheet. This filed is optional, and if not set, a stand-alone script will be created."]
        pub parent_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scriptId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The script project's Drive ID."]
        pub script_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title for the project."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the script was last updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl Project {
        pub fn builder() -> ProjectBuilder {
            ProjectBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of an execution."]
    pub struct ScriptExecutionResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "returnValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The returned value of the execution."]
        pub return_value: ::std::option::Option<::std::boxed::Box<Value>>,
    }
    impl ScriptExecutionResult {
        pub fn builder() -> ScriptExecutionResultBuilder {
            ScriptExecutionResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A stack trace through the script that shows where the execution failed."]
    pub struct ScriptStackTraceElement {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "function")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the function that failed."]
        pub function: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The line number where the script failed."]
        pub line_number: ::std::option::Option<::std::primitive::i64>,
    }
    impl ScriptStackTraceElement {
        pub fn builder() -> ScriptStackTraceElementBuilder {
            ScriptStackTraceElementBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "If a `run` call succeeds but the script function (or Apps Script itself) throws an exception, the response body's error field contains this `Status` object."]
    pub struct Status {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status code. For this API, this value either: - 10, indicating a `SCRIPT_TIMEOUT` error, - 3, indicating an `INVALID_ARGUMENT` error, or - 1, indicating a `CANCELLED` execution. "]
        pub code: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "details")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An array that contains a single ExecutionError object that provides information about the nature of the error."]
        pub details: ::std::option::Option<
            ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A developer-facing error message, which is in English. Any user-facing error message is localized and sent in the details field, or localized by the client."]
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
    #[doc = "`Struct` represents a structured data value, consisting of fields which map to dynamically typed values."]
    pub struct Struct {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unordered map of dynamically typed values."]
        pub fields:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<Value>>>,
    }
    impl Struct {
        pub fn builder() -> StructBuilder {
            StructBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request with deployment information to update an existing deployment."]
    pub struct UpdateDeploymentRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deploymentConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The deployment configuration."]
        pub deployment_config: ::std::option::Option<::std::boxed::Box<DeploymentConfig>>,
    }
    impl UpdateDeploymentRequest {
        pub fn builder() -> UpdateDeploymentRequestBuilder {
            UpdateDeploymentRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "`Value` represents a dynamically typed value which is the outcome of an executed script."]
    pub struct Value {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boolValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a boolean value."]
        pub bool_value: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bytesValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents raw byte values."]
        pub bytes_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dateValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a date in ms since the epoch."]
        pub date_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "listValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a repeated `Value`."]
        pub list_value: ::std::option::Option<::std::boxed::Box<ListValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nullValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a null value."]
        pub null_value: ::std::option::Option<ValueNullValueEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numberValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a double value."]
        pub number_value: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "protoValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a structured proto value."]
        pub proto_value:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stringValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a string value."]
        pub string_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "structValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a structured value."]
        pub struct_value: ::std::option::Option<::std::boxed::Box<Struct>>,
    }
    impl Value {
        pub fn builder() -> ValueBuilder {
            ValueBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Represents a null value."]
    pub enum ValueNullValueEnum {
        #[serde(rename = "NULL_VALUE")]
        #[doc = "Null value."]
        NullValue,
    }
    impl ::std::default::Default for ValueNullValueEnum {
        fn default() -> Self {
            Self::NullValue
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A resource representing a script project version. A version is a \"snapshot\" of a script project and is similar to a read-only branched release. When creating deployments, the version to use must be specified."]
    pub struct Version {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the version was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The description for this version."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scriptId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The script project's Drive ID."]
        pub script_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The incremental ID that is created by Apps Script when a version is created. This is system assigned number and is immutable once created."]
        pub version_number: ::std::option::Option<::std::primitive::i64>,
    }
    impl Version {
        pub fn builder() -> VersionBuilder {
            VersionBuilder::default()
        }
    }
}
