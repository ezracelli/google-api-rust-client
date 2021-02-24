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
    #[serde(rename = "relationshipTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of relationship types to export, for example: `INSTANCE_TO_INSTANCEGROUP`. This field should only be specified if content_type=RELATIONSHIP. If specified, it will snapshot [asset_types]' specified relationships, or give errors if any relationship_types' supported types are not in [asset_types]. If not specified, it will snapshot all [asset_types]' supported relationships. An unspecified [asset_types] field means all supported asset_types. See [Introduction to Cloud Asset Inventory](https://cloud.google.com/asset-inventory/docs/overview) for all supported asset types and relationship types."]
    pub relationship_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
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
    #[serde(rename = "RELATIONSHIP")]
    #[doc = "The related resources."]
    Relationship,
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
    #[doc = "The uri prefix of all generated Cloud Storage objects. Example: \"gs://bucket_name/object_name_prefix\". Each object uri is in format: \"gs://bucket_name/object_name_prefix/{ASSET_TYPE}/{SHARD_NUMBER} and only contains assets for that type. starts from 0. Example: \"gs://bucket_name/object_name_prefix/compute.googleapis.com/Disk/0\" is the first shard of output objects containing all compute.googleapis.com/Disk assets. An INVALID_ARGUMENT error will be returned if file with the same name \"gs://bucket_name/object_name_prefix\" already exists."]
    pub uri_prefix: ::std::option::Option<::std::string::String>,
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