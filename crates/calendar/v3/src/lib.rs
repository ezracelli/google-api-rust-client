#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Acl {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of the collection."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of rules on the access control list."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AclRule>>>,
    #[serde(rename = "kind")]
    #[serde(default = "acl_defaults :: kind")]
    #[doc = "Type of the collection (\"calendar#acl\")."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token used to access the next page of this result. Omitted if no further results are available, in which case nextSyncToken is provided."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextSyncToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token used at a later point in time to retrieve only the entries that have changed since this result was returned. Omitted if further results are available, in which case nextPageToken is provided."]
    pub next_sync_token: ::std::option::Option<::std::string::String>,
}
mod acl_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("calendar#acl")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AclRule {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of the resource."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier of the ACL rule."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "acl_rule_defaults :: kind")]
    #[doc = "Type of the resource (\"calendar#aclRule\")."]
    pub kind: ::std::string::String,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The role assigned to the scope. Possible values are:  \n- \"none\" - Provides no access. \n- \"freeBusyReader\" - Provides read access to free/busy information. \n- \"reader\" - Provides read access to the calendar. Private events will appear to users with reader access, but event details will be hidden. \n- \"writer\" - Provides read and write access to the calendar. Private events will appear to users with writer access, and event details will be visible. \n- \"owner\" - Provides ownership of the calendar. This role has all of the permissions of the writer role with the additional ability to see and manipulate ACLs."]
    pub role: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scope")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The scope of the rule."]
    pub scope: ::std::option::Option<AclRuleScope>,
}
mod acl_rule_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("calendar#aclRule")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The scope of the rule."]
pub struct AclRuleScope {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the scope. Possible values are:  \n- \"default\" - The public scope. This is the default value. \n- \"user\" - Limits the scope to a single user. \n- \"group\" - Limits the scope to a group. \n- \"domain\" - Limits the scope to a domain.  Note: The permissions granted to the \"default\", or public, scope apply to any user, authenticated or not."]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email address of a user or group, or the name of a domain, depending on the scope type. Omitted for type \"default\"."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Calendar {
    #[serde(rename = "conferenceProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Conferencing properties for this calendar, for example what types of conferences are allowed."]
    pub conference_properties: ::std::option::Option<::std::boxed::Box<ConferenceProperties>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the calendar. Optional."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of the resource."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier of the calendar. To retrieve IDs call the calendarList.list() method."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "calendar_defaults :: kind")]
    #[doc = "Type of the resource (\"calendar#calendar\")."]
    pub kind: ::std::string::String,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Geographic location of the calendar as free-form text. Optional."]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title of the calendar."]
    pub summary: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time zone of the calendar. (Formatted as an IANA Time Zone Database name, e.g. \"Europe/Zurich\".) Optional."]
    pub time_zone: ::std::option::Option<::std::string::String>,
}
mod calendar_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("calendar#calendar")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CalendarList {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of the collection."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Calendars that are present on the user's calendar list."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CalendarListEntry>>>,
    #[serde(rename = "kind")]
    #[serde(default = "calendar_list_defaults :: kind")]
    #[doc = "Type of the collection (\"calendar#calendarList\")."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token used to access the next page of this result. Omitted if no further results are available, in which case nextSyncToken is provided."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextSyncToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token used at a later point in time to retrieve only the entries that have changed since this result was returned. Omitted if further results are available, in which case nextPageToken is provided."]
    pub next_sync_token: ::std::option::Option<::std::string::String>,
}
mod calendar_list_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("calendar#calendarList")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CalendarListEntry {
    #[serde(rename = "accessRole")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The effective access role that the authenticated user has on the calendar. Read-only. Possible values are:  \n- \"freeBusyReader\" - Provides read access to free/busy information. \n- \"reader\" - Provides read access to the calendar. Private events will appear to users with reader access, but event details will be hidden. \n- \"writer\" - Provides read and write access to the calendar. Private events will appear to users with writer access, and event details will be visible. \n- \"owner\" - Provides ownership of the calendar. This role has all of the permissions of the writer role with the additional ability to see and manipulate ACLs."]
    pub access_role: ::std::option::Option<::std::string::String>,
    #[serde(rename = "backgroundColor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The main color of the calendar in the hexadecimal format \"#0088aa\". This property supersedes the index-based colorId property. To set or change this property, you need to specify colorRgbFormat=true in the parameters of the insert, update and patch methods. Optional."]
    pub background_color: ::std::option::Option<::std::string::String>,
    #[serde(rename = "colorId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The color of the calendar. This is an ID referring to an entry in the calendar section of the colors definition (see the colors endpoint). This property is superseded by the backgroundColor and foregroundColor properties and can be ignored when using these properties. Optional."]
    pub color_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "conferenceProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Conferencing properties for this calendar, for example what types of conferences are allowed."]
    pub conference_properties: ::std::option::Option<::std::boxed::Box<ConferenceProperties>>,
    #[serde(rename = "defaultReminders")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The default reminders that the authenticated user has for this calendar."]
    pub default_reminders: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventReminder>>>,
    #[serde(rename = "deleted")]
    #[serde(default = "calendar_list_entry_defaults :: deleted")]
    #[doc = "Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False."]
    pub deleted: ::std::primitive::bool,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the calendar. Optional. Read-only."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of the resource."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "foregroundColor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The foreground color of the calendar in the hexadecimal format \"#ffffff\". This property supersedes the index-based colorId property. To set or change this property, you need to specify colorRgbFormat=true in the parameters of the insert, update and patch methods. Optional."]
    pub foreground_color: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hidden")]
    #[serde(default = "calendar_list_entry_defaults :: hidden")]
    #[doc = "Whether the calendar has been hidden from the list. Optional. The attribute is only returned when the calendar is hidden, in which case the value is true."]
    pub hidden: ::std::primitive::bool,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier of the calendar."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "calendar_list_entry_defaults :: kind")]
    #[doc = "Type of the resource (\"calendar#calendarListEntry\")."]
    pub kind: ::std::string::String,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Geographic location of the calendar as free-form text. Optional. Read-only."]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notificationSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The notifications that the authenticated user is receiving for this calendar."]
    pub notification_settings: ::std::option::Option<CalendarListEntryNotificationSettings>,
    #[serde(rename = "primary")]
    #[serde(default = "calendar_list_entry_defaults :: primary")]
    #[doc = "Whether the calendar is the primary calendar of the authenticated user. Read-only. Optional. The default is False."]
    pub primary: ::std::primitive::bool,
    #[serde(rename = "selected")]
    #[serde(default = "calendar_list_entry_defaults :: selected")]
    #[doc = "Whether the calendar content shows up in the calendar UI. Optional. The default is False."]
    pub selected: ::std::primitive::bool,
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title of the calendar. Read-only."]
    pub summary: ::std::option::Option<::std::string::String>,
    #[serde(rename = "summaryOverride")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The summary that the authenticated user has set for this calendar. Optional."]
    pub summary_override: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time zone of the calendar. Optional. Read-only."]
    pub time_zone: ::std::option::Option<::std::string::String>,
}
mod calendar_list_entry_defaults {
    pub fn deleted() -> ::std::primitive::bool {
        serde_json::from_str(&"false").unwrap()
    }
    pub fn hidden() -> ::std::primitive::bool {
        serde_json::from_str(&"false").unwrap()
    }
    pub fn kind() -> ::std::string::String {
        String::from("calendar#calendarListEntry")
    }
    pub fn primary() -> ::std::primitive::bool {
        serde_json::from_str(&"false").unwrap()
    }
    pub fn selected() -> ::std::primitive::bool {
        serde_json::from_str(&"false").unwrap()
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The notifications that the authenticated user is receiving for this calendar."]
pub struct CalendarListEntryNotificationSettings {
    #[serde(rename = "notifications")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of notifications set for this calendar."]
    pub notifications:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CalendarNotification>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CalendarNotification {
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The method used to deliver the notification. The possible value is:  \n- \"email\" - Notifications are sent via email.  \nRequired when adding a notification."]
    pub method: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of notification. Possible values are:  \n- \"eventCreation\" - Notification sent when a new event is put on the calendar. \n- \"eventChange\" - Notification sent when an event is changed. \n- \"eventCancellation\" - Notification sent when an event is cancelled. \n- \"eventResponse\" - Notification sent when an attendee responds to the event invitation. \n- \"agenda\" - An agenda with the events of the day (sent out in the morning).  \nRequired when adding a notification."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
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
    #[doc = "Identifies this as a notification channel used to watch for changes to a resource, which is \"api#channel\"."]
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
    #[doc = "The type of delivery mechanism used for this channel. Valid values are \"web_hook\" (or \"webhook\"). Both values refer to a channel where Http requests are used to deliver messages."]
    pub _type: ::std::option::Option<::std::string::String>,
}
mod channel_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("api#channel")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ColorDefinition {
    #[serde(rename = "background")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The background color associated with this color definition."]
    pub background: ::std::option::Option<::std::string::String>,
    #[serde(rename = "foreground")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The foreground color that can be used to write on top of a background with 'background' color."]
    pub foreground: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Colors {
    #[serde(rename = "calendar")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A global palette of calendar colors, mapping from the color ID to its definition. A calendarListEntry resource refers to one of these color IDs in its color field. Read-only."]
    pub calendar: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<ColorDefinition>>,
    >,
    #[serde(rename = "event")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A global palette of event colors, mapping from the color ID to its definition. An event resource may refer to one of these color IDs in its color field. Read-only."]
    pub event: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<ColorDefinition>>,
    >,
    #[serde(rename = "kind")]
    #[serde(default = "colors_defaults :: kind")]
    #[doc = "Type of the resource (\"calendar#colors\")."]
    pub kind: ::std::string::String,
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Last modification time of the color palette (as a RFC3339 timestamp). Read-only."]
    pub updated: ::std::option::Option<::std::string::String>,
}
mod colors_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("calendar#colors")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConferenceData {
    #[serde(rename = "conferenceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the conference.\nCan be used by developers to keep track of conferences, should not be displayed to users.\nValues for solution types:  \n- \"eventHangout\": unset.\n- \"eventNamedHangout\": the name of the Hangout.\n- \"hangoutsMeet\": the 10-letter meeting code, for example \"aaa-bbbb-ccc\".\n- \"addOn\": defined by 3P conference provider.  Optional."]
    pub conference_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "conferenceSolution")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The conference solution, such as Hangouts or Google Meet.\nUnset for a conference with a failed create request.\nEither conferenceSolution and at least one entryPoint, or createRequest is required."]
    pub conference_solution: ::std::option::Option<::std::boxed::Box<ConferenceSolution>>,
    #[serde(rename = "createRequest")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A request to generate a new conference and attach it to the event. The data is generated asynchronously. To see whether the data is present check the status field.\nEither conferenceSolution and at least one entryPoint, or createRequest is required."]
    pub create_request: ::std::option::Option<::std::boxed::Box<CreateConferenceRequest>>,
    #[serde(rename = "entryPoints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about individual conference entry points, such as URLs or phone numbers.\nAll of them must belong to the same conference.\nEither conferenceSolution and at least one entryPoint, or createRequest is required."]
    pub entry_points: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EntryPoint>>>,
    #[serde(rename = "notes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional notes (such as instructions from the domain administrator, legal notices) to display to the user. Can contain HTML. The maximum length is 2048 characters. Optional."]
    pub notes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional properties related to a conference. An example would be a solution-specific setting for enabling video streaming."]
    pub parameters: ::std::option::Option<::std::boxed::Box<ConferenceParameters>>,
    #[serde(rename = "signature")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The signature of the conference data.\nGenerated on server side. Must be preserved while copying the conference data between events, otherwise the conference data will not be copied.\nUnset for a conference with a failed create request.\nOptional for a conference with a pending create request."]
    pub signature: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConferenceParameters {
    #[serde(rename = "addOnParameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional add-on specific data."]
    pub add_on_parameters:
        ::std::option::Option<::std::boxed::Box<ConferenceParametersAddOnParameters>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConferenceParametersAddOnParameters {
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub parameters:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConferenceProperties {
    #[serde(rename = "allowedConferenceSolutionTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The types of conference solutions that are supported for this calendar.\nThe possible values are:  \n- \"eventHangout\" \n- \"eventNamedHangout\" \n- \"hangoutsMeet\"  Optional."]
    pub allowed_conference_solution_types:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConferenceRequestStatus {
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current status of the conference create request. Read-only.\nThe possible values are:  \n- \"pending\": the conference create request is still being processed.\n- \"success\": the conference create request succeeded, the entry points are populated.\n- \"failure\": the conference create request failed, there are no entry points."]
    pub status_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConferenceSolution {
    #[serde(rename = "iconUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user-visible icon for this solution."]
    pub icon_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The key which can uniquely identify the conference solution for this event."]
    pub key: ::std::option::Option<::std::boxed::Box<ConferenceSolutionKey>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user-visible name of this solution. Not localized."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConferenceSolutionKey {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The conference solution type.\nIf a client encounters an unfamiliar or empty type, it should still be able to display the entry points. However, it should disallow modifications.\nThe possible values are:  \n- \"eventHangout\" for Hangouts for consumers (http://hangouts.google.com)\n- \"eventNamedHangout\" for classic Hangouts for Google Workspace users (http://hangouts.google.com)\n- \"hangoutsMeet\" for Google Meet (http://meet.google.com)\n- \"addOn\" for 3P conference providers"]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateConferenceRequest {
    #[serde(rename = "conferenceSolutionKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The conference solution, such as Hangouts or Google Meet."]
    pub conference_solution_key: ::std::option::Option<::std::boxed::Box<ConferenceSolutionKey>>,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The client-generated unique ID for this request.\nClients should regenerate this ID for every new request. If an ID provided is the same as for the previous request, the request is ignored."]
    pub request_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the conference create request."]
    pub status: ::std::option::Option<::std::boxed::Box<ConferenceRequestStatus>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntryPoint {
    #[serde(rename = "accessCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The access code to access the conference. The maximum length is 128 characters.\nWhen creating new conference data, populate only the subset of {meetingCode, accessCode, passcode, password, pin} fields that match the terminology that the conference provider uses. Only the populated fields should be displayed.\nOptional."]
    pub access_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entryPointFeatures")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Features of the entry point, such as being toll or toll-free. One entry point can have multiple features. However, toll and toll-free cannot be both set on the same entry point."]
    pub entry_point_features: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "entryPointType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the conference entry point.\nPossible values are:  \n- \"video\" - joining a conference over HTTP. A conference can have zero or one video entry point.\n- \"phone\" - joining a conference by dialing a phone number. A conference can have zero or more phone entry points.\n- \"sip\" - joining a conference over SIP. A conference can have zero or one sip entry point.\n- \"more\" - further conference joining instructions, for example additional phone numbers. A conference can have zero or one more entry point. A conference with only a more entry point is not a valid conference."]
    pub entry_point_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The label for the URI. Visible to end users. Not localized. The maximum length is 512 characters.\nExamples:  \n- for video: meet.google.com/aaa-bbbb-ccc\n- for phone: +1 123 268 2601\n- for sip: 12345678@altostrat.com\n- for more: should not be filled  \nOptional."]
    pub label: ::std::option::Option<::std::string::String>,
    #[serde(rename = "meetingCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The meeting code to access the conference. The maximum length is 128 characters.\nWhen creating new conference data, populate only the subset of {meetingCode, accessCode, passcode, password, pin} fields that match the terminology that the conference provider uses. Only the populated fields should be displayed.\nOptional."]
    pub meeting_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "passcode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The passcode to access the conference. The maximum length is 128 characters.\nWhen creating new conference data, populate only the subset of {meetingCode, accessCode, passcode, password, pin} fields that match the terminology that the conference provider uses. Only the populated fields should be displayed."]
    pub passcode: ::std::option::Option<::std::string::String>,
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The password to access the conference. The maximum length is 128 characters.\nWhen creating new conference data, populate only the subset of {meetingCode, accessCode, passcode, password, pin} fields that match the terminology that the conference provider uses. Only the populated fields should be displayed.\nOptional."]
    pub password: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pin")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The PIN to access the conference. The maximum length is 128 characters.\nWhen creating new conference data, populate only the subset of {meetingCode, accessCode, passcode, password, pin} fields that match the terminology that the conference provider uses. Only the populated fields should be displayed.\nOptional."]
    pub pin: ::std::option::Option<::std::string::String>,
    #[serde(rename = "regionCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The CLDR/ISO 3166 region code for the country associated with this phone access. Example: \"SE\" for Sweden.\nCalendar backend will populate this field only for EntryPointType.PHONE."]
    pub region_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI of the entry point. The maximum length is 1300 characters.\nFormat:  \n- for video, http: or https: schema is required.\n- for phone, tel: schema is required. The URI should include the entire dial sequence (e.g., tel:+12345678900,,,123456789;1234).\n- for sip, sip: schema is required, e.g., sip:12345678@myprovider.com.\n- for more, http: or https: schema is required."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Error {
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Domain, or broad category, of the error."]
    pub domain: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specific reason for the error. Some of the possible values are:  \n- \"groupTooBig\" - The group of users requested is too large for a single query. \n- \"tooManyCalendarsRequested\" - The number of calendars requested is too large for a single query. \n- \"notFound\" - The requested resource was not found. \n- \"internalError\" - The API service has encountered an internal error.  Additional error types may be added in the future, so clients should gracefully handle additional error statuses not included in this list."]
    pub reason: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event {
    #[serde(rename = "anyoneCanAddSelf")]
    #[serde(default = "event_defaults :: anyone_can_add_self")]
    #[doc = "Whether anyone can invite themselves to the event (currently works for Google+ events only). Optional. The default is False."]
    pub anyone_can_add_self: ::std::primitive::bool,
    #[serde(rename = "attachments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "File attachments for the event. Currently only Google Drive attachments are supported.\nIn order to modify attachments the supportsAttachments request parameter should be set to true.\nThere can be at most 25 attachments per event,"]
    pub attachments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventAttachment>>>,
    #[serde(rename = "attendees")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The attendees of the event. See the Events with attendees guide for more information on scheduling events with other calendar users. Service accounts need to use domain-wide delegation of authority to populate the attendee list."]
    pub attendees: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventAttendee>>>,
    #[serde(rename = "attendeesOmitted")]
    #[serde(default = "event_defaults :: attendees_omitted")]
    #[doc = "Whether attendees may have been omitted from the event's representation. When retrieving an event, this may be due to a restriction specified by the maxAttendee query parameter. When updating an event, this can be used to only update the participant's response. Optional. The default is False."]
    pub attendees_omitted: ::std::primitive::bool,
    #[serde(rename = "colorId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The color of the event. This is an ID referring to an entry in the event section of the colors definition (see the  colors endpoint). Optional."]
    pub color_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "conferenceData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The conference-related information, such as details of a Google Meet conference. To create new conference details use the createRequest field. To persist your changes, remember to set the conferenceDataVersion request parameter to 1 for all event modification requests."]
    pub conference_data: ::std::option::Option<::std::boxed::Box<ConferenceData>>,
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creation time of the event (as a RFC3339 timestamp). Read-only."]
    pub created: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creator")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The creator of the event. Read-only."]
    pub creator: ::std::option::Option<EventCreator>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the event. Can contain HTML. Optional."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "end")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The (exclusive) end time of the event. For a recurring event, this is the end time of the first instance."]
    pub end: ::std::option::Option<::std::boxed::Box<EventDateTime>>,
    #[serde(rename = "endTimeUnspecified")]
    #[serde(default = "event_defaults :: end_time_unspecified")]
    #[doc = "Whether the end time is actually unspecified. An end time is still provided for compatibility reasons, even if this attribute is set to True. The default is False."]
    pub end_time_unspecified: ::std::primitive::bool,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of the resource."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "eventType")]
    #[serde(default = "event_defaults :: event_type")]
    #[doc = "Specific type of the event. Read-only. Possible values are:  \n- \"default\" - A regular event or not further specified. \n- \"outOfOffice\" - An out-of-office event."]
    pub event_type: ::std::string::String,
    #[serde(rename = "extendedProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Extended properties of the event."]
    pub extended_properties: ::std::option::Option<EventExtendedProperties>,
    #[serde(rename = "gadget")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata."]
    pub gadget: ::std::option::Option<EventGadget>,
    #[serde(rename = "guestsCanInviteOthers")]
    #[serde(default = "event_defaults :: guests_can_invite_others")]
    #[doc = "Whether attendees other than the organizer can invite others to the event. Optional. The default is True."]
    pub guests_can_invite_others: ::std::primitive::bool,
    #[serde(rename = "guestsCanModify")]
    #[serde(default = "event_defaults :: guests_can_modify")]
    #[doc = "Whether attendees other than the organizer can modify the event. Optional. The default is False."]
    pub guests_can_modify: ::std::primitive::bool,
    #[serde(rename = "guestsCanSeeOtherGuests")]
    #[serde(default = "event_defaults :: guests_can_see_other_guests")]
    #[doc = "Whether attendees other than the organizer can see who the event's attendees are. Optional. The default is True."]
    pub guests_can_see_other_guests: ::std::primitive::bool,
    #[serde(rename = "hangoutLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An absolute link to the Google+ hangout associated with this event. Read-only."]
    pub hangout_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "htmlLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An absolute link to this event in the Google Calendar Web UI. Read-only."]
    pub html_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "iCalUID")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Event unique identifier as defined in RFC5545. It is used to uniquely identify events accross calendaring systems and must be supplied when importing events via the import method.\nNote that the icalUID and the id are not identical and only one of them should be supplied at event creation time. One difference in their semantics is that in recurring events, all occurrences of one event have different ids while they all share the same icalUIDs."]
    pub i_cal_uid: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Opaque identifier of the event. When creating new single or recurring events, you can specify their IDs. Provided IDs must follow these rules:  \n- characters allowed in the ID are those used in base32hex encoding, i.e. lowercase letters a-v and digits 0-9, see section 3.1.2 in RFC2938 \n- the length of the ID must be between 5 and 1024 characters \n- the ID must be unique per calendar  Due to the globally distributed nature of the system, we cannot guarantee that ID collisions will be detected at event creation time. To minimize the risk of collisions we recommend using an established UUID algorithm such as one described in RFC4122.\nIf you do not specify an ID, it will be automatically generated by the server.\nNote that the icalUID and the id are not identical and only one of them should be supplied at event creation time. One difference in their semantics is that in recurring events, all occurrences of one event have different ids while they all share the same icalUIDs."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "event_defaults :: kind")]
    #[doc = "Type of the resource (\"calendar#event\")."]
    pub kind: ::std::string::String,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Geographic location of the event as free-form text. Optional."]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locked")]
    #[serde(default = "event_defaults :: locked")]
    #[doc = "Whether this is a locked event copy where no changes can be made to the main event fields \"summary\", \"description\", \"location\", \"start\", \"end\" or \"recurrence\". The default is False. Read-Only."]
    pub locked: ::std::primitive::bool,
    #[serde(rename = "organizer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The organizer of the event. If the organizer is also an attendee, this is indicated with a separate entry in attendees with the organizer field set to True. To change the organizer, use the move operation. Read-only, except when importing an event."]
    pub organizer: ::std::option::Option<EventOrganizer>,
    #[serde(rename = "originalStartTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For an instance of a recurring event, this is the time at which this event would start according to the recurrence data in the recurring event identified by recurringEventId. It uniquely identifies the instance within the recurring event series even if the instance was moved to a different time. Immutable."]
    pub original_start_time: ::std::option::Option<::std::boxed::Box<EventDateTime>>,
    #[serde(rename = "privateCopy")]
    #[serde(default = "event_defaults :: private_copy")]
    #[doc = "If set to True, Event propagation is disabled. Note that it is not the same thing as Private event properties. Optional. Immutable. The default is False."]
    pub private_copy: ::std::primitive::bool,
    #[serde(rename = "recurrence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of RRULE, EXRULE, RDATE and EXDATE lines for a recurring event, as specified in RFC5545. Note that DTSTART and DTEND lines are not allowed in this field; event start and end times are specified in the start and end fields. This field is omitted for single events or instances of recurring events."]
    pub recurrence: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "recurringEventId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For an instance of a recurring event, this is the id of the recurring event to which this instance belongs. Immutable."]
    pub recurring_event_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reminders")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the event's reminders for the authenticated user."]
    pub reminders: ::std::option::Option<EventReminders>,
    #[serde(rename = "sequence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sequence number as per iCalendar."]
    pub sequence: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Source from which the event was created. For example, a web page, an email message or any document identifiable by an URL with HTTP or HTTPS scheme. Can only be seen or modified by the creator of the event."]
    pub source: ::std::option::Option<EventSource>,
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The (inclusive) start time of the event. For a recurring event, this is the start time of the first instance."]
    pub start: ::std::option::Option<::std::boxed::Box<EventDateTime>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the event. Optional. Possible values are:  \n- \"confirmed\" - The event is confirmed. This is the default status. \n- \"tentative\" - The event is tentatively confirmed. \n- \"cancelled\" - The event is cancelled (deleted). The list method returns cancelled events only on incremental sync (when syncToken or updatedMin are specified) or if the showDeleted flag is set to true. The get method always returns them.\nA cancelled status represents two different states depending on the event type:  \n- Cancelled exceptions of an uncancelled recurring event indicate that this instance should no longer be presented to the user. Clients should store these events for the lifetime of the parent recurring event.\nCancelled exceptions are only guaranteed to have values for the id, recurringEventId and originalStartTime fields populated. The other fields might be empty.  \n- All other cancelled events represent deleted events. Clients should remove their locally synced copies. Such cancelled events will eventually disappear, so do not rely on them being available indefinitely.\nDeleted events are only guaranteed to have the id field populated.   On the organizer's calendar, cancelled events continue to expose event details (summary, location, etc.) so that they can be restored (undeleted). Similarly, the events to which the user was invited and that they manually removed continue to provide details. However, incremental sync requests with showDeleted set to false will not return these details.\nIf an event changes its organizer (for example via the move operation) and the original organizer is not on the attendee list, it will leave behind a cancelled event where only the id field is guaranteed to be populated."]
    pub status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title of the event."]
    pub summary: ::std::option::Option<::std::string::String>,
    #[serde(rename = "transparency")]
    #[serde(default = "event_defaults :: transparency")]
    #[doc = "Whether the event blocks time on the calendar. Optional. Possible values are:  \n- \"opaque\" - Default value. The event does block time on the calendar. This is equivalent to setting Show me as to Busy in the Calendar UI. \n- \"transparent\" - The event does not block time on the calendar. This is equivalent to setting Show me as to Available in the Calendar UI."]
    pub transparency: ::std::string::String,
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Last modification time of the event (as a RFC3339 timestamp). Read-only."]
    pub updated: ::std::option::Option<::std::string::String>,
    #[serde(rename = "visibility")]
    #[serde(default = "event_defaults :: visibility")]
    #[doc = "Visibility of the event. Optional. Possible values are:  \n- \"default\" - Uses the default visibility for events on the calendar. This is the default value. \n- \"public\" - The event is public and event details are visible to all readers of the calendar. \n- \"private\" - The event is private and only event attendees may view event details. \n- \"confidential\" - The event is private. This value is provided for compatibility reasons."]
    pub visibility: ::std::string::String,
}
mod event_defaults {
    pub fn anyone_can_add_self() -> ::std::primitive::bool {
        serde_json::from_str(&"false").unwrap()
    }
    pub fn attendees_omitted() -> ::std::primitive::bool {
        serde_json::from_str(&"false").unwrap()
    }
    pub fn end_time_unspecified() -> ::std::primitive::bool {
        serde_json::from_str(&"false").unwrap()
    }
    pub fn event_type() -> ::std::string::String {
        String::from("default")
    }
    pub fn guests_can_invite_others() -> ::std::primitive::bool {
        serde_json::from_str(&"true").unwrap()
    }
    pub fn guests_can_modify() -> ::std::primitive::bool {
        serde_json::from_str(&"false").unwrap()
    }
    pub fn guests_can_see_other_guests() -> ::std::primitive::bool {
        serde_json::from_str(&"true").unwrap()
    }
    pub fn kind() -> ::std::string::String {
        String::from("calendar#event")
    }
    pub fn locked() -> ::std::primitive::bool {
        serde_json::from_str(&"false").unwrap()
    }
    pub fn private_copy() -> ::std::primitive::bool {
        serde_json::from_str(&"false").unwrap()
    }
    pub fn transparency() -> ::std::string::String {
        String::from("opaque")
    }
    pub fn visibility() -> ::std::string::String {
        String::from("default")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The creator of the event. Read-only."]
pub struct EventCreator {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The creator's name, if available."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The creator's email address, if available."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The creator's Profile ID, if available. It corresponds to the id field in the People collection of the Google+ API"]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "self")]
    #[serde(default = "event_creator_defaults :: _self")]
    #[doc = "Whether the creator corresponds to the calendar on which this copy of the event appears. Read-only. The default is False."]
    pub _self: ::std::primitive::bool,
}
mod event_creator_defaults {
    pub fn _self() -> ::std::primitive::bool {
        serde_json::from_str(&"false").unwrap()
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Extended properties of the event."]
pub struct EventExtendedProperties {
    #[serde(rename = "private")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Properties that are private to the copy of the event that appears on this calendar."]
    pub private: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "shared")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Properties that are shared between copies of the event on other attendees' calendars."]
    pub shared: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata."]
pub struct EventGadget {
    #[serde(rename = "display")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The gadget's display mode. Deprecated. Possible values are:  \n- \"icon\" - The gadget displays next to the event's title in the calendar view. \n- \"chip\" - The gadget displays when the event is clicked."]
    pub display: ::std::option::Option<::std::string::String>,
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The gadget's height in pixels. The height must be an integer greater than 0. Optional. Deprecated."]
    pub height: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "iconLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The gadget's icon URL. The URL scheme must be HTTPS. Deprecated."]
    pub icon_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "link")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The gadget's URL. The URL scheme must be HTTPS. Deprecated."]
    pub link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "preferences")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Preferences."]
    pub preferences:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The gadget's title. Deprecated."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The gadget's type. Deprecated."]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The gadget's width in pixels. The width must be an integer greater than 0. Optional. Deprecated."]
    pub width: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The organizer of the event. If the organizer is also an attendee, this is indicated with a separate entry in attendees with the organizer field set to True. To change the organizer, use the move operation. Read-only, except when importing an event."]
pub struct EventOrganizer {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The organizer's name, if available."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The organizer's email address, if available. It must be a valid email address as per RFC5322."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The organizer's Profile ID, if available. It corresponds to the id field in the People collection of the Google+ API"]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "self")]
    #[serde(default = "event_organizer_defaults :: _self")]
    #[doc = "Whether the organizer corresponds to the calendar on which this copy of the event appears. Read-only. The default is False."]
    pub _self: ::std::primitive::bool,
}
mod event_organizer_defaults {
    pub fn _self() -> ::std::primitive::bool {
        serde_json::from_str(&"false").unwrap()
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about the event's reminders for the authenticated user."]
pub struct EventReminders {
    #[serde(rename = "overrides")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the event doesn't use the default reminders, this lists the reminders specific to the event, or, if not set, indicates that no reminders are set for this event. The maximum number of override reminders is 5."]
    pub overrides: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventReminder>>>,
    #[serde(rename = "useDefault")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the default reminders of the calendar apply to the event."]
    pub use_default: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Source from which the event was created. For example, a web page, an email message or any document identifiable by an URL with HTTP or HTTPS scheme. Can only be seen or modified by the creator of the event."]
pub struct EventSource {
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title of the source; for example a title of a web page or an email subject."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL of the source pointing to a resource. The URL scheme must be HTTP or HTTPS."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventAttachment {
    #[serde(rename = "fileId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the attached file. Read-only.\nFor Google Drive files, this is the ID of the corresponding Files resource entry in the Drive API."]
    pub file_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fileUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL link to the attachment.\nFor adding Google Drive file attachments use the same format as in alternateLink property of the Files resource in the Drive API.\nRequired when adding an attachment."]
    pub file_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "iconLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL link to the attachment's icon. Read-only."]
    pub icon_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Internet media type (MIME type) of the attachment."]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Attachment title."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventAttendee {
    #[serde(rename = "additionalGuests")]
    #[serde(default = "event_attendee_defaults :: additional_guests")]
    #[doc = "Number of additional guests. Optional. The default is 0."]
    pub additional_guests: ::std::primitive::i64,
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The attendee's response comment. Optional."]
    pub comment: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The attendee's name, if available. Optional."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The attendee's email address, if available. This field must be present when adding an attendee. It must be a valid email address as per RFC5322.\nRequired when adding an attendee."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The attendee's Profile ID, if available. It corresponds to the id field in the People collection of the Google+ API"]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "optional")]
    #[serde(default = "event_attendee_defaults :: optional")]
    #[doc = "Whether this is an optional attendee. Optional. The default is False."]
    pub optional: ::std::primitive::bool,
    #[serde(rename = "organizer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the attendee is the organizer of the event. Read-only. The default is False."]
    pub organizer: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "resource")]
    #[serde(default = "event_attendee_defaults :: resource")]
    #[doc = "Whether the attendee is a resource. Can only be set when the attendee is added to the event for the first time. Subsequent modifications are ignored. Optional. The default is False."]
    pub resource: ::std::primitive::bool,
    #[serde(rename = "responseStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The attendee's response status. Possible values are:  \n- \"needsAction\" - The attendee has not responded to the invitation. \n- \"declined\" - The attendee has declined the invitation. \n- \"tentative\" - The attendee has tentatively accepted the invitation. \n- \"accepted\" - The attendee has accepted the invitation."]
    pub response_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "self")]
    #[serde(default = "event_attendee_defaults :: _self")]
    #[doc = "Whether this entry represents the calendar on which this copy of the event appears. Read-only. The default is False."]
    pub _self: ::std::primitive::bool,
}
mod event_attendee_defaults {
    pub fn additional_guests() -> ::std::primitive::i64 {
        serde_json::from_str(&"0").unwrap()
    }
    pub fn optional() -> ::std::primitive::bool {
        serde_json::from_str(&"false").unwrap()
    }
    pub fn resource() -> ::std::primitive::bool {
        serde_json::from_str(&"false").unwrap()
    }
    pub fn _self() -> ::std::primitive::bool {
        serde_json::from_str(&"false").unwrap()
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventDateTime {
    #[serde(rename = "date")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date, in the format \"yyyy-mm-dd\", if this is an all-day event."]
    pub date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time, as a combined date-time value (formatted according to RFC3339). A time zone offset is required unless a time zone is explicitly specified in timeZone."]
    pub date_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time zone in which the time is specified. (Formatted as an IANA Time Zone Database name, e.g. \"Europe/Zurich\".) For recurring events this field is required and specifies the time zone in which the recurrence is expanded. For single events this field is optional and indicates a custom time zone for the event start/end."]
    pub time_zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventReminder {
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The method used by this reminder. Possible values are:  \n- \"email\" - Reminders are sent via email. \n- \"popup\" - Reminders are sent via a UI popup.  \nRequired when adding a reminder."]
    pub method: ::std::option::Option<::std::string::String>,
    #[serde(rename = "minutes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of minutes before the start of the event when the reminder should trigger. Valid values are between 0 and 40320 (4 weeks in minutes).\nRequired when adding a reminder."]
    pub minutes: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Events {
    #[serde(rename = "accessRole")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user's access role for this calendar. Read-only. Possible values are:  \n- \"none\" - The user has no access. \n- \"freeBusyReader\" - The user has read access to free/busy information. \n- \"reader\" - The user has read access to the calendar. Private events will appear to users with reader access, but event details will be hidden. \n- \"writer\" - The user has read and write access to the calendar. Private events will appear to users with writer access, and event details will be visible. \n- \"owner\" - The user has ownership of the calendar. This role has all of the permissions of the writer role with the additional ability to see and manipulate ACLs."]
    pub access_role: ::std::option::Option<::std::string::String>,
    #[serde(rename = "defaultReminders")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The default reminders on the calendar for the authenticated user. These reminders apply to all events on this calendar that do not explicitly override them (i.e. do not have reminders.useDefault set to True)."]
    pub default_reminders: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventReminder>>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the calendar. Read-only."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of the collection."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of events on the calendar."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Event>>>,
    #[serde(rename = "kind")]
    #[serde(default = "events_defaults :: kind")]
    #[doc = "Type of the collection (\"calendar#events\")."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token used to access the next page of this result. Omitted if no further results are available, in which case nextSyncToken is provided."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextSyncToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token used at a later point in time to retrieve only the entries that have changed since this result was returned. Omitted if further results are available, in which case nextPageToken is provided."]
    pub next_sync_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title of the calendar. Read-only."]
    pub summary: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time zone of the calendar. Read-only."]
    pub time_zone: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Last modification time of the calendar (as a RFC3339 timestamp). Read-only."]
    pub updated: ::std::option::Option<::std::string::String>,
}
mod events_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("calendar#events")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreeBusyCalendar {
    #[serde(rename = "busy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of time ranges during which this calendar should be regarded as busy."]
    pub busy: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TimePeriod>>>,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional error(s) (if computation for the calendar failed)."]
    pub errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Error>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreeBusyGroup {
    #[serde(rename = "calendars")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of calendars' identifiers within a group."]
    pub calendars: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional error(s) (if computation for the group failed)."]
    pub errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Error>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreeBusyRequest {
    #[serde(rename = "calendarExpansionMax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximal number of calendars for which FreeBusy information is to be provided. Optional. Maximum value is 50."]
    pub calendar_expansion_max: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "groupExpansionMax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximal number of calendar identifiers to be provided for a single group. Optional. An error is returned for a group with more members than this value. Maximum value is 100."]
    pub group_expansion_max: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of calendars and/or groups to query."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FreeBusyRequestItem>>>,
    #[serde(rename = "timeMax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The end of the interval for the query formatted as per RFC3339."]
    pub time_max: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeMin")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The start of the interval for the query formatted as per RFC3339."]
    pub time_min: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeZone")]
    #[serde(default = "free_busy_request_defaults :: time_zone")]
    #[doc = "Time zone used in the response. Optional. The default is UTC."]
    pub time_zone: ::std::string::String,
}
mod free_busy_request_defaults {
    pub fn time_zone() -> ::std::string::String {
        String::from("UTC")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreeBusyRequestItem {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier of a calendar or a group."]
    pub id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreeBusyResponse {
    #[serde(rename = "calendars")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of free/busy information for calendars."]
    pub calendars: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<FreeBusyCalendar>>,
    >,
    #[serde(rename = "groups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Expansion of groups."]
    pub groups: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<FreeBusyGroup>>,
    >,
    #[serde(rename = "kind")]
    #[serde(default = "free_busy_response_defaults :: kind")]
    #[doc = "Type of the resource (\"calendar#freeBusy\")."]
    pub kind: ::std::string::String,
    #[serde(rename = "timeMax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The end of the interval."]
    pub time_max: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeMin")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The start of the interval."]
    pub time_min: ::std::option::Option<::std::string::String>,
}
mod free_busy_response_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("calendar#freeBusy")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Setting {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of the resource."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The id of the user setting."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "setting_defaults :: kind")]
    #[doc = "Type of the resource (\"calendar#setting\")."]
    pub kind: ::std::string::String,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Value of the user setting. The format of the value depends on the ID of the setting. It must always be a UTF-8 string of length up to 1024 characters."]
    pub value: ::std::option::Option<::std::string::String>,
}
mod setting_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("calendar#setting")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Etag of the collection."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of user settings."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Setting>>>,
    #[serde(rename = "kind")]
    #[serde(default = "settings_defaults :: kind")]
    #[doc = "Type of the collection (\"calendar#settings\")."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token used to access the next page of this result. Omitted if no further results are available, in which case nextSyncToken is provided."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextSyncToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token used at a later point in time to retrieve only the entries that have changed since this result was returned. Omitted if further results are available, in which case nextPageToken is provided."]
    pub next_sync_token: ::std::option::Option<::std::string::String>,
}
mod settings_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("calendar#settings")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TimePeriod {
    #[serde(rename = "end")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The (exclusive) end of the time period."]
    pub end: ::std::option::Option<::std::string::String>,
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The (inclusive) start of the time period."]
    pub start: ::std::option::Option<::std::string::String>,
}
