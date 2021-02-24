#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about the member, resource, and permission to check."]
pub struct GoogleCloudPolicytroubleshooterV1betaAccessTuple {
    #[serde(rename = "fullResourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The full resource name that identifies the resource. For example, `//compute.googleapis.com/projects/my-project/zones/us-central1-a/instances/my-instance`. For examples of full resource names for Google Cloud services, see https://cloud.google.com/iam/help/troubleshooter/full-resource-names."]
    pub full_resource_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "permission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The IAM permission to check for the specified member and resource. For a complete list of IAM permissions, see https://cloud.google.com/iam/help/permissions/reference. For a complete list of predefined IAM roles and the permissions in each role, see https://cloud.google.com/iam/help/roles/reference."]
    pub permission: ::std::option::Option<::std::string::String>,
    #[serde(rename = "principal")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The member, or principal, whose access you want to check, in the form of the email address that represents that member. For example, `alice@example.com` or `my-service-account@my-project.iam.gserviceaccount.com`. The member must be a Google Account or a service account. Other types of members are not supported."]
    pub principal: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details about how a binding in a policy affects a member's ability to use a permission."]
pub struct GoogleCloudPolicytroubleshooterV1betaBindingExplanation {
    #[serde(rename = "access")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether _this binding_ provides the specified permission to the specified member for the specified resource. This field does _not_ indicate whether the member actually has the permission for the resource. There might be another binding that overrides this binding. To determine whether the member actually has the permission, use the `access` field in the TroubleshootIamPolicyResponse."]
    pub access:
        ::std::option::Option<GoogleCloudPolicytroubleshooterV1betaBindingExplanationAccessEnum>,
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A condition expression that prevents access unless the expression evaluates to `true`. To learn about IAM Conditions, see http://cloud.google.com/iam/help/conditions/overview."]
    pub condition: ::std::option::Option<::std::boxed::Box<GoogleTypeExpr>>,
    #[serde(rename = "memberships")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether each member in the binding includes the member specified in the request, either directly or indirectly. Each key identifies a member in the binding, and each value indicates whether the member in the binding includes the member in the request. For example, suppose that a binding includes the following members: * `user:alice@example.com` * `group:product-eng@example.com` You want to troubleshoot access for `user:bob@example.com`. This user is a member of the group `group:product-eng@example.com`. For the first member in the binding, the key is `user:alice@example.com`, and the `membership` field in the value is set to `MEMBERSHIP_NOT_INCLUDED`. For the second member in the binding, the key is `group:product-eng@example.com`, and the `membership` field in the value is set to `MEMBERSHIP_INCLUDED`."]
    pub memberships: ::std::option::Option<
        ::std::collections::BTreeMap<
            String,
            ::std::boxed::Box<
                GoogleCloudPolicytroubleshooterV1betaBindingExplanationAnnotatedMembership,
            >,
        >,
    >,
    #[serde(rename = "relevance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The relevance of this binding to the overall determination for the entire policy."]
    pub relevance:
        ::std::option::Option<GoogleCloudPolicytroubleshooterV1betaBindingExplanationRelevanceEnum>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The role that this binding grants. For example, `roles/compute.serviceAgent`. For a complete list of predefined IAM roles, as well as the permissions in each role, see https://cloud.google.com/iam/help/roles/reference."]
    pub role: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rolePermission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the role granted by this binding contains the specified permission."]
    pub role_permission: ::std::option::Option<
        GoogleCloudPolicytroubleshooterV1betaBindingExplanationRolePermissionEnum,
    >,
    #[serde(rename = "rolePermissionRelevance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The relevance of the permission's existence, or nonexistence, in the role to the overall determination for the entire policy."]
    pub role_permission_relevance: ::std::option::Option<
        GoogleCloudPolicytroubleshooterV1betaBindingExplanationRolePermissionRelevanceEnum,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Indicates whether _this binding_ provides the specified permission to the specified member for the specified resource. This field does _not_ indicate whether the member actually has the permission for the resource. There might be another binding that overrides this binding. To determine whether the member actually has the permission, use the `access` field in the TroubleshootIamPolicyResponse."]
pub enum GoogleCloudPolicytroubleshooterV1betaBindingExplanationAccessEnum {
    #[serde(rename = "ACCESS_STATE_UNSPECIFIED")]
    #[doc = "Reserved for future use."]
    AccessStateUnspecified,
    #[serde(rename = "GRANTED")]
    #[doc = "The member has the permission."]
    Granted,
    #[serde(rename = "NOT_GRANTED")]
    #[doc = "The member does not have the permission."]
    NotGranted,
    #[serde(rename = "UNKNOWN_CONDITIONAL")]
    #[doc = "The member has the permission only if a condition expression evaluates to `true`."]
    UnknownConditional,
    #[serde(rename = "UNKNOWN_INFO_DENIED")]
    #[doc = "The sender of the request does not have access to all of the policies that Policy Troubleshooter needs to evaluate."]
    UnknownInfoDenied,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The relevance of this binding to the overall determination for the entire policy."]
pub enum GoogleCloudPolicytroubleshooterV1betaBindingExplanationRelevanceEnum {
    #[serde(rename = "HEURISTIC_RELEVANCE_UNSPECIFIED")]
    #[doc = "Reserved for future use."]
    HeuristicRelevanceUnspecified,
    #[serde(rename = "NORMAL")]
    #[doc = "The data point has a limited effect on the result. Changing the data point is unlikely to affect the overall determination."]
    Normal,
    #[serde(rename = "HIGH")]
    #[doc = "The data point has a strong effect on the result. Changing the data point is likely to affect the overall determination."]
    High,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Indicates whether the role granted by this binding contains the specified permission."]
pub enum GoogleCloudPolicytroubleshooterV1betaBindingExplanationRolePermissionEnum {
    #[serde(rename = "ROLE_PERMISSION_UNSPECIFIED")]
    #[doc = "Reserved for future use."]
    RolePermissionUnspecified,
    #[serde(rename = "ROLE_PERMISSION_INCLUDED")]
    #[doc = "The permission is included in the role."]
    RolePermissionIncluded,
    #[serde(rename = "ROLE_PERMISSION_NOT_INCLUDED")]
    #[doc = "The permission is not included in the role."]
    RolePermissionNotIncluded,
    #[serde(rename = "ROLE_PERMISSION_UNKNOWN_INFO_DENIED")]
    #[doc = "The sender of the request is not allowed to access the binding."]
    RolePermissionUnknownInfoDenied,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The relevance of the permission's existence, or nonexistence, in the role to the overall determination for the entire policy."]
pub enum GoogleCloudPolicytroubleshooterV1betaBindingExplanationRolePermissionRelevanceEnum {
    #[serde(rename = "HEURISTIC_RELEVANCE_UNSPECIFIED")]
    #[doc = "Reserved for future use."]
    HeuristicRelevanceUnspecified,
    #[serde(rename = "NORMAL")]
    #[doc = "The data point has a limited effect on the result. Changing the data point is unlikely to affect the overall determination."]
    Normal,
    #[serde(rename = "HIGH")]
    #[doc = "The data point has a strong effect on the result. Changing the data point is likely to affect the overall determination."]
    High,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details about whether the binding includes the member."]
pub struct GoogleCloudPolicytroubleshooterV1betaBindingExplanationAnnotatedMembership {
    #[serde(rename = "membership")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the binding includes the member."]
    pub membership: ::std::option::Option<
        GoogleCloudPolicytroubleshooterV1betaBindingExplanationAnnotatedMembershipMembershipEnum,
    >,
    #[serde(rename = "relevance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The relevance of the member's status to the overall determination for the binding."]
    pub relevance: ::std::option::Option<
        GoogleCloudPolicytroubleshooterV1betaBindingExplanationAnnotatedMembershipRelevanceEnum,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Indicates whether the binding includes the member."]
pub enum GoogleCloudPolicytroubleshooterV1betaBindingExplanationAnnotatedMembershipMembershipEnum {
    #[serde(rename = "MEMBERSHIP_UNSPECIFIED")]
    #[doc = "Reserved for future use."]
    MembershipUnspecified,
    #[serde(rename = "MEMBERSHIP_INCLUDED")]
    #[doc = "The binding includes the member. The member can be included directly or indirectly. For example: * A member is included directly if that member is listed in the binding. * A member is included indirectly if that member is in a Google group or G Suite domain that is listed in the binding."]
    MembershipIncluded,
    #[serde(rename = "MEMBERSHIP_NOT_INCLUDED")]
    #[doc = "The binding does not include the member."]
    MembershipNotIncluded,
    #[serde(rename = "MEMBERSHIP_UNKNOWN_INFO_DENIED")]
    #[doc = "The sender of the request is not allowed to access the binding."]
    MembershipUnknownInfoDenied,
    #[serde(rename = "MEMBERSHIP_UNKNOWN_UNSUPPORTED")]
    #[doc = "The member is an unsupported type. Only Google Accounts and service accounts are supported."]
    MembershipUnknownUnsupported,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The relevance of the member's status to the overall determination for the binding."]
pub enum GoogleCloudPolicytroubleshooterV1betaBindingExplanationAnnotatedMembershipRelevanceEnum {
    #[serde(rename = "HEURISTIC_RELEVANCE_UNSPECIFIED")]
    #[doc = "Reserved for future use."]
    HeuristicRelevanceUnspecified,
    #[serde(rename = "NORMAL")]
    #[doc = "The data point has a limited effect on the result. Changing the data point is unlikely to affect the overall determination."]
    Normal,
    #[serde(rename = "HIGH")]
    #[doc = "The data point has a strong effect on the result. Changing the data point is likely to affect the overall determination."]
    High,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details about how a specific IAM Policy contributed to the access check."]
pub struct GoogleCloudPolicytroubleshooterV1betaExplainedPolicy {
    #[serde(rename = "access")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether _this policy_ provides the specified permission to the specified member for the specified resource. This field does _not_ indicate whether the member actually has the permission for the resource. There might be another policy that overrides this policy. To determine whether the member actually has the permission, use the `access` field in the TroubleshootIamPolicyResponse."]
    pub access:
        ::std::option::Option<GoogleCloudPolicytroubleshooterV1betaExplainedPolicyAccessEnum>,
    #[serde(rename = "bindingExplanations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details about how each binding in the policy affects the member's ability, or inability, to use the permission for the resource. If the sender of the request does not have access to the policy, this field is omitted."]
    pub binding_explanations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudPolicytroubleshooterV1betaBindingExplanation>>,
    >,
    #[serde(rename = "fullResourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full resource name that identifies the resource. For example, `//compute.googleapis.com/projects/my-project/zones/us-central1-a/instances/my-instance`. If the sender of the request does not have access to the policy, this field is omitted. For examples of full resource names for Google Cloud services, see https://cloud.google.com/iam/help/troubleshooter/full-resource-names."]
    pub full_resource_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IAM policy attached to the resource. If the sender of the request does not have access to the policy, this field is empty."]
    pub policy: ::std::option::Option<::std::boxed::Box<GoogleIamV1Policy>>,
    #[serde(rename = "relevance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The relevance of this policy to the overall determination in the TroubleshootIamPolicyResponse. If the sender of the request does not have access to the policy, this field is omitted."]
    pub relevance:
        ::std::option::Option<GoogleCloudPolicytroubleshooterV1betaExplainedPolicyRelevanceEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Indicates whether _this policy_ provides the specified permission to the specified member for the specified resource. This field does _not_ indicate whether the member actually has the permission for the resource. There might be another policy that overrides this policy. To determine whether the member actually has the permission, use the `access` field in the TroubleshootIamPolicyResponse."]
pub enum GoogleCloudPolicytroubleshooterV1betaExplainedPolicyAccessEnum {
    #[serde(rename = "ACCESS_STATE_UNSPECIFIED")]
    #[doc = "Reserved for future use."]
    AccessStateUnspecified,
    #[serde(rename = "GRANTED")]
    #[doc = "The member has the permission."]
    Granted,
    #[serde(rename = "NOT_GRANTED")]
    #[doc = "The member does not have the permission."]
    NotGranted,
    #[serde(rename = "UNKNOWN_CONDITIONAL")]
    #[doc = "The member has the permission only if a condition expression evaluates to `true`."]
    UnknownConditional,
    #[serde(rename = "UNKNOWN_INFO_DENIED")]
    #[doc = "The sender of the request does not have access to all of the policies that Policy Troubleshooter needs to evaluate."]
    UnknownInfoDenied,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The relevance of this policy to the overall determination in the TroubleshootIamPolicyResponse. If the sender of the request does not have access to the policy, this field is omitted."]
pub enum GoogleCloudPolicytroubleshooterV1betaExplainedPolicyRelevanceEnum {
    #[serde(rename = "HEURISTIC_RELEVANCE_UNSPECIFIED")]
    #[doc = "Reserved for future use."]
    HeuristicRelevanceUnspecified,
    #[serde(rename = "NORMAL")]
    #[doc = "The data point has a limited effect on the result. Changing the data point is unlikely to affect the overall determination."]
    Normal,
    #[serde(rename = "HIGH")]
    #[doc = "The data point has a strong effect on the result. Changing the data point is likely to affect the overall determination."]
    High,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for TroubleshootIamPolicy."]
pub struct GoogleCloudPolicytroubleshooterV1betaTroubleshootIamPolicyRequest {
    #[serde(rename = "accessTuple")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The information to use for checking whether a member has a permission for a resource."]
    pub access_tuple:
        ::std::option::Option<::std::boxed::Box<GoogleCloudPolicytroubleshooterV1betaAccessTuple>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for TroubleshootIamPolicy."]
pub struct GoogleCloudPolicytroubleshooterV1betaTroubleshootIamPolicyResponse {
    #[serde(rename = "access")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the member has the specified permission for the specified resource, based on evaluating all of the applicable policies."]
    pub access: ::std::option::Option<
        GoogleCloudPolicytroubleshooterV1betaTroubleshootIamPolicyResponseAccessEnum,
    >,
    #[serde(rename = "explainedPolicies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of IAM policies that were evaluated to check the member's permissions, with annotations to indicate how each policy contributed to the final result. The list of policies can include the policy for the resource itself. It can also include policies that are inherited from higher levels of the resource hierarchy, including the organization, the folder, and the project. To learn more about the resource hierarchy, see https://cloud.google.com/iam/help/resource-hierarchy."]
    pub explained_policies: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudPolicytroubleshooterV1betaExplainedPolicy>>,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Indicates whether the member has the specified permission for the specified resource, based on evaluating all of the applicable policies."]
pub enum GoogleCloudPolicytroubleshooterV1betaTroubleshootIamPolicyResponseAccessEnum {
    #[serde(rename = "ACCESS_STATE_UNSPECIFIED")]
    #[doc = "Reserved for future use."]
    AccessStateUnspecified,
    #[serde(rename = "GRANTED")]
    #[doc = "The member has the permission."]
    Granted,
    #[serde(rename = "NOT_GRANTED")]
    #[doc = "The member does not have the permission."]
    NotGranted,
    #[serde(rename = "UNKNOWN_CONDITIONAL")]
    #[doc = "The member has the permission only if a condition expression evaluates to `true`."]
    UnknownConditional,
    #[serde(rename = "UNKNOWN_INFO_DENIED")]
    #[doc = "The sender of the request does not have access to all of the policies that Policy Troubleshooter needs to evaluate."]
    UnknownInfoDenied,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { \"audit_configs\": [ { \"service\": \"allServices\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" }, { \"log_type\": \"ADMIN_READ\" } ] }, { \"service\": \"sampleservice.googleapis.com\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\" }, { \"log_type\": \"DATA_WRITE\", \"exempted_members\": [ \"user:aliya@example.com\" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts jose@example.com from DATA_READ logging, and aliya@example.com from DATA_WRITE logging."]
pub struct GoogleIamV1AuditConfig {
    #[serde(rename = "auditLogConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configuration for logging of each type of permission."]
    pub audit_log_configs:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleIamV1AuditLogConfig>>>,
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services."]
    pub service: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Provides the configuration for logging a type of permissions. Example: { \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging."]
pub struct GoogleIamV1AuditLogConfig {
    #[serde(rename = "exemptedMembers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members."]
    pub exempted_members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "logType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The log type that this config enables."]
    pub log_type: ::std::option::Option<GoogleIamV1AuditLogConfigLogTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The log type that this config enables."]
pub enum GoogleIamV1AuditLogConfigLogTypeEnum {
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
pub struct GoogleIamV1Binding {
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the members in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
    pub condition: ::std::option::Option<::std::boxed::Box<GoogleTypeExpr>>,
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
#[doc = "An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members` to a single `role`. Members can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { \"bindings\": [ { \"role\": \"roles/resourcemanager.organizationAdmin\", \"members\": [ \"user:mike@example.com\", \"group:admins@example.com\", \"domain:google.com\", \"serviceAccount:my-project-id@appspot.gserviceaccount.com\" ] }, { \"role\": \"roles/resourcemanager.organizationViewer\", \"members\": [ \"user:eve@example.com\" ], \"condition\": { \"title\": \"expirable access\", \"description\": \"Does not grant access after Sep 2020\", \"expression\": \"request.time < timestamp('2020-10-01T00:00:00.000Z')\", } } ], \"etag\": \"BwWWja0YfJA=\", \"version\": 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') - etag: BwWWja0YfJA= - version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/)."]
pub struct GoogleIamV1Policy {
    #[serde(rename = "auditConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies cloud audit logging configuration for this policy."]
    pub audit_configs:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleIamV1AuditConfig>>>,
    #[serde(rename = "bindings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Associates a list of `members` to a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one member."]
    pub bindings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleIamV1Binding>>>,
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
#[doc = "Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: \"Summary size limit\" description: \"Determines if a summary is less than 100 chars\" expression: \"document.summary.size() < 100\" Example (Equality): title: \"Requestor is owner\" description: \"Determines if requestor is the document owner\" expression: \"document.owner == request.auth.claims.email\" Example (Logic): title: \"Public documents\" description: \"Determine whether the document should be publicly visible\" expression: \"document.type != 'private' && document.type != 'internal'\" Example (Data Manipulation): title: \"Notification string\" description: \"Create a notification string with a timestamp.\" expression: \"'New message received at ' + string(document.create_time)\" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information."]
pub struct GoogleTypeExpr {
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
