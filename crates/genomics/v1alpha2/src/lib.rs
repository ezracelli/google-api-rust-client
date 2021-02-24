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
#[doc = "Stores the information that the controller will fetch from the server in order to run. Should only be used by VMs created by the Pipelines Service and not by end users."]
pub struct ControllerConfig {
    #[serde(rename = "cmd")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cmd: ::std::option::Option<::std::string::String>,
    #[serde(rename = "disks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub disks: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "gcsLogPath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub gcs_log_path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gcsSinks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub gcs_sinks: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<RepeatedString>>,
    >,
    #[serde(rename = "gcsSources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub gcs_sources: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<RepeatedString>>,
    >,
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub image: ::std::option::Option<::std::string::String>,
    #[serde(rename = "machineType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub machine_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "vars")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub vars: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
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
#[doc = "A Google Compute Engine disk resource specification."]
pub struct Disk {
    #[serde(rename = "autoDelete")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Disks created by the Pipelines API will be deleted at the end of the pipeline run, regardless of what this field is set to."]
    pub auto_delete: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "mountPoint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required at create time and cannot be overridden at run time. Specifies the path in the docker container where files on this disk should be located. For example, if `mountPoint` is `/mnt/disk`, and the parameter has `localPath` `inputs/file.txt`, the docker container can access the data at `/mnt/disk/inputs/file.txt`."]
    pub mount_point: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the disk that can be used in the pipeline parameters. Must be 1 - 63 characters. The name \"boot\" is reserved for system use."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies how a sourced-base persistent disk will be mounted. See https://cloud.google.com/compute/docs/disks/persistent-disks#use_multi_instances for more details. Can only be set at create time."]
    pub read_only: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "sizeGb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The size of the disk. Defaults to 500 (GB). This field is not applicable for local SSD."]
    pub size_gb: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full or partial URL of the persistent disk to attach. See https://cloud.google.com/compute/docs/reference/latest/instances#resource and https://cloud.google.com/compute/docs/disks/persistent-disks#snapshots for more details."]
    pub source: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The type of the disk to create."]
    pub _type: ::std::option::Option<DiskTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The type of the disk to create."]
pub enum DiskTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "Default disk type. Use one of the other options below."]
    TypeUnspecified,
    #[serde(rename = "PERSISTENT_HDD")]
    #[doc = "Specifies a Google Compute Engine persistent hard disk. See https://cloud.google.com/compute/docs/disks/#pdspecs for details."]
    PersistentHdd,
    #[serde(rename = "PERSISTENT_SSD")]
    #[doc = "Specifies a Google Compute Engine persistent solid-state disk. See https://cloud.google.com/compute/docs/disks/#pdspecs for details."]
    PersistentSsd,
    #[serde(rename = "LOCAL_SSD")]
    #[doc = "Specifies a Google Compute Engine local SSD. See https://cloud.google.com/compute/docs/disks/local-ssd for details."]
    LocalSsd,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The Docker execuctor specification."]
pub struct DockerExecutor {
    #[serde(rename = "cmd")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The command or newline delimited script to run. The command string will be executed within a bash shell. If the command exits with a non-zero exit code, output parameter de-localization will be skipped and the pipeline operation's `error` field will be populated. Maximum command string length is 16384."]
    pub cmd: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imageName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Image name from either Docker Hub or Google Container Registry. Users that run pipelines must have READ access to the image."]
    pub image_name: ::std::option::Option<::std::string::String>,
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
#[doc = "The response of ListPipelines. Contains at most `pageSize` pipelines. If it contains `pageSize` pipelines, and more pipelines exist, then `nextPageToken` will be populated and should be used as the `pageToken` argument to a subsequent ListPipelines request."]
pub struct ListPipelinesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token to use to get the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pipelines")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The matched pipelines."]
    pub pipelines: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Pipeline>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "LocalCopy defines how a remote file should be copied to and from the VM."]
pub struct LocalCopy {
    #[serde(rename = "disk")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the disk where this parameter is located. Can be the name of one of the disks specified in the Resources field, or \"boot\", which represents the Docker instance's boot disk and has a mount point of `/`."]
    pub disk: ::std::option::Option<::std::string::String>,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The path within the user's docker container where this input should be localized to and from, relative to the specified disk's mount point. For example: file.txt,"]
    pub path: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The logging options for the pipeline run."]
pub struct LoggingOptions {
    #[serde(rename = "gcsPath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The location in Google Cloud Storage to which the pipeline logs will be copied. Can be specified as a fully qualified directory path, in which case logs will be output with a unique identifier as the filename in that directory, or as a fully specified path, which must end in `.log`, in which case that path will be used, and the user must ensure that logs are not overwritten. Stdout and stderr logs from the run are also generated and output as `-stdout.log` and `-stderr.log`."]
    pub gcs_path: ::std::option::Option<::std::string::String>,
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
#[doc = "The pipeline object. Represents a transformation from a set of input parameters to a set of output parameters. The transformation is defined as a docker image and command to run within that image. Each pipeline is run on a Google Compute Engine VM. A pipeline can be created with the `create` method and then later run with the `run` method, or a pipeline can be defined and run all at once with the `run` method."]
pub struct Pipeline {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User-specified description."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "docker")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the docker run information."]
    pub docker: ::std::option::Option<::std::boxed::Box<DockerExecutor>>,
    #[serde(rename = "inputParameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Input parameters of the pipeline."]
    pub input_parameters:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PipelineParameter>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A user specified pipeline name that does not have to be unique. This name can be used for filtering Pipelines in ListPipelines."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "outputParameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output parameters of the pipeline."]
    pub output_parameters:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PipelineParameter>>>,
    #[serde(rename = "pipelineId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique pipeline id that is generated by the service when CreatePipeline is called. Cannot be specified in the Pipeline used in the CreatePipelineRequest, and will be populated in the response to CreatePipeline and all subsequent Get and List calls. Indicates that the service has registered this pipeline."]
    pub pipeline_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The project in which to create the pipeline. The caller must have WRITE access."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Specifies resource requirements for the pipeline run. Required fields: * minimumCpuCores * minimumRamGb"]
    pub resources: ::std::option::Option<::std::boxed::Box<PipelineResources>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Parameters facilitate setting and delivering data into the pipeline's execution environment. They are defined at create time, with optional defaults, and can be overridden at run time. If `localCopy` is unset, then the parameter specifies a string that is passed as-is into the pipeline, as the value of the environment variable with the given name. A default value can be optionally specified at create time. The default can be overridden at run time using the inputs map. If no default is given, a value must be supplied at runtime. If `localCopy` is defined, then the parameter specifies a data source or sink, both in Google Cloud Storage and on the Docker container where the pipeline computation is run. The service account associated with the Pipeline (by default the project's Compute Engine service account) must have access to the Google Cloud Storage paths. At run time, the Google Cloud Storage paths can be overridden if a default was provided at create time, or must be set otherwise. The pipeline runner should add a key/value pair to either the inputs or outputs map. The indicated data copies will be carried out before/after pipeline execution, just as if the corresponding arguments were provided to `gsutil cp`. For example: Given the following `PipelineParameter`, specified in the `inputParameters` list: ``` {name: \"input_file\", localCopy: {path: \"file.txt\", disk: \"pd1\"}} ``` where `disk` is defined in the `PipelineResources` object as: ``` {name: \"pd1\", mountPoint: \"/mnt/disk/\"} ``` We create a disk named `pd1`, mount it on the host VM, and map `/mnt/pd1` to `/mnt/disk` in the docker container. At runtime, an entry for `input_file` would be required in the inputs map, such as: ``` inputs[\"input_file\"] = \"gs://my-bucket/bar.txt\" ``` This would generate the following gsutil call: ``` gsutil cp gs://my-bucket/bar.txt /mnt/pd1/file.txt ``` The file `/mnt/pd1/file.txt` maps to `/mnt/disk/file.txt` in the Docker container. Acceptable paths are: Google Cloud storage pathLocal path file file glob directory For outputs, the direction of the copy is reversed: ``` gsutil cp /mnt/disk/file.txt gs://my-bucket/bar.txt ``` Acceptable paths are: Local pathGoogle Cloud Storage path file file file directory - directory must already exist glob directory - directory will be created if it doesn't exist One restriction due to docker limitations, is that for outputs that are found on the boot disk, the local path cannot be a glob and must be a file."]
pub struct PipelineParameter {
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The default value for this parameter. Can be overridden at runtime. If `localCopy` is present, then this must be a Google Cloud Storage path beginning with `gs://`."]
    pub default_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Human-readable description."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "localCopy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If present, this parameter is marked for copying to and from the VM. `LocalCopy` indicates where on the VM the file should be. The value given to this parameter (either at runtime or using `defaultValue`) must be the remote path where the file should be."]
    pub local_copy: ::std::option::Option<::std::boxed::Box<LocalCopy>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Name of the parameter - the pipeline runner uses this string as the key to the input and output maps in RunPipeline."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The system resources for the pipeline run."]
pub struct PipelineResources {
    #[serde(rename = "acceleratorCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The number of accelerators of the specified type to attach. By specifying this parameter, you will download and install the following third-party software onto your managed Compute Engine instances: NVIDIA® Tesla® drivers and NVIDIA® CUDA toolkit."]
    pub accelerator_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "acceleratorType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The Compute Engine defined accelerator type. By specifying this parameter, you will download and install the following third-party software onto your managed Compute Engine instances: NVIDIA® Tesla® drivers and NVIDIA® CUDA toolkit. Please see https://cloud.google.com/compute/docs/gpus/ for a list of available accelerator types."]
    pub accelerator_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bootDiskSizeGb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The size of the boot disk. Defaults to 10 (GB)."]
    pub boot_disk_size_gb: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "disks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Disks to attach."]
    pub disks: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Disk>>>,
    #[serde(rename = "minimumCpuCores")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minimum number of cores to use. Defaults to 1."]
    pub minimum_cpu_cores: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "minimumRamGb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minimum amount of RAM to use. Defaults to 3.75 (GB)"]
    pub minimum_ram_gb: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "noAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to assign an external IP to the instance. This is an experimental feature that may go away. Defaults to false. Corresponds to `--no_address` flag for [gcloud compute instances create] (https://cloud.google.com/sdk/gcloud/reference/compute/instances/create). In order to use this, must be true for both create time and run time. Cannot be true at run time if false at create time. If you need to ssh into a private IP VM for debugging, you can ssh to a public VM and then ssh into the private VM's Internal IP. If noAddress is set, this pipeline run may only load docker images from Google Container Registry and not Docker Hub. Before using this, you must [configure access to Google services from internal IPs](https://cloud.google.com/compute/docs/configure-private-google-access#configuring_access_to_google_services_from_internal_ips)."]
    pub no_address: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "preemptible")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to use preemptible VMs. Defaults to `false`. In order to use this, must be true for both create time and run time. Cannot be true at run time if false at create time."]
    pub preemptible: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "zones")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of Google Compute Engine availability zones to which resource creation will restricted. If empty, any zone may be chosen."]
    pub zones: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
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
pub struct RepeatedString {
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The pipeline run arguments."]
pub struct RunPipelineArgs {
    #[serde(rename = "clientId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field is deprecated. Use `labels` instead. Client-specified pipeline operation identifier."]
    pub client_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inputs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pipeline input arguments; keys are defined in the pipeline documentation. All input parameters that do not have default values must be specified. If parameters with defaults are specified here, the defaults will be overridden."]
    pub inputs: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "keepVmAliveOnFailureDuration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "How long to keep the VM up after a failure (for example docker command failed, copying input or output files failed, etc). While the VM is up, one can ssh into the VM to debug. Default is 0; maximum allowed value is 1 day."]
    pub keep_vm_alive_on_failure_duration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Labels to apply to this pipeline run. Labels will also be applied to compute resources (VM, disks) created by this pipeline run. When listing operations, operations can filtered by labels. Label keys may not be empty; label values may be empty. Non-empty labels must be 1-63 characters long, and comply with [RFC1035] (https://www.ietf.org/rfc/rfc1035.txt). Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "logging")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Logging options. Used by the service to communicate results to the user."]
    pub logging: ::std::option::Option<::std::boxed::Box<LoggingOptions>>,
    #[serde(rename = "outputs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pipeline output arguments; keys are defined in the pipeline documentation. All output parameters of without default values must be specified. If parameters with defaults are specified here, the defaults will be overridden."]
    pub outputs: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The project in which to run the pipeline. The caller must have WRITER access to all Google Cloud services and resources (e.g. Google Compute Engine) will be used."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies resource requirements/overrides for the pipeline run."]
    pub resources: ::std::option::Option<::std::boxed::Box<PipelineResources>>,
    #[serde(rename = "serviceAccount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Google Cloud Service Account that will be used to access data and services. By default, the compute service account associated with `projectId` is used."]
    pub service_account: ::std::option::Option<::std::boxed::Box<ServiceAccount>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request to run a pipeline. If `pipelineId` is specified, it refers to a saved pipeline created with CreatePipeline and set as the `pipelineId` of the returned Pipeline object. If `ephemeralPipeline` is specified, that pipeline is run once with the given args and not saved. It is an error to specify both `pipelineId` and `ephemeralPipeline`. `pipelineArgs` must be specified."]
pub struct RunPipelineRequest {
    #[serde(rename = "ephemeralPipeline")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A new pipeline object to run once and then delete."]
    pub ephemeral_pipeline: ::std::option::Option<::std::boxed::Box<Pipeline>>,
    #[serde(rename = "pipelineArgs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The arguments to use when running this pipeline."]
    pub pipeline_args: ::std::option::Option<::std::boxed::Box<RunPipelineArgs>>,
    #[serde(rename = "pipelineId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The already created pipeline to run."]
    pub pipeline_id: ::std::option::Option<::std::string::String>,
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
#[doc = "A Google Cloud Service Account."]
pub struct ServiceAccount {
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Email address of the service account. Defaults to `default`, which uses the compute service account associated with the project."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scopes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of scopes to be enabled for this service account on the VM. The following scopes are automatically included: * https://www.googleapis.com/auth/compute * https://www.googleapis.com/auth/devstorage.full_control * https://www.googleapis.com/auth/genomics * https://www.googleapis.com/auth/logging.write * https://www.googleapis.com/auth/monitoring.write"]
    pub scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to set operation status. Should only be used by VMs created by the Pipelines Service and not by end users."]
pub struct SetOperationStatusRequest {
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub error_code: ::std::option::Option<SetOperationStatusRequestErrorCodeEnum>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub error_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub operation_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timestampEvents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub timestamp_events: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TimestampEvent>>>,
    #[serde(rename = "validationToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub validation_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum SetOperationStatusRequestErrorCodeEnum {
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
#[doc = "Stores the list of events and times they occured for major events in job execution."]
pub struct TimestampEvent {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "String indicating the type of event"]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time this event occured."]
    pub timestamp: ::std::option::Option<::std::string::String>,
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
