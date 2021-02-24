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
pub struct ConfigFile {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The contents of the file."]
    pub content: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Deployment {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An optional user-provided description of the deployment."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Provides a fingerprint to use in requests to modify a deployment, such as `update()`, `stop()`, and `cancelPreview()` requests. A fingerprint is a randomly generated value that must be provided with `update()`, `stop()`, and `cancelPreview()` requests to perform optimistic locking. This ensures optimistic concurrency so that only one request happens at a time. The fingerprint is initially generated by Deployment Manager and changes after every request to modify data. To get the latest fingerprint value, perform a `get()` request to a deployment."]
    pub fingerprint: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "insertTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Creation timestamp in RFC3339 text format."]
    pub insert_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`."]
    pub labels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DeploymentLabelEntry>>>,
    #[serde(rename = "manifest")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. URL of the manifest representing the last manifest that was successfully deployed. If no manifest has been successfully deployed, this field will be absent."]
    pub manifest: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the resource; provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The Operation that most recently ran, or is currently running, on this deployment."]
    pub operation: ::std::option::Option<::std::boxed::Box<Operation>>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Server defined URL for the resource."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Input Only] The parameters that define your deployment, including the deployment configuration and relevant templates."]
    pub target: ::std::option::Option<::std::boxed::Box<TargetConfiguration>>,
    #[serde(rename = "update")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. If Deployment Manager is currently updating or previewing an update to this deployment, the updated configuration appears here."]
    pub update: ::std::option::Option<::std::boxed::Box<DeploymentUpdate>>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Update timestamp in RFC3339 text format."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Label object for Deployments"]
pub struct DeploymentLabelEntry {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Key of the label"]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Value of the label"]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeploymentUpdate {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. An optional user-provided description of the deployment after the current update has been applied."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Map of One Platform labels; provided by the client when the resource is created or updated. Specifically: Label keys must be between 1 and 63 characters long and must conform to the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?` Label values must be between 0 and 63 characters long and must conform to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`."]
    pub labels:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DeploymentUpdateLabelEntry>>>,
    #[serde(rename = "manifest")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. URL of the manifest representing the update configuration of this deployment."]
    pub manifest: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Label object for DeploymentUpdate"]
pub struct DeploymentUpdateLabelEntry {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Key of the label"]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Value of the label"]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeploymentsCancelPreviewRequest {
    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies a fingerprint for `cancelPreview()` requests. A fingerprint is a randomly generated value that must be provided in `cancelPreview()` requests to perform optimistic locking. This ensures optimistic concurrency so that the deployment does not have conflicting requests (e.g. if someone attempts to make a new update request while another user attempts to cancel a preview, this would prevent one of the requests). The fingerprint is initially generated by Deployment Manager and changes after every request to modify a deployment. To get the latest fingerprint value, perform a `get()` request on the deployment."]
    pub fingerprint: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A response containing a partial list of deployments and a page token used to build the next request if the request has been truncated."]
pub struct DeploymentsListResponse {
    #[serde(rename = "deployments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The deployments contained in this response."]
    pub deployments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Deployment>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A token used to continue a truncated list request."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeploymentsStopRequest {
    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies a fingerprint for `stop()` requests. A fingerprint is a randomly generated value that must be provided in `stop()` requests to perform optimistic locking. This ensures optimistic concurrency so that the deployment does not have conflicting requests (e.g. if someone attempts to make a new update request while another user attempts to stop an ongoing update request, this would prevent a collision). The fingerprint is initially generated by Deployment Manager and changes after every request to modify a deployment. To get the latest fingerprint value, perform a `get()` request on the deployment."]
    pub fingerprint: ::std::option::Option<::std::string::String>,
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
pub struct GlobalSetPolicyRequest {
    #[serde(rename = "bindings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Flatten Policy to create a backward compatible wire-format. Deprecated. Use 'policy' to specify bindings."]
    pub bindings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Binding>>>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Flatten Policy to create a backward compatible wire-format. Deprecated. Use 'policy' to specify the etag."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "REQUIRED: The complete policy to be applied to the 'resource'. The size of the policy is limited to a few 10s of KB. An empty policy is in general a valid policy but certain services (like Projects) might reject them."]
    pub policy: ::std::option::Option<::std::boxed::Box<Policy>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImportFile {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The contents of the file."]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the file."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The YAML configuration for this manifest."]
    pub config: ::std::option::Option<::std::boxed::Box<ConfigFile>>,
    #[serde(rename = "expandedConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The fully-expanded configuration file, including any templates and references."]
    pub expanded_config: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imports")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The imported files for this manifest."]
    pub imports: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ImportFile>>>,
    #[serde(rename = "insertTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Creation timestamp in RFC3339 text format."]
    pub insert_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "layout")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The YAML layout for this manifest."]
    pub layout: ::std::option::Option<::std::string::String>,
    #[serde(rename = "manifestSizeBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The computed size of the fully expanded manifest."]
    pub manifest_size_bytes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "manifestSizeLimitBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The size limit for expanded manifests in the project."]
    pub manifest_size_limit_bytes: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The name of the manifest."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Self link for the manifest."]
    pub self_link: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A response containing a partial list of manifests and a page token used to build the next request if the request has been truncated."]
pub struct ManifestsListResponse {
    #[serde(rename = "manifests")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Manifests contained in this list response."]
    pub manifests: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Manifest>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A token used to continue a truncated list request."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an Operation resource. Google Compute Engine has three Operation resources: * [Global](/compute/docs/reference/rest/{$api_version}/globalOperations) * [Regional](/compute/docs/reference/rest/{$api_version}/regionOperations) * [Zonal](/compute/docs/reference/rest/{$api_version}/zoneOperations) You can use an operation resource to manage asynchronous API requests. For more information, read Handling API responses. Operations can be global, regional or zonal. - For global operations, use the `globalOperations` resource. - For regional operations, use the `regionOperations` resource. - For zonal operations, use the `zonalOperations` resource. For more information, read Global, Regional, and Zonal Resources."]
pub struct Operation {
    #[serde(rename = "clientOperationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] The value of `requestId` if you provided it in the request. Not present otherwise."]
    pub client_operation_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creationTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Deprecated] This field is deprecated."]
    pub creation_timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] A textual description of the operation, which is set when the operation is created."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] The time that this operation was completed. This value is in RFC3339 text format."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] If errors are generated during processing of the operation, this field will be populated."]
    pub error: ::std::option::Option<OperationError>,
    #[serde(rename = "httpErrorMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] If the operation fails, this field contains the HTTP error message that was returned, such as `NOT FOUND`."]
    pub http_error_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "httpErrorStatusCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] If the operation fails, this field contains the HTTP error status code that was returned. For example, a `404` means the resource was not found."]
    pub http_error_status_code: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] The unique identifier for the operation. This identifier is defined by the server."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "insertTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] The time that this operation was requested. This value is in RFC3339 text format."]
    pub insert_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "operation_defaults :: kind")]
    #[doc = "[Output Only] Type of the resource. Always `compute#operation` for Operation resources."]
    pub kind: ::std::string::String,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] Name of the operation."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] An ID that represents a group of operations, such as when a group of operations results from a `bulkInsert` API request."]
    pub operation_group_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] The type of operation, such as `insert`, `update`, or `delete`, and so on."]
    pub operation_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] An optional progress indicator that ranges from 0 to 100. There is no requirement that this be linear or support any granularity of operations. This should not be used to guess when the operation will be complete. This number should monotonically increase as the operation progresses."]
    pub progress: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] The URL of the region where the operation resides. Only applicable when performing regional operations."]
    pub region: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] Server-defined URL for the resource."]
    pub self_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] The time that this operation was started by the server. This value is in RFC3339 text format."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] The status of the operation, which can be one of the following: `PENDING`, `RUNNING`, or `DONE`."]
    pub status: ::std::option::Option<OperationStatusEnum>,
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] An optional textual description of the current status of the operation."]
    pub status_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] The unique target ID, which identifies a specific incarnation of the target resource."]
    pub target_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] The URL of the resource that the operation modifies. For operations related to creating a snapshot, this points to the persistent disk that the snapshot was created from."]
    pub target_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] User who requested the operation, for example: `user@example.com`."]
    pub user: ::std::option::Option<::std::string::String>,
    #[serde(rename = "warnings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] If warning messages are generated during processing of the operation, this field will be populated."]
    pub warnings: ::std::option::Option<::std::vec::Vec<OperationWarnings>>,
    #[serde(rename = "zone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] The URL of the zone where the operation resides. Only applicable when performing per-zone operations."]
    pub zone: ::std::option::Option<::std::string::String>,
}
mod operation_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("deploymentmanager#operation")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "[Output Only] If errors are generated during processing of the operation, this field will be populated."]
pub struct OperationError {
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] The array of errors encountered while processing this operation."]
    pub errors: ::std::option::Option<::std::vec::Vec<OperationErrorErrors>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperationErrorErrors {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] The error type identifier for this error."]
    pub code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] Indicates the field in the request that caused the error. This property is optional."]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] An optional, human-readable error message."]
    pub message: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "[Output Only] The status of the operation, which can be one of the following: `PENDING`, `RUNNING`, or `DONE`."]
pub enum OperationStatusEnum {
    #[serde(rename = "PENDING")]
    #[doc = ""]
    Pending,
    #[serde(rename = "RUNNING")]
    #[doc = ""]
    Running,
    #[serde(rename = "DONE")]
    #[doc = ""]
    Done,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperationWarnings {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] A warning code, if applicable. For example, Compute Engine returns NO_RESULTS_ON_PAGE if there are no results in the response."]
    pub code: ::std::option::Option<OperationWarningsCodeEnum>,
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] Metadata about this warning in key: value format. For example: \"data\": [ { \"key\": \"scope\", \"value\": \"zones/us-east1-d\" } "]
    pub data: ::std::option::Option<::std::vec::Vec<OperationWarningsData>>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] A human-readable description of the warning code."]
    pub message: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "[Output Only] A warning code, if applicable. For example, Compute Engine returns NO_RESULTS_ON_PAGE if there are no results in the response."]
pub enum OperationWarningsCodeEnum {
    #[serde(rename = "DEPRECATED_RESOURCE_USED")]
    #[doc = "A link to a deprecated resource was created."]
    DeprecatedResourceUsed,
    #[serde(rename = "NO_RESULTS_ON_PAGE")]
    #[doc = "No results are present on a particular list page."]
    NoResultsOnPage,
    #[serde(rename = "UNREACHABLE")]
    #[doc = "A given scope cannot be reached."]
    Unreachable,
    #[serde(rename = "NEXT_HOP_ADDRESS_NOT_ASSIGNED")]
    #[doc = "The route's nextHopIp address is not assigned to an instance on the network."]
    NextHopAddressNotAssigned,
    #[serde(rename = "NEXT_HOP_INSTANCE_NOT_FOUND")]
    #[doc = "The route's nextHopInstance URL refers to an instance that does not exist."]
    NextHopInstanceNotFound,
    #[serde(rename = "NEXT_HOP_INSTANCE_NOT_ON_NETWORK")]
    #[doc = "The route's nextHopInstance URL refers to an instance that is not on the same network as the route."]
    NextHopInstanceNotOnNetwork,
    #[serde(rename = "NEXT_HOP_CANNOT_IP_FORWARD")]
    #[doc = "The route's next hop instance cannot ip forward."]
    NextHopCannotIpForward,
    #[serde(rename = "NEXT_HOP_NOT_RUNNING")]
    #[doc = "The route's next hop instance does not have a status of RUNNING."]
    NextHopNotRunning,
    #[serde(rename = "INJECTED_KERNELS_DEPRECATED")]
    #[doc = "The operation involved use of an injected kernel, which is deprecated."]
    InjectedKernelsDeprecated,
    #[serde(rename = "REQUIRED_TOS_AGREEMENT")]
    #[doc = "The user attempted to use a resource that requires a TOS they have not accepted."]
    RequiredTosAgreement,
    #[serde(rename = "DISK_SIZE_LARGER_THAN_IMAGE_SIZE")]
    #[doc = "The user created a boot disk that is larger than image size."]
    DiskSizeLargerThanImageSize,
    #[serde(rename = "RESOURCE_NOT_DELETED")]
    #[doc = "One or more of the resources set to auto-delete could not be deleted because they were in use."]
    ResourceNotDeleted,
    #[serde(rename = "SINGLE_INSTANCE_PROPERTY_TEMPLATE")]
    #[doc = "Instance template used in instance group manager is valid as such, but its application does not make a lot of sense, because it allows only single instance in instance group."]
    SingleInstancePropertyTemplate,
    #[serde(rename = "NOT_CRITICAL_ERROR")]
    #[doc = "Error which is not critical. We decided to continue the process despite the mentioned error."]
    NotCriticalError,
    #[serde(rename = "CLEANUP_FAILED")]
    #[doc = "Warning about failed cleanup of transient changes made by a failed operation."]
    CleanupFailed,
    #[serde(rename = "FIELD_VALUE_OVERRIDEN")]
    #[doc = "Warning that value of a field has been overridden. Deprecated unused field."]
    FieldValueOverriden,
    #[serde(rename = "RESOURCE_IN_USE_BY_OTHER_RESOURCE_WARNING")]
    #[doc = "Warning that a resource is in use."]
    ResourceInUseByOtherResourceWarning,
    #[serde(rename = "MISSING_TYPE_DEPENDENCY")]
    #[doc = "A resource depends on a missing type"]
    MissingTypeDependency,
    #[serde(rename = "EXTERNAL_API_WARNING")]
    #[doc = "Warning that is present in an external api call"]
    ExternalApiWarning,
    #[serde(rename = "SCHEMA_VALIDATION_IGNORED")]
    #[doc = "When a resource schema validation is ignored."]
    SchemaValidationIgnored,
    #[serde(rename = "UNDECLARED_PROPERTIES")]
    #[doc = "When undeclared properties in the schema are present"]
    UndeclaredProperties,
    #[serde(rename = "EXPERIMENTAL_TYPE_USED")]
    #[doc = "When deploying and at least one of the resources has a type marked as experimental"]
    ExperimentalTypeUsed,
    #[serde(rename = "DEPRECATED_TYPE_USED")]
    #[doc = "When deploying and at least one of the resources has a type marked as deprecated"]
    DeprecatedTypeUsed,
    #[serde(rename = "PARTIAL_SUCCESS")]
    #[doc = "Success is reported, but some results may be missing due to errors"]
    PartialSuccess,
    #[serde(rename = "LARGE_DEPLOYMENT_WARNING")]
    #[doc = "When deploying a deployment with a exceedingly large number of resources"]
    LargeDeploymentWarning,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperationWarningsData {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] A key that provides more detail on the warning being returned. For example, for warnings where there are no results in a list request for a particular zone, this key might be scope and the key value might be the zone name. Other examples might be a key indicating a deprecated resource and a suggested replacement, or a warning about invalid network settings (for example, if an instance attempts to perform IP forwarding but is not enabled for IP forwarding)."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] A warning data value corresponding to the key."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A response containing a partial list of operations and a page token used to build the next request if the request has been truncated."]
pub struct OperationsListResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A token used to continue a truncated list request."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Operations contained in this list response."]
    pub operations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Operation>>>,
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
pub struct Resource {
    #[serde(rename = "accessControl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Access Control Policy set on this resource."]
    pub access_control: ::std::option::Option<::std::boxed::Box<ResourceAccessControl>>,
    #[serde(rename = "finalProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The evaluated properties of the resource with references expanded. Returned as serialized YAML."]
    pub final_properties: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "insertTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Creation timestamp in RFC3339 text format."]
    pub insert_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "manifest")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. URL of the manifest representing the current configuration of this resource."]
    pub manifest: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The name of the resource as it appears in the YAML config."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The current properties of the resource before any references have been filled in. Returned as serialized YAML."]
    pub properties: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of the resource, for example `compute.v1.instance`, or `cloudfunctions.v1beta1.function`."]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "update")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. If Deployment Manager is currently updating or previewing an update to this resource, the updated configuration appears here."]
    pub update: ::std::option::Option<::std::boxed::Box<ResourceUpdate>>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Update timestamp in RFC3339 text format."]
    pub update_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The URL of the actual resource."]
    pub url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "warnings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. If warning messages are generated during processing of this resource, this field will be populated."]
    pub warnings: ::std::option::Option<::std::vec::Vec<ResourceWarnings>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceWarnings {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] A warning code, if applicable. For example, Compute Engine returns NO_RESULTS_ON_PAGE if there are no results in the response."]
    pub code: ::std::option::Option<ResourceWarningsCodeEnum>,
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] Metadata about this warning in key: value format. For example: \"data\": [ { \"key\": \"scope\", \"value\": \"zones/us-east1-d\" } "]
    pub data: ::std::option::Option<::std::vec::Vec<ResourceWarningsData>>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] A human-readable description of the warning code."]
    pub message: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "[Output Only] A warning code, if applicable. For example, Compute Engine returns NO_RESULTS_ON_PAGE if there are no results in the response."]
pub enum ResourceWarningsCodeEnum {
    #[serde(rename = "DEPRECATED_RESOURCE_USED")]
    #[doc = "A link to a deprecated resource was created."]
    DeprecatedResourceUsed,
    #[serde(rename = "NO_RESULTS_ON_PAGE")]
    #[doc = "No results are present on a particular list page."]
    NoResultsOnPage,
    #[serde(rename = "UNREACHABLE")]
    #[doc = "A given scope cannot be reached."]
    Unreachable,
    #[serde(rename = "NEXT_HOP_ADDRESS_NOT_ASSIGNED")]
    #[doc = "The route's nextHopIp address is not assigned to an instance on the network."]
    NextHopAddressNotAssigned,
    #[serde(rename = "NEXT_HOP_INSTANCE_NOT_FOUND")]
    #[doc = "The route's nextHopInstance URL refers to an instance that does not exist."]
    NextHopInstanceNotFound,
    #[serde(rename = "NEXT_HOP_INSTANCE_NOT_ON_NETWORK")]
    #[doc = "The route's nextHopInstance URL refers to an instance that is not on the same network as the route."]
    NextHopInstanceNotOnNetwork,
    #[serde(rename = "NEXT_HOP_CANNOT_IP_FORWARD")]
    #[doc = "The route's next hop instance cannot ip forward."]
    NextHopCannotIpForward,
    #[serde(rename = "NEXT_HOP_NOT_RUNNING")]
    #[doc = "The route's next hop instance does not have a status of RUNNING."]
    NextHopNotRunning,
    #[serde(rename = "INJECTED_KERNELS_DEPRECATED")]
    #[doc = "The operation involved use of an injected kernel, which is deprecated."]
    InjectedKernelsDeprecated,
    #[serde(rename = "REQUIRED_TOS_AGREEMENT")]
    #[doc = "The user attempted to use a resource that requires a TOS they have not accepted."]
    RequiredTosAgreement,
    #[serde(rename = "DISK_SIZE_LARGER_THAN_IMAGE_SIZE")]
    #[doc = "The user created a boot disk that is larger than image size."]
    DiskSizeLargerThanImageSize,
    #[serde(rename = "RESOURCE_NOT_DELETED")]
    #[doc = "One or more of the resources set to auto-delete could not be deleted because they were in use."]
    ResourceNotDeleted,
    #[serde(rename = "SINGLE_INSTANCE_PROPERTY_TEMPLATE")]
    #[doc = "Instance template used in instance group manager is valid as such, but its application does not make a lot of sense, because it allows only single instance in instance group."]
    SingleInstancePropertyTemplate,
    #[serde(rename = "NOT_CRITICAL_ERROR")]
    #[doc = "Error which is not critical. We decided to continue the process despite the mentioned error."]
    NotCriticalError,
    #[serde(rename = "CLEANUP_FAILED")]
    #[doc = "Warning about failed cleanup of transient changes made by a failed operation."]
    CleanupFailed,
    #[serde(rename = "FIELD_VALUE_OVERRIDEN")]
    #[doc = "Warning that value of a field has been overridden. Deprecated unused field."]
    FieldValueOverriden,
    #[serde(rename = "RESOURCE_IN_USE_BY_OTHER_RESOURCE_WARNING")]
    #[doc = "Warning that a resource is in use."]
    ResourceInUseByOtherResourceWarning,
    #[serde(rename = "MISSING_TYPE_DEPENDENCY")]
    #[doc = "A resource depends on a missing type"]
    MissingTypeDependency,
    #[serde(rename = "EXTERNAL_API_WARNING")]
    #[doc = "Warning that is present in an external api call"]
    ExternalApiWarning,
    #[serde(rename = "SCHEMA_VALIDATION_IGNORED")]
    #[doc = "When a resource schema validation is ignored."]
    SchemaValidationIgnored,
    #[serde(rename = "UNDECLARED_PROPERTIES")]
    #[doc = "When undeclared properties in the schema are present"]
    UndeclaredProperties,
    #[serde(rename = "EXPERIMENTAL_TYPE_USED")]
    #[doc = "When deploying and at least one of the resources has a type marked as experimental"]
    ExperimentalTypeUsed,
    #[serde(rename = "DEPRECATED_TYPE_USED")]
    #[doc = "When deploying and at least one of the resources has a type marked as deprecated"]
    DeprecatedTypeUsed,
    #[serde(rename = "PARTIAL_SUCCESS")]
    #[doc = "Success is reported, but some results may be missing due to errors"]
    PartialSuccess,
    #[serde(rename = "LARGE_DEPLOYMENT_WARNING")]
    #[doc = "When deploying a deployment with a exceedingly large number of resources"]
    LargeDeploymentWarning,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceWarningsData {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] A key that provides more detail on the warning being returned. For example, for warnings where there are no results in a list request for a particular zone, this key might be scope and the key value might be the zone name. Other examples might be a key indicating a deprecated resource and a suggested replacement, or a warning about invalid network settings (for example, if an instance attempts to perform IP forwarding but is not enabled for IP forwarding)."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] A warning data value corresponding to the key."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The access controls set on the resource."]
pub struct ResourceAccessControl {
    #[serde(rename = "gcpIamPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The GCP IAM Policy to set on the resource."]
    pub gcp_iam_policy: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceUpdate {
    #[serde(rename = "accessControl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Access Control Policy to set on this resource after updating the resource itself."]
    pub access_control: ::std::option::Option<::std::boxed::Box<ResourceAccessControl>>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. If errors are generated during update of the resource, this field will be populated."]
    pub error: ::std::option::Option<ResourceUpdateError>,
    #[serde(rename = "finalProperties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The expanded properties of the resource with reference values expanded. Returned as serialized YAML."]
    pub final_properties: ::std::option::Option<::std::string::String>,
    #[serde(rename = "intent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The intent of the resource: `PREVIEW`, `UPDATE`, or `CANCEL`."]
    pub intent: ::std::option::Option<ResourceUpdateIntentEnum>,
    #[serde(rename = "manifest")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. URL of the manifest representing the update configuration of this resource."]
    pub manifest: ::std::option::Option<::std::string::String>,
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The set of updated properties for this resource, before references are expanded. Returned as serialized YAML."]
    pub properties: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The state of the resource."]
    pub state: ::std::option::Option<ResourceUpdateStateEnum>,
    #[serde(rename = "warnings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. If warning messages are generated during processing of this resource, this field will be populated."]
    pub warnings: ::std::option::Option<::std::vec::Vec<ResourceUpdateWarnings>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Output only. If errors are generated during update of the resource, this field will be populated."]
pub struct ResourceUpdateError {
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] The array of errors encountered while processing this operation."]
    pub errors: ::std::option::Option<::std::vec::Vec<ResourceUpdateErrorErrors>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceUpdateErrorErrors {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] The error type identifier for this error."]
    pub code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] Indicates the field in the request that caused the error. This property is optional."]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] An optional, human-readable error message."]
    pub message: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The intent of the resource: `PREVIEW`, `UPDATE`, or `CANCEL`."]
pub enum ResourceUpdateIntentEnum {
    #[serde(rename = "CREATE_OR_ACQUIRE")]
    #[doc = "The resource is scheduled to be created, or if it already exists, acquired."]
    CreateOrAcquire,
    #[serde(rename = "DELETE")]
    #[doc = "The resource is scheduled to be deleted."]
    Delete,
    #[serde(rename = "ACQUIRE")]
    #[doc = "The resource is scheduled to be acquired."]
    Acquire,
    #[serde(rename = "UPDATE")]
    #[doc = "The resource is scheduled to be updated via the UPDATE method."]
    Update,
    #[serde(rename = "ABANDON")]
    #[doc = "The resource is scheduled to be abandoned."]
    Abandon,
    #[serde(rename = "CREATE")]
    #[doc = "The resource is scheduled to be created."]
    Create,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The state of the resource."]
pub enum ResourceUpdateStateEnum {
    #[serde(rename = "PENDING")]
    #[doc = "There are changes pending for this resource."]
    Pending,
    #[serde(rename = "IN_PROGRESS")]
    #[doc = "The service is executing changes on the resource."]
    InProgress,
    #[serde(rename = "IN_PREVIEW")]
    #[doc = "The service is previewing changes on the resource."]
    InPreview,
    #[serde(rename = "FAILED")]
    #[doc = "The service has failed to change the resource."]
    Failed,
    #[serde(rename = "ABORTED")]
    #[doc = "The service has aborted trying to change the resource."]
    Aborted,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceUpdateWarnings {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] A warning code, if applicable. For example, Compute Engine returns NO_RESULTS_ON_PAGE if there are no results in the response."]
    pub code: ::std::option::Option<ResourceUpdateWarningsCodeEnum>,
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] Metadata about this warning in key: value format. For example: \"data\": [ { \"key\": \"scope\", \"value\": \"zones/us-east1-d\" } "]
    pub data: ::std::option::Option<::std::vec::Vec<ResourceUpdateWarningsData>>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] A human-readable description of the warning code."]
    pub message: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "[Output Only] A warning code, if applicable. For example, Compute Engine returns NO_RESULTS_ON_PAGE if there are no results in the response."]
pub enum ResourceUpdateWarningsCodeEnum {
    #[serde(rename = "DEPRECATED_RESOURCE_USED")]
    #[doc = "A link to a deprecated resource was created."]
    DeprecatedResourceUsed,
    #[serde(rename = "NO_RESULTS_ON_PAGE")]
    #[doc = "No results are present on a particular list page."]
    NoResultsOnPage,
    #[serde(rename = "UNREACHABLE")]
    #[doc = "A given scope cannot be reached."]
    Unreachable,
    #[serde(rename = "NEXT_HOP_ADDRESS_NOT_ASSIGNED")]
    #[doc = "The route's nextHopIp address is not assigned to an instance on the network."]
    NextHopAddressNotAssigned,
    #[serde(rename = "NEXT_HOP_INSTANCE_NOT_FOUND")]
    #[doc = "The route's nextHopInstance URL refers to an instance that does not exist."]
    NextHopInstanceNotFound,
    #[serde(rename = "NEXT_HOP_INSTANCE_NOT_ON_NETWORK")]
    #[doc = "The route's nextHopInstance URL refers to an instance that is not on the same network as the route."]
    NextHopInstanceNotOnNetwork,
    #[serde(rename = "NEXT_HOP_CANNOT_IP_FORWARD")]
    #[doc = "The route's next hop instance cannot ip forward."]
    NextHopCannotIpForward,
    #[serde(rename = "NEXT_HOP_NOT_RUNNING")]
    #[doc = "The route's next hop instance does not have a status of RUNNING."]
    NextHopNotRunning,
    #[serde(rename = "INJECTED_KERNELS_DEPRECATED")]
    #[doc = "The operation involved use of an injected kernel, which is deprecated."]
    InjectedKernelsDeprecated,
    #[serde(rename = "REQUIRED_TOS_AGREEMENT")]
    #[doc = "The user attempted to use a resource that requires a TOS they have not accepted."]
    RequiredTosAgreement,
    #[serde(rename = "DISK_SIZE_LARGER_THAN_IMAGE_SIZE")]
    #[doc = "The user created a boot disk that is larger than image size."]
    DiskSizeLargerThanImageSize,
    #[serde(rename = "RESOURCE_NOT_DELETED")]
    #[doc = "One or more of the resources set to auto-delete could not be deleted because they were in use."]
    ResourceNotDeleted,
    #[serde(rename = "SINGLE_INSTANCE_PROPERTY_TEMPLATE")]
    #[doc = "Instance template used in instance group manager is valid as such, but its application does not make a lot of sense, because it allows only single instance in instance group."]
    SingleInstancePropertyTemplate,
    #[serde(rename = "NOT_CRITICAL_ERROR")]
    #[doc = "Error which is not critical. We decided to continue the process despite the mentioned error."]
    NotCriticalError,
    #[serde(rename = "CLEANUP_FAILED")]
    #[doc = "Warning about failed cleanup of transient changes made by a failed operation."]
    CleanupFailed,
    #[serde(rename = "FIELD_VALUE_OVERRIDEN")]
    #[doc = "Warning that value of a field has been overridden. Deprecated unused field."]
    FieldValueOverriden,
    #[serde(rename = "RESOURCE_IN_USE_BY_OTHER_RESOURCE_WARNING")]
    #[doc = "Warning that a resource is in use."]
    ResourceInUseByOtherResourceWarning,
    #[serde(rename = "MISSING_TYPE_DEPENDENCY")]
    #[doc = "A resource depends on a missing type"]
    MissingTypeDependency,
    #[serde(rename = "EXTERNAL_API_WARNING")]
    #[doc = "Warning that is present in an external api call"]
    ExternalApiWarning,
    #[serde(rename = "SCHEMA_VALIDATION_IGNORED")]
    #[doc = "When a resource schema validation is ignored."]
    SchemaValidationIgnored,
    #[serde(rename = "UNDECLARED_PROPERTIES")]
    #[doc = "When undeclared properties in the schema are present"]
    UndeclaredProperties,
    #[serde(rename = "EXPERIMENTAL_TYPE_USED")]
    #[doc = "When deploying and at least one of the resources has a type marked as experimental"]
    ExperimentalTypeUsed,
    #[serde(rename = "DEPRECATED_TYPE_USED")]
    #[doc = "When deploying and at least one of the resources has a type marked as deprecated"]
    DeprecatedTypeUsed,
    #[serde(rename = "PARTIAL_SUCCESS")]
    #[doc = "Success is reported, but some results may be missing due to errors"]
    PartialSuccess,
    #[serde(rename = "LARGE_DEPLOYMENT_WARNING")]
    #[doc = "When deploying a deployment with a exceedingly large number of resources"]
    LargeDeploymentWarning,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceUpdateWarningsData {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] A key that provides more detail on the warning being returned. For example, for warnings where there are no results in a list request for a particular zone, this key might be scope and the key value might be the zone name. Other examples might be a key indicating a deprecated resource and a suggested replacement, or a warning about invalid network settings (for example, if an instance attempts to perform IP forwarding but is not enabled for IP forwarding)."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Output Only] A warning data value corresponding to the key."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A response containing a partial list of resources and a page token used to build the next request if the request has been truncated."]
pub struct ResourcesListResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token used to continue a truncated list request."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resources contained in this list response."]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Resource>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetConfiguration {
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration to use for this deployment."]
    pub config: ::std::option::Option<::std::boxed::Box<ConfigFile>>,
    #[serde(rename = "imports")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies any files to import for this configuration. This can be used to import templates or other files. For example, you might import a text file in order to use the file in a template."]
    pub imports: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ImportFile>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestPermissionsRequest {
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of permissions to check for the 'resource'. Permissions with wildcards (such as '*' or 'storage.*') are not allowed."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestPermissionsResponse {
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A subset of `TestPermissionsRequest.permissions` that the caller is allowed."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A resource type supported by Deployment Manager."]
pub struct Type {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "insertTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Creation timestamp in RFC3339 text format."]
    pub insert_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the type."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The Operation that most recently ran, or is currently running, on this type."]
    pub operation: ::std::option::Option<::std::boxed::Box<Operation>>,
    #[serde(rename = "selfLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Server defined URL for the resource."]
    pub self_link: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A response that returns all Types supported by Deployment Manager"]
pub struct TypesListResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token used to continue a truncated list request."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "types")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A list of resource types supported by Deployment Manager."]
    pub types: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Type>>>,
}
