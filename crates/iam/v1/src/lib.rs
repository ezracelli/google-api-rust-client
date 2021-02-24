#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Audit log information specific to Cloud IAM admin APIs. This message is serialized as an `Any` type in the `ServiceData` message of an `AuditLog` message."]
pub struct AdminAuditData {
    #[serde(rename = "permissionDelta")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The permission_delta when when creating or updating a Role."]
    pub permission_delta: ::std::option::Option<::std::boxed::Box<PermissionDelta>>,
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
#[doc = "Audit log information specific to Cloud IAM. This message is serialized as an `Any` type in the `ServiceData` message of an `AuditLog` message."]
pub struct AuditData {
    #[serde(rename = "policyDelta")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Policy delta between the original policy and the newly set policy."]
    pub policy_delta: ::std::option::Option<::std::boxed::Box<PolicyDelta>>,
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
#[doc = "Contains information about an auditable service."]
pub struct AuditableService {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Public name of the service. For example, the service name for Cloud IAM is 'iam.googleapis.com'."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an Amazon Web Services identity provider."]
pub struct Aws {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The AWS account ID."]
    pub account_id: ::std::option::Option<::std::string::String>,
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
#[doc = "One delta entry for Binding. Each individual change (only one member in each entry) to a binding will be a separate entry."]
pub struct BindingDelta {
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The action that was performed on a Binding. Required"]
    pub action: ::std::option::Option<BindingDeltaActionEnum>,
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The condition that is associated with this binding."]
    pub condition: ::std::option::Option<::std::boxed::Box<Expr>>,
    #[serde(rename = "member")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A single identity requesting access for a Cloud Platform resource. Follows the same format of Binding.members. Required"]
    pub member: ::std::option::Option<::std::string::String>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Role that is assigned to `members`. For example, `roles/viewer`, `roles/editor`, or `roles/owner`. Required"]
    pub role: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The action that was performed on a Binding. Required"]
pub enum BindingDeltaActionEnum {
    #[serde(rename = "ACTION_UNSPECIFIED")]
    #[doc = "Unspecified."]
    ActionUnspecified,
    #[serde(rename = "ADD")]
    #[doc = "Addition of a Binding."]
    Add,
    #[serde(rename = "REMOVE")]
    #[doc = "Removal of a Binding."]
    Remove,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request to create a new role."]
pub struct CreateRoleRequest {
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Role resource to create."]
    pub role: ::std::option::Option<::std::boxed::Box<Role>>,
    #[serde(rename = "roleId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The role ID to use for this role. A role ID may contain alphanumeric characters, underscores (`_`), and periods (`.`). It must contain a minimum of 3 characters and a maximum of 64 characters."]
    pub role_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The service account key create request."]
pub struct CreateServiceAccountKeyRequest {
    #[serde(rename = "keyAlgorithm")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Which type of key and algorithm to use for the key. The default is currently a 2K RSA key. However this may change in the future."]
    pub key_algorithm: ::std::option::Option<CreateServiceAccountKeyRequestKeyAlgorithmEnum>,
    #[serde(rename = "privateKeyType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The output format of the private key. The default value is `TYPE_GOOGLE_CREDENTIALS_FILE`, which is the Google Credentials File format."]
    pub private_key_type: ::std::option::Option<CreateServiceAccountKeyRequestPrivateKeyTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Which type of key and algorithm to use for the key. The default is currently a 2K RSA key. However this may change in the future."]
pub enum CreateServiceAccountKeyRequestKeyAlgorithmEnum {
    #[serde(rename = "KEY_ALG_UNSPECIFIED")]
    #[doc = "An unspecified key algorithm."]
    KeyAlgUnspecified,
    #[serde(rename = "KEY_ALG_RSA_1024")]
    #[doc = "1k RSA Key."]
    KeyAlgRsa1024,
    #[serde(rename = "KEY_ALG_RSA_2048")]
    #[doc = "2k RSA Key."]
    KeyAlgRsa2048,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The output format of the private key. The default value is `TYPE_GOOGLE_CREDENTIALS_FILE`, which is the Google Credentials File format."]
pub enum CreateServiceAccountKeyRequestPrivateKeyTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "Unspecified. Equivalent to `TYPE_GOOGLE_CREDENTIALS_FILE`."]
    TypeUnspecified,
    #[serde(rename = "TYPE_PKCS12_FILE")]
    #[doc = "PKCS12 format. The password for the PKCS12 file is `notasecret`. For more information, see https://tools.ietf.org/html/rfc7292."]
    TypePkcs12File,
    #[serde(rename = "TYPE_GOOGLE_CREDENTIALS_FILE")]
    #[doc = "Google Credentials File format."]
    TypeGoogleCredentialsFile,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The service account create request."]
pub struct CreateServiceAccountRequest {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The account id that is used to generate the service account email address and a stable unique id. It is unique within a project, must be 6-30 characters long, and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])` to comply with RFC1035."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serviceAccount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ServiceAccount resource to create. Currently, only the following values are user assignable: `display_name` and `description`."]
    pub service_account: ::std::option::Option<::std::boxed::Box<ServiceAccount>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The service account disable request."]
pub struct DisableServiceAccountRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The service account enable request."]
pub struct EnableServiceAccountRequest {}
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
#[doc = "The request to lint a Cloud IAM policy object."]
pub struct LintPolicyRequest {
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "google.iam.v1.Binding.condition object to be linted."]
    pub condition: ::std::option::Option<::std::boxed::Box<Expr>>,
    #[serde(rename = "fullResourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full resource name of the policy this lint request is about. The name follows the Google Cloud Platform (GCP) resource format. For example, a GCP project with ID `my-project` will be named `//cloudresourcemanager.googleapis.com/projects/my-project`. The resource name is not used to read the policy instance from the Cloud IAM database. The candidate policy for lint has to be provided in the same request object."]
    pub full_resource_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response of a lint operation. An empty response indicates the operation was able to fully execute and no lint issue was found."]
pub struct LintPolicyResponse {
    #[serde(rename = "lintResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of lint results sorted by `severity` in descending order."]
    pub lint_results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LintResult>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Structured response of a single validation unit."]
pub struct LintResult {
    #[serde(rename = "debugMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Human readable debug message associated with the issue."]
    pub debug_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fieldName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the field for which this lint result is about. For nested messages `field_name` consists of names of the embedded fields separated by period character. The top-level qualifier is the input object to lint in the request. For example, the `field_name` value `condition.expression` identifies a lint result for the `expression` field of the provided condition."]
    pub field_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "level")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The validation unit level."]
    pub level: ::std::option::Option<LintResultLevelEnum>,
    #[serde(rename = "locationOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "0-based character position of problematic construct within the object identified by `field_name`. Currently, this is populated only for condition expression."]
    pub location_offset: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The validation unit severity."]
    pub severity: ::std::option::Option<LintResultSeverityEnum>,
    #[serde(rename = "validationUnitName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The validation unit name, for instance \"lintValidationUnits/ConditionComplexityCheck\"."]
    pub validation_unit_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The validation unit level."]
pub enum LintResultLevelEnum {
    #[serde(rename = "LEVEL_UNSPECIFIED")]
    #[doc = "Level is unspecified."]
    LevelUnspecified,
    #[serde(rename = "CONDITION")]
    #[doc = "A validation unit which operates on an individual condition within a binding."]
    Condition,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The validation unit severity."]
pub enum LintResultSeverityEnum {
    #[serde(rename = "SEVERITY_UNSPECIFIED")]
    #[doc = "Severity is unspecified."]
    SeverityUnspecified,
    #[serde(rename = "ERROR")]
    #[doc = "A validation unit returns an error only for critical issues. If an attempt is made to set the problematic policy without rectifying the critical issue, it causes the `setPolicy` operation to fail."]
    Error,
    #[serde(rename = "WARNING")]
    #[doc = "Any issue which is severe enough but does not cause an error. For example, suspicious constructs in the input object will not necessarily fail `setPolicy`, but there is a high likelihood that they won't behave as expected during policy evaluation in `checkPolicy`. This includes the following common scenarios: - Unsatisfiable condition: Expired timestamp in date/time condition. - Ineffective condition: Condition on a pair which is granted unconditionally in another binding of the same policy."]
    Warning,
    #[serde(rename = "NOTICE")]
    #[doc = "Reserved for the issues that are not severe as `ERROR`/`WARNING`, but need special handling. For instance, messages about skipped validation units are issued as `NOTICE`."]
    Notice,
    #[serde(rename = "INFO")]
    #[doc = "Any informative statement which is not severe enough to raise `ERROR`/`WARNING`/`NOTICE`, like auto-correction recommendations on the input content. Note that current version of the linter does not utilize `INFO`."]
    Info,
    #[serde(rename = "DEPRECATED")]
    #[doc = "Deprecated severity level."]
    Deprecated,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response containing the roles defined under a resource."]
pub struct ListRolesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "To retrieve the next page of results, set `ListRolesRequest.page_token` to this value."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "roles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Roles defined on this resource."]
    pub roles: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Role>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The service account keys list response."]
pub struct ListServiceAccountKeysResponse {
    #[serde(rename = "keys")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The public keys for the service account."]
    pub keys: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ServiceAccountKey>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The service account list response."]
pub struct ListServiceAccountsResponse {
    #[serde(rename = "accounts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of matching service accounts."]
    pub accounts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ServiceAccount>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "To retrieve the next page of results, set ListServiceAccountsRequest.page_token to this value."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ListWorkloadIdentityPoolProviders."]
pub struct ListWorkloadIdentityPoolProvidersResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "workloadIdentityPoolProviders")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of providers."]
    pub workload_identity_pool_providers:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WorkloadIdentityPoolProvider>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ListWorkloadIdentityPools."]
pub struct ListWorkloadIdentityPoolsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "workloadIdentityPools")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of pools."]
    pub workload_identity_pools:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WorkloadIdentityPool>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an OpenId Connect 1.0 identity provider."]
pub struct Oidc {
    #[serde(rename = "allowedAudiences")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Acceptable values for the `aud` field (audience) in the OIDC token. Token exchange requests are rejected if the token audience does not match one of the configured values. Each audience may be at most 256 characters. A maximum of 10 audiences may be configured. If this list is empty, the OIDC token audience must be equal to the full canonical resource name of the WorkloadIdentityPoolProvider, with or without the HTTPS prefix. For example: ``` //iam.googleapis.com/projects//locations//workloadIdentityPools//providers/ https://iam.googleapis.com/projects//locations//workloadIdentityPools//providers/ ```"]
    pub allowed_audiences: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "issuerUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The OIDC issuer URL."]
    pub issuer_uri: ::std::option::Option<::std::string::String>,
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
#[doc = "The request for PatchServiceAccount. You can patch only the `display_name` and `description` fields. You must use the `update_mask` field to specify which of these fields you want to patch. Only the fields specified in the request are guaranteed to be returned in the response. Other fields may be empty in the response."]
pub struct PatchServiceAccountRequest {
    #[serde(rename = "serviceAccount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub service_account: ::std::option::Option<::std::boxed::Box<ServiceAccount>>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub update_mask: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A permission which can be included by a role."]
pub struct Permission {
    #[serde(rename = "apiDisabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The service API associated with the permission is not enabled."]
    pub api_disabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "customRolesSupportLevel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current custom role support level."]
    pub custom_roles_support_level: ::std::option::Option<PermissionCustomRolesSupportLevelEnum>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A brief description of what this Permission is used for. This permission can ONLY be used in predefined roles."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of this Permission."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "onlyInPredefinedRoles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub only_in_predefined_roles: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "primaryPermission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The preferred name for this permission. If present, then this permission is an alias of, and equivalent to, the listed primary_permission."]
    pub primary_permission: ::std::option::Option<::std::string::String>,
    #[serde(rename = "stage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current launch stage of the permission."]
    pub stage: ::std::option::Option<PermissionStageEnum>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of this Permission."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The current custom role support level."]
pub enum PermissionCustomRolesSupportLevelEnum {
    #[serde(rename = "SUPPORTED")]
    #[doc = "Permission is fully supported for custom role use."]
    Supported,
    #[serde(rename = "TESTING")]
    #[doc = "Permission is being tested to check custom role compatibility."]
    Testing,
    #[serde(rename = "NOT_SUPPORTED")]
    #[doc = "Permission is not supported for custom role use."]
    NotSupported,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The current launch stage of the permission."]
pub enum PermissionStageEnum {
    #[serde(rename = "ALPHA")]
    #[doc = "The permission is currently in an alpha phase."]
    Alpha,
    #[serde(rename = "BETA")]
    #[doc = "The permission is currently in a beta phase."]
    Beta,
    #[serde(rename = "GA")]
    #[doc = "The permission is generally available."]
    Ga,
    #[serde(rename = "DEPRECATED")]
    #[doc = "The permission is being deprecated."]
    Deprecated,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A PermissionDelta message to record the added_permissions and removed_permissions inside a role."]
pub struct PermissionDelta {
    #[serde(rename = "addedPermissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Added permissions."]
    pub added_permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "removedPermissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Removed permissions."]
    pub removed_permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
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
#[doc = "The difference delta between two policies."]
pub struct PolicyDelta {
    #[serde(rename = "bindingDeltas")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The delta for Bindings between two policies."]
    pub binding_deltas: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BindingDelta>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request to get the list of auditable services for a resource."]
pub struct QueryAuditableServicesRequest {
    #[serde(rename = "fullResourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The full resource name to query from the list of auditable services. The name follows the Google Cloud Platform resource format. For example, a Cloud Platform project with id `my-project` will be named `//cloudresourcemanager.googleapis.com/projects/my-project`."]
    pub full_resource_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A response containing a list of auditable services for a resource."]
pub struct QueryAuditableServicesResponse {
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The auditable services for a resource."]
    pub services: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuditableService>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The grantable role query request."]
pub struct QueryGrantableRolesRequest {
    #[serde(rename = "fullResourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The full resource name to query from the list of grantable roles. The name follows the Google Cloud Platform resource format. For example, a Cloud Platform project with id `my-project` will be named `//cloudresourcemanager.googleapis.com/projects/my-project`."]
    pub full_resource_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional limit on the number of roles to include in the response. The default is 300, and the maximum is 1,000."]
    pub page_size: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional pagination token returned in an earlier QueryGrantableRolesResponse."]
    pub page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "view")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub view: ::std::option::Option<QueryGrantableRolesRequestViewEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum QueryGrantableRolesRequestViewEnum {
    #[serde(rename = "BASIC")]
    #[doc = "Omits the `included_permissions` field. This is the default value."]
    Basic,
    #[serde(rename = "FULL")]
    #[doc = "Returns all fields."]
    Full,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The grantable role query response."]
pub struct QueryGrantableRolesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "To retrieve the next page of results, set `QueryGrantableRolesRequest.page_token` to this value."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "roles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of matching roles."]
    pub roles: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Role>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A request to get permissions which can be tested on a resource."]
pub struct QueryTestablePermissionsRequest {
    #[serde(rename = "fullResourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The full resource name to query from the list of testable permissions. The name follows the Google Cloud Platform resource format. For example, a Cloud Platform project with id `my-project` will be named `//cloudresourcemanager.googleapis.com/projects/my-project`."]
    pub full_resource_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional limit on the number of permissions to include in the response. The default is 100, and the maximum is 1,000."]
    pub page_size: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional pagination token returned in an earlier QueryTestablePermissionsRequest."]
    pub page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response containing permissions which can be tested on a resource."]
pub struct QueryTestablePermissionsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "To retrieve the next page of results, set `QueryTestableRolesRequest.page_token` to this value."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Permissions testable on the requested resource."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Permission>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A role in the Identity and Access Management API."]
pub struct Role {
    #[serde(rename = "deleted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current deleted state of the role. This field is read only. It will be ignored in calls to CreateRole and UpdateRole."]
    pub deleted: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A human-readable description for the role."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Used to perform a consistent read-modify-write."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "includedPermissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The names of the permissions this role grants when bound in an IAM policy."]
    pub included_permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the role. When Role is used in CreateRole, the role name must not be set. When Role is used in output and other input such as UpdateRole, the role name is the complete path, e.g., roles/logging.viewer for predefined roles and organizations/{ORGANIZATION_ID}/roles/logging.viewer for custom roles."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "stage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current launch stage of the role. If the `ALPHA` launch stage has been selected for a role, the `stage` field will not be included in the returned definition for the role."]
    pub stage: ::std::option::Option<RoleStageEnum>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A human-readable title for the role. Typically this is limited to 100 UTF-8 bytes."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The current launch stage of the role. If the `ALPHA` launch stage has been selected for a role, the `stage` field will not be included in the returned definition for the role."]
pub enum RoleStageEnum {
    #[serde(rename = "ALPHA")]
    #[doc = "The user has indicated this role is currently in an Alpha phase. If this launch stage is selected, the `stage` field will not be included when requesting the definition for a given role."]
    Alpha,
    #[serde(rename = "BETA")]
    #[doc = "The user has indicated this role is currently in a Beta phase."]
    Beta,
    #[serde(rename = "GA")]
    #[doc = "The user has indicated this role is generally available."]
    Ga,
    #[serde(rename = "DEPRECATED")]
    #[doc = "The user has indicated this role is being deprecated."]
    Deprecated,
    #[serde(rename = "DISABLED")]
    #[doc = "This role is disabled and will not contribute permissions to any members it is granted to in policies."]
    Disabled,
    #[serde(rename = "EAP")]
    #[doc = "The user has indicated this role is currently in an EAP phase."]
    Eap,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An IAM service account. A service account is an account for an application or a virtual machine (VM) instance, not a person. You can use a service account to call Google APIs. To learn more, read the [overview of service accounts](https://cloud.google.com/iam/help/service-accounts/overview). When you create a service account, you specify the project ID that owns the service account, as well as a name that must be unique within the project. IAM uses these values to create an email address that identifies the service account."]
pub struct ServiceAccount {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A user-specified, human-readable description of the service account. The maximum length is 256 UTF-8 bytes."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "disabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Whether the service account is disabled."]
    pub disabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A user-specified, human-readable name for the service account. The maximum length is 100 UTF-8 bytes."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The email address of the service account."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Do not use."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the service account. Use one of the following formats: * `projects/{PROJECT_ID}/serviceAccounts/{EMAIL_ADDRESS}` * `projects/{PROJECT_ID}/serviceAccounts/{UNIQUE_ID}` As an alternative, you can use the `-` wildcard character instead of the project ID: * `projects/-/serviceAccounts/{EMAIL_ADDRESS}` * `projects/-/serviceAccounts/{UNIQUE_ID}` When possible, avoid using the `-` wildcard character, because it can cause response messages to contain misleading error codes. For example, if you try to get the service account `projects/-/serviceAccounts/fake@example.com`, which does not exist, the response contains an HTTP `403 Forbidden` error instead of a `404 Not Found` error."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "oauth2ClientId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The OAuth 2.0 client ID for the service account."]
    pub oauth2_client_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The ID of the project that owns the service account."]
    pub project_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uniqueId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The unique, stable numeric ID for the service account. Each service account retains its unique ID even if you delete the service account. For example, if you delete a service account, then create a new service account with the same name, the new service account has a different unique ID than the deleted service account."]
    pub unique_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a service account key. A service account has two sets of key-pairs: user-managed, and system-managed. User-managed key-pairs can be created and deleted by users. Users are responsible for rotating these keys periodically to ensure security of their service accounts. Users retain the private key of these key-pairs, and Google retains ONLY the public key. System-managed keys are automatically rotated by Google, and are used for signing for a maximum of two weeks. The rotation process is probabilistic, and usage of the new key will gradually ramp up and down over the key's lifetime. If you cache the public key set for a service account, we recommend that you update the cache every 15 minutes. User-managed keys can be added and removed at any time, so it is important to update the cache frequently. For Google-managed keys, Google will publish a key at least 6 hours before it is first used for signing and will keep publishing it for at least 6 hours after it was last used for signing. Public keys for all service accounts are also published at the OAuth2 Service Account API."]
pub struct ServiceAccountKey {
    #[serde(rename = "keyAlgorithm")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the algorithm (and possibly key size) for the key."]
    pub key_algorithm: ::std::option::Option<ServiceAccountKeyKeyAlgorithmEnum>,
    #[serde(rename = "keyOrigin")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The key origin."]
    pub key_origin: ::std::option::Option<ServiceAccountKeyKeyOriginEnum>,
    #[serde(rename = "keyType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The key type."]
    pub key_type: ::std::option::Option<ServiceAccountKeyKeyTypeEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the service account key in the following format `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}/keys/{key}`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "privateKeyData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The private key data. Only provided in `CreateServiceAccountKey` responses. Make sure to keep the private key data secure because it allows for the assertion of the service account identity. When base64 decoded, the private key data can be used to authenticate with Google API client libraries and with gcloud auth activate-service-account."]
    pub private_key_data: ::std::option::Option<::std::string::String>,
    #[serde(rename = "privateKeyType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The output format for the private key. Only provided in `CreateServiceAccountKey` responses, not in `GetServiceAccountKey` or `ListServiceAccountKey` responses. Google never exposes system-managed private keys, and never retains user-managed private keys."]
    pub private_key_type: ::std::option::Option<ServiceAccountKeyPrivateKeyTypeEnum>,
    #[serde(rename = "publicKeyData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The public key data. Only provided in `GetServiceAccountKey` responses."]
    pub public_key_data: ::std::option::Option<::std::string::String>,
    #[serde(rename = "validAfterTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The key can be used after this timestamp."]
    pub valid_after_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "validBeforeTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The key can be used before this timestamp. For system-managed key pairs, this timestamp is the end time for the private key signing operation. The public key could still be used for verification for a few hours after this time."]
    pub valid_before_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Specifies the algorithm (and possibly key size) for the key."]
pub enum ServiceAccountKeyKeyAlgorithmEnum {
    #[serde(rename = "KEY_ALG_UNSPECIFIED")]
    #[doc = "An unspecified key algorithm."]
    KeyAlgUnspecified,
    #[serde(rename = "KEY_ALG_RSA_1024")]
    #[doc = "1k RSA Key."]
    KeyAlgRsa1024,
    #[serde(rename = "KEY_ALG_RSA_2048")]
    #[doc = "2k RSA Key."]
    KeyAlgRsa2048,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The key origin."]
pub enum ServiceAccountKeyKeyOriginEnum {
    #[serde(rename = "ORIGIN_UNSPECIFIED")]
    #[doc = "Unspecified key origin."]
    OriginUnspecified,
    #[serde(rename = "USER_PROVIDED")]
    #[doc = "Key is provided by user."]
    UserProvided,
    #[serde(rename = "GOOGLE_PROVIDED")]
    #[doc = "Key is provided by Google."]
    GoogleProvided,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The key type."]
pub enum ServiceAccountKeyKeyTypeEnum {
    #[serde(rename = "KEY_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified key type. The presence of this in the message will immediately result in an error."]
    KeyTypeUnspecified,
    #[serde(rename = "USER_MANAGED")]
    #[doc = "User-managed keys (managed and rotated by the user)."]
    UserManaged,
    #[serde(rename = "SYSTEM_MANAGED")]
    #[doc = "System-managed keys (managed and rotated by Google)."]
    SystemManaged,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The output format for the private key. Only provided in `CreateServiceAccountKey` responses, not in `GetServiceAccountKey` or `ListServiceAccountKey` responses. Google never exposes system-managed private keys, and never retains user-managed private keys."]
pub enum ServiceAccountKeyPrivateKeyTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "Unspecified. Equivalent to `TYPE_GOOGLE_CREDENTIALS_FILE`."]
    TypeUnspecified,
    #[serde(rename = "TYPE_PKCS12_FILE")]
    #[doc = "PKCS12 format. The password for the PKCS12 file is `notasecret`. For more information, see https://tools.ietf.org/html/rfc7292."]
    TypePkcs12File,
    #[serde(rename = "TYPE_GOOGLE_CREDENTIALS_FILE")]
    #[doc = "Google Credentials File format."]
    TypeGoogleCredentialsFile,
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
#[doc = "Deprecated. [Migrate to Service Account Credentials API](https://cloud.google.com/iam/help/credentials/migrate-api). The service account sign blob request."]
pub struct SignBlobRequest {
    #[serde(rename = "bytesToSign")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. [Migrate to Service Account Credentials API](https://cloud.google.com/iam/help/credentials/migrate-api). The bytes to sign."]
    pub bytes_to_sign: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Deprecated. [Migrate to Service Account Credentials API](https://cloud.google.com/iam/help/credentials/migrate-api). The service account sign blob response."]
pub struct SignBlobResponse {
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. [Migrate to Service Account Credentials API](https://cloud.google.com/iam/help/credentials/migrate-api). The id of the key used to sign the blob."]
    pub key_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "signature")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. [Migrate to Service Account Credentials API](https://cloud.google.com/iam/help/credentials/migrate-api). The signed blob."]
    pub signature: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Deprecated. [Migrate to Service Account Credentials API](https://cloud.google.com/iam/help/credentials/migrate-api). The service account sign JWT request."]
pub struct SignJwtRequest {
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Deprecated. [Migrate to Service Account Credentials API](https://cloud.google.com/iam/help/credentials/migrate-api). The JWT payload to sign. Must be a serialized JSON object that contains a JWT Claims Set. For example: `{\"sub\": \"user@example.com\", \"iat\": 313435}` If the JWT Claims Set contains an expiration time (`exp`) claim, it must be an integer timestamp that is not in the past and no more than 1 hour in the future. If the JWT Claims Set does not contain an expiration time (`exp`) claim, this claim is added automatically, with a timestamp that is 1 hour in the future."]
    pub payload: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Deprecated. [Migrate to Service Account Credentials API](https://cloud.google.com/iam/help/credentials/migrate-api). The service account sign JWT response."]
pub struct SignJwtResponse {
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. [Migrate to Service Account Credentials API](https://cloud.google.com/iam/help/credentials/migrate-api). The id of the key used to sign the JWT."]
    pub key_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "signedJwt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. [Migrate to Service Account Credentials API](https://cloud.google.com/iam/help/credentials/migrate-api). The signed JWT."]
    pub signed_jwt: ::std::option::Option<::std::string::String>,
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
#[doc = "The request to undelete an existing role."]
pub struct UndeleteRoleRequest {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Used to perform a consistent read-modify-write."]
    pub etag: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The service account undelete request."]
pub struct UndeleteServiceAccountRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UndeleteServiceAccountResponse {
    #[serde(rename = "restoredAccount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata for the restored service account."]
    pub restored_account: ::std::option::Option<::std::boxed::Box<ServiceAccount>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for UndeleteWorkloadIdentityPoolProvider."]
pub struct UndeleteWorkloadIdentityPoolProviderRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for UndeleteWorkloadIdentityPool."]
pub struct UndeleteWorkloadIdentityPoolRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The service account key upload request."]
pub struct UploadServiceAccountKeyRequest {
    #[serde(rename = "publicKeyData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A field that allows clients to upload their own public key. If set, use this public key data to create a service account key for given service account. Please note, the expected format for this field is X509_PEM."]
    pub public_key_data: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a collection of external workload identities. You can define IAM policies to grant these identities access to Google Cloud resources."]
pub struct WorkloadIdentityPool {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A description of the pool. Cannot exceed 256 characters."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "disabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the pool is disabled. You cannot use a disabled pool to exchange tokens, or use existing tokens to access resources. If the pool is re-enabled, existing tokens grant access again."]
    pub disabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A display name for the pool. Cannot exceed 32 characters."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the pool."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The state of the pool."]
    pub state: ::std::option::Option<WorkloadIdentityPoolStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The state of the pool."]
pub enum WorkloadIdentityPoolStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "State unspecified."]
    StateUnspecified,
    #[serde(rename = "ACTIVE")]
    #[doc = "The pool is active, and may be used in Google Cloud policies."]
    Active,
    #[serde(rename = "DELETED")]
    #[doc = "The pool is soft-deleted. Soft-deleted pools are permanently deleted after approximately 30 days. You can restore a soft-deleted pool using UndeleteWorkloadIdentityPool. You cannot reuse the ID of a soft-deleted pool until it is permanently deleted. While a pool is deleted, you cannot use it to exchange tokens, or use existing tokens to access resources. If the pool is undeleted, existing tokens grant access again."]
    Deleted,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A configuration for an external identity provider."]
pub struct WorkloadIdentityPoolProvider {
    #[serde(rename = "attributeCondition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[A Common Expression Language](https://opensource.google/projects/cel) expression, in plain text, to restrict what otherwise valid authentication credentials issued by the provider should not be accepted. The expression must output a boolean representing whether to allow the federation. The following keywords may be referenced in the expressions: * `assertion`: JSON representing the authentication credential issued by the provider. * `google`: The Google attributes mapped from the assertion in the `attribute_mappings`. * `attribute`: The custom attributes mapped from the assertion in the `attribute_mappings`. The maximum length of the attribute condition expression is 4096 characters. If unspecified, all valid authentication credential are accepted. The following example shows how to only allow credentials with a mapped `google.groups` value of `admins`: ``` \"'admins' in google.groups\" ```"]
    pub attribute_condition: ::std::option::Option<::std::string::String>,
    #[serde(rename = "attributeMapping")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maps attributes from authentication credentials issued by an external identity provider to Google Cloud attributes, such as `subject` and `segment`. Each key must be a string specifying the Google Cloud IAM attribute to map to. The following keys are supported: * `google.subject`: The principal IAM is authenticating. You can reference this value in IAM bindings. This is also the subject that appears in Cloud Logging logs. Cannot exceed 127 characters. * `google.groups`: Groups the external identity belongs to. You can grant groups access to resources using an IAM `principalSet` binding; access applies to all members of the group. You can also provide custom attributes by specifying `attribute.{custom_attribute}`, where `{custom_attribute}` is the name of the custom attribute to be mapped. You can define a maximum of 50 custom attributes. The maximum length of a mapped attribute key is 100 characters, and the key may only contain the characters [a-z0-9_]. You can reference these attributes in IAM policies to define fine-grained access for a workload to Google Cloud resources. For example: * `google.subject`: `principal://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/subject/{value}` * `google.groups`: `principalSet://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/group/{value}` * `attribute.{custom_attribute}`: `principalSet://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/attribute.{custom_attribute}/{value}` Each value must be a [Common Expression Language] (https://opensource.google/projects/cel) function that maps an identity provider credential to the normalized attribute specified by the corresponding map key. You can use the `assertion` keyword in the expression to access a JSON representation of the authentication credential issued by the provider. The maximum length of an attribute mapping expression is 2048 characters. When evaluated, the total size of all mapped attributes must not exceed 8KB. For AWS providers, if no attribute mapping is defined, the following default mapping applies: ``` { \"google.subject\":\"assertion.arn\", \"attribute.aws_role\": \"assertion.arn.contains('assumed-role')\" \" ? assertion.arn.extract('{account_arn}assumed-role/')\" \" + 'assumed-role/'\" \" + assertion.arn.extract('assumed-role/{role_name}/')\" \" : assertion.arn\", } ``` If any custom attribute mappings are defined, they must include a mapping to the `google.subject` attribute. For OIDC providers, you must supply a custom mapping, which must include the `google.subject` attribute. For example, the following maps the `sub` claim of the incoming credential to the `subject` attribute on a Google token: ``` {\"google.subject\": \"assertion.sub\"} ```"]
    pub attribute_mapping:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "aws")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An Amazon Web Services identity provider."]
    pub aws: ::std::option::Option<::std::boxed::Box<Aws>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A description for the provider. Cannot exceed 256 characters."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "disabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the provider is disabled. You cannot use a disabled provider to exchange tokens. However, existing tokens still grant access."]
    pub disabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A display name for the provider. Cannot exceed 32 characters."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the provider."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "oidc")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An OpenId Connect 1.0 identity provider."]
    pub oidc: ::std::option::Option<::std::boxed::Box<Oidc>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The state of the provider."]
    pub state: ::std::option::Option<WorkloadIdentityPoolProviderStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The state of the provider."]
pub enum WorkloadIdentityPoolProviderStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "State unspecified."]
    StateUnspecified,
    #[serde(rename = "ACTIVE")]
    #[doc = "The provider is active, and may be used to validate authentication credentials."]
    Active,
    #[serde(rename = "DELETED")]
    #[doc = "The provider is soft-deleted. Soft-deleted providers are permanently deleted after approximately 30 days. You can restore a soft-deleted provider using UndeleteWorkloadIdentityPoolProvider. You cannot reuse the ID of a soft-deleted provider until it is permanently deleted."]
    Deleted,
}
