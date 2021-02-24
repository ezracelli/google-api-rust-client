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
#[doc = "Spec for a group of BigQuery tables with name pattern `[prefix]YYYYMMDD`. Context: https://cloud.google.com/bigquery/docs/partitioned-tables#partitioning_versus_sharding"]
pub struct GoogleCloudDatacatalogV1beta1BigQueryDateShardedSpec {
    #[serde(rename = "dataset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The Data Catalog resource name of the dataset entry the current table belongs to, for example, `projects/{project_id}/locations/{location}/entrygroups/{entry_group_id}/entries/{entry_id}`."]
    pub dataset: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shardCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Total number of shards."]
    pub shard_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tablePrefix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The table name prefix of the shards. The name of any given shard is `[table_prefix]YYYYMMDD`, for example, for shard `MyTable20180101`, the `table_prefix` is `MyTable`."]
    pub table_prefix: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes a BigQuery table."]
pub struct GoogleCloudDatacatalogV1beta1BigQueryTableSpec {
    #[serde(rename = "tableSourceType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The table source type."]
    pub table_source_type:
        ::std::option::Option<GoogleCloudDatacatalogV1beta1BigQueryTableSpecTableSourceTypeEnum>,
    #[serde(rename = "tableSpec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Spec of a BigQuery table. This field should only be populated if `table_source_type` is `BIGQUERY_TABLE`."]
    pub table_spec:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDatacatalogV1beta1TableSpec>>,
    #[serde(rename = "viewSpec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Table view specification. This field should only be populated if `table_source_type` is `BIGQUERY_VIEW`."]
    pub view_spec: ::std::option::Option<::std::boxed::Box<GoogleCloudDatacatalogV1beta1ViewSpec>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The table source type."]
pub enum GoogleCloudDatacatalogV1beta1BigQueryTableSpecTableSourceTypeEnum {
    #[serde(rename = "TABLE_SOURCE_TYPE_UNSPECIFIED")]
    #[doc = "Default unknown type."]
    TableSourceTypeUnspecified,
    #[serde(rename = "BIGQUERY_VIEW")]
    #[doc = "Table view."]
    BigqueryView,
    #[serde(rename = "BIGQUERY_TABLE")]
    #[doc = "BigQuery native table."]
    BigqueryTable,
    #[serde(rename = "BIGQUERY_MATERIALIZED_VIEW")]
    #[doc = "BigQuery materialized view."]
    BigqueryMaterializedView,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Representation of a column within a schema. Columns could be nested inside other columns."]
pub struct GoogleCloudDatacatalogV1beta1ColumnSchema {
    #[serde(rename = "column")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Name of the column."]
    pub column: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Description of the column. Default value is an empty string."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A column's mode indicates whether the values in this column are required, nullable, etc. Only `NULLABLE`, `REQUIRED` and `REPEATED` are supported. Default mode is `NULLABLE`."]
    pub mode: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subcolumns")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Schema of sub-columns. A column can have zero or more sub-columns."]
    pub subcolumns: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1ColumnSchema>>,
    >,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Type of the column."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Entry Metadata. A Data Catalog Entry resource represents another resource in Google Cloud Platform (such as a BigQuery dataset or a Pub/Sub topic), or outside of Google Cloud Platform. Clients can use the `linked_resource` field in the Entry resource to refer to the original resource ID of the source system. An Entry resource contains resource details, such as its schema. An Entry can also be used to attach flexible metadata, such as a Tag."]
pub struct GoogleCloudDatacatalogV1beta1Entry {
    #[serde(rename = "bigqueryDateShardedSpec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specification for a group of BigQuery tables with name pattern `[prefix]YYYYMMDD`. Context: https://cloud.google.com/bigquery/docs/partitioned-tables#partitioning_versus_sharding."]
    pub bigquery_date_sharded_spec: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDatacatalogV1beta1BigQueryDateShardedSpec>,
    >,
    #[serde(rename = "bigqueryTableSpec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specification that applies to a BigQuery table. This is only valid on entries of type `TABLE`."]
    pub bigquery_table_spec:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDatacatalogV1beta1BigQueryTableSpec>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Entry description, which can consist of several sentences or paragraphs that describe entry contents. Default value is an empty string."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display information such as title and description. A short name to identify the entry, for example, \"Analytics Data - Jan 2011\". Default value is an empty string."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gcsFilesetSpec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specification that applies to a Cloud Storage fileset. This is only valid on entries of type FILESET."]
    pub gcs_fileset_spec:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDatacatalogV1beta1GcsFilesetSpec>>,
    #[serde(rename = "integratedSystem")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. This field indicates the entry's source system that Data Catalog integrates with, such as BigQuery or Pub/Sub."]
    pub integrated_system:
        ::std::option::Option<GoogleCloudDatacatalogV1beta1EntryIntegratedSystemEnum>,
    #[serde(rename = "linkedResource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource this metadata entry refers to. For Google Cloud Platform resources, `linked_resource` is the [full name of the resource](https://cloud.google.com/apis/design/resource_names#full_resource_name). For example, the `linked_resource` for a table resource from BigQuery is: * //bigquery.googleapis.com/projects/projectId/datasets/datasetId/tables/tableId Output only when Entry is of type in the EntryType enum. For entries with user_specified_type, this field is optional and defaults to an empty string."]
    pub linked_resource: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The Data Catalog resource name of the entry in URL format. Example: * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}/entries/{entry_id} Note that this Entry and its child resources may not actually be stored in the location in this name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "schema")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Schema of the entry. An entry might not have any schema attached to it."]
    pub schema: ::std::option::Option<::std::boxed::Box<GoogleCloudDatacatalogV1beta1Schema>>,
    #[serde(rename = "sourceSystemTimestamps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Timestamps about the underlying resource, not about this Data Catalog entry. Output only when Entry is of type in the EntryType enum. For entries with user_specified_type, this field is optional and defaults to an empty timestamp."]
    pub source_system_timestamps:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDatacatalogV1beta1SystemTimestamps>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the entry. Only used for Entries with types in the EntryType enum."]
    pub _type: ::std::option::Option<GoogleCloudDatacatalogV1beta1EntryTypeEnum>,
    #[serde(rename = "userSpecifiedSystem")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field indicates the entry's source system that Data Catalog does not integrate with. `user_specified_system` strings must begin with a letter or underscore and can only contain letters, numbers, and underscores; are case insensitive; must be at least 1 character and at most 64 characters long."]
    pub user_specified_system: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userSpecifiedType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Entry type if it does not fit any of the input-allowed values listed in `EntryType` enum above. When creating an entry, users should check the enum values first, if nothing matches the entry to be created, then provide a custom value, for example \"my_special_type\". `user_specified_type` strings must begin with a letter or underscore and can only contain letters, numbers, and underscores; are case insensitive; must be at least 1 character and at most 64 characters long. Currently, only FILESET enum value is allowed. All other entries created through Data Catalog must use `user_specified_type`."]
    pub user_specified_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. This field indicates the entry's source system that Data Catalog integrates with, such as BigQuery or Pub/Sub."]
pub enum GoogleCloudDatacatalogV1beta1EntryIntegratedSystemEnum {
    #[serde(rename = "INTEGRATED_SYSTEM_UNSPECIFIED")]
    #[doc = "Default unknown system."]
    IntegratedSystemUnspecified,
    #[serde(rename = "BIGQUERY")]
    #[doc = "BigQuery."]
    Bigquery,
    #[serde(rename = "CLOUD_PUBSUB")]
    #[doc = "Cloud Pub/Sub."]
    CloudPubsub,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the entry. Only used for Entries with types in the EntryType enum."]
pub enum GoogleCloudDatacatalogV1beta1EntryTypeEnum {
    #[serde(rename = "ENTRY_TYPE_UNSPECIFIED")]
    #[doc = "Default unknown type."]
    EntryTypeUnspecified,
    #[serde(rename = "TABLE")]
    #[doc = "Output only. The type of entry that has a GoogleSQL schema, including logical views."]
    Table,
    #[serde(rename = "MODEL")]
    #[doc = "Output only. The type of models. https://cloud.google.com/bigquery-ml/docs/bigqueryml-intro"]
    Model,
    #[serde(rename = "DATA_STREAM")]
    #[doc = "Output only. An entry type which is used for streaming entries. Example: Pub/Sub topic."]
    DataStream,
    #[serde(rename = "FILESET")]
    #[doc = "An entry type which is a set of files or objects. Example: Cloud Storage fileset."]
    Fileset,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "EntryGroup Metadata. An EntryGroup resource represents a logical grouping of zero or more Data Catalog Entry resources."]
pub struct GoogleCloudDatacatalogV1beta1EntryGroup {
    #[serde(rename = "dataCatalogTimestamps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Timestamps about this EntryGroup. Default value is empty timestamps."]
    pub data_catalog_timestamps:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDatacatalogV1beta1SystemTimestamps>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Entry group description, which can consist of several sentences or paragraphs that describe entry group contents. Default value is an empty string."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A short name to identify the entry group, for example, \"analytics data - jan 2011\". Default value is an empty string."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the entry group in URL format. Example: * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id} Note that this EntryGroup and its child resources may not actually be stored in the location in this name."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ExportTaxonomies."]
pub struct GoogleCloudDatacatalogV1beta1ExportTaxonomiesResponse {
    #[serde(rename = "taxonomies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of taxonomies and policy tags in a tree structure."]
    pub taxonomies: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1SerializedTaxonomy>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudDatacatalogV1beta1FieldType {
    #[serde(rename = "enumType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents an enum type."]
    pub enum_type:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDatacatalogV1beta1FieldTypeEnumType>>,
    #[serde(rename = "primitiveType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents primitive types - string, bool etc."]
    pub primitive_type:
        ::std::option::Option<GoogleCloudDatacatalogV1beta1FieldTypePrimitiveTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Represents primitive types - string, bool etc."]
pub enum GoogleCloudDatacatalogV1beta1FieldTypePrimitiveTypeEnum {
    #[serde(rename = "PRIMITIVE_TYPE_UNSPECIFIED")]
    #[doc = "This is the default invalid value for a type."]
    PrimitiveTypeUnspecified,
    #[serde(rename = "DOUBLE")]
    #[doc = "A double precision number."]
    Double,
    #[serde(rename = "STRING")]
    #[doc = "An UTF-8 string."]
    String,
    #[serde(rename = "BOOL")]
    #[doc = "A boolean value."]
    Bool,
    #[serde(rename = "TIMESTAMP")]
    #[doc = "A timestamp."]
    Timestamp,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudDatacatalogV1beta1FieldTypeEnumType {
    #[serde(rename = "allowedValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub allowed_values: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1FieldTypeEnumTypeEnumValue>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudDatacatalogV1beta1FieldTypeEnumTypeEnumValue {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The display name of the enum value. Must not be an empty string."]
    pub display_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Specifications of a single file in Cloud Storage."]
pub struct GoogleCloudDatacatalogV1beta1GcsFileSpec {
    #[serde(rename = "filePath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The full file path. Example: `gs://bucket_name/a/b.txt`."]
    pub file_path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gcsTimestamps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Timestamps about the Cloud Storage file."]
    pub gcs_timestamps:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDatacatalogV1beta1SystemTimestamps>>,
    #[serde(rename = "sizeBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The size of the file, in bytes."]
    pub size_bytes: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes a Cloud Storage fileset entry."]
pub struct GoogleCloudDatacatalogV1beta1GcsFilesetSpec {
    #[serde(rename = "filePatterns")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Patterns to identify a set of files in Google Cloud Storage. See [Cloud Storage documentation](https://cloud.google.com/storage/docs/gsutil/addlhelp/WildcardNames) for more information. Note that bucket wildcards are currently not supported. Examples of valid file_patterns: * `gs://bucket_name/dir/*`: matches all files within `bucket_name/dir` directory. * `gs://bucket_name/dir/**`: matches all files in `bucket_name/dir` spanning all subdirectories. * `gs://bucket_name/file*`: matches files prefixed by `file` in `bucket_name` * `gs://bucket_name/??.txt`: matches files with two characters followed by `.txt` in `bucket_name` * `gs://bucket_name/[aeiou].txt`: matches files that contain a single vowel character followed by `.txt` in `bucket_name` * `gs://bucket_name/[a-m].txt`: matches files that contain `a`, `b`, ... or `m` followed by `.txt` in `bucket_name` * `gs://bucket_name/a/*/b`: matches all files in `bucket_name` that match `a/*/b` pattern, such as `a/c/b`, `a/d/b` * `gs://another_bucket/a.txt`: matches `gs://another_bucket/a.txt` You can combine wildcards to provide more powerful matches, for example: * `gs://bucket_name/[a-m]??.j*g`"]
    pub file_patterns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "sampleGcsFileSpecs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Sample files contained in this fileset, not all files contained in this fileset are represented here."]
    pub sample_gcs_file_specs: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1GcsFileSpec>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for ImportTaxonomies."]
pub struct GoogleCloudDatacatalogV1beta1ImportTaxonomiesRequest {
    #[serde(rename = "inlineSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inline source used for taxonomies to be imported."]
    pub inline_source:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDatacatalogV1beta1InlineSource>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ImportTaxonomies."]
pub struct GoogleCloudDatacatalogV1beta1ImportTaxonomiesResponse {
    #[serde(rename = "taxonomies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Taxonomies that were imported."]
    pub taxonomies: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1Taxonomy>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Inline source used for taxonomies import."]
pub struct GoogleCloudDatacatalogV1beta1InlineSource {
    #[serde(rename = "taxonomies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Taxonomies to be imported."]
    pub taxonomies: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1SerializedTaxonomy>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ListEntries."]
pub struct GoogleCloudDatacatalogV1beta1ListEntriesResponse {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Entry details."]
    pub entries: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1Entry>>,
    >,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results. It is set to empty if no items remain in results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ListEntryGroups."]
pub struct GoogleCloudDatacatalogV1beta1ListEntryGroupsResponse {
    #[serde(rename = "entryGroups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "EntryGroup details."]
    pub entry_groups: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1EntryGroup>>,
    >,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results. It is set to empty if no items remain in results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ListPolicyTags."]
pub struct GoogleCloudDatacatalogV1beta1ListPolicyTagsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token used to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "policyTags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The policy tags that are in the requested taxonomy."]
    pub policy_tags: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1PolicyTag>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ListTags."]
pub struct GoogleCloudDatacatalogV1beta1ListTagsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results. It is set to empty if no items remain in results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tag details."]
    pub tags:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1Tag>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ListTaxonomies."]
pub struct GoogleCloudDatacatalogV1beta1ListTaxonomiesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token used to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "taxonomies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Taxonomies that the project contains."]
    pub taxonomies: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1Taxonomy>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Denotes one policy tag in a taxonomy (e.g. ssn). Policy Tags can be defined in a hierarchy. For example, consider the following hierarchy: Geolocation -> (LatLong, City, ZipCode). PolicyTag \"Geolocation\" contains three child policy tags: \"LatLong\", \"City\", and \"ZipCode\"."]
pub struct GoogleCloudDatacatalogV1beta1PolicyTag {
    #[serde(rename = "childPolicyTags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Resource names of child policy tags of this policy tag."]
    pub child_policy_tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of this policy tag. It must: contain only unicode characters, tabs, newlines, carriage returns and page breaks; and be at most 2000 bytes long when encoded in UTF-8. If not set, defaults to an empty description. If not set, defaults to an empty description."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. User defined name of this policy tag. It must: be unique within the parent taxonomy; contain only unicode letters, numbers, underscores, dashes and spaces; not start or end with spaces; and be at most 200 bytes long when encoded in UTF-8."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Resource name of this policy tag, whose format is: \"projects/{project_number}/locations/{location_id}/taxonomies/{taxonomy_id}/policyTags/{id}\"."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parentPolicyTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name of this policy tag's parent policy tag (e.g. for the \"LatLong\" policy tag in the example above, this field contains the resource name of the \"Geolocation\" policy tag). If empty, it means this policy tag is a top level policy tag (e.g. this field is empty for the \"Geolocation\" policy tag in the example above). If not set, defaults to an empty string."]
    pub parent_policy_tag: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for RenameTagTemplateFieldEnumValue."]
pub struct GoogleCloudDatacatalogV1beta1RenameTagTemplateFieldEnumValueRequest {
    #[serde(rename = "newEnumValueDisplayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The new display name of the enum value. For example, `my_new_enum_value`."]
    pub new_enum_value_display_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for RenameTagTemplateField."]
pub struct GoogleCloudDatacatalogV1beta1RenameTagTemplateFieldRequest {
    #[serde(rename = "newTagTemplateFieldId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The new ID of this tag template field. For example, `my_new_field`."]
    pub new_tag_template_field_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a schema (e.g. BigQuery, GoogleSQL, Avro schema)."]
pub struct GoogleCloudDatacatalogV1beta1Schema {
    #[serde(rename = "columns")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Schema of columns. A maximum of 10,000 columns and sub-columns can be specified."]
    pub columns: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1ColumnSchema>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for SearchCatalog."]
pub struct GoogleCloudDatacatalogV1beta1SearchCatalogRequest {
    #[serde(rename = "orderBy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the ordering of results, currently supported case-sensitive choices are: * `relevance`, only supports descending * `last_modified_timestamp [asc|desc]`, defaults to descending if not specified If not specified, defaults to `relevance` descending."]
    pub order_by: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of results in the search page. If <=0 then defaults to 10. Max limit for page_size is 1000. Throws an invalid argument for page_size > 1000."]
    pub page_size: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Pagination token returned in an earlier SearchCatalogResponse.next_page_token, which indicates that this is a continuation of a prior SearchCatalogRequest call, and that the system should return the next page of data. If empty, the first page is returned."]
    pub page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The query string in search query syntax. An empty query string will result in all data assets (in the specified scope) that the user has access to. Query strings can be simple as \"x\" or more qualified as: * name:x * column:x * description:y Note: Query tokens need to have a minimum of 3 characters for substring matching to work correctly. See [Data Catalog Search Syntax](https://cloud.google.com/data-catalog/docs/how-to/search-reference) for more information."]
    pub query: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scope")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The scope of this search request. A `scope` that has empty `include_org_ids`, `include_project_ids` AND false `include_gcp_public_datasets` is considered invalid. Data Catalog will return an error in such a case."]
    pub scope: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudDatacatalogV1beta1SearchCatalogRequestScope>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The criteria that select the subspace used for query matching."]
pub struct GoogleCloudDatacatalogV1beta1SearchCatalogRequestScope {
    #[serde(rename = "includeGcpPublicDatasets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If `true`, include Google Cloud Platform (GCP) public datasets in the search results. Info on GCP public datasets is available at https://cloud.google.com/public-datasets/. By default, GCP public datasets are excluded."]
    pub include_gcp_public_datasets: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "includeOrgIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of organization IDs to search within. To find your organization ID, follow instructions in https://cloud.google.com/resource-manager/docs/creating-managing-organization."]
    pub include_org_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "includeProjectIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of project IDs to search within. To learn more about the distinction between project names/IDs/numbers, go to https://cloud.google.com/docs/overview/#projects."]
    pub include_project_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "restrictedLocations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The list of locations to search within. 1. If empty, search will be performed in all locations; 2. If any of the locations are NOT in the valid locations list, error will be returned; 3. Otherwise, search only the given locations for matching results. Typical usage is to leave this field empty. When a location is unreachable as returned in the `SearchCatalogResponse.unreachable` field, users can repeat the search request with this parameter set to get additional information on the error. Valid locations: * asia-east1 * asia-east2 * asia-northeast1 * asia-northeast2 * asia-northeast3 * asia-south1 * asia-southeast1 * australia-southeast1 * eu * europe-north1 * europe-west1 * europe-west2 * europe-west3 * europe-west4 * europe-west6 * global * northamerica-northeast1 * southamerica-east1 * us * us-central1 * us-east1 * us-east4 * us-west1 * us-west2"]
    pub restricted_locations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for SearchCatalog."]
pub struct GoogleCloudDatacatalogV1beta1SearchCatalogResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token that can be used to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Search results."]
    pub results: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1SearchCatalogResult>>,
    >,
    #[serde(rename = "unreachable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unreachable locations. Search result does not include data from those locations. Users can get additional information on the error by repeating the search request with a more restrictive parameter -- setting the value for `SearchDataCatalogRequest.scope.restricted_locations`."]
    pub unreachable: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A result that appears in the response of a search request. Each result captures details of one entry that matches the search."]
pub struct GoogleCloudDatacatalogV1beta1SearchCatalogResult {
    #[serde(rename = "linkedResource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full name of the cloud resource the entry belongs to. See: https://cloud.google.com/apis/design/resource_names#full_resource_name. Example: * `//bigquery.googleapis.com/projects/projectId/datasets/datasetId/tables/tableId`"]
    pub linked_resource: ::std::option::Option<::std::string::String>,
    #[serde(rename = "modifyTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Last-modified timestamp of the entry from the managing system."]
    pub modify_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "relativeResourceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The relative resource name of the resource in URL format. Examples: * `projects/{project_id}/locations/{location_id}/entryGroups/{entry_group_id}/entries/{entry_id}` * `projects/{project_id}/tagTemplates/{tag_template_id}`"]
    pub relative_resource_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "searchResultSubtype")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sub-type of the search result. This is a dot-delimited description of the resource's full type, and is the same as the value callers would provide in the \"type\" search facet. Examples: `entry.table`, `entry.dataStream`, `tagTemplate`."]
    pub search_result_subtype: ::std::option::Option<::std::string::String>,
    #[serde(rename = "searchResultType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the search result. This field can be used to determine which Get method to call to fetch the full resource."]
    pub search_result_type:
        ::std::option::Option<GoogleCloudDatacatalogV1beta1SearchCatalogResultSearchResultTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of the search result. This field can be used to determine which Get method to call to fetch the full resource."]
pub enum GoogleCloudDatacatalogV1beta1SearchCatalogResultSearchResultTypeEnum {
    #[serde(rename = "SEARCH_RESULT_TYPE_UNSPECIFIED")]
    #[doc = "Default unknown type."]
    SearchResultTypeUnspecified,
    #[serde(rename = "ENTRY")]
    #[doc = "An Entry."]
    Entry,
    #[serde(rename = "TAG_TEMPLATE")]
    #[doc = "A TagTemplate."]
    TagTemplate,
    #[serde(rename = "ENTRY_GROUP")]
    #[doc = "An EntryGroup."]
    EntryGroup,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message representing one policy tag when exported as a nested proto."]
pub struct GoogleCloudDatacatalogV1beta1SerializedPolicyTag {
    #[serde(rename = "childPolicyTags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Children of the policy tag if any."]
    pub child_policy_tags: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1SerializedPolicyTag>>,
    >,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the serialized policy tag. The length of the description is limited to 2000 bytes when encoded in UTF-8. If not set, defaults to an empty description."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Display name of the policy tag. Max 200 bytes when encoded in UTF-8."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "policyTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name of the policy tag. This field will be ignored when calling ImportTaxonomies."]
    pub policy_tag: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message capturing a taxonomy and its policy tag hierarchy as a nested proto. Used for taxonomy import/export and mutation."]
pub struct GoogleCloudDatacatalogV1beta1SerializedTaxonomy {
    #[serde(rename = "activatedPolicyTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of policy types that are activated for a taxonomy."]
    pub activated_policy_types: ::std::option::Option<
        ::std::vec::Vec<GoogleCloudDatacatalogV1beta1SerializedTaxonomyActivatedPolicyTypesEnum>,
    >,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the serialized taxonomy. The length of the description is limited to 2000 bytes when encoded in UTF-8. If not set, defaults to an empty description."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Display name of the taxonomy. Max 200 bytes when encoded in UTF-8."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "policyTags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Top level policy tags associated with the taxonomy if any."]
    pub policy_tags: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1SerializedPolicyTag>>,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum GoogleCloudDatacatalogV1beta1SerializedTaxonomyActivatedPolicyTypesEnum {
    #[serde(rename = "POLICY_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified policy type."]
    PolicyTypeUnspecified,
    #[serde(rename = "FINE_GRAINED_ACCESS_CONTROL")]
    #[doc = "Fine grained access control policy, which enables access control on tagged resources."]
    FineGrainedAccessControl,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Timestamps about this resource according to a particular system."]
pub struct GoogleCloudDatacatalogV1beta1SystemTimestamps {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The creation time of the resource within the given system."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expireTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The expiration time of the resource within the given system. Currently only apllicable to BigQuery resources."]
    pub expire_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last-modified time of the resource within the given system."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Normal BigQuery table spec."]
pub struct GoogleCloudDatacatalogV1beta1TableSpec {
    #[serde(rename = "groupedEntry")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. If the table is a dated shard, i.e., with name pattern `[prefix]YYYYMMDD`, `grouped_entry` is the Data Catalog resource name of the date sharded grouped entry, for example, `projects/{project_id}/locations/{location}/entrygroups/{entry_group_id}/entries/{entry_id}`. Otherwise, `grouped_entry` is empty."]
    pub grouped_entry: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Tags are used to attach custom metadata to Data Catalog resources. Tags conform to the specifications within their tag template. See [Data Catalog IAM](https://cloud.google.com/data-catalog/docs/concepts/iam) for information on the permissions needed to create or view tags."]
pub struct GoogleCloudDatacatalogV1beta1Tag {
    #[serde(rename = "column")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resources like Entry can have schemas associated with them. This scope allows users to attach tags to an individual column based on that schema. For attaching a tag to a nested column, use `.` to separate the column names. Example: * `outer_column.inner_column`"]
    pub column: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. This maps the ID of a tag field to the value of and additional information about that field. Valid field IDs are defined by the tag's template. A tag must have at least 1 field and at most 500 fields."]
    pub fields: ::std::option::Option<
        ::std::collections::BTreeMap<
            String,
            ::std::boxed::Box<GoogleCloudDatacatalogV1beta1TagField>,
        >,
    >,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the tag in URL format. Example: * projects/{project_id}/locations/{location}/entrygroups/{entry_group_id}/entries/{entry_id}/tags/{tag_id} where `tag_id` is a system-generated identifier. Note that this Tag may not actually be stored in the location in this name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "template")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The resource name of the tag template that this tag uses. Example: * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id} This field cannot be modified after creation."]
    pub template: ::std::option::Option<::std::string::String>,
    #[serde(rename = "templateDisplayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name of the tag template."]
    pub template_display_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains the value and supporting information for a field within a Tag."]
pub struct GoogleCloudDatacatalogV1beta1TagField {
    #[serde(rename = "boolValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Holds the value for a tag field with boolean type."]
    pub bool_value: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The display name of this field."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "doubleValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Holds the value for a tag field with double type."]
    pub double_value: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "enumValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Holds the value for a tag field with enum type. This value must be one of the allowed values in the definition of this enum."]
    pub enum_value:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDatacatalogV1beta1TagFieldEnumValue>>,
    #[serde(rename = "order")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The order of this field with respect to other fields in this tag. It can be set in Tag. For example, a higher value can indicate a more important field. The value can be negative. Multiple fields can have the same order, and field orders within a tag do not have to be sequential."]
    pub order: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "stringValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Holds the value for a tag field with string type."]
    pub string_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timestampValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Holds the value for a tag field with timestamp type."]
    pub timestamp_value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Holds an enum value."]
pub struct GoogleCloudDatacatalogV1beta1TagFieldEnumValue {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display name of the enum value."]
    pub display_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A tag template defines a tag, which can have one or more typed fields. The template is used to create and attach the tag to GCP resources. [Tag template roles](https://cloud.google.com/iam/docs/understanding-roles#data-catalog-roles) provide permissions to create, edit, and use the template. See, for example, the [TagTemplate User](https://cloud.google.com/data-catalog/docs/how-to/template-user) role, which includes permission to use the tag template to tag resources."]
pub struct GoogleCloudDatacatalogV1beta1TagTemplate {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display name for this template. Defaults to an empty string."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Map of tag template field IDs to the settings for the field. This map is an exhaustive list of the allowed fields. This map must contain at least one field and at most 500 fields. The keys to this map are tag template field IDs. Field IDs can contain letters (both uppercase and lowercase), numbers (0-9) and underscores (_). Field IDs must be at least 1 character long and at most 64 characters long. Field IDs must start with a letter or underscore."]
    pub fields: ::std::option::Option<
        ::std::collections::BTreeMap<
            String,
            ::std::boxed::Box<GoogleCloudDatacatalogV1beta1TagTemplateField>,
        >,
    >,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the tag template in URL format. Example: * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id} Note that this TagTemplate and its child resources may not actually be stored in the location in this name."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The template for an individual field within a tag template."]
pub struct GoogleCloudDatacatalogV1beta1TagTemplateField {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The description for this field. Defaults to an empty string."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display name for this field. Defaults to an empty string."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isRequired")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this is a required field. Defaults to false."]
    pub is_required: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the tag template field in URL format. Example: * projects/{project_id}/locations/{location}/tagTemplates/{tag_template}/fields/{field} Note that this TagTemplateField may not actually be stored in the location in this name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "order")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The order of this field with respect to other fields in this tag template. A higher value indicates a more important field. The value can be negative. Multiple fields can have the same order, and field orders within a tag do not have to be sequential."]
    pub order: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The type of value this tag field can contain."]
    pub _type: ::std::option::Option<::std::boxed::Box<GoogleCloudDatacatalogV1beta1FieldType>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A taxonomy is a collection of policy tags that classify data along a common axis. For instance a data *sensitivity* taxonomy could contain policy tags denoting PII such as age, zipcode, and SSN. A data *origin* taxonomy could contain policy tags to distinguish user data, employee data, partner data, public data."]
pub struct GoogleCloudDatacatalogV1beta1Taxonomy {
    #[serde(rename = "activatedPolicyTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A list of policy types that are activated for this taxonomy. If not set, defaults to an empty list."]
    pub activated_policy_types: ::std::option::Option<
        ::std::vec::Vec<GoogleCloudDatacatalogV1beta1TaxonomyActivatedPolicyTypesEnum>,
    >,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Description of this taxonomy. It must: contain only unicode characters, tabs, newlines, carriage returns and page breaks; and be at most 2000 bytes long when encoded in UTF-8. If not set, defaults to an empty description."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. User defined name of this taxonomy. It must: contain only unicode letters, numbers, underscores, dashes and spaces; not start or end with spaces; and be at most 200 bytes long when encoded in UTF-8."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Resource name of this taxonomy, whose format is: \"projects/{project_number}/locations/{location_id}/taxonomies/{id}\"."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "policyTagCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Number of policy tags contained in this taxonomy."]
    pub policy_tag_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "taxonomyTimestamps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Timestamps about this taxonomy. Only create_time and update_time are used."]
    pub taxonomy_timestamps:
        ::std::option::Option<::std::boxed::Box<GoogleCloudDatacatalogV1beta1SystemTimestamps>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum GoogleCloudDatacatalogV1beta1TaxonomyActivatedPolicyTypesEnum {
    #[serde(rename = "POLICY_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified policy type."]
    PolicyTypeUnspecified,
    #[serde(rename = "FINE_GRAINED_ACCESS_CONTROL")]
    #[doc = "Fine grained access control policy, which enables access control on tagged resources."]
    FineGrainedAccessControl,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Table view specification."]
pub struct GoogleCloudDatacatalogV1beta1ViewSpec {
    #[serde(rename = "viewQuery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The query that defines the table view."]
    pub view_query: ::std::option::Option<::std::string::String>,
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
#[doc = "Request message for `SetIamPolicy` method."]
pub struct SetIamPolicyRequest {
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Cloud Platform services (such as Projects) might reject them."]
    pub policy: ::std::option::Option<::std::boxed::Box<Policy>>,
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
