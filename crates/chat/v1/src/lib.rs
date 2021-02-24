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
    pub mod dms {
        pub mod methods {
            pub mod messages {
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
                    #[serde(rename = "threadKey")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Opaque thread identifier string that can be specified to group messages into a single thread. If this is the first message with a given thread identifier, a new thread is created. Subsequent messages with the same thread identifier will be posted into the same thread. This relieves bots and webhooks from having to store the Hangouts Chat thread ID of a thread (created earlier by them) to post further updates to it. Has no effect if thread field, corresponding to an existing thread, is set in message."]
                    pub thread_key: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod webhooks {
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
                    #[serde(rename = "threadKey")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Opaque thread identifier string that can be specified to group messages into a single thread. If this is the first message with a given thread identifier, a new thread is created. Subsequent messages with the same thread identifier will be posted into the same thread. This relieves bots and webhooks from having to store the Hangouts Chat thread ID of a thread (created earlier by them) to post further updates to it. Has no effect if thread field, corresponding to an existing thread, is set in message."]
                    pub thread_key: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
        pub mod resources {
            pub mod conversations {
                pub mod methods {
                    pub mod messages {
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
                            #[serde(rename = "threadKey")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Opaque thread identifier string that can be specified to group messages into a single thread. If this is the first message with a given thread identifier, a new thread is created. Subsequent messages with the same thread identifier will be posted into the same thread. This relieves bots and webhooks from having to store the Hangouts Chat thread ID of a thread (created earlier by them) to post further updates to it. Has no effect if thread field, corresponding to an existing thread, is set in message."]
                            pub thread_key: ::std::option::Option<::std::string::String>,
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
    pub mod rooms {
        pub mod methods {
            pub mod messages {
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
                    #[serde(rename = "threadKey")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Opaque thread identifier string that can be specified to group messages into a single thread. If this is the first message with a given thread identifier, a new thread is created. Subsequent messages with the same thread identifier will be posted into the same thread. This relieves bots and webhooks from having to store the Hangouts Chat thread ID of a thread (created earlier by them) to post further updates to it. Has no effect if thread field, corresponding to an existing thread, is set in message."]
                    pub thread_key: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod webhooks {
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
                    #[serde(rename = "threadKey")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Opaque thread identifier string that can be specified to group messages into a single thread. If this is the first message with a given thread identifier, a new thread is created. Subsequent messages with the same thread identifier will be posted into the same thread. This relieves bots and webhooks from having to store the Hangouts Chat thread ID of a thread (created earlier by them) to post further updates to it. Has no effect if thread field, corresponding to an existing thread, is set in message."]
                    pub thread_key: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
        pub mod resources {
            pub mod conversations {
                pub mod methods {
                    pub mod messages {
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
                            #[serde(rename = "threadKey")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Opaque thread identifier string that can be specified to group messages into a single thread. If this is the first message with a given thread identifier, a new thread is created. Subsequent messages with the same thread identifier will be posted into the same thread. This relieves bots and webhooks from having to store the Hangouts Chat thread ID of a thread (created earlier by them) to post further updates to it. Has no effect if thread field, corresponding to an existing thread, is set in message."]
                            pub thread_key: ::std::option::Option<::std::string::String>,
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
    pub mod spaces {
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
                    #[doc = "Requested page size. The value is capped at 1000. Server may return fewer results than requested. If unspecified, server will default to 100."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A token identifying a page of results the server should return."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod webhooks {
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
                    #[serde(rename = "threadKey")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Opaque thread identifier string that can be specified to group messages into a single thread. If this is the first message with a given thread identifier, a new thread is created. Subsequent messages with the same thread identifier will be posted into the same thread. This relieves bots and webhooks from having to store the Hangouts Chat thread ID of a thread (created earlier by them) to post further updates to it. Has no effect if thread field, corresponding to an existing thread, is set in message."]
                    pub thread_key: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
        pub mod resources {
            pub mod members {
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
                            #[doc = "Requested page size. The value is capped at 1000. Server may return fewer results than requested. If unspecified, server will default to 100."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A token identifying a page of results the server should return."]
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
            pub mod messages {
                pub mod methods {
                    pub mod create {
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
                            #[serde(rename = "threadKey")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Opaque thread identifier string that can be specified to group messages into a single thread. If this is the first message with a given thread identifier, a new thread is created. Subsequent messages with the same thread identifier will be posted into the same thread. This relieves bots and webhooks from having to store the Hangouts Chat thread ID of a thread (created earlier by them) to post further updates to it. Has no effect if thread field, corresponding to an existing thread, is set in message."]
                            pub thread_key: ::std::option::Option<::std::string::String>,
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. The field paths to be updated, comma separated if there are multiple. Currently supported field paths: * text * cards"]
                            pub update_mask: ::std::option::Option<::std::string::String>,
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
    #[doc = "List of string parameters to supply when the action method is invoked. For example, consider three snooze buttons: snooze now, snooze 1 day, snooze next week. You might use action method = snooze(), passing the snooze type and snooze time in the list of string parameters."]
    pub struct ActionParameter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the parameter for the action script."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value of the parameter."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl ActionParameter {
        pub fn builder() -> ActionParameterBuilder {
            ActionParameterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parameters that a bot can use to configure how it's response is posted."]
    pub struct ActionResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of bot response."]
        pub _type: ::std::option::Option<ActionResponseTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL for users to auth or config. (Only for REQUEST_CONFIG response types.)"]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl ActionResponse {
        pub fn builder() -> ActionResponseBuilder {
            ActionResponseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of bot response."]
    pub enum ActionResponseTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "Default type; will be handled as NEW_MESSAGE."]
        TypeUnspecified,
        #[serde(rename = "NEW_MESSAGE")]
        #[doc = "Post as a new message in the topic."]
        NewMessage,
        #[serde(rename = "UPDATE_MESSAGE")]
        #[doc = "Update the bot's own message. (Only after CARD_CLICKED events.)"]
        UpdateMessage,
        #[serde(rename = "REQUEST_CONFIG")]
        #[doc = "Privately ask the user for additional auth or config."]
        RequestConfig,
    }
    impl ::std::default::Default for ActionResponseTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotations associated with the plain-text body of the message. Example plain-text message body: ``` Hello @FooBot how are you!\" ``` The corresponding annotations metadata: ``` \"annotations\":[{ \"type\":\"USER_MENTION\", \"startIndex\":6, \"length\":7, \"userMention\": { \"user\": { \"name\":\"users/107946847022116401880\", \"displayName\":\"FooBot\", \"avatarUrl\":\"https://goo.gl/aeDtrS\", \"type\":\"BOT\" }, \"type\":\"MENTION\" } }] ```"]
    pub struct Annotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "length")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Length of the substring in the plain-text message body this annotation corresponds to."]
        pub length: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "slashCommand")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metadata for a slash command."]
        pub slash_command: ::std::option::Option<::std::boxed::Box<SlashCommandMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Start index (0-based, inclusive) in the plain-text message body this annotation corresponds to."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of this annotation."]
        pub _type: ::std::option::Option<AnnotationTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userMention")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metadata of user mention."]
        pub user_mention: ::std::option::Option<::std::boxed::Box<UserMentionMetadata>>,
    }
    impl Annotation {
        pub fn builder() -> AnnotationBuilder {
            AnnotationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of this annotation."]
    pub enum AnnotationTypeEnum {
        #[serde(rename = "ANNOTATION_TYPE_UNSPECIFIED")]
        #[doc = "Default value for the enum. DO NOT USE."]
        AnnotationTypeUnspecified,
        #[serde(rename = "USER_MENTION")]
        #[doc = "A user is mentioned."]
        UserMention,
        #[serde(rename = "SLASH_COMMAND")]
        #[doc = "A slash command is invoked."]
        SlashCommand,
    }
    impl ::std::default::Default for AnnotationTypeEnum {
        fn default() -> Self {
            Self::AnnotationTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An attachment in Hangouts Chat."]
    pub struct Attachment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attachmentDataRef")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reference to the attachment data. This is used with the media API to download the attachment data."]
        pub attachment_data_ref: ::std::option::Option<::std::boxed::Box<AttachmentDataRef>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The original file name for the content, not the full path."]
        pub content_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The content type (MIME type) of the file."]
        pub content_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "downloadUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The download URL which should be used to allow a human user to download the attachment. Bots should not use this URL to download attachment content."]
        pub download_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "driveDataRef")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reference to the drive attachment. This is used with the Drive API."]
        pub drive_data_ref: ::std::option::Option<::std::boxed::Box<DriveDataRef>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name of the attachment, in the form \"spaces/*/messages/*/attachments/*\"."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source of the attachment."]
        pub source: ::std::option::Option<AttachmentSourceEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnailUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The thumbnail URL which should be used to preview the attachment to a human user. Bots should not use this URL to download attachment content."]
        pub thumbnail_uri: ::std::option::Option<::std::string::String>,
    }
    impl Attachment {
        pub fn builder() -> AttachmentBuilder {
            AttachmentBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The source of the attachment."]
    pub enum AttachmentSourceEnum {
        #[serde(rename = "SOURCE_UNSPECIFIED")]
        #[doc = ""]
        SourceUnspecified,
        #[serde(rename = "DRIVE_FILE")]
        #[doc = ""]
        DriveFile,
        #[serde(rename = "UPLOADED_CONTENT")]
        #[doc = ""]
        UploadedContent,
    }
    impl ::std::default::Default for AttachmentSourceEnum {
        fn default() -> Self {
            Self::SourceUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A reference to the data of an attachment."]
    pub struct AttachmentDataRef {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the attachment data. This is used with the media API to download the attachment data."]
        pub resource_name: ::std::option::Option<::std::string::String>,
    }
    impl AttachmentDataRef {
        pub fn builder() -> AttachmentDataRefBuilder {
            AttachmentDataRefBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A button. Can be a text button or an image button."]
    pub struct Button {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageButton")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A button with image and onclick action."]
        pub image_button: ::std::option::Option<::std::boxed::Box<ImageButton>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textButton")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A button with text and onclick action."]
        pub text_button: ::std::option::Option<::std::boxed::Box<TextButton>>,
    }
    impl Button {
        pub fn builder() -> ButtonBuilder {
            ButtonBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A card is a UI element that can contain UI widgets such as texts, images."]
    pub struct Card {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cardActions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The actions of this card."]
        pub card_actions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CardAction>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "header")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The header of the card. A header usually contains a title and an image."]
        pub header: ::std::option::Option<::std::boxed::Box<CardHeader>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the card."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sections")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sections are separated by a line divider."]
        pub sections: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Section>>>,
    }
    impl Card {
        pub fn builder() -> CardBuilder {
            CardBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A card action is the action associated with the card. For an invoice card, a typical action would be: delete invoice, email invoice or open the invoice in browser."]
    pub struct CardAction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actionLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The label used to be displayed in the action menu item."]
        pub action_label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "onClick")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The onclick action for this action item."]
        pub on_click: ::std::option::Option<::std::boxed::Box<OnClick>>,
    }
    impl CardAction {
        pub fn builder() -> CardActionBuilder {
            CardActionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct CardHeader {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The image's type (e.g. square border or circular border)."]
        pub image_style: ::std::option::Option<CardHeaderImageStyleEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the image in the card header."]
        pub image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subtitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The subtitle of the card header."]
        pub subtitle: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title must be specified. The header has a fixed height: if both a title and subtitle is specified, each will take up 1 line. If only the title is specified, it will take up both lines."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl CardHeader {
        pub fn builder() -> CardHeaderBuilder {
            CardHeaderBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The image's type (e.g. square border or circular border)."]
    pub enum CardHeaderImageStyleEnum {
        #[serde(rename = "IMAGE_STYLE_UNSPECIFIED")]
        #[doc = ""]
        ImageStyleUnspecified,
        #[serde(rename = "IMAGE")]
        #[doc = "Square border."]
        Image,
        #[serde(rename = "AVATAR")]
        #[doc = "Circular border."]
        Avatar,
    }
    impl ::std::default::Default for CardHeaderImageStyleEnum {
        fn default() -> Self {
            Self::ImageStyleUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Google Chat events."]
    pub struct DeprecatedEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "action")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The form action data associated with an interactive card that was clicked. Only populated for CARD_CLICKED events. See the [Interactive Cards guide](/hangouts/chat/how-tos/cards-onclick) for more information."]
        pub action: ::std::option::Option<::std::boxed::Box<FormAction>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "configCompleteRedirectUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL the bot should redirect the user to after they have completed an authorization or configuration flow outside of Google Chat. See the [Authorizing access to 3p services guide](/hangouts/chat/how-tos/auth-3p) for more information."]
        pub config_complete_redirect_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp indicating when the event was dispatched."]
        pub event_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The message that triggered the event, if applicable."]
        pub message: ::std::option::Option<::std::boxed::Box<Message>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "space")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The room or DM in which the event occurred."]
        pub space: ::std::option::Option<::std::boxed::Box<Space>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threadKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bot-defined key for the thread related to the event. See the thread_key field of the `spaces.message.create` request for more information."]
        pub thread_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "token")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A secret value that bots can use to verify if a request is from Google. The token is randomly generated by Google, remains static, and can be obtained from the Google Chat API configuration page in the Cloud Console. Developers can revoke/regenerate it if needed from the same page."]
        pub token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the event."]
        pub _type: ::std::option::Option<DeprecatedEventTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "user")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user that triggered the event."]
        pub user: ::std::option::Option<::std::boxed::Box<User>>,
    }
    impl DeprecatedEvent {
        pub fn builder() -> DeprecatedEventBuilder {
            DeprecatedEventBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the event."]
    pub enum DeprecatedEventTypeEnum {
        #[serde(rename = "UNSPECIFIED")]
        #[doc = "Default value for the enum. DO NOT USE."]
        Unspecified,
        #[serde(rename = "MESSAGE")]
        #[doc = "A message was sent in a room or direct message."]
        Message,
        #[serde(rename = "ADDED_TO_SPACE")]
        #[doc = "The bot was added to a room or DM."]
        AddedToSpace,
        #[serde(rename = "REMOVED_FROM_SPACE")]
        #[doc = "The bot was removed from a room or DM."]
        RemovedFromSpace,
        #[serde(rename = "CARD_CLICKED")]
        #[doc = "The bot's interactive card was clicked."]
        CardClicked,
    }
    impl ::std::default::Default for DeprecatedEventTypeEnum {
        fn default() -> Self {
            Self::Unspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A reference to the data of a drive attachment."]
    pub struct DriveDataRef {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "driveFileId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The id for the drive file, for use with the Drive API."]
        pub drive_file_id: ::std::option::Option<::std::string::String>,
    }
    impl DriveDataRef {
        pub fn builder() -> DriveDataRefBuilder {
            DriveDataRefBuilder::default()
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
    #[doc = "A form action describes the behavior when the form is submitted. For example, an Apps Script can be invoked to handle the form."]
    pub struct FormAction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actionMethodName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The method name is used to identify which part of the form triggered the form submission. This information is echoed back to the bot as part of the card click event. The same method name can be used for several elements that trigger a common behavior if desired."]
        pub action_method_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of action parameters."]
        pub parameters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ActionParameter>>>,
    }
    impl FormAction {
        pub fn builder() -> FormActionBuilder {
            FormActionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An image that is specified by a URL and can have an onclick action."]
    pub struct Image {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aspectRatio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The aspect ratio of this image (width/height). This field allows clients to reserve the right height for the image while waiting for it to load. It's not meant to override the native aspect ratio of the image. If unset, the server fills it by prefetching the image."]
        pub aspect_ratio: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the image."]
        pub image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "onClick")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The onclick action."]
        pub on_click: ::std::option::Option<::std::boxed::Box<OnClick>>,
    }
    impl Image {
        pub fn builder() -> ImageBuilder {
            ImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An image button with an onclick action."]
    pub struct ImageButton {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "icon")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The icon specified by an enum that indices to an icon provided by Chat API."]
        pub icon: ::std::option::Option<ImageButtonIconEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iconUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The icon specified by a URL."]
        pub icon_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of this image_button which will be used for accessibility. Default value will be provided if developers don't specify."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "onClick")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The onclick action."]
        pub on_click: ::std::option::Option<::std::boxed::Box<OnClick>>,
    }
    impl ImageButton {
        pub fn builder() -> ImageButtonBuilder {
            ImageButtonBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The icon specified by an enum that indices to an icon provided by Chat API."]
    pub enum ImageButtonIconEnum {
        #[serde(rename = "ICON_UNSPECIFIED")]
        #[doc = ""]
        IconUnspecified,
        #[serde(rename = "AIRPLANE")]
        #[doc = ""]
        Airplane,
        #[serde(rename = "BOOKMARK")]
        #[doc = ""]
        Bookmark,
        #[serde(rename = "BUS")]
        #[doc = ""]
        Bus,
        #[serde(rename = "CAR")]
        #[doc = ""]
        Car,
        #[serde(rename = "CLOCK")]
        #[doc = ""]
        Clock,
        #[serde(rename = "CONFIRMATION_NUMBER_ICON")]
        #[doc = ""]
        ConfirmationNumberIcon,
        #[serde(rename = "DOLLAR")]
        #[doc = ""]
        Dollar,
        #[serde(rename = "DESCRIPTION")]
        #[doc = ""]
        Description,
        #[serde(rename = "EMAIL")]
        #[doc = ""]
        Email,
        #[serde(rename = "EVENT_PERFORMER")]
        #[doc = ""]
        EventPerformer,
        #[serde(rename = "EVENT_SEAT")]
        #[doc = ""]
        EventSeat,
        #[serde(rename = "FLIGHT_ARRIVAL")]
        #[doc = ""]
        FlightArrival,
        #[serde(rename = "FLIGHT_DEPARTURE")]
        #[doc = ""]
        FlightDeparture,
        #[serde(rename = "HOTEL")]
        #[doc = ""]
        Hotel,
        #[serde(rename = "HOTEL_ROOM_TYPE")]
        #[doc = ""]
        HotelRoomType,
        #[serde(rename = "INVITE")]
        #[doc = ""]
        Invite,
        #[serde(rename = "MAP_PIN")]
        #[doc = ""]
        MapPin,
        #[serde(rename = "MEMBERSHIP")]
        #[doc = ""]
        Membership,
        #[serde(rename = "MULTIPLE_PEOPLE")]
        #[doc = ""]
        MultiplePeople,
        #[serde(rename = "OFFER")]
        #[doc = ""]
        Offer,
        #[serde(rename = "PERSON")]
        #[doc = ""]
        Person,
        #[serde(rename = "PHONE")]
        #[doc = ""]
        Phone,
        #[serde(rename = "RESTAURANT_ICON")]
        #[doc = ""]
        RestaurantIcon,
        #[serde(rename = "SHOPPING_CART")]
        #[doc = ""]
        ShoppingCart,
        #[serde(rename = "STAR")]
        #[doc = ""]
        Star,
        #[serde(rename = "STORE")]
        #[doc = ""]
        Store,
        #[serde(rename = "TICKET")]
        #[doc = ""]
        Ticket,
        #[serde(rename = "TRAIN")]
        #[doc = ""]
        Train,
        #[serde(rename = "VIDEO_CAMERA")]
        #[doc = ""]
        VideoCamera,
        #[serde(rename = "VIDEO_PLAY")]
        #[doc = ""]
        VideoPlay,
    }
    impl ::std::default::Default for ImageButtonIconEnum {
        fn default() -> Self {
            Self::IconUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A UI element contains a key (label) and a value (content). And this element may also contain some actions such as onclick button."]
    pub struct KeyValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bottomLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text of the bottom label. Formatted text supported."]
        pub bottom_label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "button")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A button that can be clicked to trigger an action."]
        pub button: ::std::option::Option<::std::boxed::Box<Button>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text of the content. Formatted text supported and always required."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentMultiline")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the content should be multiline."]
        pub content_multiline: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "icon")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An enum value that will be replaced by the Chat API with the corresponding icon image."]
        pub icon: ::std::option::Option<KeyValueIconEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iconUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The icon specified by a URL."]
        pub icon_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "onClick")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The onclick action. Only the top label, bottom label and content region are clickable."]
        pub on_click: ::std::option::Option<::std::boxed::Box<OnClick>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text of the top label. Formatted text supported."]
        pub top_label: ::std::option::Option<::std::string::String>,
    }
    impl KeyValue {
        pub fn builder() -> KeyValueBuilder {
            KeyValueBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "An enum value that will be replaced by the Chat API with the corresponding icon image."]
    pub enum KeyValueIconEnum {
        #[serde(rename = "ICON_UNSPECIFIED")]
        #[doc = ""]
        IconUnspecified,
        #[serde(rename = "AIRPLANE")]
        #[doc = ""]
        Airplane,
        #[serde(rename = "BOOKMARK")]
        #[doc = ""]
        Bookmark,
        #[serde(rename = "BUS")]
        #[doc = ""]
        Bus,
        #[serde(rename = "CAR")]
        #[doc = ""]
        Car,
        #[serde(rename = "CLOCK")]
        #[doc = ""]
        Clock,
        #[serde(rename = "CONFIRMATION_NUMBER_ICON")]
        #[doc = ""]
        ConfirmationNumberIcon,
        #[serde(rename = "DOLLAR")]
        #[doc = ""]
        Dollar,
        #[serde(rename = "DESCRIPTION")]
        #[doc = ""]
        Description,
        #[serde(rename = "EMAIL")]
        #[doc = ""]
        Email,
        #[serde(rename = "EVENT_PERFORMER")]
        #[doc = ""]
        EventPerformer,
        #[serde(rename = "EVENT_SEAT")]
        #[doc = ""]
        EventSeat,
        #[serde(rename = "FLIGHT_ARRIVAL")]
        #[doc = ""]
        FlightArrival,
        #[serde(rename = "FLIGHT_DEPARTURE")]
        #[doc = ""]
        FlightDeparture,
        #[serde(rename = "HOTEL")]
        #[doc = ""]
        Hotel,
        #[serde(rename = "HOTEL_ROOM_TYPE")]
        #[doc = ""]
        HotelRoomType,
        #[serde(rename = "INVITE")]
        #[doc = ""]
        Invite,
        #[serde(rename = "MAP_PIN")]
        #[doc = ""]
        MapPin,
        #[serde(rename = "MEMBERSHIP")]
        #[doc = ""]
        Membership,
        #[serde(rename = "MULTIPLE_PEOPLE")]
        #[doc = ""]
        MultiplePeople,
        #[serde(rename = "OFFER")]
        #[doc = ""]
        Offer,
        #[serde(rename = "PERSON")]
        #[doc = ""]
        Person,
        #[serde(rename = "PHONE")]
        #[doc = ""]
        Phone,
        #[serde(rename = "RESTAURANT_ICON")]
        #[doc = ""]
        RestaurantIcon,
        #[serde(rename = "SHOPPING_CART")]
        #[doc = ""]
        ShoppingCart,
        #[serde(rename = "STAR")]
        #[doc = ""]
        Star,
        #[serde(rename = "STORE")]
        #[doc = ""]
        Store,
        #[serde(rename = "TICKET")]
        #[doc = ""]
        Ticket,
        #[serde(rename = "TRAIN")]
        #[doc = ""]
        Train,
        #[serde(rename = "VIDEO_CAMERA")]
        #[doc = ""]
        VideoCamera,
        #[serde(rename = "VIDEO_PLAY")]
        #[doc = ""]
        VideoPlay,
    }
    impl ::std::default::Default for KeyValueIconEnum {
        fn default() -> Self {
            Self::IconUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ListMembershipsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "memberships")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of memberships in the requested (or first) page."]
        pub memberships: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Membership>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token to retrieve the next page of results. It will be empty for the last page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListMembershipsResponse {
        pub fn builder() -> ListMembershipsResponseBuilder {
            ListMembershipsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ListSpacesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token to retrieve the next page of results. It will be empty for the last page of results. Tokens expire in an hour. An error is thrown if an expired token is passed."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spaces")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of spaces in the requested (or first) page."]
        pub spaces: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Space>>>,
    }
    impl ListSpacesResponse {
        pub fn builder() -> ListSpacesResponseBuilder {
            ListSpacesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Media resource."]
    pub struct Media {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the media resource."]
        pub resource_name: ::std::option::Option<::std::string::String>,
    }
    impl Media {
        pub fn builder() -> MediaBuilder {
            MediaBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a membership relation in Hangouts Chat."]
    pub struct Membership {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creation time of the membership a.k.a the time at which the member joined the space, if applicable."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "member")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A User in Hangout Chat"]
        pub member: ::std::option::Option<::std::boxed::Box<User>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "State of the membership."]
        pub state: ::std::option::Option<MembershipStateEnum>,
    }
    impl Membership {
        pub fn builder() -> MembershipBuilder {
            MembershipBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "State of the membership."]
    pub enum MembershipStateEnum {
        #[serde(rename = "MEMBERSHIP_STATE_UNSPECIFIED")]
        #[doc = "Default, do not use."]
        MembershipStateUnspecified,
        #[serde(rename = "JOINED")]
        #[doc = "The user has joined the space."]
        Joined,
        #[serde(rename = "INVITED")]
        #[doc = "The user has been invited, is able to join the space, but currently has not joined."]
        Invited,
        #[serde(rename = "NOT_A_MEMBER")]
        #[doc = "The user is not a member of the space, has not been invited and is not able to join the space."]
        NotAMember,
    }
    impl ::std::default::Default for MembershipStateEnum {
        fn default() -> Self {
            Self::MembershipStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A message in Hangouts Chat."]
    pub struct Message {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actionResponse")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Input only. Parameters that a bot can use to configure how its response is posted."]
        pub action_response: ::std::option::Option<::std::boxed::Box<ActionResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Annotations associated with the text in this message."]
        pub annotations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Annotation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "argumentText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Plain-text body of the message with all bot mentions stripped out."]
        pub argument_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attachment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User uploaded attachment."]
        pub attachment: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Attachment>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cards")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rich, formatted and interactive cards that can be used to display UI elements such as: formatted texts, buttons, clickable images. Cards are normally displayed below the plain-text body of the message."]
        pub cards: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Card>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time at which the message was created in Hangouts Chat server."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fallbackText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A plain-text description of the message's cards, used when the actual cards cannot be displayed (e.g. mobile notifications)."]
        pub fallback_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name, in the form \"spaces/*/messages/*\". Example: spaces/AAAAMpdlehY/messages/UMxbHmzDlr4.UMxbHmzDlr4"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previewText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text for generating preview chips. This text will not be displayed to the user, but any links to images, web pages, videos, etc. included here will generate preview chips."]
        pub preview_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sender")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user who created the message."]
        pub sender: ::std::option::Option<::std::boxed::Box<User>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "slashCommand")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Slash command information, if applicable."]
        pub slash_command: ::std::option::Option<::std::boxed::Box<SlashCommand>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "space")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The space the message belongs to."]
        pub space: ::std::option::Option<::std::boxed::Box<Space>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Plain-text body of the message."]
        pub text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thread")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The thread the message belongs to."]
        pub thread: ::std::option::Option<::std::boxed::Box<Thread>>,
    }
    impl Message {
        pub fn builder() -> MessageBuilder {
            MessageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An onclick action (e.g. open a link)."]
    pub struct OnClick {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "action")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A form action will be triggered by this onclick if specified."]
        pub action: ::std::option::Option<::std::boxed::Box<FormAction>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "openLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This onclick triggers an open link action if specified."]
        pub open_link: ::std::option::Option<::std::boxed::Box<OpenLink>>,
    }
    impl OnClick {
        pub fn builder() -> OnClickBuilder {
            OnClickBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A link that opens a new window."]
    pub struct OpenLink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL to open."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl OpenLink {
        pub fn builder() -> OpenLinkBuilder {
            OpenLinkBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A section contains a collection of widgets that are rendered (vertically) in the order that they are specified. Across all platforms, cards have a narrow fixed width, so there is currently no need for layout properties (e.g. float)."]
    pub struct Section {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "header")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The header of the section, text formatted supported."]
        pub header: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "widgets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A section must contain at least 1 widget."]
        pub widgets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WidgetMarkup>>>,
    }
    impl Section {
        pub fn builder() -> SectionBuilder {
            SectionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Slash Command in Hangouts Chat."]
    pub struct SlashCommand {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commandId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The id of the slash command invoked."]
        pub command_id: ::std::option::Option<::std::string::String>,
    }
    impl SlashCommand {
        pub fn builder() -> SlashCommandBuilder {
            SlashCommandBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotation metadata for slash commands (/)."]
    pub struct SlashCommandMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bot")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bot whose command was invoked."]
        pub bot: ::std::option::Option<::std::boxed::Box<User>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commandId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The command id of the invoked slash command."]
        pub command_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commandName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the invoked slash command."]
        pub command_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "triggersDialog")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicating whether the slash command is for a dialog."]
        pub triggers_dialog: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of slash command."]
        pub _type: ::std::option::Option<SlashCommandMetadataTypeEnum>,
    }
    impl SlashCommandMetadata {
        pub fn builder() -> SlashCommandMetadataBuilder {
            SlashCommandMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of slash command."]
    pub enum SlashCommandMetadataTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "Default value for the enum. DO NOT USE."]
        TypeUnspecified,
        #[serde(rename = "ADD")]
        #[doc = "Add bot to space."]
        Add,
        #[serde(rename = "INVOKE")]
        #[doc = "Invoke slash command in space."]
        Invoke,
    }
    impl ::std::default::Default for SlashCommandMetadataTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A room or DM in Hangouts Chat."]
    pub struct Space {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The display name (only if the space is a room). Please note that this field might not be populated in direct messages between humans."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name of the space, in the form \"spaces/*\". Example: spaces/AAAAMpdlehYs"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "singleUserBotDm")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the space is a DM between a bot and a single human."]
        pub single_user_bot_dm: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threaded")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the messages are threaded in this space."]
        pub threaded: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The type of a space. This is deprecated. Use `single_user_bot_dm` instead."]
        pub _type: ::std::option::Option<SpaceTypeEnum>,
    }
    impl Space {
        pub fn builder() -> SpaceBuilder {
            SpaceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The type of a space. This is deprecated. Use `single_user_bot_dm` instead."]
    pub enum SpaceTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = ""]
        TypeUnspecified,
        #[serde(rename = "ROOM")]
        #[doc = "Multi-user spaces such as rooms and DMs between humans."]
        Room,
        #[serde(rename = "DM")]
        #[doc = "1:1 Direct Message between a human and a bot, where all messages are flat."]
        Dm,
    }
    impl ::std::default::Default for SpaceTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A button with text and onclick action."]
    pub struct TextButton {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "onClick")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The onclick action of the button."]
        pub on_click: ::std::option::Option<::std::boxed::Box<OnClick>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text of the button."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl TextButton {
        pub fn builder() -> TextButtonBuilder {
            TextButtonBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A paragraph of text. Formatted text supported."]
    pub struct TextParagraph {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl TextParagraph {
        pub fn builder() -> TextParagraphBuilder {
            TextParagraphBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A thread in Hangouts Chat."]
    pub struct Thread {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name, in the form \"spaces/*/threads/*\". Example: spaces/AAAAMpdlehY/threads/UMxbHmzDlr4"]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl Thread {
        pub fn builder() -> ThreadBuilder {
            ThreadBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A user in Hangouts Chat."]
    pub struct User {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's display name."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domainId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Obfuscated domain information."]
        pub domain_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isAnonymous")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True when the user is deleted or the user's proifle is not visible."]
        pub is_anonymous: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name, in the format \"users/*\"."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User type."]
        pub _type: ::std::option::Option<UserTypeEnum>,
    }
    impl User {
        pub fn builder() -> UserBuilder {
            UserBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "User type."]
    pub enum UserTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "Default value for the enum. DO NOT USE."]
        TypeUnspecified,
        #[serde(rename = "HUMAN")]
        #[doc = "Human user."]
        Human,
        #[serde(rename = "BOT")]
        #[doc = "Bot user."]
        Bot,
    }
    impl ::std::default::Default for UserTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotation metadata for user mentions (@)."]
    pub struct UserMentionMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of user mention."]
        pub _type: ::std::option::Option<UserMentionMetadataTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "user")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user mentioned."]
        pub user: ::std::option::Option<::std::boxed::Box<User>>,
    }
    impl UserMentionMetadata {
        pub fn builder() -> UserMentionMetadataBuilder {
            UserMentionMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of user mention."]
    pub enum UserMentionMetadataTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "Default value for the enum. DO NOT USE."]
        TypeUnspecified,
        #[serde(rename = "ADD")]
        #[doc = "Add user to space."]
        Add,
        #[serde(rename = "MENTION")]
        #[doc = "Mention user in space."]
        Mention,
    }
    impl ::std::default::Default for UserMentionMetadataTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A widget is a UI element that presents texts, images, etc."]
    pub struct WidgetMarkup {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buttons")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of buttons. Buttons is also oneof data and only one of these fields should be set."]
        pub buttons: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Button>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display an image in this widget."]
        pub image: ::std::option::Option<::std::boxed::Box<Image>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display a key value item in this widget."]
        pub key_value: ::std::option::Option<::std::boxed::Box<KeyValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textParagraph")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display a text paragraph in this widget."]
        pub text_paragraph: ::std::option::Option<::std::boxed::Box<TextParagraph>>,
    }
    impl WidgetMarkup {
        pub fn builder() -> WidgetMarkupBuilder {
            WidgetMarkupBuilder::default()
        }
    }
}
