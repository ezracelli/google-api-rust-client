#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the Acknowledge method."]
pub struct AcknowledgeRequest {
    #[serde(rename = "ackIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The acknowledgment ID for the messages being acknowledged that was returned by the Pub/Sub system in the `Pull` response. Must not be empty."]
    pub ack_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Associates `members` with a `role`."]
pub struct Binding {
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the members in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
    pub condition: ::std::option::Option<::std::boxed::Box<Expr>>,
    #[serde(rename = "members")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the identities requesting access for a Cloud Platform resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. "]
    pub members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Role that is assigned to `members`. For example, `roles/viewer`, `roles/editor`, or `roles/owner`."]
    pub role: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the `CreateSnapshot` method."]
pub struct CreateSnapshotRequest {
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "See Creating and managing labels."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "subscription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The subscription whose backlog the snapshot retains. Specifically, the created snapshot is guaranteed to retain: (a) The existing backlog on the subscription. More precisely, this is defined as the messages in the subscription's backlog that are unacknowledged upon the successful completion of the `CreateSnapshot` request; as well as: (b) Any messages published to the subscription's topic following the successful completion of the CreateSnapshot request. Format is `projects/{project}/subscriptions/{sub}`."]
    pub subscription: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Dead lettering is done on a best effort basis. The same message might be dead lettered multiple times. If validation on any of the fields fails at subscription creation/updation, the create/update subscription request will fail."]
pub struct DeadLetterPolicy {
    #[serde(rename = "deadLetterTopic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the topic to which dead letter messages should be published. Format is `projects/{project}/topics/{topic}`.The Cloud Pub/Sub service account associated with the enclosing subscription's parent project (i.e., service-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com) must have permission to Publish() to this topic. The operation will fail if the topic does not exist. Users should ensure that there is a subscription attached to this topic since messages published to a topic with no subscriptions are lost."]
    pub dead_letter_topic: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maxDeliveryAttempts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum number of delivery attempts for any message. The value must be between 5 and 100. The number of delivery attempts is defined as 1 + (the sum of number of NACKs and number of times the acknowledgement deadline has been exceeded for the message). A NACK is any call to ModifyAckDeadline with a 0 deadline. Note that client libraries may automatically extend ack_deadlines. This field will be honored on a best effort basis. If this parameter is 0, a default value of 5 is used."]
    pub max_delivery_attempts: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the DetachSubscription method. Reserved for future use."]
pub struct DetachSubscriptionResponse {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A policy that specifies the conditions for resource expiration (i.e., automatic resource deletion)."]
pub struct ExpirationPolicy {
    #[serde(rename = "ttl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the \"time-to-live\" duration for an associated resource. The resource expires if it is not active for a period of `ttl`. The definition of \"activity\" depends on the type of the associated resource. The minimum and maximum allowed values for `ttl` depend on the type of the associated resource, as well. If `ttl` is not set, the associated resource never expires."]
    pub ttl: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: \"Summary size limit\" description: \"Determines if a summary is less than 100 chars\" expression: \"document.summary.size() < 100\" Example (Equality): title: \"Requestor is owner\" description: \"Determines if requestor is the document owner\" expression: \"document.owner == request.auth.claims.email\" Example (Logic): title: \"Public documents\" description: \"Determine whether the document should be publicly visible\" expression: \"document.type != 'private' && document.type != 'internal'\" Example (Data Manipulation): title: \"Notification string\" description: \"Create a notification string with a timestamp.\" expression: \"'New message received at ' + string(document.create_time)\" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information."]
pub struct Expr {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expression")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: ::std::option::Option<::std::string::String>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the `ListSchemas` method."]
pub struct ListSchemasResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If not empty, indicates that there may be more schemas that match the request; this value should be passed in a new `ListSchemasRequest`."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "schemas")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resulting schemas."]
    pub schemas: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Schema>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the `ListSnapshots` method."]
pub struct ListSnapshotsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If not empty, indicates that there may be more snapshot that match the request; this value should be passed in a new `ListSnapshotsRequest`."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "snapshots")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resulting snapshots."]
    pub snapshots: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Snapshot>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the `ListSubscriptions` method."]
pub struct ListSubscriptionsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If not empty, indicates that there may be more subscriptions that match the request; this value should be passed in a new `ListSubscriptionsRequest` to get more subscriptions."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subscriptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The subscriptions that match the request."]
    pub subscriptions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Subscription>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the `ListTopicSnapshots` method."]
pub struct ListTopicSnapshotsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If not empty, indicates that there may be more snapshots that match the request; this value should be passed in a new `ListTopicSnapshotsRequest` to get more snapshots."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "snapshots")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The names of the snapshots that match the request."]
    pub snapshots: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the `ListTopicSubscriptions` method."]
pub struct ListTopicSubscriptionsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If not empty, indicates that there may be more subscriptions that match the request; this value should be passed in a new `ListTopicSubscriptionsRequest` to get more subscriptions."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subscriptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The names of subscriptions attached to the topic specified in the request."]
    pub subscriptions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the `ListTopics` method."]
pub struct ListTopicsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If not empty, indicates that there may be more topics that match the request; this value should be passed in a new `ListTopicsRequest`."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "topics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resulting topics."]
    pub topics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Topic>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A policy constraining the storage of messages published to the topic."]
pub struct MessageStoragePolicy {
    #[serde(rename = "allowedPersistenceRegions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of IDs of GCP regions where messages that are published to the topic may be persisted in storage. Messages published by publishers running in non-allowed GCP regions (or running outside of GCP altogether) will be routed for storage in one of the allowed regions. An empty list means that no regions are allowed, and is not a valid configuration."]
    pub allowed_persistence_regions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the ModifyAckDeadline method."]
pub struct ModifyAckDeadlineRequest {
    #[serde(rename = "ackDeadlineSeconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The new ack deadline with respect to the time this request was sent to the Pub/Sub system. For example, if the value is 10, the new ack deadline will expire 10 seconds after the `ModifyAckDeadline` call was made. Specifying zero might immediately make the message available for delivery to another subscriber client. This typically results in an increase in the rate of message redeliveries (that is, duplicates). The minimum deadline you can specify is 0 seconds. The maximum deadline you can specify is 600 seconds (10 minutes)."]
    pub ack_deadline_seconds: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "ackIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. List of acknowledgment IDs."]
    pub ack_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the ModifyPushConfig method."]
pub struct ModifyPushConfigRequest {
    #[serde(rename = "pushConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The push configuration for future deliveries. An empty `pushConfig` indicates that the Pub/Sub system should stop pushing messages from the given subscription and allow messages to be pulled and acknowledged - effectively pausing the subscription if `Pull` or `StreamingPull` is not called."]
    pub push_config: ::std::option::Option<::std::boxed::Box<PushConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains information needed for generating an [OpenID Connect token](https://developers.google.com/identity/protocols/OpenIDConnect)."]
pub struct OidcToken {
    #[serde(rename = "audience")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Audience to be used when generating OIDC token. The audience claim identifies the recipients that the JWT is intended for. The audience value is a single case-sensitive string. Having multiple values (array) for the audience field is not supported. More info about the OIDC JWT token audience here: https://tools.ietf.org/html/rfc7519#section-4.1.3 Note: if not specified, the Push endpoint URL will be used."]
    pub audience: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serviceAccountEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Service account email](https://cloud.google.com/iam/docs/service-accounts) to be used for generating the OIDC token. The caller (for CreateSubscription, UpdateSubscription, and ModifyPushConfig RPCs) must have the iam.serviceAccounts.actAs permission for the service account."]
    pub service_account_email: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members` to a single `role`. Members can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { \"bindings\": [ { \"role\": \"roles/resourcemanager.organizationAdmin\", \"members\": [ \"user:mike@example.com\", \"group:admins@example.com\", \"domain:google.com\", \"serviceAccount:my-project-id@appspot.gserviceaccount.com\" ] }, { \"role\": \"roles/resourcemanager.organizationViewer\", \"members\": [ \"user:eve@example.com\" ], \"condition\": { \"title\": \"expirable access\", \"description\": \"Does not grant access after Sep 2020\", \"expression\": \"request.time < timestamp('2020-10-01T00:00:00.000Z')\", } } ], \"etag\": \"BwWWja0YfJA=\", \"version\": 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') - etag: BwWWja0YfJA= - version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/)."]
pub struct Policy {
    #[serde(rename = "bindings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Associates a list of `members` to a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one member."]
    pub bindings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Binding>>>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "`etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
    pub version: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the Publish method."]
pub struct PublishRequest {
    #[serde(rename = "messages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The messages to publish."]
    pub messages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PubsubMessage>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the `Publish` method."]
pub struct PublishResponse {
    #[serde(rename = "messageIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The server-assigned ID of each published message, in the same order as the messages in the request. IDs are guaranteed to be unique within the topic."]
    pub message_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A message that is published by publishers and consumed by subscribers. The message must contain either a non-empty data field or at least one attribute. Note that client libraries represent this object differently depending on the language. See the corresponding [client library documentation](https://cloud.google.com/pubsub/docs/reference/libraries) for more information. See [quotas and limits] (https://cloud.google.com/pubsub/quotas) for more information about message limits."]
pub struct PubsubMessage {
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Attributes for this message. If this field is empty, the message must contain non-empty data. This can be used to filter messages on the subscription."]
    pub attributes:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The message data field. If this field is empty, the message must contain at least one attribute."]
    pub data: ::std::option::Option<::std::string::String>,
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of this message, assigned by the server when the message is published. Guaranteed to be unique within the topic. This value may be read by a subscriber that receives a `PubsubMessage` via a `Pull` call or a push delivery. It must not be populated by the publisher in a `Publish` call."]
    pub message_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orderingKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If non-empty, identifies related messages for which publish order should be respected. If a `Subscription` has `enable_message_ordering` set to `true`, messages published with the same non-empty `ordering_key` value will be delivered to subscribers in the order in which they are received by the Pub/Sub system. All `PubsubMessage`s published in a given `PublishRequest` must specify the same `ordering_key` value."]
    pub ordering_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publishTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the message was published, populated by the server when it receives the `Publish` call. It must not be populated by the publisher in a `Publish` call."]
    pub publish_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the `Pull` method."]
pub struct PullRequest {
    #[serde(rename = "maxMessages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The maximum number of messages to return for this request. Must be a positive integer. The Pub/Sub system may return fewer than the number specified."]
    pub max_messages: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "returnImmediately")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If this field set to true, the system will respond immediately even if it there are no messages available to return in the `Pull` response. Otherwise, the system may wait (for a bounded amount of time) until at least one message is available, rather than returning no messages. Warning: setting this field to `true` is discouraged because it adversely impacts the performance of `Pull` operations. We recommend that users do not set this field."]
    pub return_immediately: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the `Pull` method."]
pub struct PullResponse {
    #[serde(rename = "receivedMessages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Received Pub/Sub messages. The list will be empty if there are no more messages available in the backlog. For JSON, the response can be entirely empty. The Pub/Sub system may return fewer than the `maxMessages` requested even if there are more messages available in the backlog."]
    pub received_messages:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ReceivedMessage>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for a push delivery endpoint."]
pub struct PushConfig {
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Endpoint configuration attributes that can be used to control different aspects of the message delivery. The only currently supported attribute is `x-goog-version`, which you can use to change the format of the pushed message. This attribute indicates the version of the data expected by the endpoint. This controls the shape of the pushed message (i.e., its fields and metadata). If not present during the `CreateSubscription` call, it will default to the version of the Pub/Sub API used to make such call. If not present in a `ModifyPushConfig` call, its value will not be changed. `GetSubscription` calls will always return a valid version, even if the subscription was created without this attribute. The only supported values for the `x-goog-version` attribute are: * `v1beta1`: uses the push format defined in the v1beta1 Pub/Sub API. * `v1` or `v1beta2`: uses the push format defined in the v1 Pub/Sub API. For example: attributes { \"x-goog-version\": \"v1\" } "]
    pub attributes:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "oidcToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If specified, Pub/Sub will generate and attach an OIDC JWT token as an `Authorization` header in the HTTP request for every pushed message."]
    pub oidc_token: ::std::option::Option<::std::boxed::Box<OidcToken>>,
    #[serde(rename = "pushEndpoint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A URL locating the endpoint to which messages should be pushed. For example, a Webhook endpoint might use `https://example.com/push`."]
    pub push_endpoint: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A message and its corresponding acknowledgment ID."]
pub struct ReceivedMessage {
    #[serde(rename = "ackId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This ID can be used to acknowledge the received message."]
    pub ack_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deliveryAttempt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The approximate number of times that Cloud Pub/Sub has attempted to deliver the associated message to a subscriber. More precisely, this is 1 + (number of NACKs) + (number of ack_deadline exceeds) for this message. A NACK is any call to ModifyAckDeadline with a 0 deadline. An ack_deadline exceeds event is whenever a message is not acknowledged within ack_deadline. Note that ack_deadline is initially Subscription.ackDeadlineSeconds, but may get extended automatically by the client library. Upon the first delivery of a given message, `delivery_attempt` will have a value of 1. The value is calculated at best effort and is approximate. If a DeadLetterPolicy is not set on the subscription, this will be 0."]
    pub delivery_attempt: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The message."]
    pub message: ::std::option::Option<::std::boxed::Box<PubsubMessage>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A policy that specifies how Cloud Pub/Sub retries message delivery. Retry delay will be exponential based on provided minimum and maximum backoffs. https://en.wikipedia.org/wiki/Exponential_backoff. RetryPolicy will be triggered on NACKs or acknowledgement deadline exceeded events for a given message. Retry Policy is implemented on a best effort basis. At times, the delay between consecutive deliveries may not match the configuration. That is, delay can be more or less than configured backoff."]
pub struct RetryPolicy {
    #[serde(rename = "maximumBackoff")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum delay between consecutive deliveries of a given message. Value should be between 0 and 600 seconds. Defaults to 600 seconds."]
    pub maximum_backoff: ::std::option::Option<::std::string::String>,
    #[serde(rename = "minimumBackoff")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minimum delay between consecutive deliveries of a given message. Value should be between 0 and 600 seconds. Defaults to 10 seconds."]
    pub minimum_backoff: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A schema resource."]
pub struct Schema {
    #[serde(rename = "definition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The definition of the schema. This should contain a string representing the full definition of the schema that is a valid schema definition of the type specified in `type`."]
    pub definition: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Name of the schema. Format is `projects/{project}/schemas/{schema}`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the schema definition."]
    pub _type: ::std::option::Option<SchemaTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the schema definition."]
pub enum SchemaTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    TypeUnspecified,
    #[serde(rename = "PROTOCOL_BUFFER")]
    #[doc = "A Protocol Buffer schema definition."]
    ProtocolBuffer,
    #[serde(rename = "AVRO")]
    #[doc = "An Avro schema definition."]
    Avro,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings for validating messages published against a schema."]
pub struct SchemaSettings {
    #[serde(rename = "encoding")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The encoding of messages validated against `schema`."]
    pub encoding: ::std::option::Option<SchemaSettingsEncodingEnum>,
    #[serde(rename = "schema")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the schema that messages published should be validated against. Format is `projects/{project}/schemas/{schema}`. The value of this field will be `_deleted-schema_` if the schema has been deleted."]
    pub schema: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The encoding of messages validated against `schema`."]
pub enum SchemaSettingsEncodingEnum {
    #[serde(rename = "ENCODING_UNSPECIFIED")]
    #[doc = "Unspecified"]
    EncodingUnspecified,
    #[serde(rename = "JSON")]
    #[doc = "JSON encoding"]
    Json,
    #[serde(rename = "BINARY")]
    #[doc = "Binary encoding, as defined by the schema type. For some schema types, binary encoding may not be available."]
    Binary,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the `Seek` method."]
pub struct SeekRequest {
    #[serde(rename = "snapshot")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The snapshot to seek to. The snapshot's topic must be the same as that of the provided subscription. Format is `projects/{project}/snapshots/{snap}`."]
    pub snapshot: ::std::option::Option<::std::string::String>,
    #[serde(rename = "time")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time to seek to. Messages retained in the subscription that were published before this time are marked as acknowledged, and messages retained in the subscription that were published after this time are marked as unacknowledged. Note that this operation affects only those messages retained in the subscription (configured by the combination of `message_retention_duration` and `retain_acked_messages`). For example, if `time` corresponds to a point before the message retention window (or to a point before the system's notion of the subscription creation time), only retained messages will be marked as unacknowledged, and already-expunged messages will not be restored."]
    pub time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the `Seek` method (this response is empty)."]
pub struct SeekResponse {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for `SetIamPolicy` method."]
pub struct SetIamPolicyRequest {
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Cloud Platform services (such as Projects) might reject them."]
    pub policy: ::std::option::Option<::std::boxed::Box<Policy>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A snapshot resource. Snapshots are used in [Seek](https://cloud.google.com/pubsub/docs/replay-overview) operations, which allow you to manage message acknowledgments in bulk. That is, you can set the acknowledgment state of messages in an existing subscription to the state captured by a snapshot."]
pub struct Snapshot {
    #[serde(rename = "expireTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The snapshot is guaranteed to exist up until this time. A newly-created snapshot expires no later than 7 days from the time of its creation. Its exact lifetime is determined at creation by the existing backlog in the source subscription. Specifically, the lifetime of the snapshot is `7 days - (age of oldest unacked message in the subscription)`. For example, consider a subscription whose oldest unacked message is 3 days old. If a snapshot is created from this subscription, the snapshot -- which will always capture this 3-day-old backlog as long as the snapshot exists -- will expire in 4 days. The service will refuse to create a snapshot that would expire in less than 1 hour after creation."]
    pub expire_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "See [Creating and managing labels] (https://cloud.google.com/pubsub/docs/labels)."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the snapshot."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "topic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the topic from which this snapshot is retaining messages."]
    pub topic: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A subscription resource."]
pub struct Subscription {
    #[serde(rename = "ackDeadlineSeconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The approximate amount of time (on a best-effort basis) Pub/Sub waits for the subscriber to acknowledge receipt before resending the message. In the interval after the message is delivered and before it is acknowledged, it is considered to be *outstanding*. During that time period, the message will not be redelivered (on a best-effort basis). For pull subscriptions, this value is used as the initial value for the ack deadline. To override this value for a given message, call `ModifyAckDeadline` with the corresponding `ack_id` if using non-streaming pull or send the `ack_id` in a `StreamingModifyAckDeadlineRequest` if using streaming pull. The minimum custom deadline you can specify is 10 seconds. The maximum custom deadline you can specify is 600 seconds (10 minutes). If this parameter is 0, a default value of 10 seconds is used. For push delivery, this value is also used to set the request timeout for the call to the push endpoint. If the subscriber never acknowledges the message, the Pub/Sub system will eventually redeliver the message."]
    pub ack_deadline_seconds: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "deadLetterPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A policy that specifies the conditions for dead lettering messages in this subscription. If dead_letter_policy is not set, dead lettering is disabled. The Cloud Pub/Sub service account associated with this subscriptions's parent project (i.e., service-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com) must have permission to Acknowledge() messages on this subscription."]
    pub dead_letter_policy: ::std::option::Option<::std::boxed::Box<DeadLetterPolicy>>,
    #[serde(rename = "detached")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the subscription is detached from its topic. Detached subscriptions don't receive messages from their topic and don't retain any backlog. `Pull` and `StreamingPull` requests will return FAILED_PRECONDITION. If the subscription is a push subscription, pushes to the endpoint will not be made."]
    pub detached: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "enableMessageOrdering")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, messages published with the same `ordering_key` in `PubsubMessage` will be delivered to the subscribers in the order in which they are received by the Pub/Sub system. Otherwise, they may be delivered in any order."]
    pub enable_message_ordering: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "expirationPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A policy that specifies the conditions for this subscription's expiration. A subscription is considered active as long as any connected subscriber is successfully consuming messages from the subscription or is issuing operations on the subscription. If `expiration_policy` is not set, a *default policy* with `ttl` of 31 days will be used. The minimum allowed value for `expiration_policy.ttl` is 1 day."]
    pub expiration_policy: ::std::option::Option<::std::boxed::Box<ExpirationPolicy>>,
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An expression written in the Pub/Sub [filter language](https://cloud.google.com/pubsub/docs/filtering). If non-empty, then only `PubsubMessage`s whose `attributes` field matches the filter are delivered on this subscription. If empty, then no messages are filtered out."]
    pub filter: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "See Creating and managing labels."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "messageRetentionDuration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "How long to retain unacknowledged messages in the subscription's backlog, from the moment a message is published. If `retain_acked_messages` is true, then this also configures the retention of acknowledged messages, and thus configures how far back in time a `Seek` can be done. Defaults to 7 days. Cannot be more than 7 days or less than 10 minutes."]
    pub message_retention_duration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the subscription. It must have the format `\"projects/{project}/subscriptions/{subscription}\"`. `{subscription}` must start with a letter, and contain only letters (`[A-Za-z]`), numbers (`[0-9]`), dashes (`-`), underscores (`_`), periods (`.`), tildes (`~`), plus (`+`) or percent signs (`%`). It must be between 3 and 255 characters in length, and it must not start with `\"goog\"`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pushConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If push delivery is used with this subscription, this field is used to configure it. An empty `pushConfig` signifies that the subscriber will pull and ack messages using API methods."]
    pub push_config: ::std::option::Option<::std::boxed::Box<PushConfig>>,
    #[serde(rename = "retainAckedMessages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether to retain acknowledged messages. If true, then messages are not expunged from the subscription's backlog, even if they are acknowledged, until they fall out of the `message_retention_duration` window. This must be true if you would like to [Seek to a timestamp] (https://cloud.google.com/pubsub/docs/replay-overview#seek_to_a_time)."]
    pub retain_acked_messages: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "retryPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A policy that specifies how Pub/Sub retries message delivery for this subscription. If not set, the default retry policy is applied. This generally implies that messages will be retried as soon as possible for healthy subscribers. RetryPolicy will be triggered on NACKs or acknowledgement deadline exceeded events for a given message."]
    pub retry_policy: ::std::option::Option<::std::boxed::Box<RetryPolicy>>,
    #[serde(rename = "topic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the topic from which this subscription is receiving messages. Format is `projects/{project}/topics/{topic}`. The value of this field will be `_deleted-topic_` if the topic has been deleted."]
    pub topic: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for `TestIamPermissions` method."]
pub struct TestIamPermissionsRequest {
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of permissions to check for the `resource`. Permissions with wildcards (such as '*' or 'storage.*') are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions)."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for `TestIamPermissions` method."]
pub struct TestIamPermissionsResponse {
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A subset of `TestPermissionsRequest.permissions` that the caller is allowed."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A topic resource."]
pub struct Topic {
    #[serde(rename = "kmsKeyName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the Cloud KMS CryptoKey to be used to protect access to messages published on this topic. The expected format is `projects/*/locations/*/keyRings/*/cryptoKeys/*`."]
    pub kms_key_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "See [Creating and managing labels] (https://cloud.google.com/pubsub/docs/labels)."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "messageStoragePolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Policy constraining the set of Google Cloud Platform regions where messages published to the topic may be stored. If not present, then no constraints are in effect."]
    pub message_storage_policy: ::std::option::Option<::std::boxed::Box<MessageStoragePolicy>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the topic. It must have the format `\"projects/{project}/topics/{topic}\"`. `{topic}` must start with a letter, and contain only letters (`[A-Za-z]`), numbers (`[0-9]`), dashes (`-`), underscores (`_`), periods (`.`), tildes (`~`), plus (`+`) or percent signs (`%`). It must be between 3 and 255 characters in length, and it must not start with `\"goog\"`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "satisfiesPzs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reserved for future use. This field is set only in responses from the server; it is ignored if it is set in any requests."]
    pub satisfies_pzs: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "schemaSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Settings for validating messages published against a schema."]
    pub schema_settings: ::std::option::Option<::std::boxed::Box<SchemaSettings>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the UpdateSnapshot method."]
pub struct UpdateSnapshotRequest {
    #[serde(rename = "snapshot")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The updated snapshot object."]
    pub snapshot: ::std::option::Option<::std::boxed::Box<Snapshot>>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Indicates which fields in the provided snapshot to update. Must be specified and non-empty."]
    pub update_mask: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the UpdateSubscription method."]
pub struct UpdateSubscriptionRequest {
    #[serde(rename = "subscription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The updated subscription object."]
    pub subscription: ::std::option::Option<::std::boxed::Box<Subscription>>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Indicates which fields in the provided subscription to update. Must be specified and non-empty."]
    pub update_mask: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the UpdateTopic method."]
pub struct UpdateTopicRequest {
    #[serde(rename = "topic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The updated topic object."]
    pub topic: ::std::option::Option<::std::boxed::Box<Topic>>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Indicates which fields in the provided topic to update. Must be specified and non-empty. Note that if `update_mask` contains \"message_storage_policy\" but the `message_storage_policy` is not set in the `topic` provided above, then the updated value is determined by the policy configured at the project or organization level."]
    pub update_mask: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the `ValidateMessage` method."]
pub struct ValidateMessageRequest {
    #[serde(rename = "encoding")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The encoding expected for messages"]
    pub encoding: ::std::option::Option<ValidateMessageRequestEncodingEnum>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Message to validate against the provided `schema_spec`."]
    pub message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the schema against which to validate. Format is `projects/{project}/schemas/{schema}`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "schema")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ad-hoc schema against which to validate"]
    pub schema: ::std::option::Option<::std::boxed::Box<Schema>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The encoding expected for messages"]
pub enum ValidateMessageRequestEncodingEnum {
    #[serde(rename = "ENCODING_UNSPECIFIED")]
    #[doc = "Unspecified"]
    EncodingUnspecified,
    #[serde(rename = "JSON")]
    #[doc = "JSON encoding"]
    Json,
    #[serde(rename = "BINARY")]
    #[doc = "Binary encoding, as defined by the schema type. For some schema types, binary encoding may not be available."]
    Binary,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the `ValidateMessage` method. Empty for now."]
pub struct ValidateMessageResponse {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the `ValidateSchema` method."]
pub struct ValidateSchemaRequest {
    #[serde(rename = "schema")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The schema object to validate."]
    pub schema: ::std::option::Option<::std::boxed::Box<Schema>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the `ValidateSchema` method. Empty for now."]
pub struct ValidateSchemaResponse {}
