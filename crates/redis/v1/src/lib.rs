#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for Export."]
pub struct ExportInstanceRequest {
    #[serde(rename = "outputConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Specify data to be exported."]
    pub output_config: ::std::option::Option<::std::boxed::Box<OutputConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for Failover."]
pub struct FailoverInstanceRequest {
    #[serde(rename = "dataProtectionMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Available data protection modes that the user can choose. If it's unspecified, data protection mode will be LIMITED_DATA_LOSS by default."]
    pub data_protection_mode: ::std::option::Option<FailoverInstanceRequestDataProtectionModeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. Available data protection modes that the user can choose. If it's unspecified, data protection mode will be LIMITED_DATA_LOSS by default."]
pub enum FailoverInstanceRequestDataProtectionModeEnum {
    #[serde(rename = "DATA_PROTECTION_MODE_UNSPECIFIED")]
    #[doc = "Defaults to LIMITED_DATA_LOSS if a data protection mode is not specified."]
    DataProtectionModeUnspecified,
    #[serde(rename = "LIMITED_DATA_LOSS")]
    #[doc = "Instance failover will be protected with data loss control. More specifically, the failover will only be performed if the current replication offset diff between primary and replica is under a certain threshold."]
    LimitedDataLoss,
    #[serde(rename = "FORCE_DATA_LOSS")]
    #[doc = "Instance failover will be performed without data loss control."]
    ForceDataLoss,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The Cloud Storage location for the output content"]
pub struct GcsDestination {
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Data destination URI (e.g. 'gs://my_bucket/my_object'). Existing files will be overwritten."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The Cloud Storage location for the input content"]
pub struct GcsSource {
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Source data URI. (e.g. 'gs://my_bucket/my_object')."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This location metadata represents additional configuration options for a given location where a Redis instance may be created. All fields are output only. It is returned as content of the `google.cloud.location.Location.metadata` field."]
pub struct GoogleCloudRedisV1LocationMetadata {
    #[serde(rename = "availableZones")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The set of available zones in the location. The map is keyed by the lowercase ID of each zone, as defined by GCE. These keys can be specified in `location_id` or `alternative_location_id` fields when creating a Redis instance."]
    pub available_zones: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<GoogleCloudRedisV1ZoneMetadata>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the v1 metadata of the long-running operation."]
pub struct GoogleCloudRedisV1OperationMetadata {
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "API version."]
    pub api_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cancelRequested")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies if cancellation was requested for the operation."]
    pub cancel_requested: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creation timestamp."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "End timestamp."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "statusDetail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Operation status details."]
    pub status_detail: ::std::option::Option<::std::string::String>,
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Operation target."]
    pub target: ::std::option::Option<::std::string::String>,
    #[serde(rename = "verb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Operation verb."]
    pub verb: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines specific information for a particular zone. Currently empty and reserved for future use only."]
pub struct GoogleCloudRedisV1ZoneMetadata {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for Import."]
pub struct ImportInstanceRequest {
    #[serde(rename = "inputConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Specify data to be imported."]
    pub input_config: ::std::option::Option<::std::boxed::Box<InputConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The input content"]
pub struct InputConfig {
    #[serde(rename = "gcsSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Google Cloud Storage location where input content is located."]
    pub gcs_source: ::std::option::Option<::std::boxed::Box<GcsSource>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Google Cloud Redis instance."]
pub struct Instance {
    #[serde(rename = "alternativeLocationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Only applicable to STANDARD_HA tier which protects the instance against zonal failures by provisioning it across two zones. If provided, it must be a different zone from the one provided in location_id."]
    pub alternative_location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "authEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates whether OSS Redis AUTH is enabled for the instance. If set to \"true\" AUTH is enabled on the instance. Default value is \"false\" meaning AUTH is disabled."]
    pub auth_enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "authorizedNetwork")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The full name of the Google Compute Engine [network](https://cloud.google.com/vpc/docs/vpc) to which the instance is connected. If left unspecified, the `default` network will be used."]
    pub authorized_network: ::std::option::Option<::std::string::String>,
    #[serde(rename = "connectMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The network connect mode of the Redis instance. If not provided, the connect mode defaults to DIRECT_PEERING."]
    pub connect_mode: ::std::option::Option<InstanceConnectModeEnum>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time the instance was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "currentLocationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The current zone where the Redis endpoint is placed. For Basic Tier instances, this will always be the same as the location_id provided by the user at creation time. For Standard Tier instances, this can be either location_id or alternative_location_id and can change after a failover event."]
    pub current_location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An arbitrary and optional user-provided name for the instance."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "host")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Hostname or IP address of the exposed Redis endpoint used by clients to connect to the service."]
    pub host: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource labels to represent user provided metadata"]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The zone where the instance will be provisioned. If not provided, the service will choose a zone for the instance. For STANDARD_HA tier, instances will be created across two zones for protection against zonal failures. If alternative_location_id is also provided, it must be different from location_id."]
    pub location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "memorySizeGb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Redis memory size in GiB."]
    pub memory_size_gb: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Unique name of the resource in this scope including project and location using the form: `projects/{project_id}/locations/{location_id}/instances/{instance_id}` Note: Redis instances are managed and addressed at regional level so location_id here refers to a GCP region; however, users may choose which specific zone (or collection of zones for cross-zone instances) an instance should be provisioned in. Refer to location_id and alternative_location_id fields for more details."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "persistenceIamIdentity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Cloud IAM identity used by import / export operations to transfer data to/from Cloud Storage. Format is \"serviceAccount:\". The value may change over time for a given instance so should be checked before each import/export operation."]
    pub persistence_iam_identity: ::std::option::Option<::std::string::String>,
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The port number of the exposed Redis endpoint."]
    pub port: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "redisConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Redis configuration parameters, according to http://redis.io/topics/config. Currently, the only supported parameters are: Redis version 3.2 and newer: * maxmemory-policy * notify-keyspace-events Redis version 4.0 and newer: * activedefrag * lfu-decay-time * lfu-log-factor * maxmemory-gb Redis version 5.0 and newer: * stream-node-max-bytes * stream-node-max-entries"]
    pub redis_configs:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "redisVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The version of Redis software. If not provided, latest supported version will be used. Currently, the supported values are: * `REDIS_3_2` for Redis 3.2 compatibility * `REDIS_4_0` for Redis 4.0 compatibility (default) * `REDIS_5_0` for Redis 5.0 compatibility"]
    pub redis_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reservedIpRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The CIDR range of internal addresses that are reserved for this instance. If not provided, the service will choose an unused /29 block, for example, 10.0.0.0/29 or 192.168.0.0/29. Ranges must be unique and non-overlapping with existing subnets in an authorized network."]
    pub reserved_ip_range: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serverCaCerts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. List of server CA certificates for the instance."]
    pub server_ca_certs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TlsCertificate>>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The current state of this instance."]
    pub state: ::std::option::Option<InstanceStateEnum>,
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Additional information about the current status of this instance, if available."]
    pub status_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The service tier of the instance."]
    pub tier: ::std::option::Option<InstanceTierEnum>,
    #[serde(rename = "transitEncryptionMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The TLS mode of the Redis instance. If not provided, TLS is disabled for the instance."]
    pub transit_encryption_mode: ::std::option::Option<InstanceTransitEncryptionModeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. The network connect mode of the Redis instance. If not provided, the connect mode defaults to DIRECT_PEERING."]
pub enum InstanceConnectModeEnum {
    #[serde(rename = "CONNECT_MODE_UNSPECIFIED")]
    #[doc = "Not set."]
    ConnectModeUnspecified,
    #[serde(rename = "DIRECT_PEERING")]
    #[doc = "Connect via direct peering to the Memorystore for Redis hosted service."]
    DirectPeering,
    #[serde(rename = "PRIVATE_SERVICE_ACCESS")]
    #[doc = "Connect your Memorystore for Redis instance using Private Service Access. Private services access provides an IP address range for multiple Google Cloud services, including Memorystore."]
    PrivateServiceAccess,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The current state of this instance."]
pub enum InstanceStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Not set."]
    StateUnspecified,
    #[serde(rename = "CREATING")]
    #[doc = "Redis instance is being created."]
    Creating,
    #[serde(rename = "READY")]
    #[doc = "Redis instance has been created and is fully usable."]
    Ready,
    #[serde(rename = "UPDATING")]
    #[doc = "Redis instance configuration is being updated. Certain kinds of updates may cause the instance to become unusable while the update is in progress."]
    Updating,
    #[serde(rename = "DELETING")]
    #[doc = "Redis instance is being deleted."]
    Deleting,
    #[serde(rename = "REPAIRING")]
    #[doc = "Redis instance is being repaired and may be unusable."]
    Repairing,
    #[serde(rename = "MAINTENANCE")]
    #[doc = "Maintenance is being performed on this Redis instance."]
    Maintenance,
    #[serde(rename = "IMPORTING")]
    #[doc = "Redis instance is importing data (availability may be affected)."]
    Importing,
    #[serde(rename = "FAILING_OVER")]
    #[doc = "Redis instance is failing over (availability may be affected)."]
    FailingOver,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The service tier of the instance."]
pub enum InstanceTierEnum {
    #[serde(rename = "TIER_UNSPECIFIED")]
    #[doc = "Not set."]
    TierUnspecified,
    #[serde(rename = "BASIC")]
    #[doc = "BASIC tier: standalone instance"]
    Basic,
    #[serde(rename = "STANDARD_HA")]
    #[doc = "STANDARD_HA tier: highly available primary/replica instances"]
    StandardHa,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. The TLS mode of the Redis instance. If not provided, TLS is disabled for the instance."]
pub enum InstanceTransitEncryptionModeEnum {
    #[serde(rename = "TRANSIT_ENCRYPTION_MODE_UNSPECIFIED")]
    #[doc = "Not set."]
    TransitEncryptionModeUnspecified,
    #[serde(rename = "SERVER_AUTHENTICATION")]
    #[doc = "Client to Server traffic encryption enabled with server authentication."]
    ServerAuthentication,
    #[serde(rename = "DISABLED")]
    #[doc = "TLS is disabled for the instance."]
    Disabled,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Instance AUTH string details."]
pub struct InstanceAuthString {
    #[serde(rename = "authString")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "AUTH string set on the instance."]
    pub auth_string: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for ListInstances."]
pub struct ListInstancesResponse {
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of Redis instances in the project in the specified location, or across all locations. If the `location_id` in the parent field of the request is \"-\", all regions available to the project are queried, and the results aggregated. If in such an aggregated query a location is unavailable, a placeholder Redis entry is included in the response with the `name` field set to a value of the form `projects/{project_id}/locations/{location_id}/instances/`- and the `status` field set to ERROR and `status_message` field set to \"location not available for ListInstances\"."]
    pub instances: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Instance>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
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
    #[doc = "Resource ID for the region. For example: \"us-east1\"."]
    pub location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The set of available zones in the location. The map is keyed by the lowercase ID of each zone, as defined by Compute Engine. These keys can be specified in `location_id` or `alternative_location_id` fields when creating a Redis instance."]
    pub metadata: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Full resource name for the region. For example: \"projects/example-project/locations/us-east1\"."]
    pub name: ::std::option::Option<::std::string::String>,
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
    #[doc = "{ `createTime`: The time the operation was created. `endTime`: The time the operation finished running. `target`: Server-defined resource path for the target of the operation. `verb`: Name of the verb executed by the operation. `statusDetail`: Human-readable status of the operation, if any. `cancelRequested`: Identifies whether the user has requested cancellation of the operation. Operations that have successfully been cancelled have Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`. `apiVersion`: API version used to start the operation. }"]
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
#[doc = "The output content"]
pub struct OutputConfig {
    #[serde(rename = "gcsDestination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Google Cloud Storage destination for output content."]
    pub gcs_destination: ::std::option::Option<::std::boxed::Box<GcsDestination>>,
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
#[doc = "TlsCertificate Resource"]
pub struct TlsCertificate {
    #[serde(rename = "cert")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "PEM representation."]
    pub cert: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time when the certificate was created in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2020-05-18T00:00:00.094Z`."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expireTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time when the certificate expires in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2020-05-18T00:00:00.094Z`."]
    pub expire_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serialNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Serial number, as extracted from the certificate."]
    pub serial_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sha1Fingerprint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sha1 Fingerprint of the certificate."]
    pub sha1_fingerprint: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for UpgradeInstance."]
pub struct UpgradeInstanceRequest {
    #[serde(rename = "redisVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Specifies the target version of Redis software to upgrade to."]
    pub redis_version: ::std::option::Option<::std::string::String>,
}
