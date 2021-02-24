#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for Operations.CancelOperation."]
pub struct CancelOperationRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes a Compute Engine resource that is being managed by a running pipeline."]
pub struct ComputeEngine {
    #[serde(rename = "diskNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The names of the disks that were created for this pipeline."]
    pub disk_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "instanceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The instance on which the operation is running."]
    pub instance_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "machineType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The machine type of the instance."]
    pub machine_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The availability zone in which the instance resides."]
    pub zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event generated when a container is forcibly terminated by the worker. Currently, this only occurs when the container outlives the timeout specified by the user."]
pub struct ContainerKilledEvent {
    #[serde(rename = "actionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The numeric ID of the action that started the container."]
    pub action_id: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event generated when a container starts."]
pub struct ContainerStartedEvent {
    #[serde(rename = "actionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The numeric ID of the action that started this container."]
    pub action_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The public IP address that can be used to connect to the container. This field is only populated when at least one port mapping is present. If the instance was created with a private address, this field will be empty even if port mappings exist."]
    pub ip_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "portMappings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The container-to-host port mappings installed for this container. This set will contain any ports exposed using the `PUBLISH_EXPOSED_PORTS` flag as well as any specified in the `Action` definition."]
    pub port_mappings:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::primitive::i64>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event generated when a container exits."]
pub struct ContainerStoppedEvent {
    #[serde(rename = "actionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The numeric ID of the action that started this container."]
    pub action_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "exitStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The exit status of the container."]
    pub exit_status: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "stderr")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The tail end of any content written to standard error by the container. If the content emits large amounts of debugging noise or contains sensitive information, you can prevent the content from being printed by setting the `DISABLE_STANDARD_ERROR_CAPTURE` flag. Note that only a small amount of the end of the stream is captured here. The entire stream is stored in the `/google/logs` directory mounted into each action, and can be copied off the machine as described elsewhere."]
    pub stderr: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event generated whenever a resource limitation or transient error delays execution of a pipeline that was otherwise ready to run."]
pub struct DelayedEvent {
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A textual description of the cause of the delay. The string can change without notice because it is often generated by another service (such as Compute Engine)."]
    pub cause: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the delay was caused by a resource shortage, this field lists the Compute Engine metrics that are preventing this operation from running (for example, `CPUS` or `INSTANCES`). If the particular metric is not known, a single `UNKNOWN` metric will be present."]
    pub metrics: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Carries information about events that occur during pipeline execution."]
pub struct Event {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A human-readable description of the event. Note that these strings can change at any time without notice. Any application logic must use the information in the `details` field."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Machine-readable details about the event."]
    pub details: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the event occurred."]
    pub timestamp: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event generated when the execution of a pipeline has failed. Note that other events can continue to occur after this event."]
pub struct FailedEvent {
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The human-readable description of the cause of the failure."]
    pub cause: ::std::option::Option<::std::string::String>,
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Google standard error code that best describes this failure."]
    pub code: ::std::option::Option<FailedEventCodeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The Google standard error code that best describes this failure."]
pub enum FailedEventCodeEnum {
    #[serde(rename = "OK")]
    #[doc = "Not an error; returned on success HTTP Mapping: 200 OK"]
    Ok,
    #[serde(rename = "CANCELLED")]
    #[doc = "The operation was cancelled, typically by the caller. HTTP Mapping: 499 Client Closed Request"]
    Cancelled,
    #[serde(rename = "UNKNOWN")]
    #[doc = "Unknown error. For example, this error may be returned when a `Status` value received from another address space belongs to an error space that is not known in this address space. Also errors raised by APIs that do not return enough error information may be converted to this error. HTTP Mapping: 500 Internal Server Error"]
    Unknown,
    #[serde(rename = "INVALID_ARGUMENT")]
    #[doc = "The client specified an invalid argument. Note that this differs from `FAILED_PRECONDITION`. `INVALID_ARGUMENT` indicates arguments that are problematic regardless of the state of the system (e.g., a malformed file name). HTTP Mapping: 400 Bad Request"]
    InvalidArgument,
    #[serde(rename = "DEADLINE_EXCEEDED")]
    #[doc = "The deadline expired before the operation could complete. For operations that change the state of the system, this error may be returned even if the operation has completed successfully. For example, a successful response from a server could have been delayed long enough for the deadline to expire. HTTP Mapping: 504 Gateway Timeout"]
    DeadlineExceeded,
    #[serde(rename = "NOT_FOUND")]
    #[doc = "Some requested entity (e.g., file or directory) was not found. Note to server developers: if a request is denied for an entire class of users, such as gradual feature rollout or undocumented allowlist, `NOT_FOUND` may be used. If a request is denied for some users within a class of users, such as user-based access control, `PERMISSION_DENIED` must be used. HTTP Mapping: 404 Not Found"]
    NotFound,
    #[serde(rename = "ALREADY_EXISTS")]
    #[doc = "The entity that a client attempted to create (e.g., file or directory) already exists. HTTP Mapping: 409 Conflict"]
    AlreadyExists,
    #[serde(rename = "PERMISSION_DENIED")]
    #[doc = "The caller does not have permission to execute the specified operation. `PERMISSION_DENIED` must not be used for rejections caused by exhausting some resource (use `RESOURCE_EXHAUSTED` instead for those errors). `PERMISSION_DENIED` must not be used if the caller can not be identified (use `UNAUTHENTICATED` instead for those errors). This error code does not imply the request is valid or the requested entity exists or satisfies other pre-conditions. HTTP Mapping: 403 Forbidden"]
    PermissionDenied,
    #[serde(rename = "UNAUTHENTICATED")]
    #[doc = "The request does not have valid authentication credentials for the operation. HTTP Mapping: 401 Unauthorized"]
    Unauthenticated,
    #[serde(rename = "RESOURCE_EXHAUSTED")]
    #[doc = "Some resource has been exhausted, perhaps a per-user quota, or perhaps the entire file system is out of space. HTTP Mapping: 429 Too Many Requests"]
    ResourceExhausted,
    #[serde(rename = "FAILED_PRECONDITION")]
    #[doc = "The operation was rejected because the system is not in a state required for the operation's execution. For example, the directory to be deleted is non-empty, an rmdir operation is applied to a non-directory, etc. Service implementors can use the following guidelines to decide between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`: (a) Use `UNAVAILABLE` if the client can retry just the failing call. (b) Use `ABORTED` if the client should retry at a higher level (e.g., when a client-specified test-and-set fails, indicating the client should restart a read-modify-write sequence). (c) Use `FAILED_PRECONDITION` if the client should not retry until the system state has been explicitly fixed. E.g., if an \"rmdir\" fails because the directory is non-empty, `FAILED_PRECONDITION` should be returned since the client should not retry unless the files are deleted from the directory. HTTP Mapping: 400 Bad Request"]
    FailedPrecondition,
    #[serde(rename = "ABORTED")]
    #[doc = "The operation was aborted, typically due to a concurrency issue such as a sequencer check failure or transaction abort. See the guidelines above for deciding between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`. HTTP Mapping: 409 Conflict"]
    Aborted,
    #[serde(rename = "OUT_OF_RANGE")]
    #[doc = "The operation was attempted past the valid range. E.g., seeking or reading past end-of-file. Unlike `INVALID_ARGUMENT`, this error indicates a problem that may be fixed if the system state changes. For example, a 32-bit file system will generate `INVALID_ARGUMENT` if asked to read at an offset that is not in the range [0,2^32-1], but it will generate `OUT_OF_RANGE` if asked to read from an offset past the current file size. There is a fair bit of overlap between `FAILED_PRECONDITION` and `OUT_OF_RANGE`. We recommend using `OUT_OF_RANGE` (the more specific error) when it applies so that callers who are iterating through a space can easily look for an `OUT_OF_RANGE` error to detect when they are done. HTTP Mapping: 400 Bad Request"]
    OutOfRange,
    #[serde(rename = "UNIMPLEMENTED")]
    #[doc = "The operation is not implemented or is not supported/enabled in this service. HTTP Mapping: 501 Not Implemented"]
    Unimplemented,
    #[serde(rename = "INTERNAL")]
    #[doc = "Internal errors. This means that some invariants expected by the underlying system have been broken. This error code is reserved for serious errors. HTTP Mapping: 500 Internal Server Error"]
    Internal,
    #[serde(rename = "UNAVAILABLE")]
    #[doc = "The service is currently unavailable. This is most likely a transient condition, which can be corrected by retrying with a backoff. Note that it is not always safe to retry non-idempotent operations. See the guidelines above for deciding between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`. HTTP Mapping: 503 Service Unavailable"]
    Unavailable,
    #[serde(rename = "DATA_LOSS")]
    #[doc = "Unrecoverable data loss or corruption. HTTP Mapping: 500 Internal Server Error"]
    DataLoss,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for Operations.ListOperations."]
pub struct ListOperationsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The standard List next-page token."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of operations that matches the specified filter in the request."]
    pub operations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Operation>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This resource represents a long-running operation that is the result of a network API call."]
pub struct Operation {
    #[serde(rename = "done")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
    pub done: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The error result of the operation in case of failure or cancellation."]
    pub error: ::std::option::Option<::std::boxed::Box<Status>>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An OperationMetadata or Metadata object. This will always be returned with the Operation."]
    pub metadata: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. For example: `operations/CJHU7Oi_ChDrveSpBRjfuL-qzoWAgEw`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "response")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An Empty object."]
    pub response: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event that occurred during an Operation."]
pub struct OperationEvent {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required description of event."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional time of when event finished. An event can have a start time and no finish time. If an event has a finish time, there must be a start time."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional time of when event started."]
    pub start_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata describing an Operation."]
pub struct OperationMetadata {
    #[serde(rename = "clientId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field is deprecated. Use `labels` instead. Optionally provided by the caller when submitting the request that creates the operation."]
    pub client_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the job was submitted to the Genomics service."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the job stopped running."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional event messages that were generated during the job's execution. This also contains any warnings that were generated during import or export."]
    pub events: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OperationEvent>>>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optionally provided by the caller when submitting the request that creates the operation."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Google Cloud Project in which the job is scoped."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "request")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The original request that started the operation. Note that this will be in current version of the API. If the operation was started with v1beta2 API and a GetOperation is performed on v1 API, a v1 request will be returned."]
    pub request: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "runtimeMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Runtime metadata on this Operation."]
    pub runtime_metadata:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the job began to run."]
    pub start_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event generated when the worker starts pulling an image."]
pub struct PullStartedEvent {
    #[serde(rename = "imageUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI of the image that was pulled."]
    pub image_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event generated when the worker stops pulling an image."]
pub struct PullStoppedEvent {
    #[serde(rename = "imageUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URI of the image that was pulled."]
    pub image_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response to the RunPipeline method, returned in the operation's result field on success."]
pub struct RunPipelineResponse {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Runtime metadata that will be populated in the runtimeMetadata field of the Operation associated with a RunPipeline execution."]
pub struct RuntimeMetadata {
    #[serde(rename = "computeEngine")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Execution information specific to Google Compute Engine."]
    pub compute_engine: ::std::option::Option<::std::boxed::Box<ComputeEngine>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
pub struct Status {
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
#[doc = "An event generated when the execution of a container results in a non-zero exit status that was not otherwise ignored. Execution will continue, but only actions that are flagged as `ALWAYS_RUN` will be executed. Other actions will be skipped."]
pub struct UnexpectedExitStatusEvent {
    #[serde(rename = "actionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The numeric ID of the action that started the container."]
    pub action_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "exitStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The exit status of the container."]
    pub exit_status: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event generated after a worker VM has been assigned to run the pipeline."]
pub struct WorkerAssignedEvent {
    #[serde(rename = "instance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The worker's instance name."]
    pub instance: ::std::option::Option<::std::string::String>,
    #[serde(rename = "machineType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The machine type that was assigned for the worker."]
    pub machine_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The zone the worker is running in."]
    pub zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An event generated when the worker VM that was assigned to the pipeline has been released (deleted)."]
pub struct WorkerReleasedEvent {
    #[serde(rename = "instance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The worker's instance name."]
    pub instance: ::std::option::Option<::std::string::String>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The zone the worker was running in."]
    pub zone: ::std::option::Option<::std::string::String>,
}
