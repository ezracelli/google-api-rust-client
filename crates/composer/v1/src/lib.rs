#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Allowed IP range with user-provided description."]
pub struct AllowedIpRange {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. User-provided description. It must contain at most 300 characters."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "IP address or range, defined using CIDR notation, of requests that this rule applies to. Examples: `192.168.1.1` or `192.168.0.0/16` or `2001:db8::/32` or `2001:0db8:0000:0042:0000:8a2e:0370:7334`. IP range prefixes should be properly truncated. For example, `1.2.3.4/24` should be truncated to `1.2.3.0/24`. Similarly, for IPv6, `2001:db8::1/32` should be truncated to `2001:db8::/32`."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The configuration of Cloud SQL instance that is used by the Apache Airflow software."]
pub struct DatabaseConfig {
    #[serde(rename = "machineType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Cloud SQL machine type used by Airflow database. It has to be one of: db-n1-standard-2, db-n1-standard-4, db-n1-standard-8 or db-n1-standard-16. If not specified, db-n1-standard-2 will be used."]
    pub machine_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values * A month and day value, with a zero year, such as an anniversary * A year on its own, with zero month and day values * A year and month value, with a zero day, such as a credit card expiration date Related types are google.type.TimeOfDay and `google.protobuf.Timestamp`."]
pub struct Date {
    #[serde(rename = "day")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
    pub day: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "month")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
    pub month: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "year")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
    pub year: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The encryption options for the Cloud Composer environment and its dependencies."]
pub struct EncryptionConfig {
    #[serde(rename = "kmsKeyName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Customer-managed Encryption Key available through Google's Key Management Service. Cannot be updated. If not specified, Google-managed key will be used."]
    pub kms_key_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An environment for running orchestration tasks."]
pub struct Environment {
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration parameters for this environment."]
    pub config: ::std::option::Option<::std::boxed::Box<EnvironmentConfig>>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time at which this environment was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. User-defined labels for this environment. The labels map can contain no more than 64 entries. Entries of the labels map are UTF8 strings that comply with the following restrictions: * Keys must conform to regexp: \\p{Ll}\\p{Lo}{0,62} * Values must conform to regexp: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63} * Both keys and values are additionally constrained to be <= 128 bytes in size."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the environment, in the form: \"projects/{projectId}/locations/{locationId}/environments/{environmentId}\" EnvironmentId must start with a lowercase letter followed by up to 63 lowercase letters, numbers, or hyphens, and cannot end with a hyphen."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current state of the environment."]
    pub state: ::std::option::Option<EnvironmentStateEnum>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time at which this environment was last modified."]
    pub update_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uuid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The UUID (Universally Unique IDentifier) associated with this environment. This value is generated when the environment is created."]
    pub uuid: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The current state of the environment."]
pub enum EnvironmentStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "The state of the environment is unknown."]
    StateUnspecified,
    #[serde(rename = "CREATING")]
    #[doc = "The environment is in the process of being created."]
    Creating,
    #[serde(rename = "RUNNING")]
    #[doc = "The environment is currently running and healthy. It is ready for use."]
    Running,
    #[serde(rename = "UPDATING")]
    #[doc = "The environment is being updated. It remains usable but cannot receive additional update requests or be deleted at this time."]
    Updating,
    #[serde(rename = "DELETING")]
    #[doc = "The environment is undergoing deletion. It cannot be used."]
    Deleting,
    #[serde(rename = "ERROR")]
    #[doc = "The environment has encountered an error and cannot be used."]
    Error,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration information for an environment."]
pub struct EnvironmentConfig {
    #[serde(rename = "airflowUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The URI of the Apache Airflow Web UI hosted within this environment (see [Airflow web interface](/composer/docs/how-to/accessing/airflow-web-interface))."]
    pub airflow_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dagGcsPrefix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The Cloud Storage prefix of the DAGs for this environment. Although Cloud Storage objects reside in a flat namespace, a hierarchical file tree can be simulated using \"/\"-delimited object name prefixes. DAG objects for this environment reside in a simulated directory with the given prefix."]
    pub dag_gcs_prefix: ::std::option::Option<::std::string::String>,
    #[serde(rename = "databaseConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The configuration settings for Cloud SQL instance used internally by Apache Airflow software."]
    pub database_config: ::std::option::Option<::std::boxed::Box<DatabaseConfig>>,
    #[serde(rename = "encryptionConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The encryption options for the Cloud Composer environment and its dependencies. Cannot be updated."]
    pub encryption_config: ::std::option::Option<::std::boxed::Box<EncryptionConfig>>,
    #[serde(rename = "gkeCluster")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The Kubernetes Engine cluster used to run this environment."]
    pub gke_cluster: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nodeConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration used for the Kubernetes Engine cluster."]
    pub node_config: ::std::option::Option<::std::boxed::Box<NodeConfig>>,
    #[serde(rename = "nodeCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of nodes in the Kubernetes Engine cluster that will be used to run this environment."]
    pub node_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "privateEnvironmentConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration used for the Private IP Cloud Composer environment."]
    pub private_environment_config:
        ::std::option::Option<::std::boxed::Box<PrivateEnvironmentConfig>>,
    #[serde(rename = "softwareConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration settings for software inside the environment."]
    pub software_config: ::std::option::Option<::std::boxed::Box<SoftwareConfig>>,
    #[serde(rename = "webServerConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The configuration settings for the Airflow web server App Engine instance."]
    pub web_server_config: ::std::option::Option<::std::boxed::Box<WebServerConfig>>,
    #[serde(rename = "webServerNetworkAccessControl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The network-level access control policy for the Airflow web server. If unspecified, no network-level access restrictions will be applied."]
    pub web_server_network_access_control:
        ::std::option::Option<::std::boxed::Box<WebServerNetworkAccessControl>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for controlling how IPs are allocated in the GKE cluster running the Apache Airflow software."]
pub struct IpAllocationPolicy {
    #[serde(rename = "clusterIpv4CidrBlock")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The IP address range used to allocate IP addresses to pods in the GKE cluster. This field is applicable only when `use_ip_aliases` is true. Set to blank to have GKE choose a range with the default size. Set to /netmask (e.g. `/14`) to have GKE choose a range with a specific netmask. Set to a [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g. `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range to use."]
    pub cluster_ipv4_cidr_block: ::std::option::Option<::std::string::String>,
    #[serde(rename = "clusterSecondaryRangeName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The name of the GKE cluster's secondary range used to allocate IP addresses to pods. This field is applicable only when `use_ip_aliases` is true."]
    pub cluster_secondary_range_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "servicesIpv4CidrBlock")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The IP address range of the services IP addresses in this GKE cluster. This field is applicable only when `use_ip_aliases` is true. Set to blank to have GKE choose a range with the default size. Set to /netmask (e.g. `/14`) to have GKE choose a range with a specific netmask. Set to a [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g. `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range to use."]
    pub services_ipv4_cidr_block: ::std::option::Option<::std::string::String>,
    #[serde(rename = "servicesSecondaryRangeName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The name of the services' secondary range used to allocate IP addresses to the GKE cluster. This field is applicable only when `use_ip_aliases` is true."]
    pub services_secondary_range_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "useIpAliases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Whether or not to enable Alias IPs in the GKE cluster. If `true`, a VPC-native cluster is created."]
    pub use_ip_aliases: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ImageVersion information"]
pub struct ImageVersion {
    #[serde(rename = "creationDisabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether it is impossible to create an environment with the image version."]
    pub creation_disabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "imageVersionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The string identifier of the ImageVersion, in the form: \"composer-x.y.z-airflow-a.b(.c)\""]
    pub image_version_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isDefault")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this is the default ImageVersion used by Composer during environment creation if no input ImageVersion is specified."]
    pub is_default: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "releaseDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date of the version release."]
    pub release_date: ::std::option::Option<::std::boxed::Box<Date>>,
    #[serde(rename = "supportedPythonVersions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "supported python versions"]
    pub supported_python_versions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "upgradeDisabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether it is impossible to upgrade an environment running with the image version."]
    pub upgrade_disabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The environments in a project and location."]
pub struct ListEnvironmentsResponse {
    #[serde(rename = "environments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of environments returned by a ListEnvironmentsRequest."]
    pub environments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Environment>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The page token used to query for the next page if one exists."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The ImageVersions in a project and location."]
pub struct ListImageVersionsResponse {
    #[serde(rename = "imageVersions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of supported ImageVersions in a location."]
    pub image_versions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ImageVersion>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The page token used to query for the next page if one exists."]
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
#[doc = "The configuration information for the Kubernetes Engine nodes running the Apache Airflow software."]
pub struct NodeConfig {
    #[serde(rename = "diskSizeGb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The disk size in GB used for node VMs. Minimum size is 20GB. If unspecified, defaults to 100GB. Cannot be updated."]
    pub disk_size_gb: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "ipAllocationPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The configuration for controlling how IPs are allocated in the GKE cluster."]
    pub ip_allocation_policy: ::std::option::Option<::std::boxed::Box<IpAllocationPolicy>>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The Compute Engine [zone](/compute/docs/regions-zones) in which to deploy the VMs used to run the Apache Airflow software, specified as a [relative resource name](/apis/design/resource_names#relative_resource_name). For example: \"projects/{projectId}/zones/{zoneId}\". This `location` must belong to the enclosing environment's project and location. If both this field and `nodeConfig.machineType` are specified, `nodeConfig.machineType` must belong to this `location`; if both are unspecified, the service will pick a zone in the Compute Engine region corresponding to the Cloud Composer location, and propagate that choice to both fields. If only one field (`location` or `nodeConfig.machineType`) is specified, the location information from the specified field will be propagated to the unspecified field."]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "machineType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The Compute Engine [machine type](/compute/docs/machine-types) used for cluster instances, specified as a [relative resource name](/apis/design/resource_names#relative_resource_name). For example: \"projects/{projectId}/zones/{zoneId}/machineTypes/{machineTypeId}\". The `machineType` must belong to the enclosing environment's project and location. If both this field and `nodeConfig.location` are specified, this `machineType` must belong to the `nodeConfig.location`; if both are unspecified, the service will pick a zone in the Compute Engine region corresponding to the Cloud Composer location, and propagate that choice to both fields. If exactly one of this field and `nodeConfig.location` is specified, the location information from the specified field will be propagated to the unspecified field. The `machineTypeId` must not be a [shared-core machine type](/compute/docs/machine-types#sharedcore). If this field is unspecified, the `machineTypeId` defaults to \"n1-standard-1\"."]
    pub machine_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "network")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The Compute Engine network to be used for machine communications, specified as a [relative resource name](/apis/design/resource_names#relative_resource_name). For example: \"projects/{projectId}/global/networks/{networkId}\". If unspecified, the \"default\" network ID in the environment's project is used. If a [Custom Subnet Network](/vpc/docs/vpc#vpc_networks_and_subnets) is provided, `nodeConfig.subnetwork` must also be provided. For [Shared VPC](/vpc/docs/shared-vpc) subnetwork requirements, see `nodeConfig.subnetwork`."]
    pub network: ::std::option::Option<::std::string::String>,
    #[serde(rename = "oauthScopes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The set of Google API scopes to be made available on all node VMs. If `oauth_scopes` is empty, defaults to [\"https://www.googleapis.com/auth/cloud-platform\"]. Cannot be updated."]
    pub oauth_scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "serviceAccount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The Google Cloud Platform Service Account to be used by the node VMs. If a service account is not specified, the \"default\" Compute Engine service account is used. Cannot be updated."]
    pub service_account: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subnetwork")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The Compute Engine subnetwork to be used for machine communications, specified as a [relative resource name](/apis/design/resource_names#relative_resource_name). For example: \"projects/{projectId}/regions/{regionId}/subnetworks/{subnetworkId}\" If a subnetwork is provided, `nodeConfig.network` must also be provided, and the subnetwork must belong to the enclosing environment's project and location."]
    pub subnetwork: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The list of instance tags applied to all node VMs. Tags are used to identify valid sources or targets for network firewalls. Each tag within the list must comply with [RFC1035](https://www.ietf.org/rfc/rfc1035.txt). Cannot be updated."]
    pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
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
#[doc = "Metadata describing an operation."]
pub struct OperationMetadata {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time the operation was submitted to the server."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time when the operation terminated, regardless of its success. This field is unset if the operation is still ongoing."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of operation being performed."]
    pub operation_type: ::std::option::Option<OperationMetadataOperationTypeEnum>,
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource being operated on, as a [relative resource name]( /apis/design/resource_names#relative_resource_name)."]
    pub resource: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceUuid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The UUID of the resource being operated on."]
    pub resource_uuid: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The current operation state."]
    pub state: ::std::option::Option<OperationMetadataStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The type of operation being performed."]
pub enum OperationMetadataOperationTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "Unused."]
    TypeUnspecified,
    #[serde(rename = "CREATE")]
    #[doc = "A resource creation operation."]
    Create,
    #[serde(rename = "DELETE")]
    #[doc = "A resource deletion operation."]
    Delete,
    #[serde(rename = "UPDATE")]
    #[doc = "A resource update operation."]
    Update,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The current operation state."]
pub enum OperationMetadataStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Unused."]
    StateUnspecified,
    #[serde(rename = "PENDING")]
    #[doc = "The operation has been created but is not yet started."]
    Pending,
    #[serde(rename = "RUNNING")]
    #[doc = "The operation is underway."]
    Running,
    #[serde(rename = "SUCCEEDED")]
    #[doc = "The operation completed successfully."]
    Succeeded,
    #[serde(rename = "SUCCESSFUL")]
    #[doc = ""]
    Successful,
    #[serde(rename = "FAILED")]
    #[doc = "The operation is no longer running but did not succeed."]
    Failed,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration options for the private GKE cluster in a Cloud Composer environment."]
pub struct PrivateClusterConfig {
    #[serde(rename = "enablePrivateEndpoint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If `true`, access to the public endpoint of the GKE cluster is denied."]
    pub enable_private_endpoint: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "masterIpv4CidrBlock")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The CIDR block from which IPv4 range for GKE master will be reserved. If left blank, the default value of '172.16.0.0/23' is used."]
    pub master_ipv4_cidr_block: ::std::option::Option<::std::string::String>,
    #[serde(rename = "masterIpv4ReservedRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The IP range in CIDR notation to use for the hosted master network. This range is used for assigning internal IP addresses to the GKE cluster master or set of masters and to the internal load balancer virtual IP. This range must not overlap with any other ranges in use within the cluster's network."]
    pub master_ipv4_reserved_range: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The configuration information for configuring a Private IP Cloud Composer environment."]
pub struct PrivateEnvironmentConfig {
    #[serde(rename = "cloudSqlIpv4CidrBlock")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The CIDR block from which IP range in tenant project will be reserved for Cloud SQL. Needs to be disjoint from `web_server_ipv4_cidr_block`."]
    pub cloud_sql_ipv4_cidr_block: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enablePrivateEnvironment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If `true`, a Private IP Cloud Composer environment is created. If this field is set to true, `IPAllocationPolicy.use_ip_aliases` must be set to true."]
    pub enable_private_environment: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "privateClusterConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Configuration for the private GKE cluster for a Private IP Cloud Composer environment."]
    pub private_cluster_config: ::std::option::Option<::std::boxed::Box<PrivateClusterConfig>>,
    #[serde(rename = "webServerIpv4CidrBlock")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The CIDR block from which IP range for web server will be reserved. Needs to be disjoint from `private_cluster_config.master_ipv4_cidr_block` and `cloud_sql_ipv4_cidr_block`."]
    pub web_server_ipv4_cidr_block: ::std::option::Option<::std::string::String>,
    #[serde(rename = "webServerIpv4ReservedRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The IP range reserved for the tenant project's App Engine VMs."]
    pub web_server_ipv4_reserved_range: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies the selection and configuration of software inside the environment."]
pub struct SoftwareConfig {
    #[serde(rename = "airflowConfigOverrides")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Apache Airflow configuration properties to override. Property keys contain the section and property names, separated by a hyphen, for example \"core-dags_are_paused_at_creation\". Section names must not contain hyphens (\"-\"), opening square brackets (\"[\"), or closing square brackets (\"]\"). The property name must not be empty and must not contain an equals sign (\"=\") or semicolon (\";\"). Section and property names must not contain a period (\".\"). Apache Airflow configuration property names must be written in [snake_case](https://en.wikipedia.org/wiki/Snake_case). Property values can contain any character, and can be written in any lower/upper case format. Certain Apache Airflow configuration property values are [blocked](/composer/docs/concepts/airflow-configurations), and cannot be overridden."]
    pub airflow_config_overrides:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "envVariables")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Additional environment variables to provide to the Apache Airflow scheduler, worker, and webserver processes. Environment variable names must match the regular expression `a-zA-Z_*`. They cannot specify Apache Airflow software configuration overrides (they cannot match the regular expression `AIRFLOW__[A-Z0-9_]+__[A-Z0-9_]+`), and they cannot match any of the following reserved names: * `AIRFLOW_HOME` * `C_FORCE_ROOT` * `CONTAINER_NAME` * `DAGS_FOLDER` * `GCP_PROJECT` * `GCS_BUCKET` * `GKE_CLUSTER_NAME` * `SQL_DATABASE` * `SQL_INSTANCE` * `SQL_PASSWORD` * `SQL_PROJECT` * `SQL_REGION` * `SQL_USER`"]
    pub env_variables:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "imageVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version of the software running in the environment. This encapsulates both the version of Cloud Composer functionality and the version of Apache Airflow. It must match the regular expression `composer-([0-9]+\\.[0-9]+\\.[0-9]+|latest)-airflow-[0-9]+\\.[0-9]+(\\.[0-9]+.*)?`. When used as input, the server also checks if the provided version is supported and denies the request for an unsupported version. The Cloud Composer portion of the version is a [semantic version](https://semver.org) or `latest`. When the patch version is omitted, the current Cloud Composer patch version is selected. When `latest` is provided instead of an explicit version number, the server replaces `latest` with the current Cloud Composer version and stores that version number in the same field. The portion of the image version that follows *airflow-* is an official Apache Airflow repository [release name](https://github.com/apache/incubator-airflow/releases). See also [Version List](/composer/docs/concepts/versioning/composer-versions)."]
    pub image_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pypiPackages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Custom Python Package Index (PyPI) packages to be installed in the environment. Keys refer to the lowercase package name such as \"numpy\" and values are the lowercase extras and version specifier such as \"==1.12.0\", \"[devel,gcp_api]\", or \"[devel]>=1.8.2, <1.9.2\". To specify a package without pinning it to a version specifier, use the empty string as the value."]
    pub pypi_packages:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "pythonVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The major version of Python used to run the Apache Airflow scheduler, worker, and webserver processes. Can be set to '2' or '3'. If not specified, the default is '2'. Cannot be updated."]
    pub python_version: ::std::option::Option<::std::string::String>,
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
#[doc = "The configuration settings for the Airflow web server App Engine instance."]
pub struct WebServerConfig {
    #[serde(rename = "machineType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Machine type on which Airflow web server is running. It has to be one of: composer-n1-webserver-2, composer-n1-webserver-4 or composer-n1-webserver-8. If not specified, composer-n1-webserver-2 will be used. Value custom is returned only in response, if Airflow web server parameters were manually changed to a non-standard values."]
    pub machine_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Network-level access control policy for the Airflow web server."]
pub struct WebServerNetworkAccessControl {
    #[serde(rename = "allowedIpRanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A collection of allowed IP ranges with descriptions."]
    pub allowed_ip_ranges:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AllowedIpRange>>>,
}
