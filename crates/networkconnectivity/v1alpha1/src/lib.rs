#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The allowed types for [VALUE] in a `[KEY]:[VALUE]` attribute."]
pub struct AttributeValue {
    #[serde(rename = "boolValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A Boolean value represented by `true` or `false`."]
    pub bool_value: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "intValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A 64-bit signed integer."]
    pub int_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "stringValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A string up to 256 bytes long."]
    pub string_value: ::std::option::Option<::std::boxed::Box<TruncatableString>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A set of attributes, each in the format `[KEY]:[VALUE]`."]
pub struct Attributes {
    #[serde(rename = "attributeMap")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of attributes. Each attribute's key can be up to 128 bytes long. The value can be a string up to 256 bytes, a signed 64-bit integer, or the Boolean values `true` and `false`. For example: \"/instance_id\": \"my-instance\" \"/http/user_agent\": \"\" \"/http/request_bytes\": 300 \"abc.com/myattribute\": true"]
    pub attribute_map: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<AttributeValue>>,
    >,
    #[serde(rename = "droppedAttributesCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of attributes that were discarded. Attributes can be discarded because their keys are too long or because there are too many attributes. If this value is 0 then all attributes are valid."]
    pub dropped_attributes_count: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { \"audit_configs\": [ { \"service\": \"allServices\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" }, { \"log_type\": \"ADMIN_READ\" } ] }, { \"service\": \"sampleservice.googleapis.com\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\" }, { \"log_type\": \"DATA_WRITE\", \"exempted_members\": [ \"user:aliya@example.com\" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts jose@example.com from DATA_READ logging, and aliya@example.com from DATA_WRITE logging."]
pub struct AuditConfig {
    #[serde(rename = "auditLogConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration for logging of each type of permission."]
    pub audit_log_configs:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuditLogConfig>>>,
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services."]
    pub service: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Provides the configuration for logging a type of permissions. Example: { \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging."]
pub struct AuditLogConfig {
    #[serde(rename = "exemptedMembers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members."]
    pub exempted_members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "logType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The log type that this config enables."]
    pub log_type: ::std::option::Option<AuditLogConfigLogTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The log type that this config enables."]
pub enum AuditLogConfigLogTypeEnum {
    #[serde(rename = "LOG_TYPE_UNSPECIFIED")]
    #[doc = "Default case. Should never be this."]
    LogTypeUnspecified,
    #[serde(rename = "ADMIN_READ")]
    #[doc = "Admin reads. Example: CloudIAM getIamPolicy"]
    AdminRead,
    #[serde(rename = "DATA_WRITE")]
    #[doc = "Data writes. Example: CloudSQL Users create"]
    DataWrite,
    #[serde(rename = "DATA_READ")]
    #[doc = "Data reads. Example: CloudSQL Users list"]
    DataRead,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message for reporting billing requests through Eventstream."]
pub struct BillingView {
    #[serde(rename = "reportRequests")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Billing requests to be reported for cloud.eventstream.v2.ResourceEvent Each request contains billing operations to be reported under a service name. See go/billing-view-construction for documentation on constructing billing view report requests."]
    pub report_requests: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ReportRequest>>>,
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
#[doc = "Distribution represents a frequency distribution of double-valued sample points. It contains the size of the population of sample points plus additional optional information: - the arithmetic mean of the samples - the minimum and maximum of the samples - the sum-squared-deviation of the samples, used to compute variance - a histogram of the values of the sample points"]
pub struct Distribution {
    #[serde(rename = "bucketCounts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of samples in each histogram bucket. `bucket_counts` are optional. If present, they must sum to the `count` value. The buckets are defined below in `bucket_option`. There are N buckets. `bucket_counts[0]` is the number of samples in the underflow bucket. `bucket_counts[1]` to `bucket_counts[N-1]` are the numbers of samples in each of the finite buckets. And `bucket_counts[N] is the number of samples in the overflow bucket. See the comments of `bucket_option` below for more details. Any suffix of trailing zeros may be omitted."]
    pub bucket_counts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total number of samples in the distribution. Must be >= 0."]
    pub count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "exemplars")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Example points. Must be in increasing order of `value` field."]
    pub exemplars: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Exemplar>>>,
    #[serde(rename = "explicitBuckets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Buckets with arbitrary user-provided width."]
    pub explicit_buckets: ::std::option::Option<::std::boxed::Box<ExplicitBuckets>>,
    #[serde(rename = "exponentialBuckets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Buckets with exponentially growing width."]
    pub exponential_buckets: ::std::option::Option<::std::boxed::Box<ExponentialBuckets>>,
    #[serde(rename = "linearBuckets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Buckets with constant width."]
    pub linear_buckets: ::std::option::Option<::std::boxed::Box<LinearBuckets>>,
    #[serde(rename = "maximum")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum of the population of values. Ignored if `count` is zero."]
    pub maximum: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "mean")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The arithmetic mean of the samples in the distribution. If `count` is zero then this field must be zero."]
    pub mean: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "minimum")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minimum of the population of values. Ignored if `count` is zero."]
    pub minimum: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "sumOfSquaredDeviation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sum of squared deviations from the mean: Sum[i=1..count]((x_i - mean)^2) where each x_i is a sample values. If `count` is zero then this field must be zero, otherwise validation of the request fails."]
    pub sum_of_squared_deviation: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Exemplars are example points that may be used to annotate aggregated distribution values. They are metadata that gives information about a particular value added to a Distribution bucket, such as a trace ID that was active when a value was added. They may contain further information, such as a example values and timestamps, origin, etc."]
pub struct Exemplar {
    #[serde(rename = "attachments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Contextual information about the example value. Examples are: Trace: type.googleapis.com/google.monitoring.v3.SpanContext Literal string: type.googleapis.com/google.protobuf.StringValue Labels dropped during aggregation: type.googleapis.com/google.monitoring.v3.DroppedLabels There may be only a single attachment of any given message type in a single exemplar, and this is enforced by the system."]
    pub attachments: ::std::option::Option<
        ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    >,
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The observation (sampling) time of the above value."]
    pub timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Value of the exemplar point. This value determines to which bucket the exemplar belongs."]
    pub value: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describing buckets with arbitrary user-provided width."]
pub struct ExplicitBuckets {
    #[serde(rename = "bounds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "'bound' is a list of strictly increasing boundaries between buckets. Note that a list of length N-1 defines N buckets because of fenceposting. See comments on `bucket_options` for details. The i'th finite bucket covers the interval [bound[i-1], bound[i]) where i ranges from 1 to bound_size() - 1. Note that there are no finite buckets at all if 'bound' only contains a single element; in that special case the single bound defines the boundary between the underflow and overflow buckets. bucket number lower bound upper bound i == 0 (underflow) -inf bound[i] 0 < i < bound_size() bound[i-1] bound[i] i == bound_size() (overflow) bound[i-1] +inf"]
    pub bounds: ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describing buckets with exponentially growing width."]
pub struct ExponentialBuckets {
    #[serde(rename = "growthFactor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The i'th exponential bucket covers the interval [scale * growth_factor^(i-1), scale * growth_factor^i) where i ranges from 1 to num_finite_buckets inclusive. Must be larger than 1.0."]
    pub growth_factor: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "numFiniteBuckets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of finite buckets. With the underflow and overflow buckets, the total number of buckets is `num_finite_buckets` + 2. See comments on `bucket_options` for details."]
    pub num_finite_buckets: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "scale")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The i'th exponential bucket covers the interval [scale * growth_factor^(i-1), scale * growth_factor^i) where i ranges from 1 to num_finite_buckets inclusive. Must be > 0."]
    pub scale: ::std::option::Option<::std::primitive::f64>,
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
#[doc = "The request message for Operations.CancelOperation."]
pub struct GoogleLongrunningCancelOperationRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for Operations.ListOperations."]
pub struct GoogleLongrunningListOperationsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The standard List next-page token."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of operations that matches the specified filter in the request."]
    pub operations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleLongrunningOperation>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This resource represents a long-running operation that is the result of a network API call."]
pub struct GoogleLongrunningOperation {
    #[serde(rename = "done")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
    pub done: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The error result of the operation in case of failure or cancellation."]
    pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any."]
    pub metadata: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "response")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`."]
    pub response: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
pub struct GoogleRpcStatus {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status code, which should be an enum value of google.rpc.Code."]
    pub code: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of messages that carry the error details. There is a common set of message types for APIs to use."]
    pub details: ::std::option::Option<
        ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    >,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client."]
    pub message: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A common proto for logging HTTP requests. Only contains semantics defined by the HTTP specification. Product-specific logging information MUST be defined in a separate message."]
pub struct HttpRequest {
    #[serde(rename = "cacheFillBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of HTTP response bytes inserted into cache. Set only when a cache fill was attempted."]
    pub cache_fill_bytes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cacheHit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not an entity was served from cache (with or without validation)."]
    pub cache_hit: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "cacheLookup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not a cache lookup was attempted."]
    pub cache_lookup: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "cacheValidatedWithOriginServer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not the response was validated with the origin server before being served from cache. This field is only meaningful if `cache_hit` is True."]
    pub cache_validated_with_origin_server: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "latency")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request processing latency on the server, from the time the request was received until the response was sent."]
    pub latency: ::std::option::Option<::std::string::String>,
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Protocol used for the request. Examples: \"HTTP/1.1\", \"HTTP/2\", \"websocket\""]
    pub protocol: ::std::option::Option<::std::string::String>,
    #[serde(rename = "referer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The referer URL of the request, as defined in [HTTP/1.1 Header Field Definitions](http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html)."]
    pub referer: ::std::option::Option<::std::string::String>,
    #[serde(rename = "remoteIp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IP address (IPv4 or IPv6) of the client that issued the HTTP request. Examples: `\"192.168.1.1\"`, `\"FE80::0202:B3FF:FE1E:8329\"`."]
    pub remote_ip: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestMethod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request method. Examples: `\"GET\"`, `\"HEAD\"`, `\"PUT\"`, `\"POST\"`."]
    pub request_method: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The size of the HTTP request message in bytes, including the request headers and the request body."]
    pub request_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The scheme (http, https), the host name, the path, and the query portion of the URL that was requested. Example: `\"http://example.com/some/info?color=red\"`."]
    pub request_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "responseSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The size of the HTTP response message sent back to the client, in bytes, including the response headers and the response body."]
    pub response_size: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serverIp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IP address (IPv4 or IPv6) of the origin server that the request was sent to."]
    pub server_ip: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The response code indicating the status of the response. Examples: 200, 404."]
    pub status: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "userAgent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user agent sent by the client. Example: `\"Mozilla/4.0 (compatible; MSIE 6.0; Windows 98; Q312461; .NET CLR 1.0.3705)\"`."]
    pub user_agent: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Network Connectivity Center is a hub-and-spoke abstraction for network connectivity management in Google Cloud. It reduces operational complexity through a simple, centralized connectivity management model. Following is the resource message of a hub."]
pub struct Hub {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time when the Hub was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Short description of the hub resource."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User-defined labels."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The name of a Hub resource."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "spokes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A list of the URIs of all attached spokes"]
    pub spokes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The current lifecycle state of this Hub."]
    pub state: ::std::option::Option<HubStateEnum>,
    #[serde(rename = "uniqueId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Google-generated UUID for this resource. This is unique across all Hub resources. If a Hub resource is deleted and another with the same name is created, it gets a different unique_id."]
    pub unique_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time when the Hub was updated."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The current lifecycle state of this Hub."]
pub enum HubStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "No state information available"]
    StateUnspecified,
    #[serde(rename = "CREATING")]
    #[doc = "The resource's create operation is in progress"]
    Creating,
    #[serde(rename = "ACTIVE")]
    #[doc = "The resource is active"]
    Active,
    #[serde(rename = "DELETING")]
    #[doc = "The resource's Delete operation is in progress"]
    Deleting,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describing buckets with constant width."]
pub struct LinearBuckets {
    #[serde(rename = "numFiniteBuckets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of finite buckets. With the underflow and overflow buckets, the total number of buckets is `num_finite_buckets` + 2. See comments on `bucket_options` for details."]
    pub num_finite_buckets: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The i'th linear bucket covers the interval [offset + (i-1) * width, offset + i * width) where i ranges from 1 to num_finite_buckets, inclusive."]
    pub offset: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The i'th linear bucket covers the interval [offset + (i-1) * width, offset + i * width) where i ranges from 1 to num_finite_buckets, inclusive. Must be strictly positive."]
    pub width: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for HubService.ListHubs method."]
pub struct ListHubsResponse {
    #[serde(rename = "hubs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Hubs to be returned."]
    pub hubs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Hub>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The next pagination token in the List response. It should be used as page_token for the following request. An empty value means no more result."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "unreachable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Locations that could not be reached."]
    pub unreachable: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for Locations.ListLocations."]
pub struct ListLocationsResponse {
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of locations that matches the specified filter in the request."]
    pub locations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Location>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The standard List next-page token."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for HubService.ListSpokes."]
pub struct ListSpokesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The next pagination token in the List response. It should be used as page_token for the following request. An empty value means no more result."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "spokes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Spokes to be returned."]
    pub spokes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Spoke>>>,
    #[serde(rename = "unreachable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Locations that could not be reached."]
    pub unreachable: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A resource that represents Google Cloud Platform location."]
pub struct Location {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The friendly name for this location, typically a nearby city name. For example, \"Tokyo\"."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cross-service attributes for the location. For example {\"cloud.googleapis.com/region\": \"us-east1\"}"]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The canonical id for this location. For example: `\"us-east1\"`."]
    pub location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Service-specific metadata. For example the available capacity at the given location."]
    pub metadata: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name for the location, which may vary between implementations. For example: `\"projects/example-project/locations/us-east1\"`"]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An individual log entry."]
pub struct LogEntry {
    #[serde(rename = "httpRequest")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Information about the HTTP request associated with this log entry, if applicable."]
    pub http_request: ::std::option::Option<::std::boxed::Box<HttpRequest>>,
    #[serde(rename = "insertId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A unique ID for the log entry used for deduplication. If omitted, the implementation will generate one based on operation_id."]
    pub insert_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A set of user-defined (key, value) data that provides additional information about the log entry."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The log to which this log entry belongs. Examples: `\"syslog\"`, `\"book_log\"`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Information about an operation associated with the log entry, if applicable."]
    pub operation: ::std::option::Option<::std::boxed::Box<LogEntryOperation>>,
    #[serde(rename = "protoPayload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The log entry payload, represented as a protocol buffer that is expressed as a JSON object. The only accepted type currently is AuditLog."]
    pub proto_payload:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The severity of the log entry. The default value is `LogSeverity.DEFAULT`."]
    pub severity: ::std::option::Option<LogEntrySeverityEnum>,
    #[serde(rename = "sourceLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Source code location information associated with the log entry, if any."]
    pub source_location: ::std::option::Option<::std::boxed::Box<LogEntrySourceLocation>>,
    #[serde(rename = "structPayload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The log entry payload, represented as a structure that is expressed as a JSON object."]
    pub struct_payload:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "textPayload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The log entry payload, represented as a Unicode string (UTF-8)."]
    pub text_payload: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time the event described by the log entry occurred. If omitted, defaults to operation start time."]
    pub timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "trace")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Resource name of the trace associated with the log entry, if any. If this field contains a relative resource name, you can assume the name is relative to `//tracing.googleapis.com`. Example: `projects/my-projectid/traces/06796866738c859f2f19b7cfb3214824`"]
    pub trace: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The severity of the log entry. The default value is `LogSeverity.DEFAULT`."]
pub enum LogEntrySeverityEnum {
    #[serde(rename = "DEFAULT")]
    #[doc = "(0) The log entry has no assigned severity level."]
    Default,
    #[serde(rename = "DEBUG")]
    #[doc = "(100) Debug or trace information."]
    Debug,
    #[serde(rename = "INFO")]
    #[doc = "(200) Routine information, such as ongoing status or performance."]
    Info,
    #[serde(rename = "NOTICE")]
    #[doc = "(300) Normal but significant events, such as start up, shut down, or a configuration change."]
    Notice,
    #[serde(rename = "WARNING")]
    #[doc = "(400) Warning events might cause problems."]
    Warning,
    #[serde(rename = "ERROR")]
    #[doc = "(500) Error events are likely to cause problems."]
    Error,
    #[serde(rename = "CRITICAL")]
    #[doc = "(600) Critical events cause more severe problems or outages."]
    Critical,
    #[serde(rename = "ALERT")]
    #[doc = "(700) A person must take an action immediately."]
    Alert,
    #[serde(rename = "EMERGENCY")]
    #[doc = "(800) One or more systems are unusable."]
    Emergency,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Additional information about a potentially long-running operation with which a log entry is associated."]
pub struct LogEntryOperation {
    #[serde(rename = "first")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Set this to True if this is the first log entry in the operation."]
    pub first: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. An arbitrary operation identifier. Log entries with the same identifier are assumed to be part of the same operation."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "last")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Set this to True if this is the last log entry in the operation."]
    pub last: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "producer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. An arbitrary producer identifier. The combination of `id` and `producer` must be globally unique. Examples for `producer`: `\"MyDivision.MyBigCompany.com\"`, `\"github.com/MyProject/MyApplication\"`."]
    pub producer: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Additional information about the source code location that produced the log entry."]
pub struct LogEntrySourceLocation {
    #[serde(rename = "file")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Source file name. Depending on the runtime environment, this might be a simple name or a fully-qualified name."]
    pub file: ::std::option::Option<::std::string::String>,
    #[serde(rename = "function")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Human-readable name of the function or method being invoked, with optional context such as the class or package name. This information may be used in contexts such as the logs viewer, where a file and line number are less meaningful. The format can vary by language. For example: `qual.if.ied.Class.method` (Java), `dir/package.func` (Go), `function` (Python)."]
    pub function: ::std::option::Option<::std::string::String>,
    #[serde(rename = "line")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Line within the source file. 1-based; 0 indicates no line number available."]
    pub line: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a single metric value."]
pub struct MetricValue {
    #[serde(rename = "boolValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A boolean value."]
    pub bool_value: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "distributionValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A distribution value."]
    pub distribution_value: ::std::option::Option<::std::boxed::Box<Distribution>>,
    #[serde(rename = "doubleValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A double precision floating point value."]
    pub double_value: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The end of the time period over which this metric value's measurement applies. If not specified, google.api.servicecontrol.v1.Operation.end_time will be used."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "int64Value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A signed 64-bit integer value."]
    pub int64_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The labels describing the metric value. See comments on google.api.servicecontrol.v1.Operation.labels for the overriding relationship. Note that this map must not contain monitored resource labels."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "moneyValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A money value."]
    pub money_value: ::std::option::Option<::std::boxed::Box<Money>>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The start of the time period over which this metric value's measurement applies. The time period has different semantics for different metric types (cumulative, delta, and gauge). See the metric definition documentation in the service configuration for details. If not specified, google.api.servicecontrol.v1.Operation.start_time will be used."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "stringValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A text string value."]
    pub string_value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a set of metric values in the same metric. Each metric value in the set should have a unique combination of start time, end time, and label values."]
pub struct MetricValueSet {
    #[serde(rename = "metricName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The metric name defined in the service configuration."]
    pub metric_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metricValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The values in this metric."]
    pub metric_values: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricValue>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an amount of money with its currency type."]
pub struct Money {
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The three-letter currency code defined in ISO 4217."]
    pub currency_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nanos")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000."]
    pub nanos: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "units")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The whole units of the amount. For example if `currencyCode` is `\"USD\"`, then 1 unit is one US dollar."]
    pub units: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents information regarding an operation."]
pub struct Operation {
    #[serde(rename = "consumerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identity of the consumer who is using the service. This field should be filled in for the operations initiated by a consumer, but not for service-initiated operations that are not related to a specific consumer. - This can be in one of the following formats: - project:PROJECT_ID, - project`_`number:PROJECT_NUMBER, - projects/PROJECT_ID or PROJECT_NUMBER, - folders/FOLDER_NUMBER, - organizations/ORGANIZATION_NUMBER, - api`_`key:API_KEY."]
    pub consumer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "End time of the operation. Required when the operation is used in ServiceController.Report, but optional when the operation is used in ServiceController.Check."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "extensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unimplemented."]
    pub extensions: ::std::option::Option<
        ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    >,
    #[serde(rename = "importance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DO NOT USE. This is an experimental field."]
    pub importance: ::std::option::Option<OperationImportanceEnum>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Labels describing the operation. Only the following labels are allowed: - Labels describing monitored resources as defined in the service configuration. - Default labels of metric values. When specified, labels defined in the metric value override these default. - The following labels defined by Google Cloud Platform: - `cloud.googleapis.com/location` describing the location where the operation happened, - `servicecontrol.googleapis.com/user_agent` describing the user agent of the API request, - `servicecontrol.googleapis.com/service_agent` describing the service used to handle the API request (e.g. ESP), - `servicecontrol.googleapis.com/platform` describing the platform where the API is served, such as App Engine, Compute Engine, or Kubernetes Engine."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "logEntries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents information to be logged."]
    pub log_entries: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LogEntry>>>,
    #[serde(rename = "metricValueSets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents information about this operation. Each MetricValueSet corresponds to a metric defined in the service configuration. The data type used in the MetricValueSet must agree with the data type specified in the metric definition. Within a single operation, it is not allowed to have more than one MetricValue instances that have the same metric names and identical label value combinations. If a request has such duplicated MetricValue instances, the entire request is rejected with an invalid argument error."]
    pub metric_value_sets:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetricValueSet>>>,
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identity of the operation. This must be unique within the scope of the service that generated the operation. If the service calls Check() and Report() on the same operation, the two calls should carry the same id. UUID version 4 is recommended, though not required. In scenarios where an operation is computed from existing information and an idempotent id is desirable for deduplication purpose, UUID version 5 is recommended. See RFC 4122 for details."]
    pub operation_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Fully qualified name of the operation. Reserved for future use."]
    pub operation_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quotaProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents the properties needed for quota check. Applicable only if this operation is for a quota check request. If this is not specified, no quota check will be performed."]
    pub quota_properties: ::std::option::Option<::std::boxed::Box<QuotaProperties>>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resources that are involved in the operation. The maximum supported number of entries in this field is 100."]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ResourceInfo>>>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Start time of the operation."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "traceSpans")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unimplemented. A list of Cloud Trace spans. The span names shall contain the id of the destination project which can be either the produce or the consumer project."]
    pub trace_spans: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TraceSpan>>>,
    #[serde(rename = "userLabels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Private Preview. This feature is only available for approved services. User defined labels for the resource that this operation is associated with. Only a combination of 1000 user labels per consumer project are allowed."]
    pub user_labels:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "DO NOT USE. This is an experimental field."]
pub enum OperationImportanceEnum {
    #[serde(rename = "LOW")]
    #[doc = "Allows data caching, batching, and aggregation. It provides higher performance with higher data loss risk."]
    Low,
    #[serde(rename = "HIGH")]
    #[doc = "Disables data aggregation to minimize data loss. It is for operations that contains significant monetary value or audit trail. This feature only applies to the client libraries."]
    High,
    #[serde(rename = "DEBUG")]
    #[doc = "Deprecated. Do not use. Disables data aggregation and enables additional validation logic. It should only be used during the onboarding process. It is only available to Google internal services, and the service must be approved by chemist-dev@google.com in order to use this level."]
    Debug,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the metadata of the long-running operation."]
pub struct OperationMetadata {
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. API version used to start the operation."]
    pub api_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time the operation was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time the operation finished running."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestedCancellation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Identifies whether the user has requested cancellation of the operation. Operations that have successfully been cancelled have Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."]
    pub requested_cancellation: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Human-readable status of the operation, if any."]
    pub status_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Server-defined resource path for the target of the operation."]
    pub target: ::std::option::Option<::std::string::String>,
    #[serde(rename = "verb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Name of the verb executed by the operation."]
    pub verb: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members` to a single `role`. Members can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { \"bindings\": [ { \"role\": \"roles/resourcemanager.organizationAdmin\", \"members\": [ \"user:mike@example.com\", \"group:admins@example.com\", \"domain:google.com\", \"serviceAccount:my-project-id@appspot.gserviceaccount.com\" ] }, { \"role\": \"roles/resourcemanager.organizationViewer\", \"members\": [ \"user:eve@example.com\" ], \"condition\": { \"title\": \"expirable access\", \"description\": \"Does not grant access after Sep 2020\", \"expression\": \"request.time < timestamp('2020-10-01T00:00:00.000Z')\", } } ], \"etag\": \"BwWWja0YfJA=\", \"version\": 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') - etag: BwWWja0YfJA= - version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/)."]
pub struct Policy {
    #[serde(rename = "auditConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies cloud audit logging configuration for this policy."]
    pub audit_configs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuditConfig>>>,
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
#[doc = "Represents the properties needed for quota operations."]
pub struct QuotaProperties {
    #[serde(rename = "quotaMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Quota mode for this operation."]
    pub quota_mode: ::std::option::Option<QuotaPropertiesQuotaModeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Quota mode for this operation."]
pub enum QuotaPropertiesQuotaModeEnum {
    #[serde(rename = "ACQUIRE")]
    #[doc = "Decreases available quota by the cost specified for the operation. If cost is higher than available quota, operation fails and returns error."]
    Acquire,
    #[serde(rename = "ACQUIRE_BEST_EFFORT")]
    #[doc = "Decreases available quota by the cost specified for the operation. If cost is higher than available quota, operation does not fail and available quota goes down to zero but it returns error."]
    AcquireBestEffort,
    #[serde(rename = "CHECK")]
    #[doc = "Does not change any available quota. Only checks if there is enough quota. No lock is placed on the checked tokens neither."]
    Check,
    #[serde(rename = "RELEASE")]
    #[doc = "Increases available quota by the operation cost specified for the operation."]
    Release,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for the Report method."]
pub struct ReportRequest {
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Operations to be reported. Typically the service should report one operation per request. Putting multiple operations into a single request is allowed, but should be used only when multiple operations are natually available at the time of the report. There is no limit on the number of operations in the same ReportRequest, however the ReportRequest size should be no larger than 1MB. See ReportResponse.report_errors for partial failure behavior."]
    pub operations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Operation>>>,
    #[serde(rename = "serviceConfigId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies which version of service config should be used to process the request. If unspecified or no matching version can be found, the latest one will be used."]
    pub service_config_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serviceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The service name as specified in its service configuration. For example, `\"pubsub.googleapis.com\"`. See [google.api.Service](https://cloud.google.com/service-management/reference/rpc/google.api#google.api.Service) for the definition of a service name."]
    pub service_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes a resource associated with this operation."]
pub struct ResourceInfo {
    #[serde(rename = "resourceContainer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier of the parent of this resource instance. Must be in one of the following formats: - `projects/` - `folders/` - `organizations/`"]
    pub resource_container: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The location of the resource. If not empty, the resource will be checked against location policy. The value must be a valid zone, region or multiregion. For example: \"europe-west4\" or \"northamerica-northeast1-a\""]
    pub resource_location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the resource. This is used for auditing purposes."]
    pub resource_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "RouterAppliance represents a Router appliance which is specified by a VM URI and a NIC address."]
pub struct RouterApplianceInstance {
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IP address of the network interface to use for peering."]
    pub ip_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "networkInterface")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub network_interface: ::std::option::Option<::std::string::String>,
    #[serde(rename = "virtualMachine")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI of the virtual machine resource"]
    pub virtual_machine: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for `SetIamPolicy` method."]
pub struct SetIamPolicyRequest {
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Cloud Platform services (such as Projects) might reject them."]
    pub policy: ::std::option::Option<::std::boxed::Box<Policy>>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: \"bindings, etag\"`"]
    pub update_mask: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Spoke is an abstraction of a network attachment being attached to a Hub. A Spoke can be underlying a VPN tunnel, a VLAN (interconnect) attachment, a Router appliance, etc."]
pub struct Spoke {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time when the Spoke was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Short description of the spoke resource"]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hub")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource URL of the hub resource that the spoke is attached to"]
    pub hub: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User-defined labels."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "linkedInterconnectAttachments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URIs of linked interconnect attachment resources"]
    pub linked_interconnect_attachments:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "linkedRouterApplianceInstances")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URIs of linked Router appliance resources"]
    pub linked_router_appliance_instances:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RouterApplianceInstance>>>,
    #[serde(rename = "linkedVpnTunnels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URIs of linked VPN tunnel resources"]
    pub linked_vpn_tunnels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The name of a Spoke resource."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The current lifecycle state of this Hub."]
    pub state: ::std::option::Option<SpokeStateEnum>,
    #[serde(rename = "uniqueId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Google-generated UUID for this resource. This is unique across all Spoke resources. If a Spoke resource is deleted and another with the same name is created, it gets a different unique_id."]
    pub unique_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time when the Spoke was updated."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The current lifecycle state of this Hub."]
pub enum SpokeStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "No state information available"]
    StateUnspecified,
    #[serde(rename = "CREATING")]
    #[doc = "The resource's create operation is in progress"]
    Creating,
    #[serde(rename = "ACTIVE")]
    #[doc = "The resource is active"]
    Active,
    #[serde(rename = "DELETING")]
    #[doc = "The resource's Delete operation is in progress"]
    Deleting,
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
#[doc = "A span represents a single operation within a trace. Spans can be nested to form a trace tree. Often, a trace contains a root span that describes the end-to-end latency, and one or more subspans for its sub-operations. A trace can also contain multiple root spans, or none at all. Spans do not need to be contiguous—there may be gaps or overlaps between spans in a trace."]
pub struct TraceSpan {
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A set of attributes on the span. You can have up to 32 attributes per span."]
    pub attributes: ::std::option::Option<::std::boxed::Box<Attributes>>,
    #[serde(rename = "childSpanCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An optional number of child spans that were generated while this span was active. If set, allows implementation to detect missing child spans."]
    pub child_span_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A description of the span's operation (up to 128 bytes). Stackdriver Trace displays the description in the Google Cloud Platform Console. For example, the display name can be a qualified method name or a file name and a line number where the operation is called. A best practice is to use the same display name within an application and at the same call point. This makes it easier to correlate spans in different traces."]
    pub display_name: ::std::option::Option<::std::boxed::Box<TruncatableString>>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The end time of the span. On the client side, this is the time kept by the local machine where the span execution ends. On the server side, this is the time when the server application handler stops running."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the span in the following format: projects/[PROJECT_ID]/traces/[TRACE_ID]/spans/SPAN_ID is a unique identifier for a trace within a project; it is a 32-character hexadecimal encoding of a 16-byte array. [SPAN_ID] is a unique identifier for a span within a trace; it is a 16-character hexadecimal encoding of an 8-byte array."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parentSpanId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The [SPAN_ID] of this span's parent span. If this is a root span, then this field must be empty."]
    pub parent_span_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sameProcessAsParentSpan")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "(Optional) Set this parameter to indicate whether this span is in the same process as its parent. If you do not set this parameter, Stackdriver Trace is unable to take advantage of this helpful information."]
    pub same_process_as_parent_span: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "spanId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The [SPAN_ID] portion of the span's resource name."]
    pub span_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "spanKind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Distinguishes between spans generated in a particular context. For example, two spans with the same name may be distinguished using `CLIENT` (caller) and `SERVER` (callee) to identify an RPC call."]
    pub span_kind: ::std::option::Option<TraceSpanSpanKindEnum>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The start time of the span. On the client side, this is the time kept by the local machine where the span execution starts. On the server side, this is the time when the server's application handler starts running."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An optional final status for this span."]
    pub status: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Distinguishes between spans generated in a particular context. For example, two spans with the same name may be distinguished using `CLIENT` (caller) and `SERVER` (callee) to identify an RPC call."]
pub enum TraceSpanSpanKindEnum {
    #[serde(rename = "SPAN_KIND_UNSPECIFIED")]
    #[doc = "Unspecified. Do NOT use as default. Implementations MAY assume SpanKind.INTERNAL to be default."]
    SpanKindUnspecified,
    #[serde(rename = "INTERNAL")]
    #[doc = "Indicates that the span is used internally. Default value."]
    Internal,
    #[serde(rename = "SERVER")]
    #[doc = "Indicates that the span covers server-side handling of an RPC or other remote network request."]
    Server,
    #[serde(rename = "CLIENT")]
    #[doc = "Indicates that the span covers the client-side wrapper around an RPC or other remote request."]
    Client,
    #[serde(rename = "PRODUCER")]
    #[doc = "Indicates that the span describes producer sending a message to a broker. Unlike client and server, there is no direct critical path latency relationship between producer and consumer spans (e.g. publishing a message to a pubsub service)."]
    Producer,
    #[serde(rename = "CONSUMER")]
    #[doc = "Indicates that the span describes consumer receiving a message from a broker. Unlike client and server, there is no direct critical path latency relationship between producer and consumer spans (e.g. receiving a message from a pubsub service subscription)."]
    Consumer,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a string that might be shortened to a specified length."]
pub struct TruncatableString {
    #[serde(rename = "truncatedByteCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of bytes removed from the original string. If this value is 0, then the string was not shortened."]
    pub truncated_byte_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The shortened string. For example, if the original string is 500 bytes long and the limit of the string is 128 bytes, then `value` contains the first 128 bytes of the 500-byte string. Truncation always happens on a UTF8 character boundary. If there are multi-byte characters in the string, then the length of the shortened string might be less than the size limit."]
    pub value: ::std::option::Option<::std::string::String>,
}
