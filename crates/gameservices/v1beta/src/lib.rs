#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { \"audit_configs\": [ { \"service\": \"allServices\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" }, { \"log_type\": \"ADMIN_READ\" } ] }, { \"service\": \"sampleservice.googleapis.com\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\" }, { \"log_type\": \"DATA_WRITE\", \"exempted_members\": [ \"user:aliya@example.com\" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts jose@example.com from DATA_READ logging, and aliya@example.com from DATA_WRITE logging."]
pub struct AuditConfig {
    #[serde(rename = "auditLogConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration for logging of each type of permission."]
    pub audit_log_configs:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuditLogConfig>>>,
    #[serde(rename = "exemptedMembers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub exempted_members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
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
    #[serde(rename = "ignoreChildExemptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ignore_child_exemptions: ::std::option::Option<::std::primitive::bool>,
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
#[doc = "Authorization-related information used by Cloud Audit Logging."]
pub struct AuthorizationLoggingOptions {
    #[serde(rename = "permissionType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the permission that was checked."]
    pub permission_type: ::std::option::Option<AuthorizationLoggingOptionsPermissionTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the permission that was checked."]
pub enum AuthorizationLoggingOptionsPermissionTypeEnum {
    #[serde(rename = "PERMISSION_TYPE_UNSPECIFIED")]
    #[doc = "Default. Should not be used."]
    PermissionTypeUnspecified,
    #[serde(rename = "ADMIN_READ")]
    #[doc = "A read of admin (meta) data."]
    AdminRead,
    #[serde(rename = "ADMIN_WRITE")]
    #[doc = "A write of admin (meta) data."]
    AdminWrite,
    #[serde(rename = "DATA_READ")]
    #[doc = "A read of standard data."]
    DataRead,
    #[serde(rename = "DATA_WRITE")]
    #[doc = "A write of standard data."]
    DataWrite,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Associates `members` with a `role`."]
pub struct Binding {
    #[serde(rename = "bindingId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub binding_id: ::std::option::Option<::std::string::String>,
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
#[doc = "Write a Cloud Audit log"]
pub struct CloudAuditOptions {
    #[serde(rename = "authorizationLoggingOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information used by the Cloud Audit Logging pipeline."]
    pub authorization_logging_options:
        ::std::option::Option<::std::boxed::Box<AuthorizationLoggingOptions>>,
    #[serde(rename = "logName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The log_name to populate in the Cloud Audit Record."]
    pub log_name: ::std::option::Option<CloudAuditOptionsLogNameEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The log_name to populate in the Cloud Audit Record."]
pub enum CloudAuditOptionsLogNameEnum {
    #[serde(rename = "UNSPECIFIED_LOG_NAME")]
    #[doc = "Default. Should not be used."]
    UnspecifiedLogName,
    #[serde(rename = "ADMIN_ACTIVITY")]
    #[doc = "Corresponds to \"cloudaudit.googleapis.com/activity\""]
    AdminActivity,
    #[serde(rename = "DATA_ACCESS")]
    #[doc = "Corresponds to \"cloudaudit.googleapis.com/data_access\""]
    DataAccess,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A condition to be met."]
pub struct Condition {
    #[serde(rename = "iam")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Trusted attributes supplied by the IAM system."]
    pub iam: ::std::option::Option<ConditionIamEnum>,
    #[serde(rename = "op")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An operator to apply the subject with."]
    pub op: ::std::option::Option<ConditionOpEnum>,
    #[serde(rename = "svc")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Trusted attributes discharged by the service."]
    pub svc: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sys")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Trusted attributes supplied by any service that owns resources and uses the IAM system for access control."]
    pub sys: ::std::option::Option<ConditionSysEnum>,
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The objects of the condition."]
    pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Trusted attributes supplied by the IAM system."]
pub enum ConditionIamEnum {
    #[serde(rename = "NO_ATTR")]
    #[doc = "Default non-attribute."]
    NoAttr,
    #[serde(rename = "AUTHORITY")]
    #[doc = "Either principal or (if present) authority selector."]
    Authority,
    #[serde(rename = "ATTRIBUTION")]
    #[doc = "The principal (even if an authority selector is present), which must only be used for attribution, not authorization."]
    Attribution,
    #[serde(rename = "SECURITY_REALM")]
    #[doc = "Any of the security realms in the IAMContext (go/security-realms). When used with IN, the condition indicates \"any of the request's realms match one of the given values; with NOT_IN, \"none of the realms match any of the given values\". Note that a value can be: - 'self' (i.e., allow connections from clients that are in the same security realm) - 'self:metro' (i.e., clients that are in the same metro) - 'self:cloud-region' (i.e., allow connections from clients that are in the same cloud region) - 'guardians' (i.e., allow connections from its guardian realms. See go/security-realms-glossary#guardian for more information.) - a realm (e.g., 'campus-abc') - a realm group (e.g., 'realms-for-borg-cell-xx', see: go/realm-groups) A match is determined by a realm group membership check performed by a RealmAclRep object (go/realm-acl-howto). It is not permitted to grant access based on the *absence* of a realm, so realm conditions can only be used in a \"positive\" context (e.g., ALLOW/IN or DENY/NOT_IN)."]
    SecurityRealm,
    #[serde(rename = "APPROVER")]
    #[doc = "An approver (distinct from the requester) that has authorized this request. When used with IN, the condition indicates that one of the approvers associated with the request matches the specified principal, or is a member of the specified group. Approvers can only grant additional access, and are thus only used in a strictly positive context (e.g. ALLOW/IN or DENY/NOT_IN)."]
    Approver,
    #[serde(rename = "JUSTIFICATION_TYPE")]
    #[doc = "What types of justifications have been supplied with this request. String values should match enum names from security.credentials.JustificationType, e.g. \"MANUAL_STRING\". It is not permitted to grant access based on the *absence* of a justification, so justification conditions can only be used in a \"positive\" context (e.g., ALLOW/IN or DENY/NOT_IN). Multiple justifications, e.g., a Buganizer ID and a manually-entered reason, are normal and supported."]
    JustificationType,
    #[serde(rename = "CREDENTIALS_TYPE")]
    #[doc = "What type of credentials have been supplied with this request. String values should match enum names from security_loas_l2.CredentialsType - currently, only CREDS_TYPE_EMERGENCY is supported. It is not permitted to grant access based on the *absence* of a credentials type, so the conditions can only be used in a \"positive\" context (e.g., ALLOW/IN or DENY/NOT_IN)."]
    CredentialsType,
    #[serde(rename = "CREDS_ASSERTION")]
    #[doc = "EXPERIMENTAL -- DO NOT USE. The conditions can only be used in a \"positive\" context (e.g., ALLOW/IN or DENY/NOT_IN)."]
    CredsAssertion,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "An operator to apply the subject with."]
pub enum ConditionOpEnum {
    #[serde(rename = "NO_OP")]
    #[doc = "Default no-op."]
    NoOp,
    #[serde(rename = "EQUALS")]
    #[doc = "DEPRECATED. Use IN instead."]
    Equals,
    #[serde(rename = "NOT_EQUALS")]
    #[doc = "DEPRECATED. Use NOT_IN instead."]
    NotEquals,
    #[serde(rename = "IN")]
    #[doc = "The condition is true if the subject (or any element of it if it is a set) matches any of the supplied values."]
    In,
    #[serde(rename = "NOT_IN")]
    #[doc = "The condition is true if the subject (or every element of it if it is a set) matches none of the supplied values."]
    NotIn,
    #[serde(rename = "DISCHARGED")]
    #[doc = "Subject is discharged"]
    Discharged,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Trusted attributes supplied by any service that owns resources and uses the IAM system for access control."]
pub enum ConditionSysEnum {
    #[serde(rename = "NO_ATTR")]
    #[doc = "Default non-attribute type"]
    NoAttr,
    #[serde(rename = "REGION")]
    #[doc = "Region of the resource"]
    Region,
    #[serde(rename = "SERVICE")]
    #[doc = "Service name"]
    Service,
    #[serde(rename = "NAME")]
    #[doc = "Resource name"]
    Name,
    #[serde(rename = "IP")]
    #[doc = "IP address of the caller"]
    Ip,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Increment a streamz counter with the specified metric and field names. Metric names should start with a '/', generally be lowercase-only, and end in \"_count\". Field names should not contain an initial slash. The actual exported metric names will have \"/iam/policy\" prepended. Field names correspond to IAM request parameters and field values are their respective values. Supported field names: - \"authority\", which is \"[token]\" if IAMContext.token is present, otherwise the value of IAMContext.authority_selector if present, and otherwise a representation of IAMContext.principal; or - \"iam_principal\", a representation of IAMContext.principal even if a token or authority selector is present; or - \"\" (empty string), resulting in a counter with no fields. Examples: counter { metric: \"/debug_access_count\" field: \"iam_principal\" } ==> increment counter /iam/policy/debug_access_count {iam_principal=[value of IAMContext.principal]}"]
pub struct CounterOptions {
    #[serde(rename = "customFields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom fields."]
    pub custom_fields: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomField>>>,
    #[serde(rename = "field")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The field value to attribute."]
    pub field: ::std::option::Option<::std::string::String>,
    #[serde(rename = "metric")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The metric to update."]
    pub metric: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Custom fields. These can be used to create a counter with arbitrary field/value pairs. See: go/rpcsp-custom-fields."]
pub struct CustomField {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name is the field name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Value is the field value. It is important that in contrast to the CounterOptions.field, the value here is a constant that is not derived from the IAMContext."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Write a Data Access (Gin) log"]
pub struct DataAccessOptions {
    #[serde(rename = "logMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub log_mode: ::std::option::Option<DataAccessOptionsLogModeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum DataAccessOptionsLogModeEnum {
    #[serde(rename = "LOG_MODE_UNSPECIFIED")]
    #[doc = "Client is not required to write a partial Gin log immediately after the authorization check. If client chooses to write one and it fails, client may either fail open (allow the operation to continue) or fail closed (handle as a DENY outcome)."]
    LogModeUnspecified,
    #[serde(rename = "LOG_FAIL_CLOSED")]
    #[doc = "The application's operation in the context of which this authorization check is being made may only be performed if it is successfully logged to Gin. For instance, the authorization library may satisfy this obligation by emitting a partial log entry at authorization check time and only returning ALLOW to the application if it succeeds. If a matching Rule has this directive, but the client has not indicated that it will honor such requirements, then the IAM check will result in authorization failure by setting CheckPolicyResponse.success=false."]
    LogFailClosed,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The game server cluster changes made by the game server deployment."]
pub struct DeployedClusterState {
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the cluster."]
    pub cluster: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fleetDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The details about the Agones fleets and autoscalers created in the game server cluster."]
    pub fleet_details:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DeployedFleetDetails>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Agones fleet specification and details."]
pub struct DeployedFleet {
    #[serde(rename = "fleet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the Agones fleet."]
    pub fleet: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fleetSpec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fleet spec retrieved from the Agones fleet."]
    pub fleet_spec: ::std::option::Option<::std::string::String>,
    #[serde(rename = "specSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source spec that is used to create the Agones fleet. The GameServerConfig resource may no longer exist in the system."]
    pub spec_source: ::std::option::Option<::std::boxed::Box<SpecSource>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current status of the Agones fleet. Includes count of game servers in various states."]
    pub status: ::std::option::Option<::std::boxed::Box<DeployedFleetStatus>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details about the Agones autoscaler."]
pub struct DeployedFleetAutoscaler {
    #[serde(rename = "autoscaler")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the Agones autoscaler."]
    pub autoscaler: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fleetAutoscalerSpec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The autoscaler spec retrieved from Agones."]
    pub fleet_autoscaler_spec: ::std::option::Option<::std::string::String>,
    #[serde(rename = "specSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source spec that is used to create the autoscaler. The GameServerConfig resource may no longer exist in the system."]
    pub spec_source: ::std::option::Option<::std::boxed::Box<SpecSource>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of the deployed Agones fleet."]
pub struct DeployedFleetDetails {
    #[serde(rename = "deployedAutoscaler")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the Agones autoscaler for that fleet."]
    pub deployed_autoscaler: ::std::option::Option<::std::boxed::Box<DeployedFleetAutoscaler>>,
    #[serde(rename = "deployedFleet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the Agones fleet."]
    pub deployed_fleet: ::std::option::Option<::std::boxed::Box<DeployedFleet>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "DeployedFleetStatus has details about the Agones fleets such as how many are running, how many allocated, and so on."]
pub struct DeployedFleetStatus {
    #[serde(rename = "allocatedReplicas")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of GameServer replicas in the ALLOCATED state in this fleet."]
    pub allocated_replicas: ::std::option::Option<::std::string::String>,
    #[serde(rename = "readyReplicas")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of GameServer replicas in the READY state in this fleet."]
    pub ready_replicas: ::std::option::Option<::std::string::String>,
    #[serde(rename = "replicas")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total number of current GameServer replicas in this fleet."]
    pub replicas: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reservedReplicas")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of GameServer replicas in the RESERVED state in this fleet. Reserved instances won't be deleted on scale down, but won't cause an autoscaler to scale up."]
    pub reserved_replicas: ::std::option::Option<::std::string::String>,
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
#[doc = "Request message for GameServerDeploymentsService.FetchDeploymentState."]
pub struct FetchDeploymentStateRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for GameServerDeploymentsService.FetchDeploymentState."]
pub struct FetchDeploymentStateResponse {
    #[serde(rename = "clusterState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state of the game server deployment in each game server cluster."]
    pub cluster_state:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DeployedClusterState>>>,
    #[serde(rename = "unavailable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of locations that could not be reached."]
    pub unavailable: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Fleet configs for Agones."]
pub struct FleetConfig {
    #[serde(rename = "fleetSpec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Agones fleet spec. Example spec: `https://agones.dev/site/docs/reference/fleet/`."]
    pub fleet_spec: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the FleetConfig."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A game server cluster resource."]
pub struct GameServerCluster {
    #[serde(rename = "allocationPriority")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The allocation priority assigned to the game server cluster. Game server clusters receive new game server allocations based on the relative allocation priorites set for each cluster, if the realm is configured for multicluster allocation."]
    pub allocation_priority: ::std::option::Option<GameServerClusterAllocationPriorityEnum>,
    #[serde(rename = "connectionInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The game server cluster connection information. This information is used to manage game server clusters."]
    pub connection_info: ::std::option::Option<::std::boxed::Box<GameServerClusterConnectionInfo>>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The creation time."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Human readable description of the cluster."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of the resource."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The labels associated with this game server cluster. Each label is a key-value pair."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The resource name of the game server cluster, in the following form: `projects/{project}/locations/{location}/realms/{realm}/gameServerClusters/{cluster}`. For example, `projects/my-project/locations/{location}/realms/zanzibar/gameServerClusters/my-onprem-cluster`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The last-modified time."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Optional. The allocation priority assigned to the game server cluster. Game server clusters receive new game server allocations based on the relative allocation priorites set for each cluster, if the realm is configured for multicluster allocation."]
pub enum GameServerClusterAllocationPriorityEnum {
    #[serde(rename = "PRIORITY_UNSPECIFIED")]
    #[doc = "The default allocation priority. `PRIORITY_UNSPECIFIED` is the lowest possible priority."]
    PriorityUnspecified,
    #[serde(rename = "P1")]
    #[doc = "Priority 1, the highest priority."]
    P1,
    #[serde(rename = "P2")]
    #[doc = "Priority 2."]
    P2,
    #[serde(rename = "P3")]
    #[doc = "Priority 3."]
    P3,
    #[serde(rename = "P4")]
    #[doc = "Priority 4."]
    P4,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The game server cluster connection information."]
pub struct GameServerClusterConnectionInfo {
    #[serde(rename = "gkeClusterReference")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reference to the GKE cluster where the game servers are installed."]
    pub gke_cluster_reference: ::std::option::Option<::std::boxed::Box<GkeClusterReference>>,
    #[serde(rename = "gkeHubClusterReference")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reference to a Kubernetes cluster registered through GKE Hub. See https://cloud.google.com/anthos/multicluster-management/ for more information about registering Kubernetes clusters."]
    pub gke_hub_cluster_reference: ::std::option::Option<::std::boxed::Box<GkeHubClusterReference>>,
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Namespace designated on the game server cluster where the Agones game server instances will be created. Existence of the namespace will be validated during creation."]
    pub namespace: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A game server config resource."]
pub struct GameServerConfig {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The creation time."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The description of the game server config."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fleetConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "FleetConfig contains a list of Agones fleet specs. Only one FleetConfig is allowed."]
    pub fleet_configs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FleetConfig>>>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The labels associated with this game server config. Each label is a key-value pair."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the game server config, in the following form: `projects/{project}/locations/{location}/gameServerDeployments/{deployment}/configs/{config}`. For example, `projects/my-project/locations/global/gameServerDeployments/my-game/configs/my-config`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scalingConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The autoscaling settings."]
    pub scaling_configs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ScalingConfig>>>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The last-modified time."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A game server config override."]
pub struct GameServerConfigOverride {
    #[serde(rename = "configVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The game server config for this override."]
    pub config_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "realmsSelector")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Selector for choosing applicable realms."]
    pub realms_selector: ::std::option::Option<::std::boxed::Box<RealmSelector>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A game server deployment resource."]
pub struct GameServerDeployment {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The creation time."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Human readable description of the game server delpoyment."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of the resource."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The labels associated with this game server deployment. Each label is a key-value pair."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the game server deployment, in the following form: `projects/{project}/locations/{location}/gameServerDeployments/{deployment}`. For example, `projects/my-project/locations/global/gameServerDeployments/my-deployment`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The last-modified time."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The game server deployment rollout which represents the desired rollout state."]
pub struct GameServerDeploymentRollout {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The creation time."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "defaultGameServerConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The default game server config is applied to all realms unless overridden in the rollout. For example, `projects/my-project/locations/global/gameServerDeployments/my-game/configs/my-config`."]
    pub default_game_server_config: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of the resource."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gameServerConfigOverrides")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Contains the game server config rollout overrides. Overrides are processed in the order they are listed. Once a match is found for a realm, the rest of the list is not processed."]
    pub game_server_config_overrides:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GameServerConfigOverride>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the game server deployment rollout, in the following form: `projects/{project}/locations/{location}/gameServerDeployments/{deployment}/rollout`. For example, `projects/my-project/locations/global/gameServerDeployments/my-deployment/rollout`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The last-modified time."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A reference to a GKE cluster."]
pub struct GkeClusterReference {
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full or partial name of a GKE cluster, using one of the following forms: * `projects/{project}/locations/{location}/clusters/{cluster}` * `locations/{location}/clusters/{cluster}` * `{cluster}` If project and location are not specified, the project and location of the GameServerCluster resource are used to generate the full name of the GKE cluster."]
    pub cluster: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "GkeHubClusterReference represents a reference to a Kubernetes cluster registered through GKE Hub."]
pub struct GkeHubClusterReference {
    #[serde(rename = "membership")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full or partial name of a GKE Hub membership, using one of the following forms: * `https://gkehub.googleapis.com/v1beta1/projects/{project_id}/locations/global/memberships/{membership_id}` * `projects/{project_id}/locations/global/memberships/{membership_id}` * `{membership_id}` If project is not specified, the project of the GameServerCluster resource is used to generate the full name of the GKE Hub membership."]
    pub membership: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The label selector, used to group labels on the resources."]
pub struct LabelSelector {
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource labels for this selector."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for GameServerClustersService.ListGameServerClusters."]
pub struct ListGameServerClustersResponse {
    #[serde(rename = "gameServerClusters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of game server clusters."]
    pub game_server_clusters:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GameServerCluster>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "unreachable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of locations that could not be reached."]
    pub unreachable: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for GameServerConfigsService.ListGameServerConfigs."]
pub struct ListGameServerConfigsResponse {
    #[serde(rename = "gameServerConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of game server configs."]
    pub game_server_configs:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GameServerConfig>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "unreachable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of locations that could not be reached."]
    pub unreachable: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for GameServerDeploymentsService.ListGameServerDeployments."]
pub struct ListGameServerDeploymentsResponse {
    #[serde(rename = "gameServerDeployments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of game server deployments."]
    pub game_server_deployments:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GameServerDeployment>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "unreachable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of locations that could not be reached."]
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
#[doc = "Response message for RealmsService.ListRealms."]
pub struct ListRealmsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "realms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of realms."]
    pub realms: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Realm>>>,
    #[serde(rename = "unreachable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of locations that could not be reached."]
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
#[doc = "Specifies what kind of log the caller must write"]
pub struct LogConfig {
    #[serde(rename = "cloudAudit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud audit options."]
    pub cloud_audit: ::std::option::Option<::std::boxed::Box<CloudAuditOptions>>,
    #[serde(rename = "counter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Counter options."]
    pub counter: ::std::option::Option<::std::boxed::Box<CounterOptions>>,
    #[serde(rename = "dataAccess")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Data access options."]
    pub data_access: ::std::option::Option<::std::boxed::Box<DataAccessOptions>>,
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
#[doc = "Represents the metadata of the long-running operation."]
pub struct OperationMetadata {
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
    #[serde(rename = "operationStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Operation status for Game Services API operations. Operation status is in the form of key-value pairs where keys are resource IDs and the values show the status of the operation. In case of failures, the value includes an error code and error message."]
    pub operation_status: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<OperationStatus>>,
    >,
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
    #[serde(rename = "unreachable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. List of Locations that could not be reached."]
    pub unreachable: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "verb")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Name of the verb executed by the operation."]
    pub verb: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperationStatus {
    #[serde(rename = "done")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Whether the operation is done or still in progress."]
    pub done: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The error code in case of failures."]
    pub error_code: ::std::option::Option<OperationStatusErrorCodeEnum>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The human-readable error message."]
    pub error_message: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The error code in case of failures."]
pub enum OperationStatusErrorCodeEnum {
    #[serde(rename = "ERROR_CODE_UNSPECIFIED")]
    #[doc = ""]
    ErrorCodeUnspecified,
    #[serde(rename = "INTERNAL_ERROR")]
    #[doc = ""]
    InternalError,
    #[serde(rename = "PERMISSION_DENIED")]
    #[doc = ""]
    PermissionDenied,
    #[serde(rename = "CLUSTER_CONNECTION")]
    #[doc = ""]
    ClusterConnection,
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
    #[serde(rename = "iamOwned")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub iam_owned: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "rules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If more than one rule is specified, the rules are applied in the following manner: - All matching LOG rules are always applied. - If any DENY/DENY_WITH_LOG rule matches, permission is denied. Logging will be applied if one or more matching rule requires logging. - Otherwise, if any ALLOW/ALLOW_WITH_LOG rule matches, permission is granted. Logging will be applied if one or more matching rule requires logging. - Otherwise, if no rule applies, permission is denied."]
    pub rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Rule>>>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
    pub version: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for GameServerClustersService.PreviewCreateGameServerCluster."]
pub struct PreviewCreateGameServerClusterResponse {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ETag of the game server cluster."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target state."]
    pub target_state: ::std::option::Option<::std::boxed::Box<TargetState>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for GameServerClustersService.PreviewDeleteGameServerCluster."]
pub struct PreviewDeleteGameServerClusterResponse {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ETag of the game server cluster."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target state."]
    pub target_state: ::std::option::Option<::std::boxed::Box<TargetState>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for PreviewGameServerDeploymentRollout. This has details about the Agones fleet and autoscaler to be actuated."]
pub struct PreviewGameServerDeploymentRolloutResponse {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of the game server deployment."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target state."]
    pub target_state: ::std::option::Option<::std::boxed::Box<TargetState>>,
    #[serde(rename = "unavailable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Locations that could not be reached on this request."]
    pub unavailable: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for RealmsService.PreviewRealmUpdate."]
pub struct PreviewRealmUpdateResponse {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of the realm."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target state."]
    pub target_state: ::std::option::Option<::std::boxed::Box<TargetState>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for GameServerClustersService.PreviewUpdateGameServerCluster"]
pub struct PreviewUpdateGameServerClusterResponse {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ETag of the game server cluster."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target state."]
    pub target_state: ::std::option::Option<::std::boxed::Box<TargetState>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A realm resource."]
pub struct Realm {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The creation time."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Human readable description of the realm."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ETag of the resource."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The labels associated with this realm. Each label is a key-value pair."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the realm, in the following form: `projects/{project}/locations/{location}/realms/{realm}`. For example, `projects/my-project/locations/{location}/realms/my-realm`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Time zone where all policies targeting this realm are evaluated. The value of this field must be from the IANA time zone database: https://www.iana.org/time-zones."]
    pub time_zone: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The last-modified time."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The realm selector, used to match realm resources."]
pub struct RealmSelector {
    #[serde(rename = "realms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of realms to match."]
    pub realms: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A rule to be applied in a Policy."]
pub struct Rule {
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required"]
    pub action: ::std::option::Option<RuleActionEnum>,
    #[serde(rename = "conditions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional restrictions that must be met. All conditions must pass for the rule to match."]
    pub conditions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Condition>>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Human-readable description of the rule."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "in")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If one or more 'in' clauses are specified, the rule matches if the PRINCIPAL/AUTHORITY_SELECTOR is in at least one of these entries."]
    pub _in: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "logConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The config returned to callers of tech.iam.IAM.CheckPolicy for any entries that match the LOG action."]
    pub log_config: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LogConfig>>>,
    #[serde(rename = "notIn")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If one or more 'not_in' clauses are specified, the rule matches if the PRINCIPAL/AUTHORITY_SELECTOR is in none of the entries. The format for in and not_in entries can be found at in the Local IAM documentation (see go/local-iam#features)."]
    pub not_in: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A permission is a string of form '..' (e.g., 'storage.buckets.list'). A value of '*' matches all permissions, and a verb part of '*' (e.g., 'storage.buckets.*') matches all verbs."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required"]
pub enum RuleActionEnum {
    #[serde(rename = "NO_ACTION")]
    #[doc = "Default no action."]
    NoAction,
    #[serde(rename = "ALLOW")]
    #[doc = "Matching 'Entries' grant access."]
    Allow,
    #[serde(rename = "ALLOW_WITH_LOG")]
    #[doc = "Matching 'Entries' grant access and the caller promises to log the request per the returned log_configs."]
    AllowWithLog,
    #[serde(rename = "DENY")]
    #[doc = "Matching 'Entries' deny access."]
    Deny,
    #[serde(rename = "DENY_WITH_LOG")]
    #[doc = "Matching 'Entries' deny access and the caller promises to log the request per the returned log_configs."]
    DenyWithLog,
    #[serde(rename = "LOG")]
    #[doc = "Matching 'Entries' tell IAM.Check callers to generate logs."]
    Log,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Autoscaling config for an Agones fleet."]
pub struct ScalingConfig {
    #[serde(rename = "fleetAutoscalerSpec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Agones fleet autoscaler spec. Example spec: https://agones.dev/site/docs/reference/fleetautoscaler/"]
    pub fleet_autoscaler_spec: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the Scaling Config"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "schedules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The schedules to which this Scaling Config applies."]
    pub schedules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Schedule>>>,
    #[serde(rename = "selectors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Labels used to identify the game server clusters to which this Agones scaling config applies. A game server cluster is subject to this Agones scaling config if its labels match any of the selector entries."]
    pub selectors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LabelSelector>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The schedule of a recurring or one time event. The event's time span is specified by start_time and end_time. If the scheduled event's timespan is larger than the cron_spec + cron_job_duration, the event will be recurring. If only cron_spec + cron_job_duration are specified, the event is effective starting at the local time specified by cron_spec, and is recurring. start_time|-------[cron job]-------[cron job]-------[cron job]---|end_time cron job: cron spec start time + duration"]
pub struct Schedule {
    #[serde(rename = "cronJobDuration")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The duration for the cron job event. The duration of the event is effective after the cron job's start time."]
    pub cron_job_duration: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cronSpec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The cron definition of the scheduled event. See https://en.wikipedia.org/wiki/Cron. Cron spec specifies the local time as defined by the realm."]
    pub cron_spec: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The end time of the event."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The start time of the event."]
    pub start_time: ::std::option::Option<::std::string::String>,
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
#[doc = "Encapsulates Agones fleet spec and Agones autoscaler spec sources."]
pub struct SpecSource {
    #[serde(rename = "gameServerConfigName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The game server config resource. Uses the form: `projects/{project}/locations/{location}/gameServerDeployments/{deployment_id}/configs/{config_id}`."]
    pub game_server_config_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the Agones leet config or Agones scaling config used to derive the Agones fleet or Agones autoscaler spec."]
    pub name: ::std::option::Option<::std::string::String>,
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
#[doc = "Details about the Agones resources."]
pub struct TargetDetails {
    #[serde(rename = "fleetDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Agones fleet details for game server clusters and game server deployments."]
    pub fleet_details:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TargetFleetDetails>>>,
    #[serde(rename = "gameServerClusterName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The game server cluster name. Uses the form: `projects/{project}/locations/{location}/realms/{realm}/gameServerClusters/{cluster}`."]
    pub game_server_cluster_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gameServerDeploymentName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The game server deployment name. Uses the form: `projects/{project}/locations/{location}/gameServerDeployments/{deployment_id}`."]
    pub game_server_deployment_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Target Agones fleet specification."]
pub struct TargetFleet {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the Agones fleet."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "specSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Encapsulates the source of the Agones fleet spec. The Agones fleet spec source."]
    pub spec_source: ::std::option::Option<::std::boxed::Box<SpecSource>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Target Agones autoscaler policy reference."]
pub struct TargetFleetAutoscaler {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the Agones autoscaler."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "specSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Encapsulates the source of the Agones fleet spec. Details about the Agones autoscaler spec."]
    pub spec_source: ::std::option::Option<::std::boxed::Box<SpecSource>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details of the target Agones fleet."]
pub struct TargetFleetDetails {
    #[serde(rename = "autoscaler")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reference to target Agones fleet autoscaling policy."]
    pub autoscaler: ::std::option::Option<::std::boxed::Box<TargetFleetAutoscaler>>,
    #[serde(rename = "fleet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reference to target Agones fleet."]
    pub fleet: ::std::option::Option<::std::boxed::Box<TargetFleet>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Encapsulates the Target state."]
pub struct TargetState {
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details about Agones fleets."]
    pub details: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TargetDetails>>>,
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