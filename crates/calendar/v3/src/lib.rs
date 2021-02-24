#[derive(
    Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
)]
pub struct QueryParameters {
    #[builder(default = "{ query_parameters_defaults :: alt () }", setter(into))]
    #[serde(rename = "alt")]
    #[serde(default = "query_parameters_defaults :: alt")]
    #[doc = "Data format for the response."]
    pub alt: QueryParametersAltEnum,
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
    #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
    pub quota_user: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "userIp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Please use quotaUser instead."]
    pub user_ip: ::std::option::Option<::std::string::String>,
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
#[doc = "Data format for the response."]
pub enum QueryParametersAltEnum {
    #[serde(rename = "json")]
    #[doc = "Responses with Content-Type of application/json"]
    Json,
}
impl ::std::default::Default for QueryParametersAltEnum {
    fn default() -> Self {
        Self::Json
    }
}
pub mod resources {
    pub mod acl {
        pub mod methods {
            pub mod insert {
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
                    #[serde(rename = "sendNotifications")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to send notifications about the calendar sharing change. Optional. The default is True."]
                    pub send_notifications: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Maximum number of entries returned on one result page. By default the value is 100 entries. The page size can never be larger than 250 entries. Optional."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token specifying which result page to return. Optional."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "showDeleted")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to include deleted ACLs in the result. Deleted ACLs are represented by role equal to \"none\". Deleted ACLs will always be included if syncToken is provided. Optional. The default is False."]
                    pub show_deleted: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "syncToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. All entries deleted since the previous list request will always be in the result set and it is not allowed to set showDeleted to False.\nIf the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.\nLearn more about incremental synchronization.\nOptional. The default is to return all entries."]
                    pub sync_token: ::std::option::Option<::std::string::String>,
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sendNotifications")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to send notifications about the calendar sharing change. Note that there are no notifications on access removal. Optional. The default is True."]
                    pub send_notifications: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod update {
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
                    #[serde(rename = "sendNotifications")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to send notifications about the calendar sharing change. Note that there are no notifications on access removal. Optional. The default is True."]
                    pub send_notifications: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod watch {
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
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Maximum number of entries returned on one result page. By default the value is 100 entries. The page size can never be larger than 250 entries. Optional."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token specifying which result page to return. Optional."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "showDeleted")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to include deleted ACLs in the result. Deleted ACLs are represented by role equal to \"none\". Deleted ACLs will always be included if syncToken is provided. Optional. The default is False."]
                    pub show_deleted: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "syncToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. All entries deleted since the previous list request will always be in the result set and it is not allowed to set showDeleted to False.\nIf the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.\nLearn more about incremental synchronization.\nOptional. The default is to return all entries."]
                    pub sync_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod calendar_list {
        pub mod methods {
            pub mod insert {
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
                    #[serde(rename = "colorRgbFormat")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to use the foregroundColor and backgroundColor fields to write the calendar colors (RGB). If this feature is used, the index-based colorId field will be set to the best matching option automatically. Optional. The default is False."]
                    pub color_rgb_format: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Maximum number of entries returned on one result page. By default the value is 100 entries. The page size can never be larger than 250 entries. Optional."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "minAccessRole")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The minimum access role for the user in the returned entries. Optional. The default is no restriction."]
                    pub min_access_role: ::std::option::Option<QueryParametersMinAccessRoleEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token specifying which result page to return. Optional."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "showDeleted")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to include deleted calendar list entries in the result. Optional. The default is False."]
                    pub show_deleted: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "showHidden")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to show hidden entries. Optional. The default is False."]
                    pub show_hidden: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "syncToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. If only read-only fields such as calendar properties or ACLs have changed, the entry won't be returned. All entries deleted and hidden since the previous list request will always be in the result set and it is not allowed to set showDeleted neither showHidden to False.\nTo ensure client state consistency minAccessRole query parameter cannot be specified together with nextSyncToken.\nIf the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.\nLearn more about incremental synchronization.\nOptional. The default is to return all entries."]
                    pub sync_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "The minimum access role for the user in the returned entries. Optional. The default is no restriction."]
                pub enum QueryParametersMinAccessRoleEnum {
                    #[serde(rename = "freeBusyReader")]
                    #[doc = "The user can read free/busy information."]
                    FreeBusyReader,
                    #[serde(rename = "owner")]
                    #[doc = "The user can read and modify events and access control lists."]
                    Owner,
                    #[serde(rename = "reader")]
                    #[doc = "The user can read events that are not private."]
                    Reader,
                    #[serde(rename = "writer")]
                    #[doc = "The user can read and modify events."]
                    Writer,
                }
                impl ::std::default::Default for QueryParametersMinAccessRoleEnum {
                    fn default() -> Self {
                        Self::FreeBusyReader
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "colorRgbFormat")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to use the foregroundColor and backgroundColor fields to write the calendar colors (RGB). If this feature is used, the index-based colorId field will be set to the best matching option automatically. Optional. The default is False."]
                    pub color_rgb_format: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod update {
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
                    #[serde(rename = "colorRgbFormat")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to use the foregroundColor and backgroundColor fields to write the calendar colors (RGB). If this feature is used, the index-based colorId field will be set to the best matching option automatically. Optional. The default is False."]
                    pub color_rgb_format: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod watch {
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
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Maximum number of entries returned on one result page. By default the value is 100 entries. The page size can never be larger than 250 entries. Optional."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "minAccessRole")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The minimum access role for the user in the returned entries. Optional. The default is no restriction."]
                    pub min_access_role: ::std::option::Option<QueryParametersMinAccessRoleEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token specifying which result page to return. Optional."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "showDeleted")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to include deleted calendar list entries in the result. Optional. The default is False."]
                    pub show_deleted: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "showHidden")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to show hidden entries. Optional. The default is False."]
                    pub show_hidden: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "syncToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. If only read-only fields such as calendar properties or ACLs have changed, the entry won't be returned. All entries deleted and hidden since the previous list request will always be in the result set and it is not allowed to set showDeleted neither showHidden to False.\nTo ensure client state consistency minAccessRole query parameter cannot be specified together with nextSyncToken.\nIf the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.\nLearn more about incremental synchronization.\nOptional. The default is to return all entries."]
                    pub sync_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "The minimum access role for the user in the returned entries. Optional. The default is no restriction."]
                pub enum QueryParametersMinAccessRoleEnum {
                    #[serde(rename = "freeBusyReader")]
                    #[doc = "The user can read free/busy information."]
                    FreeBusyReader,
                    #[serde(rename = "owner")]
                    #[doc = "The user can read and modify events and access control lists."]
                    Owner,
                    #[serde(rename = "reader")]
                    #[doc = "The user can read events that are not private."]
                    Reader,
                    #[serde(rename = "writer")]
                    #[doc = "The user can read and modify events."]
                    Writer,
                }
                impl ::std::default::Default for QueryParametersMinAccessRoleEnum {
                    fn default() -> Self {
                        Self::FreeBusyReader
                    }
                }
            }
        }
    }
    pub mod events {
        pub mod methods {
            pub mod delete {
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
                    #[serde(rename = "sendNotifications")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Deprecated. Please use sendUpdates instead.\n\nWhether to send notifications about the deletion of the event. Note that some emails might still be sent even if you set the value to false. The default is false."]
                    pub send_notifications: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sendUpdates")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Guests who should receive notifications about the deletion of the event."]
                    pub send_updates: ::std::option::Option<QueryParametersSendUpdatesEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Guests who should receive notifications about the deletion of the event."]
                pub enum QueryParametersSendUpdatesEnum {
                    #[serde(rename = "all")]
                    #[doc = "Notifications are sent to all guests."]
                    All,
                    #[serde(rename = "externalOnly")]
                    #[doc = "Notifications are sent to non-Google Calendar guests only."]
                    ExternalOnly,
                    #[serde(rename = "none")]
                    #[doc = "No notifications are sent. This value should only be used for migration use cases (note that in most migration cases the import method should be used)."]
                    None,
                }
                impl ::std::default::Default for QueryParametersSendUpdatesEnum {
                    fn default() -> Self {
                        Self::All
                    }
                }
            }
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "alwaysIncludeEmail")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Deprecated and ignored. A value will always be returned in the email field for the organizer, creator and attendees, even if no real email address is available (i.e. a generated, non-working value will be provided)."]
                    pub always_include_email: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxAttendees")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional."]
                    pub max_attendees: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "timeZone")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Time zone used in the response. Optional. The default is the time zone of the calendar."]
                    pub time_zone: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod import {
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
                    #[serde(rename = "conferenceDataVersion")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Version number of conference data supported by the API client. Version 0 assumes no conference data support and ignores conference data in the event's body. Version 1 enables support for copying of ConferenceData as well as for creating new conferences using the createRequest field of conferenceData. The default is 0."]
                    pub conference_data_version: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "supportsAttachments")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether API client performing operation supports event attachments. Optional. The default is False."]
                    pub supports_attachments: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod insert {
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
                    #[serde(rename = "conferenceDataVersion")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Version number of conference data supported by the API client. Version 0 assumes no conference data support and ignores conference data in the event's body. Version 1 enables support for copying of ConferenceData as well as for creating new conferences using the createRequest field of conferenceData. The default is 0."]
                    pub conference_data_version: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxAttendees")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional."]
                    pub max_attendees: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sendNotifications")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Deprecated. Please use sendUpdates instead.\n\nWhether to send notifications about the creation of the new event. Note that some emails might still be sent even if you set the value to false. The default is false."]
                    pub send_notifications: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sendUpdates")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to send notifications about the creation of the new event. Note that some emails might still be sent. The default is false."]
                    pub send_updates: ::std::option::Option<QueryParametersSendUpdatesEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "supportsAttachments")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether API client performing operation supports event attachments. Optional. The default is False."]
                    pub supports_attachments: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Whether to send notifications about the creation of the new event. Note that some emails might still be sent. The default is false."]
                pub enum QueryParametersSendUpdatesEnum {
                    #[serde(rename = "all")]
                    #[doc = "Notifications are sent to all guests."]
                    All,
                    #[serde(rename = "externalOnly")]
                    #[doc = "Notifications are sent to non-Google Calendar guests only."]
                    ExternalOnly,
                    #[serde(rename = "none")]
                    #[doc = "No notifications are sent. This value should only be used for migration use cases (note that in most migration cases the import method should be used)."]
                    None,
                }
                impl ::std::default::Default for QueryParametersSendUpdatesEnum {
                    fn default() -> Self {
                        Self::All
                    }
                }
            }
            pub mod instances {
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
                    #[serde(rename = "alwaysIncludeEmail")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Deprecated and ignored. A value will always be returned in the email field for the organizer, creator and attendees, even if no real email address is available (i.e. a generated, non-working value will be provided)."]
                    pub always_include_email: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxAttendees")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional."]
                    pub max_attendees: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Maximum number of events returned on one result page. By default the value is 250 events. The page size can never be larger than 2500 events. Optional."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "originalStart")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The original start time of the instance in the result. Optional."]
                    pub original_start: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token specifying which result page to return. Optional."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "showDeleted")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to include deleted events (with status equals \"cancelled\") in the result. Cancelled instances of recurring events will still be included if singleEvents is False. Optional. The default is False."]
                    pub show_deleted: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "timeMax")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Upper bound (exclusive) for an event's start time to filter by. Optional. The default is not to filter by start time. Must be an RFC3339 timestamp with mandatory time zone offset."]
                    pub time_max: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "timeMin")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Lower bound (inclusive) for an event's end time to filter by. Optional. The default is not to filter by end time. Must be an RFC3339 timestamp with mandatory time zone offset."]
                    pub time_min: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "timeZone")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Time zone used in the response. Optional. The default is the time zone of the calendar."]
                    pub time_zone: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "alwaysIncludeEmail")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Deprecated and ignored. A value will always be returned in the email field for the organizer, creator and attendees, even if no real email address is available (i.e. a generated, non-working value will be provided)."]
                    pub always_include_email: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "iCalUID")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies event ID in the iCalendar format to be included in the response. Optional."]
                    pub i_cal_uid: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxAttendees")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional."]
                    pub max_attendees: ::std::option::Option<::std::primitive::i64>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "Maximum number of events returned on one result page. The number of events in the resulting page may be less than this value, or none at all, even if there are more events matching the query. Incomplete pages can be detected by a non-empty nextPageToken field in the response. By default the value is 250 events. The page size can never be larger than 2500 events. Optional."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "orderBy")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The order of the events returned in the result. Optional. The default is an unspecified, stable order."]
                    pub order_by: ::std::option::Option<QueryParametersOrderByEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token specifying which result page to return. Optional."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "privateExtendedProperty")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Extended properties constraint specified as propertyName=value. Matches only private properties. This parameter might be repeated multiple times to return events that match all given constraints."]
                    pub private_extended_property: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "q")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Free text search terms to find events that match these terms in any field, except for extended properties. Optional."]
                    pub q: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sharedExtendedProperty")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Extended properties constraint specified as propertyName=value. Matches only shared properties. This parameter might be repeated multiple times to return events that match all given constraints."]
                    pub shared_extended_property: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "showDeleted")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to include deleted events (with status equals \"cancelled\") in the result. Cancelled instances of recurring events (but not the underlying recurring event) will still be included if showDeleted and singleEvents are both False. If showDeleted and singleEvents are both True, only single instances of deleted events (but not the underlying recurring events) are returned. Optional. The default is False."]
                    pub show_deleted: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "showHiddenInvitations")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to include hidden invitations in the result. Optional. The default is False."]
                    pub show_hidden_invitations: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "singleEvents")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to expand recurring events into instances and only return single one-off events and instances of recurring events, but not the underlying recurring events themselves. Optional. The default is False."]
                    pub single_events: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "syncToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. All events deleted since the previous list request will always be in the result set and it is not allowed to set showDeleted to False.\nThere are several query parameters that cannot be specified together with nextSyncToken to ensure consistency of the client state.\n\nThese are: \n- iCalUID \n- orderBy \n- privateExtendedProperty \n- q \n- sharedExtendedProperty \n- timeMin \n- timeMax \n- updatedMin If the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.\nLearn more about incremental synchronization.\nOptional. The default is to return all entries."]
                    pub sync_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "timeMax")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Upper bound (exclusive) for an event's start time to filter by. Optional. The default is not to filter by start time. Must be an RFC3339 timestamp with mandatory time zone offset, for example, 2011-06-03T10:00:00-07:00, 2011-06-03T10:00:00Z. Milliseconds may be provided but are ignored. If timeMin is set, timeMax must be greater than timeMin."]
                    pub time_max: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "timeMin")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Lower bound (exclusive) for an event's end time to filter by. Optional. The default is not to filter by end time. Must be an RFC3339 timestamp with mandatory time zone offset, for example, 2011-06-03T10:00:00-07:00, 2011-06-03T10:00:00Z. Milliseconds may be provided but are ignored. If timeMax is set, timeMin must be smaller than timeMax."]
                    pub time_min: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "timeZone")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Time zone used in the response. Optional. The default is the time zone of the calendar."]
                    pub time_zone: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "updatedMin")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Lower bound for an event's last modification time (as a RFC3339 timestamp) to filter by. When specified, entries deleted since this time will always be included regardless of showDeleted. Optional. The default is not to filter by last modification time."]
                    pub updated_min: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"250").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "The order of the events returned in the result. Optional. The default is an unspecified, stable order."]
                pub enum QueryParametersOrderByEnum {
                    #[serde(rename = "startTime")]
                    #[doc = "Order by the start date/time (ascending). This is only available when querying single events (i.e. the parameter singleEvents is True)"]
                    StartTime,
                    #[serde(rename = "updated")]
                    #[doc = "Order by last modification time (ascending)."]
                    Updated,
                }
                impl ::std::default::Default for QueryParametersOrderByEnum {
                    fn default() -> Self {
                        Self::StartTime
                    }
                }
            }
            pub mod _move {
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
                    #[serde(rename = "destination")]
                    #[doc = "Calendar identifier of the target calendar where the event is to be moved to."]
                    pub destination: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sendNotifications")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Deprecated. Please use sendUpdates instead.\n\nWhether to send notifications about the change of the event's organizer. Note that some emails might still be sent even if you set the value to false. The default is false."]
                    pub send_notifications: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sendUpdates")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Guests who should receive notifications about the change of the event's organizer."]
                    pub send_updates: ::std::option::Option<QueryParametersSendUpdatesEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Guests who should receive notifications about the change of the event's organizer."]
                pub enum QueryParametersSendUpdatesEnum {
                    #[serde(rename = "all")]
                    #[doc = "Notifications are sent to all guests."]
                    All,
                    #[serde(rename = "externalOnly")]
                    #[doc = "Notifications are sent to non-Google Calendar guests only."]
                    ExternalOnly,
                    #[serde(rename = "none")]
                    #[doc = "No notifications are sent. This value should only be used for migration use cases (note that in most migration cases the import method should be used)."]
                    None,
                }
                impl ::std::default::Default for QueryParametersSendUpdatesEnum {
                    fn default() -> Self {
                        Self::All
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "alwaysIncludeEmail")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Deprecated and ignored. A value will always be returned in the email field for the organizer, creator and attendees, even if no real email address is available (i.e. a generated, non-working value will be provided)."]
                    pub always_include_email: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "conferenceDataVersion")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Version number of conference data supported by the API client. Version 0 assumes no conference data support and ignores conference data in the event's body. Version 1 enables support for copying of ConferenceData as well as for creating new conferences using the createRequest field of conferenceData. The default is 0."]
                    pub conference_data_version: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxAttendees")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional."]
                    pub max_attendees: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sendNotifications")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Deprecated. Please use sendUpdates instead.\n\nWhether to send notifications about the event update (for example, description changes, etc.). Note that some emails might still be sent even if you set the value to false. The default is false."]
                    pub send_notifications: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sendUpdates")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Guests who should receive notifications about the event update (for example, title changes, etc.)."]
                    pub send_updates: ::std::option::Option<QueryParametersSendUpdatesEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "supportsAttachments")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether API client performing operation supports event attachments. Optional. The default is False."]
                    pub supports_attachments: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Guests who should receive notifications about the event update (for example, title changes, etc.)."]
                pub enum QueryParametersSendUpdatesEnum {
                    #[serde(rename = "all")]
                    #[doc = "Notifications are sent to all guests."]
                    All,
                    #[serde(rename = "externalOnly")]
                    #[doc = "Notifications are sent to non-Google Calendar guests only."]
                    ExternalOnly,
                    #[serde(rename = "none")]
                    #[doc = "No notifications are sent. This value should only be used for migration use cases (note that in most migration cases the import method should be used)."]
                    None,
                }
                impl ::std::default::Default for QueryParametersSendUpdatesEnum {
                    fn default() -> Self {
                        Self::All
                    }
                }
            }
            pub mod quick_add {
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
                    #[serde(rename = "sendNotifications")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Deprecated. Please use sendUpdates instead.\n\nWhether to send notifications about the creation of the event. Note that some emails might still be sent even if you set the value to false. The default is false."]
                    pub send_notifications: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sendUpdates")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Guests who should receive notifications about the creation of the new event."]
                    pub send_updates: ::std::option::Option<QueryParametersSendUpdatesEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "text")]
                    #[doc = "The text describing the event to be created."]
                    pub text: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Guests who should receive notifications about the creation of the new event."]
                pub enum QueryParametersSendUpdatesEnum {
                    #[serde(rename = "all")]
                    #[doc = "Notifications are sent to all guests."]
                    All,
                    #[serde(rename = "externalOnly")]
                    #[doc = "Notifications are sent to non-Google Calendar guests only."]
                    ExternalOnly,
                    #[serde(rename = "none")]
                    #[doc = "No notifications are sent. This value should only be used for migration use cases (note that in most migration cases the import method should be used)."]
                    None,
                }
                impl ::std::default::Default for QueryParametersSendUpdatesEnum {
                    fn default() -> Self {
                        Self::All
                    }
                }
            }
            pub mod update {
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
                    #[serde(rename = "alwaysIncludeEmail")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Deprecated and ignored. A value will always be returned in the email field for the organizer, creator and attendees, even if no real email address is available (i.e. a generated, non-working value will be provided)."]
                    pub always_include_email: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "conferenceDataVersion")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Version number of conference data supported by the API client. Version 0 assumes no conference data support and ignores conference data in the event's body. Version 1 enables support for copying of ConferenceData as well as for creating new conferences using the createRequest field of conferenceData. The default is 0."]
                    pub conference_data_version: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxAttendees")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional."]
                    pub max_attendees: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sendNotifications")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Deprecated. Please use sendUpdates instead.\n\nWhether to send notifications about the event update (for example, description changes, etc.). Note that some emails might still be sent even if you set the value to false. The default is false."]
                    pub send_notifications: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sendUpdates")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Guests who should receive notifications about the event update (for example, title changes, etc.)."]
                    pub send_updates: ::std::option::Option<QueryParametersSendUpdatesEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "supportsAttachments")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether API client performing operation supports event attachments. Optional. The default is False."]
                    pub supports_attachments: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Guests who should receive notifications about the event update (for example, title changes, etc.)."]
                pub enum QueryParametersSendUpdatesEnum {
                    #[serde(rename = "all")]
                    #[doc = "Notifications are sent to all guests."]
                    All,
                    #[serde(rename = "externalOnly")]
                    #[doc = "Notifications are sent to non-Google Calendar guests only."]
                    ExternalOnly,
                    #[serde(rename = "none")]
                    #[doc = "No notifications are sent. This value should only be used for migration use cases (note that in most migration cases the import method should be used)."]
                    None,
                }
                impl ::std::default::Default for QueryParametersSendUpdatesEnum {
                    fn default() -> Self {
                        Self::All
                    }
                }
            }
            pub mod watch {
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
                    #[serde(rename = "alwaysIncludeEmail")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Deprecated and ignored. A value will always be returned in the email field for the organizer, creator and attendees, even if no real email address is available (i.e. a generated, non-working value will be provided)."]
                    pub always_include_email: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "iCalUID")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies event ID in the iCalendar format to be included in the response. Optional."]
                    pub i_cal_uid: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxAttendees")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional."]
                    pub max_attendees: ::std::option::Option<::std::primitive::i64>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "Maximum number of events returned on one result page. The number of events in the resulting page may be less than this value, or none at all, even if there are more events matching the query. Incomplete pages can be detected by a non-empty nextPageToken field in the response. By default the value is 250 events. The page size can never be larger than 2500 events. Optional."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "orderBy")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The order of the events returned in the result. Optional. The default is an unspecified, stable order."]
                    pub order_by: ::std::option::Option<QueryParametersOrderByEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token specifying which result page to return. Optional."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "privateExtendedProperty")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Extended properties constraint specified as propertyName=value. Matches only private properties. This parameter might be repeated multiple times to return events that match all given constraints."]
                    pub private_extended_property: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "q")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Free text search terms to find events that match these terms in any field, except for extended properties. Optional."]
                    pub q: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sharedExtendedProperty")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Extended properties constraint specified as propertyName=value. Matches only shared properties. This parameter might be repeated multiple times to return events that match all given constraints."]
                    pub shared_extended_property: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "showDeleted")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to include deleted events (with status equals \"cancelled\") in the result. Cancelled instances of recurring events (but not the underlying recurring event) will still be included if showDeleted and singleEvents are both False. If showDeleted and singleEvents are both True, only single instances of deleted events (but not the underlying recurring events) are returned. Optional. The default is False."]
                    pub show_deleted: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "showHiddenInvitations")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to include hidden invitations in the result. Optional. The default is False."]
                    pub show_hidden_invitations: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "singleEvents")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Whether to expand recurring events into instances and only return single one-off events and instances of recurring events, but not the underlying recurring events themselves. Optional. The default is False."]
                    pub single_events: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "syncToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. All events deleted since the previous list request will always be in the result set and it is not allowed to set showDeleted to False.\nThere are several query parameters that cannot be specified together with nextSyncToken to ensure consistency of the client state.\n\nThese are: \n- iCalUID \n- orderBy \n- privateExtendedProperty \n- q \n- sharedExtendedProperty \n- timeMin \n- timeMax \n- updatedMin If the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.\nLearn more about incremental synchronization.\nOptional. The default is to return all entries."]
                    pub sync_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "timeMax")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Upper bound (exclusive) for an event's start time to filter by. Optional. The default is not to filter by start time. Must be an RFC3339 timestamp with mandatory time zone offset, for example, 2011-06-03T10:00:00-07:00, 2011-06-03T10:00:00Z. Milliseconds may be provided but are ignored. If timeMin is set, timeMax must be greater than timeMin."]
                    pub time_max: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "timeMin")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Lower bound (exclusive) for an event's end time to filter by. Optional. The default is not to filter by end time. Must be an RFC3339 timestamp with mandatory time zone offset, for example, 2011-06-03T10:00:00-07:00, 2011-06-03T10:00:00Z. Milliseconds may be provided but are ignored. If timeMax is set, timeMin must be smaller than timeMax."]
                    pub time_min: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "timeZone")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Time zone used in the response. Optional. The default is the time zone of the calendar."]
                    pub time_zone: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "updatedMin")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Lower bound for an event's last modification time (as a RFC3339 timestamp) to filter by. When specified, entries deleted since this time will always be included regardless of showDeleted. Optional. The default is not to filter by last modification time."]
                    pub updated_min: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"250").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "The order of the events returned in the result. Optional. The default is an unspecified, stable order."]
                pub enum QueryParametersOrderByEnum {
                    #[serde(rename = "startTime")]
                    #[doc = "Order by the start date/time (ascending). This is only available when querying single events (i.e. the parameter singleEvents is True)"]
                    StartTime,
                    #[serde(rename = "updated")]
                    #[doc = "Order by last modification time (ascending)."]
                    Updated,
                }
                impl ::std::default::Default for QueryParametersOrderByEnum {
                    fn default() -> Self {
                        Self::StartTime
                    }
                }
            }
        }
    }
    pub mod settings {
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
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Maximum number of entries returned on one result page. By default the value is 100 entries. The page size can never be larger than 250 entries. Optional."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token specifying which result page to return. Optional."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "syncToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then.\nIf the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.\nLearn more about incremental synchronization.\nOptional. The default is to return all entries."]
                    pub sync_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod watch {
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
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Maximum number of entries returned on one result page. By default the value is 100 entries. The page size can never be larger than 250 entries. Optional."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token specifying which result page to return. Optional."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "syncToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then.\nIf the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.\nLearn more about incremental synchronization.\nOptional. The default is to return all entries."]
                    pub sync_token: ::std::option::Option<::std::string::String>,
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
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Acl {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the collection."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of rules on the access control list."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AclRule>>>,
        #[builder(default = "{ acl_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "acl_defaults :: kind")]
        #[doc = "Type of the collection (\"calendar#acl\")."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token used to access the next page of this result. Omitted if no further results are available, in which case nextSyncToken is provided."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextSyncToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token used at a later point in time to retrieve only the entries that have changed since this result was returned. Omitted if further results are available, in which case nextPageToken is provided."]
        pub next_sync_token: ::std::option::Option<::std::string::String>,
    }
    impl Acl {
        pub fn builder() -> AclBuilder {
            AclBuilder::default()
        }
    }
    mod acl_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"calendar#acl\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AclRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier of the ACL rule."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ acl_rule_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "acl_rule_defaults :: kind")]
        #[doc = "Type of the resource (\"calendar#aclRule\")."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The role assigned to the scope. Possible values are:  \n- \"none\" - Provides no access. \n- \"freeBusyReader\" - Provides read access to free/busy information. \n- \"reader\" - Provides read access to the calendar. Private events will appear to users with reader access, but event details will be hidden. \n- \"writer\" - Provides read and write access to the calendar. Private events will appear to users with writer access, and event details will be visible. \n- \"owner\" - Provides ownership of the calendar. This role has all of the permissions of the writer role with the additional ability to see and manipulate ACLs."]
        pub role: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scope")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The scope of the rule."]
        pub scope: ::std::option::Option<AclRuleScope>,
    }
    impl AclRule {
        pub fn builder() -> AclRuleBuilder {
            AclRuleBuilder::default()
        }
    }
    mod acl_rule_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"calendar#aclRule\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The scope of the rule."]
    pub struct AclRuleScope {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the scope. Possible values are:  \n- \"default\" - The public scope. This is the default value. \n- \"user\" - Limits the scope to a single user. \n- \"group\" - Limits the scope to a group. \n- \"domain\" - Limits the scope to a domain.  Note: The permissions granted to the \"default\", or public, scope apply to any user, authenticated or not."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address of a user or group, or the name of a domain, depending on the scope type. Omitted for type \"default\"."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl AclRuleScope {
        pub fn builder() -> AclRuleScopeBuilder {
            AclRuleScopeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Calendar {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conferenceProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Conferencing properties for this calendar, for example what types of conferences are allowed."]
        pub conference_properties: ::std::option::Option<::std::boxed::Box<ConferenceProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the calendar. Optional."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier of the calendar. To retrieve IDs call the calendarList.list() method."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ calendar_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "calendar_defaults :: kind")]
        #[doc = "Type of the resource (\"calendar#calendar\")."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Geographic location of the calendar as free-form text. Optional."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "summary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title of the calendar."]
        pub summary: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time zone of the calendar. (Formatted as an IANA Time Zone Database name, e.g. \"Europe/Zurich\".) Optional."]
        pub time_zone: ::std::option::Option<::std::string::String>,
    }
    impl Calendar {
        pub fn builder() -> CalendarBuilder {
            CalendarBuilder::default()
        }
    }
    mod calendar_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"calendar#calendar\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct CalendarList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the collection."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Calendars that are present on the user's calendar list."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CalendarListEntry>>>,
        #[builder(default = "{ calendar_list_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "calendar_list_defaults :: kind")]
        #[doc = "Type of the collection (\"calendar#calendarList\")."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token used to access the next page of this result. Omitted if no further results are available, in which case nextSyncToken is provided."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextSyncToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token used at a later point in time to retrieve only the entries that have changed since this result was returned. Omitted if further results are available, in which case nextPageToken is provided."]
        pub next_sync_token: ::std::option::Option<::std::string::String>,
    }
    impl CalendarList {
        pub fn builder() -> CalendarListBuilder {
            CalendarListBuilder::default()
        }
    }
    mod calendar_list_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"calendar#calendarList\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct CalendarListEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accessRole")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The effective access role that the authenticated user has on the calendar. Read-only. Possible values are:  \n- \"freeBusyReader\" - Provides read access to free/busy information. \n- \"reader\" - Provides read access to the calendar. Private events will appear to users with reader access, but event details will be hidden. \n- \"writer\" - Provides read and write access to the calendar. Private events will appear to users with writer access, and event details will be visible. \n- \"owner\" - Provides ownership of the calendar. This role has all of the permissions of the writer role with the additional ability to see and manipulate ACLs."]
        pub access_role: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backgroundColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The main color of the calendar in the hexadecimal format \"#0088aa\". This property supersedes the index-based colorId property. To set or change this property, you need to specify colorRgbFormat=true in the parameters of the insert, update and patch methods. Optional."]
        pub background_color: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the calendar. This is an ID referring to an entry in the calendar section of the colors definition (see the colors endpoint). This property is superseded by the backgroundColor and foregroundColor properties and can be ignored when using these properties. Optional."]
        pub color_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conferenceProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Conferencing properties for this calendar, for example what types of conferences are allowed."]
        pub conference_properties: ::std::option::Option<::std::boxed::Box<ConferenceProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultReminders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The default reminders that the authenticated user has for this calendar."]
        pub default_reminders:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventReminder>>>,
        #[builder(
            default = "{ calendar_list_entry_defaults :: deleted () }",
            setter(into)
        )]
        #[serde(rename = "deleted")]
        #[serde(default = "calendar_list_entry_defaults :: deleted")]
        #[doc = "Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False."]
        pub deleted: ::std::primitive::bool,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the calendar. Optional. Read-only."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "foregroundColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The foreground color of the calendar in the hexadecimal format \"#ffffff\". This property supersedes the index-based colorId property. To set or change this property, you need to specify colorRgbFormat=true in the parameters of the insert, update and patch methods. Optional."]
        pub foreground_color: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ calendar_list_entry_defaults :: hidden () }",
            setter(into)
        )]
        #[serde(rename = "hidden")]
        #[serde(default = "calendar_list_entry_defaults :: hidden")]
        #[doc = "Whether the calendar has been hidden from the list. Optional. The attribute is only returned when the calendar is hidden, in which case the value is true."]
        pub hidden: ::std::primitive::bool,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier of the calendar."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ calendar_list_entry_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "calendar_list_entry_defaults :: kind")]
        #[doc = "Type of the resource (\"calendar#calendarListEntry\")."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Geographic location of the calendar as free-form text. Optional. Read-only."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notificationSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The notifications that the authenticated user is receiving for this calendar."]
        pub notification_settings: ::std::option::Option<CalendarListEntryNotificationSettings>,
        #[builder(
            default = "{ calendar_list_entry_defaults :: primary () }",
            setter(into)
        )]
        #[serde(rename = "primary")]
        #[serde(default = "calendar_list_entry_defaults :: primary")]
        #[doc = "Whether the calendar is the primary calendar of the authenticated user. Read-only. Optional. The default is False."]
        pub primary: ::std::primitive::bool,
        #[builder(
            default = "{ calendar_list_entry_defaults :: selected () }",
            setter(into)
        )]
        #[serde(rename = "selected")]
        #[serde(default = "calendar_list_entry_defaults :: selected")]
        #[doc = "Whether the calendar content shows up in the calendar UI. Optional. The default is False."]
        pub selected: ::std::primitive::bool,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "summary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title of the calendar. Read-only."]
        pub summary: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "summaryOverride")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The summary that the authenticated user has set for this calendar. Optional."]
        pub summary_override: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time zone of the calendar. Optional. Read-only."]
        pub time_zone: ::std::option::Option<::std::string::String>,
    }
    impl CalendarListEntry {
        pub fn builder() -> CalendarListEntryBuilder {
            CalendarListEntryBuilder::default()
        }
    }
    mod calendar_list_entry_defaults {
        pub fn deleted() -> ::std::primitive::bool {
            serde_json::from_str(&"false").unwrap()
        }
        pub fn hidden() -> ::std::primitive::bool {
            serde_json::from_str(&"false").unwrap()
        }
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"calendar#calendarListEntry\"").unwrap()
        }
        pub fn primary() -> ::std::primitive::bool {
            serde_json::from_str(&"false").unwrap()
        }
        pub fn selected() -> ::std::primitive::bool {
            serde_json::from_str(&"false").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The notifications that the authenticated user is receiving for this calendar."]
    pub struct CalendarListEntryNotificationSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notifications")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of notifications set for this calendar."]
        pub notifications:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CalendarNotification>>>,
    }
    impl CalendarListEntryNotificationSettings {
        pub fn builder() -> CalendarListEntryNotificationSettingsBuilder {
            CalendarListEntryNotificationSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct CalendarNotification {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The method used to deliver the notification. The possible value is:  \n- \"email\" - Notifications are sent via email.  \nRequired when adding a notification."]
        pub method: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of notification. Possible values are:  \n- \"eventCreation\" - Notification sent when a new event is put on the calendar. \n- \"eventChange\" - Notification sent when an event is changed. \n- \"eventCancellation\" - Notification sent when an event is cancelled. \n- \"eventResponse\" - Notification sent when an attendee responds to the event invitation. \n- \"agenda\" - An agenda with the events of the day (sent out in the morning).  \nRequired when adding a notification."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl CalendarNotification {
        pub fn builder() -> CalendarNotificationBuilder {
            CalendarNotificationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Channel {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "address")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The address where notifications are delivered for this channel."]
        pub address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expiration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional."]
        pub expiration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A UUID or similar unique string that identifies this channel."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ channel_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "channel_defaults :: kind")]
        #[doc = "Identifies this as a notification channel used to watch for changes to a resource, which is \"api#channel\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "params")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional parameters controlling delivery channel behavior. Optional."]
        pub params:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A Boolean value to indicate whether payload is wanted. Optional."]
        pub payload: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An opaque ID that identifies the resource being watched on this channel. Stable across different API versions."]
        pub resource_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A version-specific identifier for the watched resource."]
        pub resource_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "token")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An arbitrary string delivered to the target address with each notification delivered over this channel. Optional."]
        pub token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of delivery mechanism used for this channel. Valid values are \"web_hook\" (or \"webhook\"). Both values refer to a channel where Http requests are used to deliver messages."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl Channel {
        pub fn builder() -> ChannelBuilder {
            ChannelBuilder::default()
        }
    }
    mod channel_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"api#channel\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ColorDefinition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "background")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background color associated with this color definition."]
        pub background: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "foreground")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The foreground color that can be used to write on top of a background with 'background' color."]
        pub foreground: ::std::option::Option<::std::string::String>,
    }
    impl ColorDefinition {
        pub fn builder() -> ColorDefinitionBuilder {
            ColorDefinitionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Colors {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "calendar")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A global palette of calendar colors, mapping from the color ID to its definition. A calendarListEntry resource refers to one of these color IDs in its color field. Read-only."]
        pub calendar: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<ColorDefinition>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "event")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A global palette of event colors, mapping from the color ID to its definition. An event resource may refer to one of these color IDs in its color field. Read-only."]
        pub event: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<ColorDefinition>>,
        >,
        #[builder(default = "{ colors_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "colors_defaults :: kind")]
        #[doc = "Type of the resource (\"calendar#colors\")."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Last modification time of the color palette (as a RFC3339 timestamp). Read-only."]
        pub updated: ::std::option::Option<::std::string::String>,
    }
    impl Colors {
        pub fn builder() -> ColorsBuilder {
            ColorsBuilder::default()
        }
    }
    mod colors_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"calendar#colors\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ConferenceData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conferenceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the conference.\nCan be used by developers to keep track of conferences, should not be displayed to users.\nValues for solution types:  \n- \"eventHangout\": unset.\n- \"eventNamedHangout\": the name of the Hangout.\n- \"hangoutsMeet\": the 10-letter meeting code, for example \"aaa-bbbb-ccc\".\n- \"addOn\": defined by 3P conference provider.  Optional."]
        pub conference_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conferenceSolution")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The conference solution, such as Hangouts or Google Meet.\nUnset for a conference with a failed create request.\nEither conferenceSolution and at least one entryPoint, or createRequest is required."]
        pub conference_solution: ::std::option::Option<::std::boxed::Box<ConferenceSolution>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createRequest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A request to generate a new conference and attach it to the event. The data is generated asynchronously. To see whether the data is present check the status field.\nEither conferenceSolution and at least one entryPoint, or createRequest is required."]
        pub create_request: ::std::option::Option<::std::boxed::Box<CreateConferenceRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entryPoints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about individual conference entry points, such as URLs or phone numbers.\nAll of them must belong to the same conference.\nEither conferenceSolution and at least one entryPoint, or createRequest is required."]
        pub entry_points: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EntryPoint>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional notes (such as instructions from the domain administrator, legal notices) to display to the user. Can contain HTML. The maximum length is 2048 characters. Optional."]
        pub notes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional properties related to a conference. An example would be a solution-specific setting for enabling video streaming."]
        pub parameters: ::std::option::Option<::std::boxed::Box<ConferenceParameters>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "signature")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The signature of the conference data.\nGenerated on server side. Must be preserved while copying the conference data between events, otherwise the conference data will not be copied.\nUnset for a conference with a failed create request.\nOptional for a conference with a pending create request."]
        pub signature: ::std::option::Option<::std::string::String>,
    }
    impl ConferenceData {
        pub fn builder() -> ConferenceDataBuilder {
            ConferenceDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ConferenceParameters {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addOnParameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional add-on specific data."]
        pub add_on_parameters:
            ::std::option::Option<::std::boxed::Box<ConferenceParametersAddOnParameters>>,
    }
    impl ConferenceParameters {
        pub fn builder() -> ConferenceParametersBuilder {
            ConferenceParametersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ConferenceParametersAddOnParameters {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl ConferenceParametersAddOnParameters {
        pub fn builder() -> ConferenceParametersAddOnParametersBuilder {
            ConferenceParametersAddOnParametersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ConferenceProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowedConferenceSolutionTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The types of conference solutions that are supported for this calendar.\nThe possible values are:  \n- \"eventHangout\" \n- \"eventNamedHangout\" \n- \"hangoutsMeet\"  Optional."]
        pub allowed_conference_solution_types:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ConferenceProperties {
        pub fn builder() -> ConferencePropertiesBuilder {
            ConferencePropertiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ConferenceRequestStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statusCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current status of the conference create request. Read-only.\nThe possible values are:  \n- \"pending\": the conference create request is still being processed.\n- \"success\": the conference create request succeeded, the entry points are populated.\n- \"failure\": the conference create request failed, there are no entry points."]
        pub status_code: ::std::option::Option<::std::string::String>,
    }
    impl ConferenceRequestStatus {
        pub fn builder() -> ConferenceRequestStatusBuilder {
            ConferenceRequestStatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ConferenceSolution {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iconUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user-visible icon for this solution."]
        pub icon_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The key which can uniquely identify the conference solution for this event."]
        pub key: ::std::option::Option<::std::boxed::Box<ConferenceSolutionKey>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user-visible name of this solution. Not localized."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl ConferenceSolution {
        pub fn builder() -> ConferenceSolutionBuilder {
            ConferenceSolutionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ConferenceSolutionKey {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The conference solution type.\nIf a client encounters an unfamiliar or empty type, it should still be able to display the entry points. However, it should disallow modifications.\nThe possible values are:  \n- \"eventHangout\" for Hangouts for consumers (http://hangouts.google.com)\n- \"eventNamedHangout\" for classic Hangouts for Google Workspace users (http://hangouts.google.com)\n- \"hangoutsMeet\" for Google Meet (http://meet.google.com)\n- \"addOn\" for 3P conference providers"]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl ConferenceSolutionKey {
        pub fn builder() -> ConferenceSolutionKeyBuilder {
            ConferenceSolutionKeyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct CreateConferenceRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conferenceSolutionKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The conference solution, such as Hangouts or Google Meet."]
        pub conference_solution_key:
            ::std::option::Option<::std::boxed::Box<ConferenceSolutionKey>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The client-generated unique ID for this request.\nClients should regenerate this ID for every new request. If an ID provided is the same as for the previous request, the request is ignored."]
        pub request_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the conference create request."]
        pub status: ::std::option::Option<::std::boxed::Box<ConferenceRequestStatus>>,
    }
    impl CreateConferenceRequest {
        pub fn builder() -> CreateConferenceRequestBuilder {
            CreateConferenceRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct EntryPoint {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accessCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The access code to access the conference. The maximum length is 128 characters.\nWhen creating new conference data, populate only the subset of {meetingCode, accessCode, passcode, password, pin} fields that match the terminology that the conference provider uses. Only the populated fields should be displayed.\nOptional."]
        pub access_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entryPointFeatures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Features of the entry point, such as being toll or toll-free. One entry point can have multiple features. However, toll and toll-free cannot be both set on the same entry point."]
        pub entry_point_features: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entryPointType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the conference entry point.\nPossible values are:  \n- \"video\" - joining a conference over HTTP. A conference can have zero or one video entry point.\n- \"phone\" - joining a conference by dialing a phone number. A conference can have zero or more phone entry points.\n- \"sip\" - joining a conference over SIP. A conference can have zero or one sip entry point.\n- \"more\" - further conference joining instructions, for example additional phone numbers. A conference can have zero or one more entry point. A conference with only a more entry point is not a valid conference."]
        pub entry_point_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "label")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The label for the URI. Visible to end users. Not localized. The maximum length is 512 characters.\nExamples:  \n- for video: meet.google.com/aaa-bbbb-ccc\n- for phone: +1 123 268 2601\n- for sip: 12345678@altostrat.com\n- for more: should not be filled  \nOptional."]
        pub label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "meetingCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The meeting code to access the conference. The maximum length is 128 characters.\nWhen creating new conference data, populate only the subset of {meetingCode, accessCode, passcode, password, pin} fields that match the terminology that the conference provider uses. Only the populated fields should be displayed.\nOptional."]
        pub meeting_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "passcode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The passcode to access the conference. The maximum length is 128 characters.\nWhen creating new conference data, populate only the subset of {meetingCode, accessCode, passcode, password, pin} fields that match the terminology that the conference provider uses. Only the populated fields should be displayed."]
        pub passcode: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "password")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The password to access the conference. The maximum length is 128 characters.\nWhen creating new conference data, populate only the subset of {meetingCode, accessCode, passcode, password, pin} fields that match the terminology that the conference provider uses. Only the populated fields should be displayed.\nOptional."]
        pub password: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The PIN to access the conference. The maximum length is 128 characters.\nWhen creating new conference data, populate only the subset of {meetingCode, accessCode, passcode, password, pin} fields that match the terminology that the conference provider uses. Only the populated fields should be displayed.\nOptional."]
        pub pin: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "regionCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The CLDR/ISO 3166 region code for the country associated with this phone access. Example: \"SE\" for Sweden.\nCalendar backend will populate this field only for EntryPointType.PHONE."]
        pub region_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI of the entry point. The maximum length is 1300 characters.\nFormat:  \n- for video, http: or https: schema is required.\n- for phone, tel: schema is required. The URI should include the entire dial sequence (e.g., tel:+12345678900,,,123456789;1234).\n- for sip, sip: schema is required, e.g., sip:12345678@myprovider.com.\n- for more, http: or https: schema is required."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl EntryPoint {
        pub fn builder() -> EntryPointBuilder {
            EntryPointBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Error {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Domain, or broad category, of the error."]
        pub domain: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specific reason for the error. Some of the possible values are:  \n- \"groupTooBig\" - The group of users requested is too large for a single query. \n- \"tooManyCalendarsRequested\" - The number of calendars requested is too large for a single query. \n- \"notFound\" - The requested resource was not found. \n- \"internalError\" - The API service has encountered an internal error.  Additional error types may be added in the future, so clients should gracefully handle additional error statuses not included in this list."]
        pub reason: ::std::option::Option<::std::string::String>,
    }
    impl Error {
        pub fn builder() -> ErrorBuilder {
            ErrorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Event {
        #[builder(default = "{ event_defaults :: anyone_can_add_self () }", setter(into))]
        #[serde(rename = "anyoneCanAddSelf")]
        #[serde(default = "event_defaults :: anyone_can_add_self")]
        #[doc = "Whether anyone can invite themselves to the event (currently works for Google+ events only). Optional. The default is False."]
        pub anyone_can_add_self: ::std::primitive::bool,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attachments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "File attachments for the event. Currently only Google Drive attachments are supported.\nIn order to modify attachments the supportsAttachments request parameter should be set to true.\nThere can be at most 25 attachments per event,"]
        pub attachments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventAttachment>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attendees")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The attendees of the event. See the Events with attendees guide for more information on scheduling events with other calendar users. Service accounts need to use domain-wide delegation of authority to populate the attendee list."]
        pub attendees: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventAttendee>>>,
        #[builder(default = "{ event_defaults :: attendees_omitted () }", setter(into))]
        #[serde(rename = "attendeesOmitted")]
        #[serde(default = "event_defaults :: attendees_omitted")]
        #[doc = "Whether attendees may have been omitted from the event's representation. When retrieving an event, this may be due to a restriction specified by the maxAttendee query parameter. When updating an event, this can be used to only update the participant's response. Optional. The default is False."]
        pub attendees_omitted: ::std::primitive::bool,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the event. This is an ID referring to an entry in the event section of the colors definition (see the  colors endpoint). Optional."]
        pub color_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conferenceData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The conference-related information, such as details of a Google Meet conference. To create new conference details use the createRequest field. To persist your changes, remember to set the conferenceDataVersion request parameter to 1 for all event modification requests."]
        pub conference_data: ::std::option::Option<::std::boxed::Box<ConferenceData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "created")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Creation time of the event (as a RFC3339 timestamp). Read-only."]
        pub created: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creator")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creator of the event. Read-only."]
        pub creator: ::std::option::Option<EventCreator>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the event. Can contain HTML. Optional."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "end")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The (exclusive) end time of the event. For a recurring event, this is the end time of the first instance."]
        pub end: ::std::option::Option<::std::boxed::Box<EventDateTime>>,
        #[builder(
            default = "{ event_defaults :: end_time_unspecified () }",
            setter(into)
        )]
        #[serde(rename = "endTimeUnspecified")]
        #[serde(default = "event_defaults :: end_time_unspecified")]
        #[doc = "Whether the end time is actually unspecified. An end time is still provided for compatibility reasons, even if this attribute is set to True. The default is False."]
        pub end_time_unspecified: ::std::primitive::bool,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ event_defaults :: event_type () }", setter(into))]
        #[serde(rename = "eventType")]
        #[serde(default = "event_defaults :: event_type")]
        #[doc = "Specific type of the event. Read-only. Possible values are:  \n- \"default\" - A regular event or not further specified. \n- \"outOfOffice\" - An out-of-office event."]
        pub event_type: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "extendedProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Extended properties of the event."]
        pub extended_properties: ::std::option::Option<EventExtendedProperties>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gadget")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata."]
        pub gadget: ::std::option::Option<EventGadget>,
        #[builder(
            default = "{ event_defaults :: guests_can_invite_others () }",
            setter(into)
        )]
        #[serde(rename = "guestsCanInviteOthers")]
        #[serde(default = "event_defaults :: guests_can_invite_others")]
        #[doc = "Whether attendees other than the organizer can invite others to the event. Optional. The default is True."]
        pub guests_can_invite_others: ::std::primitive::bool,
        #[builder(default = "{ event_defaults :: guests_can_modify () }", setter(into))]
        #[serde(rename = "guestsCanModify")]
        #[serde(default = "event_defaults :: guests_can_modify")]
        #[doc = "Whether attendees other than the organizer can modify the event. Optional. The default is False."]
        pub guests_can_modify: ::std::primitive::bool,
        #[builder(
            default = "{ event_defaults :: guests_can_see_other_guests () }",
            setter(into)
        )]
        #[serde(rename = "guestsCanSeeOtherGuests")]
        #[serde(default = "event_defaults :: guests_can_see_other_guests")]
        #[doc = "Whether attendees other than the organizer can see who the event's attendees are. Optional. The default is True."]
        pub guests_can_see_other_guests: ::std::primitive::bool,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hangoutLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An absolute link to the Google+ hangout associated with this event. Read-only."]
        pub hangout_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "htmlLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An absolute link to this event in the Google Calendar Web UI. Read-only."]
        pub html_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iCalUID")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Event unique identifier as defined in RFC5545. It is used to uniquely identify events accross calendaring systems and must be supplied when importing events via the import method.\nNote that the icalUID and the id are not identical and only one of them should be supplied at event creation time. One difference in their semantics is that in recurring events, all occurrences of one event have different ids while they all share the same icalUIDs."]
        pub i_cal_uid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Opaque identifier of the event. When creating new single or recurring events, you can specify their IDs. Provided IDs must follow these rules:  \n- characters allowed in the ID are those used in base32hex encoding, i.e. lowercase letters a-v and digits 0-9, see section 3.1.2 in RFC2938 \n- the length of the ID must be between 5 and 1024 characters \n- the ID must be unique per calendar  Due to the globally distributed nature of the system, we cannot guarantee that ID collisions will be detected at event creation time. To minimize the risk of collisions we recommend using an established UUID algorithm such as one described in RFC4122.\nIf you do not specify an ID, it will be automatically generated by the server.\nNote that the icalUID and the id are not identical and only one of them should be supplied at event creation time. One difference in their semantics is that in recurring events, all occurrences of one event have different ids while they all share the same icalUIDs."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ event_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "event_defaults :: kind")]
        #[doc = "Type of the resource (\"calendar#event\")."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Geographic location of the event as free-form text. Optional."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ event_defaults :: locked () }", setter(into))]
        #[serde(rename = "locked")]
        #[serde(default = "event_defaults :: locked")]
        #[doc = "Whether this is a locked event copy where no changes can be made to the main event fields \"summary\", \"description\", \"location\", \"start\", \"end\" or \"recurrence\". The default is False. Read-Only."]
        pub locked: ::std::primitive::bool,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "organizer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The organizer of the event. If the organizer is also an attendee, this is indicated with a separate entry in attendees with the organizer field set to True. To change the organizer, use the move operation. Read-only, except when importing an event."]
        pub organizer: ::std::option::Option<EventOrganizer>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalStartTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For an instance of a recurring event, this is the time at which this event would start according to the recurrence data in the recurring event identified by recurringEventId. It uniquely identifies the instance within the recurring event series even if the instance was moved to a different time. Immutable."]
        pub original_start_time: ::std::option::Option<::std::boxed::Box<EventDateTime>>,
        #[builder(default = "{ event_defaults :: private_copy () }", setter(into))]
        #[serde(rename = "privateCopy")]
        #[serde(default = "event_defaults :: private_copy")]
        #[doc = "If set to True, Event propagation is disabled. Note that it is not the same thing as Private event properties. Optional. Immutable. The default is False."]
        pub private_copy: ::std::primitive::bool,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recurrence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of RRULE, EXRULE, RDATE and EXDATE lines for a recurring event, as specified in RFC5545. Note that DTSTART and DTEND lines are not allowed in this field; event start and end times are specified in the start and end fields. This field is omitted for single events or instances of recurring events."]
        pub recurrence: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recurringEventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For an instance of a recurring event, this is the id of the recurring event to which this instance belongs. Immutable."]
        pub recurring_event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reminders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about the event's reminders for the authenticated user."]
        pub reminders: ::std::option::Option<EventReminders>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sequence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sequence number as per iCalendar."]
        pub sequence: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Source from which the event was created. For example, a web page, an email message or any document identifiable by an URL with HTTP or HTTPS scheme. Can only be seen or modified by the creator of the event."]
        pub source: ::std::option::Option<EventSource>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "start")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The (inclusive) start time of the event. For a recurring event, this is the start time of the first instance."]
        pub start: ::std::option::Option<::std::boxed::Box<EventDateTime>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of the event. Optional. Possible values are:  \n- \"confirmed\" - The event is confirmed. This is the default status. \n- \"tentative\" - The event is tentatively confirmed. \n- \"cancelled\" - The event is cancelled (deleted). The list method returns cancelled events only on incremental sync (when syncToken or updatedMin are specified) or if the showDeleted flag is set to true. The get method always returns them.\nA cancelled status represents two different states depending on the event type:  \n- Cancelled exceptions of an uncancelled recurring event indicate that this instance should no longer be presented to the user. Clients should store these events for the lifetime of the parent recurring event.\nCancelled exceptions are only guaranteed to have values for the id, recurringEventId and originalStartTime fields populated. The other fields might be empty.  \n- All other cancelled events represent deleted events. Clients should remove their locally synced copies. Such cancelled events will eventually disappear, so do not rely on them being available indefinitely.\nDeleted events are only guaranteed to have the id field populated.   On the organizer's calendar, cancelled events continue to expose event details (summary, location, etc.) so that they can be restored (undeleted). Similarly, the events to which the user was invited and that they manually removed continue to provide details. However, incremental sync requests with showDeleted set to false will not return these details.\nIf an event changes its organizer (for example via the move operation) and the original organizer is not on the attendee list, it will leave behind a cancelled event where only the id field is guaranteed to be populated."]
        pub status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "summary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title of the event."]
        pub summary: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ event_defaults :: transparency () }", setter(into))]
        #[serde(rename = "transparency")]
        #[serde(default = "event_defaults :: transparency")]
        #[doc = "Whether the event blocks time on the calendar. Optional. Possible values are:  \n- \"opaque\" - Default value. The event does block time on the calendar. This is equivalent to setting Show me as to Busy in the Calendar UI. \n- \"transparent\" - The event does not block time on the calendar. This is equivalent to setting Show me as to Available in the Calendar UI."]
        pub transparency: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Last modification time of the event (as a RFC3339 timestamp). Read-only."]
        pub updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ event_defaults :: visibility () }", setter(into))]
        #[serde(rename = "visibility")]
        #[serde(default = "event_defaults :: visibility")]
        #[doc = "Visibility of the event. Optional. Possible values are:  \n- \"default\" - Uses the default visibility for events on the calendar. This is the default value. \n- \"public\" - The event is public and event details are visible to all readers of the calendar. \n- \"private\" - The event is private and only event attendees may view event details. \n- \"confidential\" - The event is private. This value is provided for compatibility reasons."]
        pub visibility: ::std::string::String,
    }
    impl Event {
        pub fn builder() -> EventBuilder {
            EventBuilder::default()
        }
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
            serde_json::from_str(&"\"default\"").unwrap()
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
            serde_json::from_str(&"\"calendar#event\"").unwrap()
        }
        pub fn locked() -> ::std::primitive::bool {
            serde_json::from_str(&"false").unwrap()
        }
        pub fn private_copy() -> ::std::primitive::bool {
            serde_json::from_str(&"false").unwrap()
        }
        pub fn transparency() -> ::std::string::String {
            serde_json::from_str(&"\"opaque\"").unwrap()
        }
        pub fn visibility() -> ::std::string::String {
            serde_json::from_str(&"\"default\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The creator of the event. Read-only."]
    pub struct EventCreator {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creator's name, if available."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creator's email address, if available."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creator's Profile ID, if available. It corresponds to the id field in the People collection of the Google+ API"]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ event_creator_defaults :: _self () }", setter(into))]
        #[serde(rename = "self")]
        #[serde(default = "event_creator_defaults :: _self")]
        #[doc = "Whether the creator corresponds to the calendar on which this copy of the event appears. Read-only. The default is False."]
        pub _self: ::std::primitive::bool,
    }
    impl EventCreator {
        pub fn builder() -> EventCreatorBuilder {
            EventCreatorBuilder::default()
        }
    }
    mod event_creator_defaults {
        pub fn _self() -> ::std::primitive::bool {
            serde_json::from_str(&"false").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Extended properties of the event."]
    pub struct EventExtendedProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "private")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Properties that are private to the copy of the event that appears on this calendar."]
        pub private:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shared")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Properties that are shared between copies of the event on other attendees' calendars."]
        pub shared:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl EventExtendedProperties {
        pub fn builder() -> EventExtendedPropertiesBuilder {
            EventExtendedPropertiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata."]
    pub struct EventGadget {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "display")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The gadget's display mode. Deprecated. Possible values are:  \n- \"icon\" - The gadget displays next to the event's title in the calendar view. \n- \"chip\" - The gadget displays when the event is clicked."]
        pub display: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The gadget's height in pixels. The height must be an integer greater than 0. Optional. Deprecated."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iconLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The gadget's icon URL. The URL scheme must be HTTPS. Deprecated."]
        pub icon_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "link")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The gadget's URL. The URL scheme must be HTTPS. Deprecated."]
        pub link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "preferences")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Preferences."]
        pub preferences:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The gadget's title. Deprecated."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The gadget's type. Deprecated."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The gadget's width in pixels. The width must be an integer greater than 0. Optional. Deprecated."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl EventGadget {
        pub fn builder() -> EventGadgetBuilder {
            EventGadgetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The organizer of the event. If the organizer is also an attendee, this is indicated with a separate entry in attendees with the organizer field set to True. To change the organizer, use the move operation. Read-only, except when importing an event."]
    pub struct EventOrganizer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The organizer's name, if available."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The organizer's email address, if available. It must be a valid email address as per RFC5322."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The organizer's Profile ID, if available. It corresponds to the id field in the People collection of the Google+ API"]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ event_organizer_defaults :: _self () }", setter(into))]
        #[serde(rename = "self")]
        #[serde(default = "event_organizer_defaults :: _self")]
        #[doc = "Whether the organizer corresponds to the calendar on which this copy of the event appears. Read-only. The default is False."]
        pub _self: ::std::primitive::bool,
    }
    impl EventOrganizer {
        pub fn builder() -> EventOrganizerBuilder {
            EventOrganizerBuilder::default()
        }
    }
    mod event_organizer_defaults {
        pub fn _self() -> ::std::primitive::bool {
            serde_json::from_str(&"false").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about the event's reminders for the authenticated user."]
    pub struct EventReminders {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "overrides")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the event doesn't use the default reminders, this lists the reminders specific to the event, or, if not set, indicates that no reminders are set for this event. The maximum number of override reminders is 5."]
        pub overrides: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventReminder>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "useDefault")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the default reminders of the calendar apply to the event."]
        pub use_default: ::std::option::Option<::std::primitive::bool>,
    }
    impl EventReminders {
        pub fn builder() -> EventRemindersBuilder {
            EventRemindersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Source from which the event was created. For example, a web page, an email message or any document identifiable by an URL with HTTP or HTTPS scheme. Can only be seen or modified by the creator of the event."]
    pub struct EventSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title of the source; for example a title of a web page or an email subject."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL of the source pointing to a resource. The URL scheme must be HTTP or HTTPS."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl EventSource {
        pub fn builder() -> EventSourceBuilder {
            EventSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct EventAttachment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the attached file. Read-only.\nFor Google Drive files, this is the ID of the corresponding Files resource entry in the Drive API."]
        pub file_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL link to the attachment.\nFor adding Google Drive file attachments use the same format as in alternateLink property of the Files resource in the Drive API.\nRequired when adding an attachment."]
        pub file_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iconLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL link to the attachment's icon. Read-only."]
        pub icon_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Internet media type (MIME type) of the attachment."]
        pub mime_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Attachment title."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl EventAttachment {
        pub fn builder() -> EventAttachmentBuilder {
            EventAttachmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct EventAttendee {
        #[builder(
            default = "{ event_attendee_defaults :: additional_guests () }",
            setter(into)
        )]
        #[serde(rename = "additionalGuests")]
        #[serde(default = "event_attendee_defaults :: additional_guests")]
        #[doc = "Number of additional guests. Optional. The default is 0."]
        pub additional_guests: ::std::primitive::i64,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "comment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The attendee's response comment. Optional."]
        pub comment: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The attendee's name, if available. Optional."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The attendee's email address, if available. This field must be present when adding an attendee. It must be a valid email address as per RFC5322.\nRequired when adding an attendee."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The attendee's Profile ID, if available. It corresponds to the id field in the People collection of the Google+ API"]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ event_attendee_defaults :: optional () }", setter(into))]
        #[serde(rename = "optional")]
        #[serde(default = "event_attendee_defaults :: optional")]
        #[doc = "Whether this is an optional attendee. Optional. The default is False."]
        pub optional: ::std::primitive::bool,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "organizer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the attendee is the organizer of the event. Read-only. The default is False."]
        pub organizer: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ event_attendee_defaults :: resource () }", setter(into))]
        #[serde(rename = "resource")]
        #[serde(default = "event_attendee_defaults :: resource")]
        #[doc = "Whether the attendee is a resource. Can only be set when the attendee is added to the event for the first time. Subsequent modifications are ignored. Optional. The default is False."]
        pub resource: ::std::primitive::bool,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The attendee's response status. Possible values are:  \n- \"needsAction\" - The attendee has not responded to the invitation. \n- \"declined\" - The attendee has declined the invitation. \n- \"tentative\" - The attendee has tentatively accepted the invitation. \n- \"accepted\" - The attendee has accepted the invitation."]
        pub response_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ event_attendee_defaults :: _self () }", setter(into))]
        #[serde(rename = "self")]
        #[serde(default = "event_attendee_defaults :: _self")]
        #[doc = "Whether this entry represents the calendar on which this copy of the event appears. Read-only. The default is False."]
        pub _self: ::std::primitive::bool,
    }
    impl EventAttendee {
        pub fn builder() -> EventAttendeeBuilder {
            EventAttendeeBuilder::default()
        }
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
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct EventDateTime {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "date")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date, in the format \"yyyy-mm-dd\", if this is an all-day event."]
        pub date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time, as a combined date-time value (formatted according to RFC3339). A time zone offset is required unless a time zone is explicitly specified in timeZone."]
        pub date_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time zone in which the time is specified. (Formatted as an IANA Time Zone Database name, e.g. \"Europe/Zurich\".) For recurring events this field is required and specifies the time zone in which the recurrence is expanded. For single events this field is optional and indicates a custom time zone for the event start/end."]
        pub time_zone: ::std::option::Option<::std::string::String>,
    }
    impl EventDateTime {
        pub fn builder() -> EventDateTimeBuilder {
            EventDateTimeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct EventReminder {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The method used by this reminder. Possible values are:  \n- \"email\" - Reminders are sent via email. \n- \"popup\" - Reminders are sent via a UI popup.  \nRequired when adding a reminder."]
        pub method: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minutes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of minutes before the start of the event when the reminder should trigger. Valid values are between 0 and 40320 (4 weeks in minutes).\nRequired when adding a reminder."]
        pub minutes: ::std::option::Option<::std::primitive::i64>,
    }
    impl EventReminder {
        pub fn builder() -> EventReminderBuilder {
            EventReminderBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Events {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accessRole")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's access role for this calendar. Read-only. Possible values are:  \n- \"none\" - The user has no access. \n- \"freeBusyReader\" - The user has read access to free/busy information. \n- \"reader\" - The user has read access to the calendar. Private events will appear to users with reader access, but event details will be hidden. \n- \"writer\" - The user has read and write access to the calendar. Private events will appear to users with writer access, and event details will be visible. \n- \"owner\" - The user has ownership of the calendar. This role has all of the permissions of the writer role with the additional ability to see and manipulate ACLs."]
        pub access_role: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultReminders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The default reminders on the calendar for the authenticated user. These reminders apply to all events on this calendar that do not explicitly override them (i.e. do not have reminders.useDefault set to True)."]
        pub default_reminders:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventReminder>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the calendar. Read-only."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the collection."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of events on the calendar."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Event>>>,
        #[builder(default = "{ events_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "events_defaults :: kind")]
        #[doc = "Type of the collection (\"calendar#events\")."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token used to access the next page of this result. Omitted if no further results are available, in which case nextSyncToken is provided."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextSyncToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token used at a later point in time to retrieve only the entries that have changed since this result was returned. Omitted if further results are available, in which case nextPageToken is provided."]
        pub next_sync_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "summary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title of the calendar. Read-only."]
        pub summary: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time zone of the calendar. Read-only."]
        pub time_zone: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Last modification time of the calendar (as a RFC3339 timestamp). Read-only."]
        pub updated: ::std::option::Option<::std::string::String>,
    }
    impl Events {
        pub fn builder() -> EventsBuilder {
            EventsBuilder::default()
        }
    }
    mod events_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"calendar#events\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct FreeBusyCalendar {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "busy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of time ranges during which this calendar should be regarded as busy."]
        pub busy: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TimePeriod>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional error(s) (if computation for the calendar failed)."]
        pub errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Error>>>,
    }
    impl FreeBusyCalendar {
        pub fn builder() -> FreeBusyCalendarBuilder {
            FreeBusyCalendarBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct FreeBusyGroup {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "calendars")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of calendars' identifiers within a group."]
        pub calendars: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional error(s) (if computation for the group failed)."]
        pub errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Error>>>,
    }
    impl FreeBusyGroup {
        pub fn builder() -> FreeBusyGroupBuilder {
            FreeBusyGroupBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct FreeBusyRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "calendarExpansionMax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximal number of calendars for which FreeBusy information is to be provided. Optional. Maximum value is 50."]
        pub calendar_expansion_max: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groupExpansionMax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximal number of calendar identifiers to be provided for a single group. Optional. An error is returned for a group with more members than this value. Maximum value is 100."]
        pub group_expansion_max: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of calendars and/or groups to query."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FreeBusyRequestItem>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeMax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end of the interval for the query formatted as per RFC3339."]
        pub time_max: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeMin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start of the interval for the query formatted as per RFC3339."]
        pub time_min: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ free_busy_request_defaults :: time_zone () }",
            setter(into)
        )]
        #[serde(rename = "timeZone")]
        #[serde(default = "free_busy_request_defaults :: time_zone")]
        #[doc = "Time zone used in the response. Optional. The default is UTC."]
        pub time_zone: ::std::string::String,
    }
    impl FreeBusyRequest {
        pub fn builder() -> FreeBusyRequestBuilder {
            FreeBusyRequestBuilder::default()
        }
    }
    mod free_busy_request_defaults {
        pub fn time_zone() -> ::std::string::String {
            serde_json::from_str(&"\"UTC\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct FreeBusyRequestItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier of a calendar or a group."]
        pub id: ::std::option::Option<::std::string::String>,
    }
    impl FreeBusyRequestItem {
        pub fn builder() -> FreeBusyRequestItemBuilder {
            FreeBusyRequestItemBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct FreeBusyResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "calendars")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of free/busy information for calendars."]
        pub calendars: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<FreeBusyCalendar>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groups")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Expansion of groups."]
        pub groups: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<FreeBusyGroup>>,
        >,
        #[builder(default = "{ free_busy_response_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "free_busy_response_defaults :: kind")]
        #[doc = "Type of the resource (\"calendar#freeBusy\")."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeMax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end of the interval."]
        pub time_max: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeMin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start of the interval."]
        pub time_min: ::std::option::Option<::std::string::String>,
    }
    impl FreeBusyResponse {
        pub fn builder() -> FreeBusyResponseBuilder {
            FreeBusyResponseBuilder::default()
        }
    }
    mod free_busy_response_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"calendar#freeBusy\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Setting {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The id of the user setting."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ setting_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "setting_defaults :: kind")]
        #[doc = "Type of the resource (\"calendar#setting\")."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value of the user setting. The format of the value depends on the ID of the setting. It must always be a UTF-8 string of length up to 1024 characters."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl Setting {
        pub fn builder() -> SettingBuilder {
            SettingBuilder::default()
        }
    }
    mod setting_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"calendar#setting\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Settings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of the collection."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of user settings."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Setting>>>,
        #[builder(default = "{ settings_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "settings_defaults :: kind")]
        #[doc = "Type of the collection (\"calendar#settings\")."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token used to access the next page of this result. Omitted if no further results are available, in which case nextSyncToken is provided."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextSyncToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token used at a later point in time to retrieve only the entries that have changed since this result was returned. Omitted if further results are available, in which case nextPageToken is provided."]
        pub next_sync_token: ::std::option::Option<::std::string::String>,
    }
    impl Settings {
        pub fn builder() -> SettingsBuilder {
            SettingsBuilder::default()
        }
    }
    mod settings_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"calendar#settings\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TimePeriod {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "end")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The (exclusive) end of the time period."]
        pub end: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "start")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The (inclusive) start of the time period."]
        pub start: ::std::option::Option<::std::string::String>,
    }
    impl TimePeriod {
        pub fn builder() -> TimePeriodBuilder {
            TimePeriodBuilder::default()
        }
    }
}
