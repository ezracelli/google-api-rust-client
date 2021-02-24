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
#[doc = "The request message for Operations.CancelOperation."]
pub struct CancelOperationRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies required connection parameters, and, optionally, the parameters required to create a Cloud SQL destination database instance."]
pub struct CloudSqlConnectionProfile {
    #[serde(rename = "cloudSqlId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The Cloud SQL instance ID that this connection profile is associated with."]
    pub cloud_sql_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "privateIp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The Cloud SQL database instance's private IP."]
    pub private_ip: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publicIp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The Cloud SQL database instance's public IP."]
    pub public_ip: ::std::option::Option<::std::string::String>,
    #[serde(rename = "settings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. Metadata used to create the destination Cloud SQL database."]
    pub settings: ::std::option::Option<::std::boxed::Box<CloudSqlSettings>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings for creating a Cloud SQL database instance."]
pub struct CloudSqlSettings {
    #[serde(rename = "activationPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The activation policy specifies when the instance is activated; it is applicable only when the instance state is 'RUNNABLE'. Valid values: 'ALWAYS': The instance is on, and remains so even in the absence of connection requests. `NEVER`: The instance is off; it is not activated, even if a connection request arrives."]
    pub activation_policy: ::std::option::Option<CloudSqlSettingsActivationPolicyEnum>,
    #[serde(rename = "autoStorageIncrease")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[default: ON] If you enable this setting, Cloud SQL checks your available storage every 30 seconds. If the available storage falls below a threshold size, Cloud SQL automatically adds additional storage capacity. If the available storage repeatedly falls below the threshold size, Cloud SQL continues to add storage until it reaches the maximum of 30 TB."]
    pub auto_storage_increase: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "dataDiskSizeGb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The storage capacity available to the database, in GB. The minimum (and default) size is 10GB."]
    pub data_disk_size_gb: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dataDiskType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of storage: `PD_SSD` (default) or `PD_HDD`."]
    pub data_disk_type: ::std::option::Option<CloudSqlSettingsDataDiskTypeEnum>,
    #[serde(rename = "databaseFlags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The database flags passed to the Cloud SQL instance at startup. An object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub database_flags:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "databaseVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The database engine type and version."]
    pub database_version: ::std::option::Option<CloudSqlSettingsDatabaseVersionEnum>,
    #[serde(rename = "ipConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The settings for IP Management. This allows to enable or disable the instance IP and manage which external networks can connect to the instance. The IPv4 address cannot be disabled."]
    pub ip_config: ::std::option::Option<::std::boxed::Box<SqlIpConfig>>,
    #[serde(rename = "rootPassword")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Input only. Initial root password."]
    pub root_password: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rootPasswordSet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Indicates If this connection profile root password is stored."]
    pub root_password_set: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "sourceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Database Migration Service source connection profile ID, in the format: `projects/my_project_name/locations/us-central1/connectionProfiles/connection_profile_ID`"]
    pub source_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "storageAutoResizeLimit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum size to which storage capacity can be automatically increased. The default value is 0, which specifies that there is no limit."]
    pub storage_auto_resize_limit: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The tier (or machine type) for this instance, for example: `db-n1-standard-1` (MySQL instances). For more information, see [Cloud SQL Instance Settings](https://cloud.google.com/sql/docs/mysql/instance-settings)."]
    pub tier: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userLabels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource labels for a Cloud SQL instance to use to annotate any related underlying resources such as Compute Engine VMs. An object containing a list of \"key\": \"value\" pairs. Example: `{ \"name\": \"wrench\", \"mass\": \"18kg\", \"count\": \"3\" }`."]
    pub user_labels:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Google Cloud Platform zone where your Cloud SQL datdabse instance is located."]
    pub zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The activation policy specifies when the instance is activated; it is applicable only when the instance state is 'RUNNABLE'. Valid values: 'ALWAYS': The instance is on, and remains so even in the absence of connection requests. `NEVER`: The instance is off; it is not activated, even if a connection request arrives."]
pub enum CloudSqlSettingsActivationPolicyEnum {
    #[serde(rename = "SQL_ACTIVATION_POLICY_UNSPECIFIED")]
    #[doc = "unspecified policy."]
    SqlActivationPolicyUnspecified,
    #[serde(rename = "ALWAYS")]
    #[doc = "The instance is always up and running."]
    Always,
    #[serde(rename = "NEVER")]
    #[doc = "The instance should never spin up."]
    Never,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of storage: `PD_SSD` (default) or `PD_HDD`."]
pub enum CloudSqlSettingsDataDiskTypeEnum {
    #[serde(rename = "SQL_DATA_DISK_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified."]
    SqlDataDiskTypeUnspecified,
    #[serde(rename = "PD_SSD")]
    #[doc = "SSD disk."]
    PdSsd,
    #[serde(rename = "PD_HDD")]
    #[doc = "HDD disk."]
    PdHdd,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The database engine type and version."]
pub enum CloudSqlSettingsDatabaseVersionEnum {
    #[serde(rename = "SQL_DATABASE_VERSION_UNSPECIFIED")]
    #[doc = "Unspecified version."]
    SqlDatabaseVersionUnspecified,
    #[serde(rename = "MYSQL_5_6")]
    #[doc = "MySQL 5.6."]
    Mysql56,
    #[serde(rename = "MYSQL_5_7")]
    #[doc = "MySQL 5.7."]
    Mysql57,
    #[serde(rename = "MYSQL_8_0")]
    #[doc = "MySQL 8.0."]
    Mysql80,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A connection profile definition."]
pub struct ConnectionProfile {
    #[serde(rename = "cloudsql")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A CloudSQL database connection profile."]
    pub cloudsql: ::std::option::Option<::std::boxed::Box<CloudSqlConnectionProfile>>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The timestamp when the resource was created. A timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The connection profile display name."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The error details in case of state FAILED."]
    pub error: ::std::option::Option<::std::boxed::Box<Status>>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource labels for connection profile to use to annotate any related underlying resources such as Compute Engine VMs. An object containing a list of \"key\": \"value\" pairs. Example: `{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }`."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "mysql")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A MySQL database connection profile."]
    pub mysql: ::std::option::Option<::std::boxed::Box<MySqlConnectionProfile>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of this connection profile resource in the form of projects/{project}/locations/{location}/instances/{instance}."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "provider")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The database provider."]
    pub provider: ::std::option::Option<ConnectionProfileProviderEnum>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current connection profile state (e.g. DRAFT, READY, or FAILED)."]
    pub state: ::std::option::Option<ConnectionProfileStateEnum>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The timestamp when the resource was last updated. A timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The database provider."]
pub enum ConnectionProfileProviderEnum {
    #[serde(rename = "DATABASE_PROVIDER_UNSPECIFIED")]
    #[doc = "The database provider is unknown."]
    DatabaseProviderUnspecified,
    #[serde(rename = "CLOUDSQL")]
    #[doc = "CloudSQL runs the database."]
    Cloudsql,
    #[serde(rename = "RDS")]
    #[doc = "RDS runs the database."]
    Rds,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The current connection profile state (e.g. DRAFT, READY, or FAILED)."]
pub enum ConnectionProfileStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "The state of the connection profile is unknown."]
    StateUnspecified,
    #[serde(rename = "DRAFT")]
    #[doc = "The connection profile is in draft mode and fully editable."]
    Draft,
    #[serde(rename = "CREATING")]
    #[doc = "The connection profile is being created."]
    Creating,
    #[serde(rename = "READY")]
    #[doc = "The connection profile is ready."]
    Ready,
    #[serde(rename = "UPDATING")]
    #[doc = "The connection profile is being updated."]
    Updating,
    #[serde(rename = "DELETING")]
    #[doc = "The connection profile is being deleted."]
    Deleting,
    #[serde(rename = "DELETED")]
    #[doc = "The connection profile has been deleted."]
    Deleted,
    #[serde(rename = "FAILED")]
    #[doc = "The last action on the connection profile failed."]
    Failed,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A message defining the database engine and provider."]
pub struct DatabaseType {
    #[serde(rename = "engine")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The database engine."]
    pub engine: ::std::option::Option<DatabaseTypeEngineEnum>,
    #[serde(rename = "provider")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The database provider."]
    pub provider: ::std::option::Option<DatabaseTypeProviderEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The database engine."]
pub enum DatabaseTypeEngineEnum {
    #[serde(rename = "DATABASE_ENGINE_UNSPECIFIED")]
    #[doc = "The source database engine of the migration job is unknown."]
    DatabaseEngineUnspecified,
    #[serde(rename = "MYSQL")]
    #[doc = "The source engine is MySQL."]
    Mysql,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The database provider."]
pub enum DatabaseTypeProviderEnum {
    #[serde(rename = "DATABASE_PROVIDER_UNSPECIFIED")]
    #[doc = "The database provider is unknown."]
    DatabaseProviderUnspecified,
    #[serde(rename = "CLOUDSQL")]
    #[doc = "CloudSQL runs the database."]
    Cloudsql,
    #[serde(rename = "RDS")]
    #[doc = "RDS runs the database."]
    Rds,
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
#[doc = "Request message for 'GenerateSshScript' request."]
pub struct GenerateSshScriptRequest {
    #[serde(rename = "vm")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Bastion VM Instance name to use or to create."]
    pub vm: ::std::option::Option<::std::string::String>,
    #[serde(rename = "vmCreationConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The VM creation configuration"]
    pub vm_creation_config: ::std::option::Option<::std::boxed::Box<VmCreationConfig>>,
    #[serde(rename = "vmPort")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The port that will be open on the bastion host"]
    pub vm_port: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "vmSelectionConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The VM selection configuration"]
    pub vm_selection_config: ::std::option::Option<::std::boxed::Box<VmSelectionConfig>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the metadata of the long-running operation."]
pub struct GoogleCloudClouddmsV1beta1OperationMetadata {
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. API version used to start the operation."]
    pub api_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time the operation was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time the operation finished running."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestedCancellation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Identifies whether the user has requested cancellation of the operation. Operations that have successfully been cancelled have Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`."]
    pub requested_cancellation: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Human-readable status of the operation, if any."]
    pub status_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Server-defined resource path for the target of the operation."]
    pub target: ::std::option::Option<::std::string::String>,
    #[serde(rename = "verb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Name of the verb executed by the operation."]
    pub verb: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for 'ListConnectionProfiles' request."]
pub struct ListConnectionProfilesResponse {
    #[serde(rename = "connectionProfiles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The response list of connection profiles."]
    pub connection_profiles:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ConnectionProfile>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
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
#[doc = "Response message for 'ListMigrationJobs' request."]
pub struct ListMigrationJobsResponse {
    #[serde(rename = "migrationJobs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of migration jobs objects."]
    pub migration_jobs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MigrationJob>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
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
#[doc = "Represents a Database Migration Service migration job object."]
pub struct MigrationJob {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The timestamp when the migration job resource was created. A timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The resource name (URI) of the destination connection profile."]
    pub destination: ::std::option::Option<::std::string::String>,
    #[serde(rename = "destinationDatabase")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The database engine type and provider of the destination."]
    pub destination_database: ::std::option::Option<::std::boxed::Box<DatabaseType>>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The migration job display name."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dumpPath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The path to the dump file in Google Cloud Storage, in the format: (gs://[BUCKET_NAME]/[OBJECT_NAME])."]
    pub dump_path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The duration of the migration job (in seconds). A duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub duration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. If the migration job is completed, the time when it was completed."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The error details in case of state FAILED."]
    pub error: ::std::option::Option<::std::boxed::Box<Status>>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource labels for migration job to use to annotate any related underlying resources such as Compute Engine VMs. An object containing a list of \"key\": \"value\" pairs. Example: `{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }`."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name (URI) of this migration job resource, in the form of: projects/{project}/locations/{location}/instances/{instance}."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "phase")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The current migration job phase."]
    pub phase: ::std::option::Option<MigrationJobPhaseEnum>,
    #[serde(rename = "reverseSshConnectivity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The details needed to communicate to the source over Reverse SSH tunnel connectivity."]
    pub reverse_ssh_connectivity: ::std::option::Option<::std::boxed::Box<ReverseSshConnectivity>>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The resource name (URI) of the source connection profile."]
    pub source: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sourceDatabase")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The database engine type and provider of the source."]
    pub source_database: ::std::option::Option<::std::boxed::Box<DatabaseType>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current migration job state."]
    pub state: ::std::option::Option<MigrationJobStateEnum>,
    #[serde(rename = "staticIpConnectivity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "static ip connectivity data (default, no additional details needed)."]
    pub static_ip_connectivity: ::std::option::Option<::std::boxed::Box<StaticIpConnectivity>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The migration job type."]
    pub _type: ::std::option::Option<MigrationJobTypeEnum>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The timestamp when the migration job resource was last updated. A timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds. Example: \"2014-10-02T15:01:23.045123456Z\"."]
    pub update_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "vpcPeeringConnectivity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The details of the VPC network that the source database is located in."]
    pub vpc_peering_connectivity: ::std::option::Option<::std::boxed::Box<VpcPeeringConnectivity>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The current migration job phase."]
pub enum MigrationJobPhaseEnum {
    #[serde(rename = "PHASE_UNSPECIFIED")]
    #[doc = "The phase of the migration job is unknown."]
    PhaseUnspecified,
    #[serde(rename = "FULL_DUMP")]
    #[doc = "The migration job is in the full dump phase."]
    FullDump,
    #[serde(rename = "CDC")]
    #[doc = "The migration job is CDC phase."]
    Cdc,
    #[serde(rename = "PROMOTE_IN_PROGRESS")]
    #[doc = "The migration job is running the promote phase."]
    PromoteInProgress,
    #[serde(rename = "WAITING_FOR_SOURCE_WRITES_TO_STOP")]
    #[doc = "Only RDS flow - waiting for source writes to stop"]
    WaitingForSourceWritesToStop,
    #[serde(rename = "PREPARING_THE_DUMP")]
    #[doc = "Only RDS flow - the sources writes stopped, waiting for dump to begin"]
    PreparingTheDump,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The current migration job state."]
pub enum MigrationJobStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "The state of the migration job is unknown."]
    StateUnspecified,
    #[serde(rename = "MAINTENANCE")]
    #[doc = "The migration job is down for maintenance."]
    Maintenance,
    #[serde(rename = "DRAFT")]
    #[doc = "The migration job is in draft mode and no resources are created."]
    Draft,
    #[serde(rename = "CREATING")]
    #[doc = "The migration job is being created."]
    Creating,
    #[serde(rename = "NOT_STARTED")]
    #[doc = "The migration job is created, not started and is fully editable."]
    NotStarted,
    #[serde(rename = "RUNNING")]
    #[doc = "The migration job is running."]
    Running,
    #[serde(rename = "FAILED")]
    #[doc = "The migration job failed."]
    Failed,
    #[serde(rename = "COMPLETED")]
    #[doc = "The migration job has been completed."]
    Completed,
    #[serde(rename = "DELETING")]
    #[doc = "The migration job is being deleted."]
    Deleting,
    #[serde(rename = "STOPPING")]
    #[doc = "The migration job is being stopped."]
    Stopping,
    #[serde(rename = "STOPPED")]
    #[doc = "The migration job is currently stopped."]
    Stopped,
    #[serde(rename = "DELETED")]
    #[doc = "The migration job has been deleted."]
    Deleted,
    #[serde(rename = "UPDATING")]
    #[doc = "The migration job is being updated."]
    Updating,
    #[serde(rename = "STARTING")]
    #[doc = "The migration job is starting."]
    Starting,
    #[serde(rename = "RESTARTING")]
    #[doc = "The migration job is restarting."]
    Restarting,
    #[serde(rename = "RESUMING")]
    #[doc = "The migration job is resuming."]
    Resuming,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The migration job type."]
pub enum MigrationJobTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "The type of the migration job is unknown."]
    TypeUnspecified,
    #[serde(rename = "ONE_TIME")]
    #[doc = "The migration job is a one time migration."]
    OneTime,
    #[serde(rename = "CONTINUOUS")]
    #[doc = "The migration job is a continuous migration."]
    Continuous,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Error message of a verification Migration job."]
pub struct MigrationJobVerificationError {
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. An instance of ErrorCode specifying the error that occurred."]
    pub error_code: ::std::option::Option<MigrationJobVerificationErrorErrorCodeEnum>,
    #[serde(rename = "errorDetailMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A specific detailed error message, if supplied by the engine."]
    pub error_detail_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A formatted message with further details about the error and a CTA."]
    pub error_message: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. An instance of ErrorCode specifying the error that occurred."]
pub enum MigrationJobVerificationErrorErrorCodeEnum {
    #[serde(rename = "ERROR_CODE_UNSPECIFIED")]
    #[doc = "An unknown error occurred"]
    ErrorCodeUnspecified,
    #[serde(rename = "CONNECTION_FAILURE")]
    #[doc = "We failed to connect to one of the connection profile."]
    ConnectionFailure,
    #[serde(rename = "AUTHENTICATION_FAILURE")]
    #[doc = "We failed to authenticate to one of the connection profile."]
    AuthenticationFailure,
    #[serde(rename = "INVALID_CONNECTION_PROFILE_CONFIG")]
    #[doc = "One of the involved connection profiles has an invalid configuration."]
    InvalidConnectionProfileConfig,
    #[serde(rename = "VERSION_INCOMPATIBILITY")]
    #[doc = "The versions of the source and the destination are incompatible."]
    VersionIncompatibility,
    #[serde(rename = "CONNECTION_PROFILE_TYPES_INCOMPATIBILITY")]
    #[doc = "The types of the source and the destination are incompatible."]
    ConnectionProfileTypesIncompatibility,
    #[serde(rename = "UNSUPPORTED_GTID_MODE")]
    #[doc = "The gtid_mode is not supported, applicable for MySQL."]
    UnsupportedGtidMode,
    #[serde(rename = "UNSUPPORTED_DEFINER")]
    #[doc = "The definer is not supported."]
    UnsupportedDefiner,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies connection parameters required specifically for MySQL databases."]
pub struct MySqlConnectionProfile {
    #[serde(rename = "cloudSqlId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the source is a Cloud SQL database, use this field to provide the Cloud SQL instance ID of the source."]
    pub cloud_sql_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "host")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The IP or hostname of the source MySQL database."]
    pub host: ::std::option::Option<::std::string::String>,
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Input only. The password for the user that Database Migration Service will be using to connect to the database. This field is not returned on request, and the value is encrypted when stored in Database Migration Service."]
    pub password: ::std::option::Option<::std::string::String>,
    #[serde(rename = "passwordSet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Indicates If this connection profile password is stored."]
    pub password_set: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The network port of the source MySQL database."]
    pub port: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "ssl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "SSL configuration for the destination to connect to the source database."]
    pub ssl: ::std::option::Option<::std::boxed::Box<SslConfig>>,
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The username that Database Migration Service will use to connect to the database. The value is encrypted when stored in Database Migration Service."]
    pub username: ::std::option::Option<::std::string::String>,
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
#[doc = "Request message for 'PromoteMigrationJob' request."]
pub struct PromoteMigrationJobRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for 'RestartMigrationJob' request."]
pub struct RestartMigrationJobRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for 'ResumeMigrationJob' request."]
pub struct ResumeMigrationJobRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The details needed to configure a reverse SSH tunnel between the source and destination databases. These details will be used when calling the generateSshScript method (see https://cloud.google.com/database-migration/docs/reference/rest/v1beta1/projects.locations.migrationJobs/generateSshScript) to produce the script that will help set up the reverse SSH tunnel, and to set up the VPC peering between the Cloud SQL private network and the VPC."]
pub struct ReverseSshConnectivity {
    #[serde(rename = "vm")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the virtual machine (Compute Engine) used as the bastion server for the SSH tunnel."]
    pub vm: ::std::option::Option<::std::string::String>,
    #[serde(rename = "vmIp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The IP of the virtual machine (Compute Engine) used as the bastion server for the SSH tunnel."]
    pub vm_ip: ::std::option::Option<::std::string::String>,
    #[serde(rename = "vmPort")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The forwarding port of the virtual machine (Compute Engine) used as the bastion server for the SSH tunnel."]
    pub vm_port: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "vpc")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the VPC to peer with the Cloud SQL private network."]
    pub vpc: ::std::option::Option<::std::string::String>,
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
#[doc = "An entry for an Access Control list."]
pub struct SqlAclEntry {
    #[serde(rename = "expireTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time when this access control entry expires in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example: `2012-11-15T16:19:00.094Z`."]
    pub expire_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A label to identify this entry."]
    pub label: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ttl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Input only. The time-to-leave of this access control entry."]
    pub ttl: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The allowlisted value for the access control list."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "IP Management configuration."]
pub struct SqlIpConfig {
    #[serde(rename = "authorizedNetworks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of external networks that are allowed to connect to the instance using the IP. See https://en.wikipedia.org/wiki/CIDR_notation#CIDR_notation, also known as 'slash' notation (e.g. `192.168.100.0/24`)."]
    pub authorized_networks: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SqlAclEntry>>>,
    #[serde(rename = "enableIpv4")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the instance should be assigned an IPv4 address or not."]
    pub enable_ipv4: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "privateNetwork")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource link for the VPC network from which the Cloud SQL instance is accessible for private IP. For example, `/projects/myProject/global/networks/default`. This setting can be updated, but it cannot be removed after it is set."]
    pub private_network: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requireSsl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether SSL connections over IP should be enforced or not."]
    pub require_ssl: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for 'GenerateSshScript' request."]
pub struct SshScript {
    #[serde(rename = "script")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ssh configuration script."]
    pub script: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "SSL configuration information."]
pub struct SslConfig {
    #[serde(rename = "caCertificate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Input only. The x509 PEM-encoded certificate of the CA that signed the source database server's certificate. The replica will use this certificate to verify it's connecting to the right host."]
    pub ca_certificate: ::std::option::Option<::std::string::String>,
    #[serde(rename = "clientCertificate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Input only. The x509 PEM-encoded certificate that will be used by the replica to authenticate against the source database server.If this field is used then the 'client_key' field is mandatory."]
    pub client_certificate: ::std::option::Option<::std::string::String>,
    #[serde(rename = "clientKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Input only. The unencrypted PKCS#1 or PKCS#8 PEM-encoded private key associated with the Client Certificate. If this field is used then the 'client_certificate' field is mandatory."]
    pub client_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The ssl config type according to 'client_key', 'client_certificate' and 'ca_certificate'."]
    pub _type: ::std::option::Option<SslConfigTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The ssl config type according to 'client_key', 'client_certificate' and 'ca_certificate'."]
pub enum SslConfigTypeEnum {
    #[serde(rename = "SSL_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified."]
    SslTypeUnspecified,
    #[serde(rename = "SERVER_ONLY")]
    #[doc = "Only 'ca_certificate' specified."]
    ServerOnly,
    #[serde(rename = "SERVER_CLIENT")]
    #[doc = "Both server ('ca_certificate'), and client ('client_key', 'client_certificate') specified."]
    ServerClient,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for 'StartMigrationJob' request."]
pub struct StartMigrationJobRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The source database will allow incoming connections from the destination database's public IP. You can retrieve the Cloud SQL instance's public IP from the Cloud SQL console or using Cloud SQL APIs. No additional configuration is required."]
pub struct StaticIpConnectivity {}
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
#[doc = "Request message for 'StopMigrationJob' request."]
pub struct StopMigrationJobRequest {}
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
#[doc = "Request message for 'VerifyMigrationJob' request."]
pub struct VerifyMigrationJobRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "VM creation configuration message"]
pub struct VmCreationConfig {
    #[serde(rename = "subnet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The subnet name the vm needs to be created in."]
    pub subnet: ::std::option::Option<::std::string::String>,
    #[serde(rename = "vmMachineType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. VM instance machine type to create."]
    pub vm_machine_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "vmZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Google Cloud Platform zone to create the VM in."]
    pub vm_zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "VM selection configuration message"]
pub struct VmSelectionConfig {
    #[serde(rename = "vmZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The Google Cloud Platform zone the VM is located."]
    pub vm_zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The details of the VPC where the source database is located in Google Cloud. We will use this information to set up the VPC peering connection between Cloud SQL and this VPC."]
pub struct VpcPeeringConnectivity {
    #[serde(rename = "vpc")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the VPC network to peer with the Cloud SQL private network."]
    pub vpc: ::std::option::Option<::std::string::String>,
}
