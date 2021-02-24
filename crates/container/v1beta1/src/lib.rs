#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "AcceleratorConfig represents a Hardware Accelerator request."]
pub struct AcceleratorConfig {
    #[serde(rename = "acceleratorCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of the accelerator cards exposed to an instance."]
    pub accelerator_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "acceleratorType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The accelerator type resource name. List of supported accelerators [here](https://cloud.google.com/compute/docs/gpus)"]
    pub accelerator_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for the addons that can be automatically spun up in the cluster, enabling additional functionality."]
pub struct AddonsConfig {
    #[serde(rename = "cloudRunConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for the Cloud Run addon. The `IstioConfig` addon must be enabled in order to enable Cloud Run addon. This option can only be enabled at cluster creation time."]
    pub cloud_run_config: ::std::option::Option<::std::boxed::Box<CloudRunConfig>>,
    #[serde(rename = "configConnectorConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for the ConfigConnector add-on, a Kubernetes extension to manage hosted GCP services through the Kubernetes API"]
    pub config_connector_config: ::std::option::Option<::std::boxed::Box<ConfigConnectorConfig>>,
    #[serde(rename = "dnsCacheConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for NodeLocalDNS, a dns cache running on cluster nodes"]
    pub dns_cache_config: ::std::option::Option<::std::boxed::Box<DnsCacheConfig>>,
    #[serde(rename = "gcePersistentDiskCsiDriverConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for the Compute Engine Persistent Disk CSI driver."]
    pub gce_persistent_disk_csi_driver_config:
        ::std::option::Option<::std::boxed::Box<GcePersistentDiskCsiDriverConfig>>,
    #[serde(rename = "horizontalPodAutoscaling")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for the horizontal pod autoscaling feature, which increases or decreases the number of replica pods a replication controller has based on the resource usage of the existing pods."]
    pub horizontal_pod_autoscaling:
        ::std::option::Option<::std::boxed::Box<HorizontalPodAutoscaling>>,
    #[serde(rename = "httpLoadBalancing")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for the HTTP (L7) load balancing controller addon, which makes it easy to set up HTTP load balancers for services in a cluster."]
    pub http_load_balancing: ::std::option::Option<::std::boxed::Box<HttpLoadBalancing>>,
    #[serde(rename = "istioConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for Istio, an open platform to connect, manage, and secure microservices."]
    pub istio_config: ::std::option::Option<::std::boxed::Box<IstioConfig>>,
    #[serde(rename = "kalmConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for the KALM addon, which manages the lifecycle of k8s applications."]
    pub kalm_config: ::std::option::Option<::std::boxed::Box<KalmConfig>>,
    #[serde(rename = "kubernetesDashboard")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for the Kubernetes Dashboard. This addon is deprecated, and will be disabled in 1.15. It is recommended to use the Cloud Console to manage and monitor your Kubernetes clusters, workloads and applications. For more information, see: https://cloud.google.com/kubernetes-engine/docs/concepts/dashboards"]
    pub kubernetes_dashboard: ::std::option::Option<::std::boxed::Box<KubernetesDashboard>>,
    #[serde(rename = "networkPolicyConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for NetworkPolicy. This only tracks whether the addon is enabled or not on the Master, it does not track whether network policy is enabled for the nodes."]
    pub network_policy_config: ::std::option::Option<::std::boxed::Box<NetworkPolicyConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for returning group information from authenticators."]
pub struct AuthenticatorGroupsConfig {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this cluster should return group membership lookups during authentication using a group of security groups."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "securityGroup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the security group-of-groups to be used. Only relevant if enabled = true."]
    pub security_group: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "AutoUpgradeOptions defines the set of options for the user to control how the Auto Upgrades will proceed."]
pub struct AutoUpgradeOptions {
    #[serde(rename = "autoUpgradeStartTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] This field is set when upgrades are about to commence with the approximate start time for the upgrades, in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format."]
    pub auto_upgrade_start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] This field is set when upgrades are about to commence with the description of the upgrade."]
    pub description: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Autopilot is the configuration for Autopilot settings on the cluster. It is the official product name of what is previously known as AutoGKE"]
pub struct Autopilot {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enable Autopilot"]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "AutoprovisioningNodePoolDefaults contains defaults for a node pool created by NAP."]
pub struct AutoprovisioningNodePoolDefaults {
    #[serde(rename = "bootDiskKmsKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = " The Customer Managed Encryption Key used to encrypt the boot disk attached to each node in the node pool. This should be of the form projects/[KEY_PROJECT_ID]/locations/[LOCATION]/keyRings/[RING_NAME]/cryptoKeys/[KEY_NAME]. For more information about protecting resources with Cloud KMS Keys please see: https://cloud.google.com/compute/docs/disks/customer-managed-encryption"]
    pub boot_disk_kms_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "diskSizeGb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Size of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB. If unspecified, the default disk size is 100GB."]
    pub disk_size_gb: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "diskType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the disk attached to each node (e.g. 'pd-standard', 'pd-ssd' or 'pd-balanced') If unspecified, the default disk type is 'pd-standard'"]
    pub disk_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "management")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "NodeManagement configuration for this NodePool."]
    pub management: ::std::option::Option<::std::boxed::Box<NodeManagement>>,
    #[serde(rename = "minCpuPlatform")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Minimum CPU platform to be used by this instance. The instance may be scheduled on the specified or newer CPU platform. Applicable values are the friendly names of CPU platforms, such as `minCpuPlatform: \"Intel Haswell\"` or `minCpuPlatform: \"Intel Sandy Bridge\"`. For more information, read [how to specify min CPU platform](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform) To unset the min cpu platform field pass \"automatic\" as field value."]
    pub min_cpu_platform: ::std::option::Option<::std::string::String>,
    #[serde(rename = "oauthScopes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of Google API scopes to be made available on all of the node VMs under the \"default\" service account. The following scopes are recommended, but not required, and by default are not included: * `https://www.googleapis.com/auth/compute` is required for mounting persistent storage on your nodes. * `https://www.googleapis.com/auth/devstorage.read_only` is required for communicating with **gcr.io** (the [Google Container Registry](https://cloud.google.com/container-registry/)). If unspecified, no scopes are added, unless Cloud Logging or Cloud Monitoring are enabled, in which case their required scopes will be added."]
    pub oauth_scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "serviceAccount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Google Cloud Platform Service Account to be used by the node VMs. Specify the email address of the Service Account; otherwise, if no Service Account is specified, the \"default\" service account is used."]
    pub service_account: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shieldedInstanceConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shielded Instance options."]
    pub shielded_instance_config: ::std::option::Option<::std::boxed::Box<ShieldedInstanceConfig>>,
    #[serde(rename = "upgradeSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Upgrade settings control disruption and speed of the upgrade."]
    pub upgrade_settings: ::std::option::Option<::std::boxed::Box<UpgradeSettings>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Deprecated."]
pub struct AvailableVersion {
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reason for availability."]
    pub reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Kubernetes version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Parameters for using BigQuery as the destination of resource usage export."]
pub struct BigQueryDestination {
    #[serde(rename = "datasetId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of a BigQuery Dataset."]
    pub dataset_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for Binary Authorization."]
pub struct BinaryAuthorization {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enable Binary Authorization for this cluster. If enabled, all container images will be validated by Google Binauthz."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "CancelOperationRequest cancels a single operation."]
pub struct CancelOperationRequest {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name (project, location, operation id) of the operation to cancel. Specified in the format `projects/*/locations/*/operations/*`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The server-assigned `name` of the operation. This field has been deprecated and replaced by the name field."]
    pub operation_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the operation resides. This field has been deprecated and replaced by the name field."]
    pub zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "CidrBlock contains an optional name and one CIDR block."]
pub struct CidrBlock {
    #[serde(rename = "cidrBlock")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "cidr_block must be specified in CIDR notation."]
    pub cidr_block: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "display_name is an optional field for users to identify CIDR blocks."]
    pub display_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for client certificates on the cluster."]
pub struct ClientCertificateConfig {
    #[serde(rename = "issueClientCertificate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Issue a client certificate."]
    pub issue_client_certificate: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration options for the Cloud Run feature."]
pub struct CloudRunConfig {
    #[serde(rename = "disabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether Cloud Run addon is enabled for this cluster."]
    pub disabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "loadBalancerType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Which load balancer type is installed for Cloud Run."]
    pub load_balancer_type: ::std::option::Option<CloudRunConfigLoadBalancerTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Which load balancer type is installed for Cloud Run."]
pub enum CloudRunConfigLoadBalancerTypeEnum {
    #[serde(rename = "LOAD_BALANCER_TYPE_UNSPECIFIED")]
    #[doc = "Load balancer type for Cloud Run is unspecified."]
    LoadBalancerTypeUnspecified,
    #[serde(rename = "LOAD_BALANCER_TYPE_EXTERNAL")]
    #[doc = "Install external load balancer for Cloud Run."]
    LoadBalancerTypeExternal,
    #[serde(rename = "LOAD_BALANCER_TYPE_INTERNAL")]
    #[doc = "Install internal load balancer for Cloud Run."]
    LoadBalancerTypeInternal,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Google Kubernetes Engine cluster."]
pub struct Cluster {
    #[serde(rename = "addonsConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configurations for the various addons available to run in the cluster."]
    pub addons_config: ::std::option::Option<::std::boxed::Box<AddonsConfig>>,
    #[serde(rename = "authenticatorGroupsConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration controlling RBAC group membership information."]
    pub authenticator_groups_config:
        ::std::option::Option<::std::boxed::Box<AuthenticatorGroupsConfig>>,
    #[serde(rename = "autopilot")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Autopilot configuration for the cluster. It has the same semantics as AutoGKE and overrides the setting in autogke."]
    pub autopilot: ::std::option::Option<::std::boxed::Box<Autopilot>>,
    #[serde(rename = "autoscaling")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cluster-level autoscaling configuration."]
    pub autoscaling: ::std::option::Option<::std::boxed::Box<ClusterAutoscaling>>,
    #[serde(rename = "binaryAuthorization")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for Binary Authorization."]
    pub binary_authorization: ::std::option::Option<::std::boxed::Box<BinaryAuthorization>>,
    #[serde(rename = "clusterIpv4Cidr")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IP address range of the container pods in this cluster, in [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `10.96.0.0/14`). Leave blank to have one automatically chosen or specify a `/14` block in `10.0.0.0/8`."]
    pub cluster_ipv4_cidr: ::std::option::Option<::std::string::String>,
    #[serde(rename = "clusterTelemetry")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Telemetry integration for the cluster."]
    pub cluster_telemetry: ::std::option::Option<::std::boxed::Box<ClusterTelemetry>>,
    #[serde(rename = "conditions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Which conditions caused the current cluster state."]
    pub conditions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StatusCondition>>>,
    #[serde(rename = "confidentialNodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration of Confidential Nodes"]
    pub confidential_nodes: ::std::option::Option<::std::boxed::Box<ConfidentialNodes>>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] The time the cluster was created, in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "currentMasterVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] The current software version of the master endpoint."]
    pub current_master_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "currentNodeCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] The number of nodes currently in the cluster. Deprecated. Call Kubernetes API directly to retrieve node information."]
    pub current_node_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "currentNodeVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] Deprecated, use [NodePool.version](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1beta1/projects.locations.clusters.nodePools) instead. The current version of the node software components. If they are currently at multiple versions because they're in the process of being upgraded, this reflects the minimum version of all nodes."]
    pub current_node_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "databaseEncryption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration of etcd encryption."]
    pub database_encryption: ::std::option::Option<::std::boxed::Box<DatabaseEncryption>>,
    #[serde(rename = "defaultMaxPodsConstraint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The default constraint on the maximum number of pods that can be run simultaneously on a node in the node pool of this cluster. Only honored if cluster created with IP Alias support."]
    pub default_max_pods_constraint: ::std::option::Option<::std::boxed::Box<MaxPodsConstraint>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An optional description of this cluster."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enableKubernetesAlpha")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Kubernetes alpha features are enabled on this cluster. This includes alpha API groups (e.g. v1beta1) and features that may not be production ready in the kubernetes version of the master and nodes. The cluster has no SLA for uptime and master/node upgrades are disabled. Alpha enabled clusters are automatically deleted thirty days after creation."]
    pub enable_kubernetes_alpha: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "enableTpu")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enable the ability to use Cloud TPUs in this cluster. This field is deprecated, use tpu_config.enabled instead."]
    pub enable_tpu: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "endpoint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] The IP address of this cluster's master endpoint. The endpoint can be accessed from the internet at `https://username:password@endpoint/`. See the `masterAuth` property of this resource for username and password information."]
    pub endpoint: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expireTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] The time the cluster will be automatically deleted in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format."]
    pub expire_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "initialClusterVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The initial Kubernetes version for this cluster. Valid versions are those found in validMasterVersions returned by getServerConfig. The version can be upgraded over time; such upgrades are reflected in currentMasterVersion and currentNodeVersion. Users may specify either explicit versions offered by Kubernetes Engine or version aliases, which have the following behavior: - \"latest\": picks the highest valid Kubernetes version - \"1.X\": picks the highest valid patch+gke.N patch in the 1.X version - \"1.X.Y\": picks the highest valid gke.N patch in the 1.X.Y version - \"1.X.Y-gke.N\": picks an explicit Kubernetes version - \"\",\"-\": picks the default Kubernetes version"]
    pub initial_cluster_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "initialNodeCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of nodes to create in this cluster. You must ensure that your Compute Engine [resource quota](https://cloud.google.com/compute/quotas) is sufficient for this number of instances. You must also have available firewall and routes quota. For requests, this field should only be used in lieu of a \"node_pool\" object, since this configuration (along with the \"node_config\") will be used to create a \"NodePool\" object with an auto-generated name. Do not use this and a node_pool at the same time. This field is deprecated, use node_pool.initial_node_count instead."]
    pub initial_node_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "instanceGroupUrls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Use node_pools.instance_group_urls."]
    pub instance_group_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "ipAllocationPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for cluster IP allocation."]
    pub ip_allocation_policy: ::std::option::Option<::std::boxed::Box<IpAllocationPolicy>>,
    #[serde(rename = "labelFingerprint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fingerprint of the set of labels for this cluster."]
    pub label_fingerprint: ::std::option::Option<::std::string::String>,
    #[serde(rename = "legacyAbac")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for the legacy ABAC authorization mode."]
    pub legacy_abac: ::std::option::Option<::std::boxed::Box<LegacyAbac>>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) or [region](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) in which the cluster resides."]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available) in which the cluster's nodes should be located. This field provides a default value if [NodePool.Locations](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters.nodePools#NodePool.FIELDS.locations) are not specified during node pool creation. Warning: changing cluster locations will update the [NodePool.Locations](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters.nodePools#NodePool.FIELDS.locations) of all node pools and will result in nodes being added and/or removed."]
    pub locations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "loggingService")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The logging service the cluster should use to write logs. Currently available options: * `logging.googleapis.com/kubernetes` - The Cloud Logging service with a Kubernetes-native resource model * `logging.googleapis.com` - The legacy Cloud Logging service (no longer available as of GKE 1.15). * `none` - no logs will be exported from the cluster. If left as an empty string,`logging.googleapis.com/kubernetes` will be used for GKE 1.14+ or `logging.googleapis.com` for earlier versions."]
    pub logging_service: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maintenancePolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configure the maintenance policy for this cluster."]
    pub maintenance_policy: ::std::option::Option<::std::boxed::Box<MaintenancePolicy>>,
    #[serde(rename = "master")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for master components."]
    pub master: ::std::option::Option<::std::boxed::Box<Master>>,
    #[serde(rename = "masterAuth")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The authentication information for accessing the master endpoint. If unspecified, the defaults are used: For clusters before v1.12, if master_auth is unspecified, `username` will be set to \"admin\", a random password will be generated, and a client certificate will be issued."]
    pub master_auth: ::std::option::Option<::std::boxed::Box<MasterAuth>>,
    #[serde(rename = "masterAuthorizedNetworksConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration options for master authorized networks feature."]
    pub master_authorized_networks_config:
        ::std::option::Option<::std::boxed::Box<MasterAuthorizedNetworksConfig>>,
    #[serde(rename = "masterIpv4CidrBlock")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IP prefix in CIDR notation to use for the hosted master network. This prefix will be used for assigning private IP addresses to the master or set of masters, as well as the ILB VIP. This field is deprecated, use private_cluster_config.master_ipv4_cidr_block instead."]
    pub master_ipv4_cidr_block: ::std::option::Option<::std::string::String>,
    #[serde(rename = "monitoringService")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The monitoring service the cluster should use to write metrics. Currently available options: * \"monitoring.googleapis.com/kubernetes\" - The Cloud Monitoring service with a Kubernetes-native resource model * `monitoring.googleapis.com` - The legacy Cloud Monitoring service (no longer available as of GKE 1.15). * `none` - No metrics will be exported from the cluster. If left as an empty string,`monitoring.googleapis.com/kubernetes` will be used for GKE 1.14+ or `monitoring.googleapis.com` for earlier versions."]
    pub monitoring_service: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of this cluster. The name must be unique within this project and location (e.g. zone or region), and can be up to 40 characters with the following restrictions: * Lowercase letters, numbers, and hyphens only. * Must start with a letter. * Must end with a number or a letter."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "network")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the Google Compute Engine [network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks) to which the cluster is connected. If left unspecified, the `default` network will be used. On output this shows the network ID instead of the name."]
    pub network: ::std::option::Option<::std::string::String>,
    #[serde(rename = "networkConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for cluster networking."]
    pub network_config: ::std::option::Option<::std::boxed::Box<NetworkConfig>>,
    #[serde(rename = "networkPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration options for the NetworkPolicy feature."]
    pub network_policy: ::std::option::Option<::std::boxed::Box<NetworkPolicy>>,
    #[serde(rename = "nodeConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parameters used in creating the cluster's nodes. For requests, this field should only be used in lieu of a \"node_pool\" object, since this configuration (along with the \"initial_node_count\") will be used to create a \"NodePool\" object with an auto-generated name. Do not use this and a node_pool at the same time. For responses, this field will be populated with the node configuration of the first node pool. (For configuration of each node pool, see `node_pool.config`) If unspecified, the defaults are used. This field is deprecated, use node_pool.config instead."]
    pub node_config: ::std::option::Option<::std::boxed::Box<NodeConfig>>,
    #[serde(rename = "nodeIpv4CidrSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] The size of the address space on each node for hosting containers. This is provisioned from within the `container_ipv4_cidr` range. This field will only be set when cluster is in route-based network mode."]
    pub node_ipv4_cidr_size: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "nodePools")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The node pools associated with this cluster. This field should not be set if \"node_config\" or \"initial_node_count\" are specified."]
    pub node_pools: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NodePool>>>,
    #[serde(rename = "notificationConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Notification configuration of the cluster."]
    pub notification_config: ::std::option::Option<::std::boxed::Box<NotificationConfig>>,
    #[serde(rename = "podSecurityPolicyConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for the PodSecurityPolicy feature."]
    pub pod_security_policy_config:
        ::std::option::Option<::std::boxed::Box<PodSecurityPolicyConfig>>,
    #[serde(rename = "privateCluster")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If this is a private cluster setup. Private clusters are clusters that, by default have no external IP addresses on the nodes and where nodes and the master communicate over private IP addresses. This field is deprecated, use private_cluster_config.enable_private_nodes instead."]
    pub private_cluster: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "privateClusterConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for private cluster."]
    pub private_cluster_config: ::std::option::Option<::std::boxed::Box<PrivateClusterConfig>>,
    #[serde(rename = "releaseChannel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Release channel configuration."]
    pub release_channel: ::std::option::Option<::std::boxed::Box<ReleaseChannel>>,
    #[serde(rename = "resourceLabels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource labels for the cluster to use to annotate any related Google Compute Engine resources."]
    pub resource_labels:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "resourceUsageExportConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for exporting resource usages. Resource usage export is disabled when this config unspecified."]
    pub resource_usage_export_config:
        ::std::option::Option<::std::boxed::Box<ResourceUsageExportConfig>>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] Server-defined URL for the resource."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "servicesIpv4Cidr")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] The IP address range of the Kubernetes services in this cluster, in [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `1.2.3.4/29`). Service addresses are typically put in the last `/16` from the container CIDR."]
    pub services_ipv4_cidr: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shieldedNodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shielded Nodes configuration."]
    pub shielded_nodes: ::std::option::Option<::std::boxed::Box<ShieldedNodes>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] The current status of this cluster."]
    pub status: ::std::option::Option<ClusterStatusEnum>,
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] Deprecated. Use conditions instead. Additional information about the current status of this cluster, if available."]
    pub status_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subnetwork")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the Google Compute Engine [subnetwork](https://cloud.google.com/compute/docs/subnetworks) to which the cluster is connected. On output this shows the subnetwork ID instead of the name."]
    pub subnetwork: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tpuConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for Cloud TPU support;"]
    pub tpu_config: ::std::option::Option<::std::boxed::Box<TpuConfig>>,
    #[serde(rename = "tpuIpv4CidrBlock")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] The IP address range of the Cloud TPUs in this cluster, in [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `1.2.3.4/29`)."]
    pub tpu_ipv4_cidr_block: ::std::option::Option<::std::string::String>,
    #[serde(rename = "verticalPodAutoscaling")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cluster-level Vertical Pod Autoscaling configuration."]
    pub vertical_pod_autoscaling: ::std::option::Option<::std::boxed::Box<VerticalPodAutoscaling>>,
    #[serde(rename = "workloadIdentityConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for the use of Kubernetes Service Accounts in GCP IAM policies."]
    pub workload_identity_config: ::std::option::Option<::std::boxed::Box<WorkloadIdentityConfig>>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field is deprecated, use location instead."]
    pub zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "[Output only] The current status of this cluster."]
pub enum ClusterStatusEnum {
    #[serde(rename = "STATUS_UNSPECIFIED")]
    #[doc = "Not set."]
    StatusUnspecified,
    #[serde(rename = "PROVISIONING")]
    #[doc = "The PROVISIONING state indicates the cluster is being created."]
    Provisioning,
    #[serde(rename = "RUNNING")]
    #[doc = "The RUNNING state indicates the cluster has been created and is fully usable."]
    Running,
    #[serde(rename = "RECONCILING")]
    #[doc = "The RECONCILING state indicates that some work is actively being done on the cluster, such as upgrading the master or node software. Details can be found in the `statusMessage` field."]
    Reconciling,
    #[serde(rename = "STOPPING")]
    #[doc = "The STOPPING state indicates the cluster is being deleted."]
    Stopping,
    #[serde(rename = "ERROR")]
    #[doc = "The ERROR state indicates the cluster may be unusable. Details can be found in the `statusMessage` field."]
    Error,
    #[serde(rename = "DEGRADED")]
    #[doc = "The DEGRADED state indicates the cluster requires user action to restore full functionality. Details can be found in the `statusMessage` field."]
    Degraded,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ClusterAutoscaling contains global, per-cluster information required by Cluster Autoscaler to automatically adjust the size of the cluster and create/delete node pools based on the current needs."]
pub struct ClusterAutoscaling {
    #[serde(rename = "autoprovisioningLocations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available) in which the NodePool's nodes can be created by NAP."]
    pub autoprovisioning_locations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "autoprovisioningNodePoolDefaults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "AutoprovisioningNodePoolDefaults contains defaults for a node pool created by NAP."]
    pub autoprovisioning_node_pool_defaults:
        ::std::option::Option<::std::boxed::Box<AutoprovisioningNodePoolDefaults>>,
    #[serde(rename = "autoscalingProfile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Defines autoscaling behaviour."]
    pub autoscaling_profile: ::std::option::Option<ClusterAutoscalingAutoscalingProfileEnum>,
    #[serde(rename = "enableNodeAutoprovisioning")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enables automatic node pool creation and deletion."]
    pub enable_node_autoprovisioning: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "resourceLimits")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Contains global constraints regarding minimum and maximum amount of resources in the cluster."]
    pub resource_limits: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ResourceLimit>>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Defines autoscaling behaviour."]
pub enum ClusterAutoscalingAutoscalingProfileEnum {
    #[serde(rename = "PROFILE_UNSPECIFIED")]
    #[doc = "No change to autoscaling configuration."]
    ProfileUnspecified,
    #[serde(rename = "OPTIMIZE_UTILIZATION")]
    #[doc = "Prioritize optimizing utilization of resources."]
    OptimizeUtilization,
    #[serde(rename = "BALANCED")]
    #[doc = "Use default (balanced) autoscaling configuration."]
    Balanced,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Telemetry integration for the cluster."]
pub struct ClusterTelemetry {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the integration."]
    pub _type: ::std::option::Option<ClusterTelemetryTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of the integration."]
pub enum ClusterTelemetryTypeEnum {
    #[serde(rename = "UNSPECIFIED")]
    #[doc = "Not set."]
    Unspecified,
    #[serde(rename = "DISABLED")]
    #[doc = "Monitoring integration is disabled."]
    Disabled,
    #[serde(rename = "ENABLED")]
    #[doc = "Monitoring integration is enabled."]
    Enabled,
    #[serde(rename = "SYSTEM_ONLY")]
    #[doc = "Only system components are monitored and logged."]
    SystemOnly,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ClusterUpdate describes an update to the cluster. Exactly one update can be applied to a cluster with each request, so at most one field can be provided."]
pub struct ClusterUpdate {
    #[serde(rename = "desiredAddonsConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configurations for the various addons available to run in the cluster."]
    pub desired_addons_config: ::std::option::Option<::std::boxed::Box<AddonsConfig>>,
    #[serde(rename = "desiredBinaryAuthorization")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The desired configuration options for the Binary Authorization feature."]
    pub desired_binary_authorization: ::std::option::Option<::std::boxed::Box<BinaryAuthorization>>,
    #[serde(rename = "desiredClusterAutoscaling")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cluster-level autoscaling configuration."]
    pub desired_cluster_autoscaling: ::std::option::Option<::std::boxed::Box<ClusterAutoscaling>>,
    #[serde(rename = "desiredClusterTelemetry")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The desired telemetry integration for the cluster."]
    pub desired_cluster_telemetry: ::std::option::Option<::std::boxed::Box<ClusterTelemetry>>,
    #[serde(rename = "desiredDatabaseEncryption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration of etcd encryption."]
    pub desired_database_encryption: ::std::option::Option<::std::boxed::Box<DatabaseEncryption>>,
    #[serde(rename = "desiredDatapathProvider")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The desired datapath provider for the cluster."]
    pub desired_datapath_provider: ::std::option::Option<ClusterUpdateDesiredDatapathProviderEnum>,
    #[serde(rename = "desiredDefaultSnatStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The desired status of whether to disable default sNAT for this cluster."]
    pub desired_default_snat_status: ::std::option::Option<::std::boxed::Box<DefaultSnatStatus>>,
    #[serde(rename = "desiredImageType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The desired image type for the node pool. NOTE: Set the \"desired_node_pool\" field as well."]
    pub desired_image_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "desiredIntraNodeVisibilityConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The desired config of Intra-node visibility."]
    pub desired_intra_node_visibility_config:
        ::std::option::Option<::std::boxed::Box<IntraNodeVisibilityConfig>>,
    #[serde(rename = "desiredLocations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The desired list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available) in which the cluster's nodes should be located. This list must always include the cluster's primary zone. Warning: changing cluster locations will update the locations of all node pools and will result in nodes being added and/or removed."]
    pub desired_locations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "desiredLoggingService")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The logging service the cluster should use to write logs. Currently available options: * `logging.googleapis.com/kubernetes` - The Cloud Logging service with a Kubernetes-native resource model * `logging.googleapis.com` - The legacy Cloud Logging service (no longer available as of GKE 1.15). * `none` - no logs will be exported from the cluster. If left as an empty string,`logging.googleapis.com/kubernetes` will be used for GKE 1.14+ or `logging.googleapis.com` for earlier versions."]
    pub desired_logging_service: ::std::option::Option<::std::string::String>,
    #[serde(rename = "desiredMaster")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for master components."]
    pub desired_master: ::std::option::Option<::std::boxed::Box<Master>>,
    #[serde(rename = "desiredMasterAuthorizedNetworksConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The desired configuration options for master authorized networks feature."]
    pub desired_master_authorized_networks_config:
        ::std::option::Option<::std::boxed::Box<MasterAuthorizedNetworksConfig>>,
    #[serde(rename = "desiredMasterVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Kubernetes version to change the master to. The only valid value is the latest supported version. Users may specify either explicit versions offered by Kubernetes Engine or version aliases, which have the following behavior: - \"latest\": picks the highest valid Kubernetes version - \"1.X\": picks the highest valid patch+gke.N patch in the 1.X version - \"1.X.Y\": picks the highest valid gke.N patch in the 1.X.Y version - \"1.X.Y-gke.N\": picks an explicit Kubernetes version - \"-\": picks the default Kubernetes version"]
    pub desired_master_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "desiredMonitoringService")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The monitoring service the cluster should use to write metrics. Currently available options: * \"monitoring.googleapis.com/kubernetes\" - The Cloud Monitoring service with a Kubernetes-native resource model * `monitoring.googleapis.com` - The legacy Cloud Monitoring service (no longer available as of GKE 1.15). * `none` - No metrics will be exported from the cluster. If left as an empty string,`monitoring.googleapis.com/kubernetes` will be used for GKE 1.14+ or `monitoring.googleapis.com` for earlier versions."]
    pub desired_monitoring_service: ::std::option::Option<::std::string::String>,
    #[serde(rename = "desiredNodePoolAutoscaling")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Autoscaler configuration for the node pool specified in desired_node_pool_id. If there is only one pool in the cluster and desired_node_pool_id is not provided then the change applies to that single node pool."]
    pub desired_node_pool_autoscaling:
        ::std::option::Option<::std::boxed::Box<NodePoolAutoscaling>>,
    #[serde(rename = "desiredNodePoolId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The node pool to be upgraded. This field is mandatory if \"desired_node_version\", \"desired_image_family\", \"desired_node_pool_autoscaling\", or \"desired_workload_metadata_config\" is specified and there is more than one node pool on the cluster."]
    pub desired_node_pool_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "desiredNodeVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Kubernetes version to change the nodes to (typically an upgrade). Users may specify either explicit versions offered by Kubernetes Engine or version aliases, which have the following behavior: - \"latest\": picks the highest valid Kubernetes version - \"1.X\": picks the highest valid patch+gke.N patch in the 1.X version - \"1.X.Y\": picks the highest valid gke.N patch in the 1.X.Y version - \"1.X.Y-gke.N\": picks an explicit Kubernetes version - \"-\": picks the Kubernetes master version"]
    pub desired_node_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "desiredNotificationConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The desired notification configuration."]
    pub desired_notification_config: ::std::option::Option<::std::boxed::Box<NotificationConfig>>,
    #[serde(rename = "desiredPodSecurityPolicyConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The desired configuration options for the PodSecurityPolicy feature."]
    pub desired_pod_security_policy_config:
        ::std::option::Option<::std::boxed::Box<PodSecurityPolicyConfig>>,
    #[serde(rename = "desiredPrivateClusterConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The desired private cluster configuration."]
    pub desired_private_cluster_config:
        ::std::option::Option<::std::boxed::Box<PrivateClusterConfig>>,
    #[serde(rename = "desiredPrivateIpv6GoogleAccess")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The desired state of IPv6 connectivity to Google Services."]
    pub desired_private_ipv6_google_access:
        ::std::option::Option<ClusterUpdateDesiredPrivateIpv6GoogleAccessEnum>,
    #[serde(rename = "desiredReleaseChannel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The desired release channel configuration."]
    pub desired_release_channel: ::std::option::Option<::std::boxed::Box<ReleaseChannel>>,
    #[serde(rename = "desiredResourceUsageExportConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The desired configuration for exporting resource usage."]
    pub desired_resource_usage_export_config:
        ::std::option::Option<::std::boxed::Box<ResourceUsageExportConfig>>,
    #[serde(rename = "desiredShieldedNodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for Shielded Nodes."]
    pub desired_shielded_nodes: ::std::option::Option<::std::boxed::Box<ShieldedNodes>>,
    #[serde(rename = "desiredTpuConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The desired Cloud TPU configuration."]
    pub desired_tpu_config: ::std::option::Option<::std::boxed::Box<TpuConfig>>,
    #[serde(rename = "desiredVerticalPodAutoscaling")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cluster-level Vertical Pod Autoscaling configuration."]
    pub desired_vertical_pod_autoscaling:
        ::std::option::Option<::std::boxed::Box<VerticalPodAutoscaling>>,
    #[serde(rename = "desiredWorkloadIdentityConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for Workload Identity."]
    pub desired_workload_identity_config:
        ::std::option::Option<::std::boxed::Box<WorkloadIdentityConfig>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The desired datapath provider for the cluster."]
pub enum ClusterUpdateDesiredDatapathProviderEnum {
    #[serde(rename = "DATAPATH_PROVIDER_UNSPECIFIED")]
    #[doc = "Default value."]
    DatapathProviderUnspecified,
    #[serde(rename = "LEGACY_DATAPATH")]
    #[doc = "Use the IPTables implementation based on kube-proxy."]
    LegacyDatapath,
    #[serde(rename = "ADVANCED_DATAPATH")]
    #[doc = "Use the eBPF based GKE Dataplane V2 with additional features. See the [GKE Dataplane V2 documentation](https://cloud.google.com/kubernetes-enginw/docs/how-to/dataplane-v2) for more."]
    AdvancedDatapath,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The desired state of IPv6 connectivity to Google Services."]
pub enum ClusterUpdateDesiredPrivateIpv6GoogleAccessEnum {
    #[serde(rename = "PRIVATE_IPV6_GOOGLE_ACCESS_UNSPECIFIED")]
    #[doc = "Default value. Same as DISABLED"]
    PrivateIpv6GoogleAccessUnspecified,
    #[serde(rename = "PRIVATE_IPV6_GOOGLE_ACCESS_DISABLED")]
    #[doc = "No private access to or from Google Services"]
    PrivateIpv6GoogleAccessDisabled,
    #[serde(rename = "PRIVATE_IPV6_GOOGLE_ACCESS_TO_GOOGLE")]
    #[doc = "Enables private IPv6 access to Google Services from GKE"]
    PrivateIpv6GoogleAccessToGoogle,
    #[serde(rename = "PRIVATE_IPV6_GOOGLE_ACCESS_BIDIRECTIONAL")]
    #[doc = "Enables priate IPv6 access to and from Google Services"]
    PrivateIpv6GoogleAccessBidirectional,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "CompleteIPRotationRequest moves the cluster master back into single-IP mode."]
pub struct CompleteIpRotationRequest {
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the cluster. This field has been deprecated and replaced by the name field."]
    pub cluster_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name (project, location, cluster id) of the cluster to complete IP rotation. Specified in the format `projects/*/locations/*/clusters/*`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The Google Developers Console [project ID or project number](https://developers.google.com/console/help/new/#projectnumber). This field has been deprecated and replaced by the name field."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
    pub zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ConfidentialNodes is configuration for the confidential nodes feature, which makes nodes run on confidential VMs."]
pub struct ConfidentialNodes {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether Confidential Nodes feature is enabled for all nodes in this cluster."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration options for the Config Connector add-on."]
pub struct ConfigConnectorConfig {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether Cloud Connector is enabled for this cluster."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Parameters for controlling consumption metering."]
pub struct ConsumptionMeteringConfig {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to enable consumption metering for this cluster. If enabled, a second BigQuery table will be created to hold resource consumption records."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "CreateClusterRequest creates a cluster."]
pub struct CreateClusterRequest {
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A [cluster resource](https://cloud.google.com/container-engine/reference/rest/v1beta1/projects.locations.clusters)"]
    pub cluster: ::std::option::Option<::std::boxed::Box<Cluster>>,
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The parent (project and location) where the cluster will be created. Specified in the format `projects/*/locations/*`."]
    pub parent: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the parent field."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field."]
    pub zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "CreateNodePoolRequest creates a node pool for a cluster."]
pub struct CreateNodePoolRequest {
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the cluster. This field has been deprecated and replaced by the parent field."]
    pub cluster_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nodePool")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The node pool to create."]
    pub node_pool: ::std::option::Option<::std::boxed::Box<NodePool>>,
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The parent (project, location, cluster id) where the node pool will be created. Specified in the format `projects/*/locations/*/clusters/*`."]
    pub parent: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The Google Developers Console [project ID or project number](https://developers.google.com/console/help/new/#projectnumber). This field has been deprecated and replaced by the parent field."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the parent field."]
    pub zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Time window specified for daily maintenance operations."]
pub struct DailyMaintenanceWindow {
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] Duration of the time window, automatically chosen to be smallest possible in the given scenario."]
    pub duration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time within the maintenance window to start the maintenance operations. It must be in format \"HH:MM\", where HH : [00-23] and MM : [00-59] GMT."]
    pub start_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration of etcd encryption."]
pub struct DatabaseEncryption {
    #[serde(rename = "keyName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of CloudKMS key to use for the encryption of secrets in etcd. Ex. projects/my-project/locations/global/keyRings/my-ring/cryptoKeys/my-key"]
    pub key_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Denotes the state of etcd encryption."]
    pub state: ::std::option::Option<DatabaseEncryptionStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Denotes the state of etcd encryption."]
pub enum DatabaseEncryptionStateEnum {
    #[serde(rename = "UNKNOWN")]
    #[doc = "Should never be set"]
    Unknown,
    #[serde(rename = "ENCRYPTED")]
    #[doc = "Secrets in etcd are encrypted."]
    Encrypted,
    #[serde(rename = "DECRYPTED")]
    #[doc = "Secrets in etcd are stored in plain text (at etcd level) - this is unrelated to Compute Engine level full disk encryption."]
    Decrypted,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "DefaultSnatStatus contains the desired state of whether default sNAT should be disabled on the cluster."]
pub struct DefaultSnatStatus {
    #[serde(rename = "disabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Disables cluster default sNAT rules."]
    pub disabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for NodeLocal DNSCache"]
pub struct DnsCacheConfig {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether NodeLocal DNSCache is enabled for this cluster."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "EphemeralStorageConfig contains configuration for the ephemeral storage filesystem."]
pub struct EphemeralStorageConfig {
    #[serde(rename = "localSsdCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of local SSDs to use to back ephemeral storage. Uses NVMe interfaces. Each local SSD is 375 GB in size. If zero, it means to disable using local SSDs as ephemeral storage."]
    pub local_ssd_count: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for the Compute Engine PD CSI driver."]
pub struct GcePersistentDiskCsiDriverConfig {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the Compute Engine PD CSI driver is enabled for this cluster."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "GetJSONWebKeysResponse is a valid JSON Web Key Set as specififed in rfc 7517"]
pub struct GetJsonWebKeysResponse {
    #[serde(rename = "cacheHeader")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OnePlatform automatically extracts this field and uses it to set the HTTP Cache-Control header."]
    pub cache_header: ::std::option::Option<::std::boxed::Box<HttpCacheControlResponseHeader>>,
    #[serde(rename = "keys")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The public component of the keys used by the cluster to sign token requests."]
    pub keys: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Jwk>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "GetOpenIDConfigResponse is an OIDC discovery document for the cluster. See the OpenID Connect Discovery 1.0 specification for details."]
pub struct GetOpenIdConfigResponse {
    #[serde(rename = "cacheHeader")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OnePlatform automatically extracts this field and uses it to set the HTTP Cache-Control header."]
    pub cache_header: ::std::option::Option<::std::boxed::Box<HttpCacheControlResponseHeader>>,
    #[serde(rename = "claims_supported")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Supported claims."]
    pub claims_supported: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "grant_types")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Supported grant types."]
    pub grant_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "id_token_signing_alg_values_supported")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "supported ID Token signing Algorithms."]
    pub id_token_signing_alg_values_supported:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "issuer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OIDC Issuer."]
    pub issuer: ::std::option::Option<::std::string::String>,
    #[serde(rename = "jwks_uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "JSON Web Key uri."]
    pub jwks_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "response_types_supported")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Supported response types."]
    pub response_types_supported: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "subject_types_supported")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Supported subject types."]
    pub subject_types_supported: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration options for the horizontal pod autoscaling feature, which increases or decreases the number of replica pods a replication controller has based on the resource usage of the existing pods."]
pub struct HorizontalPodAutoscaling {
    #[serde(rename = "disabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the Horizontal Pod Autoscaling feature is enabled in the cluster. When enabled, it ensures that metrics are collected into Stackdriver Monitoring."]
    pub disabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "RFC-2616: cache control support"]
pub struct HttpCacheControlResponseHeader {
    #[serde(rename = "age")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "14.6 response cache age, in seconds since the response is generated"]
    pub age: ::std::option::Option<::std::string::String>,
    #[serde(rename = "directive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "14.9 request and response directives"]
    pub directive: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expires")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "14.21 response cache expires, in RFC 1123 date format"]
    pub expires: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration options for the HTTP (L7) load balancing controller addon, which makes it easy to set up HTTP load balancers for services in a cluster."]
pub struct HttpLoadBalancing {
    #[serde(rename = "disabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the HTTP Load Balancing controller is enabled in the cluster. When enabled, it runs a small pod in the cluster that manages the load balancers."]
    pub disabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for controlling how IPs are allocated in the cluster."]
pub struct IpAllocationPolicy {
    #[serde(rename = "allowRouteOverlap")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, allow allocation of cluster CIDR ranges that overlap with certain kinds of network routes. By default we do not allow cluster CIDR ranges to intersect with any user declared routes. With allow_route_overlap == true, we allow overlapping with CIDR ranges that are larger than the cluster CIDR range. If this field is set to true, then cluster and services CIDRs must be fully-specified (e.g. `10.96.0.0/14`, but not `/14`), which means: 1) When `use_ip_aliases` is true, `cluster_ipv4_cidr_block` and `services_ipv4_cidr_block` must be fully-specified. 2) When `use_ip_aliases` is false, `cluster.cluster_ipv4_cidr` muse be fully-specified."]
    pub allow_route_overlap: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "clusterIpv4Cidr")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field is deprecated, use cluster_ipv4_cidr_block."]
    pub cluster_ipv4_cidr: ::std::option::Option<::std::string::String>,
    #[serde(rename = "clusterIpv4CidrBlock")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IP address range for the cluster pod IPs. If this field is set, then `cluster.cluster_ipv4_cidr` must be left blank. This field is only applicable when `use_ip_aliases` is true. Set to blank to have a range chosen with the default size. Set to /netmask (e.g. `/14`) to have a range chosen with a specific netmask. Set to a [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g. `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range to use."]
    pub cluster_ipv4_cidr_block: ::std::option::Option<::std::string::String>,
    #[serde(rename = "clusterSecondaryRangeName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the secondary range to be used for the cluster CIDR block. The secondary range will be used for pod IP addresses. This must be an existing secondary range associated with the cluster subnetwork. This field is only applicable with use_ip_aliases and create_subnetwork is false."]
    pub cluster_secondary_range_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createSubnetwork")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether a new subnetwork will be created automatically for the cluster. This field is only applicable when `use_ip_aliases` is true."]
    pub create_subnetwork: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "nodeIpv4Cidr")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field is deprecated, use node_ipv4_cidr_block."]
    pub node_ipv4_cidr: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nodeIpv4CidrBlock")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IP address range of the instance IPs in this cluster. This is applicable only if `create_subnetwork` is true. Set to blank to have a range chosen with the default size. Set to /netmask (e.g. `/14`) to have a range chosen with a specific netmask. Set to a [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g. `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range to use."]
    pub node_ipv4_cidr_block: ::std::option::Option<::std::string::String>,
    #[serde(rename = "servicesIpv4Cidr")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field is deprecated, use services_ipv4_cidr_block."]
    pub services_ipv4_cidr: ::std::option::Option<::std::string::String>,
    #[serde(rename = "servicesIpv4CidrBlock")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IP address range of the services IPs in this cluster. If blank, a range will be automatically chosen with the default size. This field is only applicable when `use_ip_aliases` is true. Set to blank to have a range chosen with the default size. Set to /netmask (e.g. `/14`) to have a range chosen with a specific netmask. Set to a [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g. `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range to use."]
    pub services_ipv4_cidr_block: ::std::option::Option<::std::string::String>,
    #[serde(rename = "servicesSecondaryRangeName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the secondary range to be used as for the services CIDR block. The secondary range will be used for service ClusterIPs. This must be an existing secondary range associated with the cluster subnetwork. This field is only applicable with use_ip_aliases and create_subnetwork is false."]
    pub services_secondary_range_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subnetworkName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A custom subnetwork name to be used if `create_subnetwork` is true. If this field is empty, then an automatic name will be chosen for the new subnetwork."]
    pub subnetwork_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tpuIpv4CidrBlock")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IP address range of the Cloud TPUs in this cluster. If unspecified, a range will be automatically chosen with the default size. This field is only applicable when `use_ip_aliases` is true. If unspecified, the range will use the default size. Set to /netmask (e.g. `/14`) to have a range chosen with a specific netmask. Set to a [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g. `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range to use. This field is deprecated, use cluster.tpu_config.ipv4_cidr_block instead."]
    pub tpu_ipv4_cidr_block: ::std::option::Option<::std::string::String>,
    #[serde(rename = "useIpAliases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether alias IPs will be used for pod IPs in the cluster. This is used in conjunction with use_routes. It cannot be true if use_routes is true. If both use_ip_aliases and use_routes are false, then the server picks the default IP allocation mode"]
    pub use_ip_aliases: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "useRoutes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether routes will be used for pod IPs in the cluster. This is used in conjunction with use_ip_aliases. It cannot be true if use_ip_aliases is true. If both use_ip_aliases and use_routes are false, then the server picks the default IP allocation mode"]
    pub use_routes: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "IntraNodeVisibilityConfig contains the desired config of the intra-node visibility on this cluster."]
pub struct IntraNodeVisibilityConfig {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enables intra node visibility for this cluster."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration options for Istio addon."]
pub struct IstioConfig {
    #[serde(rename = "auth")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The specified Istio auth mode, either none, or mutual TLS."]
    pub auth: ::std::option::Option<IstioConfigAuthEnum>,
    #[serde(rename = "disabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether Istio is enabled for this cluster."]
    pub disabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The specified Istio auth mode, either none, or mutual TLS."]
pub enum IstioConfigAuthEnum {
    #[serde(rename = "AUTH_NONE")]
    #[doc = "auth not enabled"]
    AuthNone,
    #[serde(rename = "AUTH_MUTUAL_TLS")]
    #[doc = "auth mutual TLS enabled"]
    AuthMutualTls,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Jwk is a JSON Web Key as specified in RFC 7517"]
pub struct Jwk {
    #[serde(rename = "alg")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Algorithm."]
    pub alg: ::std::option::Option<::std::string::String>,
    #[serde(rename = "crv")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Used for ECDSA keys."]
    pub crv: ::std::option::Option<::std::string::String>,
    #[serde(rename = "e")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Used for RSA keys."]
    pub e: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Key ID."]
    pub kid: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kty")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Key Type."]
    pub kty: ::std::option::Option<::std::string::String>,
    #[serde(rename = "n")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Used for RSA keys."]
    pub n: ::std::option::Option<::std::string::String>,
    #[serde(rename = "use")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Permitted uses for the public keys."]
    pub _use: ::std::option::Option<::std::string::String>,
    #[serde(rename = "x")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Used for ECDSA keys."]
    pub x: ::std::option::Option<::std::string::String>,
    #[serde(rename = "y")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Used for ECDSA keys."]
    pub y: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration options for the KALM addon."]
pub struct KalmConfig {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether KALM is enabled for this cluster."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for the Kubernetes Dashboard."]
pub struct KubernetesDashboard {
    #[serde(rename = "disabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the Kubernetes Dashboard is enabled for this cluster."]
    pub disabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for the legacy Attribute Based Access Control authorization mode."]
pub struct LegacyAbac {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the ABAC authorizer is enabled for this cluster. When enabled, identities in the system, including service accounts, nodes, and controllers, will have statically granted permissions beyond those provided by the RBAC configuration or IAM."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Parameters that can be configured on Linux nodes."]
pub struct LinuxNodeConfig {
    #[serde(rename = "sysctls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Linux kernel parameters to be applied to the nodes and all pods running on the nodes. The following parameters are supported. net.core.netdev_max_backlog net.core.rmem_max net.core.wmem_default net.core.wmem_max net.core.optmem_max net.core.somaxconn net.ipv4.tcp_rmem net.ipv4.tcp_wmem net.ipv4.tcp_tw_reuse"]
    pub sysctls: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ListClustersResponse is the result of ListClustersRequest."]
pub struct ListClustersResponse {
    #[serde(rename = "clusters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of clusters in the project in the specified zone, or across all ones."]
    pub clusters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Cluster>>>,
    #[serde(rename = "missingZones")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If any zones are listed here, the list of clusters returned may be missing those zones."]
    pub missing_zones: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ListLocationsResponse returns the list of all GKE locations and their recommendation state."]
pub struct ListLocationsResponse {
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A full list of GKE locations."]
    pub locations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Location>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only return ListLocationsResponse that occur after the page_token. This value should be populated from the ListLocationsResponse.next_page_token if that response token was set (which happens when listing more Locations than fit in a single ListLocationsResponse)."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ListNodePoolsResponse is the result of ListNodePoolsRequest."]
pub struct ListNodePoolsResponse {
    #[serde(rename = "nodePools")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of node pools for a cluster."]
    pub node_pools: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NodePool>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ListOperationsResponse is the result of ListOperationsRequest."]
pub struct ListOperationsResponse {
    #[serde(rename = "missingZones")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If any zones are listed here, the list of operations returned may be missing the operations from those zones."]
    pub missing_zones: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of operations in the project in the specified zone."]
    pub operations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Operation>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ListUsableSubnetworksResponse is the response of ListUsableSubnetworksRequest."]
pub struct ListUsableSubnetworksResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This token allows you to get the next page of results for list requests. If the number of results is larger than `page_size`, use the `next_page_token` as a value for the query parameter `page_token` in the next request. The value will become empty when there are no more pages."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subnetworks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of usable subnetworks in the specified network project."]
    pub subnetworks: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UsableSubnetwork>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Location returns the location name, and if the location is recommended for GKE cluster scheduling."]
pub struct Location {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Contains the name of the resource requested. Specified in the format `projects/*/locations/*`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "recommended")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the location is recomended for GKE cluster scheduling."]
    pub recommended: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Contains the type of location this Location is for. Regional or Zonal."]
    pub _type: ::std::option::Option<LocationTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Contains the type of location this Location is for. Regional or Zonal."]
pub enum LocationTypeEnum {
    #[serde(rename = "LOCATION_TYPE_UNSPECIFIED")]
    #[doc = "LOCATION_TYPE_UNSPECIFIED means the location type was not determined."]
    LocationTypeUnspecified,
    #[serde(rename = "ZONE")]
    #[doc = "A GKE Location where Zonal clusters can be created."]
    Zone,
    #[serde(rename = "REGION")]
    #[doc = "A GKE Location where Regional clusters can be created."]
    Region,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "MaintenancePolicy defines the maintenance policy to be used for the cluster."]
pub struct MaintenancePolicy {
    #[serde(rename = "resourceVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A hash identifying the version of this policy, so that updates to fields of the policy won't accidentally undo intermediate changes (and so that users of the API unaware of some fields won't accidentally remove other fields). Make a `get()` request to the cluster to get the current resource version and include it with requests to set the policy."]
    pub resource_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "window")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the maintenance window in which maintenance may be performed."]
    pub window: ::std::option::Option<::std::boxed::Box<MaintenanceWindow>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "MaintenanceWindow defines the maintenance window to be used for the cluster."]
pub struct MaintenanceWindow {
    #[serde(rename = "dailyMaintenanceWindow")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DailyMaintenanceWindow specifies a daily maintenance operation window."]
    pub daily_maintenance_window: ::std::option::Option<::std::boxed::Box<DailyMaintenanceWindow>>,
    #[serde(rename = "maintenanceExclusions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Exceptions to maintenance window. Non-emergency maintenance should not occur in these windows."]
    pub maintenance_exclusions:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<TimeWindow>>>,
    #[serde(rename = "recurringWindow")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "RecurringWindow specifies some number of recurring time periods for maintenance to occur. The time windows may be overlapping. If no maintenance windows are set, maintenance can occur at any time."]
    pub recurring_window: ::std::option::Option<::std::boxed::Box<RecurringTimeWindow>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Master is the configuration for components on master."]
pub struct Master {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The authentication information for accessing the master endpoint. Authentication can be done using HTTP basic auth or using client certificates."]
pub struct MasterAuth {
    #[serde(rename = "clientCertificate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] Base64-encoded public certificate used by clients to authenticate to the cluster endpoint."]
    pub client_certificate: ::std::option::Option<::std::string::String>,
    #[serde(rename = "clientCertificateConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for client certificate authentication on the cluster. For clusters before v1.12, if no configuration is specified, a client certificate is issued."]
    pub client_certificate_config:
        ::std::option::Option<::std::boxed::Box<ClientCertificateConfig>>,
    #[serde(rename = "clientKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] Base64-encoded private key used by clients to authenticate to the cluster endpoint."]
    pub client_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "clusterCaCertificate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cluster_ca_certificate: ::std::option::Option<::std::string::String>,
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The password to use for HTTP basic authentication to the master endpoint. Because the master endpoint is open to the Internet, you should create a strong password. If a password is provided for cluster creation, username must be non-empty. Warning: basic authentication is deprecated, and will be removed in GKE control plane versions 1.19 and newer. For a list of recommended authentication methods, see: https://cloud.google.com/kubernetes-engine/docs/how-to/api-server-authentication"]
    pub password: ::std::option::Option<::std::string::String>,
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The username to use for HTTP basic authentication to the master endpoint. For clusters v1.6.0 and later, basic authentication can be disabled by leaving username unspecified (or setting it to the empty string). Warning: basic authentication is deprecated, and will be removed in GKE control plane versions 1.19 and newer. For a list of recommended authentication methods, see: https://cloud.google.com/kubernetes-engine/docs/how-to/api-server-authentication"]
    pub username: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration options for the master authorized networks feature. Enabled master authorized networks will disallow all external traffic to access Kubernetes master through HTTPS except traffic from the given CIDR blocks, Google Compute Engine Public IPs and Google Prod IPs."]
pub struct MasterAuthorizedNetworksConfig {
    #[serde(rename = "cidrBlocks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "cidr_blocks define up to 10 external networks that could access Kubernetes master through HTTPS."]
    pub cidr_blocks: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CidrBlock>>>,
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not master authorized networks is enabled."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Constraints applied to pods."]
pub struct MaxPodsConstraint {
    #[serde(rename = "maxPodsPerNode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Constraint enforced on the max num of pods per node."]
    pub max_pods_per_node: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Progress metric is (string, int|float|string) pair."]
pub struct Metric {
    #[serde(rename = "doubleValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For metrics with floating point value."]
    pub double_value: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "intValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For metrics with integer value."]
    pub int_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Metric name, e.g., \"nodes total\", \"percent done\"."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "stringValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For metrics with custom values (ratios, visual progress, etc.)."]
    pub string_value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "NetworkConfig reports the relative names of network & subnetwork."]
pub struct NetworkConfig {
    #[serde(rename = "datapathProvider")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The desired datapath provider for this cluster. By default, uses the IPTables-based kube-proxy implementation."]
    pub datapath_provider: ::std::option::Option<NetworkConfigDatapathProviderEnum>,
    #[serde(rename = "defaultSnatStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the cluster disables default in-node sNAT rules. In-node sNAT rules will be disabled when default_snat_status is disabled. When disabled is set to false, default IP masquerade rules will be applied to the nodes to prevent sNAT on cluster internal traffic."]
    pub default_snat_status: ::std::option::Option<::std::boxed::Box<DefaultSnatStatus>>,
    #[serde(rename = "enableIntraNodeVisibility")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether Intra-node visibility is enabled for this cluster. This makes same node pod to pod traffic visible for VPC network."]
    pub enable_intra_node_visibility: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "network")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The relative name of the Google Compute Engine network(https://cloud.google.com/compute/docs/networks-and-firewalls#networks) to which the cluster is connected. Example: projects/my-project/global/networks/my-network"]
    pub network: ::std::option::Option<::std::string::String>,
    #[serde(rename = "privateIpv6GoogleAccess")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The desired state of IPv6 connectivity to Google Services. By default, no private IPv6 access to or from Google Services (all access will be via IPv4)"]
    pub private_ipv6_google_access: ::std::option::Option<NetworkConfigPrivateIpv6GoogleAccessEnum>,
    #[serde(rename = "subnetwork")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The relative name of the Google Compute Engine [subnetwork](https://cloud.google.com/compute/docs/vpc) to which the cluster is connected. Example: projects/my-project/regions/us-central1/subnetworks/my-subnet"]
    pub subnetwork: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The desired datapath provider for this cluster. By default, uses the IPTables-based kube-proxy implementation."]
pub enum NetworkConfigDatapathProviderEnum {
    #[serde(rename = "DATAPATH_PROVIDER_UNSPECIFIED")]
    #[doc = "Default value."]
    DatapathProviderUnspecified,
    #[serde(rename = "LEGACY_DATAPATH")]
    #[doc = "Use the IPTables implementation based on kube-proxy."]
    LegacyDatapath,
    #[serde(rename = "ADVANCED_DATAPATH")]
    #[doc = "Use the eBPF based GKE Dataplane V2 with additional features. See the [GKE Dataplane V2 documentation](https://cloud.google.com/kubernetes-enginw/docs/how-to/dataplane-v2) for more."]
    AdvancedDatapath,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The desired state of IPv6 connectivity to Google Services. By default, no private IPv6 access to or from Google Services (all access will be via IPv4)"]
pub enum NetworkConfigPrivateIpv6GoogleAccessEnum {
    #[serde(rename = "PRIVATE_IPV6_GOOGLE_ACCESS_UNSPECIFIED")]
    #[doc = "Default value. Same as DISABLED"]
    PrivateIpv6GoogleAccessUnspecified,
    #[serde(rename = "PRIVATE_IPV6_GOOGLE_ACCESS_DISABLED")]
    #[doc = "No private access to or from Google Services"]
    PrivateIpv6GoogleAccessDisabled,
    #[serde(rename = "PRIVATE_IPV6_GOOGLE_ACCESS_TO_GOOGLE")]
    #[doc = "Enables private IPv6 access to Google Services from GKE"]
    PrivateIpv6GoogleAccessToGoogle,
    #[serde(rename = "PRIVATE_IPV6_GOOGLE_ACCESS_BIDIRECTIONAL")]
    #[doc = "Enables priate IPv6 access to and from Google Services"]
    PrivateIpv6GoogleAccessBidirectional,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration options for the NetworkPolicy feature. https://kubernetes.io/docs/concepts/services-networking/networkpolicies/"]
pub struct NetworkPolicy {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether network policy is enabled on the cluster."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "provider")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The selected network policy provider."]
    pub provider: ::std::option::Option<NetworkPolicyProviderEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The selected network policy provider."]
pub enum NetworkPolicyProviderEnum {
    #[serde(rename = "PROVIDER_UNSPECIFIED")]
    #[doc = "Not set"]
    ProviderUnspecified,
    #[serde(rename = "CALICO")]
    #[doc = "Tigera (Calico Felix)."]
    Calico,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for NetworkPolicy. This only tracks whether the addon is enabled or not on the Master, it does not track whether network policy is enabled for the nodes."]
pub struct NetworkPolicyConfig {
    #[serde(rename = "disabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether NetworkPolicy is enabled for this cluster."]
    pub disabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Parameters that describe the nodes in a cluster."]
pub struct NodeConfig {
    #[serde(rename = "accelerators")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of hardware accelerators to be attached to each node. See https://cloud.google.com/compute/docs/gpus for more information about support for GPUs."]
    pub accelerators: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AcceleratorConfig>>>,
    #[serde(rename = "bootDiskKmsKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = " The Customer Managed Encryption Key used to encrypt the boot disk attached to each node in the node pool. This should be of the form projects/[KEY_PROJECT_ID]/locations/[LOCATION]/keyRings/[RING_NAME]/cryptoKeys/[KEY_NAME]. For more information about protecting resources with Cloud KMS Keys please see: https://cloud.google.com/compute/docs/disks/customer-managed-encryption"]
    pub boot_disk_kms_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "diskSizeGb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Size of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB. If unspecified, the default disk size is 100GB."]
    pub disk_size_gb: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "diskType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the disk attached to each node (e.g. 'pd-standard', 'pd-ssd' or 'pd-balanced') If unspecified, the default disk type is 'pd-standard'"]
    pub disk_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ephemeralStorageConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parameters for the ephemeral storage filesystem. If unspecified, ephemeral storage is backed by the boot disk."]
    pub ephemeral_storage_config: ::std::option::Option<::std::boxed::Box<EphemeralStorageConfig>>,
    #[serde(rename = "imageType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The image type to use for this node. Note that for a given image type, the latest version of it will be used."]
    pub image_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kubeletConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Node kubelet configs."]
    pub kubelet_config: ::std::option::Option<::std::boxed::Box<NodeKubeletConfig>>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The map of Kubernetes labels (key/value pairs) to be applied to each node. These will added in addition to any default label(s) that Kubernetes may apply to the node. In case of conflict in label keys, the applied set may differ depending on the Kubernetes version -- it's best to assume the behavior is undefined and conflicts should be avoided. For more information, including usage and the valid values, see: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/"]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "linuxNodeConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parameters that can be configured on Linux nodes."]
    pub linux_node_config: ::std::option::Option<::std::boxed::Box<LinuxNodeConfig>>,
    #[serde(rename = "localSsdCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of local SSD disks to be attached to the node. The limit for this value is dependent upon the maximum number of disks available on a machine per zone. See: https://cloud.google.com/compute/docs/disks/local-ssd for more information."]
    pub local_ssd_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "machineType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of a Google Compute Engine [machine type](https://cloud.google.com/compute/docs/machine-types). If unspecified, the default machine type is `e2-medium`."]
    pub machine_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The metadata key/value pairs assigned to instances in the cluster. Keys must conform to the regexp `[a-zA-Z0-9-_]+` and be less than 128 bytes in length. These are reflected as part of a URL in the metadata server. Additionally, to avoid ambiguity, keys must not conflict with any other metadata keys for the project or be one of the reserved keys: - \"cluster-location\" - \"cluster-name\" - \"cluster-uid\" - \"configure-sh\" - \"containerd-configure-sh\" - \"enable-oslogin\" - \"gci-ensure-gke-docker\" - \"gci-metrics-enabled\" - \"gci-update-strategy\" - \"instance-template\" - \"kube-env\" - \"startup-script\" - \"user-data\" - \"disable-address-manager\" - \"windows-startup-script-ps1\" - \"common-psm1\" - \"k8s-node-setup-psm1\" - \"install-ssh-psm1\" - \"user-profile-psm1\" The following keys are reserved for Windows nodes: - \"serial-port-logging-enable\" Values are free-form strings, and only have meaning as interpreted by the image running in the instance. The only restriction placed on them is that each value's size must be less than or equal to 32 KB. The total size of all keys and values must be less than 512 KB."]
    pub metadata:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "minCpuPlatform")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Minimum CPU platform to be used by this instance. The instance may be scheduled on the specified or newer CPU platform. Applicable values are the friendly names of CPU platforms, such as `minCpuPlatform: \"Intel Haswell\"` or `minCpuPlatform: \"Intel Sandy Bridge\"`. For more information, read [how to specify min CPU platform](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform)"]
    pub min_cpu_platform: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nodeGroup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Setting this field will assign instances of this pool to run on the specified node group. This is useful for running workloads on [sole tenant nodes](https://cloud.google.com/compute/docs/nodes/sole-tenant-nodes)."]
    pub node_group: ::std::option::Option<::std::string::String>,
    #[serde(rename = "oauthScopes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of Google API scopes to be made available on all of the node VMs under the \"default\" service account. The following scopes are recommended, but not required, and by default are not included: * `https://www.googleapis.com/auth/compute` is required for mounting persistent storage on your nodes. * `https://www.googleapis.com/auth/devstorage.read_only` is required for communicating with **gcr.io** (the [Google Container Registry](https://cloud.google.com/container-registry/)). If unspecified, no scopes are added, unless Cloud Logging or Cloud Monitoring are enabled, in which case their required scopes will be added."]
    pub oauth_scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "preemptible")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the nodes are created as preemptible VM instances. See: https://cloud.google.com/compute/docs/instances/preemptible for more inforamtion about preemptible VM instances."]
    pub preemptible: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "reservationAffinity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The optional reservation affinity. Setting this field will apply the specified [Zonal Compute Reservation](https://cloud.google.com/compute/docs/instances/reserving-zonal-resources) to this node pool."]
    pub reservation_affinity: ::std::option::Option<::std::boxed::Box<ReservationAffinity>>,
    #[serde(rename = "sandboxConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sandbox configuration for this node."]
    pub sandbox_config: ::std::option::Option<::std::boxed::Box<SandboxConfig>>,
    #[serde(rename = "serviceAccount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Google Cloud Platform Service Account to be used by the node VMs. Specify the email address of the Service Account; otherwise, if no Service Account is specified, the \"default\" service account is used."]
    pub service_account: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shieldedInstanceConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shielded Instance options."]
    pub shielded_instance_config: ::std::option::Option<::std::boxed::Box<ShieldedInstanceConfig>>,
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of instance tags applied to all nodes. Tags are used to identify valid sources or targets for network firewalls and are specified by the client during cluster or node pool creation. Each tag within the list must comply with RFC1035."]
    pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "taints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of kubernetes taints to be applied to each node. For more information, including usage and the valid values, see: https://kubernetes.io/docs/concepts/configuration/taint-and-toleration/"]
    pub taints: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NodeTaint>>>,
    #[serde(rename = "workloadMetadataConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The workload metadata configuration for this node."]
    pub workload_metadata_config: ::std::option::Option<::std::boxed::Box<WorkloadMetadataConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Node kubelet configs."]
pub struct NodeKubeletConfig {
    #[serde(rename = "cpuCfsQuota")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enable CPU CFS quota enforcement for containers that specify CPU limits. This option is enabled by default which makes kubelet use CFS quota (https://www.kernel.org/doc/Documentation/scheduler/sched-bwc.txt) to enforce container CPU limits. Otherwise, CPU limits will not be enforced at all. Disable this option to mitigate CPU throttling problems while still having your pods to be in Guaranteed QoS class by specifying the CPU limits. The default value is 'true' if unspecified."]
    pub cpu_cfs_quota: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "cpuCfsQuotaPeriod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set the CPU CFS quota period value 'cpu.cfs_period_us'. The string must be a sequence of decimal numbers, each with optional fraction and a unit suffix, such as \"300ms\". Valid time units are \"ns\", \"us\" (or \"µs\"), \"ms\", \"s\", \"m\", \"h\". The value must be a positive duration."]
    pub cpu_cfs_quota_period: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cpuManagerPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Control the CPU management policy on the node. See https://kubernetes.io/docs/tasks/administer-cluster/cpu-management-policies/ The following values are allowed. - \"none\": the default, which represents the existing scheduling behavior. - \"static\": allows pods with certain resource characteristics to be granted increased CPU affinity and exclusivity on the node. The default value is 'none' if unspecified."]
    pub cpu_manager_policy: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "NodeManagement defines the set of node management services turned on for the node pool."]
pub struct NodeManagement {
    #[serde(rename = "autoRepair")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the nodes will be automatically repaired."]
    pub auto_repair: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "autoUpgrade")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the nodes will be automatically upgraded."]
    pub auto_upgrade: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "upgradeOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the Auto Upgrade knobs for the node pool."]
    pub upgrade_options: ::std::option::Option<::std::boxed::Box<AutoUpgradeOptions>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "NodePool contains the name and configuration for a cluster's node pool. Node pools are a set of nodes (i.e. VM's), with a common configuration and specification, under the control of the cluster master. They may have a set of Kubernetes labels applied to them, which may be used to reference them during pod scheduling. They may also be resized up or down, to accommodate the workload."]
pub struct NodePool {
    #[serde(rename = "autoscaling")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Autoscaler configuration for this NodePool. Autoscaler is enabled only if a valid configuration is present."]
    pub autoscaling: ::std::option::Option<::std::boxed::Box<NodePoolAutoscaling>>,
    #[serde(rename = "conditions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Which conditions caused the current node pool state."]
    pub conditions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StatusCondition>>>,
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The node configuration of the pool."]
    pub config: ::std::option::Option<::std::boxed::Box<NodeConfig>>,
    #[serde(rename = "initialNodeCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The initial node count for the pool. You must ensure that your Compute Engine [resource quota](https://cloud.google.com/compute/quotas) is sufficient for this number of instances. You must also have available firewall and routes quota."]
    pub initial_node_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "instanceGroupUrls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] The resource URLs of the [managed instance groups](https://cloud.google.com/compute/docs/instance-groups/creating-groups-of-managed-instances) associated with this node pool."]
    pub instance_group_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available) in which the NodePool's nodes should be located. If this value is unspecified during node pool creation, the [Cluster.Locations](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters#Cluster.FIELDS.locations) value will be used, instead. Warning: changing node pool locations will result in nodes being added and/or removed."]
    pub locations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "management")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "NodeManagement configuration for this NodePool."]
    pub management: ::std::option::Option<::std::boxed::Box<NodeManagement>>,
    #[serde(rename = "maxPodsConstraint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The constraint on the maximum number of pods that can be run simultaneously on a node in the node pool."]
    pub max_pods_constraint: ::std::option::Option<::std::boxed::Box<MaxPodsConstraint>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the node pool."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "podIpv4CidrSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] The pod CIDR block size per node in this node pool."]
    pub pod_ipv4_cidr_size: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] Server-defined URL for the resource."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] The status of the nodes in this pool instance."]
    pub status: ::std::option::Option<NodePoolStatusEnum>,
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] Deprecated. Use conditions instead. Additional information about the current status of this node pool instance, if available."]
    pub status_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "upgradeSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Upgrade settings control disruption and speed of the upgrade."]
    pub upgrade_settings: ::std::option::Option<::std::boxed::Box<UpgradeSettings>>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version of the Kubernetes of this node."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "[Output only] The status of the nodes in this pool instance."]
pub enum NodePoolStatusEnum {
    #[serde(rename = "STATUS_UNSPECIFIED")]
    #[doc = "Not set."]
    StatusUnspecified,
    #[serde(rename = "PROVISIONING")]
    #[doc = "The PROVISIONING state indicates the node pool is being created."]
    Provisioning,
    #[serde(rename = "RUNNING")]
    #[doc = "The RUNNING state indicates the node pool has been created and is fully usable."]
    Running,
    #[serde(rename = "RUNNING_WITH_ERROR")]
    #[doc = "The RUNNING_WITH_ERROR state indicates the node pool has been created and is partially usable. Some error state has occurred and some functionality may be impaired. Customer may need to reissue a request or trigger a new update."]
    RunningWithError,
    #[serde(rename = "RECONCILING")]
    #[doc = "The RECONCILING state indicates that some work is actively being done on the node pool, such as upgrading node software. Details can be found in the `statusMessage` field."]
    Reconciling,
    #[serde(rename = "STOPPING")]
    #[doc = "The STOPPING state indicates the node pool is being deleted."]
    Stopping,
    #[serde(rename = "ERROR")]
    #[doc = "The ERROR state indicates the node pool may be unusable. Details can be found in the `statusMessage` field."]
    Error,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "NodePoolAutoscaling contains information required by cluster autoscaler to adjust the size of the node pool to the current cluster usage."]
pub struct NodePoolAutoscaling {
    #[serde(rename = "autoprovisioned")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Can this node pool be deleted automatically."]
    pub autoprovisioned: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Is autoscaling enabled for this node pool."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "maxNodeCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum number of nodes in the NodePool. Must be >= min_node_count. There has to enough quota to scale up the cluster."]
    pub max_node_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "minNodeCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Minimum number of nodes in the NodePool. Must be >= 1 and <= max_node_count."]
    pub min_node_count: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Kubernetes taint is comprised of three fields: key, value, and effect. Effect can only be one of three types: NoSchedule, PreferNoSchedule or NoExecute. See [here](https://kubernetes.io/docs/concepts/configuration/taint-and-toleration) for more information, including usage and the valid values."]
pub struct NodeTaint {
    #[serde(rename = "effect")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Effect for taint."]
    pub effect: ::std::option::Option<NodeTaintEffectEnum>,
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Key for taint."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Value for taint."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Effect for taint."]
pub enum NodeTaintEffectEnum {
    #[serde(rename = "EFFECT_UNSPECIFIED")]
    #[doc = "Not set"]
    EffectUnspecified,
    #[serde(rename = "NO_SCHEDULE")]
    #[doc = "NoSchedule"]
    NoSchedule,
    #[serde(rename = "PREFER_NO_SCHEDULE")]
    #[doc = "PreferNoSchedule"]
    PreferNoSchedule,
    #[serde(rename = "NO_EXECUTE")]
    #[doc = "NoExecute"]
    NoExecute,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "NotificationConfig is the configuration of notifications."]
pub struct NotificationConfig {
    #[serde(rename = "pubsub")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Notification config for Pub/Sub."]
    pub pubsub: ::std::option::Option<::std::boxed::Box<PubSub>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This operation resource represents operations that may have happened or are happening on the cluster. All fields are output only."]
pub struct Operation {
    #[serde(rename = "clusterConditions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Which conditions caused the current cluster state. Deprecated. Use field error instead."]
    pub cluster_conditions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StatusCondition>>>,
    #[serde(rename = "detail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detailed operation progress, if available."]
    pub detail: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] The time the operation completed, in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The error result of the operation in case of failure."]
    pub error: ::std::option::Option<::std::boxed::Box<Status>>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) or [region](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) in which the cluster resides."]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The server-assigned ID for the operation."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nodepoolConditions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Which conditions caused the current node pool state. Deprecated. Use field error instead."]
    pub nodepool_conditions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<StatusCondition>>>,
    #[serde(rename = "operationType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The operation type."]
    pub operation_type: ::std::option::Option<OperationOperationTypeEnum>,
    #[serde(rename = "progress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. [Output only] Progress information for an operation."]
    pub progress: ::std::option::Option<::std::boxed::Box<OperationProgress>>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Server-defined URL for the resource."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output only] The time the operation started, in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current status of the operation."]
    pub status: ::std::option::Option<OperationStatusEnum>,
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. If an error has occurred, a textual description of the error. Deprecated. Use field error instead."]
    pub status_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Server-defined URL for the target of the operation."]
    pub target_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the operation is taking place. This field is deprecated, use location instead."]
    pub zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The operation type."]
pub enum OperationOperationTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "Not set."]
    TypeUnspecified,
    #[serde(rename = "CREATE_CLUSTER")]
    #[doc = "Cluster create."]
    CreateCluster,
    #[serde(rename = "DELETE_CLUSTER")]
    #[doc = "Cluster delete."]
    DeleteCluster,
    #[serde(rename = "UPGRADE_MASTER")]
    #[doc = "A master upgrade."]
    UpgradeMaster,
    #[serde(rename = "UPGRADE_NODES")]
    #[doc = "A node upgrade."]
    UpgradeNodes,
    #[serde(rename = "REPAIR_CLUSTER")]
    #[doc = "Cluster repair."]
    RepairCluster,
    #[serde(rename = "UPDATE_CLUSTER")]
    #[doc = "Cluster update."]
    UpdateCluster,
    #[serde(rename = "CREATE_NODE_POOL")]
    #[doc = "Node pool create."]
    CreateNodePool,
    #[serde(rename = "DELETE_NODE_POOL")]
    #[doc = "Node pool delete."]
    DeleteNodePool,
    #[serde(rename = "SET_NODE_POOL_MANAGEMENT")]
    #[doc = "Set node pool management."]
    SetNodePoolManagement,
    #[serde(rename = "AUTO_REPAIR_NODES")]
    #[doc = "Automatic node pool repair."]
    AutoRepairNodes,
    #[serde(rename = "AUTO_UPGRADE_NODES")]
    #[doc = "Automatic node upgrade."]
    AutoUpgradeNodes,
    #[serde(rename = "SET_LABELS")]
    #[doc = "Set labels."]
    SetLabels,
    #[serde(rename = "SET_MASTER_AUTH")]
    #[doc = "Set/generate master auth materials"]
    SetMasterAuth,
    #[serde(rename = "SET_NODE_POOL_SIZE")]
    #[doc = "Set node pool size."]
    SetNodePoolSize,
    #[serde(rename = "SET_NETWORK_POLICY")]
    #[doc = "Updates network policy for a cluster."]
    SetNetworkPolicy,
    #[serde(rename = "SET_MAINTENANCE_POLICY")]
    #[doc = "Set the maintenance policy."]
    SetMaintenancePolicy,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The current status of the operation."]
pub enum OperationStatusEnum {
    #[serde(rename = "STATUS_UNSPECIFIED")]
    #[doc = "Not set."]
    StatusUnspecified,
    #[serde(rename = "PENDING")]
    #[doc = "The operation has been created."]
    Pending,
    #[serde(rename = "RUNNING")]
    #[doc = "The operation is currently running."]
    Running,
    #[serde(rename = "DONE")]
    #[doc = "The operation is done, either cancelled or completed."]
    Done,
    #[serde(rename = "ABORTING")]
    #[doc = "The operation is aborting."]
    Aborting,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about operation (or operation stage) progress."]
pub struct OperationProgress {
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Progress metric bundle, for example: metrics: [{name: \"nodes done\", int_value: 15}, {name: \"nodes total\", int_value: 32}] or metrics: [{name: \"progress\", double_value: 0.56}, {name: \"progress scale\", double_value: 1.0}]"]
    pub metrics: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Metric>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A non-parameterized string describing an operation stage. Unset for single-stage operations."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "stages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Substages of an operation or a stage."]
    pub stages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OperationProgress>>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of an operation stage. Unset for single-stage operations."]
    pub status: ::std::option::Option<OperationProgressStatusEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Status of an operation stage. Unset for single-stage operations."]
pub enum OperationProgressStatusEnum {
    #[serde(rename = "STATUS_UNSPECIFIED")]
    #[doc = "Not set."]
    StatusUnspecified,
    #[serde(rename = "PENDING")]
    #[doc = "The operation has been created."]
    Pending,
    #[serde(rename = "RUNNING")]
    #[doc = "The operation is currently running."]
    Running,
    #[serde(rename = "DONE")]
    #[doc = "The operation is done, either cancelled or completed."]
    Done,
    #[serde(rename = "ABORTING")]
    #[doc = "The operation is aborting."]
    Aborting,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for the PodSecurityPolicy feature."]
pub struct PodSecurityPolicyConfig {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enable the PodSecurityPolicy controller for this cluster. If enabled, pods must be valid under a PodSecurityPolicy to be created."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration options for private clusters."]
pub struct PrivateClusterConfig {
    #[serde(rename = "enablePrivateEndpoint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the master's internal IP address is used as the cluster endpoint."]
    pub enable_private_endpoint: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "enablePrivateNodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether nodes have internal IP addresses only. If enabled, all nodes are given only RFC 1918 private addresses and communicate with the master via private networking."]
    pub enable_private_nodes: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "masterGlobalAccessConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Controls master global access settings."]
    pub master_global_access_config:
        ::std::option::Option<::std::boxed::Box<PrivateClusterMasterGlobalAccessConfig>>,
    #[serde(rename = "masterIpv4CidrBlock")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IP range in CIDR notation to use for the hosted master network. This range will be used for assigning internal IP addresses to the master or set of masters, as well as the ILB VIP. This range must not overlap with any other ranges in use within the cluster's network."]
    pub master_ipv4_cidr_block: ::std::option::Option<::std::string::String>,
    #[serde(rename = "peeringName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The peering name in the customer VPC used by this cluster."]
    pub peering_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "privateEndpoint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The internal IP address of this cluster's master endpoint."]
    pub private_endpoint: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publicEndpoint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The external IP address of this cluster's master endpoint."]
    pub public_endpoint: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for controlling master global access settings."]
pub struct PrivateClusterMasterGlobalAccessConfig {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whenever master is accessible globally or not."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Pub/Sub specific notification config."]
pub struct PubSub {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enable notifications for Pub/Sub."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "topic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The desired Pub/Sub topic to which notifications will be sent by GKE. Format is `projects/{project}/topics/{topic}`."]
    pub topic: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an arbitrary window of time that recurs."]
pub struct RecurringTimeWindow {
    #[serde(rename = "recurrence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An RRULE (https://tools.ietf.org/html/rfc5545#section-3.8.5.3) for how this window reccurs. They go on for the span of time between the start and end time. For example, to have something repeat every weekday, you'd use: `FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR` To repeat some window daily (equivalent to the DailyMaintenanceWindow): `FREQ=DAILY` For the first weekend of every month: `FREQ=MONTHLY;BYSETPOS=1;BYDAY=SA,SU` This specifies how frequently the window starts. Eg, if you wanted to have a 9-5 UTC-4 window every weekday, you'd use something like: ``` start time = 2019-01-01T09:00:00-0400 end time = 2019-01-01T17:00:00-0400 recurrence = FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR ``` Windows can span multiple days. Eg, to make the window encompass every weekend from midnight Saturday till the last minute of Sunday UTC: ``` start time = 2019-01-05T00:00:00Z end time = 2019-01-07T23:59:00Z recurrence = FREQ=WEEKLY;BYDAY=SA ``` Note the start and end time's specific dates are largely arbitrary except to specify duration of the window and when it first starts. The FREQ values of HOURLY, MINUTELY, and SECONDLY are not supported."]
    pub recurrence: ::std::option::Option<::std::string::String>,
    #[serde(rename = "window")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The window of the first recurrence."]
    pub window: ::std::option::Option<::std::boxed::Box<TimeWindow>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ReleaseChannel indicates which release channel a cluster is subscribed to. Release channels are arranged in order of risk. When a cluster is subscribed to a release channel, Google maintains both the master version and the node version. Node auto-upgrade defaults to true and cannot be disabled."]
pub struct ReleaseChannel {
    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "channel specifies which release channel the cluster is subscribed to."]
    pub channel: ::std::option::Option<ReleaseChannelChannelEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "channel specifies which release channel the cluster is subscribed to."]
pub enum ReleaseChannelChannelEnum {
    #[serde(rename = "UNSPECIFIED")]
    #[doc = "No channel specified."]
    Unspecified,
    #[serde(rename = "RAPID")]
    #[doc = "RAPID channel is offered on an early access basis for customers who want to test new releases. WARNING: Versions available in the RAPID Channel may be subject to unresolved issues with no known workaround and are not subject to any SLAs."]
    Rapid,
    #[serde(rename = "REGULAR")]
    #[doc = "Clusters subscribed to REGULAR receive versions that are considered GA quality. REGULAR is intended for production users who want to take advantage of new features."]
    Regular,
    #[serde(rename = "STABLE")]
    #[doc = "Clusters subscribed to STABLE receive versions that are known to be stable and reliable in production."]
    Stable,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ReleaseChannelConfig exposes configuration for a release channel."]
pub struct ReleaseChannelConfig {
    #[serde(rename = "availableVersions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. This field has been deprecated and replaced with the valid_versions field."]
    pub available_versions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AvailableVersion>>>,
    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The release channel this configuration applies to."]
    pub channel: ::std::option::Option<ReleaseChannelConfigChannelEnum>,
    #[serde(rename = "defaultVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The default version for newly created clusters on the channel."]
    pub default_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "validVersions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of valid versions for the channel."]
    pub valid_versions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The release channel this configuration applies to."]
pub enum ReleaseChannelConfigChannelEnum {
    #[serde(rename = "UNSPECIFIED")]
    #[doc = "No channel specified."]
    Unspecified,
    #[serde(rename = "RAPID")]
    #[doc = "RAPID channel is offered on an early access basis for customers who want to test new releases. WARNING: Versions available in the RAPID Channel may be subject to unresolved issues with no known workaround and are not subject to any SLAs."]
    Rapid,
    #[serde(rename = "REGULAR")]
    #[doc = "Clusters subscribed to REGULAR receive versions that are considered GA quality. REGULAR is intended for production users who want to take advantage of new features."]
    Regular,
    #[serde(rename = "STABLE")]
    #[doc = "Clusters subscribed to STABLE receive versions that are known to be stable and reliable in production."]
    Stable,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "[ReservationAffinity](https://cloud.google.com/compute/docs/instances/reserving-zonal-resources) is the configuration of desired reservation which instances could take capacity from."]
pub struct ReservationAffinity {
    #[serde(rename = "consumeReservationType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Corresponds to the type of reservation consumption."]
    pub consume_reservation_type:
        ::std::option::Option<ReservationAffinityConsumeReservationTypeEnum>,
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Corresponds to the label key of a reservation resource. To target a SPECIFIC_RESERVATION by name, specify \"googleapis.com/reservation-name\" as the key and specify the name of your reservation as its value."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Corresponds to the label value(s) of reservation resource(s)."]
    pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Corresponds to the type of reservation consumption."]
pub enum ReservationAffinityConsumeReservationTypeEnum {
    #[serde(rename = "UNSPECIFIED")]
    #[doc = "Default value. This should not be used."]
    Unspecified,
    #[serde(rename = "NO_RESERVATION")]
    #[doc = "Do not consume from any reserved capacity."]
    NoReservation,
    #[serde(rename = "ANY_RESERVATION")]
    #[doc = "Consume any reservation available."]
    AnyReservation,
    #[serde(rename = "SPECIFIC_RESERVATION")]
    #[doc = "Must consume from a specific reservation. Must specify key value fields for specifying the reservations."]
    SpecificReservation,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains information about amount of some resource in the cluster. For memory, value should be in GB."]
pub struct ResourceLimit {
    #[serde(rename = "maximum")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum amount of the resource in the cluster."]
    pub maximum: ::std::option::Option<::std::string::String>,
    #[serde(rename = "minimum")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Minimum amount of the resource in the cluster."]
    pub minimum: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name \"cpu\", \"memory\" or gpu-specific string."]
    pub resource_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for exporting cluster resource usages."]
pub struct ResourceUsageExportConfig {
    #[serde(rename = "bigqueryDestination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration to use BigQuery as usage export destination."]
    pub bigquery_destination: ::std::option::Option<::std::boxed::Box<BigQueryDestination>>,
    #[serde(rename = "consumptionMeteringConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration to enable resource consumption metering."]
    pub consumption_metering_config:
        ::std::option::Option<::std::boxed::Box<ConsumptionMeteringConfig>>,
    #[serde(rename = "enableNetworkEgressMetering")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to enable network egress metering for this cluster. If enabled, a daemonset will be created in the cluster to meter network egress traffic."]
    pub enable_network_egress_metering: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "RollbackNodePoolUpgradeRequest rollbacks the previously Aborted or Failed NodePool upgrade. This will be an no-op if the last upgrade successfully completed."]
pub struct RollbackNodePoolUpgradeRequest {
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the cluster to rollback. This field has been deprecated and replaced by the name field."]
    pub cluster_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name (project, location, cluster, node pool id) of the node poll to rollback upgrade. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nodePoolId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the node pool to rollback. This field has been deprecated and replaced by the name field."]
    pub node_pool_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
    pub zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "SandboxConfig contains configurations of the sandbox to use for the node."]
pub struct SandboxConfig {
    #[serde(rename = "sandboxType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the sandbox to use for the node (e.g. 'gvisor')"]
    pub sandbox_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the sandbox to use for the node."]
    pub _type: ::std::option::Option<SandboxConfigTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of the sandbox to use for the node."]
pub enum SandboxConfigTypeEnum {
    #[serde(rename = "UNSPECIFIED")]
    #[doc = "Default value. This should not be used."]
    Unspecified,
    #[serde(rename = "GVISOR")]
    #[doc = "Run sandbox using gvisor."]
    Gvisor,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Kubernetes Engine service configuration."]
pub struct ServerConfig {
    #[serde(rename = "channels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of release channel configurations."]
    pub channels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ReleaseChannelConfig>>>,
    #[serde(rename = "defaultClusterVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Version of Kubernetes the service deploys by default."]
    pub default_cluster_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "defaultImageType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Default image type."]
    pub default_image_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "validImageTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of valid image types."]
    pub valid_image_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "validMasterVersions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of valid master versions, in descending order."]
    pub valid_master_versions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "validNodeVersions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of valid node upgrade target versions, in descending order."]
    pub valid_node_versions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "SetAddonsRequest sets the addons associated with the cluster."]
pub struct SetAddonsConfigRequest {
    #[serde(rename = "addonsConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The desired configurations for the various addons available to run in the cluster."]
    pub addons_config: ::std::option::Option<::std::boxed::Box<AddonsConfig>>,
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."]
    pub cluster_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name (project, location, cluster) of the cluster to set addons. Specified in the format `projects/*/locations/*/clusters/*`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
    pub zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "SetLabelsRequest sets the Google Cloud Platform labels on a Google Container Engine cluster, which will in turn set them for Google Compute Engine resources used by that cluster"]
pub struct SetLabelsRequest {
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the cluster. This field has been deprecated and replaced by the name field."]
    pub cluster_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labelFingerprint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The fingerprint of the previous set of labels for this resource, used to detect conflicts. The fingerprint is initially generated by Kubernetes Engine and changes after every request to modify or update labels. You must always provide an up-to-date fingerprint hash when updating or changing labels. Make a `get()` request to the resource to get the latest fingerprint."]
    pub label_fingerprint: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name (project, location, cluster id) of the cluster to set labels. Specified in the format `projects/*/locations/*/clusters/*`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The Google Developers Console [project ID or project number](https://developers.google.com/console/help/new/#projectnumber). This field has been deprecated and replaced by the name field."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceLabels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The labels to set for that cluster."]
    pub resource_labels:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
    pub zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "SetLegacyAbacRequest enables or disables the ABAC authorization mechanism for a cluster."]
pub struct SetLegacyAbacRequest {
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the cluster to update. This field has been deprecated and replaced by the name field."]
    pub cluster_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Whether ABAC authorization will be enabled in the cluster."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name (project, location, cluster id) of the cluster to set legacy abac. Specified in the format `projects/*/locations/*/clusters/*`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
    pub zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "SetLocationsRequest sets the locations of the cluster."]
pub struct SetLocationsRequest {
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."]
    pub cluster_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The desired list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available) in which the cluster's nodes should be located. Changing the locations a cluster is in will result in nodes being either created or removed from the cluster, depending on whether locations are being added or removed. This list must always include the cluster's primary zone."]
    pub locations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name (project, location, cluster) of the cluster to set locations. Specified in the format `projects/*/locations/*/clusters/*`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
    pub zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "SetLoggingServiceRequest sets the logging service of a cluster."]
pub struct SetLoggingServiceRequest {
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."]
    pub cluster_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "loggingService")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The logging service the cluster should use to write logs. Currently available options: * `logging.googleapis.com/kubernetes` - The Cloud Logging service with a Kubernetes-native resource model * `logging.googleapis.com` - The legacy Cloud Logging service (no longer available as of GKE 1.15). * `none` - no logs will be exported from the cluster. If left as an empty string,`logging.googleapis.com/kubernetes` will be used for GKE 1.14+ or `logging.googleapis.com` for earlier versions."]
    pub logging_service: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name (project, location, cluster) of the cluster to set logging. Specified in the format `projects/*/locations/*/clusters/*`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
    pub zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "SetMaintenancePolicyRequest sets the maintenance policy for a cluster."]
pub struct SetMaintenancePolicyRequest {
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the cluster to update."]
    pub cluster_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maintenancePolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The maintenance policy to be set for the cluster. An empty field clears the existing maintenance policy."]
    pub maintenance_policy: ::std::option::Option<::std::boxed::Box<MaintenancePolicy>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name (project, location, cluster id) of the cluster to set maintenance policy. Specified in the format `projects/*/locations/*/clusters/*`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840)."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides."]
    pub zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "SetMasterAuthRequest updates the admin password of a cluster."]
pub struct SetMasterAuthRequest {
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The exact form of action to be taken on the master auth."]
    pub action: ::std::option::Option<SetMasterAuthRequestActionEnum>,
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."]
    pub cluster_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name (project, location, cluster) of the cluster to set auth. Specified in the format `projects/*/locations/*/clusters/*`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "update")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A description of the update."]
    pub update: ::std::option::Option<::std::boxed::Box<MasterAuth>>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
    pub zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The exact form of action to be taken on the master auth."]
pub enum SetMasterAuthRequestActionEnum {
    #[serde(rename = "UNKNOWN")]
    #[doc = "Operation is unknown and will error out."]
    Unknown,
    #[serde(rename = "SET_PASSWORD")]
    #[doc = "Set the password to a user generated value."]
    SetPassword,
    #[serde(rename = "GENERATE_PASSWORD")]
    #[doc = "Generate a new password and set it to that."]
    GeneratePassword,
    #[serde(rename = "SET_USERNAME")]
    #[doc = "Set the username. If an empty username is provided, basic authentication is disabled for the cluster. If a non-empty username is provided, basic authentication is enabled, with either a provided password or a generated one."]
    SetUsername,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "SetMonitoringServiceRequest sets the monitoring service of a cluster."]
pub struct SetMonitoringServiceRequest {
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."]
    pub cluster_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "monitoringService")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The monitoring service the cluster should use to write metrics. Currently available options: * \"monitoring.googleapis.com/kubernetes\" - The Cloud Monitoring service with a Kubernetes-native resource model * `monitoring.googleapis.com` - The legacy Cloud Monitoring service (no longer available as of GKE 1.15). * `none` - No metrics will be exported from the cluster. If left as an empty string,`monitoring.googleapis.com/kubernetes` will be used for GKE 1.14+ or `monitoring.googleapis.com` for earlier versions."]
    pub monitoring_service: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name (project, location, cluster) of the cluster to set monitoring. Specified in the format `projects/*/locations/*/clusters/*`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
    pub zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "SetNetworkPolicyRequest enables/disables network policy for a cluster."]
pub struct SetNetworkPolicyRequest {
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the cluster. This field has been deprecated and replaced by the name field."]
    pub cluster_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name (project, location, cluster id) of the cluster to set networking policy. Specified in the format `projects/*/locations/*/clusters/*`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "networkPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Configuration options for the NetworkPolicy feature."]
    pub network_policy: ::std::option::Option<::std::boxed::Box<NetworkPolicy>>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The Google Developers Console [project ID or project number](https://developers.google.com/console/help/new/#projectnumber). This field has been deprecated and replaced by the name field."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
    pub zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "SetNodePoolAutoscalingRequest sets the autoscaler settings of a node pool."]
pub struct SetNodePoolAutoscalingRequest {
    #[serde(rename = "autoscaling")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Autoscaling configuration for the node pool."]
    pub autoscaling: ::std::option::Option<::std::boxed::Box<NodePoolAutoscaling>>,
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."]
    pub cluster_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name (project, location, cluster, node pool) of the node pool to set autoscaler settings. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nodePoolId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the node pool to upgrade. This field has been deprecated and replaced by the name field."]
    pub node_pool_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
    pub zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "SetNodePoolManagementRequest sets the node management properties of a node pool."]
pub struct SetNodePoolManagementRequest {
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the cluster to update. This field has been deprecated and replaced by the name field."]
    pub cluster_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "management")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. NodeManagement configuration for the node pool."]
    pub management: ::std::option::Option<::std::boxed::Box<NodeManagement>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name (project, location, cluster, node pool id) of the node pool to set management properties. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nodePoolId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the node pool to update. This field has been deprecated and replaced by the name field."]
    pub node_pool_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
    pub zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "SetNodePoolSizeRequest sets the size of a node pool."]
pub struct SetNodePoolSizeRequest {
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the cluster to update. This field has been deprecated and replaced by the name field."]
    pub cluster_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name (project, location, cluster, node pool id) of the node pool to set size. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nodeCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The desired node count for the pool."]
    pub node_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "nodePoolId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the node pool to update. This field has been deprecated and replaced by the name field."]
    pub node_pool_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
    pub zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A set of Shielded Instance options."]
pub struct ShieldedInstanceConfig {
    #[serde(rename = "enableIntegrityMonitoring")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Defines whether the instance has integrity monitoring enabled. Enables monitoring and attestation of the boot integrity of the instance. The attestation is performed against the integrity policy baseline. This baseline is initially derived from the implicitly trusted boot image when the instance is created."]
    pub enable_integrity_monitoring: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "enableSecureBoot")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Defines whether the instance has Secure Boot enabled. Secure Boot helps ensure that the system only runs authentic software by verifying the digital signature of all boot components, and halting the boot process if signature verification fails."]
    pub enable_secure_boot: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration of Shielded Nodes feature."]
pub struct ShieldedNodes {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether Shielded Nodes features are enabled on all nodes in this cluster."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "StartIPRotationRequest creates a new IP for the cluster and then performs a node upgrade on each node pool to point to the new IP."]
pub struct StartIpRotationRequest {
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the cluster. This field has been deprecated and replaced by the name field."]
    pub cluster_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name (project, location, cluster id) of the cluster to start IP rotation. Specified in the format `projects/*/locations/*/clusters/*`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The Google Developers Console [project ID or project number](https://developers.google.com/console/help/new/#projectnumber). This field has been deprecated and replaced by the name field."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rotateCredentials")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to rotate credentials during IP rotation."]
    pub rotate_credentials: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
    pub zone: ::std::option::Option<::std::string::String>,
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
#[doc = "StatusCondition describes why a cluster or a node pool has a certain status (e.g., ERROR or DEGRADED)."]
pub struct StatusCondition {
    #[serde(rename = "canonicalCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Canonical code of the condition."]
    pub canonical_code: ::std::option::Option<StatusConditionCanonicalCodeEnum>,
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Machine-friendly representation of the condition Deprecated. Use canonical_code instead."]
    pub code: ::std::option::Option<StatusConditionCodeEnum>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Human-friendly representation of the condition"]
    pub message: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Canonical code of the condition."]
pub enum StatusConditionCanonicalCodeEnum {
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
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Machine-friendly representation of the condition Deprecated. Use canonical_code instead."]
pub enum StatusConditionCodeEnum {
    #[serde(rename = "UNKNOWN")]
    #[doc = "UNKNOWN indicates a generic condition."]
    Unknown,
    #[serde(rename = "GCE_STOCKOUT")]
    #[doc = "GCE_STOCKOUT indicates that Google Compute Engine resources are temporarily unavailable."]
    GceStockout,
    #[serde(rename = "GKE_SERVICE_ACCOUNT_DELETED")]
    #[doc = "GKE_SERVICE_ACCOUNT_DELETED indicates that the user deleted their robot service account."]
    GkeServiceAccountDeleted,
    #[serde(rename = "GCE_QUOTA_EXCEEDED")]
    #[doc = "Google Compute Engine quota was exceeded."]
    GceQuotaExceeded,
    #[serde(rename = "SET_BY_OPERATOR")]
    #[doc = "Cluster state was manually changed by an SRE due to a system logic error."]
    SetByOperator,
    #[serde(rename = "CLOUD_KMS_KEY_ERROR")]
    #[doc = "Unable to perform an encrypt operation against the CloudKMS key used for etcd level encryption. More codes TBA"]
    CloudKmsKeyError,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an arbitrary window of time."]
pub struct TimeWindow {
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time that the window ends. The end time should take place after the start time."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time that the window first starts."]
    pub start_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for Cloud TPU."]
pub struct TpuConfig {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether Cloud TPU integration is enabled or not."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "ipv4CidrBlock")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "IPv4 CIDR block reserved for Cloud TPU in the VPC."]
    pub ipv4_cidr_block: ::std::option::Option<::std::string::String>,
    #[serde(rename = "useServiceNetworking")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to use service networking for Cloud TPU or not."]
    pub use_service_networking: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "UpdateClusterRequest updates the settings of a cluster."]
pub struct UpdateClusterRequest {
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."]
    pub cluster_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name (project, location, cluster) of the cluster to update. Specified in the format `projects/*/locations/*/clusters/*`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "update")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A description of the update."]
    pub update: ::std::option::Option<::std::boxed::Box<ClusterUpdate>>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
    pub zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "UpdateMasterRequest updates the master of the cluster."]
pub struct UpdateMasterRequest {
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."]
    pub cluster_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "masterVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The Kubernetes version to change the master to. Users may specify either explicit versions offered by Kubernetes Engine or version aliases, which have the following behavior: - \"latest\": picks the highest valid Kubernetes version - \"1.X\": picks the highest valid patch+gke.N patch in the 1.X version - \"1.X.Y\": picks the highest valid gke.N patch in the 1.X.Y version - \"1.X.Y-gke.N\": picks an explicit Kubernetes version - \"-\": picks the default Kubernetes version"]
    pub master_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name (project, location, cluster) of the cluster to update. Specified in the format `projects/*/locations/*/clusters/*`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
    pub zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "SetNodePoolVersionRequest updates the version of a node pool."]
pub struct UpdateNodePoolRequest {
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the cluster to upgrade. This field has been deprecated and replaced by the name field."]
    pub cluster_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imageType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The desired image type for the node pool."]
    pub image_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kubeletConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Node kubelet configs."]
    pub kubelet_config: ::std::option::Option<::std::boxed::Box<NodeKubeletConfig>>,
    #[serde(rename = "linuxNodeConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parameters that can be configured on Linux nodes."]
    pub linux_node_config: ::std::option::Option<::std::boxed::Box<LinuxNodeConfig>>,
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The desired list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available) in which the node pool's nodes should be located. Changing the locations for a node pool will result in nodes being either created or removed from the node pool, depending on whether locations are being added or removed."]
    pub locations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name (project, location, cluster, node pool) of the node pool to update. Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nodePoolId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the node pool to upgrade. This field has been deprecated and replaced by the name field."]
    pub node_pool_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nodeVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The Kubernetes version to change the nodes to (typically an upgrade). Users may specify either explicit versions offered by Kubernetes Engine or version aliases, which have the following behavior: - \"latest\": picks the highest valid Kubernetes version - \"1.X\": picks the highest valid patch+gke.N patch in the 1.X version - \"1.X.Y\": picks the highest valid gke.N patch in the 1.X.Y version - \"1.X.Y-gke.N\": picks an explicit Kubernetes version - \"-\": picks the Kubernetes master version"]
    pub node_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The Google Developers Console [project ID or project number](https://support.google.com/cloud/answer/6158840). This field has been deprecated and replaced by the name field."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "upgradeSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Upgrade settings control disruption and speed of the upgrade."]
    pub upgrade_settings: ::std::option::Option<::std::boxed::Box<UpgradeSettings>>,
    #[serde(rename = "workloadMetadataConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The desired workload metadata config for the node pool."]
    pub workload_metadata_config: ::std::option::Option<::std::boxed::Box<WorkloadMetadataConfig>>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. The name of the Google Compute Engine [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster resides. This field has been deprecated and replaced by the name field."]
    pub zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "UpgradeAvailableEvent is a notification sent to customers when a new available version is released."]
pub struct UpgradeAvailableEvent {
    #[serde(rename = "releaseChannel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The release channel of the version. If empty, it means a non-channel release."]
    pub release_channel: ::std::option::Option<::std::boxed::Box<ReleaseChannel>>,
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Optional relative path to the resource. For example, the relative path of the node pool."]
    pub resource: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource type of the release version."]
    pub resource_type: ::std::option::Option<UpgradeAvailableEventResourceTypeEnum>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The release version available for upgrade."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The resource type of the release version."]
pub enum UpgradeAvailableEventResourceTypeEnum {
    #[serde(rename = "UPGRADE_RESOURCE_TYPE_UNSPECIFIED")]
    #[doc = "Default value. This shouldn't be used."]
    UpgradeResourceTypeUnspecified,
    #[serde(rename = "MASTER")]
    #[doc = "Master / control plane"]
    Master,
    #[serde(rename = "NODE_POOL")]
    #[doc = "Node pool"]
    NodePool,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "UpgradeEvent is a notification sent to customers by the cluster server when a resource is upgrading."]
pub struct UpgradeEvent {
    #[serde(rename = "currentVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current version before the upgrade."]
    pub current_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The operation associated with this upgrade."]
    pub operation: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationStartTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time when the operation was started."]
    pub operation_start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional relative path to the resource. For example in node pool upgrades, the relative path of the node pool."]
    pub resource: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource type that is upgrading."]
    pub resource_type: ::std::option::Option<UpgradeEventResourceTypeEnum>,
    #[serde(rename = "targetVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target version for the upgrade."]
    pub target_version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The resource type that is upgrading."]
pub enum UpgradeEventResourceTypeEnum {
    #[serde(rename = "UPGRADE_RESOURCE_TYPE_UNSPECIFIED")]
    #[doc = "Default value. This shouldn't be used."]
    UpgradeResourceTypeUnspecified,
    #[serde(rename = "MASTER")]
    #[doc = "Master / control plane"]
    Master,
    #[serde(rename = "NODE_POOL")]
    #[doc = "Node pool"]
    NodePool,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "These upgrade settings control the level of parallelism and the level of disruption caused by an upgrade. maxUnavailable controls the number of nodes that can be simultaneously unavailable. maxSurge controls the number of additional nodes that can be added to the node pool temporarily for the time of the upgrade to increase the number of available nodes. (maxUnavailable + maxSurge) determines the level of parallelism (how many nodes are being upgraded at the same time). Note: upgrades inevitably introduce some disruption since workloads need to be moved from old nodes to new, upgraded ones. Even if maxUnavailable=0, this holds true. (Disruption stays within the limits of PodDisruptionBudget, if it is configured.) Consider a hypothetical node pool with 5 nodes having maxSurge=2, maxUnavailable=1. This means the upgrade process upgrades 3 nodes simultaneously. It creates 2 additional (upgraded) nodes, then it brings down 3 old (not yet upgraded) nodes at the same time. This ensures that there are always at least 4 nodes available."]
pub struct UpgradeSettings {
    #[serde(rename = "maxSurge")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum number of nodes that can be created beyond the current size of the node pool during the upgrade process."]
    pub max_surge: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "maxUnavailable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum number of nodes that can be simultaneously unavailable during the upgrade process. A node is considered available if its status is Ready."]
    pub max_unavailable: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "UsableSubnetwork resource returns the subnetwork name, its associated network and the primary CIDR range."]
pub struct UsableSubnetwork {
    #[serde(rename = "ipCidrRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The range of internal addresses that are owned by this subnetwork."]
    pub ip_cidr_range: ::std::option::Option<::std::string::String>,
    #[serde(rename = "network")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Network Name. Example: projects/my-project/global/networks/my-network"]
    pub network: ::std::option::Option<::std::string::String>,
    #[serde(rename = "secondaryIpRanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Secondary IP ranges."]
    pub secondary_ip_ranges:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UsableSubnetworkSecondaryRange>>>,
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A human readable status message representing the reasons for cases where the caller cannot use the secondary ranges under the subnet. For example if the secondary_ip_ranges is empty due to a permission issue, an insufficient permission message will be given by status_message."]
    pub status_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subnetwork")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subnetwork Name. Example: projects/my-project/regions/us-central1/subnetworks/my-subnet"]
    pub subnetwork: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Secondary IP range of a usable subnetwork."]
pub struct UsableSubnetworkSecondaryRange {
    #[serde(rename = "ipCidrRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The range of IP addresses belonging to this subnetwork secondary range."]
    pub ip_cidr_range: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rangeName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name associated with this subnetwork secondary range, used when adding an alias IP range to a VM instance."]
    pub range_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field is to determine the status of the secondary range programmably."]
    pub status: ::std::option::Option<UsableSubnetworkSecondaryRangeStatusEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "This field is to determine the status of the secondary range programmably."]
pub enum UsableSubnetworkSecondaryRangeStatusEnum {
    #[serde(rename = "UNKNOWN")]
    #[doc = "UNKNOWN is the zero value of the Status enum. It's not a valid status."]
    Unknown,
    #[serde(rename = "UNUSED")]
    #[doc = "UNUSED denotes that this range is unclaimed by any cluster."]
    Unused,
    #[serde(rename = "IN_USE_SERVICE")]
    #[doc = "IN_USE_SERVICE denotes that this range is claimed by a cluster for services. It cannot be used for other clusters."]
    InUseService,
    #[serde(rename = "IN_USE_SHAREABLE_POD")]
    #[doc = "IN_USE_SHAREABLE_POD denotes this range was created by the network admin and is currently claimed by a cluster for pods. It can only be used by other clusters as a pod range."]
    InUseShareablePod,
    #[serde(rename = "IN_USE_MANAGED_POD")]
    #[doc = "IN_USE_MANAGED_POD denotes this range was created by GKE and is claimed for pods. It cannot be used for other clusters."]
    InUseManagedPod,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "VerticalPodAutoscaling contains global, per-cluster information required by Vertical Pod Autoscaler to automatically adjust the resources of pods controlled by it."]
pub struct VerticalPodAutoscaling {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Enables vertical pod autoscaling."]
    pub enabled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for the use of Kubernetes Service Accounts in GCP IAM policies."]
pub struct WorkloadIdentityConfig {
    #[serde(rename = "identityNamespace")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "IAM Identity Namespace to attach all Kubernetes Service Accounts to."]
    pub identity_namespace: ::std::option::Option<::std::string::String>,
    #[serde(rename = "identityProvider")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "identity provider is the third party identity provider."]
    pub identity_provider: ::std::option::Option<::std::string::String>,
    #[serde(rename = "workloadPool")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The workload pool to attach all Kubernetes service accounts to."]
    pub workload_pool: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "WorkloadMetadataConfig defines the metadata configuration to expose to workloads on the node pool."]
pub struct WorkloadMetadataConfig {
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Mode is the configuration for how to expose metadata to workloads running on the node pool."]
    pub mode: ::std::option::Option<WorkloadMetadataConfigModeEnum>,
    #[serde(rename = "nodeMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "NodeMetadata is the configuration for how to expose metadata to the workloads running on the node."]
    pub node_metadata: ::std::option::Option<WorkloadMetadataConfigNodeMetadataEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Mode is the configuration for how to expose metadata to workloads running on the node pool."]
pub enum WorkloadMetadataConfigModeEnum {
    #[serde(rename = "MODE_UNSPECIFIED")]
    #[doc = "Not set."]
    ModeUnspecified,
    #[serde(rename = "GCE_METADATA")]
    #[doc = "Expose all Compute Engine metadata to pods."]
    GceMetadata,
    #[serde(rename = "GKE_METADATA")]
    #[doc = "Run the GKE Metadata Server on this node. The GKE Metadata Server exposes a metadata API to workloads that is compatible with the V1 Compute Metadata APIs exposed by the Compute Engine and App Engine Metadata Servers. This feature can only be enabled if Workload Identity is enabled at the cluster level."]
    GkeMetadata,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "NodeMetadata is the configuration for how to expose metadata to the workloads running on the node."]
pub enum WorkloadMetadataConfigNodeMetadataEnum {
    #[serde(rename = "UNSPECIFIED")]
    #[doc = "Not set."]
    Unspecified,
    #[serde(rename = "SECURE")]
    #[doc = "Prevent workloads not in hostNetwork from accessing certain VM metadata, specifically kube-env, which contains Kubelet credentials, and the instance identity token. Metadata concealment is a temporary security solution available while the bootstrapping process for cluster nodes is being redesigned with significant security improvements. This feature is scheduled to be deprecated in the future and later removed."]
    Secure,
    #[serde(rename = "EXPOSE")]
    #[doc = "Expose all VM metadata to pods."]
    Expose,
    #[serde(rename = "GKE_METADATA_SERVER")]
    #[doc = "Run the GKE Metadata Server on this node. The GKE Metadata Server exposes a metadata API to workloads that is compatible with the V1 Compute Metadata APIs exposed by the Compute Engine and App Engine Metadata Servers. This feature can only be enabled if Workload Identity is enabled at the cluster level."]
    GkeMetadataServer,
}
