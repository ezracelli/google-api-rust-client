#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the Acknowledge method."]
pub struct AcknowledgeRequest {
    #[serde(rename = "ackId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The acknowledgment ID for the message being acknowledged. This was returned by the Pub/Sub system in the Pull response."]
    pub ack_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "subscription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The subscription whose message is being acknowledged."]
    pub subscription: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An empty message that you can re-use to avoid defining duplicated empty messages in your project. A typical example is to use it as argument or the return value of a service API. For instance: service Foo { rpc Bar (proto2.Empty) returns (proto2.Empty) { }; }; BEGIN GOOGLE-INTERNAL The difference between this one and net/rpc/empty-message.proto is that 1) The generated message here is in proto2 C++ API. 2) The proto2.Empty has minimum dependencies (no message_set or net/rpc dependencies) END GOOGLE-INTERNAL"]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A key-value pair applied to a given object."]
pub struct Label {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The key of a label is a syntactically valid URL (as per RFC 1738) with the \"scheme\" and initial slashes omitted and with the additional restrictions noted below. Each key should be globally unique. The \"host\" portion is called the \"namespace\" and is not necessarily resolvable to a network endpoint. Instead, the namespace indicates what system or entity defines the semantics of the label. Namespaces do not restrict the set of objects to which a label may be associated. Keys are defined by the following grammar: key = hostname \"/\" kpath kpath = ksegment *[ \"/\" ksegment ] ksegment = alphadigit | *[ alphadigit | \"-\" | \"_\" | \".\" ] where \"hostname\" and \"alphadigit\" are defined as in RFC 1738. Example key: spanner.google.com/universe"]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "numValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An integer value."]
    pub num_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "strValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A string value."]
    pub str_value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the ListSubscriptions method."]
pub struct ListSubscriptionsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If not empty, indicates that there are more subscriptions that match the request and this value should be passed to the next ListSubscriptionsRequest to continue."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subscription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The subscriptions that match the request."]
    pub subscription: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Subscription>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the ListTopics method."]
pub struct ListTopicsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If not empty, indicates that there are more topics that match the request, and this value should be passed to the next ListTopicsRequest to continue."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "topic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resulting topics."]
    pub topic: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Topic>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the ModifyAckDeadline method."]
pub struct ModifyAckDeadlineRequest {
    #[serde(rename = "ackDeadlineSeconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The new ack deadline with respect to the time this request was sent to the Pub/Sub system. Must be >= 0. For example, if the value is 10, the new ack deadline will expire 10 seconds after the ModifyAckDeadline call was made. Specifying zero may immediately make the message available for another pull request."]
    pub ack_deadline_seconds: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "ackId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The acknowledgment ID. Either this or ack_ids must be populated, not both."]
    pub ack_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ackIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of acknowledgment IDs. Either this field or ack_id should be populated, not both."]
    pub ack_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "subscription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Next Index: 5 The name of the subscription from which messages are being pulled."]
    pub subscription: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the ModifyPushConfig method."]
pub struct ModifyPushConfigRequest {
    #[serde(rename = "pushConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An empty push_config indicates that the Pub/Sub system should pause pushing messages from the given subscription."]
    pub push_config: ::std::option::Option<::std::boxed::Box<PushConfig>>,
    #[serde(rename = "subscription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the subscription."]
    pub subscription: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the PublishBatch method."]
pub struct PublishBatchRequest {
    #[serde(rename = "messages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The messages to publish."]
    pub messages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PubsubMessage>>>,
    #[serde(rename = "topic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The messages in the request will be published on this topic."]
    pub topic: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the PublishBatch method."]
pub struct PublishBatchResponse {
    #[serde(rename = "messageIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The server-assigned ID of each published message, in the same order as the messages in the request. IDs are guaranteed to be unique within the topic."]
    pub message_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the Publish method."]
pub struct PublishRequest {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The message to publish."]
    pub message: ::std::option::Option<::std::boxed::Box<PubsubMessage>>,
    #[serde(rename = "topic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The message in the request will be published on this topic."]
    pub topic: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event indicating a received message or truncation event."]
pub struct PubsubEvent {
    #[serde(rename = "deleted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates that this subscription has been deleted. (Note that pull subscribers will always receive NOT_FOUND in response in their pull request on the subscription, rather than seeing this boolean.)"]
    pub deleted: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A received message."]
    pub message: ::std::option::Option<::std::boxed::Box<PubsubMessage>>,
    #[serde(rename = "subscription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The subscription that received the event."]
    pub subscription: ::std::option::Option<::std::string::String>,
    #[serde(rename = "truncated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates that this subscription has been truncated."]
    pub truncated: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A message data and its labels."]
pub struct PubsubMessage {
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The message payload."]
    pub data: ::std::option::Option<::std::string::String>,
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional list of labels for this message. Keys in this collection must be unique."]
    pub label: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Label>>>,
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this message assigned by the server at publication time. Guaranteed to be unique within the topic. This value may be read by a subscriber that receives a PubsubMessage via a Pull call or a push delivery. It must not be populated by a publisher in a Publish call."]
    pub message_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publishTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the message was published. The time is milliseconds since the UNIX epoch."]
    pub publish_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the PullBatch method."]
pub struct PullBatchRequest {
    #[serde(rename = "maxEvents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum number of PubsubEvents returned for this request. The Pub/Sub system may return fewer than the number of events specified."]
    pub max_events: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "returnImmediately")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If this is specified as true the system will respond immediately even if it is not able to return a message in the Pull response. Otherwise the system is allowed to wait until at least one message is available rather than returning no messages. The client may cancel the request if it does not wish to wait any longer for the response."]
    pub return_immediately: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "subscription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The subscription from which messages should be pulled."]
    pub subscription: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the PullBatch method."]
pub struct PullBatchResponse {
    #[serde(rename = "pullResponses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Received Pub/Sub messages or status events. The Pub/Sub system will return zero messages if there are no more messages available in the backlog. The Pub/Sub system may return fewer than the max_events requested even if there are more messages available in the backlog."]
    pub pull_responses: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PullResponse>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the Pull method."]
pub struct PullRequest {
    #[serde(rename = "returnImmediately")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If this is specified as true the system will respond immediately even if it is not able to return a message in the Pull response. Otherwise the system is allowed to wait until at least one message is available rather than returning FAILED_PRECONDITION. The client may cancel the request if it does not wish to wait any longer for the response."]
    pub return_immediately: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "subscription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The subscription from which a message should be pulled."]
    pub subscription: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Either a PubsubMessage or a truncation event. One of these two must be populated."]
pub struct PullResponse {
    #[serde(rename = "ackId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This ID must be used to acknowledge the received event or message."]
    pub ack_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pubsubEvent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A pubsub message or truncation event."]
    pub pubsub_event: ::std::option::Option<::std::boxed::Box<PubsubEvent>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for a push delivery endpoint."]
pub struct PushConfig {
    #[serde(rename = "pushEndpoint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A URL locating the endpoint to which messages should be pushed. For example, a Webhook endpoint might use \"https://example.com/push\"."]
    pub push_endpoint: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A subscription resource."]
pub struct Subscription {
    #[serde(rename = "ackDeadlineSeconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For either push or pull delivery, the value is the maximum time after a subscriber receives a message before the subscriber should acknowledge or Nack the message. If the Ack deadline for a message passes without an Ack or a Nack, the Pub/Sub system will eventually redeliver the message. If a subscriber acknowledges after the deadline, the Pub/Sub system may accept the Ack, but it is possible that the message has been already delivered again. Multiple Acks to the message are allowed and will succeed. For push delivery, this value is used to set the request timeout for the call to the push endpoint. For pull delivery, this value is used as the initial value for the Ack deadline. It may be overridden for each message using its corresponding ack_id with ModifyAckDeadline. While a message is outstanding (i.e. it has been delivered to a pull subscriber and the subscriber has not yet Acked or Nacked), the Pub/Sub system will not deliver that message to another pull subscriber (on a best-effort basis)."]
    pub ack_deadline_seconds: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the subscription."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pushConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If push delivery is used with this subscription, this field is used to configure it."]
    pub push_config: ::std::option::Option<::std::boxed::Box<PushConfig>>,
    #[serde(rename = "topic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the topic from which this subscription is receiving messages."]
    pub topic: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A topic resource."]
pub struct Topic {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the topic."]
    pub name: ::std::option::Option<::std::string::String>,
}
