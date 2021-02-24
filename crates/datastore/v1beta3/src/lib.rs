#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for Datastore.AllocateIds."]
pub struct AllocateIdsRequest {
    #[serde(rename = "keys")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A list of keys with incomplete key paths for which to allocate IDs. No key may be reserved/read-only."]
    pub keys: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Key>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for Datastore.AllocateIds."]
pub struct AllocateIdsResponse {
    #[serde(rename = "keys")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The keys specified in the request (in the same order), each with its key path completed with a newly allocated ID."]
    pub keys: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Key>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An array value."]
pub struct ArrayValue {
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Values in the array. The order of values in an array is preserved as long as all values have identical settings for 'exclude_from_indexes'."]
    pub values: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Value>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for Datastore.BeginTransaction."]
pub struct BeginTransactionRequest {
    #[serde(rename = "transactionOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Options for a new transaction."]
    pub transaction_options: ::std::option::Option<::std::boxed::Box<TransactionOptions>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for Datastore.BeginTransaction."]
pub struct BeginTransactionResponse {
    #[serde(rename = "transaction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The transaction identifier (always present)."]
    pub transaction: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for Datastore.Commit."]
pub struct CommitRequest {
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of commit to perform. Defaults to `TRANSACTIONAL`."]
    pub mode: ::std::option::Option<CommitRequestModeEnum>,
    #[serde(rename = "mutations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The mutations to perform. When mode is `TRANSACTIONAL`, mutations affecting a single entity are applied in order. The following sequences of mutations affecting a single entity are not permitted in a single `Commit` request: - `insert` followed by `insert` - `update` followed by `insert` - `upsert` followed by `insert` - `delete` followed by `update` When mode is `NON_TRANSACTIONAL`, no two mutations may affect a single entity."]
    pub mutations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Mutation>>>,
    #[serde(rename = "transaction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier of the transaction associated with the commit. A transaction identifier is returned by a call to Datastore.BeginTransaction."]
    pub transaction: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of commit to perform. Defaults to `TRANSACTIONAL`."]
pub enum CommitRequestModeEnum {
    #[serde(rename = "MODE_UNSPECIFIED")]
    #[doc = "Unspecified. This value must not be used."]
    ModeUnspecified,
    #[serde(rename = "TRANSACTIONAL")]
    #[doc = "Transactional: The mutations are either all applied, or none are applied. Learn about transactions [here](https://cloud.google.com/datastore/docs/concepts/transactions)."]
    Transactional,
    #[serde(rename = "NON_TRANSACTIONAL")]
    #[doc = "Non-transactional: The mutations may not apply as all or none."]
    NonTransactional,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for Datastore.Commit."]
pub struct CommitResponse {
    #[serde(rename = "indexUpdates")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of index entries updated during the commit, or zero if none were updated."]
    pub index_updates: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "mutationResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of performing the mutations. The i-th mutation result corresponds to the i-th mutation in the request."]
    pub mutation_results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MutationResult>>>,
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
#[doc = "A Datastore data object. An entity is limited to 1 megabyte when stored. That _roughly_ corresponds to a limit of 1 megabyte for the serialized form of this message."]
pub struct Entity {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The entity's key. An entity must have a key, unless otherwise documented (for example, an entity in `Value.entity_value` may have no key). An entity's kind is its key path's last element's kind, or null if it has no key."]
    pub key: ::std::option::Option<::std::boxed::Box<Key>>,
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The entity's properties. The map's keys are property names. A property name matching regex `__.*__` is reserved. A reserved property name is forbidden in certain documented contexts. The name must not contain more than 500 characters. The name cannot be `\"\"`."]
    pub properties:
        ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<Value>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The result of fetching an entity from Datastore."]
pub struct EntityResult {
    #[serde(rename = "cursor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A cursor that points to the position after the result entity. Set only when the `EntityResult` is part of a `QueryResultBatch` message."]
    pub cursor: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resulting entity."]
    pub entity: ::std::option::Option<::std::boxed::Box<Entity>>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version of the entity, a strictly positive number that monotonically increases with changes to the entity. This field is set for `FULL` entity results. For missing entities in `LookupResponse`, this is the version of the snapshot that was used to look up the entity, and it is always set except for eventually consistent reads."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A holder for any type of filter."]
pub struct Filter {
    #[serde(rename = "compositeFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A composite filter."]
    pub composite_filter: ::std::option::Option<::std::boxed::Box<CompositeFilter>>,
    #[serde(rename = "propertyFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A filter on a property."]
    pub property_filter: ::std::option::Option<::std::boxed::Box<PropertyFilter>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata common to all Datastore Admin operations."]
pub struct GoogleDatastoreAdminV1CommonMetadata {
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time the operation ended, either successfully or otherwise."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The client-assigned labels which were provided when the operation was created. May also include additional labels."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "operationType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the operation. Can be used as a filter in ListOperationsRequest."]
    pub operation_type:
        ::std::option::Option<GoogleDatastoreAdminV1CommonMetadataOperationTypeEnum>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time that work began on the operation."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current state of the Operation."]
    pub state: ::std::option::Option<GoogleDatastoreAdminV1CommonMetadataStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the operation. Can be used as a filter in ListOperationsRequest."]
pub enum GoogleDatastoreAdminV1CommonMetadataOperationTypeEnum {
    #[serde(rename = "OPERATION_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified."]
    OperationTypeUnspecified,
    #[serde(rename = "EXPORT_ENTITIES")]
    #[doc = "ExportEntities."]
    ExportEntities,
    #[serde(rename = "IMPORT_ENTITIES")]
    #[doc = "ImportEntities."]
    ImportEntities,
    #[serde(rename = "CREATE_INDEX")]
    #[doc = "CreateIndex."]
    CreateIndex,
    #[serde(rename = "DELETE_INDEX")]
    #[doc = "DeleteIndex."]
    DeleteIndex,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The current state of the Operation."]
pub enum GoogleDatastoreAdminV1CommonMetadataStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Unspecified."]
    StateUnspecified,
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
#[doc = "Identifies a subset of entities in a project. This is specified as combinations of kinds and namespaces (either or both of which may be all, as described in the following examples). Example usage: Entire project: kinds=[], namespace_ids=[] Kinds Foo and Bar in all namespaces: kinds=['Foo', 'Bar'], namespace_ids=[] Kinds Foo and Bar only in the default namespace: kinds=['Foo', 'Bar'], namespace_ids=[''] Kinds Foo and Bar in both the default and Baz namespaces: kinds=['Foo', 'Bar'], namespace_ids=['', 'Baz'] The entire Baz namespace: kinds=[], namespace_ids=['Baz']"]
pub struct GoogleDatastoreAdminV1EntityFilter {
    #[serde(rename = "kinds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If empty, then this represents all kinds."]
    pub kinds: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "namespaceIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An empty list represents all namespaces. This is the preferred usage for projects that don't use namespaces. An empty string element represents the default namespace. This should be used if the project has data in non-default namespaces, but doesn't want to include them. Each namespace in this list must be unique."]
    pub namespace_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for ExportEntities operations."]
pub struct GoogleDatastoreAdminV1ExportEntitiesMetadata {
    #[serde(rename = "common")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata common to all Datastore Admin operations."]
    pub common: ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1CommonMetadata>>,
    #[serde(rename = "entityFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of which entities are being exported."]
    pub entity_filter: ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1EntityFilter>>,
    #[serde(rename = "outputUrlPrefix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Location for the export metadata and data files. This will be the same value as the google.datastore.admin.v1.ExportEntitiesRequest.output_url_prefix field. The final output location is provided in google.datastore.admin.v1.ExportEntitiesResponse.output_url."]
    pub output_url_prefix: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progressBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An estimate of the number of bytes processed."]
    pub progress_bytes: ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1Progress>>,
    #[serde(rename = "progressEntities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An estimate of the number of entities processed."]
    pub progress_entities: ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1Progress>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for google.datastore.admin.v1.DatastoreAdmin.ExportEntities."]
pub struct GoogleDatastoreAdminV1ExportEntitiesResponse {
    #[serde(rename = "outputUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Location of the output metadata file. This can be used to begin an import into Cloud Datastore (this project or another project). See google.datastore.admin.v1.ImportEntitiesRequest.input_url. Only present if the operation completed successfully."]
    pub output_url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for ImportEntities operations."]
pub struct GoogleDatastoreAdminV1ImportEntitiesMetadata {
    #[serde(rename = "common")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata common to all Datastore Admin operations."]
    pub common: ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1CommonMetadata>>,
    #[serde(rename = "entityFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of which entities are being imported."]
    pub entity_filter: ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1EntityFilter>>,
    #[serde(rename = "inputUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The location of the import metadata file. This will be the same value as the google.datastore.admin.v1.ExportEntitiesResponse.output_url field."]
    pub input_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progressBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An estimate of the number of bytes processed."]
    pub progress_bytes: ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1Progress>>,
    #[serde(rename = "progressEntities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An estimate of the number of entities processed."]
    pub progress_entities: ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1Progress>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for Index operations."]
pub struct GoogleDatastoreAdminV1IndexOperationMetadata {
    #[serde(rename = "common")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata common to all Datastore Admin operations."]
    pub common: ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1CommonMetadata>>,
    #[serde(rename = "indexId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The index resource ID that this operation is acting on."]
    pub index_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progressEntities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An estimate of the number of entities processed."]
    pub progress_entities: ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1Progress>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Measures the progress of a particular metric."]
pub struct GoogleDatastoreAdminV1Progress {
    #[serde(rename = "workCompleted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount of work that has been completed. Note that this may be greater than work_estimated."]
    pub work_completed: ::std::option::Option<::std::string::String>,
    #[serde(rename = "workEstimated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An estimate of how much work needs to be performed. May be zero if the work estimate is unavailable."]
    pub work_estimated: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata common to all Datastore Admin operations."]
pub struct GoogleDatastoreAdminV1beta1CommonMetadata {
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time the operation ended, either successfully or otherwise."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The client-assigned labels which were provided when the operation was created. May also include additional labels."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "operationType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the operation. Can be used as a filter in ListOperationsRequest."]
    pub operation_type:
        ::std::option::Option<GoogleDatastoreAdminV1beta1CommonMetadataOperationTypeEnum>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time that work began on the operation."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current state of the Operation."]
    pub state: ::std::option::Option<GoogleDatastoreAdminV1beta1CommonMetadataStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the operation. Can be used as a filter in ListOperationsRequest."]
pub enum GoogleDatastoreAdminV1beta1CommonMetadataOperationTypeEnum {
    #[serde(rename = "OPERATION_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified."]
    OperationTypeUnspecified,
    #[serde(rename = "EXPORT_ENTITIES")]
    #[doc = "ExportEntities."]
    ExportEntities,
    #[serde(rename = "IMPORT_ENTITIES")]
    #[doc = "ImportEntities."]
    ImportEntities,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The current state of the Operation."]
pub enum GoogleDatastoreAdminV1beta1CommonMetadataStateEnum {
    #[serde(rename = "STATE_UNSPECIFIED")]
    #[doc = "Unspecified."]
    StateUnspecified,
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
#[doc = "Identifies a subset of entities in a project. This is specified as combinations of kinds and namespaces (either or both of which may be all, as described in the following examples). Example usage: Entire project: kinds=[], namespace_ids=[] Kinds Foo and Bar in all namespaces: kinds=['Foo', 'Bar'], namespace_ids=[] Kinds Foo and Bar only in the default namespace: kinds=['Foo', 'Bar'], namespace_ids=[''] Kinds Foo and Bar in both the default and Baz namespaces: kinds=['Foo', 'Bar'], namespace_ids=['', 'Baz'] The entire Baz namespace: kinds=[], namespace_ids=['Baz']"]
pub struct GoogleDatastoreAdminV1beta1EntityFilter {
    #[serde(rename = "kinds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If empty, then this represents all kinds."]
    pub kinds: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "namespaceIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An empty list represents all namespaces. This is the preferred usage for projects that don't use namespaces. An empty string element represents the default namespace. This should be used if the project has data in non-default namespaces, but doesn't want to include them. Each namespace in this list must be unique."]
    pub namespace_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for ExportEntities operations."]
pub struct GoogleDatastoreAdminV1beta1ExportEntitiesMetadata {
    #[serde(rename = "common")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata common to all Datastore Admin operations."]
    pub common: ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1beta1CommonMetadata>>,
    #[serde(rename = "entityFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of which entities are being exported."]
    pub entity_filter:
        ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1beta1EntityFilter>>,
    #[serde(rename = "outputUrlPrefix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Location for the export metadata and data files. This will be the same value as the google.datastore.admin.v1beta1.ExportEntitiesRequest.output_url_prefix field. The final output location is provided in google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url."]
    pub output_url_prefix: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progressBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An estimate of the number of bytes processed."]
    pub progress_bytes:
        ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1beta1Progress>>,
    #[serde(rename = "progressEntities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An estimate of the number of entities processed."]
    pub progress_entities:
        ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1beta1Progress>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for google.datastore.admin.v1beta1.DatastoreAdmin.ExportEntities."]
pub struct GoogleDatastoreAdminV1beta1ExportEntitiesResponse {
    #[serde(rename = "outputUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Location of the output metadata file. This can be used to begin an import into Cloud Datastore (this project or another project). See google.datastore.admin.v1beta1.ImportEntitiesRequest.input_url. Only present if the operation completed successfully."]
    pub output_url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Metadata for ImportEntities operations."]
pub struct GoogleDatastoreAdminV1beta1ImportEntitiesMetadata {
    #[serde(rename = "common")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata common to all Datastore Admin operations."]
    pub common: ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1beta1CommonMetadata>>,
    #[serde(rename = "entityFilter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of which entities are being imported."]
    pub entity_filter:
        ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1beta1EntityFilter>>,
    #[serde(rename = "inputUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The location of the import metadata file. This will be the same value as the google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url field."]
    pub input_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progressBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An estimate of the number of bytes processed."]
    pub progress_bytes:
        ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1beta1Progress>>,
    #[serde(rename = "progressEntities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An estimate of the number of entities processed."]
    pub progress_entities:
        ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1beta1Progress>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Measures the progress of a particular metric."]
pub struct GoogleDatastoreAdminV1beta1Progress {
    #[serde(rename = "workCompleted")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount of work that has been completed. Note that this may be greater than work_estimated."]
    pub work_completed: ::std::option::Option<::std::string::String>,
    #[serde(rename = "workEstimated")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An estimate of how much work needs to be performed. May be zero if the work estimate is unavailable."]
    pub work_estimated: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A [GQL query](https://cloud.google.com/datastore/docs/apis/gql/gql_reference)."]
pub struct GqlQuery {
    #[serde(rename = "allowLiterals")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When false, the query string must not contain any literals and instead must bind all values. For example, `SELECT * FROM Kind WHERE a = 'string literal'` is not allowed, while `SELECT * FROM Kind WHERE a = @value` is."]
    pub allow_literals: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "namedBindings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For each non-reserved named binding site in the query string, there must be a named parameter with that name, but not necessarily the inverse. Key must match regex `A-Za-z_$*`, must not match regex `__.*__`, and must not be `\"\"`."]
    pub named_bindings: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<GqlQueryParameter>>,
    >,
    #[serde(rename = "positionalBindings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Numbered binding site @1 references the first numbered parameter, effectively using 1-based indexing, rather than the usual 0. For each binding site numbered i in `query_string`, there must be an i-th numbered parameter. The inverse must also be true."]
    pub positional_bindings:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GqlQueryParameter>>>,
    #[serde(rename = "queryString")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A string of the format described [here](https://cloud.google.com/datastore/docs/apis/gql/gql_reference)."]
    pub query_string: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A binding parameter for a GQL query."]
pub struct GqlQueryParameter {
    #[serde(rename = "cursor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A query cursor. Query cursors are returned in query result batches."]
    pub cursor: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A value parameter."]
    pub value: ::std::option::Option<::std::boxed::Box<Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A unique identifier for an entity. If a key's partition ID or any of its path kinds or names are reserved/read-only, the key is reserved/read-only. A reserved/read-only key is forbidden in certain documented contexts."]
pub struct Key {
    #[serde(rename = "partitionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Entities are partitioned into subsets, currently identified by a project ID and namespace ID. Queries are scoped to a single partition."]
    pub partition_id: ::std::option::Option<::std::boxed::Box<PartitionId>>,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The entity path. An entity path consists of one or more elements composed of a kind and a string or numerical identifier, which identify entities. The first element identifies a _root entity_, the second element identifies a _child_ of the root entity, the third element identifies a child of the second entity, and so forth. The entities identified by all prefixes of the path are called the element's _ancestors_. An entity path is always fully complete: *all* of the entity's ancestors are required to be in the path along with the entity identifier itself. The only exception is that in some documented cases, the identifier in the last path element (for the entity) itself may be omitted. For example, the last path element of the key of `Mutation.insert` may have no identifier. A path can never be empty, and a path can have at most 100 elements."]
    pub path: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PathElement>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A representation of a kind."]
pub struct KindExpression {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the kind."]
    pub name: ::std::option::Option<::std::string::String>,
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
#[doc = "The request for Datastore.Lookup."]
pub struct LookupRequest {
    #[serde(rename = "keys")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Keys of entities to look up."]
    pub keys: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Key>>>,
    #[serde(rename = "readOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The options for this lookup request."]
    pub read_options: ::std::option::Option<::std::boxed::Box<ReadOptions>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for Datastore.Lookup."]
pub struct LookupResponse {
    #[serde(rename = "deferred")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of keys that were not looked up due to resource constraints. The order of results in this field is undefined and has no relation to the order of the keys in the input."]
    pub deferred: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Key>>>,
    #[serde(rename = "found")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Entities found as `ResultType.FULL` entities. The order of results in this field is undefined and has no relation to the order of the keys in the input."]
    pub found: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EntityResult>>>,
    #[serde(rename = "missing")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Entities not found as `ResultType.KEY_ONLY` entities. The order of results in this field is undefined and has no relation to the order of the keys in the input."]
    pub missing: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EntityResult>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A mutation to apply to an entity."]
pub struct Mutation {
    #[serde(rename = "baseVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version of the entity that this mutation is being applied to. If this does not match the current version on the server, the mutation conflicts."]
    pub base_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "delete")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The key of the entity to delete. The entity may or may not already exist. Must have a complete key path and must not be reserved/read-only."]
    pub delete: ::std::option::Option<::std::boxed::Box<Key>>,
    #[serde(rename = "insert")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The entity to insert. The entity must not already exist. The entity key's final path element may be incomplete."]
    pub insert: ::std::option::Option<::std::boxed::Box<Entity>>,
    #[serde(rename = "update")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The entity to update. The entity must already exist. Must have a complete key path."]
    pub update: ::std::option::Option<::std::boxed::Box<Entity>>,
    #[serde(rename = "upsert")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The entity to upsert. The entity may or may not already exist. The entity key's final path element may be incomplete."]
    pub upsert: ::std::option::Option<::std::boxed::Box<Entity>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The result of applying a mutation."]
pub struct MutationResult {
    #[serde(rename = "conflictDetected")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether a conflict was detected for this mutation. Always false when a conflict detection strategy field is not set in the mutation."]
    pub conflict_detected: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The automatically allocated key. Set only when the mutation allocated a key."]
    pub key: ::std::option::Option<::std::boxed::Box<Key>>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version of the entity on the server after processing the mutation. If the mutation doesn't change anything on the server, then the version will be the version of the current entity or, if no entity is present, a version that is strictly greater than the version of any previous entity and less than the version of any possible future entity."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A partition ID identifies a grouping of entities. The grouping is always by project and namespace, however the namespace ID may be empty. A partition ID contains several dimensions: project ID and namespace ID. Partition dimensions: - May be `\"\"`. - Must be valid UTF-8 bytes. - Must have values that match regex `[A-Za-z\\d\\.\\-_]{1,100}` If the value of any dimension matches regex `__.*__`, the partition is reserved/read-only. A reserved/read-only partition ID is forbidden in certain documented contexts. Foreign partition IDs (in which the project ID does not match the context project ID ) are discouraged. Reads and writes of foreign partition IDs may fail if the project is not in an active state."]
pub struct PartitionId {
    #[serde(rename = "namespaceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If not empty, the ID of the namespace to which the entities belong."]
    pub namespace_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the project to which the entities belong."]
    pub project_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A (kind, ID/name) pair used to construct a key path. If either name or ID is set, the element is complete. If neither is set, the element is incomplete."]
pub struct PathElement {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The auto-allocated ID of the entity. Never equal to zero. Values less than zero are discouraged and may not be supported in the future."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kind of the entity. A kind matching regex `__.*__` is reserved/read-only. A kind must not contain more than 1500 bytes when UTF-8 encoded. Cannot be `\"\"`."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the entity. A name matching regex `__.*__` is reserved/read-only. A name must not be more than 1500 bytes when UTF-8 encoded. Cannot be `\"\"`."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A representation of a property in a projection."]
pub struct Projection {
    #[serde(rename = "property")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The property to project."]
    pub property: ::std::option::Option<::std::boxed::Box<PropertyReference>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A filter on a specific property."]
pub struct PropertyFilter {
    #[serde(rename = "op")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The operator to filter by."]
    pub op: ::std::option::Option<PropertyFilterOpEnum>,
    #[serde(rename = "property")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The property to filter by."]
    pub property: ::std::option::Option<::std::boxed::Box<PropertyReference>>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value to compare the property to."]
    pub value: ::std::option::Option<::std::boxed::Box<Value>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The operator to filter by."]
pub enum PropertyFilterOpEnum {
    #[serde(rename = "OPERATOR_UNSPECIFIED")]
    #[doc = "Unspecified. This value must not be used."]
    OperatorUnspecified,
    #[serde(rename = "LESS_THAN")]
    #[doc = "Less than."]
    LessThan,
    #[serde(rename = "LESS_THAN_OR_EQUAL")]
    #[doc = "Less than or equal."]
    LessThanOrEqual,
    #[serde(rename = "GREATER_THAN")]
    #[doc = "Greater than."]
    GreaterThan,
    #[serde(rename = "GREATER_THAN_OR_EQUAL")]
    #[doc = "Greater than or equal."]
    GreaterThanOrEqual,
    #[serde(rename = "EQUAL")]
    #[doc = "Equal."]
    Equal,
    #[serde(rename = "HAS_ANCESTOR")]
    #[doc = "Has ancestor."]
    HasAncestor,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The desired order for a specific property."]
pub struct PropertyOrder {
    #[serde(rename = "direction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The direction to order by. Defaults to `ASCENDING`."]
    pub direction: ::std::option::Option<PropertyOrderDirectionEnum>,
    #[serde(rename = "property")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The property to order by."]
    pub property: ::std::option::Option<::std::boxed::Box<PropertyReference>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The direction to order by. Defaults to `ASCENDING`."]
pub enum PropertyOrderDirectionEnum {
    #[serde(rename = "DIRECTION_UNSPECIFIED")]
    #[doc = "Unspecified. This value must not be used."]
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
#[doc = "A reference to a property relative to the kind expressions."]
pub struct PropertyReference {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the property. If name includes \".\"s, it may be interpreted as a property name path."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A query for entities."]
pub struct Query {
    #[serde(rename = "distinctOn")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The properties to make distinct. The query results will contain the first result for each distinct combination of values for the given properties (if empty, all results are returned)."]
    pub distinct_on: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PropertyReference>>>,
    #[serde(rename = "endCursor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An ending point for the query results. Query cursors are returned in query result batches and [can only be used to limit the same query](https://cloud.google.com/datastore/docs/concepts/queries#cursors_limits_and_offsets)."]
    pub end_cursor: ::std::option::Option<::std::string::String>,
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The filter to apply."]
    pub filter: ::std::option::Option<::std::boxed::Box<Filter>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The kinds to query (if empty, returns entities of all kinds). Currently at most 1 kind may be specified."]
    pub kind: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<KindExpression>>>,
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum number of results to return. Applies after all other constraints. Optional. Unspecified is interpreted as no limit. Must be >= 0 if specified."]
    pub limit: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of results to skip. Applies before limit, but after all other constraints. Optional. Must be >= 0 if specified."]
    pub offset: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "order")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The order to apply to the query results (if empty, order is unspecified)."]
    pub order: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PropertyOrder>>>,
    #[serde(rename = "projection")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The projection to return. Defaults to returning all properties."]
    pub projection: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Projection>>>,
    #[serde(rename = "startCursor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A starting point for the query results. Query cursors are returned in query result batches and [can only be used to continue the same query](https://cloud.google.com/datastore/docs/concepts/queries#cursors_limits_and_offsets)."]
    pub start_cursor: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A batch of results produced by a query."]
pub struct QueryResultBatch {
    #[serde(rename = "endCursor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A cursor that points to the position after the last result in the batch."]
    pub end_cursor: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityResultType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result type for every entity in `entity_results`."]
    pub entity_result_type: ::std::option::Option<QueryResultBatchEntityResultTypeEnum>,
    #[serde(rename = "entityResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The results for this batch."]
    pub entity_results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EntityResult>>>,
    #[serde(rename = "moreResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state of the query after the current batch."]
    pub more_results: ::std::option::Option<QueryResultBatchMoreResultsEnum>,
    #[serde(rename = "skippedCursor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A cursor that points to the position after the last skipped result. Will be set when `skipped_results` != 0."]
    pub skipped_cursor: ::std::option::Option<::std::string::String>,
    #[serde(rename = "skippedResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of results skipped, typically because of an offset."]
    pub skipped_results: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "snapshotVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version number of the snapshot this batch was returned from. This applies to the range of results from the query's `start_cursor` (or the beginning of the query if no cursor was given) to this batch's `end_cursor` (not the query's `end_cursor`). In a single transaction, subsequent query result batches for the same query can have a greater snapshot version number. Each batch's snapshot version is valid for all preceding batches. The value will be zero for eventually consistent queries."]
    pub snapshot_version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The result type for every entity in `entity_results`."]
pub enum QueryResultBatchEntityResultTypeEnum {
    #[serde(rename = "RESULT_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified. This value is never used."]
    ResultTypeUnspecified,
    #[serde(rename = "FULL")]
    #[doc = "The key and properties."]
    Full,
    #[serde(rename = "PROJECTION")]
    #[doc = "A projected subset of properties. The entity may have no key."]
    Projection,
    #[serde(rename = "KEY_ONLY")]
    #[doc = "Only the key."]
    KeyOnly,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The state of the query after the current batch."]
pub enum QueryResultBatchMoreResultsEnum {
    #[serde(rename = "MORE_RESULTS_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified. This value is never used."]
    MoreResultsTypeUnspecified,
    #[serde(rename = "NOT_FINISHED")]
    #[doc = "There may be additional batches to fetch from this query."]
    NotFinished,
    #[serde(rename = "MORE_RESULTS_AFTER_LIMIT")]
    #[doc = "The query is finished, but there may be more results after the limit."]
    MoreResultsAfterLimit,
    #[serde(rename = "MORE_RESULTS_AFTER_CURSOR")]
    #[doc = "The query is finished, but there may be more results after the end cursor."]
    MoreResultsAfterCursor,
    #[serde(rename = "NO_MORE_RESULTS")]
    #[doc = "The query is finished, and there are no more results."]
    NoMoreResults,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Options specific to read-only transactions."]
pub struct ReadOnly {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The options shared by read requests."]
pub struct ReadOptions {
    #[serde(rename = "readConsistency")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The non-transactional read consistency to use. Cannot be set to `STRONG` for global queries."]
    pub read_consistency: ::std::option::Option<ReadOptionsReadConsistencyEnum>,
    #[serde(rename = "transaction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier of the transaction in which to read. A transaction identifier is returned by a call to Datastore.BeginTransaction."]
    pub transaction: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The non-transactional read consistency to use. Cannot be set to `STRONG` for global queries."]
pub enum ReadOptionsReadConsistencyEnum {
    #[serde(rename = "READ_CONSISTENCY_UNSPECIFIED")]
    #[doc = "Unspecified. This value must not be used."]
    ReadConsistencyUnspecified,
    #[serde(rename = "STRONG")]
    #[doc = "Strong consistency."]
    Strong,
    #[serde(rename = "EVENTUAL")]
    #[doc = "Eventual consistency."]
    Eventual,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Options specific to read / write transactions."]
pub struct ReadWrite {
    #[serde(rename = "previousTransaction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The transaction identifier of the transaction being retried."]
    pub previous_transaction: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for Datastore.ReserveIds."]
pub struct ReserveIdsRequest {
    #[serde(rename = "databaseId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If not empty, the ID of the database against which to make the request."]
    pub database_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "keys")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A list of keys with complete key paths whose numeric IDs should not be auto-allocated."]
    pub keys: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Key>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for Datastore.ReserveIds."]
pub struct ReserveIdsResponse {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for Datastore.Rollback."]
pub struct RollbackRequest {
    #[serde(rename = "transaction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The transaction identifier, returned by a call to Datastore.BeginTransaction."]
    pub transaction: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for Datastore.Rollback. (an empty message)."]
pub struct RollbackResponse {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request for Datastore.RunQuery."]
pub struct RunQueryRequest {
    #[serde(rename = "gqlQuery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The GQL query to run."]
    pub gql_query: ::std::option::Option<::std::boxed::Box<GqlQuery>>,
    #[serde(rename = "partitionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Entities are partitioned into subsets, identified by a partition ID. Queries are scoped to a single partition. This partition ID is normalized with the standard default context partition ID."]
    pub partition_id: ::std::option::Option<::std::boxed::Box<PartitionId>>,
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The query to run."]
    pub query: ::std::option::Option<::std::boxed::Box<Query>>,
    #[serde(rename = "readOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The options for this query."]
    pub read_options: ::std::option::Option<::std::boxed::Box<ReadOptions>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for Datastore.RunQuery."]
pub struct RunQueryResponse {
    #[serde(rename = "batch")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A batch of query results (always present)."]
    pub batch: ::std::option::Option<::std::boxed::Box<QueryResultBatch>>,
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The parsed form of the `GqlQuery` from the request, if it was set."]
    pub query: ::std::option::Option<::std::boxed::Box<Query>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Options for beginning a new transaction. Transactions can be created explicitly with calls to Datastore.BeginTransaction or implicitly by setting ReadOptions.new_transaction in read requests."]
pub struct TransactionOptions {
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The transaction should only allow reads."]
    pub read_only: ::std::option::Option<::std::boxed::Box<ReadOnly>>,
    #[serde(rename = "readWrite")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The transaction should allow both reads and writes."]
    pub read_write: ::std::option::Option<::std::boxed::Box<ReadWrite>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A message that can hold any of the supported value types and associated metadata."]
pub struct Value {
    #[serde(rename = "arrayValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An array value. Cannot contain another array value. A `Value` instance that sets field `array_value` must not set fields `meaning` or `exclude_from_indexes`."]
    pub array_value: ::std::option::Option<::std::boxed::Box<ArrayValue>>,
    #[serde(rename = "blobValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A blob value. May have at most 1,000,000 bytes. When `exclude_from_indexes` is false, may have at most 1500 bytes. In JSON requests, must be base64-encoded."]
    pub blob_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "booleanValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A boolean value."]
    pub boolean_value: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "doubleValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A double value."]
    pub double_value: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "entityValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An entity value. - May have no key. - May have a key with an incomplete key path. - May have a reserved/read-only key."]
    pub entity_value: ::std::option::Option<::std::boxed::Box<Entity>>,
    #[serde(rename = "excludeFromIndexes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the value should be excluded from all indexes including those defined explicitly."]
    pub exclude_from_indexes: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "geoPointValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A geo point value representing a point on the surface of Earth."]
    pub geo_point_value: ::std::option::Option<::std::boxed::Box<LatLng>>,
    #[serde(rename = "integerValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An integer value."]
    pub integer_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "keyValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A key value."]
    pub key_value: ::std::option::Option<::std::boxed::Box<Key>>,
    #[serde(rename = "meaning")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `meaning` field should only be populated for backwards compatibility."]
    pub meaning: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "nullValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A null value."]
    pub null_value: ::std::option::Option<ValueNullValueEnum>,
    #[serde(rename = "stringValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A UTF-8 encoded string value. When `exclude_from_indexes` is false (it is indexed) , may have at most 1500 bytes. Otherwise, may be set to at most 1,000,000 bytes."]
    pub string_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timestampValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A timestamp value. When stored in the Datastore, precise only to microseconds; any additional precision is rounded down."]
    pub timestamp_value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "A null value."]
pub enum ValueNullValueEnum {
    #[serde(rename = "NULL_VALUE")]
    #[doc = "Null value."]
    NullValue,
}
