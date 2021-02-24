#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A backup of a Cloud Spanner database."]
pub struct Backup {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time the CreateBackup request is received. If the request does not specify `version_time`, the `version_time` of the backup will be equivalent to the `create_time`."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "database")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required for the CreateBackup operation. Name of the database from which this backup was created. This needs to be in the same instance as the backup. Values are of the form `projects//instances//databases/`."]
    pub database: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expireTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required for the CreateBackup operation. The expiration time of the backup, with microseconds granularity that must be at least 6 hours and at most 366 days from the time the CreateBackup request is processed. Once the `expire_time` has passed, the backup is eligible to be automatically deleted by Cloud Spanner to free the resources used by the backup."]
    pub expire_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only for the CreateBackup operation. Required for the UpdateBackup operation. A globally unique identifier for the backup which cannot be changed. Values are of the form `projects//instances//backups/a-z*[a-z0-9]` The final segment of the name must be between 2 and 60 characters in length. The backup is stored in the location(s) specified in the instance configuration of the instance containing the backup, identified by the prefix of the backup name of the form `projects//instances/`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "referencingDatabases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The names of the restored databases that reference the backup. The database names are of the form `projects//instances//databases/`. Referencing databases may exist in different instances. The existence of any referencing database prevents the backup from being deleted. When a restored database from the backup enters the `READY` state, the reference to the backup is removed."]
    pub referencing_databases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "sizeBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Size of the backup in bytes."]
    pub size_bytes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The current state of the backup."]
    pub state: ::std::option::Option<BackupStateEnum>,
    #[serde(rename = "versionTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The backup will contain an externally consistent copy of the database at the timestamp specified by `version_time`. If `version_time` is not specified, the system will set `version_time` to the `create_time` of the backup."]
    pub version_time: ::std::option::Option<::std::string::String>,
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
    #[doc = "Name of the backup."]
    pub backup: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time the CreateBackup request was received."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sourceDatabase")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the database the backup was created from."]
    pub source_database: ::std::option::Option<::std::string::String>,
    #[serde(rename = "versionTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The backup contains an externally consistent copy of `source_database` at the timestamp specified by `version_time`. If the CreateBackup request did not specify `version_time`, the `version_time` of the backup is equivalent to the `create_time`."]
    pub version_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for BatchCreateSessions."]
pub struct BatchCreateSessionsRequest {
    #[serde(rename = "sessionCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The number of sessions to be created in this batch call. The API may return fewer than the requested number of sessions. If a specific number of sessions are desired, the client can make additional calls to BatchCreateSessions (adjusting session_count as necessary)."]
    pub session_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "sessionTemplate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parameters to be applied to each created session."]
    pub session_template: ::std::option::Option<::std::boxed::Box<Session>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for BatchCreateSessions."]
pub struct BatchCreateSessionsResponse {
    #[serde(rename = "session")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The freshly created sessions."]
    pub session: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Session>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for BeginTransaction."]
pub struct BeginTransactionRequest {
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Options for the new transaction."]
    pub options: ::std::option::Option<::std::boxed::Box<TransactionOptions>>,
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
#[doc = "Metadata associated with a parent-child relationship appearing in a PlanNode."]
pub struct ChildLink {
    #[serde(rename = "childIndex")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The node to which the link points."]
    pub child_index: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the link. For example, in Hash Joins this could be used to distinguish between the build child and the probe child, or in the case of the child being an output variable, to represent the tag associated with the output variable."]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "variable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only present if the child node is SCALAR and corresponds to an output variable of the parent node. The field carries the name of the output variable. For example, a `TableScan` operator that reads rows from a table will have child links to the `SCALAR` nodes representing the output variables created for each column that is read by the operator. The corresponding `variable` fields will be set to the variable names assigned to the columns."]
    pub variable: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for Commit."]
pub struct CommitRequest {
    #[serde(rename = "mutations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The mutations to be executed when this transaction commits. All mutations are applied atomically, in the order they appear in this list."]
    pub mutations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Mutation>>>,
    #[serde(rename = "returnCommitStats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If `true`, then statistics related to the transaction will be included in the CommitResponse. Default value is `false`."]
    pub return_commit_stats: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "singleUseTransaction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Execute mutations in a temporary transaction. Note that unlike commit of a previously-started transaction, commit with a temporary transaction is non-idempotent. That is, if the `CommitRequest` is sent to Cloud Spanner more than once (for instance, due to retries in the application, or in the transport library), it is possible that the mutations are executed more than once. If this is undesirable, use BeginTransaction and Commit instead."]
    pub single_use_transaction: ::std::option::Option<::std::boxed::Box<TransactionOptions>>,
    #[serde(rename = "transactionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Commit a previously-started transaction."]
    pub transaction_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for Commit."]
pub struct CommitResponse {
    #[serde(rename = "commitStats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The statistics about this Commit. Not returned by default. For more information, see CommitRequest.return_commit_stats."]
    pub commit_stats: ::std::option::Option<::std::boxed::Box<CommitStats>>,
    #[serde(rename = "commitTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Cloud Spanner timestamp at which the transaction committed."]
    pub commit_timestamp: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Additional statistics about a commit."]
pub struct CommitStats {
    #[serde(rename = "mutationCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total number of mutations for the transaction. Knowing the `mutation_count` value can help you maximize the number of mutations in a transaction and minimize the number of API round trips. You can also monitor this value to prevent transactions from exceeding the system [limit](http://cloud.google.com/spanner/quotas#limits_for_creating_reading_updating_and_deleting_data). If the number of mutations exceeds the limit, the server returns [INVALID_ARGUMENT](http://cloud.google.com/spanner/docs/reference/rest/v1/Code#ENUM_VALUES.INVALID_ARGUMENT)."]
    pub mutation_count: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata type for the operation returned by CreateBackup."]
pub struct CreateBackupMetadata {
    #[serde(rename = "cancelTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which cancellation of this operation was received. Operations.CancelOperation starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."]
    pub cancel_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "database")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the database the backup is created from."]
    pub database: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the backup being created."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The progress of the CreateBackup operation."]
    pub progress: ::std::option::Option<::std::boxed::Box<OperationProgress>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata type for the operation returned by CreateDatabase."]
pub struct CreateDatabaseMetadata {
    #[serde(rename = "database")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The database being created."]
    pub database: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for CreateDatabase."]
pub struct CreateDatabaseRequest {
    #[serde(rename = "createStatement")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A `CREATE DATABASE` statement, which specifies the ID of the new database. The database ID must conform to the regular expression `a-z*[a-z0-9]` and be between 2 and 30 characters in length. If the database ID is a reserved word or if it contains a hyphen, the database ID must be enclosed in backticks (`` ` ``)."]
    pub create_statement: ::std::option::Option<::std::string::String>,
    #[serde(rename = "extraStatements")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A list of DDL statements to run inside the newly created database. Statements can create tables, indexes, etc. These statements execute atomically with the creation of the database: if there is an error in any statement, the database is not created."]
    pub extra_statements: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata type for the operation returned by CreateInstance."]
pub struct CreateInstanceMetadata {
    #[serde(rename = "cancelTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which this operation was cancelled. If set, this operation is in the process of undoing itself (which is guaranteed to succeed) and cannot be cancelled again."]
    pub cancel_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which this operation failed or was completed successfully."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "instance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The instance being created."]
    pub instance: ::std::option::Option<::std::boxed::Box<Instance>>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the CreateInstance request was received."]
    pub start_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for CreateInstance."]
pub struct CreateInstanceRequest {
    #[serde(rename = "instance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The instance to create. The name may be omitted, but if specified must be `/instances/`."]
    pub instance: ::std::option::Option<::std::boxed::Box<Instance>>,
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The ID of the instance to create. Valid identifiers are of the form `a-z*[a-z0-9]` and must be between 2 and 64 characters in length."]
    pub instance_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for CreateSession."]
pub struct CreateSessionRequest {
    #[serde(rename = "session")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The session to create."]
    pub session: ::std::option::Option<::std::boxed::Box<Session>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Cloud Spanner database."]
pub struct Database {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. If exists, the time at which the database creation started."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "earliestVersionTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Earliest timestamp at which older versions of the data can be read. This value is continuously updated by Cloud Spanner and becomes stale the moment it is queried. If you are using this value to recover data, make sure to account for the time from the moment when the value is queried to the moment when you initiate the recovery."]
    pub earliest_version_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the database. Values are of the form `projects//instances//databases/`, where `` is as specified in the `CREATE DATABASE` statement. This name can be passed to other API methods to identify the database."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "restoreInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Applicable only for restored databases. Contains information about the restore source."]
    pub restore_info: ::std::option::Option<::std::boxed::Box<RestoreInfo>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The current database state."]
    pub state: ::std::option::Option<DatabaseStateEnum>,
    #[serde(rename = "versionRetentionPeriod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The period in which Cloud Spanner retains all versions of data for the database. This is the same as the value of version_retention_period database option set using UpdateDatabaseDdl. Defaults to 1 hour, if not set."]
    pub version_retention_period: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The current database state."]
pub enum DatabaseStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Not specified."]
    StateUnspecified,
    #[serde(rename = "CREATING")]
    #[doc = "The database is still being created. Operations on the database may fail with `FAILED_PRECONDITION` in this state."]
    Creating,
    #[serde(rename = "READY")]
    #[doc = "The database is fully created and ready for use."]
    Ready,
    #[serde(rename = "READY_OPTIMIZING")]
    #[doc = "The database is fully created and ready for use, but is still being optimized for performance and cannot handle full load. In this state, the database still references the backup it was restore from, preventing the backup from being deleted. When optimizations are complete, the full performance of the database will be restored, and the database will transition to `READY` state."]
    ReadyOptimizing,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Arguments to delete operations."]
pub struct Delete {
    #[serde(rename = "keySet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The primary keys of the rows within table to delete. The primary keys must be specified in the order in which they appear in the `PRIMARY KEY()` clause of the table's equivalent DDL statement (the DDL statement used to create the table). Delete is idempotent. The transaction will succeed even if some or all rows do not exist."]
    pub key_set: ::std::option::Option<::std::boxed::Box<KeySet>>,
    #[serde(rename = "table")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The table whose rows will be deleted."]
    pub table: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for ExecuteBatchDml."]
pub struct ExecuteBatchDmlRequest {
    #[serde(rename = "seqno")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A per-transaction sequence number used to identify this request. This field makes each request idempotent such that if the request is received multiple times, at most one will succeed. The sequence number must be monotonically increasing within the transaction. If a request arrives for the first time with an out-of-order sequence number, the transaction may be aborted. Replays of previously handled requests will yield the same response as the first execution."]
    pub seqno: ::std::option::Option<::std::string::String>,
    #[serde(rename = "statements")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The list of statements to execute in this batch. Statements are executed serially, such that the effects of statement `i` are visible to statement `i+1`. Each statement must be a DML statement. Execution stops at the first failed statement; the remaining statements are not executed. Callers must provide at least one statement."]
    pub statements: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Statement>>>,
    #[serde(rename = "transaction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The transaction to use. Must be a read-write transaction. To protect against replays, single-use transactions are not supported. The caller must either supply an existing transaction ID or begin a new transaction."]
    pub transaction: ::std::option::Option<::std::boxed::Box<TransactionSelector>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for ExecuteBatchDml. Contains a list of ResultSet messages, one for each DML statement that has successfully executed, in the same order as the statements in the request. If a statement fails, the status in the response body identifies the cause of the failure. To check for DML statements that failed, use the following approach: 1. Check the status in the response message. The google.rpc.Code enum value `OK` indicates that all statements were executed successfully. 2. If the status was not `OK`, check the number of result sets in the response. If the response contains `N` ResultSet messages, then statement `N+1` in the request failed. Example 1: * Request: 5 DML statements, all executed successfully. * Response: 5 ResultSet messages, with the status `OK`. Example 2: * Request: 5 DML statements. The third statement has a syntax error. * Response: 2 ResultSet messages, and a syntax error (`INVALID_ARGUMENT`) status. The number of ResultSet messages indicates that the third statement failed, and the fourth and fifth statements were not executed."]
pub struct ExecuteBatchDmlResponse {
    #[serde(rename = "resultSets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "One ResultSet for each statement in the request that ran successfully, in the same order as the statements in the request. Each ResultSet does not contain any rows. The ResultSetStats in each ResultSet contain the number of rows modified by the statement. Only the first ResultSet in the response contains valid ResultSetMetadata."]
    pub result_sets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ResultSet>>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If all DML statements are executed successfully, the status is `OK`. Otherwise, the error status of the first failed statement."]
    pub status: ::std::option::Option<::std::boxed::Box<Status>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for ExecuteSql and ExecuteStreamingSql."]
pub struct ExecuteSqlRequest {
    #[serde(rename = "paramTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "It is not always possible for Cloud Spanner to infer the right SQL type from a JSON value. For example, values of type `BYTES` and values of type `STRING` both appear in params as JSON strings. In these cases, `param_types` can be used to specify the exact SQL type for some or all of the SQL statement parameters. See the definition of Type for more information about SQL types."]
    pub param_types:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<Type>>>,
    #[serde(rename = "params")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parameter names and values that bind to placeholders in the SQL string. A parameter placeholder consists of the `@` character followed by the parameter name (for example, `@firstName`). Parameter names must conform to the naming requirements of identifiers as specified at https://cloud.google.com/spanner/docs/lexical#identifiers. Parameters can appear anywhere that a literal value is expected. The same parameter name can be used more than once, for example: `\"WHERE id > @msg_id AND id < @msg_id + 100\"` It is an error to execute a SQL statement with unbound parameters."]
    pub params: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "partitionToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If present, results will be restricted to the specified partition previously created using PartitionQuery(). There must be an exact match for the values of fields common to this message and the PartitionQueryRequest message used to create this partition_token."]
    pub partition_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "queryMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Used to control the amount of debugging information returned in ResultSetStats. If partition_token is set, query_mode can only be set to QueryMode.NORMAL."]
    pub query_mode: ::std::option::Option<ExecuteSqlRequestQueryModeEnum>,
    #[serde(rename = "queryOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Query optimizer configuration to use for the given query."]
    pub query_options: ::std::option::Option<::std::boxed::Box<QueryOptions>>,
    #[serde(rename = "resumeToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If this request is resuming a previously interrupted SQL statement execution, `resume_token` should be copied from the last PartialResultSet yielded before the interruption. Doing this enables the new SQL statement execution to resume where the last one left off. The rest of the request parameters must exactly match the request that yielded this token."]
    pub resume_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "seqno")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A per-transaction sequence number used to identify this request. This field makes each request idempotent such that if the request is received multiple times, at most one will succeed. The sequence number must be monotonically increasing within the transaction. If a request arrives for the first time with an out-of-order sequence number, the transaction may be aborted. Replays of previously handled requests will yield the same response as the first execution. Required for DML statements. Ignored for queries."]
    pub seqno: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sql")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The SQL string."]
    pub sql: ::std::option::Option<::std::string::String>,
    #[serde(rename = "transaction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The transaction to use. For queries, if none is provided, the default is a temporary read-only transaction with strong concurrency. Standard DML statements require a read-write transaction. To protect against replays, single-use transactions are not supported. The caller must either supply an existing transaction ID or begin a new transaction. Partitioned DML requires an existing Partitioned DML transaction ID."]
    pub transaction: ::std::option::Option<::std::boxed::Box<TransactionSelector>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Used to control the amount of debugging information returned in ResultSetStats. If partition_token is set, query_mode can only be set to QueryMode.NORMAL."]
pub enum ExecuteSqlRequestQueryModeEnum {
    #[serde(rename = "NORMAL")]
    #[doc = "The default mode. Only the statement results are returned."]
    Normal,
    #[serde(rename = "PLAN")]
    #[doc = "This mode returns only the query plan, without any results or execution statistics information."]
    Plan,
    #[serde(rename = "PROFILE")]
    #[doc = "This mode returns both the query plan and the execution statistics along with the results."]
    Profile,
}
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
#[doc = "Message representing a single field of a struct."]
pub struct Field {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the field. For reads, this is the column name. For SQL queries, it is the column alias (e.g., `\"Word\"` in the query `\"SELECT 'hello' AS Word\"`), or the column name (e.g., `\"ColName\"` in the query `\"SELECT ColName FROM Table\"`). Some columns might have an empty name (e.g., `\"SELECT UPPER(ColName)\"`). Note that a query result can contain multiple fields with the same name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the field."]
    pub _type: ::std::option::Option<::std::boxed::Box<Type>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for GetDatabaseDdl."]
pub struct GetDatabaseDdlResponse {
    #[serde(rename = "statements")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of formatted DDL statements defining the schema of the database specified in the request."]
    pub statements: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
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
#[doc = "An isolated set of Cloud Spanner resources on which databases can be hosted."]
pub struct Instance {
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the instance's configuration. Values are of the form `projects//instanceConfigs/`. See also InstanceConfig and ListInstanceConfigs."]
    pub config: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The descriptive name for this instance as it appears in UIs. Must be unique per project and between 4 and 30 characters in length."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endpointUris")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. This field is not populated."]
    pub endpoint_uris: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud Labels are a flexible and lightweight mechanism for organizing cloud resources into groups that reflect a customer's organizational needs and deployment strategies. Cloud Labels can be used to filter collections of resources. They can be used to control how resource metrics are aggregated. And they can be used as arguments to policy management rules (e.g. route, firewall, load balancing, etc.). * Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?`. * Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. * No more than 64 labels can be associated with a given resource. See https://goo.gl/xmQnxf for more information on and examples of labels. If you plan to use labels in your own code, please note that additional characters may be allowed in the future. And so you are advised to use an internal label representation, such as JSON, which doesn't rely upon specific characters being disallowed. For example, representing labels as the string: name + \"_\" + value would prove problematic if we were to allow \"_\" in a future release."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A unique identifier for the instance, which cannot be changed after the instance is created. Values are of the form `projects//instances/a-z*[a-z0-9]`. The final segment of the name must be between 2 and 64 characters in length."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nodeCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of nodes allocated to this instance. This may be zero in API responses for instances that are not yet in state `READY`. See [the documentation](https://cloud.google.com/spanner/docs/instances#node_count) for more information about nodes."]
    pub node_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The current instance state. For CreateInstance, the state must be either omitted or set to `CREATING`. For UpdateInstance, the state must be either omitted or set to `READY`."]
    pub state: ::std::option::Option<InstanceStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The current instance state. For CreateInstance, the state must be either omitted or set to `CREATING`. For UpdateInstance, the state must be either omitted or set to `READY`."]
pub enum InstanceStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Not specified."]
    StateUnspecified,
    #[serde(rename = "CREATING")]
    #[doc = "The instance is still being created. Resources may not be available yet, and operations such as database creation may not work."]
    Creating,
    #[serde(rename = "READY")]
    #[doc = "The instance is fully created and ready to do work such as creating databases."]
    Ready,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A possible configuration for a Cloud Spanner instance. Configurations define the geographic placement of nodes and their replication."]
pub struct InstanceConfig {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of this instance configuration as it appears in UIs."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A unique identifier for the instance configuration. Values are of the form `projects//instanceConfigs/a-z*`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "replicas")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The geographic placement of nodes in this instance configuration and their replication properties."]
    pub replicas: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ReplicaInfo>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "KeyRange represents a range of rows in a table or index. A range has a start key and an end key. These keys can be open or closed, indicating if the range includes rows with that key. Keys are represented by lists, where the ith value in the list corresponds to the ith component of the table or index primary key. Individual values are encoded as described here. For example, consider the following table definition: CREATE TABLE UserEvents ( UserName STRING(MAX), EventDate STRING(10) ) PRIMARY KEY(UserName, EventDate); The following keys name rows in this table: \"Bob\", \"2014-09-23\" Since the `UserEvents` table's `PRIMARY KEY` clause names two columns, each `UserEvents` key has two elements; the first is the `UserName`, and the second is the `EventDate`. Key ranges with multiple components are interpreted lexicographically by component using the table or index key's declared sort order. For example, the following range returns all events for user `\"Bob\"` that occurred in the year 2015: \"start_closed\": [\"Bob\", \"2015-01-01\"] \"end_closed\": [\"Bob\", \"2015-12-31\"] Start and end keys can omit trailing key components. This affects the inclusion and exclusion of rows that exactly match the provided key components: if the key is closed, then rows that exactly match the provided components are included; if the key is open, then rows that exactly match are not included. For example, the following range includes all events for `\"Bob\"` that occurred during and after the year 2000: \"start_closed\": [\"Bob\", \"2000-01-01\"] \"end_closed\": [\"Bob\"] The next example retrieves all events for `\"Bob\"`: \"start_closed\": [\"Bob\"] \"end_closed\": [\"Bob\"] To retrieve events before the year 2000: \"start_closed\": [\"Bob\"] \"end_open\": [\"Bob\", \"2000-01-01\"] The following range includes all rows in the table: \"start_closed\": [] \"end_closed\": [] This range returns all users whose `UserName` begins with any character from A to C: \"start_closed\": [\"A\"] \"end_open\": [\"D\"] This range returns all users whose `UserName` begins with B: \"start_closed\": [\"B\"] \"end_open\": [\"C\"] Key ranges honor column sort order. For example, suppose a table is defined as follows: CREATE TABLE DescendingSortedTable { Key INT64, ... ) PRIMARY KEY(Key DESC); The following range retrieves all rows with key values between 1 and 100 inclusive: \"start_closed\": [\"100\"] \"end_closed\": [\"1\"] Note that 100 is passed as the start, and 1 is passed as the end, because `Key` is a descending column in the schema."]
pub struct KeyRange {
    #[serde(rename = "endClosed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the end is closed, then the range includes all rows whose first `len(end_closed)` key columns exactly match `end_closed`."]
    pub end_closed: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
    #[serde(rename = "endOpen")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the end is open, then the range excludes rows whose first `len(end_open)` key columns exactly match `end_open`."]
    pub end_open: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
    #[serde(rename = "startClosed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the start is closed, then the range includes all rows whose first `len(start_closed)` key columns exactly match `start_closed`."]
    pub start_closed: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
    #[serde(rename = "startOpen")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the start is open, then the range excludes rows whose first `len(start_open)` key columns exactly match `start_open`."]
    pub start_open: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`KeySet` defines a collection of Cloud Spanner keys and/or key ranges. All the keys are expected to be in the same table or index. The keys need not be sorted in any particular way. If the same key is specified multiple times in the set (for example if two ranges, two keys, or a key and a range overlap), Cloud Spanner behaves as if the key were only specified once."]
pub struct KeySet {
    #[serde(rename = "all")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For convenience `all` can be set to `true` to indicate that this `KeySet` matches all keys in the table or index. Note that any keys specified in `keys` or `ranges` are only yielded once."]
    pub all: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "keys")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of specific keys. Entries in `keys` should have exactly as many elements as there are columns in the primary or index key with which this `KeySet` is used. Individual key values are encoded as described here."]
    pub keys: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::serde_json::Value>>>,
    #[serde(rename = "ranges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of key ranges. See KeyRange for more information about key range specifications."]
    pub ranges: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<KeyRange>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for ListBackupOperations."]
pub struct ListBackupOperationsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "`next_page_token` can be sent in a subsequent ListBackupOperations call to fetch more of the matching metadata."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of matching backup long-running operations. Each operation's name will be prefixed by the backup's name and the operation's metadata will be of type CreateBackupMetadata. Operations returned include those that are pending or have completed/failed/canceled within the last 7 days. Operations returned are ordered by `operation.metadata.value.progress.start_time` in descending order starting from the most recently started operation."]
    pub operations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Operation>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for ListBackups."]
pub struct ListBackupsResponse {
    #[serde(rename = "backups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of matching backups. Backups returned are ordered by `create_time` in descending order, starting from the most recent `create_time`."]
    pub backups: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Backup>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "`next_page_token` can be sent in a subsequent ListBackups call to fetch more of the matching backups."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for ListDatabaseOperations."]
pub struct ListDatabaseOperationsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "`next_page_token` can be sent in a subsequent ListDatabaseOperations call to fetch more of the matching metadata."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of matching database long-running operations. Each operation's name will be prefixed by the database's name. The operation's metadata field type `metadata.type_url` describes the type of the metadata."]
    pub operations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Operation>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for ListDatabases."]
pub struct ListDatabasesResponse {
    #[serde(rename = "databases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Databases that matched the request."]
    pub databases: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Database>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "`next_page_token` can be sent in a subsequent ListDatabases call to fetch more of the matching databases."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for ListInstanceConfigs."]
pub struct ListInstanceConfigsResponse {
    #[serde(rename = "instanceConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of requested instance configurations."]
    pub instance_configs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<InstanceConfig>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "`next_page_token` can be sent in a subsequent ListInstanceConfigs call to fetch more of the matching instance configurations."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for ListInstances."]
pub struct ListInstancesResponse {
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of requested instances."]
    pub instances: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Instance>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "`next_page_token` can be sent in a subsequent ListInstances call to fetch more of the matching instances."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "unreachable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of unreachable instances. It includes the names of instances whose metadata could not be retrieved within instance_deadline."]
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
#[doc = "The response for ListSessions."]
pub struct ListSessionsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "`next_page_token` can be sent in a subsequent ListSessions call to fetch more of the matching sessions."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sessions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of requested sessions."]
    pub sessions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Session>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A modification to one or more Cloud Spanner rows. Mutations can be applied to a Cloud Spanner database by sending them in a Commit call."]
pub struct Mutation {
    #[serde(rename = "delete")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Delete rows from a table. Succeeds whether or not the named rows were present."]
    pub delete: ::std::option::Option<::std::boxed::Box<Delete>>,
    #[serde(rename = "insert")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Insert new rows in a table. If any of the rows already exist, the write or transaction fails with error `ALREADY_EXISTS`."]
    pub insert: ::std::option::Option<::std::boxed::Box<Write>>,
    #[serde(rename = "insertOrUpdate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Like insert, except that if the row already exists, then its column values are overwritten with the ones provided. Any column values not explicitly written are preserved. When using insert_or_update, just as when using insert, all `NOT NULL` columns in the table must be given a value. This holds true even when the row already exists and will therefore actually be updated."]
    pub insert_or_update: ::std::option::Option<::std::boxed::Box<Write>>,
    #[serde(rename = "replace")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Like insert, except that if the row already exists, it is deleted, and the column values provided are inserted instead. Unlike insert_or_update, this means any values not explicitly written become `NULL`. In an interleaved table, if you create the child table with the `ON DELETE CASCADE` annotation, then replacing a parent row also deletes the child rows. Otherwise, you must delete the child rows before you replace the parent row."]
    pub replace: ::std::option::Option<::std::boxed::Box<Write>>,
    #[serde(rename = "update")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Update existing rows in a table. If any of the rows does not already exist, the transaction fails with error `NOT_FOUND`."]
    pub update: ::std::option::Option<::std::boxed::Box<Write>>,
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
#[doc = "Encapsulates progress related information for a Cloud Spanner long running operation."]
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
#[doc = "Metadata type for the long-running operation used to track the progress of optimizations performed on a newly restored database. This long-running operation is automatically created by the system after the successful completion of a database restore, and cannot be cancelled."]
pub struct OptimizeRestoredDatabaseMetadata {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the restored database being optimized."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The progress of the post-restore optimizations."]
    pub progress: ::std::option::Option<::std::boxed::Box<OperationProgress>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Partial results from a streaming read or SQL query. Streaming reads and SQL queries better tolerate large result sets, large rows, and large values, but are a little trickier to consume."]
pub struct PartialResultSet {
    #[serde(rename = "chunkedValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, then the final value in values is chunked, and must be combined with more values from subsequent `PartialResultSet`s to obtain a complete field value."]
    pub chunked_value: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the result set, such as row type information. Only present in the first response."]
    pub metadata: ::std::option::Option<::std::boxed::Box<ResultSetMetadata>>,
    #[serde(rename = "resumeToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Streaming calls might be interrupted for a variety of reasons, such as TCP connection loss. If this occurs, the stream of results can be resumed by re-sending the original request and including `resume_token`. Note that executing any other transaction in the same session invalidates the token."]
    pub resume_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "stats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Query plan and execution statistics for the statement that produced this streaming result set. These can be requested by setting ExecuteSqlRequest.query_mode and are sent only once with the last response in the stream. This field will also be present in the last response for DML statements."]
    pub stats: ::std::option::Option<::std::boxed::Box<ResultSetStats>>,
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A streamed result set consists of a stream of values, which might be split into many `PartialResultSet` messages to accommodate large rows and/or large values. Every N complete values defines a row, where N is equal to the number of entries in metadata.row_type.fields. Most values are encoded based on type as described here. It is possible that the last value in values is \"chunked\", meaning that the rest of the value is sent in subsequent `PartialResultSet`(s). This is denoted by the chunked_value field. Two or more chunked values can be merged to form a complete value as follows: * `bool/number/null`: cannot be chunked * `string`: concatenate the strings * `list`: concatenate the lists. If the last element in a list is a `string`, `list`, or `object`, merge it with the first element in the next list by applying these rules recursively. * `object`: concatenate the (field name, field value) pairs. If a field name is duplicated, then apply these rules recursively to merge the field values. Some examples of merging: # Strings are concatenated. \"foo\", \"bar\" => \"foobar\" # Lists of non-strings are concatenated. [2, 3], [4] => [2, 3, 4] # Lists are concatenated, but the last and first elements are merged # because they are strings. [\"a\", \"b\"], [\"c\", \"d\"] => [\"a\", \"bc\", \"d\"] # Lists are concatenated, but the last and first elements are merged # because they are lists. Recursively, the last and first elements # of the inner lists are merged because they are strings. [\"a\", [\"b\", \"c\"]], [[\"d\"], \"e\"] => [\"a\", [\"b\", \"cd\"], \"e\"] # Non-overlapping object fields are combined. {\"a\": \"1\"}, {\"b\": \"2\"} => {\"a\": \"1\", \"b\": 2\"} # Overlapping object fields are merged. {\"a\": \"1\"}, {\"a\": \"2\"} => {\"a\": \"12\"} # Examples of merging objects containing lists of strings. {\"a\": [\"1\"]}, {\"a\": [\"2\"]} => {\"a\": [\"12\"]} For a more complete example, suppose a streaming SQL query is yielding a result set whose rows contain a single string field. The following `PartialResultSet`s might be yielded: { \"metadata\": { ... } \"values\": [\"Hello\", \"W\"] \"chunked_value\": true \"resume_token\": \"Af65...\" } { \"values\": [\"orl\"] \"chunked_value\": true \"resume_token\": \"Bqp2...\" } { \"values\": [\"d\"] \"resume_token\": \"Zx1B...\" } This sequence of `PartialResultSet`s encodes two rows, one containing the field value `\"Hello\"`, and a second containing the field value `\"World\" = \"W\" + \"orl\" + \"d\"`."]
    pub values: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information returned for each partition returned in a PartitionResponse."]
pub struct Partition {
    #[serde(rename = "partitionToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This token can be passed to Read, StreamingRead, ExecuteSql, or ExecuteStreamingSql requests to restrict the results to those identified by this partition token."]
    pub partition_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Options for a PartitionQueryRequest and PartitionReadRequest."]
pub struct PartitionOptions {
    #[serde(rename = "maxPartitions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "**Note:** This hint is currently ignored by PartitionQuery and PartitionRead requests. The desired maximum number of partitions to return. For example, this may be set to the number of workers available. The default for this option is currently 10,000. The maximum value is currently 200,000. This is only a hint. The actual number of partitions returned may be smaller or larger than this maximum count request."]
    pub max_partitions: ::std::option::Option<::std::string::String>,
    #[serde(rename = "partitionSizeBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "**Note:** This hint is currently ignored by PartitionQuery and PartitionRead requests. The desired data size for each partition generated. The default for this option is currently 1 GiB. This is only a hint. The actual size of each partition may be smaller or larger than this size request."]
    pub partition_size_bytes: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for PartitionQuery"]
pub struct PartitionQueryRequest {
    #[serde(rename = "paramTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "It is not always possible for Cloud Spanner to infer the right SQL type from a JSON value. For example, values of type `BYTES` and values of type `STRING` both appear in params as JSON strings. In these cases, `param_types` can be used to specify the exact SQL type for some or all of the SQL query parameters. See the definition of Type for more information about SQL types."]
    pub param_types:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<Type>>>,
    #[serde(rename = "params")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parameter names and values that bind to placeholders in the SQL string. A parameter placeholder consists of the `@` character followed by the parameter name (for example, `@firstName`). Parameter names can contain letters, numbers, and underscores. Parameters can appear anywhere that a literal value is expected. The same parameter name can be used more than once, for example: `\"WHERE id > @msg_id AND id < @msg_id + 100\"` It is an error to execute a SQL statement with unbound parameters."]
    pub params: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "partitionOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional options that affect how many partitions are created."]
    pub partition_options: ::std::option::Option<::std::boxed::Box<PartitionOptions>>,
    #[serde(rename = "sql")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The query request to generate partitions for. The request will fail if the query is not root partitionable. The query plan of a root partitionable query has a single distributed union operator. A distributed union operator conceptually divides one or more tables into multiple splits, remotely evaluates a subquery independently on each split, and then unions all results. This must not contain DML commands, such as INSERT, UPDATE, or DELETE. Use ExecuteStreamingSql with a PartitionedDml transaction for large, partition-friendly DML operations."]
    pub sql: ::std::option::Option<::std::string::String>,
    #[serde(rename = "transaction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Read only snapshot transactions are supported, read/write and single use transactions are not."]
    pub transaction: ::std::option::Option<::std::boxed::Box<TransactionSelector>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for PartitionRead"]
pub struct PartitionReadRequest {
    #[serde(rename = "columns")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The columns of table to be returned for each row matching this request."]
    pub columns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "index")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If non-empty, the name of an index on table. This index is used instead of the table primary key when interpreting key_set and sorting result rows. See key_set for further information."]
    pub index: ::std::option::Option<::std::string::String>,
    #[serde(rename = "keySet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. `key_set` identifies the rows to be yielded. `key_set` names the primary keys of the rows in table to be yielded, unless index is present. If index is present, then key_set instead names index keys in index. It is not an error for the `key_set` to name rows that do not exist in the database. Read yields nothing for nonexistent rows."]
    pub key_set: ::std::option::Option<::std::boxed::Box<KeySet>>,
    #[serde(rename = "partitionOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional options that affect how many partitions are created."]
    pub partition_options: ::std::option::Option<::std::boxed::Box<PartitionOptions>>,
    #[serde(rename = "table")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the table in the database to be read."]
    pub table: ::std::option::Option<::std::string::String>,
    #[serde(rename = "transaction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Read only snapshot transactions are supported, read/write and single use transactions are not."]
    pub transaction: ::std::option::Option<::std::boxed::Box<TransactionSelector>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for PartitionQuery or PartitionRead"]
pub struct PartitionResponse {
    #[serde(rename = "partitions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Partitions created by this request."]
    pub partitions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Partition>>>,
    #[serde(rename = "transaction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Transaction created by this request."]
    pub transaction: ::std::option::Option<::std::boxed::Box<Transaction>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message type to initiate a Partitioned DML transaction."]
pub struct PartitionedDml {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Node information for nodes appearing in a QueryPlan.plan_nodes."]
pub struct PlanNode {
    #[serde(rename = "childLinks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of child node `index`es and their relationship to this parent."]
    pub child_links: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ChildLink>>>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display name for the node."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "executionStats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The execution statistics associated with the node, contained in a group of key-value pairs. Only present if the plan was returned as a result of a profile query. For example, number of executions, number of rows/time per execution etc."]
    pub execution_stats:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "index")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `PlanNode`'s index in node list."]
    pub index: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Used to determine the type of node. May be needed for visualizing different kinds of nodes differently. For example, If the node is a SCALAR node, it will have a condensed representation which can be used to directly embed a description of the node in its parent."]
    pub kind: ::std::option::Option<PlanNodeKindEnum>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Attributes relevant to the node contained in a group of key-value pairs. For example, a Parameter Reference node could have the following information in its metadata: { \"parameter_reference\": \"param1\", \"parameter_type\": \"array\" }"]
    pub metadata: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "shortRepresentation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Condensed representation for SCALAR nodes."]
    pub short_representation: ::std::option::Option<::std::boxed::Box<ShortRepresentation>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Used to determine the type of node. May be needed for visualizing different kinds of nodes differently. For example, If the node is a SCALAR node, it will have a condensed representation which can be used to directly embed a description of the node in its parent."]
pub enum PlanNodeKindEnum {
    #[serde(rename = "KIND_UNSPECIFIED")]
    #[doc = "Not specified."]
    KindUnspecified,
    #[serde(rename = "RELATIONAL")]
    #[doc = "Denotes a Relational operator node in the expression tree. Relational operators represent iterative processing of rows during query execution. For example, a `TableScan` operation that reads rows from a table."]
    Relational,
    #[serde(rename = "SCALAR")]
    #[doc = "Denotes a Scalar node in the expression tree. Scalar nodes represent non-iterable entities in the query plan. For example, constants or arithmetic operators appearing inside predicate expressions or references to column names."]
    Scalar,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members` to a single `role`. Members can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { \"bindings\": [ { \"role\": \"roles/resourcemanager.organizationAdmin\", \"members\": [ \"user:mike@example.com\", \"group:admins@example.com\", \"domain:google.com\", \"serviceAccount:my-project-id@appspot.gserviceaccount.com\" ] }, { \"role\": \"roles/resourcemanager.organizationViewer\", \"members\": [ \"user:eve@example.com\" ], \"condition\": { \"title\": \"expirable access\", \"description\": \"Does not grant access after Sep 2020\", \"expression\": \"request.time < timestamp('2020-10-01T00:00:00.000Z')\", } } ], \"etag\": \"BwWWja0YfJA=\", \"version\": 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') - etag: BwWWja0YfJA= - version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/)."]
pub struct Policy {
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
#[doc = "Query optimizer configuration."]
pub struct QueryOptions {
    #[serde(rename = "optimizerVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An option to control the selection of optimizer version. This parameter allows individual queries to pick different query optimizer versions. Specifying \"latest\" as a value instructs Cloud Spanner to use the latest supported query optimizer version. If not specified, Cloud Spanner uses optimizer version set at the database level options. Any other positive integer (from the list of supported optimizer versions) overrides the default optimizer version for query execution. The list of supported optimizer versions can be queried from SPANNER_SYS.SUPPORTED_OPTIMIZER_VERSIONS. Executing a SQL statement with an invalid optimizer version will fail with a syntax error (`INVALID_ARGUMENT`) status. See https://cloud.google.com/spanner/docs/query-optimizer/manage-query-optimizer for more information on managing the query optimizer. The `optimizer_version` statement hint has precedence over this setting."]
    pub optimizer_version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains an ordered list of nodes appearing in the query plan."]
pub struct QueryPlan {
    #[serde(rename = "planNodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The nodes in the query plan. Plan nodes are returned in pre-order starting with the plan root. Each PlanNode's `id` corresponds to its index in `plan_nodes`."]
    pub plan_nodes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PlanNode>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message type to initiate a read-only transaction."]
pub struct ReadOnly {
    #[serde(rename = "exactStaleness")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Executes all reads at a timestamp that is `exact_staleness` old. The timestamp is chosen soon after the read is started. Guarantees that all writes that have committed more than the specified number of seconds ago are visible. Because Cloud Spanner chooses the exact timestamp, this mode works even if the client's local clock is substantially skewed from Cloud Spanner commit timestamps. Useful for reading at nearby replicas without the distributed timestamp negotiation overhead of `max_staleness`."]
    pub exact_staleness: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maxStaleness")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Read data at a timestamp >= `NOW - max_staleness` seconds. Guarantees that all writes that have committed more than the specified number of seconds ago are visible. Because Cloud Spanner chooses the exact timestamp, this mode works even if the client's local clock is substantially skewed from Cloud Spanner commit timestamps. Useful for reading the freshest data available at a nearby replica, while bounding the possible staleness if the local replica has fallen behind. Note that this option can only be used in single-use transactions."]
    pub max_staleness: ::std::option::Option<::std::string::String>,
    #[serde(rename = "minReadTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Executes all reads at a timestamp >= `min_read_timestamp`. This is useful for requesting fresher data than some previous read, or data that is fresh enough to observe the effects of some previously committed transaction whose timestamp is known. Note that this option can only be used in single-use transactions. A timestamp in RFC3339 UTC \\\"Zulu\\\" format, accurate to nanoseconds. Example: `\"2014-10-02T15:01:23.045123456Z\"`."]
    pub min_read_timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "readTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Executes all reads at the given timestamp. Unlike other modes, reads at a specific timestamp are repeatable; the same read at the same timestamp always returns the same data. If the timestamp is in the future, the read will block until the specified timestamp, modulo the read's deadline. Useful for large scale consistent reads such as mapreduces, or for coordinating many reads against a consistent snapshot of the data. A timestamp in RFC3339 UTC \\\"Zulu\\\" format, accurate to nanoseconds. Example: `\"2014-10-02T15:01:23.045123456Z\"`."]
    pub read_timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "returnReadTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, the Cloud Spanner-selected read timestamp is included in the Transaction message that describes the transaction."]
    pub return_read_timestamp: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "strong")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Read at a timestamp where all previously committed transactions are visible."]
    pub strong: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for Read and StreamingRead."]
pub struct ReadRequest {
    #[serde(rename = "columns")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The columns of table to be returned for each row matching this request."]
    pub columns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "index")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If non-empty, the name of an index on table. This index is used instead of the table primary key when interpreting key_set and sorting result rows. See key_set for further information."]
    pub index: ::std::option::Option<::std::string::String>,
    #[serde(rename = "keySet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. `key_set` identifies the rows to be yielded. `key_set` names the primary keys of the rows in table to be yielded, unless index is present. If index is present, then key_set instead names index keys in index. If the partition_token field is empty, rows are yielded in table primary key order (if index is empty) or index key order (if index is non-empty). If the partition_token field is not empty, rows will be yielded in an unspecified order. It is not an error for the `key_set` to name rows that do not exist in the database. Read yields nothing for nonexistent rows."]
    pub key_set: ::std::option::Option<::std::boxed::Box<KeySet>>,
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If greater than zero, only the first `limit` rows are yielded. If `limit` is zero, the default is no limit. A limit cannot be specified if `partition_token` is set."]
    pub limit: ::std::option::Option<::std::string::String>,
    #[serde(rename = "partitionToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If present, results will be restricted to the specified partition previously created using PartitionRead(). There must be an exact match for the values of fields common to this message and the PartitionReadRequest message used to create this partition_token."]
    pub partition_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resumeToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If this request is resuming a previously interrupted read, `resume_token` should be copied from the last PartialResultSet yielded before the interruption. Doing this enables the new read to resume where the last read left off. The rest of the request parameters must exactly match the request that yielded this token."]
    pub resume_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "table")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the table in the database to be read."]
    pub table: ::std::option::Option<::std::string::String>,
    #[serde(rename = "transaction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The transaction to use. If none is provided, the default is a temporary read-only transaction with strong concurrency."]
    pub transaction: ::std::option::Option<::std::boxed::Box<TransactionSelector>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message type to initiate a read-write transaction. Currently this transaction type has no options."]
pub struct ReadWrite {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplicaInfo {
    #[serde(rename = "defaultLeaderLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, this location is designated as the default leader location where leader replicas are placed. See the [region types documentation](https://cloud.google.com/spanner/docs/instances#region_types) for more details."]
    pub default_leader_location: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The location of the serving resources, e.g. \"us-central1\"."]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of replica."]
    pub _type: ::std::option::Option<ReplicaInfoTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of replica."]
pub enum ReplicaInfoTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "Not specified."]
    TypeUnspecified,
    #[serde(rename = "READ_WRITE")]
    #[doc = "Read-write replicas support both reads and writes. These replicas: * Maintain a full copy of your data. * Serve reads. * Can vote whether to commit a write. * Participate in leadership election. * Are eligible to become a leader."]
    ReadWrite,
    #[serde(rename = "READ_ONLY")]
    #[doc = "Read-only replicas only support reads (not writes). Read-only replicas: * Maintain a full copy of your data. * Serve reads. * Do not participate in voting to commit writes. * Are not eligible to become a leader."]
    ReadOnly,
    #[serde(rename = "WITNESS")]
    #[doc = "Witness replicas don't support reads but do participate in voting to commit writes. Witness replicas: * Do not maintain a full copy of data. * Do not serve reads. * Vote whether to commit writes. * Participate in leader election but are not eligible to become leader."]
    Witness,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata type for the long-running operation returned by RestoreDatabase."]
pub struct RestoreDatabaseMetadata {
    #[serde(rename = "backupInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the backup used to restore the database."]
    pub backup_info: ::std::option::Option<::std::boxed::Box<BackupInfo>>,
    #[serde(rename = "cancelTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which cancellation of this operation was received. Operations.CancelOperation starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."]
    pub cancel_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the database being created and restored to."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "optimizeDatabaseOperationName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If exists, the name of the long-running operation that will be used to track the post-restore optimization process to optimize the performance of the restored database, and remove the dependency on the restore source. The name is of the form `projects//instances//databases//operations/` where the is the name of database being created and restored to. The metadata type of the long-running operation is OptimizeRestoredDatabaseMetadata. This long-running operation will be automatically created by the system after the RestoreDatabase long-running operation completes successfully. This operation will not be created if the restore was not successful."]
    pub optimize_database_operation_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The progress of the RestoreDatabase operation."]
    pub progress: ::std::option::Option<::std::boxed::Box<OperationProgress>>,
    #[serde(rename = "sourceType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the restore source."]
    pub source_type: ::std::option::Option<RestoreDatabaseMetadataSourceTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the restore source."]
pub enum RestoreDatabaseMetadataSourceTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "No restore associated."]
    TypeUnspecified,
    #[serde(rename = "BACKUP")]
    #[doc = "A backup was used as the source of the restore."]
    Backup,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for RestoreDatabase."]
pub struct RestoreDatabaseRequest {
    #[serde(rename = "backup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the backup from which to restore. Values are of the form `projects//instances//backups/`."]
    pub backup: ::std::option::Option<::std::string::String>,
    #[serde(rename = "databaseId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The id of the database to create and restore to. This database must not already exist. The `database_id` appended to `parent` forms the full database name of the form `projects//instances//databases/`."]
    pub database_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about the database restore."]
pub struct RestoreInfo {
    #[serde(rename = "backupInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the backup used to restore the database. The backup may no longer exist."]
    pub backup_info: ::std::option::Option<::std::boxed::Box<BackupInfo>>,
    #[serde(rename = "sourceType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the restore source."]
    pub source_type: ::std::option::Option<RestoreInfoSourceTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the restore source."]
pub enum RestoreInfoSourceTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "No restore associated."]
    TypeUnspecified,
    #[serde(rename = "BACKUP")]
    #[doc = "A backup was used as the source of the restore."]
    Backup,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Results from Read or ExecuteSql."]
pub struct ResultSet {
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the result set, such as row type information."]
    pub metadata: ::std::option::Option<::std::boxed::Box<ResultSetMetadata>>,
    #[serde(rename = "rows")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Each element in `rows` is a row whose format is defined by metadata.row_type. The ith element in each row matches the ith field in metadata.row_type. Elements are encoded based on type as described here."]
    pub rows: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::serde_json::Value>>>,
    #[serde(rename = "stats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Query plan and execution statistics for the SQL statement that produced this result set. These can be requested by setting ExecuteSqlRequest.query_mode. DML statements always produce stats containing the number of rows modified, unless executed using the ExecuteSqlRequest.QueryMode.PLAN ExecuteSqlRequest.query_mode. Other fields may or may not be populated, based on the ExecuteSqlRequest.query_mode."]
    pub stats: ::std::option::Option<::std::boxed::Box<ResultSetStats>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata about a ResultSet or PartialResultSet."]
pub struct ResultSetMetadata {
    #[serde(rename = "rowType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates the field names and types for the rows in the result set. For example, a SQL query like `\"SELECT UserId, UserName FROM Users\"` could return a `row_type` value like: \"fields\": [ { \"name\": \"UserId\", \"type\": { \"code\": \"INT64\" } }, { \"name\": \"UserName\", \"type\": { \"code\": \"STRING\" } }, ]"]
    pub row_type: ::std::option::Option<::std::boxed::Box<StructType>>,
    #[serde(rename = "transaction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the read or SQL query began a transaction as a side-effect, the information about the new transaction is yielded here."]
    pub transaction: ::std::option::Option<::std::boxed::Box<Transaction>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Additional statistics about a ResultSet or PartialResultSet."]
pub struct ResultSetStats {
    #[serde(rename = "queryPlan")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "QueryPlan for the query associated with this result."]
    pub query_plan: ::std::option::Option<::std::boxed::Box<QueryPlan>>,
    #[serde(rename = "queryStats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Aggregated statistics from the execution of the query. Only present when the query is profiled. For example, a query could return the statistics as follows: { \"rows_returned\": \"3\", \"elapsed_time\": \"1.22 secs\", \"cpu_time\": \"1.19 secs\" }"]
    pub query_stats:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "rowCountExact")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Standard DML returns an exact count of rows that were modified."]
    pub row_count_exact: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rowCountLowerBound")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Partitioned DML does not offer exactly-once semantics, so it returns a lower bound of the rows modified."]
    pub row_count_lower_bound: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for Rollback."]
pub struct RollbackRequest {
    #[serde(rename = "transactionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The transaction to roll back."]
    pub transaction_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A session in the Cloud Spanner API."]
pub struct Session {
    #[serde(rename = "approximateLastUseTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The approximate timestamp when the session is last used. It is typically earlier than the actual last use time."]
    pub approximate_last_use_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The timestamp when the session is created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The labels for the session. * Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?`. * Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`. * No more than 64 labels can be associated with a given session. See https://goo.gl/xmQnxf for more information on and examples of labels."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The name of the session. This is always system-assigned."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for `SetIamPolicy` method."]
pub struct SetIamPolicyRequest {
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Cloud Platform services (such as Projects) might reject them."]
    pub policy: ::std::option::Option<::std::boxed::Box<Policy>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Condensed representation of a node and its subtree. Only present for `SCALAR` PlanNode(s)."]
pub struct ShortRepresentation {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A string representation of the expression subtree rooted at this node."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subqueries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mapping of (subquery variable name) -> (subquery node id) for cases where the `description` string of this node references a `SCALAR` subquery contained in the expression subtree rooted at this node. The referenced `SCALAR` subquery may not necessarily be a direct child of this node."]
    pub subqueries:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::primitive::i64>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single DML statement."]
pub struct Statement {
    #[serde(rename = "paramTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "It is not always possible for Cloud Spanner to infer the right SQL type from a JSON value. For example, values of type `BYTES` and values of type `STRING` both appear in params as JSON strings. In these cases, `param_types` can be used to specify the exact SQL type for some or all of the SQL statement parameters. See the definition of Type for more information about SQL types."]
    pub param_types:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<Type>>>,
    #[serde(rename = "params")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Parameter names and values that bind to placeholders in the DML string. A parameter placeholder consists of the `@` character followed by the parameter name (for example, `@firstName`). Parameter names can contain letters, numbers, and underscores. Parameters can appear anywhere that a literal value is expected. The same parameter name can be used more than once, for example: `\"WHERE id > @msg_id AND id < @msg_id + 100\"` It is an error to execute a SQL statement with unbound parameters."]
    pub params: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "sql")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The DML string."]
    pub sql: ::std::option::Option<::std::string::String>,
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
#[doc = "`StructType` defines the fields of a STRUCT type."]
pub struct StructType {
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of fields that make up this struct. Order is significant, because values of this struct type are represented as lists, where the order of field values matches the order of fields in the StructType. In turn, the order of fields matches the order of columns in a read request, or the order of fields in the `SELECT` clause of a query."]
    pub fields: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Field>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for `TestIamPermissions` method."]
pub struct TestIamPermissionsRequest {
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "REQUIRED: The set of permissions to check for 'resource'. Permissions with wildcards (such as '*', 'spanner.*', 'spanner.instances.*') are not allowed."]
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
#[doc = "A transaction."]
pub struct Transaction {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "`id` may be used to identify the transaction in subsequent Read, ExecuteSql, Commit, or Rollback calls. Single-use read-only transactions do not have IDs, because single-use transactions do not support multiple requests."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "readTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For snapshot read-only transactions, the read timestamp chosen for the transaction. Not returned by default: see TransactionOptions.ReadOnly.return_read_timestamp. A timestamp in RFC3339 UTC \\\"Zulu\\\" format, accurate to nanoseconds. Example: `\"2014-10-02T15:01:23.045123456Z\"`."]
    pub read_timestamp: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "# Transactions Each session can have at most one active transaction at a time (note that standalone reads and queries use a transaction internally and do count towards the one transaction limit). After the active transaction is completed, the session can immediately be re-used for the next transaction. It is not necessary to create a new session for each transaction. # Transaction Modes Cloud Spanner supports three transaction modes: 1. Locking read-write. This type of transaction is the only way to write data into Cloud Spanner. These transactions rely on pessimistic locking and, if necessary, two-phase commit. Locking read-write transactions may abort, requiring the application to retry. 2. Snapshot read-only. This transaction type provides guaranteed consistency across several reads, but does not allow writes. Snapshot read-only transactions can be configured to read at timestamps in the past. Snapshot read-only transactions do not need to be committed. 3. Partitioned DML. This type of transaction is used to execute a single Partitioned DML statement. Partitioned DML partitions the key space and runs the DML statement over each partition in parallel using separate, internal transactions that commit independently. Partitioned DML transactions do not need to be committed. For transactions that only read, snapshot read-only transactions provide simpler semantics and are almost always faster. In particular, read-only transactions do not take locks, so they do not conflict with read-write transactions. As a consequence of not taking locks, they also do not abort, so retry loops are not needed. Transactions may only read/write data in a single database. They may, however, read/write data in different tables within that database. ## Locking Read-Write Transactions Locking transactions may be used to atomically read-modify-write data anywhere in a database. This type of transaction is externally consistent. Clients should attempt to minimize the amount of time a transaction is active. Faster transactions commit with higher probability and cause less contention. Cloud Spanner attempts to keep read locks active as long as the transaction continues to do reads, and the transaction has not been terminated by Commit or Rollback. Long periods of inactivity at the client may cause Cloud Spanner to release a transaction's locks and abort it. Conceptually, a read-write transaction consists of zero or more reads or SQL statements followed by Commit. At any time before Commit, the client can send a Rollback request to abort the transaction. ## Semantics Cloud Spanner can commit the transaction if all read locks it acquired are still valid at commit time, and it is able to acquire write locks for all writes. Cloud Spanner can abort the transaction for any reason. If a commit attempt returns `ABORTED`, Cloud Spanner guarantees that the transaction has not modified any user data in Cloud Spanner. Unless the transaction commits, Cloud Spanner makes no guarantees about how long the transaction's locks were held for. It is an error to use Cloud Spanner locks for any sort of mutual exclusion other than between Cloud Spanner transactions themselves. ## Retrying Aborted Transactions When a transaction aborts, the application can choose to retry the whole transaction again. To maximize the chances of successfully committing the retry, the client should execute the retry in the same session as the original attempt. The original session's lock priority increases with each consecutive abort, meaning that each attempt has a slightly better chance of success than the previous. Under some circumstances (e.g., many transactions attempting to modify the same row(s)), a transaction can abort many times in a short period before successfully committing. Thus, it is not a good idea to cap the number of retries a transaction can attempt; instead, it is better to limit the total amount of wall time spent retrying. ## Idle Transactions A transaction is considered idle if it has no outstanding reads or SQL queries and has not started a read or SQL query within the last 10 seconds. Idle transactions can be aborted by Cloud Spanner so that they don't hold on to locks indefinitely. In that case, the commit will fail with error `ABORTED`. If this behavior is undesirable, periodically executing a simple SQL query in the transaction (e.g., `SELECT 1`) prevents the transaction from becoming idle. ## Snapshot Read-Only Transactions Snapshot read-only transactions provides a simpler method than locking read-write transactions for doing several consistent reads. However, this type of transaction does not support writes. Snapshot transactions do not take locks. Instead, they work by choosing a Cloud Spanner timestamp, then executing all reads at that timestamp. Since they do not acquire locks, they do not block concurrent read-write transactions. Unlike locking read-write transactions, snapshot read-only transactions never abort. They can fail if the chosen read timestamp is garbage collected; however, the default garbage collection policy is generous enough that most applications do not need to worry about this in practice. Snapshot read-only transactions do not need to call Commit or Rollback (and in fact are not permitted to do so). To execute a snapshot transaction, the client specifies a timestamp bound, which tells Cloud Spanner how to choose a read timestamp. The types of timestamp bound are: - Strong (the default). - Bounded staleness. - Exact staleness. If the Cloud Spanner database to be read is geographically distributed, stale read-only transactions can execute more quickly than strong or read-write transaction, because they are able to execute far from the leader replica. Each type of timestamp bound is discussed in detail below. ## Strong Strong reads are guaranteed to see the effects of all transactions that have committed before the start of the read. Furthermore, all rows yielded by a single read are consistent with each other -- if any part of the read observes a transaction, all parts of the read see the transaction. Strong reads are not repeatable: two consecutive strong read-only transactions might return inconsistent results if there are concurrent writes. If consistency across reads is required, the reads should be executed within a transaction or at an exact read timestamp. See TransactionOptions.ReadOnly.strong. ## Exact Staleness These timestamp bounds execute reads at a user-specified timestamp. Reads at a timestamp are guaranteed to see a consistent prefix of the global transaction history: they observe modifications done by all transactions with a commit timestamp <= the read timestamp, and observe none of the modifications done by transactions with a larger commit timestamp. They will block until all conflicting transactions that may be assigned commit timestamps <= the read timestamp have finished. The timestamp can either be expressed as an absolute Cloud Spanner commit timestamp or a staleness relative to the current time. These modes do not require a \"negotiation phase\" to pick a timestamp. As a result, they execute slightly faster than the equivalent boundedly stale concurrency modes. On the other hand, boundedly stale reads usually return fresher results. See TransactionOptions.ReadOnly.read_timestamp and TransactionOptions.ReadOnly.exact_staleness. ## Bounded Staleness Bounded staleness modes allow Cloud Spanner to pick the read timestamp, subject to a user-provided staleness bound. Cloud Spanner chooses the newest timestamp within the staleness bound that allows execution of the reads at the closest available replica without blocking. All rows yielded are consistent with each other -- if any part of the read observes a transaction, all parts of the read see the transaction. Boundedly stale reads are not repeatable: two stale reads, even if they use the same staleness bound, can execute at different timestamps and thus return inconsistent results. Boundedly stale reads execute in two phases: the first phase negotiates a timestamp among all replicas needed to serve the read. In the second phase, reads are executed at the negotiated timestamp. As a result of the two phase execution, bounded staleness reads are usually a little slower than comparable exact staleness reads. However, they are typically able to return fresher results, and are more likely to execute at the closest replica. Because the timestamp negotiation requires up-front knowledge of which rows will be read, it can only be used with single-use read-only transactions. See TransactionOptions.ReadOnly.max_staleness and TransactionOptions.ReadOnly.min_read_timestamp. ## Old Read Timestamps and Garbage Collection Cloud Spanner continuously garbage collects deleted and overwritten data in the background to reclaim storage space. This process is known as \"version GC\". By default, version GC reclaims versions after they are one hour old. Because of this, Cloud Spanner cannot perform reads at read timestamps more than one hour in the past. This restriction also applies to in-progress reads and/or SQL queries whose timestamp become too old while executing. Reads and SQL queries with too-old read timestamps fail with the error `FAILED_PRECONDITION`. ## Partitioned DML Transactions Partitioned DML transactions are used to execute DML statements with a different execution strategy that provides different, and often better, scalability properties for large, table-wide operations than DML in a ReadWrite transaction. Smaller scoped statements, such as an OLTP workload, should prefer using ReadWrite transactions. Partitioned DML partitions the keyspace and runs the DML statement on each partition in separate, internal transactions. These transactions commit automatically when complete, and run independently from one another. To reduce lock contention, this execution strategy only acquires read locks on rows that match the WHERE clause of the statement. Additionally, the smaller per-partition transactions hold locks for less time. That said, Partitioned DML is not a drop-in replacement for standard DML used in ReadWrite transactions. - The DML statement must be fully-partitionable. Specifically, the statement must be expressible as the union of many statements which each access only a single row of the table. - The statement is not applied atomically to all rows of the table. Rather, the statement is applied atomically to partitions of the table, in independent transactions. Secondary index rows are updated atomically with the base table rows. - Partitioned DML does not guarantee exactly-once execution semantics against a partition. The statement will be applied at least once to each partition. It is strongly recommended that the DML statement should be idempotent to avoid unexpected results. For instance, it is potentially dangerous to run a statement such as `UPDATE table SET column = column + 1` as it could be run multiple times against some rows. - The partitions are committed automatically - there is no support for Commit or Rollback. If the call returns an error, or if the client issuing the ExecuteSql call dies, it is possible that some rows had the statement executed on them successfully. It is also possible that statement was never executed against other rows. - Partitioned DML transactions may only contain the execution of a single DML statement via ExecuteSql or ExecuteStreamingSql. - If any error is encountered during the execution of the partitioned DML operation (for instance, a UNIQUE INDEX violation, division by zero, or a value that cannot be stored due to schema constraints), then the operation is stopped at that point and an error is returned. It is possible that at this point, some partitions have been committed (or even committed multiple times), and other partitions have not been run at all. Given the above, Partitioned DML is good fit for large, database-wide, operations that are idempotent, such as deleting old rows from a very large table."]
pub struct TransactionOptions {
    #[serde(rename = "partitionedDml")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Partitioned DML transaction. Authorization to begin a Partitioned DML transaction requires `spanner.databases.beginPartitionedDmlTransaction` permission on the `session` resource."]
    pub partitioned_dml: ::std::option::Option<::std::boxed::Box<PartitionedDml>>,
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Transaction will not write. Authorization to begin a read-only transaction requires `spanner.databases.beginReadOnlyTransaction` permission on the `session` resource."]
    pub read_only: ::std::option::Option<::std::boxed::Box<ReadOnly>>,
    #[serde(rename = "readWrite")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Transaction may write. Authorization to begin a read-write transaction requires `spanner.databases.beginOrRollbackReadWriteTransaction` permission on the `session` resource."]
    pub read_write: ::std::option::Option<::std::boxed::Box<ReadWrite>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This message is used to select the transaction in which a Read or ExecuteSql call runs. See TransactionOptions for more information about transactions."]
pub struct TransactionSelector {
    #[serde(rename = "begin")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Begin a new transaction and execute this read or SQL query in it. The transaction ID of the new transaction is returned in ResultSetMetadata.transaction, which is a Transaction."]
    pub begin: ::std::option::Option<::std::boxed::Box<TransactionOptions>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Execute the read or SQL query in a previously-started transaction."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "singleUse")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Execute the read or SQL query in a temporary transaction. This is the most efficient way to execute a transaction that consists of a single SQL query."]
    pub single_use: ::std::option::Option<::std::boxed::Box<TransactionOptions>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`Type` indicates the type of a Cloud Spanner value, as might be stored in a table cell or returned from an SQL query."]
pub struct Type {
    #[serde(rename = "arrayElementType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If code == ARRAY, then `array_element_type` is the type of the array elements."]
    pub array_element_type: ::std::option::Option<::std::boxed::Box<Type>>,
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The TypeCode for this type."]
    pub code: ::std::option::Option<TypeCodeEnum>,
    #[serde(rename = "structType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If code == STRUCT, then `struct_type` provides type information for the struct's fields."]
    pub struct_type: ::std::option::Option<::std::boxed::Box<StructType>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The TypeCode for this type."]
pub enum TypeCodeEnum {
    #[serde(rename = "TYPE_CODE_UNSPECIFIED")]
    #[doc = "Not specified."]
    TypeCodeUnspecified,
    #[serde(rename = "BOOL")]
    #[doc = "Encoded as JSON `true` or `false`."]
    Bool,
    #[serde(rename = "INT64")]
    #[doc = "Encoded as `string`, in decimal format."]
    Int64,
    #[serde(rename = "FLOAT64")]
    #[doc = "Encoded as `number`, or the strings `\"NaN\"`, `\"Infinity\"`, or `\"-Infinity\"`."]
    Float64,
    #[serde(rename = "TIMESTAMP")]
    #[doc = "Encoded as `string` in RFC 3339 timestamp format. The time zone must be present, and must be `\"Z\"`. If the schema has the column option `allow_commit_timestamp=true`, the placeholder string `\"spanner.commit_timestamp()\"` can be used to instruct the system to insert the commit timestamp associated with the transaction commit."]
    Timestamp,
    #[serde(rename = "DATE")]
    #[doc = "Encoded as `string` in RFC 3339 date format."]
    Date,
    #[serde(rename = "STRING")]
    #[doc = "Encoded as `string`."]
    String,
    #[serde(rename = "BYTES")]
    #[doc = "Encoded as a base64-encoded `string`, as described in RFC 4648, section 4."]
    Bytes,
    #[serde(rename = "ARRAY")]
    #[doc = "Encoded as `list`, where the list elements are represented according to array_element_type."]
    Array,
    #[serde(rename = "STRUCT")]
    #[doc = "Encoded as `list`, where list element `i` is represented according to [struct_type.fields[i]][google.spanner.v1.StructType.fields]."]
    Struct,
    #[serde(rename = "NUMERIC")]
    #[doc = "Encoded as `string`, in decimal format or scientific notation format. Decimal format: `[+-]Digits[.[Digits]]` or `+-.Digits` Scientific notation: `[+-]Digits[.[Digits]][ExponentIndicator[+-]Digits]` or `+-.Digits[ExponentIndicator[+-]Digits]` (ExponentIndicator is `\"e\"` or `\"E\"`)"]
    Numeric,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata type for the operation returned by UpdateDatabaseDdl."]
pub struct UpdateDatabaseDdlMetadata {
    #[serde(rename = "commitTimestamps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reports the commit timestamps of all statements that have succeeded so far, where `commit_timestamps[i]` is the commit timestamp for the statement `statements[i]`."]
    pub commit_timestamps: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "database")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The database being modified."]
    pub database: ::std::option::Option<::std::string::String>,
    #[serde(rename = "statements")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For an update this list contains all the statements. For an individual statement, this list contains only that statement."]
    pub statements: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "throttled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. When true, indicates that the operation is throttled e.g due to resource constraints. When resources become available the operation will resume and this field will be false again."]
    pub throttled: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Enqueues the given DDL statements to be applied, in order but not necessarily all at once, to the database schema at some point (or points) in the future. The server checks that the statements are executable (syntactically valid, name tables that exist, etc.) before enqueueing them, but they may still fail upon later execution (e.g., if a statement from another batch of statements is applied first and it conflicts in some way, or if there is some data-related problem like a `NULL` value in a column to which `NOT NULL` would be added). If a statement fails, all subsequent statements in the batch are automatically cancelled. Each batch of statements is assigned a name which can be used with the Operations API to monitor progress. See the operation_id field for more details."]
pub struct UpdateDatabaseDdlRequest {
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If empty, the new update request is assigned an automatically-generated operation ID. Otherwise, `operation_id` is used to construct the name of the resulting Operation. Specifying an explicit operation ID simplifies determining whether the statements were executed in the event that the UpdateDatabaseDdl call is replayed, or the return value is otherwise lost: the database and `operation_id` fields can be combined to form the name of the resulting longrunning.Operation: `/operations/`. `operation_id` should be unique within the database, and must be a valid identifier: `a-z*`. Note that automatically-generated operation IDs always begin with an underscore. If the named operation already exists, UpdateDatabaseDdl returns `ALREADY_EXISTS`."]
    pub operation_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "statements")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. DDL statements to be applied to the database."]
    pub statements: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata type for the operation returned by UpdateInstance."]
pub struct UpdateInstanceMetadata {
    #[serde(rename = "cancelTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which this operation was cancelled. If set, this operation is in the process of undoing itself (which is guaranteed to succeed) and cannot be cancelled again."]
    pub cancel_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which this operation failed or was completed successfully."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "instance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The desired end state of the update."]
    pub instance: ::std::option::Option<::std::boxed::Box<Instance>>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which UpdateInstance request was received."]
    pub start_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for UpdateInstance."]
pub struct UpdateInstanceRequest {
    #[serde(rename = "fieldMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A mask specifying which fields in Instance should be updated. The field mask must always be specified; this prevents any future fields in Instance from being erased accidentally by clients that do not know about them."]
    pub field_mask: ::std::option::Option<::std::string::String>,
    #[serde(rename = "instance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The instance to update, which must always include the instance name. Otherwise, only fields mentioned in field_mask need be included."]
    pub instance: ::std::option::Option<::std::boxed::Box<Instance>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Arguments to insert, update, insert_or_update, and replace operations."]
pub struct Write {
    #[serde(rename = "columns")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The names of the columns in table to be written. The list of columns must contain enough columns to allow Cloud Spanner to derive values for all primary key columns in the row(s) to be modified."]
    pub columns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "table")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The table whose rows will be written."]
    pub table: ::std::option::Option<::std::string::String>,
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The values to be written. `values` can contain more than one list of values. If it does, then multiple rows are written, one for each entry in `values`. Each list in `values` must have exactly as many entries as there are entries in columns above. Sending multiple lists is equivalent to sending multiple `Mutation`s, each containing one `values` entry and repeating table and columns. Individual values in each list are encoded as described here."]
    pub values: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::serde_json::Value>>>,
}
