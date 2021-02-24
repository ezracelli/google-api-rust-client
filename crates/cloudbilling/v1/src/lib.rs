#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the aggregation level and interval for pricing of a single SKU."]
pub struct AggregationInfo {
    #[serde(rename = "aggregationCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of intervals to aggregate over. Example: If aggregation_level is \"DAILY\" and aggregation_count is 14, aggregation will be over 14 days."]
    pub aggregation_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "aggregationInterval")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub aggregation_interval: ::std::option::Option<AggregationInfoAggregationIntervalEnum>,
    #[serde(rename = "aggregationLevel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub aggregation_level: ::std::option::Option<AggregationInfoAggregationLevelEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum AggregationInfoAggregationIntervalEnum {
    #[serde(rename = "AGGREGATION_INTERVAL_UNSPECIFIED")]
    #[doc = ""]
    AggregationIntervalUnspecified,
    #[serde(rename = "DAILY")]
    #[doc = ""]
    Daily,
    #[serde(rename = "MONTHLY")]
    #[doc = ""]
    Monthly,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum AggregationInfoAggregationLevelEnum {
    #[serde(rename = "AGGREGATION_LEVEL_UNSPECIFIED")]
    #[doc = ""]
    AggregationLevelUnspecified,
    #[serde(rename = "ACCOUNT")]
    #[doc = ""]
    Account,
    #[serde(rename = "PROJECT")]
    #[doc = ""]
    Project,
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
#[doc = "A billing account in the [Google Cloud Console](https://console.cloud.google.com/). You can assign a billing account to one or more projects."]
pub struct BillingAccount {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display name given to the billing account, such as `My Billing Account`. This name is displayed in the Google Cloud Console."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "masterBillingAccount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If this account is a [subaccount](https://cloud.google.com/billing/docs/concepts), then this will be the resource name of the master billing account that it is being resold through. Otherwise this will be empty."]
    pub master_billing_account: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the billing account. The resource name has the form `billingAccounts/{billing_account_id}`. For example, `billingAccounts/012345-567890-ABCDEF` would be the resource name for billing account `012345-567890-ABCDEF`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "open")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. True if the billing account is open, and will therefore be charged for any usage on associated projects. False if the billing account is closed, and therefore projects associated with it will be unable to use paid services."]
    pub open: ::std::option::Option<::std::primitive::bool>,
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
#[doc = "Represents the category hierarchy of a SKU."]
pub struct Category {
    #[serde(rename = "resourceFamily")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of product the SKU refers to. Example: \"Compute\", \"Storage\", \"Network\", \"ApplicationServices\" etc."]
    pub resource_family: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceGroup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A group classification for related SKUs. Example: \"RAM\", \"GPU\", \"Prediction\", \"Ops\", \"GoogleEgress\" etc."]
    pub resource_group: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serviceDisplayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display name of the service this SKU belongs to."]
    pub service_display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "usageType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents how the SKU is consumed. Example: \"OnDemand\", \"Preemptible\", \"Commit1Mo\", \"Commit1Yr\" etc."]
    pub usage_type: ::std::option::Option<::std::string::String>,
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
#[doc = "Encapsulates the geographic taxonomy data for a sku."]
pub struct GeoTaxonomy {
    #[serde(rename = "regions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of regions associated with a sku. Empty for Global skus, which are associated with all Google Cloud regions."]
    pub regions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of Geo Taxonomy: GLOBAL, REGIONAL, or MULTI_REGIONAL."]
    pub _type: ::std::option::Option<GeoTaxonomyTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of Geo Taxonomy: GLOBAL, REGIONAL, or MULTI_REGIONAL."]
pub enum GeoTaxonomyTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "The type is not specified."]
    TypeUnspecified,
    #[serde(rename = "GLOBAL")]
    #[doc = "The sku is global in nature, e.g. a license sku. Global skus are available in all regions, and so have an empty region list."]
    Global,
    #[serde(rename = "REGIONAL")]
    #[doc = "The sku is available in a specific region, e.g. \"us-west2\"."]
    Regional,
    #[serde(rename = "MULTI_REGIONAL")]
    #[doc = "The sku is associated with multiple regions, e.g. \"us-west2\" and \"us-east1\"."]
    MultiRegional,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for `ListBillingAccounts`."]
pub struct ListBillingAccountsResponse {
    #[serde(rename = "billingAccounts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of billing accounts."]
    pub billing_accounts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BillingAccount>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. To retrieve the next page, call `ListBillingAccounts` again with the `page_token` field set to this value. This field is empty if there are no more results to retrieve."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for `ListProjectBillingInfoResponse`."]
pub struct ListProjectBillingInfoResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. To retrieve the next page, call `ListProjectBillingInfo` again with the `page_token` field set to this value. This field is empty if there are no more results to retrieve."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectBillingInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of `ProjectBillingInfo` resources representing the projects associated with the billing account."]
    pub project_billing_info:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProjectBillingInfo>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for `ListServices`."]
pub struct ListServicesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. To retrieve the next page, call `ListServices` again with the `page_token` field set to this value. This field is empty if there are no more results to retrieve."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of services."]
    pub services: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Service>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for `ListSkus`."]
pub struct ListSkusResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve the next page of results. To retrieve the next page, call `ListSkus` again with the `page_token` field set to this value. This field is empty if there are no more results to retrieve."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "skus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of public SKUs of the given service."]
    pub skus: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Sku>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents an amount of money with its currency type."]
pub struct Money {
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The three-letter currency code defined in ISO 4217."]
    pub currency_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nanos")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000."]
    pub nanos: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "units")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The whole units of the amount. For example if `currencyCode` is `\"USD\"`, then 1 unit is one US dollar."]
    pub units: ::std::option::Option<::std::string::String>,
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
#[doc = "Expresses a mathematical pricing formula. For Example:- `usage_unit: GBy` `tiered_rates:` `[start_usage_amount: 20, unit_price: $10]` `[start_usage_amount: 100, unit_price: $5]` The above expresses a pricing formula where the first 20GB is free, the next 80GB is priced at $10 per GB followed by $5 per GB for additional usage."]
pub struct PricingExpression {
    #[serde(rename = "baseUnit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The base unit for the SKU which is the unit used in usage exports. Example: \"By\""]
    pub base_unit: ::std::option::Option<::std::string::String>,
    #[serde(rename = "baseUnitConversionFactor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Conversion factor for converting from price per usage_unit to price per base_unit, and start_usage_amount to start_usage_amount in base_unit. unit_price / base_unit_conversion_factor = price per base_unit. start_usage_amount * base_unit_conversion_factor = start_usage_amount in base_unit."]
    pub base_unit_conversion_factor: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "baseUnitDescription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The base unit in human readable form. Example: \"byte\"."]
    pub base_unit_description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayQuantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The recommended quantity of units for displaying pricing info. When displaying pricing info it is recommended to display: (unit_price * display_quantity) per display_quantity usage_unit. This field does not affect the pricing formula and is for display purposes only. Example: If the unit_price is \"0.0001 USD\", the usage_unit is \"GB\" and the display_quantity is \"1000\" then the recommended way of displaying the pricing info is \"0.10 USD per 1000 GB\""]
    pub display_quantity: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "tieredRates")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of tiered rates for this pricing. The total cost is computed by applying each of the tiered rates on usage. This repeated list is sorted by ascending order of start_usage_amount."]
    pub tiered_rates: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TierRate>>>,
    #[serde(rename = "usageUnit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The short hand for unit of usage this pricing is specified in. Example: usage_unit of \"GiBy\" means that usage is specified in \"Gibi Byte\"."]
    pub usage_unit: ::std::option::Option<::std::string::String>,
    #[serde(rename = "usageUnitDescription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unit of usage in human readable form. Example: \"gibi byte\"."]
    pub usage_unit_description: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the pricing information for a SKU at a single point of time."]
pub struct PricingInfo {
    #[serde(rename = "aggregationInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Aggregation Info. This can be left unspecified if the pricing expression doesn't require aggregation."]
    pub aggregation_info: ::std::option::Option<::std::boxed::Box<AggregationInfo>>,
    #[serde(rename = "currencyConversionRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Conversion rate used for currency conversion, from USD to the currency specified in the request. This includes any surcharge collected for billing in non USD currency. If a currency is not specified in the request this defaults to 1.0. Example: USD * currency_conversion_rate = JPY"]
    pub currency_conversion_rate: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "effectiveTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The timestamp from which this pricing was effective within the requested time range. This is guaranteed to be greater than or equal to the start_time field in the request and less than the end_time field in the request. If a time range was not specified in the request this field will be equivalent to a time within the last 12 hours, indicating the latest pricing info."]
    pub effective_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pricingExpression")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Expresses the pricing formula. See `PricingExpression` for an example."]
    pub pricing_expression: ::std::option::Option<::std::boxed::Box<PricingExpression>>,
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An optional human readable summary of the pricing information, has a maximum length of 256 characters."]
    pub summary: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Encapsulation of billing information for a Google Cloud Console project. A project has at most one associated billing account at a time (but a billing account can be assigned to multiple projects)."]
pub struct ProjectBillingInfo {
    #[serde(rename = "billingAccountName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the billing account associated with the project, if any. For example, `billingAccounts/012345-567890-ABCDEF`."]
    pub billing_account_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "billingEnabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True if the project is associated with an open billing account, to which usage on the project is charged. False if the project is associated with a closed billing account, or no billing account at all, and therefore cannot use paid services. This field is read-only."]
    pub billing_enabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name for the `ProjectBillingInfo`; has the form `projects/{project_id}/billingInfo`. For example, the resource name for the billing information for project `tokyo-rain-123` would be `projects/tokyo-rain-123/billingInfo`. This field is read-only."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the project that this `ProjectBillingInfo` represents, such as `tokyo-rain-123`. This is a convenience field so that you don't need to parse the `name` field to obtain a project ID. This field is read-only."]
    pub project_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Encapsulates a single service in Google Cloud Platform."]
pub struct Service {
    #[serde(rename = "businessEntityName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The business under which the service is offered. Ex. \"businessEntities/GCP\", \"businessEntities/Maps\""]
    pub business_entity_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A human readable display name for this service."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name for the service. Example: \"services/DA34-426B-A397\""]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serviceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier for the service. Example: \"DA34-426B-A397\""]
    pub service_id: ::std::option::Option<::std::string::String>,
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
#[doc = "Encapsulates a single SKU in Google Cloud Platform"]
pub struct Sku {
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The category hierarchy of this SKU, purely for organizational purpose."]
    pub category: ::std::option::Option<::std::boxed::Box<Category>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A human readable description of the SKU, has a maximum length of 256 characters."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "geoTaxonomy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The geographic taxonomy for this sku."]
    pub geo_taxonomy: ::std::option::Option<::std::boxed::Box<GeoTaxonomy>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name for the SKU. Example: \"services/DA34-426B-A397/skus/AA95-CD31-42FE\""]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pricingInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A timeline of pricing info for this SKU in chronological order."]
    pub pricing_info: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PricingInfo>>>,
    #[serde(rename = "serviceProviderName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies the service provider. This is 'Google' for first party services in Google Cloud Platform."]
    pub service_provider_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serviceRegions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of service regions this SKU is offered at. Example: \"asia-east1\" Service regions can be found at https://cloud.google.com/about/locations/"]
    pub service_regions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "skuId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier for the SKU. Example: \"AA95-CD31-42FE\""]
    pub sku_id: ::std::option::Option<::std::string::String>,
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
#[doc = "The price rate indicating starting usage and its corresponding price."]
pub struct TierRate {
    #[serde(rename = "startUsageAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Usage is priced at this rate only after this amount. Example: start_usage_amount of 10 indicates that the usage will be priced at the unit_price after the first 10 usage_units."]
    pub start_usage_amount: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "unitPrice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The price per unit of usage. Example: unit_price of amount $10 indicates that each unit will cost $10."]
    pub unit_price: ::std::option::Option<::std::boxed::Box<Money>>,
}
