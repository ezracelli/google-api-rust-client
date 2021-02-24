#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A configuration object describing how Cloud Bigtable should treat traffic from a particular end user application."]
pub struct AppProfile {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Long form description of the use case for this AppProfile."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Strongly validated etag for optimistic concurrency control. Preserve the value returned from `GetAppProfile` when calling `UpdateAppProfile` to fail the request if there has been a modification in the mean time. The `update_mask` of the request need not include `etag` for this protection to apply. See [Wikipedia](https://en.wikipedia.org/wiki/HTTP_ETag) and [RFC 7232](https://tools.ietf.org/html/rfc7232#section-2.3) for more details."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "multiClusterRoutingUseAny")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Use a multi-cluster routing policy."]
    pub multi_cluster_routing_use_any:
        ::std::option::Option<::std::boxed::Box<MultiClusterRoutingUseAny>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique name of the app profile. Values are of the form `projects/{project}/instances/{instance}/appProfiles/_a-zA-Z0-9*`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "singleClusterRouting")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Use a single-cluster routing policy."]
    pub single_cluster_routing: ::std::option::Option<::std::boxed::Box<SingleClusterRouting>>,
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
#[doc = "A backup of a Cloud Bigtable table."]
pub struct Backup {
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. `end_time` is the time that the backup was finished. The row data in the backup will be no newer than this timestamp."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expireTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The expiration time of the backup, with microseconds granularity that must be at least 6 hours and at most 30 days from the time the request is received. Once the `expire_time` has passed, Cloud Bigtable will delete the backup and free the resources used by the backup."]
    pub expire_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A globally unique identifier for the backup which cannot be changed. Values are of the form `projects/{project}/instances/{instance}/clusters/{cluster}/ backups/_a-zA-Z0-9*` The final segment of the name must be between 1 and 50 characters in length. The backup is stored in the cluster identified by the prefix of the backup name of the form `projects/{project}/instances/{instance}/clusters/{cluster}`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sizeBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Size of the backup in bytes."]
    pub size_bytes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sourceTable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. Name of the table from which this backup was created. This needs to be in the same instance as the backup. Values are of the form `projects/{project}/instances/{instance}/tables/{source_table}`."]
    pub source_table: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. `start_time` is the time that the backup was started (i.e. approximately the time the CreateBackup request is received). The row data in this backup will be no older than this timestamp."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The current state of the backup."]
    pub state: ::std::option::Option<BackupStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The current state of the backup."]
pub enum BackupStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Not specified."]
    StateUnspecified,
    #[serde(rename = "CREATING")]
    #[doc = "The pending backup is still being created. Operations on the backup may fail with `FAILED_PRECONDITION` in this state."]
    Creating,
    #[serde(rename = "READY")]
    #[doc = "The backup is complete and ready for use."]
    Ready,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about a backup."]
pub struct BackupInfo {
    #[serde(rename = "backup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Name of the backup."]
    pub backup: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. This time that the backup was finished. Row data in the backup will be no newer than this timestamp."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sourceTable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Name of the table the backup was created from."]
    pub source_table: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time that the backup was started. Row data in the backup will be no older than this timestamp."]
    pub start_time: ::std::option::Option<::std::string::String>,
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
#[doc = "Request message for google.bigtable.admin.v2.BigtableTableAdmin.CheckConsistency"]
pub struct CheckConsistencyRequest {
    #[serde(rename = "consistencyToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The token created using GenerateConsistencyToken for the Table."]
    pub consistency_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for google.bigtable.admin.v2.BigtableTableAdmin.CheckConsistency"]
pub struct CheckConsistencyResponse {
    #[serde(rename = "consistent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True only if the token is consistent. A token is consistent if replication has caught up with the restrictions specified in the request."]
    pub consistent: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A resizable group of nodes in a particular cloud location, capable of serving all Tables in the parent Instance."]
pub struct Cluster {
    #[serde(rename = "defaultStorageType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The type of storage used by this cluster to serve its parent instance's tables, unless explicitly overridden."]
    pub default_storage_type: ::std::option::Option<ClusterDefaultStorageTypeEnum>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The location where this cluster's nodes and storage reside. For best performance, clients should be located as close as possible to this cluster. Currently only zones are supported, so values should be of the form `projects/{project}/locations/{zone}`."]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique name of the cluster. Values are of the form `projects/{project}/instances/{instance}/clusters/a-z*`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serveNodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The number of nodes allocated to this cluster. More nodes enable higher throughput and more consistent performance."]
    pub serve_nodes: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The current state of the cluster."]
    pub state: ::std::option::Option<ClusterStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Immutable. The type of storage used by this cluster to serve its parent instance's tables, unless explicitly overridden."]
pub enum ClusterDefaultStorageTypeEnum {
    #[serde(rename = "STORAGE_TYPE_UNSPECIFIED")]
    #[doc = "The user did not specify a storage type."]
    StorageTypeUnspecified,
    #[serde(rename = "SSD")]
    #[doc = "Flash (SSD) storage should be used."]
    Ssd,
    #[serde(rename = "HDD")]
    #[doc = "Magnetic drive (HDD) storage should be used."]
    Hdd,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The current state of the cluster."]
pub enum ClusterStateEnum {
    #[serde(rename = "STATE_NOT_KNOWN")]
    #[doc = "The state of the cluster could not be determined."]
    StateNotKnown,
    #[serde(rename = "READY")]
    #[doc = "The cluster has been successfully created and is ready to serve requests."]
    Ready,
    #[serde(rename = "CREATING")]
    #[doc = "The cluster is currently being created, and may be destroyed if the creation process encounters an error. A cluster may not be able to serve requests while being created."]
    Creating,
    #[serde(rename = "RESIZING")]
    #[doc = "The cluster is currently being resized, and may revert to its previous node count if the process encounters an error. A cluster is still capable of serving requests while being resized, but may exhibit performance as if its number of allocated nodes is between the starting and requested states."]
    Resizing,
    #[serde(rename = "DISABLED")]
    #[doc = "The cluster has no backing nodes. The data (tables) still exist, but no operations can be performed on the cluster."]
    Disabled,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The state of a table's data in a particular cluster."]
pub struct ClusterState {
    #[serde(rename = "replicationState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The state of replication for the table in this cluster."]
    pub replication_state: ::std::option::Option<ClusterStateReplicationStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The state of replication for the table in this cluster."]
pub enum ClusterStateReplicationStateEnum {
    #[serde(rename = "STATE_NOT_KNOWN")]
    #[doc = "The replication state of the table is unknown in this cluster."]
    StateNotKnown,
    #[serde(rename = "INITIALIZING")]
    #[doc = "The cluster was recently created, and the table must finish copying over pre-existing data from other clusters before it can begin receiving live replication updates and serving Data API requests."]
    Initializing,
    #[serde(rename = "PLANNED_MAINTENANCE")]
    #[doc = "The table is temporarily unable to serve Data API requests from this cluster due to planned internal maintenance."]
    PlannedMaintenance,
    #[serde(rename = "UNPLANNED_MAINTENANCE")]
    #[doc = "The table is temporarily unable to serve Data API requests from this cluster due to unplanned or emergency maintenance."]
    UnplannedMaintenance,
    #[serde(rename = "READY")]
    #[doc = "The table can serve Data API requests from this cluster. Depending on replication delay, reads may not immediately reflect the state of the table in other clusters."]
    Ready,
    #[serde(rename = "READY_OPTIMIZING")]
    #[doc = "The table is fully created and ready for use after a restore, and is being optimized for performance. When optimizations are complete, the table will transition to `READY` state."]
    ReadyOptimizing,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A set of columns within a table which share a common configuration."]
pub struct ColumnFamily {
    #[serde(rename = "gcRule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Garbage collection rule specified as a protobuf. Must serialize to at most 500 bytes. NOTE: Garbage collection executes opportunistically in the background, and so it's possible for reads to return a cell even if it matches the active GC expression for its family."]
    pub gc_rule: ::std::option::Option<::std::boxed::Box<GcRule>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata type for the operation returned by CreateBackup."]
pub struct CreateBackupMetadata {
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, the time at which this operation finished or was cancelled."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the backup being created."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sourceTable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the table the backup is created from."]
    pub source_table: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which this operation started."]
    pub start_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The metadata for the Operation returned by CreateCluster."]
pub struct CreateClusterMetadata {
    #[serde(rename = "finishTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the operation failed or was completed successfully."]
    pub finish_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "originalRequest")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request that prompted the initiation of this CreateCluster operation."]
    pub original_request: ::std::option::Option<::std::boxed::Box<CreateClusterRequest>>,
    #[serde(rename = "requestTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the original request was received."]
    pub request_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tables")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Keys: the full `name` of each table that existed in the instance when CreateCluster was first called, i.e. `projects//instances//tables/`. Any table added to the instance by a later API call will be created in the new cluster by that API call, not this one. Values: information on how much of a table's data has been copied to the newly-created cluster so far."]
    pub tables: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<TableProgress>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for BigtableInstanceAdmin.CreateCluster."]
pub struct CreateClusterRequest {
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The cluster to be created. Fields marked `OutputOnly` must be left blank."]
    pub cluster: ::std::option::Option<::std::boxed::Box<Cluster>>,
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The ID to be used when referring to the new cluster within its instance, e.g., just `mycluster` rather than `projects/myproject/instances/myinstance/clusters/mycluster`."]
    pub cluster_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The unique name of the instance in which to create the new cluster. Values are of the form `projects/{project}/instances/{instance}`."]
    pub parent: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The metadata for the Operation returned by CreateInstance."]
pub struct CreateInstanceMetadata {
    #[serde(rename = "finishTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the operation failed or was completed successfully."]
    pub finish_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "originalRequest")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request that prompted the initiation of this CreateInstance operation."]
    pub original_request: ::std::option::Option<::std::boxed::Box<CreateInstanceRequest>>,
    #[serde(rename = "requestTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the original request was received."]
    pub request_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for BigtableInstanceAdmin.CreateInstance."]
pub struct CreateInstanceRequest {
    #[serde(rename = "clusters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The clusters to be created within the instance, mapped by desired cluster ID, e.g., just `mycluster` rather than `projects/myproject/instances/myinstance/clusters/mycluster`. Fields marked `OutputOnly` must be left blank. Currently, at most four clusters can be specified."]
    pub clusters:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<Cluster>>>,
    #[serde(rename = "instance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The instance to create. Fields marked `OutputOnly` must be left blank."]
    pub instance: ::std::option::Option<::std::boxed::Box<Instance>>,
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The ID to be used when referring to the new instance within its project, e.g., just `myinstance` rather than `projects/myproject/instances/myinstance`."]
    pub instance_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The unique name of the project in which to create the new instance. Values are of the form `projects/{project}`."]
    pub parent: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for google.bigtable.admin.v2.BigtableTableAdmin.CreateTable"]
pub struct CreateTableRequest {
    #[serde(rename = "initialSplits")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The optional list of row keys that will be used to initially split the table into several tablets (tablets are similar to HBase regions). Given two split keys, `s1` and `s2`, three tablets will be created, spanning the key ranges: `[, s1), [s1, s2), [s2, )`. Example: * Row keys := `[\"a\", \"apple\", \"custom\", \"customer_1\", \"customer_2\",` `\"other\", \"zz\"]` * initial_split_keys := `[\"apple\", \"customer_1\", \"customer_2\", \"other\"]` * Key assignment: - Tablet 1 `[, apple) => {\"a\"}.` - Tablet 2 `[apple, customer_1) => {\"apple\", \"custom\"}.` - Tablet 3 `[customer_1, customer_2) => {\"customer_1\"}.` - Tablet 4 `[customer_2, other) => {\"customer_2\"}.` - Tablet 5 `[other, ) => {\"other\", \"zz\"}.`"]
    pub initial_splits: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Split>>>,
    #[serde(rename = "table")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The Table to create."]
    pub table: ::std::option::Option<::std::boxed::Box<Table>>,
    #[serde(rename = "tableId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name by which the new table should be referred to within the parent instance, e.g., `foobar` rather than `{parent}/tables/foobar`. Maximum 50 characters."]
    pub table_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for google.bigtable.admin.v2.BigtableTableAdmin.DropRowRange"]
pub struct DropRowRangeRequest {
    #[serde(rename = "deleteAllDataFromTable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Delete all rows in the table. Setting this to false is a no-op."]
    pub delete_all_data_from_table: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "rowKeyPrefix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Delete all rows that start with this row key prefix. Prefix cannot be zero length."]
    pub row_key_prefix: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
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
#[doc = "Added to the error payload."]
pub struct FailureTrace {
    #[serde(rename = "frames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub frames: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Frame>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Frame {
    #[serde(rename = "targetName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub target_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "workflowGuid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub workflow_guid: ::std::option::Option<::std::string::String>,
    #[serde(rename = "zoneId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub zone_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Rule for determining which cells to delete during garbage collection."]
pub struct GcRule {
    #[serde(rename = "intersection")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Delete cells that would be deleted by every nested rule."]
    pub intersection: ::std::option::Option<::std::boxed::Box<Intersection>>,
    #[serde(rename = "maxAge")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Delete cells in a column older than the given age. Values must be at least one millisecond, and will be truncated to microsecond granularity."]
    pub max_age: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maxNumVersions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Delete all cells in a column except the most recent N."]
    pub max_num_versions: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "union")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Delete cells that would be deleted by any nested rule."]
    pub _union: ::std::option::Option<::std::boxed::Box<Union>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for google.bigtable.admin.v2.BigtableTableAdmin.GenerateConsistencyToken"]
pub struct GenerateConsistencyTokenRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for google.bigtable.admin.v2.BigtableTableAdmin.GenerateConsistencyToken"]
pub struct GenerateConsistencyTokenResponse {
    #[serde(rename = "consistencyToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The generated consistency token."]
    pub consistency_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for `GetIamPolicy` method."]
pub struct GetIamPolicyRequest {
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OPTIONAL: A `GetPolicyOptions` object for specifying options to `GetIamPolicy`."]
    pub options: ::std::option::Option<::std::boxed::Box<GetPolicyOptions>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Encapsulates settings provided to GetIamPolicy."]
pub struct GetPolicyOptions {
    #[serde(rename = "requestedPolicyVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The policy format version to be returned. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional bindings must specify version 3. Policies without any conditional bindings may specify any valid value or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
    pub requested_policy_version: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A collection of Bigtable Tables and the resources that serve them. All tables in an instance are served from all Clusters in the instance."]
pub struct Instance {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The descriptive name for this instance as it appears in UIs. Can be changed at any time, but should be kept globally unique to avoid confusion."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Labels are a flexible and lightweight mechanism for organizing cloud resources into groups that reflect a customer's organizational needs and deployment strategies. They can be used to filter resources and aggregate metrics. * Label keys must be between 1 and 63 characters long and must conform to the regular expression: `\\p{Ll}\\p{Lo}{0,62}`. * Label values must be between 0 and 63 characters long and must conform to the regular expression: `[\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}`. * No more than 64 labels can be associated with a given resource. * Keys and values must both be under 128 bytes."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique name of the instance. Values are of the form `projects/{project}/instances/a-z+[a-z0-9]`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The current state of the instance."]
    pub state: ::std::option::Option<InstanceStateEnum>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The type of the instance. Defaults to `PRODUCTION`."]
    pub _type: ::std::option::Option<InstanceTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The current state of the instance."]
pub enum InstanceStateEnum {
    #[serde(rename = "STATE_NOT_KNOWN")]
    #[doc = "The state of the instance could not be determined."]
    StateNotKnown,
    #[serde(rename = "READY")]
    #[doc = "The instance has been successfully created and can serve requests to its tables."]
    Ready,
    #[serde(rename = "CREATING")]
    #[doc = "The instance is currently being created, and may be destroyed if the creation process encounters an error."]
    Creating,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The type of the instance. Defaults to `PRODUCTION`."]
pub enum InstanceTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "The type of the instance is unspecified. If set when creating an instance, a `PRODUCTION` instance will be created. If set when updating an instance, the type will be left unchanged."]
    TypeUnspecified,
    #[serde(rename = "PRODUCTION")]
    #[doc = "An instance meant for production use. `serve_nodes` must be set on the cluster."]
    Production,
    #[serde(rename = "DEVELOPMENT")]
    #[doc = "DEPRECATED: Prefer PRODUCTION for all use cases, as it no longer enforces a higher minimum node count than DEVELOPMENT."]
    Development,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A GcRule which deletes cells matching all of the given rules."]
pub struct Intersection {
    #[serde(rename = "rules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only delete cells which would be deleted by every element of `rules`."]
    pub rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GcRule>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for BigtableInstanceAdmin.ListAppProfiles."]
pub struct ListAppProfilesResponse {
    #[serde(rename = "appProfiles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of requested app profiles."]
    pub app_profiles: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AppProfile>>>,
    #[serde(rename = "failedLocations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Locations from which AppProfile information could not be retrieved, due to an outage or some other transient condition. AppProfiles from these locations may be missing from `app_profiles`. Values are of the form `projects//locations/`"]
    pub failed_locations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set if not all app profiles could be returned in a single response. Pass this value to `page_token` in another request to get the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for ListBackups."]
pub struct ListBackupsResponse {
    #[serde(rename = "backups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of matching backups."]
    pub backups: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Backup>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "`next_page_token` can be sent in a subsequent ListBackups call to fetch more of the matching backups."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for BigtableInstanceAdmin.ListClusters."]
pub struct ListClustersResponse {
    #[serde(rename = "clusters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of requested clusters."]
    pub clusters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Cluster>>>,
    #[serde(rename = "failedLocations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Locations from which Cluster information could not be retrieved, due to an outage or some other transient condition. Clusters from these locations may be missing from `clusters`, or may only have partial information returned. Values are of the form `projects//locations/`"]
    pub failed_locations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DEPRECATED: This field is unused and ignored."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for BigtableInstanceAdmin.ListInstances."]
pub struct ListInstancesResponse {
    #[serde(rename = "failedLocations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Locations from which Instance information could not be retrieved, due to an outage or some other transient condition. Instances whose Clusters are all in one of the failed locations may be missing from `instances`, and Instances with at least one Cluster in a failed location may only have partial information returned. Values are of the form `projects//locations/`"]
    pub failed_locations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of requested instances."]
    pub instances: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Instance>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DEPRECATED: This field is unused and ignored."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
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
#[doc = "Response message for google.bigtable.admin.v2.BigtableTableAdmin.ListTables"]
pub struct ListTablesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set if not all tables could be returned in a single response. Pass this value to `page_token` in another request to get the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tables")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The tables present in the requested instance."]
    pub tables: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Table>>>,
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
#[doc = "A create, update, or delete of a particular column family."]
pub struct Modification {
    #[serde(rename = "create")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Create a new column family with the specified schema, or fail if one already exists with the given ID."]
    pub create: ::std::option::Option<::std::boxed::Box<ColumnFamily>>,
    #[serde(rename = "drop")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Drop (delete) the column family with the given ID, or fail if no such family exists."]
    pub drop: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the column family to be modified."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "update")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Update an existing column family to the specified schema, or fail if no column family exists with the given ID."]
    pub update: ::std::option::Option<::std::boxed::Box<ColumnFamily>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for google.bigtable.admin.v2.BigtableTableAdmin.ModifyColumnFamilies"]
pub struct ModifyColumnFamiliesRequest {
    #[serde(rename = "modifications")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Modifications to be atomically applied to the specified table's families. Entries are applied in order, meaning that earlier modifications can be masked by later ones (in the case of repeated updates to the same family, for example)."]
    pub modifications: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Modification>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Read/write requests are routed to the nearest cluster in the instance, and will fail over to the nearest cluster that is available in the event of transient errors or delays. Clusters in a region are considered equidistant. Choosing this option sacrifices read-your-writes consistency to improve availability."]
pub struct MultiClusterRoutingUseAny {}
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
#[doc = "Encapsulates progress related information for a Cloud Bigtable long running operation."]
pub struct OperationProgress {
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, the time at which this operation failed or was completed successfully."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progressPercent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Percent completion of the operation. Values are between 0 and 100 inclusive."]
    pub progress_percent: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time the request was received."]
    pub start_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata type for the long-running operation used to track the progress of optimizations performed on a newly restored table. This long-running operation is automatically created by the system after the successful completion of a table restore, and cannot be cancelled."]
pub struct OptimizeRestoredTableMetadata {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the restored table being optimized."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The progress of the post-restore optimizations."]
    pub progress: ::std::option::Option<::std::boxed::Box<OperationProgress>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for BigtableInstanceAdmin.PartialUpdateInstance."]
pub struct PartialUpdateInstanceRequest {
    #[serde(rename = "instance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The Instance which will (partially) replace the current value."]
    pub instance: ::std::option::Option<::std::boxed::Box<Instance>>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The subset of Instance fields which should be replaced. Must be explicitly set."]
    pub update_mask: ::std::option::Option<::std::string::String>,
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
#[doc = "Information about a table restore."]
pub struct RestoreInfo {
    #[serde(rename = "backupInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the backup used to restore the table. The backup may no longer exist."]
    pub backup_info: ::std::option::Option<::std::boxed::Box<BackupInfo>>,
    #[serde(rename = "sourceType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the restore source."]
    pub source_type: ::std::option::Option<RestoreInfoSourceTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the restore source."]
pub enum RestoreInfoSourceTypeEnum {
    #[serde(rename = "RESTORE_SOURCE_TYPE_UNSPECIFIED")]
    #[doc = "No restore associated."]
    RestoreSourceTypeUnspecified,
    #[serde(rename = "BACKUP")]
    #[doc = "A backup was used as the source of the restore."]
    Backup,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata type for the long-running operation returned by RestoreTable."]
pub struct RestoreTableMetadata {
    #[serde(rename = "backupInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub backup_info: ::std::option::Option<::std::boxed::Box<BackupInfo>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the table being created and restored to."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "optimizeTableOperationName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If exists, the name of the long-running operation that will be used to track the post-restore optimization process to optimize the performance of the restored table. The metadata type of the long-running operation is OptimizeRestoreTableMetadata. The response type is Empty. This long-running operation may be automatically created by the system if applicable after the RestoreTable long-running operation completes successfully. This operation may not be created if the table is already optimized or the restore was not successful."]
    pub optimize_table_operation_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The progress of the RestoreTable operation."]
    pub progress: ::std::option::Option<::std::boxed::Box<OperationProgress>>,
    #[serde(rename = "sourceType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the restore source."]
    pub source_type: ::std::option::Option<RestoreTableMetadataSourceTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the restore source."]
pub enum RestoreTableMetadataSourceTypeEnum {
    #[serde(rename = "RESTORE_SOURCE_TYPE_UNSPECIFIED")]
    #[doc = "No restore associated."]
    RestoreSourceTypeUnspecified,
    #[serde(rename = "BACKUP")]
    #[doc = "A backup was used as the source of the restore."]
    Backup,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for RestoreTable."]
pub struct RestoreTableRequest {
    #[serde(rename = "backup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the backup from which to restore. Values are of the form `projects//instances//clusters//backups/`."]
    pub backup: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tableId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The id of the table to create and restore to. This table must not already exist. The `table_id` appended to `parent` forms the full table name of the form `projects//instances//tables/`."]
    pub table_id: ::std::option::Option<::std::string::String>,
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
#[doc = "Unconditionally routes all read/write requests to a specific cluster. This option preserves read-your-writes consistency but does not improve availability."]
pub struct SingleClusterRouting {
    #[serde(rename = "allowTransactionalWrites")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not `CheckAndMutateRow` and `ReadModifyWriteRow` requests are allowed by this app profile. It is unsafe to send these requests to the same table/row/column in multiple clusters."]
    pub allow_transactional_writes: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The cluster to which read/write requests should be routed."]
    pub cluster_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An initial split point for a newly created table."]
pub struct Split {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Row key to use as an initial tablet boundary."]
    pub key: ::std::option::Option<::std::string::String>,
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
#[doc = "A collection of user data indexed by row, column, and timestamp. Each table is served using the resources of its parent cluster."]
pub struct Table {
    #[serde(rename = "clusterStates")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Map from cluster ID to per-cluster table state. If it could not be determined whether or not the table has data in a particular cluster (for example, if its zone is unavailable), then there will be an entry for the cluster with UNKNOWN `replication_status`. Views: `REPLICATION_VIEW`, `FULL`"]
    pub cluster_states: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<ClusterState>>,
    >,
    #[serde(rename = "columnFamilies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The column families configured for this table, mapped by column family ID. Views: `SCHEMA_VIEW`, `FULL`"]
    pub column_families: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<ColumnFamily>>,
    >,
    #[serde(rename = "granularity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The granularity (i.e. `MILLIS`) at which timestamps are stored in this table. Timestamps not matching the granularity will be rejected. If unspecified at creation time, the value will be set to `MILLIS`. Views: `SCHEMA_VIEW`, `FULL`."]
    pub granularity: ::std::option::Option<TableGranularityEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique name of the table. Values are of the form `projects/{project}/instances/{instance}/tables/_a-zA-Z0-9*`. Views: `NAME_ONLY`, `SCHEMA_VIEW`, `REPLICATION_VIEW`, `FULL`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "restoreInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. If this table was restored from another data source (e.g. a backup), this field will be populated with information about the restore."]
    pub restore_info: ::std::option::Option<::std::boxed::Box<RestoreInfo>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Immutable. The granularity (i.e. `MILLIS`) at which timestamps are stored in this table. Timestamps not matching the granularity will be rejected. If unspecified at creation time, the value will be set to `MILLIS`. Views: `SCHEMA_VIEW`, `FULL`."]
pub enum TableGranularityEnum {
    #[serde(rename = "TIMESTAMP_GRANULARITY_UNSPECIFIED")]
    #[doc = "The user did not specify a granularity. Should not be returned. When specified during table creation, MILLIS will be used."]
    TimestampGranularityUnspecified,
    #[serde(rename = "MILLIS")]
    #[doc = "The table keeps data versioned at a granularity of 1ms."]
    Millis,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Progress info for copying a table's data to the new cluster."]
pub struct TableProgress {
    #[serde(rename = "estimatedCopiedBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Estimate of the number of bytes copied so far for this table. This will eventually reach 'estimated_size_bytes' unless the table copy is CANCELLED."]
    pub estimated_copied_bytes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "estimatedSizeBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Estimate of the size of the table to be copied."]
    pub estimated_size_bytes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub state: ::std::option::Option<TableProgressStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum TableProgressStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = ""]
    StateUnspecified,
    #[serde(rename = "PENDING")]
    #[doc = "The table has not yet begun copying to the new cluster."]
    Pending,
    #[serde(rename = "COPYING")]
    #[doc = "The table is actively being copied to the new cluster."]
    Copying,
    #[serde(rename = "COMPLETED")]
    #[doc = "The table has been fully copied to the new cluster."]
    Completed,
    #[serde(rename = "CANCELLED")]
    #[doc = "The table was deleted before it finished copying to the new cluster. Note that tables deleted after completion will stay marked as COMPLETED, not CANCELLED."]
    Cancelled,
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
#[doc = "A GcRule which deletes cells matching any of the given rules."]
pub struct Union {
    #[serde(rename = "rules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Delete cells which would be deleted by any element of `rules`."]
    pub rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GcRule>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The metadata for the Operation returned by UpdateAppProfile."]
pub struct UpdateAppProfileMetadata {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The metadata for the Operation returned by UpdateCluster."]
pub struct UpdateClusterMetadata {
    #[serde(rename = "finishTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the operation failed or was completed successfully."]
    pub finish_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "originalRequest")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request that prompted the initiation of this UpdateCluster operation."]
    pub original_request: ::std::option::Option<::std::boxed::Box<Cluster>>,
    #[serde(rename = "requestTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the original request was received."]
    pub request_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The metadata for the Operation returned by UpdateInstance."]
pub struct UpdateInstanceMetadata {
    #[serde(rename = "finishTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the operation failed or was completed successfully."]
    pub finish_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "originalRequest")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request that prompted the initiation of this UpdateInstance operation."]
    pub original_request: ::std::option::Option<::std::boxed::Box<PartialUpdateInstanceRequest>>,
    #[serde(rename = "requestTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the original request was received."]
    pub request_time: ::std::option::Option<::std::string::String>,
}