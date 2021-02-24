#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies roles and/or permissions to analyze, to determine both the identities possessing them and the resources they control. If multiple values are specified, results will include identities and resources matching any of them. The total number of roles and permissions should be equal or less than 10."]
pub struct AccessSelector {
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The permissions to appear in result."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "roles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The roles to appear in result."]
    pub roles: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A response message for AssetService.AnalyzeIamPolicy."]
pub struct AnalyzeIamPolicyResponse {
    #[serde(rename = "fullyExplored")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents whether all entries in the main_analysis and service_account_impersonation_analysis have been fully explored to answer the query in the request."]
    pub fully_explored: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "mainAnalysis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The main analysis that matches the original request."]
    pub main_analysis: ::std::option::Option<::std::boxed::Box<IamPolicyAnalysis>>,
    #[serde(rename = "nonCriticalErrors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of non-critical errors happened during the request handling to explain why `fully_explored` is false, or empty if no error happened."]
    pub non_critical_errors: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudAssetV1p4beta1AnalysisState>>,
    >,
    #[serde(rename = "serviceAccountImpersonationAnalysis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The service account impersonation analysis if AnalyzeIamPolicyRequest.analyze_service_account_impersonation is enabled."]
    pub service_account_impersonation_analysis:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<IamPolicyAnalysis>>>,
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
#[doc = "A request message for AssetService.ExportIamPolicyAnalysis."]
pub struct ExportIamPolicyAnalysisRequest {
    #[serde(rename = "analysisQuery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The request query."]
    pub analysis_query: ::std::option::Option<::std::boxed::Box<IamPolicyAnalysisQuery>>,
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The request options."]
    pub options: ::std::option::Option<::std::boxed::Box<Options>>,
    #[serde(rename = "outputConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Output configuration indicating where the results will be output to."]
    pub output_config: ::std::option::Option<::std::boxed::Box<IamPolicyAnalysisOutputConfig>>,
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
#[doc = "A Cloud Storage location."]
pub struct GcsDestination {
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The uri of the Cloud Storage object. It's the same uri that is used by gsutil. For example: \"gs://bucket_name/object_name\". See [Quickstart: Using the gsutil tool] (https://cloud.google.com/storage/docs/quickstart-gsutil) for examples."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An IAM role or permission under analysis."]
pub struct GoogleCloudAssetV1p4beta1Access {
    #[serde(rename = "analysisState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The analysis state of this access."]
    pub analysis_state:
        ::std::option::Option<::std::boxed::Box<GoogleCloudAssetV1p4beta1AnalysisState>>,
    #[serde(rename = "permission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The permission."]
    pub permission: ::std::option::Option<::std::string::String>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The role."]
    pub role: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An access control list, derived from the above IAM policy binding, which contains a set of resources and accesses. May include one item from each set to compose an access control entry. NOTICE that there could be multiple access control lists for one IAM policy binding. The access control lists are created based on resource and access combinations. For example, assume we have the following cases in one IAM policy binding: - Permission P1 and P2 apply to resource R1 and R2; - Permission P3 applies to resource R2 and R3; This will result in the following access control lists: - AccessControlList 1: [R1, R2], [P1, P2] - AccessControlList 2: [R2, R3], [P3]"]
pub struct GoogleCloudAssetV1p4beta1AccessControlList {
    #[serde(rename = "accesses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The accesses that match one of the following conditions: - The access_selector, if it is specified in request; - Otherwise, access specifiers reachable from the policy binding's role."]
    pub accesses:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudAssetV1p4beta1Access>>>,
    #[serde(rename = "resourceEdges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource edges of the graph starting from the policy attached resource to any descendant resources. The Edge.source_node contains the full resource name of a parent resource and Edge.target_node contains the full resource name of a child resource. This field is present only if the output_resource_edges option is enabled in request."]
    pub resource_edges:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudAssetV1p4beta1Edge>>>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resources that match one of the following conditions: - The resource_selector, if it is specified in request; - Otherwise, resources reachable from the policy attached resource."]
    pub resources: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudAssetV1p4beta1Resource>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the detailed state of an entity under analysis, such as a resource, an identity or an access."]
pub struct GoogleCloudAssetV1p4beta1AnalysisState {
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The human-readable description of the cause of failure."]
    pub cause: ::std::option::Option<::std::string::String>,
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Google standard error code that best describes the state. For example: - OK means the analysis on this entity has been successfully finished; - PERMISSION_DENIED means an access denied error is encountered; - DEADLINE_EXCEEDED means the analysis on this entity hasn't been started in time;"]
    pub code: ::std::option::Option<GoogleCloudAssetV1p4beta1AnalysisStateCodeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The Google standard error code that best describes the state. For example: - OK means the analysis on this entity has been successfully finished; - PERMISSION_DENIED means an access denied error is encountered; - DEADLINE_EXCEEDED means the analysis on this entity hasn't been started in time;"]
pub enum GoogleCloudAssetV1p4beta1AnalysisStateCodeEnum {
    #[serde(rename = "OK")]
    #[doc = "Not an error; returned on success HTTP Mapping: 200 OK"]
    Ok,
    #[serde(rename = "CANCELLED")]
    #[doc = "The operation was cancelled, typically by the caller. HTTP Mapping: 499 Client Closed Request"]
    Cancelled,
    #[serde(rename = "UNKNOWN")]
    #[doc = "Unknown error. For example, this error may be returned when a `Status` value received from another address space belongs to an error space that is not known in this address space. Also errors raised by APIs that do not return enough error information may be converted to this error. HTTP Mapping: 500 Internal Server Error"]
    Unknown,
    #[serde(rename = "INVALID_ARGUMENT")]
    #[doc = "The client specified an invalid argument. Note that this differs from `FAILED_PRECONDITION`. `INVALID_ARGUMENT` indicates arguments that are problematic regardless of the state of the system (e.g., a malformed file name). HTTP Mapping: 400 Bad Request"]
    InvalidArgument,
    #[serde(rename = "DEADLINE_EXCEEDED")]
    #[doc = "The deadline expired before the operation could complete. For operations that change the state of the system, this error may be returned even if the operation has completed successfully. For example, a successful response from a server could have been delayed long enough for the deadline to expire. HTTP Mapping: 504 Gateway Timeout"]
    DeadlineExceeded,
    #[serde(rename = "NOT_FOUND")]
    #[doc = "Some requested entity (e.g., file or directory) was not found. Note to server developers: if a request is denied for an entire class of users, such as gradual feature rollout or undocumented allowlist, `NOT_FOUND` may be used. If a request is denied for some users within a class of users, such as user-based access control, `PERMISSION_DENIED` must be used. HTTP Mapping: 404 Not Found"]
    NotFound,
    #[serde(rename = "ALREADY_EXISTS")]
    #[doc = "The entity that a client attempted to create (e.g., file or directory) already exists. HTTP Mapping: 409 Conflict"]
    AlreadyExists,
    #[serde(rename = "PERMISSION_DENIED")]
    #[doc = "The caller does not have permission to execute the specified operation. `PERMISSION_DENIED` must not be used for rejections caused by exhausting some resource (use `RESOURCE_EXHAUSTED` instead for those errors). `PERMISSION_DENIED` must not be used if the caller can not be identified (use `UNAUTHENTICATED` instead for those errors). This error code does not imply the request is valid or the requested entity exists or satisfies other pre-conditions. HTTP Mapping: 403 Forbidden"]
    PermissionDenied,
    #[serde(rename = "UNAUTHENTICATED")]
    #[doc = "The request does not have valid authentication credentials for the operation. HTTP Mapping: 401 Unauthorized"]
    Unauthenticated,
    #[serde(rename = "RESOURCE_EXHAUSTED")]
    #[doc = "Some resource has been exhausted, perhaps a per-user quota, or perhaps the entire file system is out of space. HTTP Mapping: 429 Too Many Requests"]
    ResourceExhausted,
    #[serde(rename = "FAILED_PRECONDITION")]
    #[doc = "The operation was rejected because the system is not in a state required for the operation's execution. For example, the directory to be deleted is non-empty, an rmdir operation is applied to a non-directory, etc. Service implementors can use the following guidelines to decide between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`: (a) Use `UNAVAILABLE` if the client can retry just the failing call. (b) Use `ABORTED` if the client should retry at a higher level (e.g., when a client-specified test-and-set fails, indicating the client should restart a read-modify-write sequence). (c) Use `FAILED_PRECONDITION` if the client should not retry until the system state has been explicitly fixed. E.g., if an \"rmdir\" fails because the directory is non-empty, `FAILED_PRECONDITION` should be returned since the client should not retry unless the files are deleted from the directory. HTTP Mapping: 400 Bad Request"]
    FailedPrecondition,
    #[serde(rename = "ABORTED")]
    #[doc = "The operation was aborted, typically due to a concurrency issue such as a sequencer check failure or transaction abort. See the guidelines above for deciding between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`. HTTP Mapping: 409 Conflict"]
    Aborted,
    #[serde(rename = "OUT_OF_RANGE")]
    #[doc = "The operation was attempted past the valid range. E.g., seeking or reading past end-of-file. Unlike `INVALID_ARGUMENT`, this error indicates a problem that may be fixed if the system state changes. For example, a 32-bit file system will generate `INVALID_ARGUMENT` if asked to read at an offset that is not in the range [0,2^32-1], but it will generate `OUT_OF_RANGE` if asked to read from an offset past the current file size. There is a fair bit of overlap between `FAILED_PRECONDITION` and `OUT_OF_RANGE`. We recommend using `OUT_OF_RANGE` (the more specific error) when it applies so that callers who are iterating through a space can easily look for an `OUT_OF_RANGE` error to detect when they are done. HTTP Mapping: 400 Bad Request"]
    OutOfRange,
    #[serde(rename = "UNIMPLEMENTED")]
    #[doc = "The operation is not implemented or is not supported/enabled in this service. HTTP Mapping: 501 Not Implemented"]
    Unimplemented,
    #[serde(rename = "INTERNAL")]
    #[doc = "Internal errors. This means that some invariants expected by the underlying system have been broken. This error code is reserved for serious errors. HTTP Mapping: 500 Internal Server Error"]
    Internal,
    #[serde(rename = "UNAVAILABLE")]
    #[doc = "The service is currently unavailable. This is most likely a transient condition, which can be corrected by retrying with a backoff. Note that it is not always safe to retry non-idempotent operations. See the guidelines above for deciding between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`. HTTP Mapping: 503 Service Unavailable"]
    Unavailable,
    #[serde(rename = "DATA_LOSS")]
    #[doc = "Unrecoverable data loss or corruption. HTTP Mapping: 500 Internal Server Error"]
    DataLoss,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A directional edge."]
pub struct GoogleCloudAssetV1p4beta1Edge {
    #[serde(rename = "sourceNode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source node of the edge."]
    pub source_node: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetNode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target node of the edge."]
    pub target_node: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An identity under analysis."]
pub struct GoogleCloudAssetV1p4beta1Identity {
    #[serde(rename = "analysisState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The analysis state of this identity."]
    pub analysis_state:
        ::std::option::Option<::std::boxed::Box<GoogleCloudAssetV1p4beta1AnalysisState>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identity name in any form of members appear in [IAM policy binding](https://cloud.google.com/iam/reference/rest/v1/Binding), such as: - user:foo@google.com - group:group1@google.com - serviceAccount:s1@prj1.iam.gserviceaccount.com - projectOwner:some_project_id - domain:google.com - allUsers - etc."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudAssetV1p4beta1IdentityList {
    #[serde(rename = "groupEdges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Group identity edges of the graph starting from the binding's group members to any node of the identities. The Edge.source_node contains a group, such as \"group:parent@google.com\". The Edge.target_node contains a member of the group, such as \"group:child@google.com\" or \"user:foo@google.com\". This field is present only if the output_group_edges option is enabled in request."]
    pub group_edges:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudAssetV1p4beta1Edge>>>,
    #[serde(rename = "identities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only the identities that match one of the following conditions will be presented: - The identity_selector, if it is specified in request; - Otherwise, identities reachable from the policy binding's members."]
    pub identities: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudAssetV1p4beta1Identity>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Google Cloud resource under analysis."]
pub struct GoogleCloudAssetV1p4beta1Resource {
    #[serde(rename = "analysisState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The analysis state of this resource."]
    pub analysis_state:
        ::std::option::Option<::std::boxed::Box<GoogleCloudAssetV1p4beta1AnalysisState>>,
    #[serde(rename = "fullResourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The [full resource name](https://cloud.google.com/asset-inventory/docs/resource-name-format)"]
    pub full_resource_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An analysis message to group the query and results."]
pub struct IamPolicyAnalysis {
    #[serde(rename = "analysisQuery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The analysis query."]
    pub analysis_query: ::std::option::Option<::std::boxed::Box<IamPolicyAnalysisQuery>>,
    #[serde(rename = "analysisResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of IamPolicyAnalysisResult that matches the analysis query, or empty if no result is found."]
    pub analysis_results:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<IamPolicyAnalysisResult>>>,
    #[serde(rename = "fullyExplored")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents whether all entries in the analysis_results have been fully explored to answer the query."]
    pub fully_explored: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Output configuration for export IAM policy analysis destination."]
pub struct IamPolicyAnalysisOutputConfig {
    #[serde(rename = "gcsDestination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Destination on Cloud Storage."]
    pub gcs_destination: ::std::option::Option<::std::boxed::Box<GcsDestination>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "IAM policy analysis query message."]
pub struct IamPolicyAnalysisQuery {
    #[serde(rename = "accessSelector")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Specifies roles or permissions for analysis. This is optional."]
    pub access_selector: ::std::option::Option<::std::boxed::Box<AccessSelector>>,
    #[serde(rename = "identitySelector")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Specifies an identity for analysis."]
    pub identity_selector: ::std::option::Option<::std::boxed::Box<IdentitySelector>>,
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The relative name of the root asset. Only resources and IAM policies within the parent will be analyzed. This can only be an organization number (such as \"organizations/123\"), a folder number (such as \"folders/123\"), a project ID (such as \"projects/my-project-id\"), or a project number (such as \"projects/12345\"). To know how to get organization id, visit [here ](https://cloud.google.com/resource-manager/docs/creating-managing-organization#retrieving_your_organization_id). To know how to get folder or project id, visit [here ](https://cloud.google.com/resource-manager/docs/creating-managing-folders#viewing_or_listing_folders_and_projects)."]
    pub parent: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceSelector")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Specifies a resource for analysis."]
    pub resource_selector: ::std::option::Option<::std::boxed::Box<ResourceSelector>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "IAM Policy analysis result, consisting of one IAM policy binding and derived access control lists."]
pub struct IamPolicyAnalysisResult {
    #[serde(rename = "accessControlLists")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The access control lists derived from the iam_binding that match or potentially match resource and access selectors specified in the request."]
    pub access_control_lists: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudAssetV1p4beta1AccessControlList>>,
    >,
    #[serde(rename = "attachedResourceFullName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The [full resource name](https://cloud.google.com/asset-inventory/docs/resource-name-format) of the resource to which the iam_binding policy attaches."]
    pub attached_resource_full_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fullyExplored")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents whether all analyses on the iam_binding have successfully finished."]
    pub fully_explored: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "iamBinding")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Cloud IAM policy binding under analysis."]
    pub iam_binding: ::std::option::Option<::std::boxed::Box<Binding>>,
    #[serde(rename = "identityList")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identity list derived from members of the iam_binding that match or potentially match identity selector specified in the request."]
    pub identity_list:
        ::std::option::Option<::std::boxed::Box<GoogleCloudAssetV1p4beta1IdentityList>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies an identity for which to determine resource access, based on roles assigned either directly to them or to the groups they belong to, directly or indirectly."]
pub struct IdentitySelector {
    #[serde(rename = "identity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The identity appear in the form of members in [IAM policy binding](https://cloud.google.com/iam/reference/rest/v1/Binding). The examples of supported forms are: \"user:mike@example.com\", \"group:admins@example.com\", \"domain:google.com\", \"serviceAccount:my-project-id@appspot.gserviceaccount.com\". Notice that wildcard characters (such as * and ?) are not supported. You must give a specific identity."]
    pub identity: ::std::option::Option<::std::string::String>,
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
#[doc = "Contains request options."]
pub struct Options {
    #[serde(rename = "analyzeServiceAccountImpersonation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If true, the response will include access analysis from identities to resources via service account impersonation. This is a very expensive operation, because many derived queries will be executed. For example, if the request analyzes for which resources user A has permission P, and there's an IAM policy states user A has iam.serviceAccounts.getAccessToken permission to a service account SA, and there's another IAM policy states service account SA has permission P to a GCP folder F, then user A potentially has access to the GCP folder F. And those advanced analysis results will be included in AnalyzeIamPolicyResponse.service_account_impersonation_analysis. Another example, if the request analyzes for who has permission P to a GCP folder F, and there's an IAM policy states user A has iam.serviceAccounts.actAs permission to a service account SA, and there's another IAM policy states service account SA has permission P to the GCP folder F, then user A potentially has access to the GCP folder F. And those advanced analysis results will be included in AnalyzeIamPolicyResponse.service_account_impersonation_analysis. Default is false."]
    pub analyze_service_account_impersonation: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "expandGroups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If true, the identities section of the result will expand any Google groups appearing in an IAM policy binding. If identity_selector is specified, the identity in the result will be determined by the selector, and this flag will have no effect. Default is false."]
    pub expand_groups: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "expandResources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If true, the resource section of the result will expand any resource attached to an IAM policy to include resources lower in the resource hierarchy. For example, if the request analyzes for which resources user A has permission P, and the results include an IAM policy with P on a GCP folder, the results will also include resources in that folder with permission P. If resource_selector is specified, the resource section of the result will be determined by the selector, and this flag will have no effect. Default is false."]
    pub expand_resources: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "expandRoles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If true, the access section of result will expand any roles appearing in IAM policy bindings to include their permissions. If access_selector is specified, the access section of the result will be determined by the selector, and this flag will have no effect. Default is false."]
    pub expand_roles: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "outputGroupEdges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If true, the result will output group identity edges, starting from the binding's group members, to any expanded identities. Default is false."]
    pub output_group_edges: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "outputResourceEdges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If true, the result will output resource edges, starting from the policy attached resource, to any expanded resources. Default is false."]
    pub output_resource_edges: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies the resource to analyze for access policies, which may be set directly on the resource, or on ancestors such as organizations, folders or projects."]
pub struct ResourceSelector {
    #[serde(rename = "fullResourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The [full resource name](https://cloud.google.com/asset-inventory/docs/resource-name-format) of a resource of [supported resource types](https://cloud.google.com/asset-inventory/docs/supported-asset-types#analyzable_asset_types)."]
    pub full_resource_name: ::std::option::Option<::std::string::String>,
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
