#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies the type and number of accelerator cards attached to the instances of an instance. See GPUs on Compute Engine (https://cloud.google.com/compute/docs/gpus/)."]
pub struct AcceleratorConfig {
    #[serde(rename = "acceleratorCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of the accelerator cards of this type exposed to this instance."]
    pub accelerator_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "acceleratorTypeUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Full URL, partial URI, or short name of the accelerator type resource to expose to this instance. See Compute Engine AcceleratorTypes (https://cloud.google.com/compute/docs/reference/beta/acceleratorTypes).Examples: https://www.googleapis.com/compute/beta/projects/[project_id]/zones/us-east1-a/acceleratorTypes/nvidia-tesla-k80 projects/[project_id]/zones/us-east1-a/acceleratorTypes/nvidia-tesla-k80 nvidia-tesla-k80Auto Zone Exception: If you are using the Dataproc Auto Zone Placement (https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/auto-zone#using_auto_zone_placement) feature, you must use the short name of the accelerator type resource, for example, nvidia-tesla-k80."]
    pub accelerator_type_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Autoscaling Policy config associated with the cluster."]
pub struct AutoscalingConfig {
    #[serde(rename = "policyUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The autoscaling policy used by the cluster.Only resource names including projectid and location (region) are valid. Examples: https://www.googleapis.com/compute/v1/projects/[project_id]/locations/[dataproc_region]/autoscalingPolicies/[policy_id] projects/[project_id]/locations/[dataproc_region]/autoscalingPolicies/[policy_id]Note that the policy must be in the same project and Dataproc region."]
    pub policy_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes an autoscaling policy for Dataproc cluster autoscaler."]
pub struct AutoscalingPolicy {
    #[serde(rename = "basicAlgorithm")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub basic_algorithm: ::std::option::Option<::std::boxed::Box<BasicAutoscalingAlgorithm>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The policy id.The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between 3 and 50 characters."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The \"resource name\" of the autoscaling policy, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies, the resource name of the policy has the following format: projects/{project_id}/regions/{region}/autoscalingPolicies/{policy_id} For projects.locations.autoscalingPolicies, the resource name of the policy has the following format: projects/{project_id}/locations/{location}/autoscalingPolicies/{policy_id}"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "secondaryWorkerConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Describes how the autoscaler will operate for secondary workers."]
    pub secondary_worker_config:
        ::std::option::Option<::std::boxed::Box<InstanceGroupAutoscalingPolicyConfig>>,
    #[serde(rename = "workerConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Describes how the autoscaler will operate for primary workers."]
    pub worker_config:
        ::std::option::Option<::std::boxed::Box<InstanceGroupAutoscalingPolicyConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Basic algorithm for autoscaling."]
pub struct BasicAutoscalingAlgorithm {
    #[serde(rename = "cooldownPeriod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Duration between scaling events. A scaling period starts after the update operation from the previous event has completed.Bounds: 2m, 1d. Default: 2m."]
    pub cooldown_period: ::std::option::Option<::std::string::String>,
    #[serde(rename = "yarnConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. YARN autoscaling configuration."]
    pub yarn_config: ::std::option::Option<::std::boxed::Box<BasicYarnAutoscalingConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Basic autoscaling configurations for YARN."]
pub struct BasicYarnAutoscalingConfig {
    #[serde(rename = "gracefulDecommissionTimeout")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Timeout for YARN graceful decommissioning of Node Managers. Specifies the duration to wait for jobs to complete before forcefully removing workers (and potentially interrupting jobs). Only applicable to downscaling operations.Bounds: 0s, 1d."]
    pub graceful_decommission_timeout: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scaleDownFactor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Fraction of average YARN pending memory in the last cooldown period for which to remove workers. A scale-down factor of 1 will result in scaling down so that there is no available memory remaining after the update (more aggressive scaling). A scale-down factor of 0 disables removing workers, which can be beneficial for autoscaling a single job. See How autoscaling works (https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/autoscaling#how_autoscaling_works) for more information.Bounds: 0.0, 1.0."]
    pub scale_down_factor: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "scaleDownMinWorkerFraction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Minimum scale-down threshold as a fraction of total cluster size before scaling occurs. For example, in a 20-worker cluster, a threshold of 0.1 means the autoscaler must recommend at least a 2 worker scale-down for the cluster to scale. A threshold of 0 means the autoscaler will scale down on any recommended change.Bounds: 0.0, 1.0. Default: 0.0."]
    pub scale_down_min_worker_fraction: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "scaleUpFactor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Fraction of average YARN pending memory in the last cooldown period for which to add workers. A scale-up factor of 1.0 will result in scaling up so that there is no pending memory remaining after the update (more aggressive scaling). A scale-up factor closer to 0 will result in a smaller magnitude of scaling up (less aggressive scaling). See How autoscaling works (https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/autoscaling#how_autoscaling_works) for more information.Bounds: 0.0, 1.0."]
    pub scale_up_factor: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "scaleUpMinWorkerFraction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Minimum scale-up threshold as a fraction of total cluster size before scaling occurs. For example, in a 20-worker cluster, a threshold of 0.1 means the autoscaler must recommend at least a 2-worker scale-up for the cluster to scale. A threshold of 0 means the autoscaler will scale up on any recommended change.Bounds: 0.0, 1.0. Default: 0.0."]
    pub scale_up_min_worker_fraction: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Associates members with a role."]
pub struct Binding {
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The condition that is associated with this binding.If the condition evaluates to true, then this binding applies to the current request.If the condition evaluates to false, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the members in this binding.To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies)."]
    pub condition: ::std::option::Option<::std::boxed::Box<Expr>>,
    #[serde(rename = "members")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the identities requesting access for a Cloud Platform resource. members can have the following values: allUsers: A special identifier that represents anyone who is on the internet; with or without a Google account. allAuthenticatedUsers: A special identifier that represents anyone who is authenticated with a Google account or a service account. user:{emailid}: An email address that represents a specific Google account. For example, alice@example.com . serviceAccount:{emailid}: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com. group:{emailid}: An email address that represents a Google group. For example, admins@example.com. deleted:user:{emailid}?uid={uniqueid}: An email address (plus unique identifier) representing a user that has been recently deleted. For example, alice@example.com?uid=123456789012345678901. If the user is recovered, this value reverts to user:{emailid} and the recovered user retains the role in the binding. deleted:serviceAccount:{emailid}?uid={uniqueid}: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901. If the service account is undeleted, this value reverts to serviceAccount:{emailid} and the undeleted service account retains the role in the binding. deleted:group:{emailid}?uid={uniqueid}: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, admins@example.com?uid=123456789012345678901. If the group is recovered, this value reverts to group:{emailid} and the recovered group retains the role in the binding. domain:{domain}: The G Suite domain (primary) that represents all the users of that domain. For example, google.com or example.com."]
    pub members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Role that is assigned to members. For example, roles/viewer, roles/editor, or roles/owner."]
    pub role: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request to cancel a job."]
pub struct CancelJobRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes the identifying information, config, and status of a cluster of Compute Engine instances."]
pub struct Cluster {
    #[serde(rename = "clusterName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The cluster name. Cluster names within a project must be unique. Names of deleted clusters can be reused."]
    pub cluster_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "clusterUuid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A cluster UUID (Unique Universal Identifier). Dataproc generates this value when it creates the cluster."]
    pub cluster_uuid: ::std::option::Option<::std::string::String>,
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The cluster config. Note that Dataproc may set default values, and values may change when clusters are updated."]
    pub config: ::std::option::Option<::std::boxed::Box<ClusterConfig>>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The labels to associate with this cluster. Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). Label values may be empty, but, if present, must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with a cluster."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Contains cluster daemon metrics such as HDFS and YARN stats.Beta Feature: This report is available for testing purposes only. It may be changed before final release."]
    pub metrics: ::std::option::Option<::std::boxed::Box<ClusterMetrics>>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The Google Cloud Platform project ID that the cluster belongs to."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Cluster status."]
    pub status: ::std::option::Option<::std::boxed::Box<ClusterStatus>>,
    #[serde(rename = "statusHistory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The previous cluster status."]
    pub status_history: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ClusterStatus>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The cluster config."]
pub struct ClusterConfig {
    #[serde(rename = "autoscalingConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Autoscaling config for the policy associated with the cluster. Cluster does not autoscale if this field is unset."]
    pub autoscaling_config: ::std::option::Option<::std::boxed::Box<AutoscalingConfig>>,
    #[serde(rename = "configBucket")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A Cloud Storage bucket used to stage job dependencies, config files, and job driver console output. If you do not specify a staging bucket, Cloud Dataproc will determine a Cloud Storage location (US, ASIA, or EU) for your cluster's staging bucket according to the Compute Engine zone where your cluster is deployed, and then create and manage this project-level, per-location bucket (see Dataproc staging bucket (https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/staging-bucket)). This field requires a Cloud Storage bucket name, not a URI to a Cloud Storage bucket."]
    pub config_bucket: ::std::option::Option<::std::string::String>,
    #[serde(rename = "encryptionConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Encryption settings for the cluster."]
    pub encryption_config: ::std::option::Option<::std::boxed::Box<EncryptionConfig>>,
    #[serde(rename = "endpointConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Port/endpoint configuration for this cluster"]
    pub endpoint_config: ::std::option::Option<::std::boxed::Box<EndpointConfig>>,
    #[serde(rename = "gceClusterConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The shared Compute Engine config settings for all instances in a cluster."]
    pub gce_cluster_config: ::std::option::Option<::std::boxed::Box<GceClusterConfig>>,
    #[serde(rename = "initializationActions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Commands to execute on each node after config is completed. By default, executables are run on master and all worker nodes. You can test a node's role metadata to run an executable on a master or worker node, as shown below using curl (you can also use wget): ROLE=$(curl -H Metadata-Flavor:Google http://metadata/computeMetadata/v1/instance/attributes/dataproc-role) if [[ \"${ROLE}\" == 'Master' ]]; then ... master specific actions ... else ... worker specific actions ... fi "]
    pub initialization_actions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NodeInitializationAction>>>,
    #[serde(rename = "lifecycleConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Lifecycle setting for the cluster."]
    pub lifecycle_config: ::std::option::Option<::std::boxed::Box<LifecycleConfig>>,
    #[serde(rename = "masterConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The Compute Engine config settings for the master instance in a cluster."]
    pub master_config: ::std::option::Option<::std::boxed::Box<InstanceGroupConfig>>,
    #[serde(rename = "metastoreConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Metastore configuration."]
    pub metastore_config: ::std::option::Option<::std::boxed::Box<MetastoreConfig>>,
    #[serde(rename = "secondaryWorkerConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The Compute Engine config settings for additional worker instances in a cluster."]
    pub secondary_worker_config: ::std::option::Option<::std::boxed::Box<InstanceGroupConfig>>,
    #[serde(rename = "securityConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Security settings for the cluster."]
    pub security_config: ::std::option::Option<::std::boxed::Box<SecurityConfig>>,
    #[serde(rename = "softwareConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The config settings for software inside the cluster."]
    pub software_config: ::std::option::Option<::std::boxed::Box<SoftwareConfig>>,
    #[serde(rename = "tempBucket")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A Cloud Storage bucket used to store ephemeral cluster and jobs data, such as Spark and MapReduce history files. If you do not specify a temp bucket, Dataproc will determine a Cloud Storage location (US, ASIA, or EU) for your cluster's temp bucket according to the Compute Engine zone where your cluster is deployed, and then create and manage this project-level, per-location bucket. The default bucket has a TTL of 90 days, but you can use any TTL (or none) if you specify a bucket. This field requires a Cloud Storage bucket name, not a URI to a Cloud Storage bucket."]
    pub temp_bucket: ::std::option::Option<::std::string::String>,
    #[serde(rename = "workerConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The Compute Engine config settings for worker instances in a cluster."]
    pub worker_config: ::std::option::Option<::std::boxed::Box<InstanceGroupConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains cluster daemon metrics, such as HDFS and YARN stats.Beta Feature: This report is available for testing purposes only. It may be changed before final release."]
pub struct ClusterMetrics {
    #[serde(rename = "hdfsMetrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The HDFS metrics."]
    pub hdfs_metrics:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "yarnMetrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The YARN metrics."]
    pub yarn_metrics:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The cluster operation triggered by a workflow."]
pub struct ClusterOperation {
    #[serde(rename = "done")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Indicates the operation is done."]
    pub done: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Error, if operation failed."]
    pub error: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The id of the cluster operation."]
    pub operation_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata describing the operation."]
pub struct ClusterOperationMetadata {
    #[serde(rename = "clusterName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Name of the cluster for the operation."]
    pub cluster_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "clusterUuid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Cluster UUID for the operation."]
    pub cluster_uuid: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Short description of operation."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Labels associated with the operation"]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "operationType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The operation type."]
    pub operation_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Current operation status."]
    pub status: ::std::option::Option<::std::boxed::Box<ClusterOperationStatus>>,
    #[serde(rename = "statusHistory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The previous operation status."]
    pub status_history:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ClusterOperationStatus>>>,
    #[serde(rename = "warnings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Errors encountered during operation execution."]
    pub warnings: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The status of the operation."]
pub struct ClusterOperationStatus {
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A message containing any operation metadata details."]
    pub details: ::std::option::Option<::std::string::String>,
    #[serde(rename = "innerState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A message containing the detailed operation state."]
    pub inner_state: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A message containing the operation state."]
    pub state: ::std::option::Option<ClusterOperationStatusStateEnum>,
    #[serde(rename = "stateStartTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time this state was entered."]
    pub state_start_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. A message containing the operation state."]
pub enum ClusterOperationStatusStateEnum {
    #[serde(rename = "UNKNOWN")]
    #[doc = "Unused."]
    Unknown,
    #[serde(rename = "PENDING")]
    #[doc = "The operation has been created."]
    Pending,
    #[serde(rename = "RUNNING")]
    #[doc = "The operation is running."]
    Running,
    #[serde(rename = "DONE")]
    #[doc = "The operation is done; either cancelled or completed."]
    Done,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A selector that chooses target cluster for jobs based on metadata."]
pub struct ClusterSelector {
    #[serde(rename = "clusterLabels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The cluster labels. Cluster must have all labels to match."]
    pub cluster_labels:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The zone where workflow process executes. This parameter does not affect the selection of the cluster.If unspecified, the zone of the first cluster matching the selector is used."]
    pub zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The status of a cluster and its instances."]
pub struct ClusterStatus {
    #[serde(rename = "detail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Output only. Details of cluster's state."]
    pub detail: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The cluster's state."]
    pub state: ::std::option::Option<ClusterStatusStateEnum>,
    #[serde(rename = "stateStartTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Time when this state was entered (see JSON representation of Timestamp (https://developers.google.com/protocol-buffers/docs/proto3#json))."]
    pub state_start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "substate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Additional state information that includes status reported by the agent."]
    pub substate: ::std::option::Option<ClusterStatusSubstateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The cluster's state."]
pub enum ClusterStatusStateEnum {
    #[serde(rename = "UNKNOWN")]
    #[doc = "The cluster state is unknown."]
    Unknown,
    #[serde(rename = "CREATING")]
    #[doc = "The cluster is being created and set up. It is not ready for use."]
    Creating,
    #[serde(rename = "RUNNING")]
    #[doc = "The cluster is currently running and healthy. It is ready for use."]
    Running,
    #[serde(rename = "ERROR")]
    #[doc = "The cluster encountered an error. It is not ready for use."]
    Error,
    #[serde(rename = "DELETING")]
    #[doc = "The cluster is being deleted. It cannot be used."]
    Deleting,
    #[serde(rename = "UPDATING")]
    #[doc = "The cluster is being updated. It continues to accept and process jobs."]
    Updating,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. Additional state information that includes status reported by the agent."]
pub enum ClusterStatusSubstateEnum {
    #[serde(rename = "UNSPECIFIED")]
    #[doc = "The cluster substate is unknown."]
    Unspecified,
    #[serde(rename = "UNHEALTHY")]
    #[doc = "The cluster is known to be in an unhealthy state (for example, critical daemons are not running or HDFS capacity is exhausted).Applies to RUNNING state."]
    Unhealthy,
    #[serde(rename = "STALE_STATUS")]
    #[doc = "The agent-reported status is out of date (may occur if Dataproc loses communication with Agent).Applies to RUNNING state."]
    StaleStatus,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request to collect cluster diagnostic information."]
pub struct DiagnoseClusterRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The location of diagnostic output."]
pub struct DiagnoseClusterResults {
    #[serde(rename = "outputUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The Cloud Storage URI of the diagnostic output. The output report is a plain text file with a summary of collected diagnostics."]
    pub output_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies the config of disk options for a group of VM instances."]
pub struct DiskConfig {
    #[serde(rename = "bootDiskSizeGb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Size in GB of the boot disk (default is 500GB)."]
    pub boot_disk_size_gb: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "bootDiskType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Type of the boot disk (default is \"pd-standard\"). Valid values: \"pd-balanced\" (Persistent Disk Balanced Solid State Drive), \"pd-ssd\" (Persistent Disk Solid State Drive), or \"pd-standard\" (Persistent Disk Hard Disk Drive). See Disk types (https://cloud.google.com/compute/docs/disks#disk-types)."]
    pub boot_disk_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "numLocalSsds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Number of attached SSDs, from 0 to 4 (default is 0). If SSDs are not attached, the boot disk is used to store runtime logs and HDFS (https://hadoop.apache.org/docs/r1.2.1/hdfs_user_guide.html) data. If one or more SSDs are attached, this runtime bulk data is spread across them, and the boot disk contains only basic config and installed binaries."]
    pub num_local_ssds: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for Empty is empty JSON object {}."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Encryption settings for the cluster."]
pub struct EncryptionConfig {
    #[serde(rename = "gcePdKmsKeyName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The Cloud KMS key name to use for PD disk encryption for all instances in the cluster."]
    pub gce_pd_kms_key_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Endpoint config for this cluster"]
pub struct EndpointConfig {
    #[serde(rename = "enableHttpPortAccess")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If true, enable http access to specific ports on the cluster from external sources. Defaults to false."]
    pub enable_http_port_access: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "httpPorts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The map of port descriptions to URLs. Will only be populated if enable_http_port_access is true."]
    pub http_ports:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec.Example (Comparison): title: \"Summary size limit\" description: \"Determines if a summary is less than 100 chars\" expression: \"document.summary.size() < 100\" Example (Equality): title: \"Requestor is owner\" description: \"Determines if requestor is the document owner\" expression: \"document.owner == request.auth.claims.email\" Example (Logic): title: \"Public documents\" description: \"Determine whether the document should be publicly visible\" expression: \"document.type != 'private' && document.type != 'internal'\" Example (Data Manipulation): title: \"Notification string\" description: \"Create a notification string with a timestamp.\" expression: \"'New message received at ' + string(document.create_time)\" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information."]
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
#[doc = "Common config settings for resources of Compute Engine cluster instances, applicable to all instances in the cluster."]
pub struct GceClusterConfig {
    #[serde(rename = "internalIpOnly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If true, all instances in the cluster will only have internal IP addresses. By default, clusters are not restricted to internal IP addresses, and will have ephemeral external IP addresses assigned to each instance. This internal_ip_only restriction can only be enabled for subnetwork enabled networks, and all off-cluster dependencies must be configured to be accessible without external IP addresses."]
    pub internal_ip_only: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Compute Engine metadata entries to add to all instances (see Project and instance metadata (https://cloud.google.com/compute/docs/storing-retrieving-metadata#project_and_instance_metadata))."]
    pub metadata:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "networkUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The Compute Engine network to be used for machine communications. Cannot be specified with subnetwork_uri. If neither network_uri nor subnetwork_uri is specified, the \"default\" network of the project is used, if it exists. Cannot be a \"Custom Subnet Network\" (see Using Subnetworks (https://cloud.google.com/compute/docs/subnetworks) for more information).A full URL, partial URI, or short name are valid. Examples: https://www.googleapis.com/compute/v1/projects/[project_id]/regions/global/default projects/[project_id]/regions/global/default default"]
    pub network_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nodeGroupAffinity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Node Group Affinity for sole-tenant clusters."]
    pub node_group_affinity: ::std::option::Option<::std::boxed::Box<NodeGroupAffinity>>,
    #[serde(rename = "privateIpv6GoogleAccess")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The type of IPv6 access for a cluster."]
    pub private_ipv6_google_access:
        ::std::option::Option<GceClusterConfigPrivateIpv6GoogleAccessEnum>,
    #[serde(rename = "reservationAffinity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Reservation Affinity for consuming Zonal reservation."]
    pub reservation_affinity: ::std::option::Option<::std::boxed::Box<ReservationAffinity>>,
    #[serde(rename = "serviceAccount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The Dataproc service account (https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/service-accounts#service_accounts_in_dataproc) (also see VM Data Plane identity (https://cloud.google.com/dataproc/docs/concepts/iam/dataproc-principals#vm_service_account_data_plane_identity)) used by Dataproc cluster VM instances to access Google Cloud Platform services.If not specified, the Compute Engine default service account (https://cloud.google.com/compute/docs/access/service-accounts#default_service_account) is used."]
    pub service_account: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serviceAccountScopes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The URIs of service account scopes to be included in Compute Engine instances. The following base set of scopes is always included: https://www.googleapis.com/auth/cloud.useraccounts.readonly https://www.googleapis.com/auth/devstorage.read_write https://www.googleapis.com/auth/logging.writeIf no scopes are specified, the following defaults are also provided: https://www.googleapis.com/auth/bigquery https://www.googleapis.com/auth/bigtable.admin.table https://www.googleapis.com/auth/bigtable.data https://www.googleapis.com/auth/devstorage.full_control"]
    pub service_account_scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "shieldedInstanceConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Shielded Instance Config for clusters using Compute Engine Shielded VMs (https://cloud.google.com/security/shielded-cloud/shielded-vm)."]
    pub shielded_instance_config: ::std::option::Option<::std::boxed::Box<ShieldedInstanceConfig>>,
    #[serde(rename = "subnetworkUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The Compute Engine subnetwork to be used for machine communications. Cannot be specified with network_uri.A full URL, partial URI, or short name are valid. Examples: https://www.googleapis.com/compute/v1/projects/[project_id]/regions/us-east1/subnetworks/sub0 projects/[project_id]/regions/us-east1/subnetworks/sub0 sub0"]
    pub subnetwork_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Compute Engine tags to add to all instances (see Tagging instances (https://cloud.google.com/compute/docs/label-or-tag-resources#tags))."]
    pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "zoneUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The zone where the Compute Engine cluster will be located. On a create request, it is required in the \"global\" region. If omitted in a non-global Dataproc region, the service will pick a zone in the corresponding Compute Engine region. On a get request, zone will always be present.A full URL, partial URI, or short name are valid. Examples: https://www.googleapis.com/compute/v1/projects/[project_id]/zones/[zone] projects/[project_id]/zones/[zone] us-central1-f"]
    pub zone_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. The type of IPv6 access for a cluster."]
pub enum GceClusterConfigPrivateIpv6GoogleAccessEnum {
    #[serde(rename = "PRIVATE_IPV6_GOOGLE_ACCESS_UNSPECIFIED")]
    #[doc = "If unspecified, Compute Engine default behavior will apply, which is the same as INHERIT_FROM_SUBNETWORK."]
    PrivateIpv6GoogleAccessUnspecified,
    #[serde(rename = "INHERIT_FROM_SUBNETWORK")]
    #[doc = "Private access to and from Google Services configuration inherited from the subnetwork configuration. This is the default Compute Engine behavior."]
    InheritFromSubnetwork,
    #[serde(rename = "OUTBOUND")]
    #[doc = "Enables outbound private IPv6 access to Google Services from the Dataproc cluster."]
    Outbound,
    #[serde(rename = "BIDIRECTIONAL")]
    #[doc = "Enables bidirectional private IPv6 access between Google Services and the Dataproc cluster."]
    Bidirectional,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for GetIamPolicy method."]
pub struct GetIamPolicyRequest {
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OPTIONAL: A GetPolicyOptions object for specifying options to GetIamPolicy."]
    pub options: ::std::option::Option<::std::boxed::Box<GetPolicyOptions>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Encapsulates settings provided to GetIamPolicy."]
pub struct GetPolicyOptions {
    #[serde(rename = "requestedPolicyVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The policy format version to be returned.Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected.Requests for policies with any conditional bindings must specify version 3. Policies without any conditional bindings may specify any valid value or leave the field unset.To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies)."]
    pub requested_policy_version: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Dataproc job for running Apache Hadoop MapReduce (https://hadoop.apache.org/docs/current/hadoop-mapreduce-client/hadoop-mapreduce-client-core/MapReduceTutorial.html) jobs on Apache Hadoop YARN (https://hadoop.apache.org/docs/r2.7.1/hadoop-yarn/hadoop-yarn-site/YARN.html)."]
pub struct HadoopJob {
    #[serde(rename = "archiveUris")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. HCFS URIs of archives to be extracted in the working directory of Hadoop drivers and tasks. Supported file types: .jar, .tar, .tar.gz, .tgz, or .zip."]
    pub archive_uris: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "args")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The arguments to pass to the driver. Do not include arguments, such as -libjars or -Dfoo=bar, that can be set as job properties, since a collision may occur that causes an incorrect job submission."]
    pub args: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "fileUris")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. HCFS (Hadoop Compatible Filesystem) URIs of files to be copied to the working directory of Hadoop drivers and distributed tasks. Useful for naively parallel tasks."]
    pub file_uris: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "jarFileUris")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Jar file URIs to add to the CLASSPATHs of the Hadoop driver and tasks."]
    pub jar_file_uris: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "loggingConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The runtime log config for job execution."]
    pub logging_config: ::std::option::Option<::std::boxed::Box<LoggingConfig>>,
    #[serde(rename = "mainClass")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the driver's main class. The jar file containing the class must be in the default CLASSPATH or specified in jar_file_uris."]
    pub main_class: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mainJarFileUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The HCFS URI of the jar file containing the main class. Examples: 'gs://foo-bucket/analytics-binaries/extract-useful-metrics-mr.jar' 'hdfs:/tmp/test-samples/custom-wordcount.jar' 'file:///home/usr/lib/hadoop-mapreduce/hadoop-mapreduce-examples.jar'"]
    pub main_jar_file_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A mapping of property names to values, used to configure Hadoop. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/hadoop/conf/*-site and classes in user code."]
    pub properties:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Dataproc job for running Apache Hive (https://hive.apache.org/) queries on YARN."]
pub struct HiveJob {
    #[serde(rename = "continueOnFailure")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Whether to continue executing queries if a query fails. The default value is false. Setting to true can be useful when executing independent parallel queries."]
    pub continue_on_failure: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "jarFileUris")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. HCFS URIs of jar files to add to the CLASSPATH of the Hive server and Hadoop MapReduce (MR) tasks. Can contain Hive SerDes and UDFs."]
    pub jar_file_uris: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A mapping of property names and values, used to configure Hive. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/hadoop/conf/*-site.xml, /etc/hive/conf/hive-site.xml, and classes in user code."]
    pub properties:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "queryFileUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The HCFS URI of the script that contains Hive queries."]
    pub query_file_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "queryList")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of queries."]
    pub query_list: ::std::option::Option<::std::boxed::Box<QueryList>>,
    #[serde(rename = "scriptVariables")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Mapping of query variable names to values (equivalent to the Hive command: SET name=\"value\";)."]
    pub script_variables:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request to inject credentials into a cluster."]
pub struct InjectCredentialsRequest {
    #[serde(rename = "clusterUuid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The cluster UUID."]
    pub cluster_uuid: ::std::option::Option<::std::string::String>,
    #[serde(rename = "credentialsCiphertext")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The encrypted credentials being injected in to the cluster.The client is responsible for encrypting the credentials in a way that is supported by the cluster.A wrapped value is used here so that the actual contents of the encrypted credentials are not written to audit logs."]
    pub credentials_ciphertext: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for the size bounds of an instance group, including its proportional size to other groups."]
pub struct InstanceGroupAutoscalingPolicyConfig {
    #[serde(rename = "maxInstances")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Maximum number of instances for this group. Required for primary workers. Note that by default, clusters will not use secondary workers. Required for secondary workers if the minimum secondary instances is set.Primary workers - Bounds: [min_instances, ). Secondary workers - Bounds: [min_instances, ). Default: 0."]
    pub max_instances: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "minInstances")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Minimum number of instances for this group.Primary workers - Bounds: 2, max_instances. Default: 2. Secondary workers - Bounds: 0, max_instances. Default: 0."]
    pub min_instances: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "weight")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Weight for the instance group, which is used to determine the fraction of total workers in the cluster from this instance group. For example, if primary workers have weight 2, and secondary workers have weight 1, the cluster will have approximately 2 primary workers for each secondary worker.The cluster may not reach the specified balance if constrained by min/max bounds or other autoscaling settings. For example, if max_instances for secondary workers is 0, then only primary workers will be added. The cluster can also be out of balance when created.If weight is not set on any instance group, the cluster will default to equal weight for all groups: the cluster will attempt to maintain an equal number of workers in each group within the configured size bounds for each group. If weight is set for one group only, the cluster will default to zero weight on the unset group. For example if weight is set only on primary workers, the cluster will use primary workers only and no secondary workers."]
    pub weight: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The config settings for Compute Engine resources in an instance group, such as a master or worker group."]
pub struct InstanceGroupConfig {
    #[serde(rename = "accelerators")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The Compute Engine accelerator configuration for these instances."]
    pub accelerators: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AcceleratorConfig>>>,
    #[serde(rename = "diskConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Disk option config settings."]
    pub disk_config: ::std::option::Option<::std::boxed::Box<DiskConfig>>,
    #[serde(rename = "imageUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The Compute Engine image resource used for cluster instances.The URI can represent an image or image family.Image examples: https://www.googleapis.com/compute/beta/projects/[project_id]/global/images/[image-id] projects/[project_id]/global/images/[image-id] image-idImage family examples. Dataproc will use the most recent image from the family: https://www.googleapis.com/compute/beta/projects/[project_id]/global/images/family/[custom-image-family-name] projects/[project_id]/global/images/family/[custom-image-family-name]If the URI is unspecified, it will be inferred from SoftwareConfig.image_version or the system default."]
    pub image_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "instanceNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The list of instance names. Dataproc derives the names from cluster_name, num_instances, and the instance group."]
    pub instance_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "instanceReferences")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. List of references to Compute Engine instances."]
    pub instance_references:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<InstanceReference>>>,
    #[serde(rename = "isPreemptible")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Specifies that this instance group contains preemptible instances."]
    pub is_preemptible: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "machineTypeUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The Compute Engine machine type used for cluster instances.A full URL, partial URI, or short name are valid. Examples: https://www.googleapis.com/compute/v1/projects/[project_id]/zones/us-east1-a/machineTypes/n1-standard-2 projects/[project_id]/zones/us-east1-a/machineTypes/n1-standard-2 n1-standard-2Auto Zone Exception: If you are using the Dataproc Auto Zone Placement (https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/auto-zone#using_auto_zone_placement) feature, you must use the short name of the machine type resource, for example, n1-standard-2."]
    pub machine_type_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "managedGroupConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The config for Compute Engine Instance Group Manager that manages this group. This is only used for preemptible instance groups."]
    pub managed_group_config: ::std::option::Option<::std::boxed::Box<ManagedGroupConfig>>,
    #[serde(rename = "minCpuPlatform")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Specifies the minimum cpu platform for the Instance Group. See Dataproc -> Minimum CPU Platform (https://cloud.google.com/dataproc/docs/concepts/compute/dataproc-min-cpu)."]
    pub min_cpu_platform: ::std::option::Option<::std::string::String>,
    #[serde(rename = "numInstances")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The number of VM instances in the instance group. For master instance groups, must be set to 1."]
    pub num_instances: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "preemptibility")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Specifies the preemptibility of the instance group.The default value for master and worker groups is NON_PREEMPTIBLE. This default cannot be changed.The default value for secondary instances is PREEMPTIBLE."]
    pub preemptibility: ::std::option::Option<InstanceGroupConfigPreemptibilityEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. Specifies the preemptibility of the instance group.The default value for master and worker groups is NON_PREEMPTIBLE. This default cannot be changed.The default value for secondary instances is PREEMPTIBLE."]
pub enum InstanceGroupConfigPreemptibilityEnum {
    #[serde(rename = "PREEMPTIBILITY_UNSPECIFIED")]
    #[doc = "Preemptibility is unspecified, the system will choose the appropriate setting for each instance group."]
    PreemptibilityUnspecified,
    #[serde(rename = "NON_PREEMPTIBLE")]
    #[doc = "Instances are non-preemptible.This option is allowed for all instance groups and is the only valid value for Master and Worker instance groups."]
    NonPreemptible,
    #[serde(rename = "PREEMPTIBLE")]
    #[doc = "Instances are preemptible.This option is allowed only for secondary worker groups."]
    Preemptible,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A reference to a Compute Engine instance."]
pub struct InstanceReference {
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier of the Compute Engine instance."]
    pub instance_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "instanceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user-friendly name of the Compute Engine instance."]
    pub instance_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publicKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The public key used for sharing data with this instance."]
    pub public_key: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request to instantiate a workflow template."]
pub struct InstantiateWorkflowTemplateRequest {
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Map from parameter names to values that should be used for those parameters. Values may not exceed 1000 characters."]
    pub parameters:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A tag that prevents multiple concurrent workflow instances with the same tag from running. This mitigates risk of concurrent instances started due to retries.It is recommended to always set this value to a UUID (https://en.wikipedia.org/wiki/Universally_unique_identifier).The tag must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). The maximum length is 40 characters."]
    pub request_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The version of workflow template to instantiate. If specified, the workflow will be instantiated only if the current version of the workflow template has the supplied version.This option cannot be used to instantiate a previous version of workflow template."]
    pub version: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Dataproc job resource."]
pub struct Job {
    #[serde(rename = "done")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Indicates whether the job is completed. If the value is false, the job is still in progress. If true, the job is completed, and status.state field will indicate if it was successful, failed, or cancelled."]
    pub done: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "driverControlFilesUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. If present, the location of miscellaneous control files which may be used as part of job setup and handling. If not present, control files may be placed in the same location as driver_output_uri."]
    pub driver_control_files_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "driverOutputResourceUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A URI pointing to the location of the stdout of the job's driver program."]
    pub driver_output_resource_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hadoopJob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Job is a Hadoop job."]
    pub hadoop_job: ::std::option::Option<::std::boxed::Box<HadoopJob>>,
    #[serde(rename = "hiveJob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Job is a Hive job."]
    pub hive_job: ::std::option::Option<::std::boxed::Box<HiveJob>>,
    #[serde(rename = "jobUuid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A UUID that uniquely identifies a job within the project over time. This is in contrast to a user-settable reference.job_id that may be reused over time."]
    pub job_uuid: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The labels to associate with this job. Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). Label values may be empty, but, if present, must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with a job."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "pigJob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Job is a Pig job."]
    pub pig_job: ::std::option::Option<::std::boxed::Box<PigJob>>,
    #[serde(rename = "placement")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Job information, including how, when, and where to run the job."]
    pub placement: ::std::option::Option<::std::boxed::Box<JobPlacement>>,
    #[serde(rename = "prestoJob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Job is a Presto job."]
    pub presto_job: ::std::option::Option<::std::boxed::Box<PrestoJob>>,
    #[serde(rename = "pysparkJob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Job is a PySpark job."]
    pub pyspark_job: ::std::option::Option<::std::boxed::Box<PySparkJob>>,
    #[serde(rename = "reference")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The fully qualified reference to the job, which can be used to obtain the equivalent REST path of the job resource. If this property is not specified when a job is created, the server generates a job_id."]
    pub reference: ::std::option::Option<::std::boxed::Box<JobReference>>,
    #[serde(rename = "scheduling")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Job scheduling configuration."]
    pub scheduling: ::std::option::Option<::std::boxed::Box<JobScheduling>>,
    #[serde(rename = "sparkJob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Job is a Spark job."]
    pub spark_job: ::std::option::Option<::std::boxed::Box<SparkJob>>,
    #[serde(rename = "sparkRJob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Job is a SparkR job."]
    pub spark_r_job: ::std::option::Option<::std::boxed::Box<SparkRJob>>,
    #[serde(rename = "sparkSqlJob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Job is a SparkSql job."]
    pub spark_sql_job: ::std::option::Option<::std::boxed::Box<SparkSqlJob>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The job status. Additional application-specific status information may be contained in the type_job and yarn_applications fields."]
    pub status: ::std::option::Option<::std::boxed::Box<JobStatus>>,
    #[serde(rename = "statusHistory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The previous job status."]
    pub status_history: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<JobStatus>>>,
    #[serde(rename = "yarnApplications")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The collection of YARN applications spun up by this job.Beta Feature: This report is available for testing purposes only. It may be changed before final release."]
    pub yarn_applications:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<YarnApplication>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Job Operation metadata."]
pub struct JobMetadata {
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The job id."]
    pub job_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Operation type."]
    pub operation_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Job submission time."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Most recent job status."]
    pub status: ::std::option::Option<::std::boxed::Box<JobStatus>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Dataproc job config."]
pub struct JobPlacement {
    #[serde(rename = "clusterLabels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Cluster labels to identify a cluster where the job will be submitted."]
    pub cluster_labels:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "clusterName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the cluster where the job will be submitted."]
    pub cluster_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "clusterUuid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A cluster UUID generated by the Dataproc service when the job is submitted."]
    pub cluster_uuid: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Encapsulates the full scoping used to reference a job."]
pub struct JobReference {
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The job ID, which must be unique within the project.The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), or hyphens (-). The maximum length is 100 characters.If not specified by the caller, the job ID will be provided by the server."]
    pub job_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The ID of the Google Cloud Platform project that the job belongs to. If specified, must match the request project ID."]
    pub project_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Job scheduling options."]
pub struct JobScheduling {
    #[serde(rename = "maxFailuresPerHour")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Maximum number of times per hour a driver may be restarted as a result of driver exiting with non-zero code before job is reported failed.A job may be reported as thrashing if driver exits with non-zero code 4 times within 10 minute window.Maximum value is 10."]
    pub max_failures_per_hour: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "maxFailuresTotal")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Maximum number of times in total a driver may be restarted as a result of driver exiting with non-zero code before job is reported failed. Maximum value is 240."]
    pub max_failures_total: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Dataproc job status."]
pub struct JobStatus {
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Output only. Job state details, such as an error description if the state is ERROR."]
    pub details: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A state message specifying the overall job state."]
    pub state: ::std::option::Option<JobStatusStateEnum>,
    #[serde(rename = "stateStartTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time when this state was entered."]
    pub state_start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "substate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Additional state information, which includes status reported by the agent."]
    pub substate: ::std::option::Option<JobStatusSubstateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. A state message specifying the overall job state."]
pub enum JobStatusStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "The job state is unknown."]
    StateUnspecified,
    #[serde(rename = "PENDING")]
    #[doc = "The job is pending; it has been submitted, but is not yet running."]
    Pending,
    #[serde(rename = "SETUP_DONE")]
    #[doc = "Job has been received by the service and completed initial setup; it will soon be submitted to the cluster."]
    SetupDone,
    #[serde(rename = "RUNNING")]
    #[doc = "The job is running on the cluster."]
    Running,
    #[serde(rename = "CANCEL_PENDING")]
    #[doc = "A CancelJob request has been received, but is pending."]
    CancelPending,
    #[serde(rename = "CANCEL_STARTED")]
    #[doc = "Transient in-flight resources have been canceled, and the request to cancel the running job has been issued to the cluster."]
    CancelStarted,
    #[serde(rename = "CANCELLED")]
    #[doc = "The job cancellation was successful."]
    Cancelled,
    #[serde(rename = "DONE")]
    #[doc = "The job has completed successfully."]
    Done,
    #[serde(rename = "ERROR")]
    #[doc = "The job has completed, but encountered an error."]
    Error,
    #[serde(rename = "ATTEMPT_FAILURE")]
    #[doc = "Job attempt has failed. The detail field contains failure details for this attempt.Applies to restartable jobs only."]
    AttemptFailure,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. Additional state information, which includes status reported by the agent."]
pub enum JobStatusSubstateEnum {
    #[serde(rename = "UNSPECIFIED")]
    #[doc = "The job substate is unknown."]
    Unspecified,
    #[serde(rename = "SUBMITTED")]
    #[doc = "The Job is submitted to the agent.Applies to RUNNING state."]
    Submitted,
    #[serde(rename = "QUEUED")]
    #[doc = "The Job has been received and is awaiting execution (it may be waiting for a condition to be met). See the \"details\" field for the reason for the delay.Applies to RUNNING state."]
    Queued,
    #[serde(rename = "STALE_STATUS")]
    #[doc = "The agent-reported status is out of date, which may be caused by a loss of communication between the agent and Dataproc. If the agent does not send a timely update, the job will fail.Applies to RUNNING state."]
    StaleStatus,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies Kerberos related configuration."]
pub struct KerberosConfig {
    #[serde(rename = "crossRealmTrustAdminServer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The admin server (IP or hostname) for the remote trusted realm in a cross realm trust relationship."]
    pub cross_realm_trust_admin_server: ::std::option::Option<::std::string::String>,
    #[serde(rename = "crossRealmTrustKdc")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The KDC (IP or hostname) for the remote trusted realm in a cross realm trust relationship."]
    pub cross_realm_trust_kdc: ::std::option::Option<::std::string::String>,
    #[serde(rename = "crossRealmTrustRealm")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The remote realm the Dataproc on-cluster KDC will trust, should the user enable cross realm trust."]
    pub cross_realm_trust_realm: ::std::option::Option<::std::string::String>,
    #[serde(rename = "crossRealmTrustSharedPasswordUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The Cloud Storage URI of a KMS encrypted file containing the shared password between the on-cluster Kerberos realm and the remote trusted realm, in a cross realm trust relationship."]
    pub cross_realm_trust_shared_password_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enableKerberos")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Flag to indicate whether to Kerberize the cluster (default: false). Set this field to true to enable Kerberos on a cluster."]
    pub enable_kerberos: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kdcDbKeyUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The Cloud Storage URI of a KMS encrypted file containing the master key of the KDC database."]
    pub kdc_db_key_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "keyPasswordUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The Cloud Storage URI of a KMS encrypted file containing the password to the user provided key. For the self-signed certificate, this password is generated by Dataproc."]
    pub key_password_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "keystorePasswordUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The Cloud Storage URI of a KMS encrypted file containing the password to the user provided keystore. For the self-signed certificate, this password is generated by Dataproc."]
    pub keystore_password_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "keystoreUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The Cloud Storage URI of the keystore file used for SSL encryption. If not provided, Dataproc will provide a self-signed certificate."]
    pub keystore_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kmsKeyUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The uri of the KMS key used to encrypt various sensitive files."]
    pub kms_key_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "realm")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The name of the on-cluster Kerberos realm. If not specified, the uppercased domain of hostnames will be the realm."]
    pub realm: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rootPrincipalPasswordUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The Cloud Storage URI of a KMS encrypted file containing the root principal password."]
    pub root_principal_password_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tgtLifetimeHours")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The lifetime of the ticket granting ticket, in hours. If not specified, or user specifies 0, then default value 10 will be used."]
    pub tgt_lifetime_hours: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "truststorePasswordUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The Cloud Storage URI of a KMS encrypted file containing the password to the user provided truststore. For the self-signed certificate, this password is generated by Dataproc."]
    pub truststore_password_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "truststoreUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The Cloud Storage URI of the truststore file used for SSL encryption. If not provided, Dataproc will provide a self-signed certificate."]
    pub truststore_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies the cluster auto-delete schedule configuration."]
pub struct LifecycleConfig {
    #[serde(rename = "autoDeleteTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The time when cluster will be auto-deleted (see JSON representation of Timestamp (https://developers.google.com/protocol-buffers/docs/proto3#json))."]
    pub auto_delete_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "autoDeleteTtl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The lifetime duration of cluster. The cluster will be auto-deleted at the end of this period. Minimum value is 10 minutes; maximum value is 14 days (see JSON representation of Duration (https://developers.google.com/protocol-buffers/docs/proto3#json))."]
    pub auto_delete_ttl: ::std::option::Option<::std::string::String>,
    #[serde(rename = "idleDeleteTtl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The duration to keep the cluster alive while idling (when no jobs are running). Passing this threshold will cause the cluster to be deleted. Minimum value is 5 minutes; maximum value is 14 days (see JSON representation of Duration (https://developers.google.com/protocol-buffers/docs/proto3#json))."]
    pub idle_delete_ttl: ::std::option::Option<::std::string::String>,
    #[serde(rename = "idleStartTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time when cluster became idle (most recent job finished) and became eligible for deletion due to idleness (see JSON representation of Timestamp (https://developers.google.com/protocol-buffers/docs/proto3#json))."]
    pub idle_start_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A response to a request to list autoscaling policies in a project."]
pub struct ListAutoscalingPoliciesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. This token is included in the response if there are more results to fetch."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "policies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Autoscaling policies list."]
    pub policies: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AutoscalingPolicy>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The list of all clusters in a project."]
pub struct ListClustersResponse {
    #[serde(rename = "clusters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The clusters in the project."]
    pub clusters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Cluster>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. This token is included in the response if there are more results to fetch. To fetch additional results, provide this value as the page_token in a subsequent ListClustersRequest."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of jobs in a project."]
pub struct ListJobsResponse {
    #[serde(rename = "jobs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Jobs list."]
    pub jobs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Job>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. This token is included in the response if there are more results to fetch. To fetch additional results, provide this value as the page_token in a subsequent ListJobsRequest."]
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
#[doc = "A response to a request to list workflow templates in a project."]
pub struct ListWorkflowTemplatesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. This token is included in the response if there are more results to fetch. To fetch additional results, provide this value as the page_token in a subsequent ListWorkflowTemplatesRequest."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "templates")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. WorkflowTemplates list."]
    pub templates: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WorkflowTemplate>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The runtime logging config of the job."]
pub struct LoggingConfig {
    #[serde(rename = "driverLogLevels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The per-package log levels for the driver. This may include \"root\" package name to configure rootLogger. Examples: 'com.google = FATAL', 'root = INFO', 'org.apache = DEBUG'"]
    pub driver_log_levels: ::std::option::Option<
        ::std::collections::BTreeMap<String, LoggingConfigDriverLogLevelsEnum>,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum LoggingConfigDriverLogLevelsEnum {
    #[serde(rename = "LEVEL_UNSPECIFIED")]
    #[doc = "Level is unspecified. Use default level for log4j."]
    LevelUnspecified,
    #[serde(rename = "ALL")]
    #[doc = "Use ALL level for log4j."]
    All,
    #[serde(rename = "TRACE")]
    #[doc = "Use TRACE level for log4j."]
    Trace,
    #[serde(rename = "DEBUG")]
    #[doc = "Use DEBUG level for log4j."]
    Debug,
    #[serde(rename = "INFO")]
    #[doc = "Use INFO level for log4j."]
    Info,
    #[serde(rename = "WARN")]
    #[doc = "Use WARN level for log4j."]
    Warn,
    #[serde(rename = "ERROR")]
    #[doc = "Use ERROR level for log4j."]
    Error,
    #[serde(rename = "FATAL")]
    #[doc = "Use FATAL level for log4j."]
    Fatal,
    #[serde(rename = "OFF")]
    #[doc = "Turn off log4j."]
    Off,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Cluster that is managed by the workflow."]
pub struct ManagedCluster {
    #[serde(rename = "clusterName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The cluster name prefix. A unique cluster name will be formed by appending a random suffix.The name must contain only lower-case letters (a-z), numbers (0-9), and hyphens (-). Must begin with a letter. Cannot begin or end with hyphen. Must consist of between 2 and 35 characters."]
    pub cluster_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The cluster configuration."]
    pub config: ::std::option::Option<::std::boxed::Box<ClusterConfig>>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The labels to associate with this cluster.Label keys must be between 1 and 63 characters long, and must conform to the following PCRE regular expression: \\p{Ll}\\p{Lo}{0,62}Label values must be between 1 and 63 characters long, and must conform to the following PCRE regular expression: \\p{Ll}\\p{Lo}\\p{N}_-{0,63}No more than 32 labels can be associated with a given cluster."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies the resources used to actively manage an instance group."]
pub struct ManagedGroupConfig {
    #[serde(rename = "instanceGroupManagerName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The name of the Instance Group Manager for this group."]
    pub instance_group_manager_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "instanceTemplateName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The name of the Instance Template used for the Managed Instance Group."]
    pub instance_template_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies a Metastore configuration."]
pub struct MetastoreConfig {
    #[serde(rename = "dataprocMetastoreService")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Resource name of an existing Dataproc Metastore service.Example: projects/[project_id]/locations/[dataproc_region]/services/[service-name]"]
    pub dataproc_metastore_service: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Node Group Affinity for clusters using sole-tenant node groups."]
pub struct NodeGroupAffinity {
    #[serde(rename = "nodeGroupUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The URI of a sole-tenant node group resource (https://cloud.google.com/compute/docs/reference/rest/v1/nodeGroups) that the cluster will be created on.A full URL, partial URI, or node group name are valid. Examples: https://www.googleapis.com/compute/v1/projects/[project_id]/zones/us-central1-a/nodeGroups/node-group-1 projects/[project_id]/zones/us-central1-a/nodeGroups/node-group-1 node-group-1"]
    pub node_group_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies an executable to run on a fully configured node and a timeout period for executable completion."]
pub struct NodeInitializationAction {
    #[serde(rename = "executableFile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Cloud Storage URI of executable file."]
    pub executable_file: ::std::option::Option<::std::string::String>,
    #[serde(rename = "executionTimeout")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Amount of time executable has to complete. Default is 10 minutes (see JSON representation of Duration (https://developers.google.com/protocol-buffers/docs/proto3#json)).Cluster creation fails with an explanatory error message (the name of the executable that caused the error and the exceeded timeout period) if the executable is not completed at end of the timeout period."]
    pub execution_timeout: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This resource represents a long-running operation that is the result of a network API call."]
pub struct Operation {
    #[serde(rename = "done")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available."]
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
    #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the name should be a resource name ending with operations/{unique_id}."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "response")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The normal response of the operation in case of success. If the original method returns no data on success, such as Delete, the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type XxxResponse, where Xxx is the original method name. For example, if the original method name is TakeSnapshot(), the inferred response type is TakeSnapshotResponse."]
    pub response: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A job executed by the workflow."]
pub struct OrderedJob {
    #[serde(rename = "hadoopJob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Job is a Hadoop job."]
    pub hadoop_job: ::std::option::Option<::std::boxed::Box<HadoopJob>>,
    #[serde(rename = "hiveJob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Job is a Hive job."]
    pub hive_job: ::std::option::Option<::std::boxed::Box<HiveJob>>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The labels to associate with this job.Label keys must be between 1 and 63 characters long, and must conform to the following regular expression: \\p{Ll}\\p{Lo}{0,62}Label values must be between 1 and 63 characters long, and must conform to the following regular expression: \\p{Ll}\\p{Lo}\\p{N}_-{0,63}No more than 32 labels can be associated with a given job."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "pigJob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Job is a Pig job."]
    pub pig_job: ::std::option::Option<::std::boxed::Box<PigJob>>,
    #[serde(rename = "prerequisiteStepIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The optional list of prerequisite job step_ids. If not specified, the job will start at the beginning of workflow."]
    pub prerequisite_step_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "prestoJob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Job is a Presto job."]
    pub presto_job: ::std::option::Option<::std::boxed::Box<PrestoJob>>,
    #[serde(rename = "pysparkJob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Job is a PySpark job."]
    pub pyspark_job: ::std::option::Option<::std::boxed::Box<PySparkJob>>,
    #[serde(rename = "scheduling")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Job scheduling configuration."]
    pub scheduling: ::std::option::Option<::std::boxed::Box<JobScheduling>>,
    #[serde(rename = "sparkJob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Job is a Spark job."]
    pub spark_job: ::std::option::Option<::std::boxed::Box<SparkJob>>,
    #[serde(rename = "sparkRJob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Job is a SparkR job."]
    pub spark_r_job: ::std::option::Option<::std::boxed::Box<SparkRJob>>,
    #[serde(rename = "sparkSqlJob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Job is a SparkSql job."]
    pub spark_sql_job: ::std::option::Option<::std::boxed::Box<SparkSqlJob>>,
    #[serde(rename = "stepId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The step id. The id must be unique among all jobs within the template.The step id is used as prefix for job id, as job goog-dataproc-workflow-step-id label, and in prerequisiteStepIds field from other steps.The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between 3 and 50 characters."]
    pub step_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration for parameter validation."]
pub struct ParameterValidation {
    #[serde(rename = "regex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Validation based on regular expressions."]
    pub regex: ::std::option::Option<::std::boxed::Box<RegexValidation>>,
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Validation based on a list of allowed values."]
    pub values: ::std::option::Option<::std::boxed::Box<ValueValidation>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Dataproc job for running Apache Pig (https://pig.apache.org/) queries on YARN."]
pub struct PigJob {
    #[serde(rename = "continueOnFailure")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Whether to continue executing queries if a query fails. The default value is false. Setting to true can be useful when executing independent parallel queries."]
    pub continue_on_failure: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "jarFileUris")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. HCFS URIs of jar files to add to the CLASSPATH of the Pig Client and Hadoop MapReduce (MR) tasks. Can contain Pig UDFs."]
    pub jar_file_uris: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "loggingConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The runtime log config for job execution."]
    pub logging_config: ::std::option::Option<::std::boxed::Box<LoggingConfig>>,
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A mapping of property names to values, used to configure Pig. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/hadoop/conf/*-site.xml, /etc/pig/conf/pig.properties, and classes in user code."]
    pub properties:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "queryFileUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The HCFS URI of the script that contains the Pig queries."]
    pub query_file_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "queryList")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of queries."]
    pub query_list: ::std::option::Option<::std::boxed::Box<QueryList>>,
    #[serde(rename = "scriptVariables")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Mapping of query variable names to values (equivalent to the Pig command: name=[value])."]
    pub script_variables:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources.A Policy is a collection of bindings. A binding binds one or more members to a single role. Members can be user accounts, service accounts, Google groups, and domains (such as G Suite). A role is a named list of permissions; each role can be an IAM predefined role or a user-created custom role.For some types of Google Cloud resources, a binding can also specify a condition, which is a logical expression that allows access to a resource only if the expression evaluates to true. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies).JSON example: { \"bindings\": [ { \"role\": \"roles/resourcemanager.organizationAdmin\", \"members\": [ \"user:mike@example.com\", \"group:admins@example.com\", \"domain:google.com\", \"serviceAccount:my-project-id@appspot.gserviceaccount.com\" ] }, { \"role\": \"roles/resourcemanager.organizationViewer\", \"members\": [ \"user:eve@example.com\" ], \"condition\": { \"title\": \"expirable access\", \"description\": \"Does not grant access after Sep 2020\", \"expression\": \"request.time < timestamp('2020-10-01T00:00:00.000Z')\", } } ], \"etag\": \"BwWWja0YfJA=\", \"version\": 3 } YAML example: bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') - etag: BwWWja0YfJA= - version: 3 For a description of IAM and its features, see the IAM documentation (https://cloud.google.com/iam/docs/)."]
pub struct Policy {
    #[serde(rename = "bindings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Associates a list of members to a role. Optionally, may specify a condition that determines how and when the bindings are applied. Each of the bindings must contain at least one member."]
    pub bindings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Binding>>>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the etag in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An etag is returned in the response to getIamPolicy, and systems are expected to put that etag in the request to setIamPolicy to ensure that their change will be applied to the same version of the policy.Important: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the format of the policy.Valid values are 0, 1, and 3. Requests that specify an invalid value are rejected.Any operation that affects conditional role bindings must specify version 3. This requirement applies to the following operations: Getting a policy that includes a conditional role binding Adding a conditional role binding to a policy Changing a conditional role binding in a policy Removing any role binding, with or without a condition, from a policy that includes conditionsImportant: If you use IAM Conditions, you must include the etag field whenever you call setIamPolicy. If you omit this field, then IAM allows you to overwrite a version 3 policy with a version 1 policy, and all of the conditions in the version 3 policy are lost.If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset.To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies)."]
    pub version: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Dataproc job for running Presto (https://prestosql.io/) queries. IMPORTANT: The Dataproc Presto Optional Component (https://cloud.google.com/dataproc/docs/concepts/components/presto) must be enabled when the cluster is created to submit a Presto job to the cluster."]
pub struct PrestoJob {
    #[serde(rename = "clientTags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Presto client tags to attach to this query"]
    pub client_tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "continueOnFailure")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Whether to continue executing queries if a query fails. The default value is false. Setting to true can be useful when executing independent parallel queries."]
    pub continue_on_failure: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "loggingConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The runtime log config for job execution."]
    pub logging_config: ::std::option::Option<::std::boxed::Box<LoggingConfig>>,
    #[serde(rename = "outputFormat")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The format in which query output will be displayed. See the Presto documentation for supported output formats"]
    pub output_format: ::std::option::Option<::std::string::String>,
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A mapping of property names to values. Used to set Presto session properties (https://prestodb.io/docs/current/sql/set-session.html) Equivalent to using the --session flag in the Presto CLI"]
    pub properties:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "queryFileUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The HCFS URI of the script that contains SQL queries."]
    pub query_file_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "queryList")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of queries."]
    pub query_list: ::std::option::Option<::std::boxed::Box<QueryList>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Dataproc job for running Apache PySpark (https://spark.apache.org/docs/0.9.0/python-programming-guide.html) applications on YARN."]
pub struct PySparkJob {
    #[serde(rename = "archiveUris")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. HCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub archive_uris: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "args")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The arguments to pass to the driver. Do not include arguments, such as --conf, that can be set as job properties, since a collision may occur that causes an incorrect job submission."]
    pub args: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "fileUris")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. HCFS URIs of files to be placed in the working directory of each executor. Useful for naively parallel tasks."]
    pub file_uris: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "jarFileUris")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. HCFS URIs of jar files to add to the CLASSPATHs of the Python driver and tasks."]
    pub jar_file_uris: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "loggingConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The runtime log config for job execution."]
    pub logging_config: ::std::option::Option<::std::boxed::Box<LoggingConfig>>,
    #[serde(rename = "mainPythonFileUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The HCFS URI of the main Python file to use as the driver. Must be a .py file."]
    pub main_python_file_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A mapping of property names to values, used to configure PySpark. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code."]
    pub properties:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "pythonFileUris")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. HCFS file URIs of Python files to pass to the PySpark framework. Supported file types: .py, .egg, and .zip."]
    pub python_file_uris: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of queries to run on a cluster."]
pub struct QueryList {
    #[serde(rename = "queries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The queries to execute. You do not need to end a query expression with a semicolon. Multiple queries can be specified in one string by separating each with a semicolon. Here is an example of a Dataproc API snippet that uses a QueryList to specify a HiveJob: \"hiveJob\": { \"queryList\": { \"queries\": [ \"query1\", \"query2\", \"query3;query4\", ] } } "]
    pub queries: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Validation based on regular expressions."]
pub struct RegexValidation {
    #[serde(rename = "regexes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. RE2 regular expressions used to validate the parameter's value. The value must match the regex in its entirety (substring matches are not sufficient)."]
    pub regexes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Reservation Affinity for consuming Zonal reservation."]
pub struct ReservationAffinity {
    #[serde(rename = "consumeReservationType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Type of reservation to consume"]
    pub consume_reservation_type:
        ::std::option::Option<ReservationAffinityConsumeReservationTypeEnum>,
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Corresponds to the label key of reservation resource."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Corresponds to the label values of reservation resource."]
    pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. Type of reservation to consume"]
pub enum ReservationAffinityConsumeReservationTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = ""]
    TypeUnspecified,
    #[serde(rename = "NO_RESERVATION")]
    #[doc = "Do not consume from any allocated capacity."]
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
#[doc = "Security related configuration, including encryption, Kerberos, etc."]
pub struct SecurityConfig {
    #[serde(rename = "kerberosConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Kerberos related configuration."]
    pub kerberos_config: ::std::option::Option<::std::boxed::Box<KerberosConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for SetIamPolicy method."]
pub struct SetIamPolicyRequest {
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "REQUIRED: The complete policy to be applied to the resource. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Cloud Platform services (such as Projects) might reject them."]
    pub policy: ::std::option::Option<::std::boxed::Box<Policy>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Shielded Instance Config for clusters using Compute Engine Shielded VMs (https://cloud.google.com/security/shielded-cloud/shielded-vm)."]
pub struct ShieldedInstanceConfig {
    #[serde(rename = "enableIntegrityMonitoring")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Defines whether instances have integrity monitoring enabled."]
    pub enable_integrity_monitoring: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "enableSecureBoot")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Defines whether instances have Secure Boot enabled."]
    pub enable_secure_boot: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "enableVtpm")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Defines whether instances have the vTPM enabled."]
    pub enable_vtpm: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies the selection and config of software inside the cluster."]
pub struct SoftwareConfig {
    #[serde(rename = "imageVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The version of software inside the cluster. It must be one of the supported Dataproc Versions (https://cloud.google.com/dataproc/docs/concepts/versioning/dataproc-versions#supported_dataproc_versions), such as \"1.2\" (including a subminor version, such as \"1.2.29\"), or the \"preview\" version (https://cloud.google.com/dataproc/docs/concepts/versioning/dataproc-versions#other_versions). If unspecified, it defaults to the latest Debian version."]
    pub image_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "optionalComponents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The set of components to activate on the cluster."]
    pub optional_components:
        ::std::option::Option<::std::vec::Vec<SoftwareConfigOptionalComponentsEnum>>,
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The properties to set on daemon config files.Property keys are specified in prefix:property format, for example core:hadoop.tmp.dir. The following are supported prefixes and their mappings: capacity-scheduler: capacity-scheduler.xml core: core-site.xml distcp: distcp-default.xml hdfs: hdfs-site.xml hive: hive-site.xml mapred: mapred-site.xml pig: pig.properties spark: spark-defaults.conf yarn: yarn-site.xmlFor more information, see Cluster properties (https://cloud.google.com/dataproc/docs/concepts/cluster-properties)."]
    pub properties:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum SoftwareConfigOptionalComponentsEnum {
    #[serde(rename = "COMPONENT_UNSPECIFIED")]
    #[doc = "Unspecified component. Specifying this will cause Cluster creation to fail."]
    ComponentUnspecified,
    #[serde(rename = "ANACONDA")]
    #[doc = "The Anaconda python distribution. The Anaconda component is not supported in the Dataproc 2.0 image. The 2.0 image is pre-installed with Miniconda."]
    Anaconda,
    #[serde(rename = "DOCKER")]
    #[doc = "Docker"]
    Docker,
    #[serde(rename = "FLINK")]
    #[doc = "Flink"]
    Flink,
    #[serde(rename = "HIVE_WEBHCAT")]
    #[doc = "The Hive Web HCatalog (the REST service for accessing HCatalog)."]
    HiveWebhcat,
    #[serde(rename = "JUPYTER")]
    #[doc = "The Jupyter Notebook."]
    Jupyter,
    #[serde(rename = "PRESTO")]
    #[doc = "The Presto query engine."]
    Presto,
    #[serde(rename = "RANGER")]
    #[doc = "The Ranger service."]
    Ranger,
    #[serde(rename = "SOLR")]
    #[doc = "The Solr service."]
    Solr,
    #[serde(rename = "ZEPPELIN")]
    #[doc = "The Zeppelin notebook."]
    Zeppelin,
    #[serde(rename = "ZOOKEEPER")]
    #[doc = "The Zookeeper service."]
    Zookeeper,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Dataproc job for running Apache Spark (http://spark.apache.org/) applications on YARN."]
pub struct SparkJob {
    #[serde(rename = "archiveUris")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. HCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub archive_uris: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "args")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The arguments to pass to the driver. Do not include arguments, such as --conf, that can be set as job properties, since a collision may occur that causes an incorrect job submission."]
    pub args: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "fileUris")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. HCFS URIs of files to be placed in the working directory of each executor. Useful for naively parallel tasks."]
    pub file_uris: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "jarFileUris")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. HCFS URIs of jar files to add to the CLASSPATHs of the Spark driver and tasks."]
    pub jar_file_uris: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "loggingConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The runtime log config for job execution."]
    pub logging_config: ::std::option::Option<::std::boxed::Box<LoggingConfig>>,
    #[serde(rename = "mainClass")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the driver's main class. The jar file that contains the class must be in the default CLASSPATH or specified in jar_file_uris."]
    pub main_class: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mainJarFileUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The HCFS URI of the jar file that contains the main class."]
    pub main_jar_file_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A mapping of property names to values, used to configure Spark. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code."]
    pub properties:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Dataproc job for running Apache SparkR (https://spark.apache.org/docs/latest/sparkr.html) applications on YARN."]
pub struct SparkRJob {
    #[serde(rename = "archiveUris")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. HCFS URIs of archives to be extracted into the working directory of each executor. Supported file types: .jar, .tar, .tar.gz, .tgz, and .zip."]
    pub archive_uris: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "args")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The arguments to pass to the driver. Do not include arguments, such as --conf, that can be set as job properties, since a collision may occur that causes an incorrect job submission."]
    pub args: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "fileUris")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. HCFS URIs of files to be placed in the working directory of each executor. Useful for naively parallel tasks."]
    pub file_uris: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "loggingConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The runtime log config for job execution."]
    pub logging_config: ::std::option::Option<::std::boxed::Box<LoggingConfig>>,
    #[serde(rename = "mainRFileUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The HCFS URI of the main R file to use as the driver. Must be a .R file."]
    pub main_r_file_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A mapping of property names to values, used to configure SparkR. Properties that conflict with values set by the Dataproc API may be overwritten. Can include properties set in /etc/spark/conf/spark-defaults.conf and classes in user code."]
    pub properties:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Dataproc job for running Apache Spark SQL (http://spark.apache.org/sql/) queries."]
pub struct SparkSqlJob {
    #[serde(rename = "jarFileUris")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. HCFS URIs of jar files to be added to the Spark CLASSPATH."]
    pub jar_file_uris: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "loggingConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The runtime log config for job execution."]
    pub logging_config: ::std::option::Option<::std::boxed::Box<LoggingConfig>>,
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A mapping of property names to values, used to configure Spark SQL's SparkConf. Properties that conflict with values set by the Dataproc API may be overwritten."]
    pub properties:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "queryFileUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The HCFS URI of the script that contains SQL queries."]
    pub query_file_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "queryList")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of queries."]
    pub query_list: ::std::option::Option<::std::boxed::Box<QueryList>>,
    #[serde(rename = "scriptVariables")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Mapping of query variable names to values (equivalent to the Spark SQL command: SET name=\"value\";)."]
    pub script_variables:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request to start a cluster."]
pub struct StartClusterRequest {
    #[serde(rename = "clusterUuid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Specifying the cluster_uuid means the RPC will fail (with error NOT_FOUND) if a cluster with the specified UUID does not exist."]
    pub cluster_uuid: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A unique id used to identify the request. If the server receives two StartClusterRequest (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#google.cloud.dataproc.v1.StartClusterRequest)s with the same id, then the second request will be ignored and the first google.longrunning.Operation created and stored in the backend is returned.Recommendation: Set this value to a UUID (https://en.wikipedia.org/wiki/Universally_unique_identifier).The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). The maximum length is 40 characters."]
    pub request_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The Status type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by gRPC (https://github.com/grpc). Each Status message contains three pieces of data: error code, error message, and error details.You can find out more about this error model and how to work with it in the API Design Guide (https://cloud.google.com/apis/design/errors)."]
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
#[doc = "A request to stop a cluster."]
pub struct StopClusterRequest {
    #[serde(rename = "clusterUuid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Specifying the cluster_uuid means the RPC will fail (with error NOT_FOUND) if a cluster with the specified UUID does not exist."]
    pub cluster_uuid: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A unique id used to identify the request. If the server receives two StopClusterRequest (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#google.cloud.dataproc.v1.StopClusterRequest)s with the same id, then the second request will be ignored and the first google.longrunning.Operation created and stored in the backend is returned.Recommendation: Set this value to a UUID (https://en.wikipedia.org/wiki/Universally_unique_identifier).The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). The maximum length is 40 characters."]
    pub request_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request to submit a job."]
pub struct SubmitJobRequest {
    #[serde(rename = "job")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The job resource."]
    pub job: ::std::option::Option<::std::boxed::Box<Job>>,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A unique id used to identify the request. If the server receives two SubmitJobRequest (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#google.cloud.dataproc.v1.SubmitJobRequest)s with the same id, then the second request will be ignored and the first Job created and stored in the backend is returned.It is recommended to always set this value to a UUID (https://en.wikipedia.org/wiki/Universally_unique_identifier).The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). The maximum length is 40 characters."]
    pub request_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A configurable parameter that replaces one or more fields in the template. Parameterizable fields: - Labels - File uris - Job properties - Job arguments - Script variables - Main class (in HadoopJob and SparkJob) - Zone (in ClusterSelector)"]
pub struct TemplateParameter {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Brief description of the parameter. Must not exceed 1024 characters."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Paths to all fields that the parameter replaces. A field is allowed to appear in at most one parameter's list of field paths.A field path is similar in syntax to a google.protobuf.FieldMask. For example, a field path that references the zone field of a workflow template's cluster selector would be specified as placement.clusterSelector.zone.Also, field paths can reference fields using the following syntax: Values in maps can be referenced by key: labels'key' placement.clusterSelector.clusterLabels'key' placement.managedCluster.labels'key' placement.clusterSelector.clusterLabels'key' jobs'step-id'.labels'key' Jobs in the jobs list can be referenced by step-id: jobs'step-id'.hadoopJob.mainJarFileUri jobs'step-id'.hiveJob.queryFileUri jobs'step-id'.pySparkJob.mainPythonFileUri jobs'step-id'.hadoopJob.jarFileUris0 jobs'step-id'.hadoopJob.archiveUris0 jobs'step-id'.hadoopJob.fileUris0 jobs'step-id'.pySparkJob.pythonFileUris0 Items in repeated fields can be referenced by a zero-based index: jobs'step-id'.sparkJob.args0 Other examples: jobs'step-id'.hadoopJob.properties'key' jobs'step-id'.hadoopJob.args0 jobs'step-id'.hiveJob.scriptVariables'key' jobs'step-id'.hadoopJob.mainJarFileUri placement.clusterSelector.zoneIt may not be possible to parameterize maps and repeated fields in their entirety since only individual map values and individual items in repeated fields can be referenced. For example, the following field paths are invalid: placement.clusterSelector.clusterLabels jobs'step-id'.sparkJob.args"]
    pub fields: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Parameter name. The parameter name is used as the key, and paired with the parameter value, which are passed to the template when the template is instantiated. The name must contain only capital letters (A-Z), numbers (0-9), and underscores (_), and must not start with a number. The maximum length is 40 characters."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "validation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Validation rules to be applied to this parameter's value."]
    pub validation: ::std::option::Option<::std::boxed::Box<ParameterValidation>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for TestIamPermissions method."]
pub struct TestIamPermissionsRequest {
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of permissions to check for the resource. Permissions with wildcards (such as '*' or 'storage.*') are not allowed. For more information see IAM Overview (https://cloud.google.com/iam/docs/overview#permissions)."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for TestIamPermissions method."]
pub struct TestIamPermissionsResponse {
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A subset of TestPermissionsRequest.permissions that the caller is allowed."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Validation based on a list of allowed values."]
pub struct ValueValidation {
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. List of allowed values for the parameter."]
    pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The workflow graph."]
pub struct WorkflowGraph {
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The workflow nodes."]
    pub nodes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WorkflowNode>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Dataproc workflow template resource."]
pub struct WorkflowMetadata {
    #[serde(rename = "clusterName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The name of the target cluster."]
    pub cluster_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "clusterUuid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The UUID of target cluster."]
    pub cluster_uuid: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createCluster")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The create cluster operation metadata."]
    pub create_cluster: ::std::option::Option<::std::boxed::Box<ClusterOperation>>,
    #[serde(rename = "dagEndTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. DAG end time, only set for workflows with dag_timeout when DAG ends."]
    pub dag_end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dagStartTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. DAG start time, only set for workflows with dag_timeout when DAG begins."]
    pub dag_start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dagTimeout")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The timeout duration for the DAG of jobs, expressed in seconds (see JSON representation of duration (https://developers.google.com/protocol-buffers/docs/proto3#json))."]
    pub dag_timeout: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deleteCluster")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The delete cluster operation metadata."]
    pub delete_cluster: ::std::option::Option<::std::boxed::Box<ClusterOperation>>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Workflow end time."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "graph")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The workflow graph."]
    pub graph: ::std::option::Option<::std::boxed::Box<WorkflowGraph>>,
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Map from parameter names to values that were used for those parameters."]
    pub parameters:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Workflow start time."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The workflow state."]
    pub state: ::std::option::Option<WorkflowMetadataStateEnum>,
    #[serde(rename = "template")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the workflow template as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates, the resource name of the template has the following format: projects/{project_id}/regions/{region}/workflowTemplates/{template_id} For projects.locations.workflowTemplates, the resource name of the template has the following format: projects/{project_id}/locations/{location}/workflowTemplates/{template_id}"]
    pub template: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The version of template at the time of workflow instantiation."]
    pub version: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The workflow state."]
pub enum WorkflowMetadataStateEnum {
    #[serde(rename = "UNKNOWN")]
    #[doc = "Unused."]
    Unknown,
    #[serde(rename = "PENDING")]
    #[doc = "The operation has been created."]
    Pending,
    #[serde(rename = "RUNNING")]
    #[doc = "The operation is running."]
    Running,
    #[serde(rename = "DONE")]
    #[doc = "The operation is done; either cancelled or completed."]
    Done,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The workflow node."]
pub struct WorkflowNode {
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The error detail."]
    pub error: ::std::option::Option<::std::string::String>,
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The job id; populated after the node enters RUNNING state."]
    pub job_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "prerequisiteStepIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Node's prerequisite nodes."]
    pub prerequisite_step_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The node state."]
    pub state: ::std::option::Option<WorkflowNodeStateEnum>,
    #[serde(rename = "stepId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The name of the node."]
    pub step_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The node state."]
pub enum WorkflowNodeStateEnum {
    #[serde(rename = "NODE_STATE_UNSPECIFIED")]
    #[doc = "State is unspecified."]
    NodeStateUnspecified,
    #[serde(rename = "BLOCKED")]
    #[doc = "The node is awaiting prerequisite node to finish."]
    Blocked,
    #[serde(rename = "RUNNABLE")]
    #[doc = "The node is runnable but not running."]
    Runnable,
    #[serde(rename = "RUNNING")]
    #[doc = "The node is running."]
    Running,
    #[serde(rename = "COMPLETED")]
    #[doc = "The node completed successfully."]
    Completed,
    #[serde(rename = "FAILED")]
    #[doc = "The node failed. A node can be marked FAILED because its ancestor or peer failed."]
    Failed,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Dataproc workflow template resource."]
pub struct WorkflowTemplate {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time template was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dagTimeout")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Timeout duration for the DAG of jobs, expressed in seconds (see JSON representation of duration (https://developers.google.com/protocol-buffers/docs/proto3#json)). The timeout duration must be from 10 minutes (\"600s\") to 24 hours (\"86400s\"). The timer begins when the first job is submitted. If the workflow is running at the end of the timeout period, any remaining jobs are cancelled, the workflow is ended, and if the workflow was running on a managed cluster, the cluster is deleted."]
    pub dag_timeout: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "jobs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The Directed Acyclic Graph of Jobs to submit."]
    pub jobs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderedJob>>>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The labels to associate with this template. These labels will be propagated to all jobs and clusters created by the workflow instance.Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt).Label values may be empty, but, if present, must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt).No more than 32 labels can be associated with a template."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates, the resource name of the template has the following format: projects/{project_id}/regions/{region}/workflowTemplates/{template_id} For projects.locations.workflowTemplates, the resource name of the template has the following format: projects/{project_id}/locations/{location}/workflowTemplates/{template_id}"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Template parameters whose values are substituted into the template. Values for parameters must be provided when the template is instantiated."]
    pub parameters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TemplateParameter>>>,
    #[serde(rename = "placement")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. WorkflowTemplate scheduling information."]
    pub placement: ::std::option::Option<::std::boxed::Box<WorkflowTemplatePlacement>>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time template was last updated."]
    pub update_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Used to perform a consistent read-modify-write.This field should be left blank for a CreateWorkflowTemplate request. It is required for an UpdateWorkflowTemplate request, and must match the current server version. A typical update template flow would fetch the current template with a GetWorkflowTemplate request, which will return the current template with the version field filled in with the current server version. The user updates other fields in the template, then returns it as part of the UpdateWorkflowTemplate request."]
    pub version: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies workflow execution target.Either managed_cluster or cluster_selector is required."]
pub struct WorkflowTemplatePlacement {
    #[serde(rename = "clusterSelector")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A selector that chooses target cluster for jobs based on metadata.The selector is evaluated at the time each job is submitted."]
    pub cluster_selector: ::std::option::Option<::std::boxed::Box<ClusterSelector>>,
    #[serde(rename = "managedCluster")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A cluster that is managed by the workflow."]
    pub managed_cluster: ::std::option::Option<::std::boxed::Box<ManagedCluster>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A YARN application created by a job. Application information is a subset of org.apache.hadoop.yarn.proto.YarnProtos.ApplicationReportProto.Beta Feature: This report is available for testing purposes only. It may be changed before final release."]
pub struct YarnApplication {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The application name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The numerical progress of the application, from 1 to 100."]
    pub progress: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The application state."]
    pub state: ::std::option::Option<YarnApplicationStateEnum>,
    #[serde(rename = "trackingUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The HTTP URL of the ApplicationMaster, HistoryServer, or TimelineServer that provides application-specific information. The URL uses the internal hostname, and requires a proxy server for resolution and, possibly, access."]
    pub tracking_url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The application state."]
pub enum YarnApplicationStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Status is unspecified."]
    StateUnspecified,
    #[serde(rename = "NEW")]
    #[doc = "Status is NEW."]
    New,
    #[serde(rename = "NEW_SAVING")]
    #[doc = "Status is NEW_SAVING."]
    NewSaving,
    #[serde(rename = "SUBMITTED")]
    #[doc = "Status is SUBMITTED."]
    Submitted,
    #[serde(rename = "ACCEPTED")]
    #[doc = "Status is ACCEPTED."]
    Accepted,
    #[serde(rename = "RUNNING")]
    #[doc = "Status is RUNNING."]
    Running,
    #[serde(rename = "FINISHED")]
    #[doc = "Status is FINISHED."]
    Finished,
    #[serde(rename = "FAILED")]
    #[doc = "Status is FAILED."]
    Failed,
    #[serde(rename = "KILLED")]
    #[doc = "Status is KILLED."]
    Killed,
}
