#[derive(
    Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
)]
pub struct QueryParameters {
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "$.xgafv")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "V1 error format."]
    pub xgafv: ::std::option::Option<QueryParametersXgafvEnum>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "access_token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OAuth access token."]
    pub access_token: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ query_parameters_defaults :: alt () }", setter(into))]
    #[serde(rename = "alt")]
    #[serde(default = "query_parameters_defaults :: alt")]
    #[doc = "Data format for response."]
    pub alt: QueryParametersAltEnum,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "callback")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "JSONP"]
    pub callback: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Selector specifying which fields to include in a partial response."]
    pub fields: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
    pub key: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "oauth_token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OAuth 2.0 token for the current user."]
    pub oauth_token: ::std::option::Option<::std::string::String>,
    #[builder(
        default = "{ query_parameters_defaults :: pretty_print () }",
        setter(into)
    )]
    #[serde(rename = "prettyPrint")]
    #[serde(default = "query_parameters_defaults :: pretty_print")]
    #[doc = "Returns response with indentations and line breaks."]
    pub pretty_print: ::std::primitive::bool,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "quotaUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
    pub quota_user: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "uploadType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
    pub upload_type: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "upload_protocol")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
    pub upload_protocol: ::std::option::Option<::std::string::String>,
}
impl QueryParameters {
    pub fn builder() -> QueryParametersBuilder {
        QueryParametersBuilder::default()
    }
}
mod query_parameters_defaults {
    pub fn alt() -> super::QueryParametersAltEnum {
        serde_json::from_str(&"json").unwrap()
    }
    pub fn pretty_print() -> ::std::primitive::bool {
        serde_json::from_str(&"true").unwrap()
    }
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "V1 error format."]
pub enum QueryParametersXgafvEnum {
    #[serde(rename = "1")]
    #[doc = "v1 error format"]
    _1,
    #[serde(rename = "2")]
    #[doc = "v2 error format"]
    _2,
}
impl ::std::default::Default for QueryParametersXgafvEnum {
    fn default() -> Self {
        Self::_1
    }
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Data format for response."]
pub enum QueryParametersAltEnum {
    #[serde(rename = "json")]
    #[doc = "Responses with Content-Type of application/json"]
    Json,
    #[serde(rename = "media")]
    #[doc = "Media download with context-dependent Content-Type"]
    Media,
    #[serde(rename = "proto")]
    #[doc = "Responses with Content-Type of application/x-protobuf"]
    Proto,
}
impl ::std::default::Default for QueryParametersAltEnum {
    fn default() -> Self {
        Self::Json
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A backup of a Cloud Bigtable table."]
    pub struct Backup {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. `end_time` is the time that the backup was finished. The row data in the backup will be no newer than this timestamp."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expireTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The expiration time of the backup, with microseconds granularity that must be at least 6 hours and at most 30 days from the time the request is received. Once the `expire_time` has passed, Cloud Bigtable will delete the backup and free the resources used by the backup."]
        pub expire_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A globally unique identifier for the backup which cannot be changed. Values are of the form `projects/{project}/instances/{instance}/clusters/{cluster}/ backups/_a-zA-Z0-9*` The final segment of the name must be between 1 and 50 characters in length. The backup is stored in the cluster identified by the prefix of the backup name of the form `projects/{project}/instances/{instance}/clusters/{cluster}`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sizeBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Size of the backup in bytes."]
        pub size_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceTable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Immutable. Name of the table from which this backup was created. This needs to be in the same instance as the backup. Values are of the form `projects/{project}/instances/{instance}/tables/{source_table}`."]
        pub source_table: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. `start_time` is the time that the backup was started (i.e. approximately the time the CreateBackup request is received). The row data in this backup will be no older than this timestamp."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The current state of the backup."]
        pub state: ::std::option::Option<BackupStateEnum>,
    }
    impl Backup {
        pub fn builder() -> BackupBuilder {
            BackupBuilder::default()
        }
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
    impl ::std::default::Default for BackupStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a backup."]
    pub struct BackupInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Name of the backup."]
        pub backup: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. This time that the backup was finished. Row data in the backup will be no newer than this timestamp."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceTable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Name of the table the backup was created from."]
        pub source_table: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time that the backup was started. Row data in the backup will be no older than this timestamp."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl BackupInfo {
        pub fn builder() -> BackupInfoBuilder {
            BackupInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A resizable group of nodes in a particular cloud location, capable of serving all Tables in the parent Instance."]
    pub struct Cluster {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultStorageType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Immutable. The type of storage used by this cluster to serve its parent instance's tables, unless explicitly overridden."]
        pub default_storage_type: ::std::option::Option<ClusterDefaultStorageTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Immutable. The location where this cluster's nodes and storage reside. For best performance, clients should be located as close as possible to this cluster. Currently only zones are supported, so values should be of the form `projects/{project}/locations/{zone}`."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique name of the cluster. Values are of the form `projects/{project}/instances/{instance}/clusters/a-z*`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serveNodes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The number of nodes allocated to this cluster. More nodes enable higher throughput and more consistent performance."]
        pub serve_nodes: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The current state of the cluster."]
        pub state: ::std::option::Option<ClusterStateEnum>,
    }
    impl Cluster {
        pub fn builder() -> ClusterBuilder {
            ClusterBuilder::default()
        }
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
    impl ::std::default::Default for ClusterDefaultStorageTypeEnum {
        fn default() -> Self {
            Self::StorageTypeUnspecified
        }
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
    impl ::std::default::Default for ClusterStateEnum {
        fn default() -> Self {
            Self::StateNotKnown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata type for the operation returned by CreateBackup."]
    pub struct CreateBackupMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, the time at which this operation finished or was cancelled."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the backup being created."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceTable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the table the backup is created from."]
        pub source_table: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which this operation started."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl CreateBackupMetadata {
        pub fn builder() -> CreateBackupMetadataBuilder {
            CreateBackupMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The metadata for the Operation returned by CreateCluster."]
    pub struct CreateClusterMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "finishTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the operation failed or was completed successfully."]
        pub finish_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalRequest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request that prompted the initiation of this CreateCluster operation."]
        pub original_request: ::std::option::Option<::std::boxed::Box<CreateClusterRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the original request was received."]
        pub request_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tables")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Keys: the full `name` of each table that existed in the instance when CreateCluster was first called, i.e. `projects//instances//tables/`. Any table added to the instance by a later API call will be created in the new cluster by that API call, not this one. Values: information on how much of a table's data has been copied to the newly-created cluster so far."]
        pub tables: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<TableProgress>>,
        >,
    }
    impl CreateClusterMetadata {
        pub fn builder() -> CreateClusterMetadataBuilder {
            CreateClusterMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for BigtableInstanceAdmin.CreateCluster."]
    pub struct CreateClusterRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cluster")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The cluster to be created. Fields marked `OutputOnly` must be left blank."]
        pub cluster: ::std::option::Option<::std::boxed::Box<Cluster>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The ID to be used when referring to the new cluster within its instance, e.g., just `mycluster` rather than `projects/myproject/instances/myinstance/clusters/mycluster`."]
        pub cluster_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The unique name of the instance in which to create the new cluster. Values are of the form `projects/{project}/instances/{instance}`."]
        pub parent: ::std::option::Option<::std::string::String>,
    }
    impl CreateClusterRequest {
        pub fn builder() -> CreateClusterRequestBuilder {
            CreateClusterRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The metadata for the Operation returned by CreateInstance."]
    pub struct CreateInstanceMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "finishTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the operation failed or was completed successfully."]
        pub finish_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalRequest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request that prompted the initiation of this CreateInstance operation."]
        pub original_request: ::std::option::Option<::std::boxed::Box<CreateInstanceRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the original request was received."]
        pub request_time: ::std::option::Option<::std::string::String>,
    }
    impl CreateInstanceMetadata {
        pub fn builder() -> CreateInstanceMetadataBuilder {
            CreateInstanceMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for BigtableInstanceAdmin.CreateInstance."]
    pub struct CreateInstanceRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The clusters to be created within the instance, mapped by desired cluster ID, e.g., just `mycluster` rather than `projects/myproject/instances/myinstance/clusters/mycluster`. Fields marked `OutputOnly` must be left blank. Currently, at most four clusters can be specified."]
        pub clusters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<Cluster>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The instance to create. Fields marked `OutputOnly` must be left blank."]
        pub instance: ::std::option::Option<::std::boxed::Box<Instance>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instanceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The ID to be used when referring to the new instance within its project, e.g., just `myinstance` rather than `projects/myproject/instances/myinstance`."]
        pub instance_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The unique name of the project in which to create the new instance. Values are of the form `projects/{project}`."]
        pub parent: ::std::option::Option<::std::string::String>,
    }
    impl CreateInstanceRequest {
        pub fn builder() -> CreateInstanceRequestBuilder {
            CreateInstanceRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Added to the error payload."]
    pub struct FailureTrace {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub frames: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Frame>>>,
    }
    impl FailureTrace {
        pub fn builder() -> FailureTraceBuilder {
            FailureTraceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Frame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub target_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workflowGuid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub workflow_guid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zoneId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub zone_id: ::std::option::Option<::std::string::String>,
    }
    impl Frame {
        pub fn builder() -> FrameBuilder {
            FrameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A collection of Bigtable Tables and the resources that serve them. All tables in an instance are served from all Clusters in the instance."]
    pub struct Instance {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The descriptive name for this instance as it appears in UIs. Can be changed at any time, but should be kept globally unique to avoid confusion."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Labels are a flexible and lightweight mechanism for organizing cloud resources into groups that reflect a customer's organizational needs and deployment strategies. They can be used to filter resources and aggregate metrics. * Label keys must be between 1 and 63 characters long and must conform to the regular expression: `\\p{Ll}\\p{Lo}{0,62}`. * Label values must be between 0 and 63 characters long and must conform to the regular expression: `[\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}`. * No more than 64 labels can be associated with a given resource. * Keys and values must both be under 128 bytes."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique name of the instance. Values are of the form `projects/{project}/instances/a-z+[a-z0-9]`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The current state of the instance."]
        pub state: ::std::option::Option<InstanceStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The type of the instance. Defaults to `PRODUCTION`."]
        pub _type: ::std::option::Option<InstanceTypeEnum>,
    }
    impl Instance {
        pub fn builder() -> InstanceBuilder {
            InstanceBuilder::default()
        }
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
    impl ::std::default::Default for InstanceStateEnum {
        fn default() -> Self {
            Self::StateNotKnown
        }
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
    impl ::std::default::Default for InstanceTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Encapsulates progress related information for a Cloud Bigtable long running operation."]
    pub struct OperationProgress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, the time at which this operation failed or was completed successfully."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressPercent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Percent completion of the operation. Values are between 0 and 100 inclusive."]
        pub progress_percent: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time the request was received."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl OperationProgress {
        pub fn builder() -> OperationProgressBuilder {
            OperationProgressBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata type for the long-running operation used to track the progress of optimizations performed on a newly restored table. This long-running operation is automatically created by the system after the successful completion of a table restore, and cannot be cancelled."]
    pub struct OptimizeRestoredTableMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the restored table being optimized."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The progress of the post-restore optimizations."]
        pub progress: ::std::option::Option<::std::boxed::Box<OperationProgress>>,
    }
    impl OptimizeRestoredTableMetadata {
        pub fn builder() -> OptimizeRestoredTableMetadataBuilder {
            OptimizeRestoredTableMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for BigtableInstanceAdmin.PartialUpdateInstance."]
    pub struct PartialUpdateInstanceRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The Instance which will (partially) replace the current value."]
        pub instance: ::std::option::Option<::std::boxed::Box<Instance>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateMask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The subset of Instance fields which should be replaced. Must be explicitly set."]
        pub update_mask: ::std::option::Option<::std::string::String>,
    }
    impl PartialUpdateInstanceRequest {
        pub fn builder() -> PartialUpdateInstanceRequestBuilder {
            PartialUpdateInstanceRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata type for the long-running operation returned by RestoreTable."]
    pub struct RestoreTableMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backupInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub backup_info: ::std::option::Option<::std::boxed::Box<BackupInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the table being created and restored to."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "optimizeTableOperationName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If exists, the name of the long-running operation that will be used to track the post-restore optimization process to optimize the performance of the restored table. The metadata type of the long-running operation is OptimizeRestoreTableMetadata. The response type is Empty. This long-running operation may be automatically created by the system if applicable after the RestoreTable long-running operation completes successfully. This operation may not be created if the table is already optimized or the restore was not successful."]
        pub optimize_table_operation_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The progress of the RestoreTable operation."]
        pub progress: ::std::option::Option<::std::boxed::Box<OperationProgress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the restore source."]
        pub source_type: ::std::option::Option<RestoreTableMetadataSourceTypeEnum>,
    }
    impl RestoreTableMetadata {
        pub fn builder() -> RestoreTableMetadataBuilder {
            RestoreTableMetadataBuilder::default()
        }
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
    impl ::std::default::Default for RestoreTableMetadataSourceTypeEnum {
        fn default() -> Self {
            Self::RestoreSourceTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Progress info for copying a table's data to the new cluster."]
    pub struct TableProgress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "estimatedCopiedBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Estimate of the number of bytes copied so far for this table. This will eventually reach 'estimated_size_bytes' unless the table copy is CANCELLED."]
        pub estimated_copied_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "estimatedSizeBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Estimate of the size of the table to be copied."]
        pub estimated_size_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub state: ::std::option::Option<TableProgressStateEnum>,
    }
    impl TableProgress {
        pub fn builder() -> TableProgressBuilder {
            TableProgressBuilder::default()
        }
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
    impl ::std::default::Default for TableProgressStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The metadata for the Operation returned by UpdateAppProfile."]
    pub struct UpdateAppProfileMetadata {}
    impl UpdateAppProfileMetadata {
        pub fn builder() -> UpdateAppProfileMetadataBuilder {
            UpdateAppProfileMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The metadata for the Operation returned by UpdateCluster."]
    pub struct UpdateClusterMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "finishTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the operation failed or was completed successfully."]
        pub finish_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalRequest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request that prompted the initiation of this UpdateCluster operation."]
        pub original_request: ::std::option::Option<::std::boxed::Box<Cluster>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the original request was received."]
        pub request_time: ::std::option::Option<::std::string::String>,
    }
    impl UpdateClusterMetadata {
        pub fn builder() -> UpdateClusterMetadataBuilder {
            UpdateClusterMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The metadata for the Operation returned by UpdateInstance."]
    pub struct UpdateInstanceMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "finishTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the operation failed or was completed successfully."]
        pub finish_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalRequest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request that prompted the initiation of this UpdateInstance operation."]
        pub original_request:
            ::std::option::Option<::std::boxed::Box<PartialUpdateInstanceRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the original request was received."]
        pub request_time: ::std::option::Option<::std::string::String>,
    }
    impl UpdateInstanceMetadata {
        pub fn builder() -> UpdateInstanceMetadataBuilder {
            UpdateInstanceMetadataBuilder::default()
        }
    }
}
