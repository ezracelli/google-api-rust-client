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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Maximum number of subscriptions to return."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The value obtained in the last ListSubscriptionsResponse for continuation."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "query")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A valid label query expression."]
                    pub query: ::std::option::Option<::std::string::String>,
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
                    #[doc = "Maximum number of topics to return."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The value obtained in the last ListTopicsResponse for continuation."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "query")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A valid label query expression."]
                    pub query: ::std::option::Option<::std::string::String>,
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
    #[doc = "Request for the Acknowledge method."]
    pub struct AcknowledgeRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ackId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The acknowledgment ID for the message being acknowledged. This was returned by the Pub/Sub system in the Pull response."]
        pub ack_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subscription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The subscription whose message is being acknowledged."]
        pub subscription: ::std::option::Option<::std::string::String>,
    }
    impl AcknowledgeRequest {
        pub fn builder() -> AcknowledgeRequestBuilder {
            AcknowledgeRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An empty message that you can re-use to avoid defining duplicated empty messages in your project. A typical example is to use it as argument or the return value of a service API. For instance: service Foo { rpc Bar (proto2.Empty) returns (proto2.Empty) { }; }; BEGIN GOOGLE-INTERNAL The difference between this one and net/rpc/empty-message.proto is that 1) The generated message here is in proto2 C++ API. 2) The proto2.Empty has minimum dependencies (no message_set or net/rpc dependencies) END GOOGLE-INTERNAL"]
    pub struct Empty {}
    impl Empty {
        pub fn builder() -> EmptyBuilder {
            EmptyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A key-value pair applied to a given object."]
    pub struct Label {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The key of a label is a syntactically valid URL (as per RFC 1738) with the \"scheme\" and initial slashes omitted and with the additional restrictions noted below. Each key should be globally unique. The \"host\" portion is called the \"namespace\" and is not necessarily resolvable to a network endpoint. Instead, the namespace indicates what system or entity defines the semantics of the label. Namespaces do not restrict the set of objects to which a label may be associated. Keys are defined by the following grammar: key = hostname \"/\" kpath kpath = ksegment *[ \"/\" ksegment ] ksegment = alphadigit | *[ alphadigit | \"-\" | \"_\" | \".\" ] where \"hostname\" and \"alphadigit\" are defined as in RFC 1738. Example key: spanner.google.com/universe"]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An integer value."]
        pub num_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "strValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A string value."]
        pub str_value: ::std::option::Option<::std::string::String>,
    }
    impl Label {
        pub fn builder() -> LabelBuilder {
            LabelBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for the ListSubscriptions method."]
    pub struct ListSubscriptionsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If not empty, indicates that there are more subscriptions that match the request and this value should be passed to the next ListSubscriptionsRequest to continue."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subscription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The subscriptions that match the request."]
        pub subscription: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Subscription>>>,
    }
    impl ListSubscriptionsResponse {
        pub fn builder() -> ListSubscriptionsResponseBuilder {
            ListSubscriptionsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for the ListTopics method."]
    pub struct ListTopicsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If not empty, indicates that there are more topics that match the request, and this value should be passed to the next ListTopicsRequest to continue."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resulting topics."]
        pub topic: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Topic>>>,
    }
    impl ListTopicsResponse {
        pub fn builder() -> ListTopicsResponseBuilder {
            ListTopicsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for the ModifyAckDeadline method."]
    pub struct ModifyAckDeadlineRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ackDeadlineSeconds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The new ack deadline with respect to the time this request was sent to the Pub/Sub system. Must be >= 0. For example, if the value is 10, the new ack deadline will expire 10 seconds after the ModifyAckDeadline call was made. Specifying zero may immediately make the message available for another pull request."]
        pub ack_deadline_seconds: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ackId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The acknowledgment ID. Either this or ack_ids must be populated, not both."]
        pub ack_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ackIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of acknowledgment IDs. Either this field or ack_id should be populated, not both."]
        pub ack_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subscription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Next Index: 5 The name of the subscription from which messages are being pulled."]
        pub subscription: ::std::option::Option<::std::string::String>,
    }
    impl ModifyAckDeadlineRequest {
        pub fn builder() -> ModifyAckDeadlineRequestBuilder {
            ModifyAckDeadlineRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for the ModifyPushConfig method."]
    pub struct ModifyPushConfigRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pushConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An empty push_config indicates that the Pub/Sub system should pause pushing messages from the given subscription."]
        pub push_config: ::std::option::Option<::std::boxed::Box<PushConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subscription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the subscription."]
        pub subscription: ::std::option::Option<::std::string::String>,
    }
    impl ModifyPushConfigRequest {
        pub fn builder() -> ModifyPushConfigRequestBuilder {
            ModifyPushConfigRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for the PublishBatch method."]
    pub struct PublishBatchRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The messages to publish."]
        pub messages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PubsubMessage>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The messages in the request will be published on this topic."]
        pub topic: ::std::option::Option<::std::string::String>,
    }
    impl PublishBatchRequest {
        pub fn builder() -> PublishBatchRequestBuilder {
            PublishBatchRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for the PublishBatch method."]
    pub struct PublishBatchResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messageIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The server-assigned ID of each published message, in the same order as the messages in the request. IDs are guaranteed to be unique within the topic."]
        pub message_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl PublishBatchResponse {
        pub fn builder() -> PublishBatchResponseBuilder {
            PublishBatchResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for the Publish method."]
    pub struct PublishRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The message to publish."]
        pub message: ::std::option::Option<::std::boxed::Box<PubsubMessage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The message in the request will be published on this topic."]
        pub topic: ::std::option::Option<::std::string::String>,
    }
    impl PublishRequest {
        pub fn builder() -> PublishRequestBuilder {
            PublishRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An event indicating a received message or truncation event."]
    pub struct PubsubEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates that this subscription has been deleted. (Note that pull subscribers will always receive NOT_FOUND in response in their pull request on the subscription, rather than seeing this boolean.)"]
        pub deleted: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A received message."]
        pub message: ::std::option::Option<::std::boxed::Box<PubsubMessage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subscription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The subscription that received the event."]
        pub subscription: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "truncated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates that this subscription has been truncated."]
        pub truncated: ::std::option::Option<::std::primitive::bool>,
    }
    impl PubsubEvent {
        pub fn builder() -> PubsubEventBuilder {
            PubsubEventBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A message data and its labels."]
    pub struct PubsubMessage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The message payload."]
        pub data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "label")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional list of labels for this message. Keys in this collection must be unique."]
        pub label: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Label>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messageId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of this message assigned by the server at publication time. Guaranteed to be unique within the topic. This value may be read by a subscriber that receives a PubsubMessage via a Pull call or a push delivery. It must not be populated by a publisher in a Publish call."]
        pub message_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the message was published. The time is milliseconds since the UNIX epoch."]
        pub publish_time: ::std::option::Option<::std::string::String>,
    }
    impl PubsubMessage {
        pub fn builder() -> PubsubMessageBuilder {
            PubsubMessageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for the PullBatch method."]
    pub struct PullBatchRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxEvents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of PubsubEvents returned for this request. The Pub/Sub system may return fewer than the number of events specified."]
        pub max_events: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "returnImmediately")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If this is specified as true the system will respond immediately even if it is not able to return a message in the Pull response. Otherwise the system is allowed to wait until at least one message is available rather than returning no messages. The client may cancel the request if it does not wish to wait any longer for the response."]
        pub return_immediately: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subscription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The subscription from which messages should be pulled."]
        pub subscription: ::std::option::Option<::std::string::String>,
    }
    impl PullBatchRequest {
        pub fn builder() -> PullBatchRequestBuilder {
            PullBatchRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for the PullBatch method."]
    pub struct PullBatchResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pullResponses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Received Pub/Sub messages or status events. The Pub/Sub system will return zero messages if there are no more messages available in the backlog. The Pub/Sub system may return fewer than the max_events requested even if there are more messages available in the backlog."]
        pub pull_responses: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PullResponse>>>,
    }
    impl PullBatchResponse {
        pub fn builder() -> PullBatchResponseBuilder {
            PullBatchResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for the Pull method."]
    pub struct PullRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "returnImmediately")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If this is specified as true the system will respond immediately even if it is not able to return a message in the Pull response. Otherwise the system is allowed to wait until at least one message is available rather than returning FAILED_PRECONDITION. The client may cancel the request if it does not wish to wait any longer for the response."]
        pub return_immediately: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subscription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The subscription from which a message should be pulled."]
        pub subscription: ::std::option::Option<::std::string::String>,
    }
    impl PullRequest {
        pub fn builder() -> PullRequestBuilder {
            PullRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Either a PubsubMessage or a truncation event. One of these two must be populated."]
    pub struct PullResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ackId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This ID must be used to acknowledge the received event or message."]
        pub ack_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pubsubEvent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A pubsub message or truncation event."]
        pub pubsub_event: ::std::option::Option<::std::boxed::Box<PubsubEvent>>,
    }
    impl PullResponse {
        pub fn builder() -> PullResponseBuilder {
            PullResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for a push delivery endpoint."]
    pub struct PushConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pushEndpoint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A URL locating the endpoint to which messages should be pushed. For example, a Webhook endpoint might use \"https://example.com/push\"."]
        pub push_endpoint: ::std::option::Option<::std::string::String>,
    }
    impl PushConfig {
        pub fn builder() -> PushConfigBuilder {
            PushConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A subscription resource."]
    pub struct Subscription {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ackDeadlineSeconds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For either push or pull delivery, the value is the maximum time after a subscriber receives a message before the subscriber should acknowledge or Nack the message. If the Ack deadline for a message passes without an Ack or a Nack, the Pub/Sub system will eventually redeliver the message. If a subscriber acknowledges after the deadline, the Pub/Sub system may accept the Ack, but it is possible that the message has been already delivered again. Multiple Acks to the message are allowed and will succeed. For push delivery, this value is used to set the request timeout for the call to the push endpoint. For pull delivery, this value is used as the initial value for the Ack deadline. It may be overridden for each message using its corresponding ack_id with ModifyAckDeadline. While a message is outstanding (i.e. it has been delivered to a pull subscriber and the subscriber has not yet Acked or Nacked), the Pub/Sub system will not deliver that message to another pull subscriber (on a best-effort basis)."]
        pub ack_deadline_seconds: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the subscription."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pushConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If push delivery is used with this subscription, this field is used to configure it."]
        pub push_config: ::std::option::Option<::std::boxed::Box<PushConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the topic from which this subscription is receiving messages."]
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
    #[doc = "A topic resource."]
    pub struct Topic {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the topic."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl Topic {
        pub fn builder() -> TopicBuilder {
            TopicBuilder::default()
        }
    }
}
