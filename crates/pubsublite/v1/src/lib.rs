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
    pub mod admin {
        pub mod resources {
            pub mod projects {
                pub mod resources {
                    pub mod locations {
                        pub mod resources {
                            pub mod subscriptions {
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
                                            #[serde(rename = "subscriptionId")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Required. The ID to use for the subscription, which will become the final component of the subscription's name. This value is structured like: `my-sub-name`."]
                                            pub subscription_id:
                                                ::std::option::Option<::std::string::String>,
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
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The maximum number of subscriptions to return. The service may return fewer than this value. If unset or zero, all subscriptions for the parent will be returned."]
                                            pub page_size:
                                                ::std::option::Option<::std::primitive::i64>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "A page token, received from a previous `ListSubscriptions` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListSubscriptions` must match the call that provided the page token."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
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
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "updateMask")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Required. A mask specifying the subscription fields to change."]
                                            pub update_mask:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                }
                            }
                            pub mod topics {
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
                                            #[serde(rename = "topicId")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Required. The ID to use for the topic, which will become the final component of the topic's name. This value is structured like: `my-topic-name`."]
                                            pub topic_id:
                                                ::std::option::Option<::std::string::String>,
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
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The maximum number of topics to return. The service may return fewer than this value. If unset or zero, all topics for the parent will be returned."]
                                            pub page_size:
                                                ::std::option::Option<::std::primitive::i64>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "A page token, received from a previous `ListTopics` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListTopics` must match the call that provided the page token."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
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
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "updateMask")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Required. A mask specifying the topic fields to change."]
                                            pub update_mask:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                }
                                pub mod resources {
                                    pub mod subscriptions {
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
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "The maximum number of subscriptions to return. The service may return fewer than this value. If unset or zero, all subscriptions for the given topic will be returned."]
                                                    pub page_size: ::std::option::Option<
                                                        ::std::primitive::i64,
                                                    >,
                                                    #[builder(
                                                        default = "{ ::std::default::Default::default() }",
                                                        setter(into)
                                                    )]
                                                    #[serde(rename = "pageToken")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "A page token, received from a previous `ListTopicSubscriptions` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListTopicSubscriptions` must match the call that provided the page token."]
                                                    pub page_token: ::std::option::Option<
                                                        ::std::string::String,
                                                    >,
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
                    }
                }
            }
        }
    }
    pub mod cursor {
        pub mod resources {
            pub mod projects {
                pub mod resources {
                    pub mod locations {
                        pub mod resources {
                            pub mod subscriptions {
                                pub mod resources {
                                    pub mod cursors {
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
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "The maximum number of cursors to return. The service may return fewer than this value. If unset or zero, all cursors for the parent will be returned."]
                                                    pub page_size: ::std::option::Option<
                                                        ::std::primitive::i64,
                                                    >,
                                                    #[builder(
                                                        default = "{ ::std::default::Default::default() }",
                                                        setter(into)
                                                    )]
                                                    #[serde(rename = "pageToken")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "A page token, received from a previous `ListPartitionCursors` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListPartitionCursors` must match the call that provided the page token."]
                                                    pub page_token: ::std::option::Option<
                                                        ::std::string::String,
                                                    >,
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
    #[doc = "The throughput capacity configuration for each partition."]
    pub struct Capacity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishMibPerSec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Publish throughput capacity per partition in MiB/s. Must be >= 4 and <= 16."]
        pub publish_mib_per_sec: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subscribeMibPerSec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Subscribe throughput capacity per partition in MiB/s. Must be >= 4 and <= 32."]
        pub subscribe_mib_per_sec: ::std::option::Option<::std::primitive::i64>,
    }
    impl Capacity {
        pub fn builder() -> CapacityBuilder {
            CapacityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Compute the current head cursor for a partition."]
    pub struct ComputeHeadCursorRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The partition for which we should compute the head cursor."]
        pub partition: ::std::option::Option<::std::string::String>,
    }
    impl ComputeHeadCursorRequest {
        pub fn builder() -> ComputeHeadCursorRequestBuilder {
            ComputeHeadCursorRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response containing the head cursor for the requested topic and partition."]
    pub struct ComputeHeadCursorResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headCursor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The head cursor."]
        pub head_cursor: ::std::option::Option<::std::boxed::Box<Cursor>>,
    }
    impl ComputeHeadCursorResponse {
        pub fn builder() -> ComputeHeadCursorResponseBuilder {
            ComputeHeadCursorResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Compute statistics about a range of messages in a given topic and partition."]
    pub struct ComputeMessageStatsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endCursor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The exclusive end of the range. The range is empty if end_cursor <= start_cursor. Specifying a start_cursor before the first message and an end_cursor after the last message will retrieve all messages."]
        pub end_cursor: ::std::option::Option<::std::boxed::Box<Cursor>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The partition for which we should compute message stats."]
        pub partition: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startCursor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The inclusive start of the range."]
        pub start_cursor: ::std::option::Option<::std::boxed::Box<Cursor>>,
    }
    impl ComputeMessageStatsRequest {
        pub fn builder() -> ComputeMessageStatsRequestBuilder {
            ComputeMessageStatsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response containing stats for messages in the requested topic and partition."]
    pub struct ComputeMessageStatsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messageBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of quota bytes accounted to these messages."]
        pub message_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messageCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The count of messages."]
        pub message_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minimumEventTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minimum event timestamp across these messages. For the purposes of this computation, if a message does not have an event time, we use the publish time. The timestamp will be unset if there are no messages."]
        pub minimum_event_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minimumPublishTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minimum publish timestamp across these messages. Note that publish timestamps within a partition are not guaranteed to be non-decreasing. The timestamp will be unset if there are no messages."]
        pub minimum_publish_time: ::std::option::Option<::std::string::String>,
    }
    impl ComputeMessageStatsResponse {
        pub fn builder() -> ComputeMessageStatsResponseBuilder {
            ComputeMessageStatsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A cursor that describes the position of a message within a topic partition."]
    pub struct Cursor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The offset of a message within a topic partition. Must be greater than or equal 0."]
        pub offset: ::std::option::Option<::std::string::String>,
    }
    impl Cursor {
        pub fn builder() -> CursorBuilder {
            CursorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The settings for a subscription's message delivery."]
    pub struct DeliveryConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deliveryRequirement")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The DeliveryRequirement for this subscription."]
        pub delivery_requirement: ::std::option::Option<DeliveryConfigDeliveryRequirementEnum>,
    }
    impl DeliveryConfig {
        pub fn builder() -> DeliveryConfigBuilder {
            DeliveryConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The DeliveryRequirement for this subscription."]
    pub enum DeliveryConfigDeliveryRequirementEnum {
        #[serde(rename = "DELIVERY_REQUIREMENT_UNSPECIFIED")]
        #[doc = "Default value. This value is unused."]
        DeliveryRequirementUnspecified,
        #[serde(rename = "DELIVER_IMMEDIATELY")]
        #[doc = "The server does not wait for a published message to be successfully written to storage before delivering it to subscribers."]
        DeliverImmediately,
        #[serde(rename = "DELIVER_AFTER_STORED")]
        #[doc = "The server will not deliver a published message to subscribers until the message has been successfully written to storage. This will result in higher end-to-end latency, but consistent delivery."]
        DeliverAfterStored,
    }
    impl ::std::default::Default for DeliveryConfigDeliveryRequirementEnum {
        fn default() -> Self {
            Self::DeliveryRequirementUnspecified
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
    #[doc = "Response for ListPartitionCursors"]
    pub struct ListPartitionCursorsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partitionCursors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The partition cursors from this request."]
        pub partition_cursors:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PartitionCursor>>>,
    }
    impl ListPartitionCursorsResponse {
        pub fn builder() -> ListPartitionCursorsResponseBuilder {
            ListPartitionCursorsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for ListSubscriptions."]
    pub struct ListSubscriptionsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token that can be sent as `page_token` to retrieve the next page of results. If this field is omitted, there are no more results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subscriptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of subscriptions in the requested parent. The order of the subscriptions is unspecified."]
        pub subscriptions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Subscription>>>,
    }
    impl ListSubscriptionsResponse {
        pub fn builder() -> ListSubscriptionsResponseBuilder {
            ListSubscriptionsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for ListTopicSubscriptions."]
    pub struct ListTopicSubscriptionsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token that can be sent as `page_token` to retrieve the next page of results. If this field is omitted, there are no more results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subscriptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The names of subscriptions attached to the topic. The order of the subscriptions is unspecified."]
        pub subscriptions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ListTopicSubscriptionsResponse {
        pub fn builder() -> ListTopicSubscriptionsResponseBuilder {
            ListTopicSubscriptionsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for ListTopics."]
    pub struct ListTopicsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token that can be sent as `page_token` to retrieve the next page of results. If this field is omitted, there are no more results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of topic in the requested parent. The order of the topics is unspecified."]
        pub topics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Topic>>>,
    }
    impl ListTopicsResponse {
        pub fn builder() -> ListTopicsResponseBuilder {
            ListTopicsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The settings for a topic's partitions."]
    pub struct PartitionConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "capacity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The capacity configuration."]
        pub capacity: ::std::option::Option<::std::boxed::Box<Capacity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of partitions in the topic. Must be at least 1."]
        pub count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scale")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DEPRECATED: Use capacity instead which can express a superset of configurations. Every partition in the topic is allocated throughput equivalent to `scale` times the standard partition throughput (4 MiB/s). This is also reflected in the cost of this topic; a topic with `scale` of 2 and count of 10 is charged for 20 partitions. This value must be in the range [1,4]."]
        pub scale: ::std::option::Option<::std::primitive::i64>,
    }
    impl PartitionConfig {
        pub fn builder() -> PartitionConfigBuilder {
            PartitionConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A pair of a Cursor and the partition it is for."]
    pub struct PartitionCursor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cursor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value of the cursor."]
        pub cursor: ::std::option::Option<::std::boxed::Box<Cursor>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The partition this is for."]
        pub partition: ::std::option::Option<::std::string::String>,
    }
    impl PartitionCursor {
        pub fn builder() -> PartitionCursorBuilder {
            PartitionCursorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The settings for a topic's message retention."]
    pub struct RetentionConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "perPartitionBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The provisioned storage, in bytes, per partition. If the number of bytes stored in any of the topic's partitions grows beyond this value, older messages will be dropped to make room for newer ones, regardless of the value of `period`."]
        pub per_partition_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "period")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How long a published message is retained. If unset, messages will be retained as long as the bytes retained for each partition is below `per_partition_bytes`."]
        pub period: ::std::option::Option<::std::string::String>,
    }
    impl RetentionConfig {
        pub fn builder() -> RetentionConfigBuilder {
            RetentionConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata about a subscription resource."]
    pub struct Subscription {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deliveryConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The settings for this subscription's message delivery."]
        pub delivery_config: ::std::option::Option<::std::boxed::Box<DeliveryConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the subscription. Structured like: projects/{project_number}/locations/{location}/subscriptions/{subscription_id}"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the topic this subscription is attached to. Structured like: projects/{project_number}/locations/{location}/topics/{topic_id}"]
        pub topic: ::std::option::Option<::std::string::String>,
    }
    impl Subscription {
        pub fn builder() -> SubscriptionBuilder {
            SubscriptionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata about a topic resource."]
    pub struct Topic {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the topic. Structured like: projects/{project_number}/locations/{location}/topics/{topic_id}"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partitionConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The settings for this topic's partitions."]
        pub partition_config: ::std::option::Option<::std::boxed::Box<PartitionConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "retentionConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The settings for this topic's message retention."]
        pub retention_config: ::std::option::Option<::std::boxed::Box<RetentionConfig>>,
    }
    impl Topic {
        pub fn builder() -> TopicBuilder {
            TopicBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for GetTopicPartitions."]
    pub struct TopicPartitions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partitionCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of partitions in the topic."]
        pub partition_count: ::std::option::Option<::std::string::String>,
    }
    impl TopicPartitions {
        pub fn builder() -> TopicPartitionsBuilder {
            TopicPartitionsBuilder::default()
        }
    }
}
