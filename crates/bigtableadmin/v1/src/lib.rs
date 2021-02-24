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
