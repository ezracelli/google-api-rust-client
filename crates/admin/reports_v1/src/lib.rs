#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "JSON template for a collection of activities."]
pub struct Activities {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of the resource."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Each activity record in the response."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Activity>>>,
    #[serde(rename = "kind")]
    #[serde(default = "activities_defaults :: kind")]
    #[doc = "The type of API resource. For an activity report, the value is `reports#activities`."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token for retrieving the follow-on next page of the report. The `nextPageToken` value is used in the request's `pageToken` query string."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
mod activities_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("admin#reports#activities")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "JSON template for the activity resource."]
pub struct Activity {
    #[serde(rename = "actor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User doing the action."]
    pub actor: ::std::option::Option<ActivityActor>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of the entry."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Activity events in the report."]
    pub events: ::std::option::Option<::std::vec::Vec<ActivityEvents>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier for each activity record."]
    pub id: ::std::option::Option<ActivityId>,
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "IP address of the user doing the action. This is the Internet Protocol (IP) address of the user when logging into G Suite which may or may not reflect the user's physical location. For example, the IP address can be the user's proxy server's address or a virtual private network (VPN) address. The API supports IPv4 and IPv6."]
    pub ip_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "activity_defaults :: kind")]
    #[doc = "The type of API resource. For an activity report, the value is `audit#activity`."]
    pub kind: ::std::string::String,
    #[serde(rename = "ownerDomain")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This is the domain that is affected by the report's event. For example domain of Admin console or the Drive application's document owner."]
    pub owner_domain: ::std::option::Option<::std::string::String>,
}
mod activity_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("admin#reports#activity")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "User doing the action."]
pub struct ActivityActor {
    #[serde(rename = "callerType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of actor."]
    pub caller_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The primary email address of the actor. May be absent if there is no email address associated with the actor."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only present when `callerType` is `KEY`. Can be the `consumer_key` of the requestor for OAuth 2LO API requests or an identifier for robot accounts."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "profileId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique G Suite profile ID of the actor. May be absent if the actor is not a G Suite user."]
    pub profile_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActivityEvents {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the event. This is the specific name of the activity reported by the API. And each `eventName` is related to a specific G Suite service or feature which the API organizes into types of events. For `eventName` request parameters in general: - If no `eventName` is given, the report returns all possible instances of an `eventName`. - When you request an `eventName`, the API's response returns all activities which contain that `eventName`. It is possible that the returned activities will have other `eventName` properties in addition to the one requested. For more information about `eventName` properties, see the list of event names for various applications above in `applicationName`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parameter value pairs for various applications. For more information about `eventName` parameters, see the list of event names for various applications above in `applicationName`."]
    pub parameters: ::std::option::Option<::std::vec::Vec<ActivityEventsParameters>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of event. The G Suite service or feature that an administrator changes is identified in the `type` property which identifies an event using the `eventName` property. For a full list of the API's `type` categories, see the list of event names for various applications above in `applicationName`."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActivityEventsParameters {
    #[serde(rename = "boolValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Boolean value of the parameter."]
    pub bool_value: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "intValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Integer value of the parameter."]
    pub int_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "messageValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Nested parameter value pairs associated with this parameter. Complex value type for a parameter are returned as a list of parameter values. For example, the address parameter may have a value as `[{parameter: [{name: city, value: abc}]}]`"]
    pub message_value: ::std::option::Option<ActivityEventsParametersMessageValue>,
    #[serde(rename = "multiIntValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Integer values of the parameter."]
    pub multi_int_value: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "multiMessageValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of `messageValue` objects."]
    pub multi_message_value:
        ::std::option::Option<::std::vec::Vec<ActivityEventsParametersMultiMessageValue>>,
    #[serde(rename = "multiValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "String values of the parameter."]
    pub multi_value: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the parameter."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "String value of the parameter."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Nested parameter value pairs associated with this parameter. Complex value type for a parameter are returned as a list of parameter values. For example, the address parameter may have a value as `[{parameter: [{name: city, value: abc}]}]`"]
pub struct ActivityEventsParametersMessageValue {
    #[serde(rename = "parameter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parameter values"]
    pub parameter: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NestedParameter>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActivityEventsParametersMultiMessageValue {
    #[serde(rename = "parameter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parameter values"]
    pub parameter: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NestedParameter>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Unique identifier for each activity record."]
pub struct ActivityId {
    #[serde(rename = "applicationName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Application name to which the event belongs. For possible values see the list of applications above in `applicationName`."]
    pub application_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier for a G suite account."]
    pub customer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "time")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time of occurrence of the activity. This is in UNIX epoch time in seconds."]
    pub time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uniqueQualifier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique qualifier if multiple events have the same time."]
    pub unique_qualifier: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A notification channel used to watch for resource changes."]
pub struct Channel {
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The address where notifications are delivered for this channel."]
    pub address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expiration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional."]
    pub expiration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A UUID or similar unique string that identifies this channel."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "channel_defaults :: kind")]
    #[doc = "Identifies this as a notification channel used to watch for changes to a resource, which is \"`api#channel`\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "params")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional parameters controlling delivery channel behavior. Optional."]
    pub params: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A Boolean value to indicate whether payload is wanted. Optional."]
    pub payload: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An opaque ID that identifies the resource being watched on this channel. Stable across different API versions."]
    pub resource_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A version-specific identifier for the watched resource."]
    pub resource_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An arbitrary string delivered to the target address with each notification delivered over this channel. Optional."]
    pub token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of delivery mechanism used for this channel. The value should be set to `\"web_hook\"`."]
    pub _type: ::std::option::Option<::std::string::String>,
}
mod channel_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("api#channel")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "JSON template for a parameter used in various reports."]
pub struct NestedParameter {
    #[serde(rename = "boolValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Boolean value of the parameter."]
    pub bool_value: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "intValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Integer value of the parameter."]
    pub int_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "multiBoolValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Multiple boolean values of the parameter."]
    pub multi_bool_value: ::std::option::Option<::std::vec::Vec<::std::primitive::bool>>,
    #[serde(rename = "multiIntValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Multiple integer values of the parameter."]
    pub multi_int_value: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "multiValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Multiple string values of the parameter."]
    pub multi_value: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the parameter."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "String value of the parameter."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "JSON template for a usage report."]
pub struct UsageReport {
    #[serde(rename = "date")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The date of the report request."]
    pub date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Information about the type of the item."]
    pub entity: ::std::option::Option<UsageReportEntity>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of the resource."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "usage_report_defaults :: kind")]
    #[doc = "The type of API resource. For a usage report, the value is `admin#reports#usageReport`."]
    pub kind: ::std::string::String,
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Parameter value pairs for various applications. For the Entity Usage Report parameters and values, see [the Entity Usage parameters reference](/admin-sdk/reports/v1/reference/usage-ref-appendix-a/entities)."]
    pub parameters: ::std::option::Option<::std::vec::Vec<UsageReportParameters>>,
}
mod usage_report_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("admin#reports#usageReport")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Output only. Information about the type of the item."]
pub struct UsageReportEntity {
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique identifier of the customer's account."]
    pub customer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Object key. Only relevant if entity.type = \"OBJECT\" Note: external-facing name of report is \"Entities\" rather than \"Objects\"."]
    pub entity_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "profileId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The user's immutable G Suite profile identifier."]
    pub profile_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of item. The value is `user`."]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The user's email address. Only relevant if entity.type = \"USER\""]
    pub user_email: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsageReportParameters {
    #[serde(rename = "boolValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Boolean value of the parameter."]
    pub bool_value: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "datetimeValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The RFC 3339 formatted value of the parameter, for example 2010-10-28T10:26:35.000Z."]
    pub datetime_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "intValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Integer value of the parameter."]
    pub int_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "msgValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Nested message value of the parameter."]
    pub msg_value: ::std::option::Option<
        ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    >,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the parameter. For the User Usage Report parameter names, see the User Usage parameters reference."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "stringValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. String value of the parameter."]
    pub string_value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsageReports {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of the resource."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "usage_reports_defaults :: kind")]
    #[doc = "The type of API resource. For a usage report, the value is `admin#reports#usageReports`."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to specify next page. A report with multiple pages has a `nextPageToken` property in the response. For your follow-on requests getting all of the report's pages, enter the `nextPageToken` value in the `pageToken` query string."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "usageReports")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Various application parameter records."]
    pub usage_reports: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UsageReport>>>,
    #[serde(rename = "warnings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Warnings, if any."]
    pub warnings: ::std::option::Option<::std::vec::Vec<UsageReportsWarnings>>,
}
mod usage_reports_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("admin#reports#usageReports")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsageReportsWarnings {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Machine readable code or warning type. The warning code value is `200`."]
    pub code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Key-value pairs to give detailed information on the warning."]
    pub data: ::std::option::Option<::std::vec::Vec<UsageReportsWarningsData>>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The human readable messages for a warning are: - Data is not available warning - Sorry, data for date yyyy-mm-dd for application \"`application name`\" is not available. - Partial data is available warning - Data for date yyyy-mm-dd for application \"`application name`\" is not available right now, please try again after a few hours. "]
    pub message: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UsageReportsWarningsData {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Key associated with a key-value pair to give detailed information on the warning."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Value associated with a key-value pair to give detailed information on the warning."]
    pub value: ::std::option::Option<::std::string::String>,
}
