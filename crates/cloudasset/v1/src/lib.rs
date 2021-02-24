#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies roles and/or permissions to analyze, to determine both the identities possessing them and the resources they control. If multiple values are specified, results will include roles or permissions matching any of them. The total number of roles and permissions should be equal or less than 10."]
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
#[doc = "A request message for AssetService.AnalyzeIamPolicyLongrunning."]
pub struct AnalyzeIamPolicyLongrunningRequest {
    #[serde(rename = "analysisQuery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The request query."]
    pub analysis_query: ::std::option::Option<::std::boxed::Box<IamPolicyAnalysisQuery>>,
    #[serde(rename = "outputConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Output configuration indicating where the results will be output to."]
    pub output_config: ::std::option::Option<::std::boxed::Box<IamPolicyAnalysisOutputConfig>>,
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
    #[serde(rename = "serviceAccountImpersonationAnalysis")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The service account impersonation analysis if AnalyzeIamPolicyRequest.analyze_service_account_impersonation is enabled."]
    pub service_account_impersonation_analysis:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<IamPolicyAnalysis>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An asset in Google Cloud. An asset can be any resource in the Google Cloud [resource hierarchy](https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy), a resource outside the Google Cloud resource hierarchy (such as Google Kubernetes Engine clusters and objects), or a policy (e.g. Cloud IAM policy). See [Supported asset types](https://cloud.google.com/asset-inventory/docs/supported-asset-types) for more information."]
pub struct Asset {
    #[serde(rename = "accessLevel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Please also refer to the [access level user guide](https://cloud.google.com/access-context-manager/docs/overview#access-levels)."]
    pub access_level:
        ::std::option::Option<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1AccessLevel>>,
    #[serde(rename = "accessPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Please also refer to the [access policy user guide](https://cloud.google.com/access-context-manager/docs/overview#access-policies)."]
    pub access_policy:
        ::std::option::Option<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1AccessPolicy>>,
    #[serde(rename = "ancestors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ancestry path of an asset in Google Cloud [resource hierarchy](https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy), represented as a list of relative resource names. An ancestry path starts with the closest ancestor in the hierarchy and ends at root. If the asset is a project, folder, or organization, the ancestry path starts from the asset itself. Example: `[\"projects/123456789\", \"folders/5432\", \"organizations/1234\"]`"]
    pub ancestors: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "assetType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the asset. Example: `compute.googleapis.com/Disk` See [Supported asset types](https://cloud.google.com/asset-inventory/docs/supported-asset-types) for more information."]
    pub asset_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "iamPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A representation of the Cloud IAM policy set on a Google Cloud resource. There can be a maximum of one Cloud IAM policy set on any given resource. In addition, Cloud IAM policies inherit their granted access scope from any policies set on parent resources in the resource hierarchy. Therefore, the effectively policy is the union of both the policy set on this resource and each policy set on all of the resource's ancestry resource levels in the hierarchy. See [this topic](https://cloud.google.com/iam/docs/policies#inheritance) for more information."]
    pub iam_policy: ::std::option::Option<::std::boxed::Box<Policy>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full name of the asset. Example: `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1` See [Resource names](https://cloud.google.com/apis/design/resource_names#full_resource_name) for more information."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orgPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A representation of an [organization policy](https://cloud.google.com/resource-manager/docs/organization-policy/overview#organization_policy). There can be more than one organization policy with different constraints set on a given resource."]
    pub org_policy:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudOrgpolicyV1Policy>>>,
    #[serde(rename = "osInventory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A representation of runtime OS Inventory information. See [this topic](https://cloud.google.com/compute/docs/instances/os-inventory-management) for more information."]
    pub os_inventory: ::std::option::Option<::std::boxed::Box<Inventory>>,
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A representation of the resource."]
    pub resource: ::std::option::Option<::std::boxed::Box<Resource>>,
    #[serde(rename = "servicePerimeter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Please also refer to the [service perimeter user guide](https://cloud.google.com/vpc-service-controls/docs/overview)."]
    pub service_perimeter: ::std::option::Option<
        ::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1ServicePerimeter>,
    >,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last update timestamp of an asset. update_time is updated when create/update/delete operation is performed."]
    pub update_time: ::std::option::Option<::std::string::String>,
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
#[doc = "Batch get assets history response."]
pub struct BatchGetAssetsHistoryResponse {
    #[serde(rename = "assets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of assets with valid time windows."]
    pub assets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TemporalAsset>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A BigQuery destination for exporting assets to."]
pub struct BigQueryDestination {
    #[serde(rename = "dataset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The BigQuery dataset in format \"projects/projectId/datasets/datasetId\", to which the snapshot result should be exported. If this dataset does not exist, the export call returns an INVALID_ARGUMENT error."]
    pub dataset: ::std::option::Option<::std::string::String>,
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the destination table already exists and this flag is `TRUE`, the table will be overwritten by the contents of assets snapshot. If the flag is `FALSE` or unset and the destination table already exists, the export call returns an INVALID_ARGUMEMT error."]
    pub force: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "partitionSpec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[partition_spec] determines whether to export to partitioned table(s) and how to partition the data. If [partition_spec] is unset or [partition_spec.partition_key] is unset or `PARTITION_KEY_UNSPECIFIED`, the snapshot results will be exported to non-partitioned table(s). [force] will decide whether to overwrite existing table(s). If [partition_spec] is specified. First, the snapshot results will be written to partitioned table(s) with two additional timestamp columns, readTime and requestTime, one of which will be the partition key. Secondly, in the case when any destination table already exists, it will first try to update existing table's schema as necessary by appending additional columns. Then, if [force] is `TRUE`, the corresponding partition will be overwritten by the snapshot results (data in different partitions will remain intact); if [force] is unset or `FALSE`, it will append the data. An error will be returned if the schema update or data appension fails."]
    pub partition_spec: ::std::option::Option<::std::boxed::Box<PartitionSpec>>,
    #[serde(rename = "separateTablesPerAssetType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If this flag is `TRUE`, the snapshot results will be written to one or multiple tables, each of which contains results of one asset type. The [force] and [partition_spec] fields will apply to each of them. Field [table] will be concatenated with \"_\" and the asset type names (see https://cloud.google.com/asset-inventory/docs/supported-asset-types for supported asset types) to construct per-asset-type table names, in which all non-alphanumeric characters like \".\" and \"/\" will be substituted by \"_\". Example: if field [table] is \"mytable\" and snapshot results contain \"storage.googleapis.com/Bucket\" assets, the corresponding table name will be \"mytable_storage_googleapis_com_Bucket\". If any of these tables does not exist, a new table with the concatenated name will be created. When [content_type] in the ExportAssetsRequest is `RESOURCE`, the schema of each table will include RECORD-type columns mapped to the nested fields in the Asset.resource.data field of that asset type (up to the 15 nested level BigQuery supports (https://cloud.google.com/bigquery/docs/nested-repeated#limitations)). The fields in >15 nested levels will be stored in JSON format string as a child column of its parent RECORD column. If error occurs when exporting to any table, the whole export call will return an error but the export results that already succeed will persist. Example: if exporting to table_type_A succeeds when exporting to table_type_B fails during one export call, the results in table_type_A will persist and there will not be partial results persisting in a table."]
    pub separate_tables_per_asset_type: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "table")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The BigQuery table to which the snapshot result should be written. If this table does not exist, a new table with the given name will be created."]
    pub table: ::std::option::Option<::std::string::String>,
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
#[doc = "Create asset feed request."]
pub struct CreateFeedRequest {
    #[serde(rename = "feed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The feed details. The field `name` must be empty and it will be generated in the format of: projects/project_number/feeds/feed_id folders/folder_number/feeds/feed_id organizations/organization_number/feeds/feed_id"]
    pub feed: ::std::option::Option<::std::boxed::Box<Feed>>,
    #[serde(rename = "feedId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. This is the client-assigned asset feed identifier and it needs to be unique under a specific parent project/folder/organization."]
    pub feed_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Explanation about the IAM policy search result."]
pub struct Explanation {
    #[serde(rename = "matchedPermissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The map from roles to their included permissions that match the permission query (i.e., a query containing `policy.role.permissions:`). Example: if query `policy.role.permissions:compute.disk.get` matches a policy binding that contains owner role, the matched_permissions will be `{\"roles/owner\": [\"compute.disk.get\"]}`. The roles can also be found in the returned `policy` bindings. Note that the map is populated only for requests with permission queries."]
    pub matched_permissions:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<Permissions>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Export asset request."]
pub struct ExportAssetsRequest {
    #[serde(rename = "assetTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of asset types to take a snapshot for. For example: \"compute.googleapis.com/Disk\". Regular expressions are also supported. For example: * \"compute.googleapis.com.*\" snapshots resources whose asset type starts with \"compute.googleapis.com\". * \".*Instance\" snapshots resources whose asset type ends with \"Instance\". * \".*Instance.*\" snapshots resources whose asset type contains \"Instance\". See [RE2](https://github.com/google/re2/wiki/Syntax) for all supported regular expression syntax. If the regular expression does not match any supported asset type, an INVALID_ARGUMENT error will be returned. If specified, only matching assets will be returned, otherwise, it will snapshot all asset types. See [Introduction to Cloud Asset Inventory](https://cloud.google.com/asset-inventory/docs/overview) for all supported asset types."]
    pub asset_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Asset content type. If not specified, no content but the asset name will be returned."]
    pub content_type: ::std::option::Option<ExportAssetsRequestContentTypeEnum>,
    #[serde(rename = "outputConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Output configuration indicating where the results will be output to."]
    pub output_config: ::std::option::Option<::std::boxed::Box<OutputConfig>>,
    #[serde(rename = "readTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp to take an asset snapshot. This can only be set to a timestamp between the current time and the current time minus 35 days (inclusive). If not specified, the current time will be used. Due to delays in resource data collection and indexing, there is a volatile window during which running the same query may get different results."]
    pub read_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Asset content type. If not specified, no content but the asset name will be returned."]
pub enum ExportAssetsRequestContentTypeEnum {
    #[serde(rename = "CONTENT_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified content type."]
    ContentTypeUnspecified,
    #[serde(rename = "RESOURCE")]
    #[doc = "Resource metadata."]
    Resource,
    #[serde(rename = "IAM_POLICY")]
    #[doc = "The actual IAM policy set on a resource."]
    IamPolicy,
    #[serde(rename = "ORG_POLICY")]
    #[doc = "The Cloud Organization Policy set on an asset."]
    OrgPolicy,
    #[serde(rename = "ACCESS_POLICY")]
    #[doc = "The Cloud Access context manager Policy set on an asset."]
    AccessPolicy,
    #[serde(rename = "OS_INVENTORY")]
    #[doc = "The runtime OS Inventory information."]
    OsInventory,
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
#[doc = "An asset feed used to export asset updates to a destinations. An asset feed filter controls what updates are exported. The asset feed must be created within a project, organization, or folder. Supported destinations are: Pub/Sub topics."]
pub struct Feed {
    #[serde(rename = "assetNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of the full names of the assets to receive updates. You must specify either or both of asset_names and asset_types. Only asset updates matching specified asset_names or asset_types are exported to the feed. Example: `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`. See [Resource Names](https://cloud.google.com/apis/design/resource_names#full_resource_name) for more info."]
    pub asset_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "assetTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of types of the assets to receive updates. You must specify either or both of asset_names and asset_types. Only asset updates matching specified asset_names or asset_types are exported to the feed. Example: `\"compute.googleapis.com/Disk\"` See [this topic](https://cloud.google.com/asset-inventory/docs/supported-asset-types) for a list of all supported asset types."]
    pub asset_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A condition which determines whether an asset update should be published. If specified, an asset will be returned only when the expression evaluates to true. When set, `expression` field in the `Expr` must be a valid [CEL expression] (https://github.com/google/cel-spec) on a TemporalAsset with name `temporal_asset`. Example: a Feed with expression (\"temporal_asset.deleted == true\") will only publish Asset deletions. Other fields of `Expr` are optional. See our [user guide](https://cloud.google.com/asset-inventory/docs/monitoring-asset-changes#feed_with_condition) for detailed instructions."]
    pub condition: ::std::option::Option<::std::boxed::Box<Expr>>,
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Asset content type. If not specified, no content but the asset name and type will be returned."]
    pub content_type: ::std::option::Option<FeedContentTypeEnum>,
    #[serde(rename = "feedOutputConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Feed output configuration defining where the asset updates are published to."]
    pub feed_output_config: ::std::option::Option<::std::boxed::Box<FeedOutputConfig>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The format will be projects/{project_number}/feeds/{client-assigned_feed_identifier} or folders/{folder_number}/feeds/{client-assigned_feed_identifier} or organizations/{organization_number}/feeds/{client-assigned_feed_identifier} The client-assigned feed identifier must be unique within the parent project/folder/organization."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Asset content type. If not specified, no content but the asset name and type will be returned."]
pub enum FeedContentTypeEnum {
    #[serde(rename = "CONTENT_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified content type."]
    ContentTypeUnspecified,
    #[serde(rename = "RESOURCE")]
    #[doc = "Resource metadata."]
    Resource,
    #[serde(rename = "IAM_POLICY")]
    #[doc = "The actual IAM policy set on a resource."]
    IamPolicy,
    #[serde(rename = "ORG_POLICY")]
    #[doc = "The Cloud Organization Policy set on an asset."]
    OrgPolicy,
    #[serde(rename = "ACCESS_POLICY")]
    #[doc = "The Cloud Access context manager Policy set on an asset."]
    AccessPolicy,
    #[serde(rename = "OS_INVENTORY")]
    #[doc = "The runtime OS Inventory information."]
    OsInventory,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Output configuration for asset feed destination."]
pub struct FeedOutputConfig {
    #[serde(rename = "pubsubDestination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Destination on Pub/Sub."]
    pub pubsub_destination: ::std::option::Option<::std::boxed::Box<PubsubDestination>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Cloud Storage location."]
pub struct GcsDestination {
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The uri of the Cloud Storage object. It's the same uri that is used by gsutil. Example: \"gs://bucket_name/object_name\". See [Viewing and Editing Object Metadata](https://cloud.google.com/storage/docs/viewing-editing-metadata) for more information."]
    pub uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "uriPrefix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The uri prefix of all generated Cloud Storage objects. Example: \"gs://bucket_name/object_name_prefix\". Each object uri is in format: \"gs://bucket_name/object_name_prefix// and only contains assets for that type. starts from 0. Example: \"gs://bucket_name/object_name_prefix/compute.googleapis.com/Disk/0\" is the first shard of output objects containing all compute.googleapis.com/Disk assets. An INVALID_ARGUMENT error will be returned if file with the same name \"gs://bucket_name/object_name_prefix\" already exists."]
    pub uri_prefix: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An IAM role or permission under analysis."]
pub struct GoogleCloudAssetV1Access {
    #[serde(rename = "analysisState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The analysis state of this access."]
    pub analysis_state: ::std::option::Option<::std::boxed::Box<IamPolicyAnalysisState>>,
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
pub struct GoogleCloudAssetV1AccessControlList {
    #[serde(rename = "accesses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The accesses that match one of the following conditions: - The access_selector, if it is specified in request; - Otherwise, access specifiers reachable from the policy binding's role."]
    pub accesses:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudAssetV1Access>>>,
    #[serde(rename = "resourceEdges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource edges of the graph starting from the policy attached resource to any descendant resources. The Edge.source_node contains the full resource name of a parent resource and Edge.target_node contains the full resource name of a child resource. This field is present only if the output_resource_edges option is enabled in request."]
    pub resource_edges:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudAssetV1Edge>>>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resources that match one of the following conditions: - The resource_selector, if it is specified in request; - Otherwise, resources reachable from the policy attached resource."]
    pub resources:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudAssetV1Resource>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A BigQuery destination."]
pub struct GoogleCloudAssetV1BigQueryDestination {
    #[serde(rename = "dataset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The BigQuery dataset in format \"projects/projectId/datasets/datasetId\", to which the analysis results should be exported. If this dataset does not exist, the export call will return an INVALID_ARGUMENT error."]
    pub dataset: ::std::option::Option<::std::string::String>,
    #[serde(rename = "partitionKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The partition key for BigQuery partitioned table."]
    pub partition_key: ::std::option::Option<GoogleCloudAssetV1BigQueryDestinationPartitionKeyEnum>,
    #[serde(rename = "tablePrefix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The prefix of the BigQuery tables to which the analysis results will be written. Tables will be created based on this table_prefix if not exist: * _analysis table will contain export operation's metadata. * _analysis_result will contain all the IamPolicyAnalysisResult. When [partition_key] is specified, both tables will be partitioned based on the [partition_key]."]
    pub table_prefix: ::std::option::Option<::std::string::String>,
    #[serde(rename = "writeDisposition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Specifies the action that occurs if the destination table or partition already exists. The following values are supported: * WRITE_TRUNCATE: If the table or partition already exists, BigQuery overwrites the entire table or all the partitions data. * WRITE_APPEND: If the table or partition already exists, BigQuery appends the data to the table or the latest partition. * WRITE_EMPTY: If the table already exists and contains data, an error is returned. The default value is WRITE_APPEND. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Details are at https://cloud.google.com/bigquery/docs/loading-data-local#appending_to_or_overwriting_a_table_using_a_local_file."]
    pub write_disposition: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The partition key for BigQuery partitioned table."]
pub enum GoogleCloudAssetV1BigQueryDestinationPartitionKeyEnum {
    #[serde(rename = "PARTITION_KEY_UNSPECIFIED")]
    #[doc = "Unspecified partition key. Tables won't be partitioned using this option."]
    PartitionKeyUnspecified,
    #[serde(rename = "REQUEST_TIME")]
    #[doc = "The time when the request is received. If specified as partition key, the result table(s) is partitoned by the RequestTime column, an additional timestamp column representing when the request was received."]
    RequestTime,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A directional edge."]
pub struct GoogleCloudAssetV1Edge {
    #[serde(rename = "sourceNode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source node of the edge. For example, it could be a full resource name for a resource node or an email of an identity."]
    pub source_node: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetNode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target node of the edge. For example, it could be a full resource name for a resource node or an email of an identity."]
    pub target_node: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Cloud Storage location."]
pub struct GoogleCloudAssetV1GcsDestination {
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The uri of the Cloud Storage object. It's the same uri that is used by gsutil. For example: \"gs://bucket_name/object_name\". See [Quickstart: Using the gsutil tool] (https://cloud.google.com/storage/docs/quickstart-gsutil) for examples."]
    pub uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An identity under analysis."]
pub struct GoogleCloudAssetV1Identity {
    #[serde(rename = "analysisState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The analysis state of this identity."]
    pub analysis_state: ::std::option::Option<::std::boxed::Box<IamPolicyAnalysisState>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identity name in any form of members appear in [IAM policy binding](https://cloud.google.com/iam/reference/rest/v1/Binding), such as: - user:foo@google.com - group:group1@google.com - serviceAccount:s1@prj1.iam.gserviceaccount.com - projectOwner:some_project_id - domain:google.com - allUsers - etc."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The identities and group edges."]
pub struct GoogleCloudAssetV1IdentityList {
    #[serde(rename = "groupEdges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Group identity edges of the graph starting from the binding's group members to any node of the identities. The Edge.source_node contains a group, such as `group:parent@google.com`. The Edge.target_node contains a member of the group, such as `group:child@google.com` or `user:foo@google.com`. This field is present only if the output_group_edges option is enabled in request."]
    pub group_edges:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudAssetV1Edge>>>,
    #[serde(rename = "identities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only the identities that match one of the following conditions will be presented: - The identity_selector, if it is specified in request; - Otherwise, identities reachable from the policy binding's members."]
    pub identities:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudAssetV1Identity>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Google Cloud resource under analysis."]
pub struct GoogleCloudAssetV1Resource {
    #[serde(rename = "analysisState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The analysis state of this resource."]
    pub analysis_state: ::std::option::Option<::std::boxed::Box<IamPolicyAnalysisState>>,
    #[serde(rename = "fullResourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The [full resource name](https://cloud.google.com/asset-inventory/docs/resource-name-format)"]
    pub full_resource_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Used in `policy_type` to specify how `boolean_policy` will behave at this resource."]
pub struct GoogleCloudOrgpolicyV1BooleanPolicy {
    #[serde(rename = "enforced")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If `true`, then the `Policy` is enforced. If `false`, then any configuration is acceptable. Suppose you have a `Constraint` `constraints/compute.disableSerialPortAccess` with `constraint_default` set to `ALLOW`. A `Policy` for that `Constraint` exhibits the following behavior: - If the `Policy` at this resource has enforced set to `false`, serial port connection attempts will be allowed. - If the `Policy` at this resource has enforced set to `true`, serial port connection attempts will be refused. - If the `Policy` at this resource is `RestoreDefault`, serial port connection attempts will be allowed. - If no `Policy` is set at this resource or anywhere higher in the resource hierarchy, serial port connection attempts will be allowed. - If no `Policy` is set at this resource, but one exists higher in the resource hierarchy, the behavior is as if the`Policy` were set at this resource. The following examples demonstrate the different possible layerings: Example 1 (nearest `Constraint` wins): `organizations/foo` has a `Policy` with: {enforced: false} `projects/bar` has no `Policy` set. The constraint at `projects/bar` and `organizations/foo` will not be enforced. Example 2 (enforcement gets replaced): `organizations/foo` has a `Policy` with: {enforced: false} `projects/bar` has a `Policy` with: {enforced: true} The constraint at `organizations/foo` is not enforced. The constraint at `projects/bar` is enforced. Example 3 (RestoreDefault): `organizations/foo` has a `Policy` with: {enforced: true} `projects/bar` has a `Policy` with: {RestoreDefault: {}} The constraint at `organizations/foo` is enforced. The constraint at `projects/bar` is not enforced, because `constraint_default` for the `Constraint` is `ALLOW`."]
    pub enforced: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Used in `policy_type` to specify how `list_policy` behaves at this resource. `ListPolicy` can define specific values and subtrees of Cloud Resource Manager resource hierarchy (`Organizations`, `Folders`, `Projects`) that are allowed or denied by setting the `allowed_values` and `denied_values` fields. This is achieved by using the `under:` and optional `is:` prefixes. The `under:` prefix is used to denote resource subtree values. The `is:` prefix is used to denote specific values, and is required only if the value contains a \":\". Values prefixed with \"is:\" are treated the same as values with no prefix. Ancestry subtrees must be in one of the following formats: - \"projects/\", e.g. \"projects/tokyo-rain-123\" - \"folders/\", e.g. \"folders/1234\" - \"organizations/\", e.g. \"organizations/1234\" The `supports_under` field of the associated `Constraint` defines whether ancestry prefixes can be used. You can set `allowed_values` and `denied_values` in the same `Policy` if `all_values` is `ALL_VALUES_UNSPECIFIED`. `ALLOW` or `DENY` are used to allow or deny all values. If `all_values` is set to either `ALLOW` or `DENY`, `allowed_values` and `denied_values` must be unset."]
pub struct GoogleCloudOrgpolicyV1ListPolicy {
    #[serde(rename = "allValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The policy all_values state."]
    pub all_values: ::std::option::Option<GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum>,
    #[serde(rename = "allowedValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of values allowed at this resource. Can only be set if `all_values` is set to `ALL_VALUES_UNSPECIFIED`."]
    pub allowed_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "deniedValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of values denied at this resource. Can only be set if `all_values` is set to `ALL_VALUES_UNSPECIFIED`."]
    pub denied_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "inheritFromParent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Determines the inheritance behavior for this `Policy`. By default, a `ListPolicy` set at a resource supersedes any `Policy` set anywhere up the resource hierarchy. However, if `inherit_from_parent` is set to `true`, then the values from the effective `Policy` of the parent resource are inherited, meaning the values set in this `Policy` are added to the values inherited up the hierarchy. Setting `Policy` hierarchies that inherit both allowed values and denied values isn't recommended in most circumstances to keep the configuration simple and understandable. However, it is possible to set a `Policy` with `allowed_values` set that inherits a `Policy` with `denied_values` set. In this case, the values that are allowed must be in `allowed_values` and not present in `denied_values`. For example, suppose you have a `Constraint` `constraints/serviceuser.services`, which has a `constraint_type` of `list_constraint`, and with `constraint_default` set to `ALLOW`. Suppose that at the Organization level, a `Policy` is applied that restricts the allowed API activations to {`E1`, `E2`}. Then, if a `Policy` is applied to a project below the Organization that has `inherit_from_parent` set to `false` and field all_values set to DENY, then an attempt to activate any API will be denied. The following examples demonstrate different possible layerings for `projects/bar` parented by `organizations/foo`: Example 1 (no inherited values): `organizations/foo` has a `Policy` with values: {allowed_values: \"E1\" allowed_values:\"E2\"} `projects/bar` has `inherit_from_parent` `false` and values: {allowed_values: \"E3\" allowed_values: \"E4\"} The accepted values at `organizations/foo` are `E1`, `E2`. The accepted values at `projects/bar` are `E3`, and `E4`. Example 2 (inherited values): `organizations/foo` has a `Policy` with values: {allowed_values: \"E1\" allowed_values:\"E2\"} `projects/bar` has a `Policy` with values: {value: \"E3\" value: \"E4\" inherit_from_parent: true} The accepted values at `organizations/foo` are `E1`, `E2`. The accepted values at `projects/bar` are `E1`, `E2`, `E3`, and `E4`. Example 3 (inheriting both allowed and denied values): `organizations/foo` has a `Policy` with values: {allowed_values: \"E1\" allowed_values: \"E2\"} `projects/bar` has a `Policy` with: {denied_values: \"E1\"} The accepted values at `organizations/foo` are `E1`, `E2`. The value accepted at `projects/bar` is `E2`. Example 4 (RestoreDefault): `organizations/foo` has a `Policy` with values: {allowed_values: \"E1\" allowed_values:\"E2\"} `projects/bar` has a `Policy` with values: {RestoreDefault: {}} The accepted values at `organizations/foo` are `E1`, `E2`. The accepted values at `projects/bar` are either all or none depending on the value of `constraint_default` (if `ALLOW`, all; if `DENY`, none). Example 5 (no policy inherits parent policy): `organizations/foo` has no `Policy` set. `projects/bar` has no `Policy` set. The accepted values at both levels are either all or none depending on the value of `constraint_default` (if `ALLOW`, all; if `DENY`, none). Example 6 (ListConstraint allowing all): `organizations/foo` has a `Policy` with values: {allowed_values: \"E1\" allowed_values: \"E2\"} `projects/bar` has a `Policy` with: {all: ALLOW} The accepted values at `organizations/foo` are `E1`, E2`. Any value is accepted at `projects/bar`. Example 7 (ListConstraint allowing none): `organizations/foo` has a `Policy` with values: {allowed_values: \"E1\" allowed_values: \"E2\"} `projects/bar` has a `Policy` with: {all: DENY} The accepted values at `organizations/foo` are `E1`, E2`. No value is accepted at `projects/bar`. Example 10 (allowed and denied subtrees of Resource Manager hierarchy): Given the following resource hierarchy O1->{F1, F2}; F1->{P1}; F2->{P2, P3}, `organizations/foo` has a `Policy` with values: {allowed_values: \"under:organizations/O1\"} `projects/bar` has a `Policy` with: {allowed_values: \"under:projects/P3\"} {denied_values: \"under:folders/F2\"} The accepted values at `organizations/foo` are `organizations/O1`, `folders/F1`, `folders/F2`, `projects/P1`, `projects/P2`, `projects/P3`. The accepted values at `projects/bar` are `organizations/O1`, `folders/F1`, `projects/P1`."]
    pub inherit_from_parent: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "suggestedValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The Google Cloud Console will try to default to a configuration that matches the value specified in this `Policy`. If `suggested_value` is not set, it will inherit the value specified higher in the hierarchy, unless `inherit_from_parent` is `false`."]
    pub suggested_value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The policy all_values state."]
pub enum GoogleCloudOrgpolicyV1ListPolicyAllValuesEnum {
    #[serde(rename = "ALL_VALUES_UNSPECIFIED")]
    #[doc = "Indicates that allowed_values or denied_values must be set."]
    AllValuesUnspecified,
    #[serde(rename = "ALLOW")]
    #[doc = "A policy with this set allows all values."]
    Allow,
    #[serde(rename = "DENY")]
    #[doc = "A policy with this set denies all values."]
    Deny,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines a Cloud Organization `Policy` which is used to specify `Constraints` for configurations of Cloud Platform resources."]
pub struct GoogleCloudOrgpolicyV1Policy {
    #[serde(rename = "booleanPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For boolean `Constraints`, whether to enforce the `Constraint` or not."]
    pub boolean_policy:
        ::std::option::Option<::std::boxed::Box<GoogleCloudOrgpolicyV1BooleanPolicy>>,
    #[serde(rename = "constraint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the `Constraint` the `Policy` is configuring, for example, `constraints/serviceuser.services`. A [list of available constraints](/resource-manager/docs/organization-policy/org-policy-constraints) is available. Immutable after creation."]
    pub constraint: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An opaque tag indicating the current version of the `Policy`, used for concurrency control. When the `Policy` is returned from either a `GetPolicy` or a `ListOrgPolicy` request, this `etag` indicates the version of the current `Policy` to use when executing a read-modify-write loop. When the `Policy` is returned from a `GetEffectivePolicy` request, the `etag` will be unset. When the `Policy` is used in a `SetOrgPolicy` method, use the `etag` value that was returned from a `GetOrgPolicy` request as part of a read-modify-write loop for concurrency control. Not setting the `etag`in a `SetOrgPolicy` request will result in an unconditional write of the `Policy`."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "listPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of values either allowed or disallowed."]
    pub list_policy: ::std::option::Option<::std::boxed::Box<GoogleCloudOrgpolicyV1ListPolicy>>,
    #[serde(rename = "restoreDefault")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restores the default behavior of the constraint; independent of `Constraint` type."]
    pub restore_default:
        ::std::option::Option<::std::boxed::Box<GoogleCloudOrgpolicyV1RestoreDefault>>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time stamp the `Policy` was previously updated. This is set by the server, not specified by the caller, and represents the last time a call to `SetOrgPolicy` was made for that `Policy`. Any value set by the client will be ignored."]
    pub update_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Version of the `Policy`. Default version is 0;"]
    pub version: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Ignores policies set above this resource and restores the `constraint_default` enforcement behavior of the specific `Constraint` at this resource. Suppose that `constraint_default` is set to `ALLOW` for the `Constraint` `constraints/serviceuser.services`. Suppose that organization foo.com sets a `Policy` at their Organization resource node that restricts the allowed service activations to deny all service activations. They could then set a `Policy` with the `policy_type` `restore_default` on several experimental projects, restoring the `constraint_default` enforcement of the `Constraint` for only those projects, allowing those projects to have all services activated."]
pub struct GoogleCloudOrgpolicyV1RestoreDefault {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An `AccessLevel` is a label that can be applied to requests to Google Cloud services, along with a list of requirements necessary for the label to be applied."]
pub struct GoogleIdentityAccesscontextmanagerV1AccessLevel {
    #[serde(rename = "basic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A `BasicLevel` composed of `Conditions`."]
    pub basic:
        ::std::option::Option<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1BasicLevel>>,
    #[serde(rename = "custom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A `CustomLevel` written in the Common Expression Language."]
    pub custom:
        ::std::option::Option<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1CustomLevel>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the `AccessLevel` and its use. Does not affect behavior."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Resource name for the Access Level. The `short_name` component must begin with a letter and only include alphanumeric and '_'. Format: `accessPolicies/{policy_id}/accessLevels/{short_name}`. The maximum length of the `short_name` component is 50 characters."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Human readable title. Must be unique within the Policy."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`AccessPolicy` is a container for `AccessLevels` (which define the necessary attributes to use Google Cloud services) and `ServicePerimeters` (which define regions of services able to freely pass data within a perimeter). An access policy is globally visible within an organization, and the restrictions it specifies apply to all projects within an organization."]
pub struct GoogleIdentityAccesscontextmanagerV1AccessPolicy {
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. An opaque identifier for the current version of the `AccessPolicy`. This will always be a strongly validated etag, meaning that two Access Polices will be identical if and only if their etags are identical. Clients should not expect this to be in any specific format."]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Resource name of the `AccessPolicy`. Format: `accessPolicies/{policy_id}`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The parent of this `AccessPolicy` in the Cloud Resource Hierarchy. Currently immutable once created. Format: `organizations/{organization_id}`"]
    pub parent: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Human readable title. Does not affect behavior."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Identification for an API Operation."]
pub struct GoogleIdentityAccesscontextmanagerV1ApiOperation {
    #[serde(rename = "methodSelectors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "API methods or permissions to allow. Method or permission must belong to the service specified by `service_name` field. A single MethodSelector entry with `*` specified for the `method` field will allow all methods AND permissions for the service specified in `service_name`."]
    pub method_selectors: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1MethodSelector>>,
    >,
    #[serde(rename = "serviceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the API whose methods or permissions the IngressPolicy or EgressPolicy want to allow. A single ApiOperation with `service_name` field set to `*` will allow all methods AND permissions for all services."]
    pub service_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`BasicLevel` is an `AccessLevel` using a set of recommended features."]
pub struct GoogleIdentityAccesscontextmanagerV1BasicLevel {
    #[serde(rename = "combiningFunction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "How the `conditions` list should be combined to determine if a request is granted this `AccessLevel`. If AND is used, each `Condition` in `conditions` must be satisfied for the `AccessLevel` to be applied. If OR is used, at least one `Condition` in `conditions` must be satisfied for the `AccessLevel` to be applied. Default behavior is AND."]
    pub combining_function:
        ::std::option::Option<GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunctionEnum>,
    #[serde(rename = "conditions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A list of requirements for the `AccessLevel` to be granted."]
    pub conditions: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1Condition>>,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "How the `conditions` list should be combined to determine if a request is granted this `AccessLevel`. If AND is used, each `Condition` in `conditions` must be satisfied for the `AccessLevel` to be applied. If OR is used, at least one `Condition` in `conditions` must be satisfied for the `AccessLevel` to be applied. Default behavior is AND."]
pub enum GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunctionEnum {
    #[serde(rename = "AND")]
    #[doc = "All `Conditions` must be true for the `BasicLevel` to be true."]
    And,
    #[serde(rename = "OR")]
    #[doc = "If at least one `Condition` is true, then the `BasicLevel` is true."]
    Or,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A condition necessary for an `AccessLevel` to be granted. The Condition is an AND over its fields. So a Condition is true if: 1) the request IP is from one of the listed subnetworks AND 2) the originating device complies with the listed device policy AND 3) all listed access levels are granted AND 4) the request was sent at a time allowed by the DateTimeRestriction."]
pub struct GoogleIdentityAccesscontextmanagerV1Condition {
    #[serde(rename = "devicePolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Device specific restrictions, all restrictions must hold for the Condition to be true. If not specified, all devices are allowed."]
    pub device_policy:
        ::std::option::Option<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1DevicePolicy>>,
    #[serde(rename = "ipSubnetworks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "CIDR block IP subnetwork specification. May be IPv4 or IPv6. Note that for a CIDR IP address block, the specified IP address portion must be properly truncated (i.e. all the host bits must be zero) or the input is considered malformed. For example, \"192.0.2.0/24\" is accepted but \"192.0.2.1/24\" is not. Similarly, for IPv6, \"2001:db8::/32\" is accepted whereas \"2001:db8::1/32\" is not. The originating IP of a request must be in one of the listed subnets in order for this Condition to be true. If empty, all IP addresses are allowed."]
    pub ip_subnetworks: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "members")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request must be made by one of the provided user or service accounts. Groups are not supported. Syntax: `user:{emailid}` `serviceAccount:{emailid}` If not specified, a request may come from any user."]
    pub members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "negate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to negate the Condition. If true, the Condition becomes a NAND over its non-empty fields, each field must be false for the Condition overall to be satisfied. Defaults to false."]
    pub negate: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "regions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request must originate from one of the provided countries/regions. Must be valid ISO 3166-1 alpha-2 codes."]
    pub regions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "requiredAccessLevels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of other access levels defined in the same `Policy`, referenced by resource name. Referencing an `AccessLevel` which does not exist is an error. All access levels listed must be granted for the Condition to be true. Example: \"`accessPolicies/MY_POLICY/accessLevels/LEVEL_NAME\"`"]
    pub required_access_levels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`CustomLevel` is an `AccessLevel` using the Cloud Common Expression Language to represent the necessary conditions for the level to apply to a request. See CEL spec at: https://github.com/google/cel-spec"]
pub struct GoogleIdentityAccesscontextmanagerV1CustomLevel {
    #[serde(rename = "expr")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A Cloud CEL expression evaluating to a boolean."]
    pub expr: ::std::option::Option<::std::boxed::Box<Expr>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`DevicePolicy` specifies device specific restrictions necessary to acquire a given access level. A `DevicePolicy` specifies requirements for requests from devices to be granted access levels, it does not do any enforcement on the device. `DevicePolicy` acts as an AND over all specified fields, and each repeated field is an OR over its elements. Any unset fields are ignored. For example, if the proto is { os_type : DESKTOP_WINDOWS, os_type : DESKTOP_LINUX, encryption_status: ENCRYPTED}, then the DevicePolicy will be true for requests originating from encrypted Linux desktops and encrypted Windows desktops."]
pub struct GoogleIdentityAccesscontextmanagerV1DevicePolicy {
    #[serde(rename = "allowedDeviceManagementLevels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Allowed device management levels, an empty list allows all management levels."]
    pub allowed_device_management_levels: ::std::option::Option<
        ::std::vec::Vec<
            GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum,
        >,
    >,
    #[serde(rename = "allowedEncryptionStatuses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Allowed encryptions statuses, an empty list allows all statuses."]
    pub allowed_encryption_statuses: ::std::option::Option<
        ::std::vec::Vec<
            GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum,
        >,
    >,
    #[serde(rename = "osConstraints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Allowed OS versions, an empty list allows all types and all versions."]
    pub os_constraints: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1OsConstraint>>,
    >,
    #[serde(rename = "requireAdminApproval")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the device needs to be approved by the customer admin."]
    pub require_admin_approval: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "requireCorpOwned")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the device needs to be corp owned."]
    pub require_corp_owned: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "requireScreenlock")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether or not screenlock is required for the DevicePolicy to be true. Defaults to `false`."]
    pub require_screenlock: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsEnum {
    #[serde(rename = "MANAGEMENT_UNSPECIFIED")]
    #[doc = "The device's management level is not specified or not known."]
    ManagementUnspecified,
    #[serde(rename = "NONE")]
    #[doc = "The device is not managed."]
    None,
    #[serde(rename = "BASIC")]
    #[doc = "Basic management is enabled, which is generally limited to monitoring and wiping the corporate account."]
    Basic,
    #[serde(rename = "COMPLETE")]
    #[doc = "Complete device management. This includes more thorough monitoring and the ability to directly manage the device (such as remote wiping). This can be enabled through the Android Enterprise Platform."]
    Complete,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesEnum {
    #[serde(rename = "ENCRYPTION_UNSPECIFIED")]
    #[doc = "The encryption status of the device is not specified or not known."]
    EncryptionUnspecified,
    #[serde(rename = "ENCRYPTION_UNSUPPORTED")]
    #[doc = "The device does not support encryption."]
    EncryptionUnsupported,
    #[serde(rename = "UNENCRYPTED")]
    #[doc = "The device supports encryption, but is currently unencrypted."]
    Unencrypted,
    #[serde(rename = "ENCRYPTED")]
    #[doc = "The device is encrypted."]
    Encrypted,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines the conditions under which an EgressPolicy matches a request. Conditions based on information about the source of the request. Note that if the destination of the request is protected by a ServicePerimeter, then that ServicePerimeter must have an IngressPolicy which allows access in order for this request to succeed."]
pub struct GoogleIdentityAccesscontextmanagerV1EgressFrom {
    #[serde(rename = "identities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of identities that are allowed access through this [EgressPolicy]. Should be in the format of email address. The email address should represent individual user or service account only."]
    pub identities: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "identityType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the type of identities that are allowed access to outside the perimeter. If left unspecified, then members of `identities` field will be allowed access."]
    pub identity_type:
        ::std::option::Option<GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Specifies the type of identities that are allowed access to outside the perimeter. If left unspecified, then members of `identities` field will be allowed access."]
pub enum GoogleIdentityAccesscontextmanagerV1EgressFromIdentityTypeEnum {
    #[serde(rename = "IDENTITY_TYPE_UNSPECIFIED")]
    #[doc = "No blanket identity group specified."]
    IdentityTypeUnspecified,
    #[serde(rename = "ANY_IDENTITY")]
    #[doc = "Authorize access from all identities outside the perimeter."]
    AnyIdentity,
    #[serde(rename = "ANY_USER_ACCOUNT")]
    #[doc = "Authorize access from all human users outside the perimeter."]
    AnyUserAccount,
    #[serde(rename = "ANY_SERVICE_ACCOUNT")]
    #[doc = "Authorize access from all service accounts outside the perimeter."]
    AnyServiceAccount,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Policy for egress from perimeter. EgressPolicies match requests based on `egress_from` and `egress_to` stanzas. For an EgressPolicy to match, both `egress_from` and `egress_to` stanzas must be matched. If an EgressPolicy matches a request, the request is allowed to span the ServicePerimeter boundary. For example, an EgressPolicy can be used to allow VMs on networks within the ServicePerimeter to access a defined set of projects outside the perimeter in certain contexts (e.g. to read data from a Cloud Storage bucket or query against a BigQuery dataset). EgressPolicies are concerned with the *resources* that a request relates as well as the API services and API actions being used. They do not related to the direction of data movement. More detailed documentation for this concept can be found in the descriptions of EgressFrom and EgressTo."]
pub struct GoogleIdentityAccesscontextmanagerV1EgressPolicy {
    #[serde(rename = "egressFrom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Defines conditions on the source of a request causing this EgressPolicy to apply."]
    pub egress_from:
        ::std::option::Option<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1EgressFrom>>,
    #[serde(rename = "egressTo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Defines the conditions on the ApiOperation and destination resources that cause this EgressPolicy to apply."]
    pub egress_to:
        ::std::option::Option<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1EgressTo>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines the conditions under which an EgressPolicy matches a request. Conditions are based on information about the ApiOperation intended to be performed on the `resources` specified. Note that if the destination of the request is protected by a ServicePerimeter, then that ServicePerimeter must have an IngressPolicy which allows access in order for this request to succeed."]
pub struct GoogleIdentityAccesscontextmanagerV1EgressTo {
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of ApiOperations that this egress rule applies to. A request matches if it contains an operation/service in this list."]
    pub operations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1ApiOperation>>,
    >,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of resources, currently only projects in the form `projects/`, that match this to stanza. A request matches if it contains a resource in this list. If `*` is specified for resources, then this EgressTo rule will authorize access to all resources outside the perimeter."]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines the conditions under which an IngressPolicy matches a request. Conditions are based on information about the source of the request."]
pub struct GoogleIdentityAccesscontextmanagerV1IngressFrom {
    #[serde(rename = "identities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of identities that are allowed access through this ingress policy. Should be in the format of email address. The email address should represent individual user or service account only."]
    pub identities: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "identityType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the type of identities that are allowed access from outside the perimeter. If left unspecified, then members of `identities` field will be allowed access."]
    pub identity_type:
        ::std::option::Option<GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum>,
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sources that this IngressPolicy authorizes access from."]
    pub sources: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1IngressSource>>,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Specifies the type of identities that are allowed access from outside the perimeter. If left unspecified, then members of `identities` field will be allowed access."]
pub enum GoogleIdentityAccesscontextmanagerV1IngressFromIdentityTypeEnum {
    #[serde(rename = "IDENTITY_TYPE_UNSPECIFIED")]
    #[doc = "No blanket identity group specified."]
    IdentityTypeUnspecified,
    #[serde(rename = "ANY_IDENTITY")]
    #[doc = "Authorize access from all identities outside the perimeter."]
    AnyIdentity,
    #[serde(rename = "ANY_USER_ACCOUNT")]
    #[doc = "Authorize access from all human users outside the perimeter."]
    AnyUserAccount,
    #[serde(rename = "ANY_SERVICE_ACCOUNT")]
    #[doc = "Authorize access from all service accounts outside the perimeter."]
    AnyServiceAccount,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Policy for ingress into ServicePerimeter. IngressPolicies match requests based on `ingress_from` and `ingress_to` stanzas. For an ingress policy to match, both the `ingress_from` and `ingress_to` stanzas must be matched. If an IngressPolicy matches a request, the request is allowed through the perimeter boundary from outside the perimeter. For example, access from the internet can be allowed either based on an AccessLevel or, for traffic hosted on Google Cloud, the project of the source network. For access from private networks, using the project of the hosting network is required. Individual ingress policies can be limited by restricting which services and/or actions they match using the `ingress_to` field."]
pub struct GoogleIdentityAccesscontextmanagerV1IngressPolicy {
    #[serde(rename = "ingressFrom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Defines the conditions on the source of a request causing this IngressPolicy to apply."]
    pub ingress_from:
        ::std::option::Option<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1IngressFrom>>,
    #[serde(rename = "ingressTo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Defines the conditions on the ApiOperation and request destination that cause this IngressPolicy to apply."]
    pub ingress_to:
        ::std::option::Option<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1IngressTo>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The source that IngressPolicy authorizes access from."]
pub struct GoogleIdentityAccesscontextmanagerV1IngressSource {
    #[serde(rename = "accessLevel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An AccessLevel resource name that allow resources within the ServicePerimeters to be accessed from the internet. AccessLevels listed must be in the same policy as this ServicePerimeter. Referencing a nonexistent AccessLevel will cause an error. If no AccessLevel names are listed, resources within the perimeter can only be accessed via Google Cloud calls with request origins within the perimeter. Example: `accessPolicies/MY_POLICY/accessLevels/MY_LEVEL`. If `*` is specified, then all IngressSources will be allowed."]
    pub access_level: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A Google Cloud resource that is allowed to ingress the perimeter. Requests from these resources will be allowed to access perimeter data. Currently only projects are allowed. Format: `projects/{project_number}` The project may be in any Google Cloud organization, not just the organization that the perimeter is defined in. `*` is not allowed, the case of allowing all Google Cloud resources only is not supported."]
    pub resource: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines the conditions under which an IngressPolicy matches a request. Conditions are based on information about the ApiOperation intended to be performed on the destination of the request."]
pub struct GoogleIdentityAccesscontextmanagerV1IngressTo {
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of ApiOperations the sources specified in corresponding IngressFrom are allowed to perform in this ServicePerimeter."]
    pub operations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1ApiOperation>>,
    >,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of resources, currently only projects in the form `projects/`, protected by this ServicePerimeter that are allowed to be accessed by sources defined in the corresponding IngressFrom. A request matches if it contains a resource in this list. If `*` is specified for resources, then this IngressTo rule will authorize access to all resources inside the perimeter, provided that the request also matches the `operations` field."]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An allowed method or permission of a service specified in ApiOperation."]
pub struct GoogleIdentityAccesscontextmanagerV1MethodSelector {
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Value for `method` should be a valid method name for the corresponding `service_name` in ApiOperation. If `*` used as value for `method`, then ALL methods and permissions are allowed."]
    pub method: ::std::option::Option<::std::string::String>,
    #[serde(rename = "permission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Value for `permission` should be a valid Cloud IAM permission for the corresponding `service_name` in ApiOperation."]
    pub permission: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A restriction on the OS type and version of devices making requests."]
pub struct GoogleIdentityAccesscontextmanagerV1OsConstraint {
    #[serde(rename = "minimumVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minimum allowed OS version. If not set, any version of this OS satisfies the constraint. Format: `\"major.minor.patch\"`. Examples: `\"10.5.301\"`, `\"9.2.1\"`."]
    pub minimum_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "osType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The allowed OS type."]
    pub os_type: ::std::option::Option<GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum>,
    #[serde(rename = "requireVerifiedChromeOs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only allows requests from devices with a verified Chrome OS. Verifications includes requirements that the device is enterprise-managed, conformant to domain policies, and the caller has permission to call the API targeted by the request."]
    pub require_verified_chrome_os: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. The allowed OS type."]
pub enum GoogleIdentityAccesscontextmanagerV1OsConstraintOsTypeEnum {
    #[serde(rename = "OS_UNSPECIFIED")]
    #[doc = "The operating system of the device is not specified or not known."]
    OsUnspecified,
    #[serde(rename = "DESKTOP_MAC")]
    #[doc = "A desktop Mac operating system."]
    DesktopMac,
    #[serde(rename = "DESKTOP_WINDOWS")]
    #[doc = "A desktop Windows operating system."]
    DesktopWindows,
    #[serde(rename = "DESKTOP_LINUX")]
    #[doc = "A desktop Linux operating system."]
    DesktopLinux,
    #[serde(rename = "DESKTOP_CHROME_OS")]
    #[doc = "A desktop ChromeOS operating system."]
    DesktopChromeOs,
    #[serde(rename = "ANDROID")]
    #[doc = "An Android operating system."]
    Android,
    #[serde(rename = "IOS")]
    #[doc = "An iOS operating system."]
    Ios,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`ServicePerimeter` describes a set of Google Cloud resources which can freely import and export data amongst themselves, but not export outside of the `ServicePerimeter`. If a request with a source within this `ServicePerimeter` has a target outside of the `ServicePerimeter`, the request will be blocked. Otherwise the request is allowed. There are two types of Service Perimeter - Regular and Bridge. Regular Service Perimeters cannot overlap, a single Google Cloud project can only belong to a single regular Service Perimeter. Service Perimeter Bridges can contain only Google Cloud projects as members, a single Google Cloud project may belong to multiple Service Perimeter Bridges."]
pub struct GoogleIdentityAccesscontextmanagerV1ServicePerimeter {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the `ServicePerimeter` and its use. Does not affect behavior."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Resource name for the ServicePerimeter. The `short_name` component must begin with a letter and only include alphanumeric and '_'. Format: `accessPolicies/{policy_id}/servicePerimeters/{short_name}`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "perimeterType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Perimeter type indicator. A single project is allowed to be a member of single regular perimeter, but multiple service perimeter bridges. A project cannot be a included in a perimeter bridge without being included in regular perimeter. For perimeter bridges, the restricted service list as well as access level lists must be empty."]
    pub perimeter_type: ::std::option::Option<
        GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterTypeEnum,
    >,
    #[serde(rename = "spec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Proposed (or dry run) ServicePerimeter configuration. This configuration allows to specify and test ServicePerimeter configuration without enforcing actual access restrictions. Only allowed to be set when the \"use_explicit_dry_run_spec\" flag is set."]
    pub spec: ::std::option::Option<
        ::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1ServicePerimeterConfig>,
    >,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Current ServicePerimeter configuration. Specifies sets of resources, restricted services and access levels that determine perimeter content and boundaries."]
    pub status: ::std::option::Option<
        ::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1ServicePerimeterConfig>,
    >,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Human readable title. Must be unique within the Policy."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "useExplicitDryRunSpec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Use explicit dry run spec flag. Ordinarily, a dry-run spec implicitly exists for all Service Perimeters, and that spec is identical to the status for those Service Perimeters. When this flag is set, it inhibits the generation of the implicit spec, thereby allowing the user to explicitly provide a configuration (\"spec\") to use in a dry-run version of the Service Perimeter. This allows the user to test changes to the enforced config (\"status\") without actually enforcing them. This testing is done through analyzing the differences between currently enforced and suggested restrictions. use_explicit_dry_run_spec must bet set to True if any of the fields in the spec are set to non-default values."]
    pub use_explicit_dry_run_spec: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Perimeter type indicator. A single project is allowed to be a member of single regular perimeter, but multiple service perimeter bridges. A project cannot be a included in a perimeter bridge without being included in regular perimeter. For perimeter bridges, the restricted service list as well as access level lists must be empty."]
pub enum GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterTypeEnum {
    #[serde(rename = "PERIMETER_TYPE_REGULAR")]
    #[doc = "Regular Perimeter."]
    PerimeterTypeRegular,
    #[serde(rename = "PERIMETER_TYPE_BRIDGE")]
    #[doc = "Perimeter Bridge."]
    PerimeterTypeBridge,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`ServicePerimeterConfig` specifies a set of Google Cloud resources that describe specific Service Perimeter configuration."]
pub struct GoogleIdentityAccesscontextmanagerV1ServicePerimeterConfig {
    #[serde(rename = "accessLevels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of `AccessLevel` resource names that allow resources within the `ServicePerimeter` to be accessed from the internet. `AccessLevels` listed must be in the same policy as this `ServicePerimeter`. Referencing a nonexistent `AccessLevel` is a syntax error. If no `AccessLevel` names are listed, resources within the perimeter can only be accessed via Google Cloud calls with request origins within the perimeter. Example: `\"accessPolicies/MY_POLICY/accessLevels/MY_LEVEL\"`. For Service Perimeter Bridge, must be empty."]
    pub access_levels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "egressPolicies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of EgressPolicies to apply to the perimeter. A perimeter may have multiple EgressPolicies, each of which is evaluated separately. Access is granted if any EgressPolicy grants it. Must be empty for a perimeter bridge."]
    pub egress_policies: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1EgressPolicy>>,
    >,
    #[serde(rename = "ingressPolicies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of IngressPolicies to apply to the perimeter. A perimeter may have multiple IngressPolicies, each of which is evaluated separately. Access is granted if any Ingress Policy grants it. Must be empty for a perimeter bridge."]
    pub ingress_policies: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1IngressPolicy>>,
    >,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of Google Cloud resources that are inside of the service perimeter. Currently only projects are allowed. Format: `projects/{project_number}`"]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "restrictedServices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Google Cloud services that are subject to the Service Perimeter restrictions. For example, if `storage.googleapis.com` is specified, access to the storage buckets inside the perimeter must meet the perimeter's access restrictions."]
    pub restricted_services: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "vpcAccessibleServices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration for APIs allowed within Perimeter."]
    pub vpc_accessible_services: ::std::option::Option<
        ::std::boxed::Box<GoogleIdentityAccesscontextmanagerV1VpcAccessibleServices>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies how APIs are allowed to communicate within the Service Perimeter."]
pub struct GoogleIdentityAccesscontextmanagerV1VpcAccessibleServices {
    #[serde(rename = "allowedServices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of APIs usable within the Service Perimeter. Must be empty unless 'enable_restriction' is True. You can specify a list of individual services, as well as include the 'RESTRICTED-SERVICES' value, which automatically includes all of the services protected by the perimeter."]
    pub allowed_services: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "enableRestriction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to restrict API calls within the Service Perimeter to the list of APIs specified in 'allowed_services'."]
    pub enable_restriction: ::std::option::Option<::std::primitive::bool>,
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
    #[serde(rename = "nonCriticalErrors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of non-critical errors happened during the query handling."]
    pub non_critical_errors:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<IamPolicyAnalysisState>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Output configuration for export IAM policy analysis destination."]
pub struct IamPolicyAnalysisOutputConfig {
    #[serde(rename = "bigqueryDestination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Destination on BigQuery."]
    pub bigquery_destination:
        ::std::option::Option<::std::boxed::Box<GoogleCloudAssetV1BigQueryDestination>>,
    #[serde(rename = "gcsDestination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Destination on Cloud Storage."]
    pub gcs_destination: ::std::option::Option<::std::boxed::Box<GoogleCloudAssetV1GcsDestination>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "## IAM policy analysis query message."]
pub struct IamPolicyAnalysisQuery {
    #[serde(rename = "accessSelector")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Specifies roles or permissions for analysis. This is optional."]
    pub access_selector: ::std::option::Option<::std::boxed::Box<AccessSelector>>,
    #[serde(rename = "identitySelector")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Specifies an identity for analysis."]
    pub identity_selector: ::std::option::Option<::std::boxed::Box<IdentitySelector>>,
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The query options."]
    pub options: ::std::option::Option<::std::boxed::Box<Options>>,
    #[serde(rename = "resourceSelector")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Specifies a resource for analysis."]
    pub resource_selector: ::std::option::Option<::std::boxed::Box<ResourceSelector>>,
    #[serde(rename = "scope")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The relative name of the root asset. Only resources and IAM policies within the scope will be analyzed. This can only be an organization number (such as \"organizations/123\"), a folder number (such as \"folders/123\"), a project ID (such as \"projects/my-project-id\"), or a project number (such as \"projects/12345\"). To know how to get organization id, visit [here ](https://cloud.google.com/resource-manager/docs/creating-managing-organization#retrieving_your_organization_id). To know how to get folder or project id, visit [here ](https://cloud.google.com/resource-manager/docs/creating-managing-folders#viewing_or_listing_folders_and_projects)."]
    pub scope: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "IAM Policy analysis result, consisting of one IAM policy binding and derived access control lists."]
pub struct IamPolicyAnalysisResult {
    #[serde(rename = "accessControlLists")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The access control lists derived from the iam_binding that match or potentially match resource and access selectors specified in the request."]
    pub access_control_lists: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudAssetV1AccessControlList>>,
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
    pub identity_list: ::std::option::Option<::std::boxed::Box<GoogleCloudAssetV1IdentityList>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the detailed state of an entity under analysis, such as a resource, an identity or an access."]
pub struct IamPolicyAnalysisState {
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The human-readable description of the cause of failure."]
    pub cause: ::std::option::Option<::std::string::String>,
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Google standard error code that best describes the state. For example: - OK means the analysis on this entity has been successfully finished; - PERMISSION_DENIED means an access denied error is encountered; - DEADLINE_EXCEEDED means the analysis on this entity hasn't been started in time;"]
    pub code: ::std::option::Option<IamPolicyAnalysisStateCodeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The Google standard error code that best describes the state. For example: - OK means the analysis on this entity has been successfully finished; - PERMISSION_DENIED means an access denied error is encountered; - DEADLINE_EXCEEDED means the analysis on this entity hasn't been started in time;"]
pub enum IamPolicyAnalysisStateCodeEnum {
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
#[doc = "A result of IAM Policy search, containing information of an IAM policy."]
pub struct IamPolicySearchResult {
    #[serde(rename = "explanation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Explanation about the IAM policy search result. It contains additional information to explain why the search result matches the query."]
    pub explanation: ::std::option::Option<::std::boxed::Box<Explanation>>,
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IAM policy directly set on the given resource. Note that the original IAM policy can contain multiple bindings. This only contains the bindings that match the given query. For queries that don't contain a constrain on policies (e.g., an empty query), this contains all the bindings. To search against the `policy` bindings: * use a field query: - query by the policy contained members. Example: `policy:amy@gmail.com` - query by the policy contained roles. Example: `policy:roles/compute.admin` - query by the policy contained roles' included permissions. Example: `policy.role.permissions:compute.instances.create`"]
    pub policy: ::std::option::Option<::std::boxed::Box<Policy>>,
    #[serde(rename = "project")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The project that the associated GCP resource belongs to, in the form of projects/{PROJECT_NUMBER}. If an IAM policy is set on a resource (like VM instance, Cloud Storage bucket), the project field will indicate the project that contains the resource. If an IAM policy is set on a folder or orgnization, this field will be empty. To search against the `project`: * specify the `scope` field as this project in your search request."]
    pub project: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full resource name of the resource associated with this IAM policy. Example: `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`. See [Cloud Asset Inventory Resource Name Format](https://cloud.google.com/asset-inventory/docs/resource-name-format) for more information. To search against the `resource`: * use a field query. Example: `resource:organizations/123`"]
    pub resource: ::std::option::Option<::std::string::String>,
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
#[doc = "The inventory details of a VM."]
pub struct Inventory {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inventory items related to the VM keyed by an opaque unique identifier for each inventory item. The identifier is unique to each distinct and addressable inventory item and will change, when there is a new package version."]
    pub items: ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<Item>>>,
    #[serde(rename = "osInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Base level operating system information for the VM."]
    pub os_info: ::std::option::Option<::std::boxed::Box<OsInfo>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single piece of inventory on a VM."]
pub struct Item {
    #[serde(rename = "availablePackage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Software package available to be installed on the VM instance."]
    pub available_package: ::std::option::Option<::std::boxed::Box<SoftwarePackage>>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When this inventory item was first detected."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier for this item, unique across items for this VM."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "installedPackage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Software package present on the VM instance."]
    pub installed_package: ::std::option::Option<::std::boxed::Box<SoftwarePackage>>,
    #[serde(rename = "originType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The origin of this inventory item."]
    pub origin_type: ::std::option::Option<ItemOriginTypeEnum>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The specific type of inventory, correlating to its specific details."]
    pub _type: ::std::option::Option<ItemTypeEnum>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When this inventory item was last modified."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The origin of this inventory item."]
pub enum ItemOriginTypeEnum {
    #[serde(rename = "ORIGIN_TYPE_UNSPECIFIED")]
    #[doc = "Invalid. An origin type must be specified."]
    OriginTypeUnspecified,
    #[serde(rename = "INVENTORY_REPORT")]
    #[doc = "This inventory item was discovered as the result of the agent reporting inventory via the reporting API."]
    InventoryReport,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The specific type of inventory, correlating to its specific details."]
pub enum ItemTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "Invalid. An type must be specified."]
    TypeUnspecified,
    #[serde(rename = "INSTALLED_PACKAGE")]
    #[doc = "This represents a package that is installed on the VM."]
    InstalledPackage,
    #[serde(rename = "AVAILABLE_PACKAGE")]
    #[doc = "This represents an update that is available for a package."]
    AvailablePackage,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListFeedsResponse {
    #[serde(rename = "feeds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of feeds."]
    pub feeds: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Feed>>>,
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
#[doc = "Contains query options."]
pub struct Options {
    #[serde(rename = "analyzeServiceAccountImpersonation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If true, the response will include access analysis from identities to resources via service account impersonation. This is a very expensive operation, because many derived queries will be executed. We highly recommend you use AssetService.AnalyzeIamPolicyLongrunning rpc instead. For example, if the request analyzes for which resources user A has permission P, and there's an IAM policy states user A has iam.serviceAccounts.getAccessToken permission to a service account SA, and there's another IAM policy states service account SA has permission P to a GCP folder F, then user A potentially has access to the GCP folder F. And those advanced analysis results will be included in AnalyzeIamPolicyResponse.service_account_impersonation_analysis. Another example, if the request analyzes for who has permission P to a GCP folder F, and there's an IAM policy states user A has iam.serviceAccounts.actAs permission to a service account SA, and there's another IAM policy states service account SA has permission P to the GCP folder F, then user A potentially has access to the GCP folder F. And those advanced analysis results will be included in AnalyzeIamPolicyResponse.service_account_impersonation_analysis. Default is false."]
    pub analyze_service_account_impersonation: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "expandGroups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If true, the identities section of the result will expand any Google groups appearing in an IAM policy binding. If IamPolicyAnalysisQuery.identity_selector is specified, the identity in the result will be determined by the selector, and this flag is not allowed to set. Default is false."]
    pub expand_groups: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "expandResources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If true and IamPolicyAnalysisQuery.resource_selector is not specified, the resource section of the result will expand any resource attached to an IAM policy to include resources lower in the resource hierarchy. For example, if the request analyzes for which resources user A has permission P, and the results include an IAM policy with P on a GCP folder, the results will also include resources in that folder with permission P. If true and IamPolicyAnalysisQuery.resource_selector is specified, the resource section of the result will expand the specified resource to include resources lower in the resource hierarchy. Only project or lower resources are supported. Folder and organization resource cannot be used together with this option. For example, if the request analyzes for which users have permission P on a GCP project with this option enabled, the results will include all users who have permission P on that project or any lower resource. Default is false."]
    pub expand_resources: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "expandRoles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If true, the access section of result will expand any roles appearing in IAM policy bindings to include their permissions. If IamPolicyAnalysisQuery.access_selector is specified, the access section of the result will be determined by the selector, and this flag is not allowed to set. Default is false."]
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
#[doc = "Operating system information for the VM."]
pub struct OsInfo {
    #[serde(rename = "architecture")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The system architecture of the operating system."]
    pub architecture: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hostname")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The VM hostname."]
    pub hostname: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kernelRelease")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kernel release of the operating system."]
    pub kernel_release: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kernelVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kernel version of the operating system."]
    pub kernel_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "longName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The operating system long name. For example 'Debian GNU/Linux 9' or 'Microsoft Window Server 2019 Datacenter'."]
    pub long_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "osconfigAgentVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current version of the OS Config agent running on the VM."]
    pub osconfig_agent_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shortName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The operating system short name. For example, 'windows' or 'debian'."]
    pub short_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version of the operating system."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Output configuration for export assets destination."]
pub struct OutputConfig {
    #[serde(rename = "bigqueryDestination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Destination on BigQuery. The output table stores the fields in asset proto as columns in BigQuery."]
    pub bigquery_destination: ::std::option::Option<::std::boxed::Box<BigQueryDestination>>,
    #[serde(rename = "gcsDestination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Destination on Cloud Storage."]
    pub gcs_destination: ::std::option::Option<::std::boxed::Box<GcsDestination>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifications of BigQuery partitioned table as export destination."]
pub struct PartitionSpec {
    #[serde(rename = "partitionKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The partition key for BigQuery partitioned table."]
    pub partition_key: ::std::option::Option<PartitionSpecPartitionKeyEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The partition key for BigQuery partitioned table."]
pub enum PartitionSpecPartitionKeyEnum {
    #[serde(rename = "PARTITION_KEY_UNSPECIFIED")]
    #[doc = "Unspecified partition key. If used, it means using non-partitioned table."]
    PartitionKeyUnspecified,
    #[serde(rename = "READ_TIME")]
    #[doc = "The time when the snapshot is taken. If specified as partition key, the result table(s) is partitoned by the additional timestamp column, readTime. If [read_time] in ExportAssetsRequest is specified, the readTime column's value will be the same as it. Otherwise, its value will be the current time that is used to take the snapshot."]
    ReadTime,
    #[serde(rename = "REQUEST_TIME")]
    #[doc = "The time when the request is received and started to be processed. If specified as partition key, the result table(s) is partitoned by the requestTime column, an additional timestamp column representing when the request was received."]
    RequestTime,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "IAM permissions"]
pub struct Permissions {
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of permissions. A sample permission string: `compute.disk.get`."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
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
#[doc = "A Pub/Sub destination."]
pub struct PubsubDestination {
    #[serde(rename = "topic")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the Pub/Sub topic to publish to. Example: `projects/PROJECT_ID/topics/TOPIC_ID`."]
    pub topic: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A representation of a Google Cloud resource."]
pub struct Resource {
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content of the resource, in which some sensitive fields are removed and may not be present."]
    pub data: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "discoveryDocumentUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the discovery document containing the resource's JSON schema. Example: `https://www.googleapis.com/discovery/v1/apis/compute/v1/rest` This value is unspecified for resources that do not have an API based on a discovery document, such as Cloud Bigtable."]
    pub discovery_document_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "discoveryName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The JSON schema name listed in the discovery document. Example: `Project` This value is unspecified for resources that do not have an API based on a discovery document, such as Cloud Bigtable."]
    pub discovery_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The location of the resource in Google Cloud, such as its zone and region. For more information, see https://cloud.google.com/about/locations/."]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full name of the immediate parent of this resource. See [Resource Names](https://cloud.google.com/apis/design/resource_names#full_resource_name) for more information. For Google Cloud assets, this value is the parent resource defined in the [Cloud IAM policy hierarchy](https://cloud.google.com/iam/docs/overview#policy_hierarchy). Example: `//cloudresourcemanager.googleapis.com/projects/my_project_123` For third-party assets, this field may be set differently."]
    pub parent: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The REST URL for accessing the resource. An HTTP `GET` request using this URL returns the resource itself. Example: `https://cloudresourcemanager.googleapis.com/v1/projects/my-project-123` This value is unspecified for resources without a REST API."]
    pub resource_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The API version. Example: `v1`"]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A result of Resource Search, containing information of a cloud resource."]
pub struct ResourceSearchResult {
    #[serde(rename = "additionalAttributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The additional searchable attributes of this resource. The attributes may vary from one resource type to another. Examples: `projectId` for Project, `dnsName` for DNS ManagedZone. This field contains a subset of the resource metadata fields that are returned by the List or Get APIs provided by the corresponding GCP service (e.g., Compute Engine). see [API references and supported searchable attributes](https://cloud.google.com/asset-inventory/docs/supported-asset-types#searchable_asset_types) to see which fields are included. You can search values of these fields through free text search. However, you should not consume the field programically as the field names and values may change as the GCP service updates to a new incompatible API version. To search against the `additional_attributes`: * use a free text query to match the attributes values. Example: to search `additional_attributes = { dnsName: \"foobar\" }`, you can issue a query `foobar`."]
    pub additional_attributes:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "assetType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of this resource. Example: `compute.googleapis.com/Disk`. To search against the `asset_type`: * specify the `asset_type` field in your search request."]
    pub asset_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The create timestamp of this resource, at which the resource was created. The granularity is in seconds. Timestamp.nanos will always be 0. This field is available only when the resource's proto contains it. To search against `create_time`: * use a field query (value in seconds). Example: `createTime >= 1594294238`"]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "One or more paragraphs of text description of this resource. Maximum length could be up to 1M bytes. This field is available only when the resource's proto contains it. To search against the `description`: * use a field query. Example: `description:\"important instance\"` * use a free text query. Example: `\"important instance\"`"]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display name of this resource. This field is available only when the resource's proto contains it. To search against the `display_name`: * use a field query. Example: `displayName:\"My Instance\"` * use a free text query. Example: `\"My Instance\"`"]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "folders")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The folder(s) that this resource belongs to, in the form of folders/{FOLDER_NUMBER}. This field is available when the resource belongs to one or more folders. To search against `folders`: * use a field query. Example: `folders:(123 OR 456)` * use a free text query. Example: `123` * specify the `scope` field as this folder in your search request."]
    pub folders: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "kmsKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Cloud KMS [CryptoKey](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys?hl=en) name or [CryptoKeyVersion](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys.cryptoKeyVersions?hl=en) name. This field is available only when the resource's proto contains it. To search against the `kms_key`: * use a field query. Example: `kmsKey:key` * use a free text query. Example: `key`"]
    pub kms_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Labels associated with this resource. See [Labelling and grouping GCP resources](https://cloud.google.com/blog/products/gcp/labelling-and-grouping-your-google-cloud-platform-resources) for more information. This field is available only when the resource's proto contains it. To search against the `labels`: * use a field query: - query on any label's key or value. Example: `labels:prod` - query by a given label. Example: `labels.env:prod` - query by a given label's existence. Example: `labels.env:*` * use a free text query. Example: `prod`"]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Location can be `global`, regional like `us-east1`, or zonal like `us-west1-b`. This field is available only when the resource's proto contains it. To search against the `location`: * use a field query. Example: `location:us-west*` * use a free text query. Example: `us-west*`"]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full resource name of this resource. Example: `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`. See [Cloud Asset Inventory Resource Name Format](https://cloud.google.com/asset-inventory/docs/resource-name-format) for more information. To search against the `name`: * use a field query. Example: `name:instance1` * use a free text query. Example: `instance1`"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "networkTags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Network tags associated with this resource. Like labels, network tags are a type of annotations used to group GCP resources. See [Labelling GCP resources](https://cloud.google.com/blog/products/gcp/labelling-and-grouping-your-google-cloud-platform-resources) for more information. This field is available only when the resource's proto contains it. To search against the `network_tags`: * use a field query. Example: `networkTags:internal` * use a free text query. Example: `internal`"]
    pub network_tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "organization")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The organization that this resource belongs to, in the form of organizations/{ORGANIZATION_NUMBER}. This field is available when the resource belongs to an organization. To search against `organization`: * use a field query. Example: `organization:123` * use a free text query. Example: `123` * specify the `scope` field as this organization in your search request."]
    pub organization: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parentAssetType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of this resource's immediate parent, if there is one. To search against the `parent_asset_type`: * use a field query. Example: `parentAssetType:\"cloudresourcemanager.googleapis.com/Project\"` * use a free text query. Example: `cloudresourcemanager.googleapis.com/Project`"]
    pub parent_asset_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parentFullResourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full resource name of this resource's parent, if it has one."]
    pub parent_full_resource_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "project")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The project that this resource belongs to, in the form of projects/{PROJECT_NUMBER}. This field is available when the resource belongs to a project. To search against `project`: * use a field query. Example: `project:12345` * use a free text query. Example: `12345` * specify the `scope` field as this project in your search request."]
    pub project: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state of this resource. Different resources types have different state definitions that are mapped from various fields of different resource types. This field is available only when the resource's proto contains it. Example: If the resource is an instance provided by Compute Engine, its state will include PROVISIONING, STAGING, RUNNING, STOPPING, SUSPENDING, SUSPENDED, REPAIRING, and TERMINATED. See `status` definition in [API Reference](https://cloud.google.com/compute/docs/reference/rest/v1/instances). If the resource is a project provided by Cloud Resource Manager, its state will include LIFECYCLE_STATE_UNSPECIFIED, ACTIVE, DELETE_REQUESTED and DELETE_IN_PROGRESS. See `lifecycleState` definition in [API Reference](https://cloud.google.com/resource-manager/reference/rest/v1/projects). To search against the `state`: * use a field query. Example: `state:RUNNING` * use a free text query. Example: `RUNNING`"]
    pub state: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last update timestamp of this resource, at which the resource was last modified or deleted. The granularity is in seconds. Timestamp.nanos will always be 0. This field is available only when the resource's proto contains it. To search against `update_time`: * use a field query (value in seconds). Example: `updateTime < 1594294238`"]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifies the resource to analyze for access policies, which may be set directly on the resource, or on ancestors such as organizations, folders or projects."]
pub struct ResourceSelector {
    #[serde(rename = "fullResourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The [full resource name] (https://cloud.google.com/asset-inventory/docs/resource-name-format) of a resource of [supported resource types](https://cloud.google.com/asset-inventory/docs/supported-asset-types#analyzable_asset_types)."]
    pub full_resource_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Search all IAM policies response."]
pub struct SearchAllIamPoliciesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set if there are more results than those appearing in this response; to get the next set of results, call this method again, using this value as the `page_token`."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of IamPolicy that match the search query. Related information such as the associated resource is returned along with the policy."]
    pub results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<IamPolicySearchResult>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Search all resources response."]
pub struct SearchAllResourcesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If there are more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token`."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of Resources that match the search query. It contains the resource standard metadata information."]
    pub results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ResourceSearchResult>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Software package information of the operating system."]
pub struct SoftwarePackage {
    #[serde(rename = "aptPackage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details of an APT package. For details about the apt package manager, see https://wiki.debian.org/Apt."]
    pub apt_package: ::std::option::Option<::std::boxed::Box<VersionedPackage>>,
    #[serde(rename = "cosPackage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details of a COS package."]
    pub cos_package: ::std::option::Option<::std::boxed::Box<VersionedPackage>>,
    #[serde(rename = "googetPackage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details of a Googet package. For details about the googet package manager, see https://github.com/google/googet."]
    pub googet_package: ::std::option::Option<::std::boxed::Box<VersionedPackage>>,
    #[serde(rename = "qfePackage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details of a Windows Quick Fix engineering package. See https://docs.microsoft.com/en-us/windows/win32/cimwin32prov/win32-quickfixengineering for info in Windows Quick Fix Engineering."]
    pub qfe_package: ::std::option::Option<::std::boxed::Box<WindowsQuickFixEngineeringPackage>>,
    #[serde(rename = "wuaPackage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details of a Windows Update package. See https://docs.microsoft.com/en-us/windows/win32/api/_wua/ for information about Windows Update."]
    pub wua_package: ::std::option::Option<::std::boxed::Box<WindowsUpdatePackage>>,
    #[serde(rename = "yumPackage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Yum package info. For details about the yum package manager, see https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/6/html/deployment_guide/ch-yum."]
    pub yum_package: ::std::option::Option<::std::boxed::Box<VersionedPackage>>,
    #[serde(rename = "zypperPackage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details of a Zypper package. For details about the Zypper package manager, see https://en.opensuse.org/SDB:Zypper_manual."]
    pub zypper_package: ::std::option::Option<::std::boxed::Box<VersionedPackage>>,
    #[serde(rename = "zypperPatch")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details of a Zypper patch. For details about the Zypper package manager, see https://en.opensuse.org/SDB:Zypper_manual."]
    pub zypper_patch: ::std::option::Option<::std::boxed::Box<ZypperPatch>>,
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
#[doc = "An asset in Google Cloud and its temporal metadata, including the time window when it was observed and its status during that window."]
pub struct TemporalAsset {
    #[serde(rename = "asset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An asset in Google Cloud."]
    pub asset: ::std::option::Option<::std::boxed::Box<Asset>>,
    #[serde(rename = "deleted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the asset has been deleted or not."]
    pub deleted: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "priorAsset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Prior copy of the asset. Populated if prior_asset_state is PRESENT. Currently this is only set for responses in Real-Time Feed."]
    pub prior_asset: ::std::option::Option<::std::boxed::Box<Asset>>,
    #[serde(rename = "priorAssetState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "State of prior_asset."]
    pub prior_asset_state: ::std::option::Option<TemporalAssetPriorAssetStateEnum>,
    #[serde(rename = "window")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time window when the asset data and state was observed."]
    pub window: ::std::option::Option<::std::boxed::Box<TimeWindow>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "State of prior_asset."]
pub enum TemporalAssetPriorAssetStateEnum {
    #[serde(rename = "PRIOR_ASSET_STATE_UNSPECIFIED")]
    #[doc = "prior_asset is not applicable for the current asset."]
    PriorAssetStateUnspecified,
    #[serde(rename = "PRESENT")]
    #[doc = "prior_asset is populated correctly."]
    Present,
    #[serde(rename = "INVALID")]
    #[doc = "Failed to set prior_asset."]
    Invalid,
    #[serde(rename = "DOES_NOT_EXIST")]
    #[doc = "Current asset is the first known state."]
    DoesNotExist,
    #[serde(rename = "DELETED")]
    #[doc = "prior_asset is a deletion."]
    Deleted,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A time window specified by its `start_time` and `end_time`."]
pub struct TimeWindow {
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "End time of the time window (inclusive). If not specified, the current timestamp is used instead."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Start time of the time window (exclusive)."]
    pub start_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Update asset feed request."]
pub struct UpdateFeedRequest {
    #[serde(rename = "feed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The new values of feed details. It must match an existing feed and the field `name` must be in the format of: projects/project_number/feeds/feed_id or folders/folder_number/feeds/feed_id or organizations/organization_number/feeds/feed_id."]
    pub feed: ::std::option::Option<::std::boxed::Box<Feed>>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Only updates the `feed` fields indicated by this mask. The field mask must not be empty, and it must not contain fields that are immutable or only set by the server."]
    pub update_mask: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information related to the a standard versioned package. This includes package info for APT, Yum, Zypper, and Googet package managers."]
pub struct VersionedPackage {
    #[serde(rename = "architecture")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The system architecture this package is intended for."]
    pub architecture: ::std::option::Option<::std::string::String>,
    #[serde(rename = "packageName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the package."]
    pub package_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version of the package."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information related to a Quick Fix Engineering package. Fields are taken from Windows QuickFixEngineering Interface and match the source names: https://docs.microsoft.com/en-us/windows/win32/cimwin32prov/win32-quickfixengineering"]
pub struct WindowsQuickFixEngineeringPackage {
    #[serde(rename = "caption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A short textual description of the QFE update."]
    pub caption: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A textual description of the QFE update."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hotFixId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier associated with a particular QFE update."]
    pub hot_fix_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "installTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date that the QFE update was installed. Mapped from installed_on field."]
    pub install_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Categories specified by the Windows Update."]
pub struct WindowsUpdateCategory {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier of the windows update category."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the windows update category."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details related to a Windows Update package. Field data and names are taken from Windows Update API IUpdate Interface: https://docs.microsoft.com/en-us/windows/win32/api/_wua/ Descriptive fields like title, and description are localized based on the locale of the VM being updated."]
pub struct WindowsUpdatePackage {
    #[serde(rename = "categories")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The categories that are associated with this update package."]
    pub categories:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WindowsUpdateCategory>>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The localized description of the update package."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kbArticleIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A collection of Microsoft Knowledge Base article IDs that are associated with the update package."]
    pub kb_article_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "lastDeploymentChangeTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last published date of the update, in (UTC) date and time."]
    pub last_deployment_change_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "moreInfoUrls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A collection of URLs that provide more information about the update package."]
    pub more_info_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "revisionNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The revision number of this update package."]
    pub revision_number: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "supportUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A hyperlink to the language-specific support information for the update."]
    pub support_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The localized title of the update package."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Gets the identifier of an update package. Stays the same across revisions."]
    pub update_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Details related to a Zypper Patch."]
pub struct ZypperPatch {
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The category of the patch."]
    pub category: ::std::option::Option<::std::string::String>,
    #[serde(rename = "patchName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the patch."]
    pub patch_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The severity specified for this patch"]
    pub severity: ::std::option::Option<::std::string::String>,
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Any summary information provided about this patch."]
    pub summary: ::std::option::Option<::std::string::String>,
}
