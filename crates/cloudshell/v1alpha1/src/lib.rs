#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for AuthorizeEnvironment."]
pub struct AuthorizeEnvironmentRequest {
    #[serde(rename = "accessToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The OAuth access token that should be sent to the environment."]
    pub access_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expireTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time when the credentials expire. If not set, defaults to one hour from when the server received the request."]
    pub expire_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "idToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The OAuth ID token that should be sent to the environment."]
    pub id_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for CreatePublicKey."]
pub struct CreatePublicKeyRequest {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Key that should be added to the environment."]
    pub key: ::std::option::Option<::std::boxed::Box<PublicKey>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Cloud Shell environment, which is defined as the combination of a Docker image specifying what is installed on the environment and a home directory containing the user's data that will remain across sessions. Each user has a single environment with the ID \"default\"."]
pub struct Environment {
    #[serde(rename = "dockerImage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Full path to the Docker image used to run this environment, e.g. \"gcr.io/dev-con/cloud-devshell:latest\"."]
    pub docker_image: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The environment's identifier, unique among the user's environments."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Full name of this resource, in the format `users/{owner_email}/environments/{environment_id}`. `{owner_email}` is the email address of the user to whom this environment belongs, and `{environment_id}` is the identifier of this environment. For example, `users/someone@example.com/environments/default`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publicKeys")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Public keys associated with the environment. Clients can connect to this environment via SSH only if they possess a private key corresponding to at least one of these public keys. Keys can be added to or removed from the environment using the CreatePublicKey and DeletePublicKey methods."]
    pub public_keys: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PublicKey>>>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the size of the backing VM running the environment. If set to something other than DEFAULT, it will be reverted to the default VM size after vm_size_expire_time."]
    pub size: ::std::option::Option<EnvironmentSizeEnum>,
    #[serde(rename = "sshHost")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Host to which clients can connect to initiate SSH sessions with the environment."]
    pub ssh_host: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sshPort")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Port to which clients can connect to initiate SSH sessions with the environment."]
    pub ssh_port: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "sshUsername")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Username that clients should use when initiating SSH sessions with the environment."]
    pub ssh_username: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Current execution state of this environment."]
    pub state: ::std::option::Option<EnvironmentStateEnum>,
    #[serde(rename = "vmSizeExpireTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time when the Environment will expire back to the default VM size."]
    pub vm_size_expire_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "webHost")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Host to which clients can connect to initiate HTTPS or WSS connections with the environment."]
    pub web_host: ::std::option::Option<::std::string::String>,
    #[serde(rename = "webPorts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Ports to which clients can connect to initiate HTTPS or WSS connections with the environment."]
    pub web_ports: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Indicates the size of the backing VM running the environment. If set to something other than DEFAULT, it will be reverted to the default VM size after vm_size_expire_time."]
pub enum EnvironmentSizeEnum {
    #[serde(rename = "VM_SIZE_UNSPECIFIED")]
    #[doc = "The VM size is unknown."]
    VmSizeUnspecified,
    #[serde(rename = "DEFAULT")]
    #[doc = "The default VM size."]
    Default,
    #[serde(rename = "BOOSTED")]
    #[doc = "The boosted VM size."]
    Boosted,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. Current execution state of this environment."]
pub enum EnvironmentStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "The environment's states is unknown."]
    StateUnspecified,
    #[serde(rename = "DISABLED")]
    #[doc = "The environment is not running and can't be connected to. Starting the environment will transition it to the STARTING state."]
    Disabled,
    #[serde(rename = "STARTING")]
    #[doc = "The environment is being started but is not yet ready to accept connections."]
    Starting,
    #[serde(rename = "RUNNING")]
    #[doc = "The environment is running and ready to accept connections. It will automatically transition back to DISABLED after a period of inactivity or if another environment is started."]
    Running,
    #[serde(rename = "DELETING")]
    #[doc = "The environment is being deleted and can't be connected to."]
    Deleting,
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
#[doc = "A public SSH key, corresponding to a private SSH key held by the client."]
pub struct PublicKey {
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Format of this key's content."]
    pub format: ::std::option::Option<PublicKeyFormatEnum>,
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Content of this key."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Full name of this resource, in the format `users/{owner_email}/environments/{environment_id}/publicKeys/{key_id}`. `{owner_email}` is the email address of the user to whom the key belongs. `{environment_id}` is the identifier of the environment to which the key grants access. `{key_id}` is the unique identifier of the key. For example, `users/someone@example.com/environments/default/publicKeys/myKey`."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Format of this key's content."]
pub enum PublicKeyFormatEnum {
    #[serde(rename = "FORMAT_UNSPECIFIED")]
    #[doc = "Unknown format. Do not use."]
    FormatUnspecified,
    #[serde(rename = "SSH_DSS")]
    #[doc = "`ssh-dss` key format (see RFC4253)."]
    SshDss,
    #[serde(rename = "SSH_RSA")]
    #[doc = "`ssh-rsa` key format (see RFC4253)."]
    SshRsa,
    #[serde(rename = "ECDSA_SHA2_NISTP256")]
    #[doc = "`ecdsa-sha2-nistp256` key format (see RFC5656)."]
    EcdsaSha2Nistp256,
    #[serde(rename = "ECDSA_SHA2_NISTP384")]
    #[doc = "`ecdsa-sha2-nistp384` key format (see RFC5656)."]
    EcdsaSha2Nistp384,
    #[serde(rename = "ECDSA_SHA2_NISTP521")]
    #[doc = "`ecdsa-sha2-nistp521` key format (see RFC5656)."]
    EcdsaSha2Nistp521,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message included in the metadata field of operations returned from StartEnvironment."]
pub struct StartEnvironmentMetadata {
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Current state of the environment being started."]
    pub state: ::std::option::Option<StartEnvironmentMetadataStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Current state of the environment being started."]
pub enum StartEnvironmentMetadataStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "The environment's start state is unknown."]
    StateUnspecified,
    #[serde(rename = "STARTING")]
    #[doc = "The environment is in the process of being started, but no additional details are available."]
    Starting,
    #[serde(rename = "UNARCHIVING_DISK")]
    #[doc = "Startup is waiting for the user's disk to be unarchived. This can happen when the user returns to Cloud Shell after not having used it for a while, and suggests that startup will take longer than normal."]
    UnarchivingDisk,
    #[serde(rename = "AWAITING_VM")]
    #[doc = "Startup is waiting for a VM to be assigned to the environment. This should normally happen very quickly, but an environment might stay in this state for an extended period of time if the system is experiencing heavy load."]
    AwaitingVm,
    #[serde(rename = "AWAITING_COMPUTE_RESOURCES")]
    #[doc = "Startup is waiting for compute resources to be assigned to the environment. This should normally happen very quickly, but an environment might stay in this state for an extended period of time if the system is experiencing heavy load."]
    AwaitingComputeResources,
    #[serde(rename = "FINISHED")]
    #[doc = "Startup has completed. If the start operation was successful, the user should be able to establish an SSH connection to their environment. Otherwise, the operation will contain details of the failure."]
    Finished,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for StartEnvironment."]
pub struct StartEnvironmentRequest {
    #[serde(rename = "accessToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The initial access token passed to the environment. If this is present and valid, the environment will be pre-authenticated with gcloud so that the user can run gcloud commands in Cloud Shell without having to log in. This code can be updated later by calling AuthorizeEnvironment."]
    pub access_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publicKeys")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Public keys that should be added to the environment before it is started."]
    pub public_keys: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PublicKey>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message included in the response field of operations returned from StartEnvironment once the operation is complete."]
pub struct StartEnvironmentResponse {
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Environment that was started."]
    pub environment: ::std::option::Option<::std::boxed::Box<Environment>>,
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
