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
    pub mod users {
        pub mod resources {
            pub mod drafts {
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
                                default = "{ query_parameters_defaults :: format () }",
                                setter(into)
                            )]
                            #[serde(rename = "format")]
                            #[serde(default = "query_parameters_defaults :: format")]
                            #[doc = "The format to return the draft in."]
                            pub format: QueryParametersFormatEnum,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        mod query_parameters_defaults {
                            pub fn format() -> super::QueryParametersFormatEnum {
                                serde_json::from_str(&"\"full\"").unwrap()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "The format to return the draft in."]
                        pub enum QueryParametersFormatEnum {
                            #[serde(rename = "minimal")]
                            #[doc = "Returns only email message ID and labels; does not return the email headers, body, or payload."]
                            Minimal,
                            #[serde(rename = "full")]
                            #[doc = "Returns the full email message data with body content parsed in the `payload` field; the `raw` field is not used. Format cannot be used when accessing the api using the gmail.metadata scope."]
                            Full,
                            #[serde(rename = "raw")]
                            #[doc = "Returns the full email message data with body content in the `raw` field as a base64url encoded string; the `payload` field is not used. Format cannot be used when accessing the api using the gmail.metadata scope."]
                            Raw,
                            #[serde(rename = "metadata")]
                            #[doc = "Returns only email message ID, labels, and email headers."]
                            Metadata,
                        }
                        impl ::std::default::Default for QueryParametersFormatEnum {
                            fn default() -> Self {
                                Self::Full
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
                                default = "{ query_parameters_defaults :: include_spam_trash () }",
                                setter(into)
                            )]
                            #[serde(rename = "includeSpamTrash")]
                            #[serde(default = "query_parameters_defaults :: include_spam_trash")]
                            #[doc = "Include drafts from `SPAM` and `TRASH` in the results."]
                            pub include_spam_trash: ::std::primitive::bool,
                            #[builder(
                                default = "{ query_parameters_defaults :: max_results () }",
                                setter(into)
                            )]
                            #[serde(rename = "maxResults")]
                            #[serde(default = "query_parameters_defaults :: max_results")]
                            #[doc = "Maximum number of drafts to return."]
                            pub max_results: ::std::primitive::i64,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Page token to retrieve a specific page of results in the list."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "q")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Only return draft messages matching the specified query. Supports the same query format as the Gmail search box. For example, `\"from:someuser@example.com rfc822msgid: is:unread\"`."]
                            pub q: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        mod query_parameters_defaults {
                            pub fn include_spam_trash() -> ::std::primitive::bool {
                                serde_json::from_str(&"false").unwrap()
                            }
                            pub fn max_results() -> ::std::primitive::i64 {
                                serde_json::from_str(&"100").unwrap()
                            }
                        }
                    }
                }
            }
            pub mod history {
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
                            #[serde(rename = "historyTypes")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "History types to be returned by the function"]
                            pub history_types:
                                ::std::option::Option<QueryParametersHistoryTypesEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "labelId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Only return messages with a label matching the ID."]
                            pub label_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ query_parameters_defaults :: max_results () }",
                                setter(into)
                            )]
                            #[serde(rename = "maxResults")]
                            #[serde(default = "query_parameters_defaults :: max_results")]
                            #[doc = "The maximum number of history records to return."]
                            pub max_results: ::std::primitive::i64,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Page token to retrieve a specific page of results in the list."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "startHistoryId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. Returns history records after the specified `startHistoryId`. The supplied `startHistoryId` should be obtained from the `historyId` of a message, thread, or previous `list` response. History IDs increase chronologically but are not contiguous with random gaps in between valid IDs. Supplying an invalid or out of date `startHistoryId` typically returns an `HTTP 404` error code. A `historyId` is typically valid for at least a week, but in some rare circumstances may be valid for only a few hours. If you receive an `HTTP 404` error response, your application should perform a full sync. If you receive no `nextPageToken` in the response, there are no updates to retrieve and you can store the returned `historyId` for a future request."]
                            pub start_history_id: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        mod query_parameters_defaults {
                            pub fn max_results() -> ::std::primitive::i64 {
                                serde_json::from_str(&"100").unwrap()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "History types to be returned by the function"]
                        pub enum QueryParametersHistoryTypesEnum {
                            #[serde(rename = "messageAdded")]
                            #[doc = ""]
                            MessageAdded,
                            #[serde(rename = "messageDeleted")]
                            #[doc = ""]
                            MessageDeleted,
                            #[serde(rename = "labelAdded")]
                            #[doc = ""]
                            LabelAdded,
                            #[serde(rename = "labelRemoved")]
                            #[doc = ""]
                            LabelRemoved,
                        }
                        impl ::std::default::Default for QueryParametersHistoryTypesEnum {
                            fn default() -> Self {
                                Self::MessageAdded
                            }
                        }
                    }
                }
            }
            pub mod messages {
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
                                default = "{ query_parameters_defaults :: format () }",
                                setter(into)
                            )]
                            #[serde(rename = "format")]
                            #[serde(default = "query_parameters_defaults :: format")]
                            #[doc = "The format to return the message in."]
                            pub format: QueryParametersFormatEnum,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "metadataHeaders")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "When given and format is `METADATA`, only include headers specified."]
                            pub metadata_headers: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        mod query_parameters_defaults {
                            pub fn format() -> super::QueryParametersFormatEnum {
                                serde_json::from_str(&"\"full\"").unwrap()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "The format to return the message in."]
                        pub enum QueryParametersFormatEnum {
                            #[serde(rename = "minimal")]
                            #[doc = "Returns only email message ID and labels; does not return the email headers, body, or payload."]
                            Minimal,
                            #[serde(rename = "full")]
                            #[doc = "Returns the full email message data with body content parsed in the `payload` field; the `raw` field is not used. Format cannot be used when accessing the api using the gmail.metadata scope."]
                            Full,
                            #[serde(rename = "raw")]
                            #[doc = "Returns the full email message data with body content in the `raw` field as a base64url encoded string; the `payload` field is not used. Format cannot be used when accessing the api using the gmail.metadata scope."]
                            Raw,
                            #[serde(rename = "metadata")]
                            #[doc = "Returns only email message ID, labels, and email headers."]
                            Metadata,
                        }
                        impl ::std::default::Default for QueryParametersFormatEnum {
                            fn default() -> Self {
                                Self::Full
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
                            #[builder(
                                default = "{ query_parameters_defaults :: deleted () }",
                                setter(into)
                            )]
                            #[serde(rename = "deleted")]
                            #[serde(default = "query_parameters_defaults :: deleted")]
                            #[doc = "Mark the email as permanently deleted (not TRASH) and only visible in Google Vault to a Vault administrator. Only used for G Suite accounts."]
                            pub deleted: ::std::primitive::bool,
                            #[builder(
                                default = "{ query_parameters_defaults :: internal_date_source () }",
                                setter(into)
                            )]
                            #[serde(rename = "internalDateSource")]
                            #[serde(default = "query_parameters_defaults :: internal_date_source")]
                            #[doc = "Source for Gmail's internal date of the message."]
                            pub internal_date_source: QueryParametersInternalDateSourceEnum,
                            #[builder(
                                default = "{ query_parameters_defaults :: never_mark_spam () }",
                                setter(into)
                            )]
                            #[serde(rename = "neverMarkSpam")]
                            #[serde(default = "query_parameters_defaults :: never_mark_spam")]
                            #[doc = "Ignore the Gmail spam classifier decision and never mark this email as SPAM in the mailbox."]
                            pub never_mark_spam: ::std::primitive::bool,
                            #[builder(
                                default = "{ query_parameters_defaults :: process_for_calendar () }",
                                setter(into)
                            )]
                            #[serde(rename = "processForCalendar")]
                            #[serde(default = "query_parameters_defaults :: process_for_calendar")]
                            #[doc = "Process calendar invites in the email and add any extracted meetings to the Google Calendar for this user."]
                            pub process_for_calendar: ::std::primitive::bool,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        mod query_parameters_defaults {
                            pub fn deleted() -> ::std::primitive::bool {
                                serde_json::from_str(&"false").unwrap()
                            }
                            pub fn internal_date_source(
                            ) -> super::QueryParametersInternalDateSourceEnum
                            {
                                serde_json::from_str(&"\"dateHeader\"").unwrap()
                            }
                            pub fn never_mark_spam() -> ::std::primitive::bool {
                                serde_json::from_str(&"false").unwrap()
                            }
                            pub fn process_for_calendar() -> ::std::primitive::bool {
                                serde_json::from_str(&"false").unwrap()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Source for Gmail's internal date of the message."]
                        pub enum QueryParametersInternalDateSourceEnum {
                            #[serde(rename = "receivedTime")]
                            #[doc = "Internal message date set to current time when received by Gmail."]
                            ReceivedTime,
                            #[serde(rename = "dateHeader")]
                            #[doc = "Internal message time based on 'Date' header in email, when valid."]
                            DateHeader,
                        }
                        impl ::std::default::Default for QueryParametersInternalDateSourceEnum {
                            fn default() -> Self {
                                Self::DateHeader
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
                            #[builder(
                                default = "{ query_parameters_defaults :: deleted () }",
                                setter(into)
                            )]
                            #[serde(rename = "deleted")]
                            #[serde(default = "query_parameters_defaults :: deleted")]
                            #[doc = "Mark the email as permanently deleted (not TRASH) and only visible in Google Vault to a Vault administrator. Only used for G Suite accounts."]
                            pub deleted: ::std::primitive::bool,
                            #[builder(
                                default = "{ query_parameters_defaults :: internal_date_source () }",
                                setter(into)
                            )]
                            #[serde(rename = "internalDateSource")]
                            #[serde(default = "query_parameters_defaults :: internal_date_source")]
                            #[doc = "Source for Gmail's internal date of the message."]
                            pub internal_date_source: QueryParametersInternalDateSourceEnum,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        mod query_parameters_defaults {
                            pub fn deleted() -> ::std::primitive::bool {
                                serde_json::from_str(&"false").unwrap()
                            }
                            pub fn internal_date_source(
                            ) -> super::QueryParametersInternalDateSourceEnum
                            {
                                serde_json::from_str(&"\"receivedTime\"").unwrap()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Source for Gmail's internal date of the message."]
                        pub enum QueryParametersInternalDateSourceEnum {
                            #[serde(rename = "receivedTime")]
                            #[doc = "Internal message date set to current time when received by Gmail."]
                            ReceivedTime,
                            #[serde(rename = "dateHeader")]
                            #[doc = "Internal message time based on 'Date' header in email, when valid."]
                            DateHeader,
                        }
                        impl ::std::default::Default for QueryParametersInternalDateSourceEnum {
                            fn default() -> Self {
                                Self::ReceivedTime
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
                                default = "{ query_parameters_defaults :: include_spam_trash () }",
                                setter(into)
                            )]
                            #[serde(rename = "includeSpamTrash")]
                            #[serde(default = "query_parameters_defaults :: include_spam_trash")]
                            #[doc = "Include messages from `SPAM` and `TRASH` in the results."]
                            pub include_spam_trash: ::std::primitive::bool,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "labelIds")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Only return messages with labels that match all of the specified label IDs."]
                            pub label_ids: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ query_parameters_defaults :: max_results () }",
                                setter(into)
                            )]
                            #[serde(rename = "maxResults")]
                            #[serde(default = "query_parameters_defaults :: max_results")]
                            #[doc = "Maximum number of messages to return."]
                            pub max_results: ::std::primitive::i64,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Page token to retrieve a specific page of results in the list."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "q")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Only return messages matching the specified query. Supports the same query format as the Gmail search box. For example, `\"from:someuser@example.com rfc822msgid: is:unread\"`. Parameter cannot be used when accessing the api using the gmail.metadata scope."]
                            pub q: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        mod query_parameters_defaults {
                            pub fn include_spam_trash() -> ::std::primitive::bool {
                                serde_json::from_str(&"false").unwrap()
                            }
                            pub fn max_results() -> ::std::primitive::i64 {
                                serde_json::from_str(&"100").unwrap()
                            }
                        }
                    }
                }
            }
            pub mod threads {
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
                                default = "{ query_parameters_defaults :: format () }",
                                setter(into)
                            )]
                            #[serde(rename = "format")]
                            #[serde(default = "query_parameters_defaults :: format")]
                            #[doc = "The format to return the messages in."]
                            pub format: QueryParametersFormatEnum,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "metadataHeaders")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "When given and format is METADATA, only include headers specified."]
                            pub metadata_headers: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        mod query_parameters_defaults {
                            pub fn format() -> super::QueryParametersFormatEnum {
                                serde_json::from_str(&"\"full\"").unwrap()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "The format to return the messages in."]
                        pub enum QueryParametersFormatEnum {
                            #[serde(rename = "full")]
                            #[doc = "Returns the full email message data with body content parsed in the `payload` field; the `raw` field is not used. Format cannot be used when accessing the api using the gmail.metadata scope."]
                            Full,
                            #[serde(rename = "metadata")]
                            #[doc = "Returns only email message IDs, labels, and email headers."]
                            Metadata,
                            #[serde(rename = "minimal")]
                            #[doc = "Returns only email message IDs and labels; does not return the email headers, body, or payload."]
                            Minimal,
                        }
                        impl ::std::default::Default for QueryParametersFormatEnum {
                            fn default() -> Self {
                                Self::Full
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
                                default = "{ query_parameters_defaults :: include_spam_trash () }",
                                setter(into)
                            )]
                            #[serde(rename = "includeSpamTrash")]
                            #[serde(default = "query_parameters_defaults :: include_spam_trash")]
                            #[doc = "Include threads from `SPAM` and `TRASH` in the results."]
                            pub include_spam_trash: ::std::primitive::bool,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "labelIds")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Only return threads with labels that match all of the specified label IDs."]
                            pub label_ids: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ query_parameters_defaults :: max_results () }",
                                setter(into)
                            )]
                            #[serde(rename = "maxResults")]
                            #[serde(default = "query_parameters_defaults :: max_results")]
                            #[doc = "Maximum number of threads to return."]
                            pub max_results: ::std::primitive::i64,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Page token to retrieve a specific page of results in the list."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "q")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Only return threads matching the specified query. Supports the same query format as the Gmail search box. For example, `\"from:someuser@example.com rfc822msgid: is:unread\"`. Parameter cannot be used when accessing the api using the gmail.metadata scope."]
                            pub q: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        mod query_parameters_defaults {
                            pub fn include_spam_trash() -> ::std::primitive::bool {
                                serde_json::from_str(&"false").unwrap()
                            }
                            pub fn max_results() -> ::std::primitive::i64 {
                                serde_json::from_str(&"100").unwrap()
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
    #[doc = "Auto-forwarding settings for an account."]
    pub struct AutoForwarding {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disposition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state that a message should be left in after it has been forwarded."]
        pub disposition: ::std::option::Option<AutoForwardingDispositionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emailAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email address to which all incoming messages are forwarded. This email address must be a verified member of the forwarding addresses."]
        pub email_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether all incoming mail is automatically forwarded to another address."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl AutoForwarding {
        pub fn builder() -> AutoForwardingBuilder {
            AutoForwardingBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The state that a message should be left in after it has been forwarded."]
    pub enum AutoForwardingDispositionEnum {
        #[serde(rename = "dispositionUnspecified")]
        #[doc = "Unspecified disposition."]
        DispositionUnspecified,
        #[serde(rename = "leaveInInbox")]
        #[doc = "Leave the message in the `INBOX`."]
        LeaveInInbox,
        #[serde(rename = "archive")]
        #[doc = "Archive the message."]
        Archive,
        #[serde(rename = "trash")]
        #[doc = "Move the message to the `TRASH`."]
        Trash,
        #[serde(rename = "markRead")]
        #[doc = "Leave the message in the `INBOX` and mark it as read."]
        MarkRead,
    }
    impl ::std::default::Default for AutoForwardingDispositionEnum {
        fn default() -> Self {
            Self::DispositionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct BatchDeleteMessagesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ids")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IDs of the messages to delete."]
        pub ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl BatchDeleteMessagesRequest {
        pub fn builder() -> BatchDeleteMessagesRequestBuilder {
            BatchDeleteMessagesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct BatchModifyMessagesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addLabelIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of label IDs to add to messages."]
        pub add_label_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ids")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IDs of the messages to modify. There is a limit of 1000 ids per request."]
        pub ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "removeLabelIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of label IDs to remove from messages."]
        pub remove_label_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl BatchModifyMessagesRequest {
        pub fn builder() -> BatchModifyMessagesRequestBuilder {
            BatchModifyMessagesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Settings for a delegate. Delegates can read, send, and delete messages, as well as view and add contacts, for the delegator's account. See \"Set up mail delegation\" for more information about delegates."]
    pub struct Delegate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "delegateEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address of the delegate."]
        pub delegate_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verificationStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether this address has been verified and can act as a delegate for the account. Read-only."]
        pub verification_status: ::std::option::Option<DelegateVerificationStatusEnum>,
    }
    impl Delegate {
        pub fn builder() -> DelegateBuilder {
            DelegateBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Indicates whether this address has been verified and can act as a delegate for the account. Read-only."]
    pub enum DelegateVerificationStatusEnum {
        #[serde(rename = "verificationStatusUnspecified")]
        #[doc = "Unspecified verification status."]
        VerificationStatusUnspecified,
        #[serde(rename = "accepted")]
        #[doc = "The address can act a delegate for the account."]
        Accepted,
        #[serde(rename = "pending")]
        #[doc = "A verification request was mailed to the address, and the owner has not yet accepted it."]
        Pending,
        #[serde(rename = "rejected")]
        #[doc = "A verification request was mailed to the address, and the owner rejected it."]
        Rejected,
        #[serde(rename = "expired")]
        #[doc = "A verification request was mailed to the address, and it expired without verification."]
        Expired,
    }
    impl ::std::default::Default for DelegateVerificationStatusEnum {
        fn default() -> Self {
            Self::VerificationStatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A draft email in the user's mailbox."]
    pub struct Draft {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The immutable ID of the draft."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The message content of the draft."]
        pub message: ::std::option::Option<::std::boxed::Box<Message>>,
    }
    impl Draft {
        pub fn builder() -> DraftBuilder {
            DraftBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Resource definition for Gmail filters. Filters apply to specific messages instead of an entire email thread."]
    pub struct Filter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "action")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Action that the filter performs."]
        pub action: ::std::option::Option<::std::boxed::Box<FilterAction>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "criteria")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Matching criteria for the filter."]
        pub criteria: ::std::option::Option<::std::boxed::Box<FilterCriteria>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The server assigned ID of the filter."]
        pub id: ::std::option::Option<::std::string::String>,
    }
    impl Filter {
        pub fn builder() -> FilterBuilder {
            FilterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A set of actions to perform on a message."]
    pub struct FilterAction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addLabelIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of labels to add to the message."]
        pub add_label_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "forward")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email address that the message should be forwarded to."]
        pub forward: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "removeLabelIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of labels to remove from the message."]
        pub remove_label_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl FilterAction {
        pub fn builder() -> FilterActionBuilder {
            FilterActionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message matching criteria."]
    pub struct FilterCriteria {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludeChats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the response should exclude chats."]
        pub exclude_chats: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "from")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sender's display name or email address."]
        pub from: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hasAttachment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the message has any attachment."]
        pub has_attachment: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "negatedQuery")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Only return messages not matching the specified query. Supports the same query format as the Gmail search box. For example, `\"from:someuser@example.com rfc822msgid: is:unread\"`."]
        pub negated_query: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "query")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Only return messages matching the specified query. Supports the same query format as the Gmail search box. For example, `\"from:someuser@example.com rfc822msgid: is:unread\"`."]
        pub query: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size of the entire RFC822 message in bytes, including all headers and attachments."]
        pub size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sizeComparison")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How the message size in bytes should be in relation to the size field."]
        pub size_comparison: ::std::option::Option<FilterCriteriaSizeComparisonEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subject")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Case-insensitive phrase found in the message's subject. Trailing and leading whitespace are be trimmed and adjacent spaces are collapsed."]
        pub subject: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "to")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The recipient's display name or email address. Includes recipients in the \"to\", \"cc\", and \"bcc\" header fields. You can use simply the local part of the email address. For example, \"example\" and \"example@\" both match \"example@gmail.com\". This field is case-insensitive."]
        pub to: ::std::option::Option<::std::string::String>,
    }
    impl FilterCriteria {
        pub fn builder() -> FilterCriteriaBuilder {
            FilterCriteriaBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "How the message size in bytes should be in relation to the size field."]
    pub enum FilterCriteriaSizeComparisonEnum {
        #[serde(rename = "unspecified")]
        #[doc = ""]
        Unspecified,
        #[serde(rename = "smaller")]
        #[doc = "Find messages smaller than the given size."]
        Smaller,
        #[serde(rename = "larger")]
        #[doc = "Find messages larger than the given size."]
        Larger,
    }
    impl ::std::default::Default for FilterCriteriaSizeComparisonEnum {
        fn default() -> Self {
            Self::Unspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Settings for a forwarding address."]
    pub struct ForwardingAddress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "forwardingEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An email address to which messages can be forwarded."]
        pub forwarding_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verificationStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether this address has been verified and is usable for forwarding. Read-only."]
        pub verification_status: ::std::option::Option<ForwardingAddressVerificationStatusEnum>,
    }
    impl ForwardingAddress {
        pub fn builder() -> ForwardingAddressBuilder {
            ForwardingAddressBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Indicates whether this address has been verified and is usable for forwarding. Read-only."]
    pub enum ForwardingAddressVerificationStatusEnum {
        #[serde(rename = "verificationStatusUnspecified")]
        #[doc = "Unspecified verification status."]
        VerificationStatusUnspecified,
        #[serde(rename = "accepted")]
        #[doc = "The address is ready to use for forwarding."]
        Accepted,
        #[serde(rename = "pending")]
        #[doc = "The address is awaiting verification by the owner."]
        Pending,
    }
    impl ::std::default::Default for ForwardingAddressVerificationStatusEnum {
        fn default() -> Self {
            Self::VerificationStatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A record of a change to the user's mailbox. Each history change may affect multiple messages in multiple ways."]
    pub struct History {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The mailbox sequence ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelsAdded")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Labels added to messages in this history record."]
        pub labels_added:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<HistoryLabelAdded>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelsRemoved")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Labels removed from messages in this history record."]
        pub labels_removed:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<HistoryLabelRemoved>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of messages changed in this history record. The fields for specific change types, such as `messagesAdded` may duplicate messages in this field. We recommend using the specific change-type fields instead of this."]
        pub messages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Message>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messagesAdded")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Messages added to the mailbox in this history record."]
        pub messages_added:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<HistoryMessageAdded>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messagesDeleted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Messages deleted (not Trashed) from the mailbox in this history record."]
        pub messages_deleted:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<HistoryMessageDeleted>>>,
    }
    impl History {
        pub fn builder() -> HistoryBuilder {
            HistoryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct HistoryLabelAdded {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label IDs added to the message."]
        pub label_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::boxed::Box<Message>>,
    }
    impl HistoryLabelAdded {
        pub fn builder() -> HistoryLabelAddedBuilder {
            HistoryLabelAddedBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct HistoryLabelRemoved {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label IDs removed from the message."]
        pub label_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::boxed::Box<Message>>,
    }
    impl HistoryLabelRemoved {
        pub fn builder() -> HistoryLabelRemovedBuilder {
            HistoryLabelRemovedBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct HistoryMessageAdded {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::boxed::Box<Message>>,
    }
    impl HistoryMessageAdded {
        pub fn builder() -> HistoryMessageAddedBuilder {
            HistoryMessageAddedBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct HistoryMessageDeleted {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::boxed::Box<Message>>,
    }
    impl HistoryMessageDeleted {
        pub fn builder() -> HistoryMessageDeletedBuilder {
            HistoryMessageDeletedBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "IMAP settings for an account."]
    pub struct ImapSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoExpunge")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If this value is true, Gmail will immediately expunge a message when it is marked as deleted in IMAP. Otherwise, Gmail will wait for an update from the client before expunging messages marked as deleted."]
        pub auto_expunge: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether IMAP is enabled for the account."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expungeBehavior")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The action that will be executed on a message when it is marked as deleted and expunged from the last visible IMAP folder."]
        pub expunge_behavior: ::std::option::Option<ImapSettingsExpungeBehaviorEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxFolderSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional limit on the number of messages that an IMAP folder may contain. Legal values are 0, 1000, 2000, 5000 or 10000. A value of zero is interpreted to mean that there is no limit."]
        pub max_folder_size: ::std::option::Option<::std::primitive::i64>,
    }
    impl ImapSettings {
        pub fn builder() -> ImapSettingsBuilder {
            ImapSettingsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The action that will be executed on a message when it is marked as deleted and expunged from the last visible IMAP folder."]
    pub enum ImapSettingsExpungeBehaviorEnum {
        #[serde(rename = "expungeBehaviorUnspecified")]
        #[doc = "Unspecified behavior."]
        ExpungeBehaviorUnspecified,
        #[serde(rename = "archive")]
        #[doc = "Archive messages marked as deleted."]
        Archive,
        #[serde(rename = "trash")]
        #[doc = "Move messages marked as deleted to the trash."]
        Trash,
        #[serde(rename = "deleteForever")]
        #[doc = "Immediately and permanently delete messages marked as deleted. The expunged messages cannot be recovered."]
        DeleteForever,
    }
    impl ::std::default::Default for ImapSettingsExpungeBehaviorEnum {
        fn default() -> Self {
            Self::ExpungeBehaviorUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Labels are used to categorize messages and threads within the user's mailbox."]
    pub struct Label {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "color")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color to assign to the label. Color is only available for labels that have their `type` set to `user`."]
        pub color: ::std::option::Option<::std::boxed::Box<LabelColor>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The immutable ID of the label."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelListVisibility")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visibility of the label in the label list in the Gmail web interface."]
        pub label_list_visibility: ::std::option::Option<LabelLabelListVisibilityEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messageListVisibility")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visibility of messages with this label in the message list in the Gmail web interface."]
        pub message_list_visibility: ::std::option::Option<LabelMessageListVisibilityEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messagesTotal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of messages with the label."]
        pub messages_total: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messagesUnread")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of unread messages with the label."]
        pub messages_unread: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The display name of the label."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threadsTotal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of threads with the label."]
        pub threads_total: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threadsUnread")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of unread threads with the label."]
        pub threads_unread: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The owner type for the label. User labels are created by the user and can be modified and deleted by the user and can be applied to any message or thread. System labels are internally created and cannot be added, modified, or deleted. System labels may be able to be applied to or removed from messages and threads under some circumstances but this is not guaranteed. For example, users can apply and remove the `INBOX` and `UNREAD` labels from messages and threads, but cannot apply or remove the `DRAFTS` or `SENT` labels from messages or threads."]
        pub _type: ::std::option::Option<LabelTypeEnum>,
    }
    impl Label {
        pub fn builder() -> LabelBuilder {
            LabelBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The visibility of the label in the label list in the Gmail web interface."]
    pub enum LabelLabelListVisibilityEnum {
        #[serde(rename = "labelShow")]
        #[doc = "Show the label in the label list."]
        LabelShow,
        #[serde(rename = "labelShowIfUnread")]
        #[doc = "Show the label if there are any unread messages with that label."]
        LabelShowIfUnread,
        #[serde(rename = "labelHide")]
        #[doc = "Do not show the label in the label list."]
        LabelHide,
    }
    impl ::std::default::Default for LabelLabelListVisibilityEnum {
        fn default() -> Self {
            Self::LabelShow
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The visibility of messages with this label in the message list in the Gmail web interface."]
    pub enum LabelMessageListVisibilityEnum {
        #[serde(rename = "show")]
        #[doc = "Show the label in the message list."]
        Show,
        #[serde(rename = "hide")]
        #[doc = "Do not show the label in the message list."]
        Hide,
    }
    impl ::std::default::Default for LabelMessageListVisibilityEnum {
        fn default() -> Self {
            Self::Show
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The owner type for the label. User labels are created by the user and can be modified and deleted by the user and can be applied to any message or thread. System labels are internally created and cannot be added, modified, or deleted. System labels may be able to be applied to or removed from messages and threads under some circumstances but this is not guaranteed. For example, users can apply and remove the `INBOX` and `UNREAD` labels from messages and threads, but cannot apply or remove the `DRAFTS` or `SENT` labels from messages or threads."]
    pub enum LabelTypeEnum {
        #[serde(rename = "system")]
        #[doc = "Labels created by Gmail."]
        System,
        #[serde(rename = "user")]
        #[doc = "Custom labels created by the user or application."]
        User,
    }
    impl ::std::default::Default for LabelTypeEnum {
        fn default() -> Self {
            Self::System
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LabelColor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backgroundColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background color represented as hex string #RRGGBB (ex #000000). This field is required in order to set the color of a label. Only the following predefined set of color values are allowed: \\#000000, #434343, #666666, #999999, #cccccc, #efefef, #f3f3f3, #ffffff, \\#fb4c2f, #ffad47, #fad165, #16a766, #43d692, #4a86e8, #a479e2, #f691b3, \\#f6c5be, #ffe6c7, #fef1d1, #b9e4d0, #c6f3de, #c9daf8, #e4d7f5, #fcdee8, \\#efa093, #ffd6a2, #fce8b3, #89d3b2, #a0eac9, #a4c2f4, #d0bcf1, #fbc8d9, \\#e66550, #ffbc6b, #fcda83, #44b984, #68dfa9, #6d9eeb, #b694e8, #f7a7c0, \\#cc3a21, #eaa041, #f2c960, #149e60, #3dc789, #3c78d8, #8e63ce, #e07798, \\#ac2b16, #cf8933, #d5ae49, #0b804b, #2a9c68, #285bac, #653e9b, #b65775, \\#822111, #a46a21, #aa8831, #076239, #1a764d, #1c4587, #41236d, #83334c \\#464646, #e7e7e7, #0d3472, #b6cff5, #0d3b44, #98d7e4, #3d188e, #e3d7ff, \\#711a36, #fbd3e0, #8a1c0a, #f2b2a8, #7a2e0b, #ffc8af, #7a4706, #ffdeb5, \\#594c05, #fbe983, #684e07, #fdedc1, #0b4f30, #b3efd3, #04502e, #a2dcc1, \\#c2c2c2, #4986e7, #2da2bb, #b99aff, #994a64, #f691b2, #ff7537, #ffad46, \\#662e37, #ebdbde, #cca6ac, #094228, #42d692, #16a765"]
        pub background_color: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text color of the label, represented as hex string. This field is required in order to set the color of a label. Only the following predefined set of color values are allowed: \\#000000, #434343, #666666, #999999, #cccccc, #efefef, #f3f3f3, #ffffff, \\#fb4c2f, #ffad47, #fad165, #16a766, #43d692, #4a86e8, #a479e2, #f691b3, \\#f6c5be, #ffe6c7, #fef1d1, #b9e4d0, #c6f3de, #c9daf8, #e4d7f5, #fcdee8, \\#efa093, #ffd6a2, #fce8b3, #89d3b2, #a0eac9, #a4c2f4, #d0bcf1, #fbc8d9, \\#e66550, #ffbc6b, #fcda83, #44b984, #68dfa9, #6d9eeb, #b694e8, #f7a7c0, \\#cc3a21, #eaa041, #f2c960, #149e60, #3dc789, #3c78d8, #8e63ce, #e07798, \\#ac2b16, #cf8933, #d5ae49, #0b804b, #2a9c68, #285bac, #653e9b, #b65775, \\#822111, #a46a21, #aa8831, #076239, #1a764d, #1c4587, #41236d, #83334c \\#464646, #e7e7e7, #0d3472, #b6cff5, #0d3b44, #98d7e4, #3d188e, #e3d7ff, \\#711a36, #fbd3e0, #8a1c0a, #f2b2a8, #7a2e0b, #ffc8af, #7a4706, #ffdeb5, \\#594c05, #fbe983, #684e07, #fdedc1, #0b4f30, #b3efd3, #04502e, #a2dcc1, \\#c2c2c2, #4986e7, #2da2bb, #b99aff, #994a64, #f691b2, #ff7537, #ffad46, \\#662e37, #ebdbde, #cca6ac, #094228, #42d692, #16a765"]
        pub text_color: ::std::option::Option<::std::string::String>,
    }
    impl LabelColor {
        pub fn builder() -> LabelColorBuilder {
            LabelColorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Language settings for an account. These settings correspond to the \"Language settings\" feature in the web interface."]
    pub struct LanguageSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language to display Gmail in, formatted as an RFC 3066 Language Tag (for example `en-GB`, `fr` or `ja` for British English, French, or Japanese respectively). The set of languages supported by Gmail evolves over time, so please refer to the \"Language\" dropdown in the Gmail settings for all available options, as described in the language settings help article. A table of sample values is also provided in the Managing Language Settings guide Not all Gmail clients can display the same set of languages. In the case that a user's display language is not available for use on a particular client, said client automatically chooses to display in the closest supported variant (or a reasonable default)."]
        pub display_language: ::std::option::Option<::std::string::String>,
    }
    impl LanguageSettings {
        pub fn builder() -> LanguageSettingsBuilder {
            LanguageSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for the ListDelegates method."]
    pub struct ListDelegatesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "delegates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of the user's delegates (with any verification status). If an account doesn't have delegates, this field doesn't appear."]
        pub delegates: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Delegate>>>,
    }
    impl ListDelegatesResponse {
        pub fn builder() -> ListDelegatesResponseBuilder {
            ListDelegatesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ListDraftsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "drafts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of drafts. Note that the `Message` property in each `Draft` resource only contains an `id` and a `threadId`. The messages.get method can fetch additional message details."]
        pub drafts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Draft>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resultSizeEstimate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Estimated total number of results."]
        pub result_size_estimate: ::std::option::Option<::std::primitive::i64>,
    }
    impl ListDraftsResponse {
        pub fn builder() -> ListDraftsResponseBuilder {
            ListDraftsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for the ListFilters method."]
    pub struct ListFiltersResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of a user's filters."]
        pub filter: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Filter>>>,
    }
    impl ListFiltersResponse {
        pub fn builder() -> ListFiltersResponseBuilder {
            ListFiltersResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for the ListForwardingAddresses method."]
    pub struct ListForwardingAddressesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "forwardingAddresses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of addresses that may be used for forwarding."]
        pub forwarding_addresses:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ForwardingAddress>>>,
    }
    impl ListForwardingAddressesResponse {
        pub fn builder() -> ListForwardingAddressesResponseBuilder {
            ListForwardingAddressesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ListHistoryResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "history")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of history records. Any `messages` contained in the response will typically only have `id` and `threadId` fields populated."]
        pub history: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<History>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "historyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the mailbox's current history record."]
        pub history_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page token to retrieve the next page of results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListHistoryResponse {
        pub fn builder() -> ListHistoryResponseBuilder {
            ListHistoryResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ListLabelsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of labels. Note that each label resource only contains an `id`, `name`, `messageListVisibility`, `labelListVisibility`, and `type`. The labels.get method can fetch additional label details."]
        pub labels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Label>>>,
    }
    impl ListLabelsResponse {
        pub fn builder() -> ListLabelsResponseBuilder {
            ListLabelsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ListMessagesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of messages. Note that each message resource contains only an `id` and a `threadId`. Additional message details can be fetched using the messages.get method."]
        pub messages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Message>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resultSizeEstimate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Estimated total number of results."]
        pub result_size_estimate: ::std::option::Option<::std::primitive::i64>,
    }
    impl ListMessagesResponse {
        pub fn builder() -> ListMessagesResponseBuilder {
            ListMessagesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for the ListSendAs method."]
    pub struct ListSendAsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sendAs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of send-as aliases."]
        pub send_as: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SendAs>>>,
    }
    impl ListSendAsResponse {
        pub fn builder() -> ListSendAsResponseBuilder {
            ListSendAsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ListSmimeInfoResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "smimeInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of SmimeInfo."]
        pub smime_info: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SmimeInfo>>>,
    }
    impl ListSmimeInfoResponse {
        pub fn builder() -> ListSmimeInfoResponseBuilder {
            ListSmimeInfoResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ListThreadsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page token to retrieve the next page of results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resultSizeEstimate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Estimated total number of results."]
        pub result_size_estimate: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threads")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of threads. Note that each thread resource does not contain a list of `messages`. The list of `messages` for a given thread can be fetched using the threads.get method."]
        pub threads: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Thread>>>,
    }
    impl ListThreadsResponse {
        pub fn builder() -> ListThreadsResponseBuilder {
            ListThreadsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An email message."]
    pub struct Message {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "historyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the last history record that modified this message."]
        pub history_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The immutable ID of the message."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "internalDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The internal message creation timestamp (epoch ms), which determines ordering in the inbox. For normal SMTP-received email, this represents the time the message was originally accepted by Google, which is more reliable than the `Date` header. However, for API-migrated mail, it can be configured by client to be based on the `Date` header."]
        pub internal_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of IDs of labels applied to this message."]
        pub label_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The parsed email structure in the message parts."]
        pub payload: ::std::option::Option<::std::boxed::Box<MessagePart>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "raw")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entire email message in an RFC 2822 formatted and base64url encoded string. Returned in `messages.get` and `drafts.get` responses when the `format=RAW` parameter is supplied."]
        pub raw: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sizeEstimate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Estimated size in bytes of the message."]
        pub size_estimate: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short part of the message text."]
        pub snippet: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threadId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the thread the message belongs to. To add a message or draft to a thread, the following criteria must be met: 1. The requested `threadId` must be specified on the `Message` or `Draft.Message` you supply with your request. 2. The `References` and `In-Reply-To` headers must be set in compliance with the [RFC 2822](https://tools.ietf.org/html/rfc2822) standard. 3. The `Subject` headers must match. "]
        pub thread_id: ::std::option::Option<::std::string::String>,
    }
    impl Message {
        pub fn builder() -> MessageBuilder {
            MessageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single MIME message part."]
    pub struct MessagePart {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "body")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The message part body for this part, which may be empty for container MIME message parts."]
        pub body: ::std::option::Option<::std::boxed::Box<MessagePartBody>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filename")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The filename of the attachment. Only present if this message part represents an attachment."]
        pub filename: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of headers on this message part. For the top-level message part, representing the entire message payload, it will contain the standard RFC 2822 email headers such as `To`, `From`, and `Subject`."]
        pub headers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MessagePartHeader>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The MIME type of the message part."]
        pub mime_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The immutable ID of the message part."]
        pub part_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The child MIME message parts of this part. This only applies to container MIME message parts, for example `multipart/*`. For non- container MIME message part types, such as `text/plain`, this field is empty. For more information, see RFC 1521."]
        pub parts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MessagePart>>>,
    }
    impl MessagePart {
        pub fn builder() -> MessagePartBuilder {
            MessagePartBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The body of a single MIME message part."]
    pub struct MessagePartBody {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attachmentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When present, contains the ID of an external attachment that can be retrieved in a separate `messages.attachments.get` request. When not present, the entire content of the message part body is contained in the data field."]
        pub attachment_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The body data of a MIME message part as a base64url encoded string. May be empty for MIME container types that have no message body or when the body data is sent as a separate attachment. An attachment ID is present if the body data is contained in a separate attachment."]
        pub data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of bytes for the message part data (encoding notwithstanding)."]
        pub size: ::std::option::Option<::std::primitive::i64>,
    }
    impl MessagePartBody {
        pub fn builder() -> MessagePartBodyBuilder {
            MessagePartBodyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct MessagePartHeader {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the header before the `:` separator. For example, `To`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value of the header after the `:` separator. For example, `someuser@example.com`."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl MessagePartHeader {
        pub fn builder() -> MessagePartHeaderBuilder {
            MessagePartHeaderBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ModifyMessageRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addLabelIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of IDs of labels to add to this message."]
        pub add_label_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "removeLabelIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list IDs of labels to remove from this message."]
        pub remove_label_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ModifyMessageRequest {
        pub fn builder() -> ModifyMessageRequestBuilder {
            ModifyMessageRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ModifyThreadRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addLabelIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of IDs of labels to add to this thread."]
        pub add_label_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "removeLabelIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of IDs of labels to remove from this thread."]
        pub remove_label_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ModifyThreadRequest {
        pub fn builder() -> ModifyThreadRequestBuilder {
            ModifyThreadRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "POP settings for an account."]
    pub struct PopSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accessWindow")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range of messages which are accessible via POP."]
        pub access_window: ::std::option::Option<PopSettingsAccessWindowEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disposition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The action that will be executed on a message after it has been fetched via POP."]
        pub disposition: ::std::option::Option<PopSettingsDispositionEnum>,
    }
    impl PopSettings {
        pub fn builder() -> PopSettingsBuilder {
            PopSettingsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The range of messages which are accessible via POP."]
    pub enum PopSettingsAccessWindowEnum {
        #[serde(rename = "accessWindowUnspecified")]
        #[doc = "Unspecified range."]
        AccessWindowUnspecified,
        #[serde(rename = "disabled")]
        #[doc = "Indicates that no messages are accessible via POP."]
        Disabled,
        #[serde(rename = "fromNowOn")]
        #[doc = "Indicates that unfetched messages received after some past point in time are accessible via POP."]
        FromNowOn,
        #[serde(rename = "allMail")]
        #[doc = "Indicates that all unfetched messages are accessible via POP."]
        AllMail,
    }
    impl ::std::default::Default for PopSettingsAccessWindowEnum {
        fn default() -> Self {
            Self::AccessWindowUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The action that will be executed on a message after it has been fetched via POP."]
    pub enum PopSettingsDispositionEnum {
        #[serde(rename = "dispositionUnspecified")]
        #[doc = "Unspecified disposition."]
        DispositionUnspecified,
        #[serde(rename = "leaveInInbox")]
        #[doc = "Leave the message in the `INBOX`."]
        LeaveInInbox,
        #[serde(rename = "archive")]
        #[doc = "Archive the message."]
        Archive,
        #[serde(rename = "trash")]
        #[doc = "Move the message to the `TRASH`."]
        Trash,
        #[serde(rename = "markRead")]
        #[doc = "Leave the message in the `INBOX` and mark it as read."]
        MarkRead,
    }
    impl ::std::default::Default for PopSettingsDispositionEnum {
        fn default() -> Self {
            Self::DispositionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Profile for a Gmail user."]
    pub struct Profile {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emailAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's email address."]
        pub email_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "historyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the mailbox's current history record."]
        pub history_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messagesTotal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of messages in the mailbox."]
        pub messages_total: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threadsTotal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of threads in the mailbox."]
        pub threads_total: ::std::option::Option<::std::primitive::i64>,
    }
    impl Profile {
        pub fn builder() -> ProfileBuilder {
            ProfileBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Settings associated with a send-as alias, which can be either the primary login address associated with the account or a custom \"from\" address. Send-as aliases correspond to the \"Send Mail As\" feature in the web interface."]
    pub struct SendAs {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A name that appears in the \"From:\" header for mail sent using this alias. For custom \"from\" addresses, when this is empty, Gmail will populate the \"From:\" header with the name that is used for the primary address associated with the account. If the admin has disabled the ability for users to update their name format, requests to update this field for the primary login will silently fail."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isDefault")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this address is selected as the default \"From:\" address in situations such as composing a new message or sending a vacation auto-reply. Every Gmail account has exactly one default send-as address, so the only legal value that clients may write to this field is `true`. Changing this from `false` to `true` for an address will result in this field becoming `false` for the other previous default address."]
        pub is_default: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isPrimary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this address is the primary address used to login to the account. Every Gmail account has exactly one primary address, and it cannot be deleted from the collection of send-as aliases. This field is read-only."]
        pub is_primary: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replyToAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional email address that is included in a \"Reply-To:\" header for mail sent using this alias. If this is empty, Gmail will not generate a \"Reply-To:\" header."]
        pub reply_to_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sendAsEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address that appears in the \"From:\" header for mail sent using this alias. This is read-only for all operations except create."]
        pub send_as_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "signature")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional HTML signature that is included in messages composed with this alias in the Gmail web UI."]
        pub signature: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "smtpMsa")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional SMTP service that will be used as an outbound relay for mail sent using this alias. If this is empty, outbound mail will be sent directly from Gmail's servers to the destination SMTP service. This setting only applies to custom \"from\" aliases."]
        pub smtp_msa: ::std::option::Option<::std::boxed::Box<SmtpMsa>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "treatAsAlias")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether Gmail should treat this address as an alias for the user's primary email address. This setting only applies to custom \"from\" aliases."]
        pub treat_as_alias: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verificationStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether this address has been verified for use as a send-as alias. Read-only. This setting only applies to custom \"from\" aliases."]
        pub verification_status: ::std::option::Option<SendAsVerificationStatusEnum>,
    }
    impl SendAs {
        pub fn builder() -> SendAsBuilder {
            SendAsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Indicates whether this address has been verified for use as a send-as alias. Read-only. This setting only applies to custom \"from\" aliases."]
    pub enum SendAsVerificationStatusEnum {
        #[serde(rename = "verificationStatusUnspecified")]
        #[doc = "Unspecified verification status."]
        VerificationStatusUnspecified,
        #[serde(rename = "accepted")]
        #[doc = "The address is ready to use as a send-as alias."]
        Accepted,
        #[serde(rename = "pending")]
        #[doc = "The address is awaiting verification by the owner."]
        Pending,
    }
    impl ::std::default::Default for SendAsVerificationStatusEnum {
        fn default() -> Self {
            Self::VerificationStatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An S/MIME email config."]
    pub struct SmimeInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "encryptedKeyPassword")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Encrypted key password, when key is encrypted."]
        pub encrypted_key_password: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expiration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the certificate expires (in milliseconds since epoch)."]
        pub expiration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The immutable ID for the SmimeInfo."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isDefault")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this SmimeInfo is the default one for this user's send-as address."]
        pub is_default: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "issuerCn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The S/MIME certificate issuer's common name."]
        pub issuer_cn: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pem")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "PEM formatted X509 concatenated certificate string (standard base64 encoding). Format used for returning key, which includes public key as well as certificate chain (not private key)."]
        pub pem: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pkcs12")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "PKCS#12 format containing a single private/public key pair and certificate chain. This format is only accepted from client for creating a new SmimeInfo and is never returned, because the private key is not intended to be exported. PKCS#12 may be encrypted, in which case encryptedKeyPassword should be set appropriately."]
        pub pkcs12: ::std::option::Option<::std::string::String>,
    }
    impl SmimeInfo {
        pub fn builder() -> SmimeInfoBuilder {
            SmimeInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for communication with an SMTP service."]
    pub struct SmtpMsa {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "host")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The hostname of the SMTP service. Required."]
        pub host: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "password")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The password that will be used for authentication with the SMTP service. This is a write-only field that can be specified in requests to create or update SendAs settings; it is never populated in responses."]
        pub password: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "port")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The port of the SMTP service. Required."]
        pub port: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "securityMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The protocol that will be used to secure communication with the SMTP service. Required."]
        pub security_mode: ::std::option::Option<SmtpMsaSecurityModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "username")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The username that will be used for authentication with the SMTP service. This is a write-only field that can be specified in requests to create or update SendAs settings; it is never populated in responses."]
        pub username: ::std::option::Option<::std::string::String>,
    }
    impl SmtpMsa {
        pub fn builder() -> SmtpMsaBuilder {
            SmtpMsaBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The protocol that will be used to secure communication with the SMTP service. Required."]
    pub enum SmtpMsaSecurityModeEnum {
        #[serde(rename = "securityModeUnspecified")]
        #[doc = "Unspecified security mode."]
        SecurityModeUnspecified,
        #[serde(rename = "none")]
        #[doc = "Communication with the remote SMTP service is unsecured. Requires port 25."]
        None,
        #[serde(rename = "ssl")]
        #[doc = "Communication with the remote SMTP service is secured using SSL."]
        Ssl,
        #[serde(rename = "starttls")]
        #[doc = "Communication with the remote SMTP service is secured using STARTTLS."]
        Starttls,
    }
    impl ::std::default::Default for SmtpMsaSecurityModeEnum {
        fn default() -> Self {
            Self::SecurityModeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A collection of messages representing a conversation."]
    pub struct Thread {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "historyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the last history record that modified this thread."]
        pub history_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique ID of the thread."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of messages in the thread."]
        pub messages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Message>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short part of the message text."]
        pub snippet: ::std::option::Option<::std::string::String>,
    }
    impl Thread {
        pub fn builder() -> ThreadBuilder {
            ThreadBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Vacation auto-reply settings for an account. These settings correspond to the \"Vacation responder\" feature in the web interface."]
    pub struct VacationSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableAutoReply")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Flag that controls whether Gmail automatically replies to messages."]
        pub enable_auto_reply: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional end time for sending auto-replies (epoch ms). When this is specified, Gmail will automatically reply only to messages that it receives before the end time. If both `startTime` and `endTime` are specified, `startTime` must precede `endTime`."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseBodyHtml")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Response body in HTML format. Gmail will sanitize the HTML before storing it. If both `response_body_plain_text` and `response_body_html` are specified, `response_body_html` will be used."]
        pub response_body_html: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseBodyPlainText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Response body in plain text format. If both `response_body_plain_text` and `response_body_html` are specified, `response_body_html` will be used."]
        pub response_body_plain_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseSubject")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional text to prepend to the subject line in vacation responses. In order to enable auto-replies, either the response subject or the response body must be nonempty."]
        pub response_subject: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "restrictToContacts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Flag that determines whether responses are sent to recipients who are not in the user's list of contacts."]
        pub restrict_to_contacts: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "restrictToDomain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Flag that determines whether responses are sent to recipients who are outside of the user's domain. This feature is only available for G Suite users."]
        pub restrict_to_domain: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional start time for sending auto-replies (epoch ms). When this is specified, Gmail will automatically reply only to messages that it receives after the start time. If both `startTime` and `endTime` are specified, `startTime` must precede `endTime`."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl VacationSettings {
        pub fn builder() -> VacationSettingsBuilder {
            VacationSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set up or update a new push notification watch on this user's mailbox."]
    pub struct WatchRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelFilterAction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Filtering behavior of labelIds list specified."]
        pub label_filter_action: ::std::option::Option<WatchRequestLabelFilterActionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of label_ids to restrict notifications about. By default, if unspecified, all changes are pushed out. If specified then dictates which labels are required for a push notification to be generated."]
        pub label_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topicName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A fully qualified Google Cloud Pub/Sub API topic name to publish the events to. This topic name **must** already exist in Cloud Pub/Sub and you **must** have already granted gmail \"publish\" permission on it. For example, \"projects/my-project-identifier/topics/my-topic-name\" (using the Cloud Pub/Sub \"v1\" topic naming format). Note that the \"my-project-identifier\" portion must exactly match your Google developer project id (the one executing this watch request)."]
        pub topic_name: ::std::option::Option<::std::string::String>,
    }
    impl WatchRequest {
        pub fn builder() -> WatchRequestBuilder {
            WatchRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Filtering behavior of labelIds list specified."]
    pub enum WatchRequestLabelFilterActionEnum {
        #[serde(rename = "include")]
        #[doc = "Only get push notifications for message changes relating to labelIds specified."]
        Include,
        #[serde(rename = "exclude")]
        #[doc = "Get push notifications for all message changes except those relating to labelIds specified."]
        Exclude,
    }
    impl ::std::default::Default for WatchRequestLabelFilterActionEnum {
        fn default() -> Self {
            Self::Include
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Push notification watch response."]
    pub struct WatchResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expiration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When Gmail will stop sending notifications for mailbox updates (epoch millis). Call `watch` again before this time to renew the watch."]
        pub expiration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "historyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the mailbox's current history record."]
        pub history_id: ::std::option::Option<::std::string::String>,
    }
    impl WatchResponse {
        pub fn builder() -> WatchResponseBuilder {
            WatchResponseBuilder::default()
        }
    }
}
