#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An array value."]
pub struct ArrayValue {
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Values in the array."]
    pub values: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Value>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for Firestore.BatchGetDocuments."]
pub struct BatchGetDocumentsRequest {
    #[serde(rename = "documents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The names of the documents to retrieve. In the format: `projects/{project_id}/databases/{database_id}/documents/{document_path}`. The request will fail if any of the document is not a child resource of the given `database`. Duplicate names will be elided."]
    pub documents: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "mask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fields to return. If not set, returns all fields. If a document has a field that is not present in this mask, that field will not be returned in the response."]
    pub mask: ::std::option::Option<::std::boxed::Box<DocumentMask>>,
    #[serde(rename = "newTransaction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Starts a new transaction and reads the documents. Defaults to a read-only transaction. The new transaction ID will be returned as the first response in the stream."]
    pub new_transaction: ::std::option::Option<::std::boxed::Box<TransactionOptions>>,
    #[serde(rename = "readTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reads documents as they were at the given time. This may not be older than 270 seconds."]
    pub read_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "transaction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reads documents in a transaction."]
    pub transaction: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The streamed response for Firestore.BatchGetDocuments."]
pub struct BatchGetDocumentsResponse {
    #[serde(rename = "found")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A document that was requested."]
    pub found: ::std::option::Option<::std::boxed::Box<Document>>,
    #[serde(rename = "missing")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A document name that was requested but does not exist. In the format: `projects/{project_id}/databases/{database_id}/documents/{document_path}`."]
    pub missing: ::std::option::Option<::std::string::String>,
    #[serde(rename = "readTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the document was read. This may be monotically increasing, in this case the previous documents in the result stream are guaranteed not to have changed between their read_time and this one."]
    pub read_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "transaction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The transaction that was started as part of this request. Will only be set in the first response, and only if BatchGetDocumentsRequest.new_transaction was set in the request."]
    pub transaction: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for Firestore.BatchWrite."]
pub struct BatchWriteRequest {
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Labels associated with this batch write."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "writes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The writes to apply. Method does not apply writes atomically and does not guarantee ordering. Each write succeeds or fails independently. You cannot write to the same document more than once per request."]
    pub writes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Write>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response from Firestore.BatchWrite."]
pub struct BatchWriteResponse {
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of applying the writes. This i-th write status corresponds to the i-th write in the request."]
    pub status: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Status>>>,
    #[serde(rename = "writeResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of applying the writes. This i-th write result corresponds to the i-th write in the request."]
    pub write_results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WriteResult>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for Firestore.BeginTransaction."]
pub struct BeginTransactionRequest {
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The options for the transaction. Defaults to a read-write transaction."]
    pub options: ::std::option::Option<::std::boxed::Box<TransactionOptions>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for Firestore.BeginTransaction."]
pub struct BeginTransactionResponse {
    #[serde(rename = "transaction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The transaction that was started."]
    pub transaction: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A selection of a collection, such as `messages as m1`."]
pub struct CollectionSelector {
    #[serde(rename = "allDescendants")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When false, selects only collections that are immediate children of the `parent` specified in the containing `RunQueryRequest`. When true, selects all descendant collections."]
    pub all_descendants: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "collectionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The collection ID. When set, selects only collections with this ID."]
    pub collection_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for Firestore.Commit."]
pub struct CommitRequest {
    #[serde(rename = "transaction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, applies all writes in this transaction, and commits it."]
    pub transaction: ::std::option::Option<::std::string::String>,
    #[serde(rename = "writes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The writes to apply. Always executed atomically and in order."]
    pub writes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Write>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for Firestore.Commit."]
pub struct CommitResponse {
    #[serde(rename = "commitTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the commit occurred. Any read with an equal or greater `read_time` is guaranteed to see the effects of the commit."]
    pub commit_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "writeResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of applying the writes. This i-th write result corresponds to the i-th write in the request."]
    pub write_results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WriteResult>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A filter that merges multiple other filters using the given operator."]
pub struct CompositeFilter {
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of filters to combine. Must contain at least one filter."]
    pub filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Filter>>>,
    #[serde(rename = "op")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The operator for combining multiple filters."]
    pub op: ::std::option::Option<CompositeFilterOpEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The operator for combining multiple filters."]
pub enum CompositeFilterOpEnum {
    #[serde(rename = "OPERATOR_UNSPECIFIED")]
    #[doc = "Unspecified. This value must not be used."]
    OperatorUnspecified,
    #[serde(rename = "AND")]
    #[doc = "The results are required to satisfy each of the combined filters."]
    And,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A position in a query result set."]
pub struct Cursor {
    #[serde(rename = "before")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the position is just before or just after the given values, relative to the sort order defined by the query."]
    pub before: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The values that represent a position, in the order they appear in the order by clause of a query. Can contain fewer values than specified in the order by clause."]
    pub values: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Value>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Firestore document. Must not exceed 1 MiB - 4 bytes."]
pub struct Document {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time at which the document was created. This value increases monotonically when a document is deleted then recreated. It can also be compared to values from other documents and the `read_time` of a query."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The document's fields. The map keys represent field names. A simple field name contains only characters `a` to `z`, `A` to `Z`, `0` to `9`, or `_`, and must not start with `0` to `9`. For example, `foo_bar_17`. Field names matching the regular expression `__.*__` are reserved. Reserved field names are forbidden except in certain documented contexts. The map keys, represented as UTF-8, must not exceed 1,500 bytes and cannot be empty. Field paths may be used in other contexts to refer to structured fields defined here. For `map_value`, the field path is represented by the simple or quoted field names of the containing fields, delimited by `.`. For example, the structured field `\"foo\" : { map_value: { \"x&y\" : { string_value: \"hello\" }}}` would be represented by the field path `foo.x&y`. Within a field path, a quoted field name starts and ends with `` ` `` and may contain any character. Some characters, including `` ` ``, must be escaped using a `\\`. For example, `` `x&y` `` represents `x&y` and `` `bak\\`tik` `` represents `` bak`tik ``."]
    pub fields:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<Value>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the document, for example `projects/{project_id}/databases/{database_id}/documents/{document_path}`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time at which the document was last changed. This value is initially set to the `create_time` then increases monotonically with each change to the document. It can also be compared to values from other documents and the `read_time` of a query."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Document has changed. May be the result of multiple writes, including deletes, that ultimately resulted in a new value for the Document. Multiple DocumentChange messages may be returned for the same logical change, if multiple targets are affected."]
pub struct DocumentChange {
    #[serde(rename = "document")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The new state of the Document. If `mask` is set, contains only fields that were updated or added."]
    pub document: ::std::option::Option<::std::boxed::Box<Document>>,
    #[serde(rename = "removedTargetIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A set of target IDs for targets that no longer match this document."]
    pub removed_target_ids: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    #[serde(rename = "targetIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A set of target IDs of targets that match this document."]
    pub target_ids: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Document has been deleted. May be the result of multiple writes, including updates, the last of which deleted the Document. Multiple DocumentDelete messages may be returned for the same logical delete, if multiple targets are affected."]
pub struct DocumentDelete {
    #[serde(rename = "document")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the Document that was deleted."]
    pub document: ::std::option::Option<::std::string::String>,
    #[serde(rename = "readTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The read timestamp at which the delete was observed. Greater or equal to the `commit_time` of the delete."]
    pub read_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "removedTargetIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A set of target IDs for targets that previously matched this entity."]
    pub removed_target_ids: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A set of field paths on a document. Used to restrict a get or update operation on a document to a subset of its fields. This is different from standard field masks, as this is always scoped to a Document, and takes in account the dynamic nature of Value."]
pub struct DocumentMask {
    #[serde(rename = "fieldPaths")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of field paths in the mask. See Document.fields for a field path syntax reference."]
    pub field_paths: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Document has been removed from the view of the targets. Sent if the document is no longer relevant to a target and is out of view. Can be sent instead of a DocumentDelete or a DocumentChange if the server can not send the new value of the document. Multiple DocumentRemove messages may be returned for the same logical write or delete, if multiple targets are affected."]
pub struct DocumentRemove {
    #[serde(rename = "document")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the Document that has gone out of view."]
    pub document: ::std::option::Option<::std::string::String>,
    #[serde(rename = "readTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The read timestamp at which the remove was observed. Greater or equal to the `commit_time` of the change/delete/remove."]
    pub read_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "removedTargetIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A set of target IDs for targets that previously matched this document."]
    pub removed_target_ids: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A transformation of a document."]
pub struct DocumentTransform {
    #[serde(rename = "document")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the document to transform."]
    pub document: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fieldTransforms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of transformations to apply to the fields of the document, in order. This must not be empty."]
    pub field_transforms: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FieldTransform>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A target specified by a set of documents names."]
pub struct DocumentsTarget {
    #[serde(rename = "documents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The names of the documents to retrieve. In the format: `projects/{project_id}/databases/{database_id}/documents/{document_path}`. The request will fail if any of the document is not a child resource of the given `database`. Duplicate names will be elided."]
    pub documents: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A digest of all the documents that match a given target."]
pub struct ExistenceFilter {
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total count of documents that match target_id. If different from the count of documents in the client that match, the client must manually determine which documents no longer match the target."]
    pub count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target ID to which this filter applies."]
    pub target_id: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A filter on a specific field."]
pub struct FieldFilter {
    #[serde(rename = "field")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The field to filter by."]
    pub field: ::std::option::Option<::std::boxed::Box<FieldReference>>,
    #[serde(rename = "op")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The operator to filter by."]
    pub op: ::std::option::Option<FieldFilterOpEnum>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value to compare to."]
    pub value: ::std::option::Option<::std::boxed::Box<Value>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The operator to filter by."]
pub enum FieldFilterOpEnum {
    #[serde(rename = "OPERATOR_UNSPECIFIED")]
    #[doc = "Unspecified. This value must not be used."]
    OperatorUnspecified,
    #[serde(rename = "LESS_THAN")]
    #[doc = "The given `field` is less than the given `value`. Requires: * That `field` come first in `order_by`."]
    LessThan,
    #[serde(rename = "LESS_THAN_OR_EQUAL")]
    #[doc = "The given `field` is less than or equal to the given `value`. Requires: * That `field` come first in `order_by`."]
    LessThanOrEqual,
    #[serde(rename = "GREATER_THAN")]
    #[doc = "The given `field` is greater than the given `value`. Requires: * That `field` come first in `order_by`."]
    GreaterThan,
    #[serde(rename = "GREATER_THAN_OR_EQUAL")]
    #[doc = "The given `field` is greater than or equal to the given `value`. Requires: * That `field` come first in `order_by`."]
    GreaterThanOrEqual,
    #[serde(rename = "EQUAL")]
    #[doc = "The given `field` is equal to the given `value`."]
    Equal,
    #[serde(rename = "NOT_EQUAL")]
    #[doc = "The given `field` is not equal to the given `value`. Requires: * No other `NOT_EQUAL`, `NOT_IN`, `IS_NOT_NULL`, or `IS_NOT_NAN`. * That `field` comes first in the `order_by`."]
    NotEqual,
    #[serde(rename = "ARRAY_CONTAINS")]
    #[doc = "The given `field` is an array that contains the given `value`."]
    ArrayContains,
    #[serde(rename = "IN")]
    #[doc = "The given `field` is equal to at least one value in the given array. Requires: * That `value` is a non-empty `ArrayValue` with at most 10 values. * No other `IN` or `ARRAY_CONTAINS_ANY` or `NOT_IN`."]
    In,
    #[serde(rename = "ARRAY_CONTAINS_ANY")]
    #[doc = "The given `field` is an array that contains any of the values in the given array. Requires: * That `value` is a non-empty `ArrayValue` with at most 10 values. * No other `IN` or `ARRAY_CONTAINS_ANY` or `NOT_IN`."]
    ArrayContainsAny,
    #[serde(rename = "NOT_IN")]
    #[doc = "The value of the `field` is not in the given array. Requires: * That `value` is a non-empty `ArrayValue` with at most 10 values. * No other `IN`, `ARRAY_CONTAINS_ANY`, `NOT_IN`, `NOT_EQUAL`, `IS_NOT_NULL`, or `IS_NOT_NAN`. * That `field` comes first in the `order_by`."]
    NotIn,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A reference to a field, such as `max(messages.time) as max_time`."]
pub struct FieldReference {
    #[serde(rename = "fieldPath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub field_path: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A transformation of a field of the document."]
pub struct FieldTransform {
    #[serde(rename = "appendMissingElements")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Append the given elements in order if they are not already present in the current field value. If the field is not an array, or if the field does not yet exist, it is first set to the empty array. Equivalent numbers of different types (e.g. 3L and 3.0) are considered equal when checking if a value is missing. NaN is equal to NaN, and Null is equal to Null. If the input contains multiple equivalent values, only the first will be considered. The corresponding transform_result will be the null value."]
    pub append_missing_elements: ::std::option::Option<::std::boxed::Box<ArrayValue>>,
    #[serde(rename = "fieldPath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The path of the field. See Document.fields for the field path syntax reference."]
    pub field_path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "increment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Adds the given value to the field's current value. This must be an integer or a double value. If the field is not an integer or double, or if the field does not yet exist, the transformation will set the field to the given value. If either of the given value or the current field value are doubles, both values will be interpreted as doubles. Double arithmetic and representation of double values follow IEEE 754 semantics. If there is positive/negative integer overflow, the field is resolved to the largest magnitude positive/negative integer."]
    pub increment: ::std::option::Option<::std::boxed::Box<Value>>,
    #[serde(rename = "maximum")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sets the field to the maximum of its current value and the given value. This must be an integer or a double value. If the field is not an integer or double, or if the field does not yet exist, the transformation will set the field to the given value. If a maximum operation is applied where the field and the input value are of mixed types (that is - one is an integer and one is a double) the field takes on the type of the larger operand. If the operands are equivalent (e.g. 3 and 3.0), the field does not change. 0, 0.0, and -0.0 are all zero. The maximum of a zero stored value and zero input value is always the stored value. The maximum of any numeric value x and NaN is NaN."]
    pub maximum: ::std::option::Option<::std::boxed::Box<Value>>,
    #[serde(rename = "minimum")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sets the field to the minimum of its current value and the given value. This must be an integer or a double value. If the field is not an integer or double, or if the field does not yet exist, the transformation will set the field to the input value. If a minimum operation is applied where the field and the input value are of mixed types (that is - one is an integer and one is a double) the field takes on the type of the smaller operand. If the operands are equivalent (e.g. 3 and 3.0), the field does not change. 0, 0.0, and -0.0 are all zero. The minimum of a zero stored value and zero input value is always the stored value. The minimum of any numeric value x and NaN is NaN."]
    pub minimum: ::std::option::Option<::std::boxed::Box<Value>>,
    #[serde(rename = "removeAllFromArray")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Remove all of the given elements from the array in the field. If the field is not an array, or if the field does not yet exist, it is set to the empty array. Equivalent numbers of the different types (e.g. 3L and 3.0) are considered equal when deciding whether an element should be removed. NaN is equal to NaN, and Null is equal to Null. This will remove all equivalent values if there are duplicates. The corresponding transform_result will be the null value."]
    pub remove_all_from_array: ::std::option::Option<::std::boxed::Box<ArrayValue>>,
    #[serde(rename = "setToServerValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sets the field to the given server value."]
    pub set_to_server_value: ::std::option::Option<FieldTransformSetToServerValueEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Sets the field to the given server value."]
pub enum FieldTransformSetToServerValueEnum {
    #[serde(rename = "SERVER_VALUE_UNSPECIFIED")]
    #[doc = "Unspecified. This value must not be used."]
    ServerValueUnspecified,
    #[serde(rename = "REQUEST_TIME")]
    #[doc = "The time at which the server processed the request, with millisecond precision. If used on multiple fields (same or different documents) in a transaction, all the fields will get the same server timestamp."]
    RequestTime,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A filter."]
pub struct Filter {
    #[serde(rename = "compositeFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A composite filter."]
    pub composite_filter: ::std::option::Option<::std::boxed::Box<CompositeFilter>>,
    #[serde(rename = "fieldFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A filter on a document field."]
    pub field_filter: ::std::option::Option<::std::boxed::Box<FieldFilter>>,
    #[serde(rename = "unaryFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A filter that takes exactly one argument."]
    pub unary_filter: ::std::option::Option<::std::boxed::Box<UnaryFilter>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for google.longrunning.Operation results from FirestoreAdmin.ExportDocuments."]
pub struct GoogleFirestoreAdminV1ExportDocumentsMetadata {
    #[serde(rename = "collectionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Which collection ids are being exported."]
    pub collection_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time this operation completed. Will be unset if operation still in progress."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state of the export operation."]
    pub operation_state:
        ::std::option::Option<GoogleFirestoreAdminV1ExportDocumentsMetadataOperationStateEnum>,
    #[serde(rename = "outputUriPrefix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Where the entities are being exported to."]
    pub output_uri_prefix: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progressBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The progress, in bytes, of this operation."]
    pub progress_bytes: ::std::option::Option<::std::boxed::Box<GoogleFirestoreAdminV1Progress>>,
    #[serde(rename = "progressDocuments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The progress, in documents, of this operation."]
    pub progress_documents:
        ::std::option::Option<::std::boxed::Box<GoogleFirestoreAdminV1Progress>>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time this operation started."]
    pub start_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The state of the export operation."]
pub enum GoogleFirestoreAdminV1ExportDocumentsMetadataOperationStateEnum {
    #[serde(rename = "OPERATION_STATE_UNSPECIFIED")]
    #[doc = "Unspecified."]
    OperationStateUnspecified,
    #[serde(rename = "INITIALIZING")]
    #[doc = "Request is being prepared for processing."]
    Initializing,
    #[serde(rename = "PROCESSING")]
    #[doc = "Request is actively being processed."]
    Processing,
    #[serde(rename = "CANCELLING")]
    #[doc = "Request is in the process of being cancelled after user called google.longrunning.Operations.CancelOperation on the operation."]
    Cancelling,
    #[serde(rename = "FINALIZING")]
    #[doc = "Request has been processed and is in its finalization stage."]
    Finalizing,
    #[serde(rename = "SUCCESSFUL")]
    #[doc = "Request has completed successfully."]
    Successful,
    #[serde(rename = "FAILED")]
    #[doc = "Request has finished being processed, but encountered an error."]
    Failed,
    #[serde(rename = "CANCELLED")]
    #[doc = "Request has finished being cancelled after user called google.longrunning.Operations.CancelOperation."]
    Cancelled,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for FirestoreAdmin.ExportDocuments."]
pub struct GoogleFirestoreAdminV1ExportDocumentsRequest {
    #[serde(rename = "collectionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Which collection ids to export. Unspecified means all collections."]
    pub collection_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "outputUriPrefix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The output URI. Currently only supports Google Cloud Storage URIs of the form: `gs://BUCKET_NAME[/NAMESPACE_PATH]`, where `BUCKET_NAME` is the name of the Google Cloud Storage bucket and `NAMESPACE_PATH` is an optional Google Cloud Storage namespace path. When choosing a name, be sure to consider Google Cloud Storage naming guidelines: https://cloud.google.com/storage/docs/naming. If the URI is a bucket (without a namespace path), a prefix will be generated based on the start time."]
    pub output_uri_prefix: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Returned in the google.longrunning.Operation response field."]
pub struct GoogleFirestoreAdminV1ExportDocumentsResponse {
    #[serde(rename = "outputUriPrefix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Location of the output files. This can be used to begin an import into Cloud Firestore (this project or another project) after the operation completes successfully."]
    pub output_uri_prefix: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a single field in the database. Fields are grouped by their \"Collection Group\", which represent all collections in the database with the same id."]
pub struct GoogleFirestoreAdminV1Field {
    #[serde(rename = "indexConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The index configuration for this field. If unset, field indexing will revert to the configuration defined by the `ancestor_field`. To explicitly remove all indexes for this field, specify an index config with an empty list of indexes."]
    pub index_config: ::std::option::Option<::std::boxed::Box<GoogleFirestoreAdminV1IndexConfig>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A field name of the form `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/fields/{field_path}` A field path may be a simple field name, e.g. `address` or a path to fields within map_value , e.g. `address.city`, or a special field path. The only valid special field is `*`, which represents any field. Field paths may be quoted using ` (backtick). The only character that needs to be escaped within a quoted field path is the backtick character itself, escaped using a backslash. Special characters in field paths that must be quoted include: `*`, `.`, ``` (backtick), `[`, `]`, as well as any ascii symbolic characters. Examples: (Note: Comments here are written in markdown syntax, so there is an additional layer of backticks to represent a code block) `\\`address.city\\`` represents a field named `address.city`, not the map key `city` in the field `address`. `\\`*\\`` represents a field named `*`, not any field. A special `Field` contains the default indexing settings for all fields. This field's resource name is: `projects/{project_id}/databases/{database_id}/collectionGroups/__default__/fields/*` Indexes defined on this `Field` will be applied to all fields which do not have their own `Field` index configuration."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for google.longrunning.Operation results from FirestoreAdmin.UpdateField."]
pub struct GoogleFirestoreAdminV1FieldOperationMetadata {
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time this operation completed. Will be unset if operation still in progress."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "field")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The field resource that this operation is acting on. For example: `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/fields/{field_path}`"]
    pub field: ::std::option::Option<::std::string::String>,
    #[serde(rename = "indexConfigDeltas")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of IndexConfigDelta, which describe the intent of this operation."]
    pub index_config_deltas: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleFirestoreAdminV1IndexConfigDelta>>,
    >,
    #[serde(rename = "progressBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The progress, in bytes, of this operation."]
    pub progress_bytes: ::std::option::Option<::std::boxed::Box<GoogleFirestoreAdminV1Progress>>,
    #[serde(rename = "progressDocuments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The progress, in documents, of this operation."]
    pub progress_documents:
        ::std::option::Option<::std::boxed::Box<GoogleFirestoreAdminV1Progress>>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time this operation started."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state of the operation."]
    pub state: ::std::option::Option<GoogleFirestoreAdminV1FieldOperationMetadataStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The state of the operation."]
pub enum GoogleFirestoreAdminV1FieldOperationMetadataStateEnum {
    #[serde(rename = "OPERATION_STATE_UNSPECIFIED")]
    #[doc = "Unspecified."]
    OperationStateUnspecified,
    #[serde(rename = "INITIALIZING")]
    #[doc = "Request is being prepared for processing."]
    Initializing,
    #[serde(rename = "PROCESSING")]
    #[doc = "Request is actively being processed."]
    Processing,
    #[serde(rename = "CANCELLING")]
    #[doc = "Request is in the process of being cancelled after user called google.longrunning.Operations.CancelOperation on the operation."]
    Cancelling,
    #[serde(rename = "FINALIZING")]
    #[doc = "Request has been processed and is in its finalization stage."]
    Finalizing,
    #[serde(rename = "SUCCESSFUL")]
    #[doc = "Request has completed successfully."]
    Successful,
    #[serde(rename = "FAILED")]
    #[doc = "Request has finished being processed, but encountered an error."]
    Failed,
    #[serde(rename = "CANCELLED")]
    #[doc = "Request has finished being cancelled after user called google.longrunning.Operations.CancelOperation."]
    Cancelled,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for google.longrunning.Operation results from FirestoreAdmin.ImportDocuments."]
pub struct GoogleFirestoreAdminV1ImportDocumentsMetadata {
    #[serde(rename = "collectionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Which collection ids are being imported."]
    pub collection_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time this operation completed. Will be unset if operation still in progress."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inputUriPrefix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The location of the documents being imported."]
    pub input_uri_prefix: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state of the import operation."]
    pub operation_state:
        ::std::option::Option<GoogleFirestoreAdminV1ImportDocumentsMetadataOperationStateEnum>,
    #[serde(rename = "progressBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The progress, in bytes, of this operation."]
    pub progress_bytes: ::std::option::Option<::std::boxed::Box<GoogleFirestoreAdminV1Progress>>,
    #[serde(rename = "progressDocuments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The progress, in documents, of this operation."]
    pub progress_documents:
        ::std::option::Option<::std::boxed::Box<GoogleFirestoreAdminV1Progress>>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time this operation started."]
    pub start_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The state of the import operation."]
pub enum GoogleFirestoreAdminV1ImportDocumentsMetadataOperationStateEnum {
    #[serde(rename = "OPERATION_STATE_UNSPECIFIED")]
    #[doc = "Unspecified."]
    OperationStateUnspecified,
    #[serde(rename = "INITIALIZING")]
    #[doc = "Request is being prepared for processing."]
    Initializing,
    #[serde(rename = "PROCESSING")]
    #[doc = "Request is actively being processed."]
    Processing,
    #[serde(rename = "CANCELLING")]
    #[doc = "Request is in the process of being cancelled after user called google.longrunning.Operations.CancelOperation on the operation."]
    Cancelling,
    #[serde(rename = "FINALIZING")]
    #[doc = "Request has been processed and is in its finalization stage."]
    Finalizing,
    #[serde(rename = "SUCCESSFUL")]
    #[doc = "Request has completed successfully."]
    Successful,
    #[serde(rename = "FAILED")]
    #[doc = "Request has finished being processed, but encountered an error."]
    Failed,
    #[serde(rename = "CANCELLED")]
    #[doc = "Request has finished being cancelled after user called google.longrunning.Operations.CancelOperation."]
    Cancelled,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for FirestoreAdmin.ImportDocuments."]
pub struct GoogleFirestoreAdminV1ImportDocumentsRequest {
    #[serde(rename = "collectionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Which collection ids to import. Unspecified means all collections included in the import."]
    pub collection_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "inputUriPrefix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Location of the exported files. This must match the output_uri_prefix of an ExportDocumentsResponse from an export that has completed successfully. See: google.firestore.admin.v1.ExportDocumentsResponse.output_uri_prefix."]
    pub input_uri_prefix: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Cloud Firestore indexes enable simple and complex queries against documents in a database."]
pub struct GoogleFirestoreAdminV1Index {
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fields supported by this index. For composite indexes, this is always 2 or more fields. The last field entry is always for the field path `__name__`. If, on creation, `__name__` was not specified as the last field, it will be added automatically with the same direction as that of the last field defined. If the final field in a composite index is not directional, the `__name__` will be ordered ASCENDING (unless explicitly specified). For single field indexes, this will always be exactly one entry with a field path equal to the field path of the associated field."]
    pub fields:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleFirestoreAdminV1IndexField>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A server defined name for this index. The form of this name for composite indexes will be: `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/indexes/{composite_index_id}` For single field indexes, this field will be empty."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "queryScope")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indexes with a collection query scope specified allow queries against a collection that is the child of a specific document, specified at query time, and that has the same collection id. Indexes with a collection group query scope specified allow queries against all collections descended from a specific document, specified at query time, and that have the same collection id as this index."]
    pub query_scope: ::std::option::Option<GoogleFirestoreAdminV1IndexQueryScopeEnum>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The serving state of the index."]
    pub state: ::std::option::Option<GoogleFirestoreAdminV1IndexStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Indexes with a collection query scope specified allow queries against a collection that is the child of a specific document, specified at query time, and that has the same collection id. Indexes with a collection group query scope specified allow queries against all collections descended from a specific document, specified at query time, and that have the same collection id as this index."]
pub enum GoogleFirestoreAdminV1IndexQueryScopeEnum {
    #[serde(rename = "QUERY_SCOPE_UNSPECIFIED")]
    #[doc = "The query scope is unspecified. Not a valid option."]
    QueryScopeUnspecified,
    #[serde(rename = "COLLECTION")]
    #[doc = "Indexes with a collection query scope specified allow queries against a collection that is the child of a specific document, specified at query time, and that has the collection id specified by the index."]
    Collection,
    #[serde(rename = "COLLECTION_GROUP")]
    #[doc = "Indexes with a collection group query scope specified allow queries against all collections that has the collection id specified by the index."]
    CollectionGroup,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The serving state of the index."]
pub enum GoogleFirestoreAdminV1IndexStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "The state is unspecified."]
    StateUnspecified,
    #[serde(rename = "CREATING")]
    #[doc = "The index is being created. There is an active long-running operation for the index. The index is updated when writing a document. Some index data may exist."]
    Creating,
    #[serde(rename = "READY")]
    #[doc = "The index is ready to be used. The index is updated when writing a document. The index is fully populated from all stored documents it applies to."]
    Ready,
    #[serde(rename = "NEEDS_REPAIR")]
    #[doc = "The index was being created, but something went wrong. There is no active long-running operation for the index, and the most recently finished long-running operation failed. The index is not updated when writing a document. Some index data may exist. Use the google.longrunning.Operations API to determine why the operation that last attempted to create this index failed, then re-create the index."]
    NeedsRepair,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The index configuration for this field."]
pub struct GoogleFirestoreAdminV1IndexConfig {
    #[serde(rename = "ancestorField")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Specifies the resource name of the `Field` from which this field's index configuration is set (when `uses_ancestor_config` is true), or from which it *would* be set if this field had no index configuration (when `uses_ancestor_config` is false)."]
    pub ancestor_field: ::std::option::Option<::std::string::String>,
    #[serde(rename = "indexes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The indexes supported for this field."]
    pub indexes:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleFirestoreAdminV1Index>>>,
    #[serde(rename = "reverting")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only When true, the `Field`'s index configuration is in the process of being reverted. Once complete, the index config will transition to the same state as the field specified by `ancestor_field`, at which point `uses_ancestor_config` will be `true` and `reverting` will be `false`."]
    pub reverting: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "usesAncestorConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. When true, the `Field`'s index configuration is set from the configuration specified by the `ancestor_field`. When false, the `Field`'s index configuration is defined explicitly."]
    pub uses_ancestor_config: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about an index configuration change."]
pub struct GoogleFirestoreAdminV1IndexConfigDelta {
    #[serde(rename = "changeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies how the index is changing."]
    pub change_type: ::std::option::Option<GoogleFirestoreAdminV1IndexConfigDeltaChangeTypeEnum>,
    #[serde(rename = "index")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The index being changed."]
    pub index: ::std::option::Option<::std::boxed::Box<GoogleFirestoreAdminV1Index>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Specifies how the index is changing."]
pub enum GoogleFirestoreAdminV1IndexConfigDeltaChangeTypeEnum {
    #[serde(rename = "CHANGE_TYPE_UNSPECIFIED")]
    #[doc = "The type of change is not specified or known."]
    ChangeTypeUnspecified,
    #[serde(rename = "ADD")]
    #[doc = "The single field index is being added."]
    Add,
    #[serde(rename = "REMOVE")]
    #[doc = "The single field index is being removed."]
    Remove,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A field in an index. The field_path describes which field is indexed, the value_mode describes how the field value is indexed."]
pub struct GoogleFirestoreAdminV1IndexField {
    #[serde(rename = "arrayConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates that this field supports operations on `array_value`s."]
    pub array_config: ::std::option::Option<GoogleFirestoreAdminV1IndexFieldArrayConfigEnum>,
    #[serde(rename = "fieldPath")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Can be __name__. For single field indexes, this must match the name of the field or may be omitted."]
    pub field_path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "order")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates that this field supports ordering by the specified order or comparing using =, <, <=, >, >=."]
    pub order: ::std::option::Option<GoogleFirestoreAdminV1IndexFieldOrderEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Indicates that this field supports operations on `array_value`s."]
pub enum GoogleFirestoreAdminV1IndexFieldArrayConfigEnum {
    #[serde(rename = "ARRAY_CONFIG_UNSPECIFIED")]
    #[doc = "The index does not support additional array queries."]
    ArrayConfigUnspecified,
    #[serde(rename = "CONTAINS")]
    #[doc = "The index supports array containment queries."]
    Contains,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Indicates that this field supports ordering by the specified order or comparing using =, <, <=, >, >=."]
pub enum GoogleFirestoreAdminV1IndexFieldOrderEnum {
    #[serde(rename = "ORDER_UNSPECIFIED")]
    #[doc = "The ordering is unspecified. Not a valid option."]
    OrderUnspecified,
    #[serde(rename = "ASCENDING")]
    #[doc = "The field is ordered by ascending field value."]
    Ascending,
    #[serde(rename = "DESCENDING")]
    #[doc = "The field is ordered by descending field value."]
    Descending,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for google.longrunning.Operation results from FirestoreAdmin.CreateIndex."]
pub struct GoogleFirestoreAdminV1IndexOperationMetadata {
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time this operation completed. Will be unset if operation still in progress."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "index")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The index resource that this operation is acting on. For example: `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/indexes/{index_id}`"]
    pub index: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progressBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The progress, in bytes, of this operation."]
    pub progress_bytes: ::std::option::Option<::std::boxed::Box<GoogleFirestoreAdminV1Progress>>,
    #[serde(rename = "progressDocuments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The progress, in documents, of this operation."]
    pub progress_documents:
        ::std::option::Option<::std::boxed::Box<GoogleFirestoreAdminV1Progress>>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time this operation started."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state of the operation."]
    pub state: ::std::option::Option<GoogleFirestoreAdminV1IndexOperationMetadataStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The state of the operation."]
pub enum GoogleFirestoreAdminV1IndexOperationMetadataStateEnum {
    #[serde(rename = "OPERATION_STATE_UNSPECIFIED")]
    #[doc = "Unspecified."]
    OperationStateUnspecified,
    #[serde(rename = "INITIALIZING")]
    #[doc = "Request is being prepared for processing."]
    Initializing,
    #[serde(rename = "PROCESSING")]
    #[doc = "Request is actively being processed."]
    Processing,
    #[serde(rename = "CANCELLING")]
    #[doc = "Request is in the process of being cancelled after user called google.longrunning.Operations.CancelOperation on the operation."]
    Cancelling,
    #[serde(rename = "FINALIZING")]
    #[doc = "Request has been processed and is in its finalization stage."]
    Finalizing,
    #[serde(rename = "SUCCESSFUL")]
    #[doc = "Request has completed successfully."]
    Successful,
    #[serde(rename = "FAILED")]
    #[doc = "Request has finished being processed, but encountered an error."]
    Failed,
    #[serde(rename = "CANCELLED")]
    #[doc = "Request has finished being cancelled after user called google.longrunning.Operations.CancelOperation."]
    Cancelled,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for FirestoreAdmin.ListFields."]
pub struct GoogleFirestoreAdminV1ListFieldsResponse {
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The requested fields."]
    pub fields:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleFirestoreAdminV1Field>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A page token that may be used to request another page of results. If blank, this is the last page."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for FirestoreAdmin.ListIndexes."]
pub struct GoogleFirestoreAdminV1ListIndexesResponse {
    #[serde(rename = "indexes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The requested indexes."]
    pub indexes:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleFirestoreAdminV1Index>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A page token that may be used to request another page of results. If blank, this is the last page."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The metadata message for google.cloud.location.Location.metadata."]
pub struct GoogleFirestoreAdminV1LocationMetadata {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes the progress of the operation. Unit of work is generic and must be interpreted based on where Progress is used."]
pub struct GoogleFirestoreAdminV1Progress {
    #[serde(rename = "completedWork")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount of work completed."]
    pub completed_work: ::std::option::Option<::std::string::String>,
    #[serde(rename = "estimatedWork")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount of work estimated."]
    pub estimated_work: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for Operations.CancelOperation."]
pub struct GoogleLongrunningCancelOperationRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for Operations.ListOperations."]
pub struct GoogleLongrunningListOperationsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The standard List next-page token."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of operations that matches the specified filter in the request."]
    pub operations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleLongrunningOperation>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This resource represents a long-running operation that is the result of a network API call."]
pub struct GoogleLongrunningOperation {
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
#[doc = "An object representing a latitude/longitude pair. This is expressed as a pair of doubles representing degrees latitude and degrees longitude. Unless specified otherwise, this must conform to the WGS84 standard. Values must be within normalized ranges."]
pub struct LatLng {
    #[serde(rename = "latitude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The latitude in degrees. It must be in the range [-90.0, +90.0]."]
    pub latitude: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "longitude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The longitude in degrees. It must be in the range [-180.0, +180.0]."]
    pub longitude: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for Firestore.ListCollectionIds."]
pub struct ListCollectionIdsRequest {
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum number of results to return."]
    pub page_size: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A page token. Must be a value from ListCollectionIdsResponse."]
    pub page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response from Firestore.ListCollectionIds."]
pub struct ListCollectionIdsResponse {
    #[serde(rename = "collectionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The collection ids."]
    pub collection_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A page token that may be used to continue the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for Firestore.ListDocuments."]
pub struct ListDocumentsResponse {
    #[serde(rename = "documents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Documents found."]
    pub documents: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Document>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The next page token."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
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
#[doc = "A request for Firestore.Listen"]
pub struct ListenRequest {
    #[serde(rename = "addTarget")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A target to add to this stream."]
    pub add_target: ::std::option::Option<::std::boxed::Box<Target>>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Labels associated with this target change."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "removeTarget")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of a target to remove from this stream."]
    pub remove_target: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for Firestore.Listen."]
pub struct ListenResponse {
    #[serde(rename = "documentChange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A Document has changed."]
    pub document_change: ::std::option::Option<::std::boxed::Box<DocumentChange>>,
    #[serde(rename = "documentDelete")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A Document has been deleted."]
    pub document_delete: ::std::option::Option<::std::boxed::Box<DocumentDelete>>,
    #[serde(rename = "documentRemove")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A Document has been removed from a target (because it is no longer relevant to that target)."]
    pub document_remove: ::std::option::Option<::std::boxed::Box<DocumentRemove>>,
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A filter to apply to the set of documents previously returned for the given target. Returned when documents may have been removed from the given target, but the exact documents are unknown."]
    pub filter: ::std::option::Option<::std::boxed::Box<ExistenceFilter>>,
    #[serde(rename = "targetChange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Targets have changed."]
    pub target_change: ::std::option::Option<::std::boxed::Box<TargetChange>>,
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
#[doc = "A map value."]
pub struct MapValue {
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The map's fields. The map keys represent field names. Field names matching the regular expression `__.*__` are reserved. Reserved field names are forbidden except in certain documented contexts. The map keys, represented as UTF-8, must not exceed 1,500 bytes and cannot be empty."]
    pub fields:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<Value>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An order on a field."]
pub struct Order {
    #[serde(rename = "direction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The direction to order by. Defaults to `ASCENDING`."]
    pub direction: ::std::option::Option<OrderDirectionEnum>,
    #[serde(rename = "field")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The field to order by."]
    pub field: ::std::option::Option<::std::boxed::Box<FieldReference>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The direction to order by. Defaults to `ASCENDING`."]
pub enum OrderDirectionEnum {
    #[serde(rename = "DIRECTION_UNSPECIFIED")]
    #[doc = "Unspecified."]
    DirectionUnspecified,
    #[serde(rename = "ASCENDING")]
    #[doc = "Ascending."]
    Ascending,
    #[serde(rename = "DESCENDING")]
    #[doc = "Descending."]
    Descending,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for Firestore.PartitionQuery."]
pub struct PartitionQueryRequest {
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum number of partitions to return in this call, subject to `partition_count`. For example, if `partition_count` = 10 and `page_size` = 8, the first call to PartitionQuery will return up to 8 partitions and a `next_page_token` if more results exist. A second call to PartitionQuery will return up to 2 partitions, to complete the total of 10 specified in `partition_count`."]
    pub page_size: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `next_page_token` value returned from a previous call to PartitionQuery that may be used to get an additional set of results. There are no ordering guarantees between sets of results. Thus, using multiple sets of results will require merging the different result sets. For example, two subsequent calls using a page_token may return: * cursor B, cursor M, cursor Q * cursor A, cursor U, cursor W To obtain a complete result set ordered with respect to the results of the query supplied to PartitionQuery, the results sets should be merged: cursor A, cursor B, cursor M, cursor Q, cursor U, cursor W"]
    pub page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "partitionCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The desired maximum number of partition points. The partitions may be returned across multiple pages of results. The number must be positive. The actual number of partitions returned may be fewer. For example, this may be set to one fewer than the number of parallel queries to be run, or in running a data pipeline job, one fewer than the number of workers or compute instances available."]
    pub partition_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "structuredQuery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A structured query. Query must specify collection with all descendants and be ordered by name ascending. Other filters, order bys, limits, offsets, and start/end cursors are not supported."]
    pub structured_query: ::std::option::Option<::std::boxed::Box<StructuredQuery>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for Firestore.PartitionQuery."]
pub struct PartitionQueryResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A page token that may be used to request an additional set of results, up to the number specified by `partition_count` in the PartitionQuery request. If blank, there are no more results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "partitions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Partition results. Each partition is a split point that can be used by RunQuery as a starting or end point for the query results. The RunQuery requests must be made with the same query supplied to this PartitionQuery request. The partition cursors will be ordered according to same ordering as the results of the query supplied to PartitionQuery. For example, if a PartitionQuery request returns partition cursors A and B, running the following three queries will return the entire result set of the original query: * query, end_at A * query, start_at A, end_at B * query, start_at B An empty result may indicate that the query has too few results to be partitioned."]
    pub partitions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Cursor>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A precondition on a document, used for conditional operations."]
pub struct Precondition {
    #[serde(rename = "exists")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When set to `true`, the target document must exist. When set to `false`, the target document must not exist."]
    pub exists: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When set, the target document must exist and have been last updated at that time."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The projection of document's fields to return."]
pub struct Projection {
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fields to return. If empty, all fields are returned. To only return the name of the document, use `['__name__']`."]
    pub fields: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FieldReference>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A target specified by a query."]
pub struct QueryTarget {
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The parent resource name. In the format: `projects/{project_id}/databases/{database_id}/documents` or `projects/{project_id}/databases/{database_id}/documents/{document_path}`. For example: `projects/my-project/databases/my-database/documents` or `projects/my-project/databases/my-database/documents/chatrooms/my-chatroom`"]
    pub parent: ::std::option::Option<::std::string::String>,
    #[serde(rename = "structuredQuery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A structured query."]
    pub structured_query: ::std::option::Option<::std::boxed::Box<StructuredQuery>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Options for a transaction that can only be used to read documents."]
pub struct ReadOnly {
    #[serde(rename = "readTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reads documents at the given time. This may not be older than 60 seconds."]
    pub read_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Options for a transaction that can be used to read and write documents."]
pub struct ReadWrite {
    #[serde(rename = "retryTransaction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An optional transaction to retry."]
    pub retry_transaction: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for Firestore.Rollback."]
pub struct RollbackRequest {
    #[serde(rename = "transaction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The transaction to roll back."]
    pub transaction: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for Firestore.RunQuery."]
pub struct RunQueryRequest {
    #[serde(rename = "newTransaction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Starts a new transaction and reads the documents. Defaults to a read-only transaction. The new transaction ID will be returned as the first response in the stream."]
    pub new_transaction: ::std::option::Option<::std::boxed::Box<TransactionOptions>>,
    #[serde(rename = "readTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reads documents as they were at the given time. This may not be older than 270 seconds."]
    pub read_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "structuredQuery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A structured query."]
    pub structured_query: ::std::option::Option<::std::boxed::Box<StructuredQuery>>,
    #[serde(rename = "transaction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reads documents in a transaction."]
    pub transaction: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for Firestore.RunQuery."]
pub struct RunQueryResponse {
    #[serde(rename = "document")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A query result. Not set when reporting partial progress."]
    pub document: ::std::option::Option<::std::boxed::Box<Document>>,
    #[serde(rename = "readTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the document was read. This may be monotonically increasing; in this case, the previous documents in the result stream are guaranteed not to have changed between their `read_time` and this one. If the query returns no results, a response with `read_time` and no `document` will be sent, and this represents the time at which the query was run."]
    pub read_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "skippedResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of results that have been skipped due to an offset between the last response and the current response."]
    pub skipped_results: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "transaction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The transaction that was started as part of this request. Can only be set in the first response, and only if RunQueryRequest.new_transaction was set in the request. If set, no other fields will be set in this response."]
    pub transaction: ::std::option::Option<::std::string::String>,
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
#[doc = "A Firestore query."]
pub struct StructuredQuery {
    #[serde(rename = "endAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A end point for the query results."]
    pub end_at: ::std::option::Option<::std::boxed::Box<Cursor>>,
    #[serde(rename = "from")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The collections to query."]
    pub from: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CollectionSelector>>>,
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum number of results to return. Applies after all other constraints. Must be >= 0 if specified."]
    pub limit: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of results to skip. Applies before limit, but after all other constraints. Must be >= 0 if specified."]
    pub offset: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "orderBy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The order to apply to the query results. Firestore guarantees a stable ordering through the following rules: * Any field required to appear in `order_by`, that is not already specified in `order_by`, is appended to the order in field name order by default. * If an order on `__name__` is not specified, it is appended by default. Fields are appended with the same sort direction as the last order specified, or 'ASCENDING' if no order was specified. For example: * `SELECT * FROM Foo ORDER BY A` becomes `SELECT * FROM Foo ORDER BY A, __name__` * `SELECT * FROM Foo ORDER BY A DESC` becomes `SELECT * FROM Foo ORDER BY A DESC, __name__ DESC` * `SELECT * FROM Foo WHERE A > 1` becomes `SELECT * FROM Foo WHERE A > 1 ORDER BY A, __name__`"]
    pub order_by: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Order>>>,
    #[serde(rename = "select")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The projection to return."]
    pub select: ::std::option::Option<::std::boxed::Box<Projection>>,
    #[serde(rename = "startAt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A starting point for the query results."]
    pub start_at: ::std::option::Option<::std::boxed::Box<Cursor>>,
    #[serde(rename = "where")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The filter to apply."]
    pub _where: ::std::option::Option<::std::boxed::Box<Filter>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A specification of a set of documents to listen to."]
pub struct Target {
    #[serde(rename = "documents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A target specified by a set of document names."]
    pub documents: ::std::option::Option<::std::boxed::Box<DocumentsTarget>>,
    #[serde(rename = "once")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the target should be removed once it is current and consistent."]
    pub once: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A target specified by a query."]
    pub query: ::std::option::Option<::std::boxed::Box<QueryTarget>>,
    #[serde(rename = "readTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Start listening after a specific `read_time`. The client must know the state of matching documents at this time."]
    pub read_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resumeToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A resume token from a prior TargetChange for an identical target. Using a resume token with a different target is unsupported and may fail."]
    pub resume_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target ID that identifies the target on the stream. Must be a positive number and non-zero."]
    pub target_id: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Targets being watched have changed."]
pub struct TargetChange {
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The error that resulted in this change, if applicable."]
    pub cause: ::std::option::Option<::std::boxed::Box<Status>>,
    #[serde(rename = "readTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The consistent `read_time` for the given `target_ids` (omitted when the target_ids are not at a consistent snapshot). The stream is guaranteed to send a `read_time` with `target_ids` empty whenever the entire stream reaches a new consistent snapshot. ADD, CURRENT, and RESET messages are guaranteed to (eventually) result in a new consistent snapshot (while NO_CHANGE and REMOVE messages are not). For a given stream, `read_time` is guaranteed to be monotonically increasing."]
    pub read_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resumeToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token that can be used to resume the stream for the given `target_ids`, or all targets if `target_ids` is empty. Not set on every target change."]
    pub resume_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetChangeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of change that occurred."]
    pub target_change_type: ::std::option::Option<TargetChangeTargetChangeTypeEnum>,
    #[serde(rename = "targetIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target IDs of targets that have changed. If empty, the change applies to all targets. The order of the target IDs is not defined."]
    pub target_ids: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of change that occurred."]
pub enum TargetChangeTargetChangeTypeEnum {
    #[serde(rename = "NO_CHANGE")]
    #[doc = "No change has occurred. Used only to send an updated `resume_token`."]
    NoChange,
    #[serde(rename = "ADD")]
    #[doc = "The targets have been added."]
    Add,
    #[serde(rename = "REMOVE")]
    #[doc = "The targets have been removed."]
    Remove,
    #[serde(rename = "CURRENT")]
    #[doc = "The targets reflect all changes committed before the targets were added to the stream. This will be sent after or with a `read_time` that is greater than or equal to the time at which the targets were added. Listeners can wait for this change if read-after-write semantics are desired."]
    Current,
    #[serde(rename = "RESET")]
    #[doc = "The targets have been reset, and a new initial state for the targets will be returned in subsequent changes. After the initial state is complete, `CURRENT` will be returned even if the target was previously indicated to be `CURRENT`."]
    Reset,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Options for creating a new transaction."]
pub struct TransactionOptions {
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The transaction can only be used for read operations."]
    pub read_only: ::std::option::Option<::std::boxed::Box<ReadOnly>>,
    #[serde(rename = "readWrite")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The transaction can be used for both read and write operations."]
    pub read_write: ::std::option::Option<::std::boxed::Box<ReadWrite>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A filter with a single operand."]
pub struct UnaryFilter {
    #[serde(rename = "field")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The field to which to apply the operator."]
    pub field: ::std::option::Option<::std::boxed::Box<FieldReference>>,
    #[serde(rename = "op")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unary operator to apply."]
    pub op: ::std::option::Option<UnaryFilterOpEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The unary operator to apply."]
pub enum UnaryFilterOpEnum {
    #[serde(rename = "OPERATOR_UNSPECIFIED")]
    #[doc = "Unspecified. This value must not be used."]
    OperatorUnspecified,
    #[serde(rename = "IS_NAN")]
    #[doc = "The given `field` is equal to `NaN`."]
    IsNan,
    #[serde(rename = "IS_NULL")]
    #[doc = "The given `field` is equal to `NULL`."]
    IsNull,
    #[serde(rename = "IS_NOT_NAN")]
    #[doc = "The given `field` is not equal to `NaN`. Requires: * No other `NOT_EQUAL`, `NOT_IN`, `IS_NOT_NULL`, or `IS_NOT_NAN`. * That `field` comes first in the `order_by`."]
    IsNotNan,
    #[serde(rename = "IS_NOT_NULL")]
    #[doc = "The given `field` is not equal to `NULL`. Requires: * A single `NOT_EQUAL`, `NOT_IN`, `IS_NOT_NULL`, or `IS_NOT_NAN`. * That `field` comes first in the `order_by`."]
    IsNotNull,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A message that can hold any of the supported value types."]
pub struct Value {
    #[serde(rename = "arrayValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An array value. Cannot directly contain another array value, though can contain an map which contains another array."]
    pub array_value: ::std::option::Option<::std::boxed::Box<ArrayValue>>,
    #[serde(rename = "booleanValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A boolean value."]
    pub boolean_value: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "bytesValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A bytes value. Must not exceed 1 MiB - 89 bytes. Only the first 1,500 bytes are considered by queries."]
    pub bytes_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "doubleValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A double value."]
    pub double_value: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "geoPointValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A geo point value representing a point on the surface of Earth."]
    pub geo_point_value: ::std::option::Option<::std::boxed::Box<LatLng>>,
    #[serde(rename = "integerValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An integer value."]
    pub integer_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mapValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A map value."]
    pub map_value: ::std::option::Option<::std::boxed::Box<MapValue>>,
    #[serde(rename = "nullValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A null value."]
    pub null_value: ::std::option::Option<ValueNullValueEnum>,
    #[serde(rename = "referenceValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A reference to a document. For example: `projects/{project_id}/databases/{database_id}/documents/{document_path}`."]
    pub reference_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "stringValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A string value. The string, represented as UTF-8, must not exceed 1 MiB - 89 bytes. Only the first 1,500 bytes of the UTF-8 representation are considered by queries."]
    pub string_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timestampValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A timestamp value. Precise only to microseconds. When stored, any additional precision is rounded down."]
    pub timestamp_value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "A null value."]
pub enum ValueNullValueEnum {
    #[serde(rename = "NULL_VALUE")]
    #[doc = "Null value."]
    NullValue,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A write on a document."]
pub struct Write {
    #[serde(rename = "currentDocument")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An optional precondition on the document. The write will fail if this is set and not met by the target document."]
    pub current_document: ::std::option::Option<::std::boxed::Box<Precondition>>,
    #[serde(rename = "delete")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A document name to delete. In the format: `projects/{project_id}/databases/{database_id}/documents/{document_path}`."]
    pub delete: ::std::option::Option<::std::string::String>,
    #[serde(rename = "transform")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Applies a transformation to a document."]
    pub transform: ::std::option::Option<::std::boxed::Box<DocumentTransform>>,
    #[serde(rename = "update")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A document to write."]
    pub update: ::std::option::Option<::std::boxed::Box<Document>>,
    #[serde(rename = "updateMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fields to update in this write. This field can be set only when the operation is `update`. If the mask is not set for an `update` and the document exists, any existing data will be overwritten. If the mask is set and the document on the server has fields not covered by the mask, they are left unchanged. Fields referenced in the mask, but not present in the input document, are deleted from the document on the server. The field paths in this mask must not contain a reserved field name."]
    pub update_mask: ::std::option::Option<::std::boxed::Box<DocumentMask>>,
    #[serde(rename = "updateTransforms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The transforms to perform after update. This field can be set only when the operation is `update`. If present, this write is equivalent to performing `update` and `transform` to the same document atomically and in order."]
    pub update_transforms:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FieldTransform>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for Firestore.Write. The first request creates a stream, or resumes an existing one from a token. When creating a new stream, the server replies with a response containing only an ID and a token, to use in the next request. When resuming a stream, the server first streams any responses later than the given token, then a response containing only an up-to-date token, to use in the next request."]
pub struct WriteRequest {
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Labels associated with this write request."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "streamId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the write stream to resume. This may only be set in the first message. When left empty, a new write stream will be created."]
    pub stream_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "streamToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A stream token that was previously sent by the server. The client should set this field to the token from the most recent WriteResponse it has received. This acknowledges that the client has received responses up to this token. After sending this token, earlier tokens may not be used anymore. The server may close the stream if there are too many unacknowledged responses. Leave this field unset when creating a new stream. To resume a stream at a specific point, set this field and the `stream_id` field. Leave this field unset when creating a new stream."]
    pub stream_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "writes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The writes to apply. Always executed atomically and in order. This must be empty on the first request. This may be empty on the last request. This must not be empty on all other requests."]
    pub writes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Write>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for Firestore.Write."]
pub struct WriteResponse {
    #[serde(rename = "commitTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the commit occurred. Any read with an equal or greater `read_time` is guaranteed to see the effects of the write."]
    pub commit_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "streamId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the stream. Only set on the first message, when a new stream was created."]
    pub stream_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "streamToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token that represents the position of this response in the stream. This can be used by a client to resume the stream at this point. This field is always set."]
    pub stream_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "writeResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of applying the writes. This i-th write result corresponds to the i-th write in the request."]
    pub write_results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WriteResult>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The result of applying a write."]
pub struct WriteResult {
    #[serde(rename = "transformResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The results of applying each DocumentTransform.FieldTransform, in the same order."]
    pub transform_results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Value>>>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last update time of the document after applying the write. Not set after a `delete`. If the write did not actually change the document, this will be the previous update_time."]
    pub update_time: ::std::option::Option<::std::string::String>,
}