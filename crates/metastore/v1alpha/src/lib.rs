#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs.If there are AuditConfigs for both allServices and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted.Example Policy with multiple AuditConfigs: { \"audit_configs\": [ { \"service\": \"allServices\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" }, { \"log_type\": \"ADMIN_READ\" } ] }, { \"service\": \"sampleservice.googleapis.com\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\" }, { \"log_type\": \"DATA_WRITE\", \"exempted_members\": [ \"user:aliya@example.com\" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts jose@example.com from DATA_READ logging, and aliya@example.com from DATA_WRITE logging."]
pub struct AuditConfig {
    #[serde(rename = "auditLogConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration for logging of each type of permission."]
    pub audit_log_configs:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuditLogConfig>>>,
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies a service that will be enabled for audit logging. For example, storage.googleapis.com, cloudsql.googleapis.com. allServices is a special value that covers all services."]
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
#[doc = "A specification of the location of and metadata about a database dump from a relational database management system."]
pub struct DatabaseDump {
    #[serde(rename = "databaseType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the database."]
    pub database_type: ::std::option::Option<DatabaseDumpDatabaseTypeEnum>,
    #[serde(rename = "gcsUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A Cloud Storage object URI that specifies the source from which to import metadata. It must begin with gs://."]
    pub gcs_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sourceDatabase")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the source database."]
    pub source_database: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the database."]
pub enum DatabaseDumpDatabaseTypeEnum {
    #[serde(rename = "DATABASE_TYPE_UNSPECIFIED")]
    #[doc = "The type of the source database is unknown."]
    DatabaseTypeUnspecified,
    #[serde(rename = "MYSQL")]
    #[doc = "The type of the source database is MySQL."]
    Mysql,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for Empty is empty JSON object {}."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for DataprocMetastore.ExportMetadata."]
pub struct ExportMetadataRequest {
    #[serde(rename = "destinationGcsFolder")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A Cloud Storage URI of a folder that metadata are exported to, in the format gs:///. A sub-folder containing exported files will be created below it."]
    pub destination_gcs_folder: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A request ID. Specify a unique request ID to allow the server to ignore the request if it has completed. The server will ignore subsequent requests that provide a duplicate request ID for at least 60 minutes after the first request.For example, if an initial request times out, followed by another request with the same request ID, the server ignores the second request to prevent the creation of duplicate commitments.The request ID must be a valid UUID (https://en.wikipedia.org/wiki/Universally_unique_identifier#Format). A zero UUID (00000000-0000-0000-0000-000000000000) is not supported."]
    pub request_id: ::std::option::Option<::std::string::String>,
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
#[doc = "Represents the metadata of a long-running operation."]
pub struct GoogleCloudMetastoreV1alphaOperationMetadata {
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
    #[doc = "Output only. Identifies whether the caller has requested cancellation of the operation. Operations that have successfully been cancelled have Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED."]
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
#[doc = "Specifies configuration information specific to running Hive metastore software as the metastore service."]
pub struct HiveMetastoreConfig {
    #[serde(rename = "configOverrides")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A mapping of Hive metastore configuration key-value pairs to apply to the Hive metastore (configured in hive-site.xml). The mappings override system defaults (some keys cannot be overridden)."]
    pub config_overrides:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "kerberosConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information used to configure the Hive metastore service as a service principal in a Kerberos realm. To disable Kerberos, use the UpdateService method and specify this field's path (\"hive_metastore_config.kerberos_config\") in the request's update_mask while omitting this field from the request's service."]
    pub kerberos_config: ::std::option::Option<::std::boxed::Box<KerberosConfig>>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The Hive metastore schema version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A specification of a supported version of the Hive Metastore software."]
pub struct HiveMetastoreVersion {
    #[serde(rename = "isDefault")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether version will be chosen by the server if a metastore service is created with a HiveMetastoreConfig that omits the version."]
    pub is_default: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The semantic version of the Hive Metastore software."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration information for a Kerberos principal."]
pub struct KerberosConfig {
    #[serde(rename = "keytab")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A Kerberos keytab file that can be used to authenticate a service principal with a Kerberos Key Distribution Center (KDC)."]
    pub keytab: ::std::option::Option<::std::boxed::Box<Secret>>,
    #[serde(rename = "krb5ConfigGcsUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A Cloud Storage URI that specifies the path to a krb5.conf file. It is of the form gs://{bucket_name}/path/to/krb5.conf, although the file does not need to be named krb5.conf explicitly."]
    pub krb5_config_gcs_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "principal")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A Kerberos principal that exists in the both the keytab the KDC to authenticate as. A typical principal is of the form \"primary/instance@REALM\", but there is no exact format."]
    pub principal: ::std::option::Option<::std::string::String>,
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
#[doc = "Response message for DataprocMetastore.ListMetadataImports."]
pub struct ListMetadataImportsResponse {
    #[serde(rename = "metadataImports")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The imports in the specified service."]
    pub metadata_imports: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetadataImport>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token that can be sent as page_token to retrieve the next page. If this field is omitted, there are no subsequent pages."]
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
#[doc = "Response message for DataprocMetastore.ListServices."]
pub struct ListServicesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token that can be sent as page_token to retrieve the next page. If this field is omitted, there are no subsequent pages."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The services in the specified location."]
    pub services: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Service>>>,
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
    #[doc = "Cross-service attributes for the location. For example {\"cloud.googleapis.com/region\": \"us-east1\"} "]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The canonical id for this location. For example: \"us-east1\"."]
    pub location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Service-specific metadata. For example the available capacity at the given location."]
    pub metadata: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name for the location, which may vary between implementations. For example: \"projects/example-project/locations/us-east1\""]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata about the service in a location."]
pub struct LocationMetadata {
    #[serde(rename = "supportedHiveMetastoreVersions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The versions of Hive Metastore that can be used when creating a new metastore service in this location. The server guarantees that exactly one HiveMetastoreVersion in the list will set is_default."]
    pub supported_hive_metastore_versions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<HiveMetastoreVersion>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Maintenance window. This specifies when Dataproc Metastore may perform system maintenance operation to the service."]
pub struct MaintenanceWindow {
    #[serde(rename = "dayOfWeek")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The day of week, when the window starts."]
    pub day_of_week: ::std::option::Option<MaintenanceWindowDayOfWeekEnum>,
    #[serde(rename = "hourOfDay")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The hour of day (0-23) when the window starts."]
    pub hour_of_day: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The day of week, when the window starts."]
pub enum MaintenanceWindowDayOfWeekEnum {
    #[serde(rename = "DAY_OF_WEEK_UNSPECIFIED")]
    #[doc = "The day of the week is unspecified."]
    DayOfWeekUnspecified,
    #[serde(rename = "MONDAY")]
    #[doc = "Monday"]
    Monday,
    #[serde(rename = "TUESDAY")]
    #[doc = "Tuesday"]
    Tuesday,
    #[serde(rename = "WEDNESDAY")]
    #[doc = "Wednesday"]
    Wednesday,
    #[serde(rename = "THURSDAY")]
    #[doc = "Thursday"]
    Thursday,
    #[serde(rename = "FRIDAY")]
    #[doc = "Friday"]
    Friday,
    #[serde(rename = "SATURDAY")]
    #[doc = "Saturday"]
    Saturday,
    #[serde(rename = "SUNDAY")]
    #[doc = "Sunday"]
    Sunday,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The details of a metadata export operation."]
pub struct MetadataExport {
    #[serde(rename = "destinationGcsUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A Cloud Storage URI of a folder that metadata are exported to, in the form of gs:////, where ` is automatically generated."]
    pub destination_gcs_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time when the export ended."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time when the export started."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The current state of the export."]
    pub state: ::std::option::Option<MetadataExportStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The current state of the export."]
pub enum MetadataExportStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "The state of the metadata export is unknown."]
    StateUnspecified,
    #[serde(rename = "RUNNING")]
    #[doc = "The metadata export is running."]
    Running,
    #[serde(rename = "SUCCEEDED")]
    #[doc = "The metadata export completed successfully."]
    Succeeded,
    #[serde(rename = "FAILED")]
    #[doc = "The metadata export failed."]
    Failed,
    #[serde(rename = "CANCELLED")]
    #[doc = "The metadata export is cancelled."]
    Cancelled,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A metastore resource that imports metadata."]
pub struct MetadataImport {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time when the metadata import was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "databaseDump")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. A database dump from a pre-existing metastore's database."]
    pub database_dump: ::std::option::Option<::std::boxed::Box<DatabaseDump>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The description of the metadata import."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The relative resource name of the metadata import, of the form:\"projects/{project_number}/locations/{location_id}/services/{service_id}/metadataImports/{metadata_import_id}\"."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The current state of the metadata import."]
    pub state: ::std::option::Option<MetadataImportStateEnum>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time when the metadata import was last updated."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The current state of the metadata import."]
pub enum MetadataImportStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "The state of the metadata import is unknown."]
    StateUnspecified,
    #[serde(rename = "RUNNING")]
    #[doc = "The metadata import is running."]
    Running,
    #[serde(rename = "SUCCEEDED")]
    #[doc = "The metadata import completed successfully."]
    Succeeded,
    #[serde(rename = "UPDATING")]
    #[doc = "The metadata import is being updated."]
    Updating,
    #[serde(rename = "FAILED")]
    #[doc = "The metadata import failed, and attempted metadata changes were rolled back."]
    Failed,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies how metastore metadata should be integrated with external services."]
pub struct MetadataIntegration {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The metadata management activities of the metastore service."]
pub struct MetadataManagementActivity {
    #[serde(rename = "metadataExports")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The latest metadata exports of the metastore service."]
    pub metadata_exports: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MetadataExport>>>,
    #[serde(rename = "restores")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The latest restores of the metastore service."]
    pub restores: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Restore>>>,
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
#[doc = "An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources.A Policy is a collection of bindings. A binding binds one or more members to a single role. Members can be user accounts, service accounts, Google groups, and domains (such as G Suite). A role is a named list of permissions; each role can be an IAM predefined role or a user-created custom role.For some types of Google Cloud resources, a binding can also specify a condition, which is a logical expression that allows access to a resource only if the expression evaluates to true. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the IAM documentation (https://cloud.google.com/iam/help/conditions/resource-policies).JSON example: { \"bindings\": [ { \"role\": \"roles/resourcemanager.organizationAdmin\", \"members\": [ \"user:mike@example.com\", \"group:admins@example.com\", \"domain:google.com\", \"serviceAccount:my-project-id@appspot.gserviceaccount.com\" ] }, { \"role\": \"roles/resourcemanager.organizationViewer\", \"members\": [ \"user:eve@example.com\" ], \"condition\": { \"title\": \"expirable access\", \"description\": \"Does not grant access after Sep 2020\", \"expression\": \"request.time < timestamp('2020-10-01T00:00:00.000Z')\", } } ], \"etag\": \"BwWWja0YfJA=\", \"version\": 3 } YAML example: bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') - etag: BwWWja0YfJA= - version: 3 For a description of IAM and its features, see the IAM documentation (https://cloud.google.com/iam/docs/)."]
pub struct Policy {
    #[serde(rename = "auditConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies cloud audit logging configuration for this policy."]
    pub audit_configs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuditConfig>>>,
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
#[doc = "The details of a metadata restore operation."]
pub struct Restore {
    #[serde(rename = "backup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The relative resource name of the metastore service backup to restore from, in the following form:projects/{project_id}/locations/{location_id}/services/{service_id}/backups/{backup_id}"]
    pub backup: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time when the restore ended."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time when the restore started."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The current state of the restore."]
    pub state: ::std::option::Option<RestoreStateEnum>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of restore."]
    pub _type: ::std::option::Option<RestoreTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The current state of the restore."]
pub enum RestoreStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "The state of the metadata restore is unknown."]
    StateUnspecified,
    #[serde(rename = "RUNNING")]
    #[doc = "The metadata restore is running."]
    Running,
    #[serde(rename = "SUCCEEDED")]
    #[doc = "The metadata restore completed successfully."]
    Succeeded,
    #[serde(rename = "FAILED")]
    #[doc = "The metadata restore failed."]
    Failed,
    #[serde(rename = "CANCELLED")]
    #[doc = "The metadata restore is cancelled."]
    Cancelled,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The type of restore."]
pub enum RestoreTypeEnum {
    #[serde(rename = "RESTORE_TYPE_UNSPECIFIED")]
    #[doc = "The restore type is unknown."]
    RestoreTypeUnspecified,
    #[serde(rename = "FULL")]
    #[doc = "The service's metadata and configuration are restored."]
    Full,
    #[serde(rename = "METADATA_ONLY")]
    #[doc = "Only the service's metadata is restored."]
    MetadataOnly,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A securely stored value."]
pub struct Secret {
    #[serde(rename = "cloudSecret")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The relative resource name of a Secret Manager secret version, in the following form:\"projects/{project_number}/secrets/{secret_id}/versions/{version_id}\"."]
    pub cloud_secret: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A managed metastore service that serves metadata queries."]
pub struct Service {
    #[serde(rename = "artifactGcsUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A Cloud Storage URI (starting with gs://) that specifies where artifacts related to the metastore service are stored."]
    pub artifact_gcs_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time when the metastore service was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endpointUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The URI of the endpoint used to access the metastore service."]
    pub endpoint_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hiveMetastoreConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration information specific to running Hive metastore software as the metastore service."]
    pub hive_metastore_config: ::std::option::Option<::std::boxed::Box<HiveMetastoreConfig>>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User-defined labels for the metastore service."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "maintenanceWindow")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The one hour maintenance window of the metastore service. This specifies when the service can be restarted for maintenance purposes in UTC time."]
    pub maintenance_window: ::std::option::Option<::std::boxed::Box<MaintenanceWindow>>,
    #[serde(rename = "metadataIntegration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The setting that defines how metastore metadata should be integrated with external services and systems."]
    pub metadata_integration: ::std::option::Option<::std::boxed::Box<MetadataIntegration>>,
    #[serde(rename = "metadataManagementActivity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The metadata management activities of the metastore service."]
    pub metadata_management_activity:
        ::std::option::Option<::std::boxed::Box<MetadataManagementActivity>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The relative resource name of the metastore service, of the form:\"projects/{project_number}/locations/{location_id}/services/{service_id}\"."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "network")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The relative resource name of the VPC network on which the instance can be accessed. It is specified in the following form:\"projects/{project_number}/global/networks/{network_id}\"."]
    pub network: ::std::option::Option<::std::string::String>,
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The TCP port at which the metastore service is reached. Default: 9083."]
    pub port: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The current state of the metastore service."]
    pub state: ::std::option::Option<ServiceStateEnum>,
    #[serde(rename = "stateMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Additional information about the current state of the metastore service, if available."]
    pub state_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The tier of the service."]
    pub tier: ::std::option::Option<ServiceTierEnum>,
    #[serde(rename = "uid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The globally unique resource identifier of the metastore service."]
    pub uid: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time when the metastore service was last updated."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The current state of the metastore service."]
pub enum ServiceStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "The state of the metastore service is unknown."]
    StateUnspecified,
    #[serde(rename = "CREATING")]
    #[doc = "The metastore service is in the process of being created."]
    Creating,
    #[serde(rename = "ACTIVE")]
    #[doc = "The metastore service is running and ready to serve queries."]
    Active,
    #[serde(rename = "SUSPENDING")]
    #[doc = "The metastore service is entering suspension. Its query-serving availability may cease unexpectedly."]
    Suspending,
    #[serde(rename = "SUSPENDED")]
    #[doc = "The metastore service is suspended and unable to serve queries."]
    Suspended,
    #[serde(rename = "UPDATING")]
    #[doc = "The metastore service is being updated. It remains usable but cannot accept additional update requests or be deleted at this time."]
    Updating,
    #[serde(rename = "DELETING")]
    #[doc = "The metastore service is undergoing deletion. It cannot be used."]
    Deleting,
    #[serde(rename = "ERROR")]
    #[doc = "The metastore service has encountered an error and cannot be used. The metastore service should be deleted."]
    Error,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The tier of the service."]
pub enum ServiceTierEnum {
    #[serde(rename = "TIER_UNSPECIFIED")]
    #[doc = "The tier is not set."]
    TierUnspecified,
    #[serde(rename = "DEVELOPER")]
    #[doc = "The developer tier provides limited scalability and no fault tolerance. Good for low-cost proof-of-concept."]
    Developer,
    #[serde(rename = "ENTERPRISE")]
    #[doc = "The enterprise tier provides multi-zone high availability, and sufficient scalability for enterprise-level Dataproc Metastore workloads."]
    Enterprise,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for SetIamPolicy method."]
pub struct SetIamPolicyRequest {
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "REQUIRED: The complete policy to be applied to the resource. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Cloud Platform services (such as Projects) might reject them."]
    pub policy: ::std::option::Option<::std::boxed::Box<Policy>>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used:paths: \"bindings, etag\""]
    pub update_mask: ::std::option::Option<::std::string::String>,
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
