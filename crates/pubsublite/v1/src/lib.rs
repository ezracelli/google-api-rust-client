#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The throughput capacity configuration for each partition."]
pub struct Capacity {
    #[serde(rename = "publishMibPerSec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Publish throughput capacity per partition in MiB/s. Must be >= 4 and <= 16."]
    pub publish_mib_per_sec: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "subscribeMibPerSec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subscribe throughput capacity per partition in MiB/s. Must be >= 4 and <= 32."]
    pub subscribe_mib_per_sec: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Compute the current head cursor for a partition."]
pub struct ComputeHeadCursorRequest {
    #[serde(rename = "partition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The partition for which we should compute the head cursor."]
    pub partition: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response containing the head cursor for the requested topic and partition."]
pub struct ComputeHeadCursorResponse {
    #[serde(rename = "headCursor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The head cursor."]
    pub head_cursor: ::std::option::Option<::std::boxed::Box<Cursor>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Compute statistics about a range of messages in a given topic and partition."]
pub struct ComputeMessageStatsRequest {
    #[serde(rename = "endCursor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The exclusive end of the range. The range is empty if end_cursor <= start_cursor. Specifying a start_cursor before the first message and an end_cursor after the last message will retrieve all messages."]
    pub end_cursor: ::std::option::Option<::std::boxed::Box<Cursor>>,
    #[serde(rename = "partition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The partition for which we should compute message stats."]
    pub partition: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startCursor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The inclusive start of the range."]
    pub start_cursor: ::std::option::Option<::std::boxed::Box<Cursor>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response containing stats for messages in the requested topic and partition."]
pub struct ComputeMessageStatsResponse {
    #[serde(rename = "messageBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of quota bytes accounted to these messages."]
    pub message_bytes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "messageCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The count of messages."]
    pub message_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "minimumEventTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minimum event timestamp across these messages. For the purposes of this computation, if a message does not have an event time, we use the publish time. The timestamp will be unset if there are no messages."]
    pub minimum_event_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "minimumPublishTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minimum publish timestamp across these messages. Note that publish timestamps within a partition are not guaranteed to be non-decreasing. The timestamp will be unset if there are no messages."]
    pub minimum_publish_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A cursor that describes the position of a message within a topic partition."]
pub struct Cursor {
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The offset of a message within a topic partition. Must be greater than or equal 0."]
    pub offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The settings for a subscription's message delivery."]
pub struct DeliveryConfig {
    #[serde(rename = "deliveryRequirement")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The DeliveryRequirement for this subscription."]
    pub delivery_requirement: ::std::option::Option<DeliveryConfigDeliveryRequirementEnum>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for ListPartitionCursors"]
pub struct ListPartitionCursorsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "partitionCursors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The partition cursors from this request."]
    pub partition_cursors:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PartitionCursor>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for ListSubscriptions."]
pub struct ListSubscriptionsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token that can be sent as `page_token` to retrieve the next page of results. If this field is omitted, there are no more results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subscriptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of subscriptions in the requested parent. The order of the subscriptions is unspecified."]
    pub subscriptions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Subscription>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for ListTopicSubscriptions."]
pub struct ListTopicSubscriptionsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token that can be sent as `page_token` to retrieve the next page of results. If this field is omitted, there are no more results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subscriptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The names of subscriptions attached to the topic. The order of the subscriptions is unspecified."]
    pub subscriptions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for ListTopics."]
pub struct ListTopicsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token that can be sent as `page_token` to retrieve the next page of results. If this field is omitted, there are no more results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "topics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of topic in the requested parent. The order of the topics is unspecified."]
    pub topics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Topic>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The settings for a topic's partitions."]
pub struct PartitionConfig {
    #[serde(rename = "capacity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The capacity configuration."]
    pub capacity: ::std::option::Option<::std::boxed::Box<Capacity>>,
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of partitions in the topic. Must be at least 1."]
    pub count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scale")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DEPRECATED: Use capacity instead which can express a superset of configurations. Every partition in the topic is allocated throughput equivalent to `scale` times the standard partition throughput (4 MiB/s). This is also reflected in the cost of this topic; a topic with `scale` of 2 and count of 10 is charged for 20 partitions. This value must be in the range [1,4]."]
    pub scale: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A pair of a Cursor and the partition it is for."]
pub struct PartitionCursor {
    #[serde(rename = "cursor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value of the cursor."]
    pub cursor: ::std::option::Option<::std::boxed::Box<Cursor>>,
    #[serde(rename = "partition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The partition this is for."]
    pub partition: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The settings for a topic's message retention."]
pub struct RetentionConfig {
    #[serde(rename = "perPartitionBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The provisioned storage, in bytes, per partition. If the number of bytes stored in any of the topic's partitions grows beyond this value, older messages will be dropped to make room for newer ones, regardless of the value of `period`."]
    pub per_partition_bytes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "period")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "How long a published message is retained. If unset, messages will be retained as long as the bytes retained for each partition is below `per_partition_bytes`."]
    pub period: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata about a subscription resource."]
pub struct Subscription {
    #[serde(rename = "deliveryConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The settings for this subscription's message delivery."]
    pub delivery_config: ::std::option::Option<::std::boxed::Box<DeliveryConfig>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the subscription. Structured like: projects/{project_number}/locations/{location}/subscriptions/{subscription_id}"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "topic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the topic this subscription is attached to. Structured like: projects/{project_number}/locations/{location}/topics/{topic_id}"]
    pub topic: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata about a topic resource."]
pub struct Topic {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the topic. Structured like: projects/{project_number}/locations/{location}/topics/{topic_id}"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "partitionConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The settings for this topic's partitions."]
    pub partition_config: ::std::option::Option<::std::boxed::Box<PartitionConfig>>,
    #[serde(rename = "retentionConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The settings for this topic's message retention."]
    pub retention_config: ::std::option::Option<::std::boxed::Box<RetentionConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for GetTopicPartitions."]
pub struct TopicPartitions {
    #[serde(rename = "partitionCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of partitions in the topic."]
    pub partition_count: ::std::option::Option<::std::string::String>,
}
