#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A accelerator type that a Node can be configured with."]
pub struct AcceleratorType {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "the accelerator type."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for ListAcceleratorTypes."]
pub struct ListAcceleratorTypesResponse {
    #[serde(rename = "acceleratorTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The listed nodes."]
    pub accelerator_types:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AcceleratorType>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The next page token or empty if none."]
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
#[doc = "Response for ListNodes."]
pub struct ListNodesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The next page token or empty if none."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The listed nodes."]
    pub nodes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Node>>>,
    #[serde(rename = "unreachable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Locations that could not be reached."]
    pub unreachable: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
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
#[doc = "Response for ListTensorFlowVersions."]
pub struct ListTensorFlowVersionsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The next page token or empty if none."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tensorflowVersions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The listed nodes."]
    pub tensorflow_versions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TensorFlowVersion>>>,
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
#[doc = "A network endpoint over which a TPU worker can be reached."]
pub struct NetworkEndpoint {
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IP address of this network endpoint."]
    pub ip_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The port of this network endpoint."]
    pub port: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A TPU instance."]
pub struct Node {
    #[serde(rename = "acceleratorType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The type of hardware accelerators associated with this node."]
    pub accelerator_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cidrBlock")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The CIDR block that the TPU node will use when selecting an IP address. This CIDR block must be a /29 block; the Compute Engine networks API forbids a smaller block, and using a larger block would be wasteful (a node can only consume one IP address). Errors will occur if the CIDR block has already been used for a currently existing TPU node, the CIDR block conflicts with any subnetworks in the user's provided network, or the provided network is peered with another network that is using that CIDR block."]
    pub cidr_block: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time when the node was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user-supplied description of the TPU. Maximum of 512 characters."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "health")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The health status of the TPU node."]
    pub health: ::std::option::Option<NodeHealthEnum>,
    #[serde(rename = "healthDescription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. If this field is populated, it contains a description of why the TPU Node is unhealthy."]
    pub health_description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. DEPRECATED! Use network_endpoints instead. The network address for the TPU Node as visible to Compute Engine instances."]
    pub ip_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource labels to represent user-provided metadata."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Immutable. The name of the TPU"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "network")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of a network they wish to peer the TPU node to. It must be a preexisting Compute Engine network inside of the project on which this API has been activated. If none is provided, \"default\" will be used."]
    pub network: ::std::option::Option<::std::string::String>,
    #[serde(rename = "networkEndpoints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The network endpoints where TPU workers can be accessed and sent work. It is recommended that Tensorflow clients of the node reach out to the 0th entry in this map first."]
    pub network_endpoints:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NetworkEndpoint>>>,
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. DEPRECATED! Use network_endpoints instead. The network port for the TPU Node as visible to Compute Engine instances."]
    pub port: ::std::option::Option<::std::string::String>,
    #[serde(rename = "schedulingConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The scheduling options for this node."]
    pub scheduling_config: ::std::option::Option<::std::boxed::Box<SchedulingConfig>>,
    #[serde(rename = "serviceAccount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The service account used to run the tensor flow services within the node. To share resources, including Google Cloud Storage data, with the Tensorflow job running in the Node, this account must have permissions to that data."]
    pub service_account: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The current state for the TPU Node."]
    pub state: ::std::option::Option<NodeStateEnum>,
    #[serde(rename = "symptoms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The Symptoms that have occurred to the TPU Node."]
    pub symptoms: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Symptom>>>,
    #[serde(rename = "tensorflowVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The version of Tensorflow running in the Node."]
    pub tensorflow_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "useServiceNetworking")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the VPC peering for the node is set up through Service Networking API. The VPC Peering should be set up before provisioning the node. If this field is set, cidr_block field should not be specified. If the network, that you want to peer the TPU Node to, is Shared VPC networks, the node must be created with this this field enabled."]
    pub use_service_networking: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The health status of the TPU node."]
pub enum NodeHealthEnum {
    #[serde(rename = "HEALTH_UNSPECIFIED")]
    #[doc = "Health status is unknown: not initialized or failed to retrieve."]
    HealthUnspecified,
    #[serde(rename = "HEALTHY")]
    #[doc = "The resource is healthy."]
    Healthy,
    #[serde(rename = "DEPRECATED_UNHEALTHY")]
    #[doc = "The resource is unhealthy."]
    DeprecatedUnhealthy,
    #[serde(rename = "TIMEOUT")]
    #[doc = "The resource is unresponsive."]
    Timeout,
    #[serde(rename = "UNHEALTHY_TENSORFLOW")]
    #[doc = "The in-guest ML stack is unhealthy."]
    UnhealthyTensorflow,
    #[serde(rename = "UNHEALTHY_MAINTENANCE")]
    #[doc = "The node is under maintenance/priority boost caused rescheduling and will resume running once rescheduled."]
    UnhealthyMaintenance,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The current state for the TPU Node."]
pub enum NodeStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "TPU node state is not known/set."]
    StateUnspecified,
    #[serde(rename = "CREATING")]
    #[doc = "TPU node is being created."]
    Creating,
    #[serde(rename = "READY")]
    #[doc = "TPU node has been created and is fully usable."]
    Ready,
    #[serde(rename = "RESTARTING")]
    #[doc = "TPU node is restarting."]
    Restarting,
    #[serde(rename = "REIMAGING")]
    #[doc = "TPU node is undergoing reimaging."]
    Reimaging,
    #[serde(rename = "DELETING")]
    #[doc = "TPU node is being deleted."]
    Deleting,
    #[serde(rename = "REPAIRING")]
    #[doc = "TPU node is being repaired and may be unusable. Details can be found in the `help_description` field."]
    Repairing,
    #[serde(rename = "STOPPED")]
    #[doc = "TPU node is stopped."]
    Stopped,
    #[serde(rename = "STOPPING")]
    #[doc = "TPU node is currently stopping."]
    Stopping,
    #[serde(rename = "STARTING")]
    #[doc = "TPU node is currently starting."]
    Starting,
    #[serde(rename = "PREEMPTED")]
    #[doc = "TPU node has been preempted. Only applies to Preemptible TPU Nodes."]
    Preempted,
    #[serde(rename = "TERMINATED")]
    #[doc = "TPU node has been terminated due to maintenance or has reached the end of its life cycle (for preemptible nodes)."]
    Terminated,
    #[serde(rename = "HIDING")]
    #[doc = "TPU node is currently hiding."]
    Hiding,
    #[serde(rename = "HIDDEN")]
    #[doc = "TPU node has been hidden."]
    Hidden,
    #[serde(rename = "UNHIDING")]
    #[doc = "TPU node is currently unhiding."]
    Unhiding,
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
#[doc = "Represents the metadata of the long-running operation."]
pub struct OperationMetadata {
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] API version used to start the operation."]
    pub api_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cancelRequested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] Identifies whether the user has requested cancellation of the operation. Operations that have successfully been cancelled have Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."]
    pub cancel_requested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] The time the operation was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] The time the operation finished running."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "statusDetail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] Human-readable status of the operation, if any."]
    pub status_detail: ::std::option::Option<::std::string::String>,
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] Server-defined resource path for the target of the operation."]
    pub target: ::std::option::Option<::std::string::String>,
    #[serde(rename = "verb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] Name of the verb executed by the operation."]
    pub verb: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for ReimageNode."]
pub struct ReimageNodeRequest {
    #[serde(rename = "tensorflowVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version for reimage to create."]
    pub tensorflow_version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Sets the scheduling options for this node."]
pub struct SchedulingConfig {
    #[serde(rename = "preemptible")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Defines whether the node is preemptible."]
    pub preemptible: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "reserved")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the node is created under a reservation."]
    pub reserved: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for StartNode."]
pub struct StartNodeRequest {}
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
#[doc = "Request for StopNode."]
pub struct StopNodeRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Symptom instance."]
pub struct Symptom {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp when the Symptom is created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detailed information of the current Symptom."]
    pub details: ::std::option::Option<::std::string::String>,
    #[serde(rename = "symptomType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the Symptom."]
    pub symptom_type: ::std::option::Option<SymptomSymptomTypeEnum>,
    #[serde(rename = "workerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A string used to uniquely distinguish a worker within a TPU node."]
    pub worker_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of the Symptom."]
pub enum SymptomSymptomTypeEnum {
    #[serde(rename = "SYMPTOM_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified symptom."]
    SymptomTypeUnspecified,
    #[serde(rename = "LOW_MEMORY")]
    #[doc = "TPU VM memory is low."]
    LowMemory,
    #[serde(rename = "OUT_OF_MEMORY")]
    #[doc = "TPU runtime is out of memory."]
    OutOfMemory,
    #[serde(rename = "EXECUTE_TIMED_OUT")]
    #[doc = "TPU runtime execution has timed out."]
    ExecuteTimedOut,
    #[serde(rename = "MESH_BUILD_FAIL")]
    #[doc = "TPU runtime fails to construct a mesh that recognizes each TPU device's neighbors."]
    MeshBuildFail,
    #[serde(rename = "HBM_OUT_OF_MEMORY")]
    #[doc = "TPU HBM is out of memory."]
    HbmOutOfMemory,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A tensorflow version that a Node can be configured with."]
pub struct TensorFlowVersion {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "the tensorflow version."]
    pub version: ::std::option::Option<::std::string::String>,
}
