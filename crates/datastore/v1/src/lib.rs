#[derive(
    Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
)]
pub struct QueryParameters {
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "$.xgafv")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "V1 error format."]
    pub xgafv: ::std::option::Option<QueryParametersXgafvEnum>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "access_token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OAuth access token."]
    pub access_token: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ query_parameters_defaults :: alt () }", setter(into))]
    #[serde(rename = "alt")]
    #[serde(default = "query_parameters_defaults :: alt")]
    #[doc = "Data format for response."]
    pub alt: QueryParametersAltEnum,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "callback")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "JSONP"]
    pub callback: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Selector specifying which fields to include in a partial response."]
    pub fields: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
    pub key: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "oauth_token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OAuth 2.0 token for the current user."]
    pub oauth_token: ::std::option::Option<::std::string::String>,
    #[builder(
        default = "{ query_parameters_defaults :: pretty_print () }",
        setter(into)
    )]
    #[serde(rename = "prettyPrint")]
    #[serde(default = "query_parameters_defaults :: pretty_print")]
    #[doc = "Returns response with indentations and line breaks."]
    pub pretty_print: ::std::primitive::bool,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "quotaUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
    pub quota_user: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "uploadType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
    pub upload_type: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "upload_protocol")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
    pub upload_protocol: ::std::option::Option<::std::string::String>,
}
impl QueryParameters {
    pub fn builder() -> QueryParametersBuilder {
        QueryParametersBuilder::default()
    }
}
mod query_parameters_defaults {
    pub fn alt() -> super::QueryParametersAltEnum {
        serde_json::from_str(&"json").unwrap()
    }
    pub fn pretty_print() -> ::std::primitive::bool {
        serde_json::from_str(&"true").unwrap()
    }
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "V1 error format."]
pub enum QueryParametersXgafvEnum {
    #[serde(rename = "1")]
    #[doc = "v1 error format"]
    _1,
    #[serde(rename = "2")]
    #[doc = "v2 error format"]
    _2,
}
impl ::std::default::Default for QueryParametersXgafvEnum {
    fn default() -> Self {
        Self::_1
    }
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Data format for response."]
pub enum QueryParametersAltEnum {
    #[serde(rename = "json")]
    #[doc = "Responses with Content-Type of application/json"]
    Json,
    #[serde(rename = "media")]
    #[doc = "Media download with context-dependent Content-Type"]
    Media,
    #[serde(rename = "proto")]
    #[doc = "Responses with Content-Type of application/x-protobuf"]
    Proto,
}
impl ::std::default::Default for QueryParametersAltEnum {
    fn default() -> Self {
        Self::Json
    }
}
pub mod resources {
    pub mod projects {
        pub mod resources {
            pub mod indexes {
                pub mod methods {
                    pub mod list {
                        #[derive(
                            Clone,
                            Debug,
                            PartialEq,
                            derive_builder :: Builder,
                            serde :: Serialize,
                            serde :: Deserialize,
                        )]
                        pub struct QueryParameters {
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "filter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of items to return. If zero, then all results will be returned."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The next_page_token value returned from a previous List request, if any."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod operations {
                pub mod methods {
                    pub mod list {
                        #[derive(
                            Clone,
                            Debug,
                            PartialEq,
                            derive_builder :: Builder,
                            serde :: Serialize,
                            serde :: Deserialize,
                        )]
                        pub struct QueryParameters {
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "filter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The standard list filter."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The standard list page size."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The standard list page token."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
        }
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request for Datastore.AllocateIds."]
    pub struct AllocateIdsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keys")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A list of keys with incomplete key paths for which to allocate IDs. No key may be reserved/read-only."]
        pub keys: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Key>>>,
    }
    impl AllocateIdsRequest {
        pub fn builder() -> AllocateIdsRequestBuilder {
            AllocateIdsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for Datastore.AllocateIds."]
    pub struct AllocateIdsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keys")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The keys specified in the request (in the same order), each with its key path completed with a newly allocated ID."]
        pub keys: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Key>>>,
    }
    impl AllocateIdsResponse {
        pub fn builder() -> AllocateIdsResponseBuilder {
            AllocateIdsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An array value."]
    pub struct ArrayValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "values")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Values in the array. The order of values in an array is preserved as long as all values have identical settings for 'exclude_from_indexes'."]
        pub values: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Value>>>,
    }
    impl ArrayValue {
        pub fn builder() -> ArrayValueBuilder {
            ArrayValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request for Datastore.BeginTransaction."]
    pub struct BeginTransactionRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transactionOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Options for a new transaction."]
        pub transaction_options: ::std::option::Option<::std::boxed::Box<TransactionOptions>>,
    }
    impl BeginTransactionRequest {
        pub fn builder() -> BeginTransactionRequestBuilder {
            BeginTransactionRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for Datastore.BeginTransaction."]
    pub struct BeginTransactionResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transaction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The transaction identifier (always present)."]
        pub transaction: ::std::option::Option<::std::string::String>,
    }
    impl BeginTransactionResponse {
        pub fn builder() -> BeginTransactionResponseBuilder {
            BeginTransactionResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request for Datastore.Commit."]
    pub struct CommitRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of commit to perform. Defaults to `TRANSACTIONAL`."]
        pub mode: ::std::option::Option<CommitRequestModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mutations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The mutations to perform. When mode is `TRANSACTIONAL`, mutations affecting a single entity are applied in order. The following sequences of mutations affecting a single entity are not permitted in a single `Commit` request: - `insert` followed by `insert` - `update` followed by `insert` - `upsert` followed by `insert` - `delete` followed by `update` When mode is `NON_TRANSACTIONAL`, no two mutations may affect a single entity."]
        pub mutations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Mutation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transaction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier of the transaction associated with the commit. A transaction identifier is returned by a call to Datastore.BeginTransaction."]
        pub transaction: ::std::option::Option<::std::string::String>,
    }
    impl CommitRequest {
        pub fn builder() -> CommitRequestBuilder {
            CommitRequestBuilder::default()
        }
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
    impl ::std::default::Default for CommitRequestModeEnum {
        fn default() -> Self {
            Self::ModeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for Datastore.Commit."]
    pub struct CommitResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "indexUpdates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of index entries updated during the commit, or zero if none were updated."]
        pub index_updates: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mutationResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of performing the mutations. The i-th mutation result corresponds to the i-th mutation in the request."]
        pub mutation_results:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MutationResult>>>,
    }
    impl CommitResponse {
        pub fn builder() -> CommitResponseBuilder {
            CommitResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A filter that merges multiple other filters using the given operator."]
    pub struct CompositeFilter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of filters to combine. Must contain at least one filter."]
        pub filters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Filter>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "op")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The operator for combining multiple filters."]
        pub op: ::std::option::Option<CompositeFilterOpEnum>,
    }
    impl CompositeFilter {
        pub fn builder() -> CompositeFilterBuilder {
            CompositeFilterBuilder::default()
        }
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
    impl ::std::default::Default for CompositeFilterOpEnum {
        fn default() -> Self {
            Self::OperatorUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
    pub struct Empty {}
    impl Empty {
        pub fn builder() -> EmptyBuilder {
            EmptyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Datastore data object. An entity is limited to 1 megabyte when stored. That _roughly_ corresponds to a limit of 1 megabyte for the serialized form of this message."]
    pub struct Entity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entity's key. An entity must have a key, unless otherwise documented (for example, an entity in `Value.entity_value` may have no key). An entity's kind is its key path's last element's kind, or null if it has no key."]
        pub key: ::std::option::Option<::std::boxed::Box<Key>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entity's properties. The map's keys are property names. A property name matching regex `__.*__` is reserved. A reserved property name is forbidden in certain documented contexts. The name must not contain more than 500 characters. The name cannot be `\"\"`."]
        pub properties:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<Value>>>,
    }
    impl Entity {
        pub fn builder() -> EntityBuilder {
            EntityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of fetching an entity from Datastore."]
    pub struct EntityResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cursor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A cursor that points to the position after the result entity. Set only when the `EntityResult` is part of a `QueryResultBatch` message."]
        pub cursor: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resulting entity."]
        pub entity: ::std::option::Option<::std::boxed::Box<Entity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version of the entity, a strictly positive number that monotonically increases with changes to the entity. This field is set for `FULL` entity results. For missing entities in `LookupResponse`, this is the version of the snapshot that was used to look up the entity, and it is always set except for eventually consistent reads."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl EntityResult {
        pub fn builder() -> EntityResultBuilder {
            EntityResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A holder for any type of filter."]
    pub struct Filter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "compositeFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A composite filter."]
        pub composite_filter: ::std::option::Option<::std::boxed::Box<CompositeFilter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "propertyFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A filter on a property."]
        pub property_filter: ::std::option::Option<::std::boxed::Box<PropertyFilter>>,
    }
    impl Filter {
        pub fn builder() -> FilterBuilder {
            FilterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata common to all Datastore Admin operations."]
    pub struct GoogleDatastoreAdminV1CommonMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time the operation ended, either successfully or otherwise."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The client-assigned labels which were provided when the operation was created. May also include additional labels."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the operation. Can be used as a filter in ListOperationsRequest."]
        pub operation_type:
            ::std::option::Option<GoogleDatastoreAdminV1CommonMetadataOperationTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time that work began on the operation."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current state of the Operation."]
        pub state: ::std::option::Option<GoogleDatastoreAdminV1CommonMetadataStateEnum>,
    }
    impl GoogleDatastoreAdminV1CommonMetadata {
        pub fn builder() -> GoogleDatastoreAdminV1CommonMetadataBuilder {
            GoogleDatastoreAdminV1CommonMetadataBuilder::default()
        }
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
    impl ::std::default::Default for GoogleDatastoreAdminV1CommonMetadataOperationTypeEnum {
        fn default() -> Self {
            Self::OperationTypeUnspecified
        }
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
    impl ::std::default::Default for GoogleDatastoreAdminV1CommonMetadataStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Identifies a subset of entities in a project. This is specified as combinations of kinds and namespaces (either or both of which may be all, as described in the following examples). Example usage: Entire project: kinds=[], namespace_ids=[] Kinds Foo and Bar in all namespaces: kinds=['Foo', 'Bar'], namespace_ids=[] Kinds Foo and Bar only in the default namespace: kinds=['Foo', 'Bar'], namespace_ids=[''] Kinds Foo and Bar in both the default and Baz namespaces: kinds=['Foo', 'Bar'], namespace_ids=['', 'Baz'] The entire Baz namespace: kinds=[], namespace_ids=['Baz']"]
    pub struct GoogleDatastoreAdminV1EntityFilter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kinds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If empty, then this represents all kinds."]
        pub kinds: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "namespaceIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An empty list represents all namespaces. This is the preferred usage for projects that don't use namespaces. An empty string element represents the default namespace. This should be used if the project has data in non-default namespaces, but doesn't want to include them. Each namespace in this list must be unique."]
        pub namespace_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleDatastoreAdminV1EntityFilter {
        pub fn builder() -> GoogleDatastoreAdminV1EntityFilterBuilder {
            GoogleDatastoreAdminV1EntityFilterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for ExportEntities operations."]
    pub struct GoogleDatastoreAdminV1ExportEntitiesMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "common")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata common to all Datastore Admin operations."]
        pub common: ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1CommonMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of which entities are being exported."]
        pub entity_filter:
            ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1EntityFilter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputUrlPrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location for the export metadata and data files. This will be the same value as the google.datastore.admin.v1.ExportEntitiesRequest.output_url_prefix field. The final output location is provided in google.datastore.admin.v1.ExportEntitiesResponse.output_url."]
        pub output_url_prefix: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An estimate of the number of bytes processed."]
        pub progress_bytes:
            ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1Progress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressEntities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An estimate of the number of entities processed."]
        pub progress_entities:
            ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1Progress>>,
    }
    impl GoogleDatastoreAdminV1ExportEntitiesMetadata {
        pub fn builder() -> GoogleDatastoreAdminV1ExportEntitiesMetadataBuilder {
            GoogleDatastoreAdminV1ExportEntitiesMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request for google.datastore.admin.v1.DatastoreAdmin.ExportEntities."]
    pub struct GoogleDatastoreAdminV1ExportEntitiesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of what data from the project is included in the export."]
        pub entity_filter:
            ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1EntityFilter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Client-assigned labels."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputUrlPrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Location for the export metadata and data files. The full resource URL of the external storage location. Currently, only Google Cloud Storage is supported. So output_url_prefix should be of the form: `gs://BUCKET_NAME[/NAMESPACE_PATH]`, where `BUCKET_NAME` is the name of the Cloud Storage bucket and `NAMESPACE_PATH` is an optional Cloud Storage namespace path (this is not a Cloud Datastore namespace). For more information about Cloud Storage namespace paths, see [Object name considerations](https://cloud.google.com/storage/docs/naming#object-considerations). The resulting files will be nested deeper than the specified URL prefix. The final output URL will be provided in the google.datastore.admin.v1.ExportEntitiesResponse.output_url field. That value should be used for subsequent ImportEntities operations. By nesting the data files deeper, the same Cloud Storage bucket can be used in multiple ExportEntities operations without conflict."]
        pub output_url_prefix: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDatastoreAdminV1ExportEntitiesRequest {
        pub fn builder() -> GoogleDatastoreAdminV1ExportEntitiesRequestBuilder {
            GoogleDatastoreAdminV1ExportEntitiesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for google.datastore.admin.v1.DatastoreAdmin.ExportEntities."]
    pub struct GoogleDatastoreAdminV1ExportEntitiesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location of the output metadata file. This can be used to begin an import into Cloud Datastore (this project or another project). See google.datastore.admin.v1.ImportEntitiesRequest.input_url. Only present if the operation completed successfully."]
        pub output_url: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDatastoreAdminV1ExportEntitiesResponse {
        pub fn builder() -> GoogleDatastoreAdminV1ExportEntitiesResponseBuilder {
            GoogleDatastoreAdminV1ExportEntitiesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for ImportEntities operations."]
    pub struct GoogleDatastoreAdminV1ImportEntitiesMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "common")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata common to all Datastore Admin operations."]
        pub common: ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1CommonMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of which entities are being imported."]
        pub entity_filter:
            ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1EntityFilter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location of the import metadata file. This will be the same value as the google.datastore.admin.v1.ExportEntitiesResponse.output_url field."]
        pub input_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An estimate of the number of bytes processed."]
        pub progress_bytes:
            ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1Progress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressEntities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An estimate of the number of entities processed."]
        pub progress_entities:
            ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1Progress>>,
    }
    impl GoogleDatastoreAdminV1ImportEntitiesMetadata {
        pub fn builder() -> GoogleDatastoreAdminV1ImportEntitiesMetadataBuilder {
            GoogleDatastoreAdminV1ImportEntitiesMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request for google.datastore.admin.v1.DatastoreAdmin.ImportEntities."]
    pub struct GoogleDatastoreAdminV1ImportEntitiesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optionally specify which kinds/namespaces are to be imported. If provided, the list must be a subset of the EntityFilter used in creating the export, otherwise a FAILED_PRECONDITION error will be returned. If no filter is specified then all entities from the export are imported."]
        pub entity_filter:
            ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1EntityFilter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The full resource URL of the external storage location. Currently, only Google Cloud Storage is supported. So input_url should be of the form: `gs://BUCKET_NAME[/NAMESPACE_PATH]/OVERALL_EXPORT_METADATA_FILE`, where `BUCKET_NAME` is the name of the Cloud Storage bucket, `NAMESPACE_PATH` is an optional Cloud Storage namespace path (this is not a Cloud Datastore namespace), and `OVERALL_EXPORT_METADATA_FILE` is the metadata file written by the ExportEntities operation. For more information about Cloud Storage namespace paths, see [Object name considerations](https://cloud.google.com/storage/docs/naming#object-considerations). For more information, see google.datastore.admin.v1.ExportEntitiesResponse.output_url."]
        pub input_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Client-assigned labels."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl GoogleDatastoreAdminV1ImportEntitiesRequest {
        pub fn builder() -> GoogleDatastoreAdminV1ImportEntitiesRequestBuilder {
            GoogleDatastoreAdminV1ImportEntitiesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Datastore composite index definition."]
    pub struct GoogleDatastoreAdminV1Index {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ancestor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The index's ancestor mode. Must not be ANCESTOR_MODE_UNSPECIFIED."]
        pub ancestor: ::std::option::Option<GoogleDatastoreAdminV1IndexAncestorEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "indexId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The resource ID of the index."]
        pub index_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The entity kind to which this index applies."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Project ID."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. An ordered sequence of property names and their index attributes."]
        pub properties: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleDatastoreAdminV1IndexedProperty>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The state of the index."]
        pub state: ::std::option::Option<GoogleDatastoreAdminV1IndexStateEnum>,
    }
    impl GoogleDatastoreAdminV1Index {
        pub fn builder() -> GoogleDatastoreAdminV1IndexBuilder {
            GoogleDatastoreAdminV1IndexBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The index's ancestor mode. Must not be ANCESTOR_MODE_UNSPECIFIED."]
    pub enum GoogleDatastoreAdminV1IndexAncestorEnum {
        #[serde(rename = "ANCESTOR_MODE_UNSPECIFIED")]
        #[doc = "The ancestor mode is unspecified."]
        AncestorModeUnspecified,
        #[serde(rename = "NONE")]
        #[doc = "Do not include the entity's ancestors in the index."]
        None,
        #[serde(rename = "ALL_ANCESTORS")]
        #[doc = "Include all the entity's ancestors in the index."]
        AllAncestors,
    }
    impl ::std::default::Default for GoogleDatastoreAdminV1IndexAncestorEnum {
        fn default() -> Self {
            Self::AncestorModeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The state of the index."]
    pub enum GoogleDatastoreAdminV1IndexStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "The state is unspecified."]
        StateUnspecified,
        #[serde(rename = "CREATING")]
        #[doc = "The index is being created, and cannot be used by queries. There is an active long-running operation for the index. The index is updated when writing an entity. Some index data may exist."]
        Creating,
        #[serde(rename = "READY")]
        #[doc = "The index is ready to be used. The index is updated when writing an entity. The index is fully populated from all stored entities it applies to."]
        Ready,
        #[serde(rename = "DELETING")]
        #[doc = "The index is being deleted, and cannot be used by queries. There is an active long-running operation for the index. The index is not updated when writing an entity. Some index data may exist."]
        Deleting,
        #[serde(rename = "ERROR")]
        #[doc = "The index was being created or deleted, but something went wrong. The index cannot by used by queries. There is no active long-running operation for the index, and the most recently finished long-running operation failed. The index is not updated when writing an entity. Some index data may exist."]
        Error,
    }
    impl ::std::default::Default for GoogleDatastoreAdminV1IndexStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for Index operations."]
    pub struct GoogleDatastoreAdminV1IndexOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "common")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata common to all Datastore Admin operations."]
        pub common: ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1CommonMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "indexId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index resource ID that this operation is acting on."]
        pub index_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressEntities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An estimate of the number of entities processed."]
        pub progress_entities:
            ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1Progress>>,
    }
    impl GoogleDatastoreAdminV1IndexOperationMetadata {
        pub fn builder() -> GoogleDatastoreAdminV1IndexOperationMetadataBuilder {
            GoogleDatastoreAdminV1IndexOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A property of an index."]
    pub struct GoogleDatastoreAdminV1IndexedProperty {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "direction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The indexed property's direction. Must not be DIRECTION_UNSPECIFIED."]
        pub direction: ::std::option::Option<GoogleDatastoreAdminV1IndexedPropertyDirectionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The property name to index."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDatastoreAdminV1IndexedProperty {
        pub fn builder() -> GoogleDatastoreAdminV1IndexedPropertyBuilder {
            GoogleDatastoreAdminV1IndexedPropertyBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The indexed property's direction. Must not be DIRECTION_UNSPECIFIED."]
    pub enum GoogleDatastoreAdminV1IndexedPropertyDirectionEnum {
        #[serde(rename = "DIRECTION_UNSPECIFIED")]
        #[doc = "The direction is unspecified."]
        DirectionUnspecified,
        #[serde(rename = "ASCENDING")]
        #[doc = "The property's values are indexed so as to support sequencing in ascending order and also query by <, >, <=, >=, and =."]
        Ascending,
        #[serde(rename = "DESCENDING")]
        #[doc = "The property's values are indexed so as to support sequencing in descending order and also query by <, >, <=, >=, and =."]
        Descending,
    }
    impl ::std::default::Default for GoogleDatastoreAdminV1IndexedPropertyDirectionEnum {
        fn default() -> Self {
            Self::DirectionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for google.datastore.admin.v1.DatastoreAdmin.ListIndexes."]
    pub struct GoogleDatastoreAdminV1ListIndexesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "indexes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The indexes."]
        pub indexes:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleDatastoreAdminV1Index>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The standard List next-page token."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDatastoreAdminV1ListIndexesResponse {
        pub fn builder() -> GoogleDatastoreAdminV1ListIndexesResponseBuilder {
            GoogleDatastoreAdminV1ListIndexesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Measures the progress of a particular metric."]
    pub struct GoogleDatastoreAdminV1Progress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workCompleted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of work that has been completed. Note that this may be greater than work_estimated."]
        pub work_completed: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workEstimated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An estimate of how much work needs to be performed. May be zero if the work estimate is unavailable."]
        pub work_estimated: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDatastoreAdminV1Progress {
        pub fn builder() -> GoogleDatastoreAdminV1ProgressBuilder {
            GoogleDatastoreAdminV1ProgressBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata common to all Datastore Admin operations."]
    pub struct GoogleDatastoreAdminV1beta1CommonMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time the operation ended, either successfully or otherwise."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The client-assigned labels which were provided when the operation was created. May also include additional labels."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the operation. Can be used as a filter in ListOperationsRequest."]
        pub operation_type:
            ::std::option::Option<GoogleDatastoreAdminV1beta1CommonMetadataOperationTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time that work began on the operation."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current state of the Operation."]
        pub state: ::std::option::Option<GoogleDatastoreAdminV1beta1CommonMetadataStateEnum>,
    }
    impl GoogleDatastoreAdminV1beta1CommonMetadata {
        pub fn builder() -> GoogleDatastoreAdminV1beta1CommonMetadataBuilder {
            GoogleDatastoreAdminV1beta1CommonMetadataBuilder::default()
        }
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
    impl ::std::default::Default for GoogleDatastoreAdminV1beta1CommonMetadataOperationTypeEnum {
        fn default() -> Self {
            Self::OperationTypeUnspecified
        }
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
    impl ::std::default::Default for GoogleDatastoreAdminV1beta1CommonMetadataStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Identifies a subset of entities in a project. This is specified as combinations of kinds and namespaces (either or both of which may be all, as described in the following examples). Example usage: Entire project: kinds=[], namespace_ids=[] Kinds Foo and Bar in all namespaces: kinds=['Foo', 'Bar'], namespace_ids=[] Kinds Foo and Bar only in the default namespace: kinds=['Foo', 'Bar'], namespace_ids=[''] Kinds Foo and Bar in both the default and Baz namespaces: kinds=['Foo', 'Bar'], namespace_ids=['', 'Baz'] The entire Baz namespace: kinds=[], namespace_ids=['Baz']"]
    pub struct GoogleDatastoreAdminV1beta1EntityFilter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kinds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If empty, then this represents all kinds."]
        pub kinds: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "namespaceIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An empty list represents all namespaces. This is the preferred usage for projects that don't use namespaces. An empty string element represents the default namespace. This should be used if the project has data in non-default namespaces, but doesn't want to include them. Each namespace in this list must be unique."]
        pub namespace_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleDatastoreAdminV1beta1EntityFilter {
        pub fn builder() -> GoogleDatastoreAdminV1beta1EntityFilterBuilder {
            GoogleDatastoreAdminV1beta1EntityFilterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for ExportEntities operations."]
    pub struct GoogleDatastoreAdminV1beta1ExportEntitiesMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "common")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata common to all Datastore Admin operations."]
        pub common:
            ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1beta1CommonMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of which entities are being exported."]
        pub entity_filter:
            ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1beta1EntityFilter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputUrlPrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location for the export metadata and data files. This will be the same value as the google.datastore.admin.v1beta1.ExportEntitiesRequest.output_url_prefix field. The final output location is provided in google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url."]
        pub output_url_prefix: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An estimate of the number of bytes processed."]
        pub progress_bytes:
            ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1beta1Progress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressEntities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An estimate of the number of entities processed."]
        pub progress_entities:
            ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1beta1Progress>>,
    }
    impl GoogleDatastoreAdminV1beta1ExportEntitiesMetadata {
        pub fn builder() -> GoogleDatastoreAdminV1beta1ExportEntitiesMetadataBuilder {
            GoogleDatastoreAdminV1beta1ExportEntitiesMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for google.datastore.admin.v1beta1.DatastoreAdmin.ExportEntities."]
    pub struct GoogleDatastoreAdminV1beta1ExportEntitiesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location of the output metadata file. This can be used to begin an import into Cloud Datastore (this project or another project). See google.datastore.admin.v1beta1.ImportEntitiesRequest.input_url. Only present if the operation completed successfully."]
        pub output_url: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDatastoreAdminV1beta1ExportEntitiesResponse {
        pub fn builder() -> GoogleDatastoreAdminV1beta1ExportEntitiesResponseBuilder {
            GoogleDatastoreAdminV1beta1ExportEntitiesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for ImportEntities operations."]
    pub struct GoogleDatastoreAdminV1beta1ImportEntitiesMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "common")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata common to all Datastore Admin operations."]
        pub common:
            ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1beta1CommonMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of which entities are being imported."]
        pub entity_filter:
            ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1beta1EntityFilter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location of the import metadata file. This will be the same value as the google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url field."]
        pub input_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An estimate of the number of bytes processed."]
        pub progress_bytes:
            ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1beta1Progress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressEntities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An estimate of the number of entities processed."]
        pub progress_entities:
            ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1beta1Progress>>,
    }
    impl GoogleDatastoreAdminV1beta1ImportEntitiesMetadata {
        pub fn builder() -> GoogleDatastoreAdminV1beta1ImportEntitiesMetadataBuilder {
            GoogleDatastoreAdminV1beta1ImportEntitiesMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Measures the progress of a particular metric."]
    pub struct GoogleDatastoreAdminV1beta1Progress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workCompleted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of work that has been completed. Note that this may be greater than work_estimated."]
        pub work_completed: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workEstimated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An estimate of how much work needs to be performed. May be zero if the work estimate is unavailable."]
        pub work_estimated: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDatastoreAdminV1beta1Progress {
        pub fn builder() -> GoogleDatastoreAdminV1beta1ProgressBuilder {
            GoogleDatastoreAdminV1beta1ProgressBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Operations.ListOperations."]
    pub struct GoogleLongrunningListOperationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The standard List next-page token."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of operations that matches the specified filter in the request."]
        pub operations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleLongrunningOperation>>>,
    }
    impl GoogleLongrunningListOperationsResponse {
        pub fn builder() -> GoogleLongrunningListOperationsResponseBuilder {
            GoogleLongrunningListOperationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This resource represents a long-running operation that is the result of a network API call."]
    pub struct GoogleLongrunningOperation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "done")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
        pub done: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error result of the operation in case of failure or cancellation."]
        pub error: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any."]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "response")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`."]
        pub response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl GoogleLongrunningOperation {
        pub fn builder() -> GoogleLongrunningOperationBuilder {
            GoogleLongrunningOperationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A [GQL query](https://cloud.google.com/datastore/docs/apis/gql/gql_reference)."]
    pub struct GqlQuery {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowLiterals")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When false, the query string must not contain any literals and instead must bind all values. For example, `SELECT * FROM Kind WHERE a = 'string literal'` is not allowed, while `SELECT * FROM Kind WHERE a = @value` is."]
        pub allow_literals: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "namedBindings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For each non-reserved named binding site in the query string, there must be a named parameter with that name, but not necessarily the inverse. Key must match regex `A-Za-z_$*`, must not match regex `__.*__`, and must not be `\"\"`."]
        pub named_bindings: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<GqlQueryParameter>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "positionalBindings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Numbered binding site @1 references the first numbered parameter, effectively using 1-based indexing, rather than the usual 0. For each binding site numbered i in `query_string`, there must be an i-th numbered parameter. The inverse must also be true."]
        pub positional_bindings:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GqlQueryParameter>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queryString")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A string of the format described [here](https://cloud.google.com/datastore/docs/apis/gql/gql_reference)."]
        pub query_string: ::std::option::Option<::std::string::String>,
    }
    impl GqlQuery {
        pub fn builder() -> GqlQueryBuilder {
            GqlQueryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A binding parameter for a GQL query."]
    pub struct GqlQueryParameter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cursor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A query cursor. Query cursors are returned in query result batches."]
        pub cursor: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A value parameter."]
        pub value: ::std::option::Option<::std::boxed::Box<Value>>,
    }
    impl GqlQueryParameter {
        pub fn builder() -> GqlQueryParameterBuilder {
            GqlQueryParameterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A unique identifier for an entity. If a key's partition ID or any of its path kinds or names are reserved/read-only, the key is reserved/read-only. A reserved/read-only key is forbidden in certain documented contexts."]
    pub struct Key {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partitionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entities are partitioned into subsets, currently identified by a project ID and namespace ID. Queries are scoped to a single partition."]
        pub partition_id: ::std::option::Option<::std::boxed::Box<PartitionId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entity path. An entity path consists of one or more elements composed of a kind and a string or numerical identifier, which identify entities. The first element identifies a _root entity_, the second element identifies a _child_ of the root entity, the third element identifies a child of the second entity, and so forth. The entities identified by all prefixes of the path are called the element's _ancestors_. An entity path is always fully complete: *all* of the entity's ancestors are required to be in the path along with the entity identifier itself. The only exception is that in some documented cases, the identifier in the last path element (for the entity) itself may be omitted. For example, the last path element of the key of `Mutation.insert` may have no identifier. A path can never be empty, and a path can have at most 100 elements."]
        pub path: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PathElement>>>,
    }
    impl Key {
        pub fn builder() -> KeyBuilder {
            KeyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A representation of a kind."]
    pub struct KindExpression {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the kind."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl KindExpression {
        pub fn builder() -> KindExpressionBuilder {
            KindExpressionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An object representing a latitude/longitude pair. This is expressed as a pair of doubles representing degrees latitude and degrees longitude. Unless specified otherwise, this must conform to the WGS84 standard. Values must be within normalized ranges."]
    pub struct LatLng {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "latitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The latitude in degrees. It must be in the range [-90.0, +90.0]."]
        pub latitude: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "longitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The longitude in degrees. It must be in the range [-180.0, +180.0]."]
        pub longitude: ::std::option::Option<::std::primitive::f64>,
    }
    impl LatLng {
        pub fn builder() -> LatLngBuilder {
            LatLngBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request for Datastore.Lookup."]
    pub struct LookupRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keys")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Keys of entities to look up."]
        pub keys: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Key>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "readOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The options for this lookup request."]
        pub read_options: ::std::option::Option<::std::boxed::Box<ReadOptions>>,
    }
    impl LookupRequest {
        pub fn builder() -> LookupRequestBuilder {
            LookupRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for Datastore.Lookup."]
    pub struct LookupResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deferred")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of keys that were not looked up due to resource constraints. The order of results in this field is undefined and has no relation to the order of the keys in the input."]
        pub deferred: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Key>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "found")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entities found as `ResultType.FULL` entities. The order of results in this field is undefined and has no relation to the order of the keys in the input."]
        pub found: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EntityResult>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "missing")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entities not found as `ResultType.KEY_ONLY` entities. The order of results in this field is undefined and has no relation to the order of the keys in the input."]
        pub missing: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EntityResult>>>,
    }
    impl LookupResponse {
        pub fn builder() -> LookupResponseBuilder {
            LookupResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A mutation to apply to an entity."]
    pub struct Mutation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "baseVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version of the entity that this mutation is being applied to. If this does not match the current version on the server, the mutation conflicts."]
        pub base_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "delete")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The key of the entity to delete. The entity may or may not already exist. Must have a complete key path and must not be reserved/read-only."]
        pub delete: ::std::option::Option<::std::boxed::Box<Key>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insert")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entity to insert. The entity must not already exist. The entity key's final path element may be incomplete."]
        pub insert: ::std::option::Option<::std::boxed::Box<Entity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "update")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entity to update. The entity must already exist. Must have a complete key path."]
        pub update: ::std::option::Option<::std::boxed::Box<Entity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "upsert")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entity to upsert. The entity may or may not already exist. The entity key's final path element may be incomplete."]
        pub upsert: ::std::option::Option<::std::boxed::Box<Entity>>,
    }
    impl Mutation {
        pub fn builder() -> MutationBuilder {
            MutationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of applying a mutation."]
    pub struct MutationResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conflictDetected")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether a conflict was detected for this mutation. Always false when a conflict detection strategy field is not set in the mutation."]
        pub conflict_detected: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The automatically allocated key. Set only when the mutation allocated a key."]
        pub key: ::std::option::Option<::std::boxed::Box<Key>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version of the entity on the server after processing the mutation. If the mutation doesn't change anything on the server, then the version will be the version of the current entity or, if no entity is present, a version that is strictly greater than the version of any previous entity and less than the version of any possible future entity."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl MutationResult {
        pub fn builder() -> MutationResultBuilder {
            MutationResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A partition ID identifies a grouping of entities. The grouping is always by project and namespace, however the namespace ID may be empty. A partition ID contains several dimensions: project ID and namespace ID. Partition dimensions: - May be `\"\"`. - Must be valid UTF-8 bytes. - Must have values that match regex `[A-Za-z\\d\\.\\-_]{1,100}` If the value of any dimension matches regex `__.*__`, the partition is reserved/read-only. A reserved/read-only partition ID is forbidden in certain documented contexts. Foreign partition IDs (in which the project ID does not match the context project ID ) are discouraged. Reads and writes of foreign partition IDs may fail if the project is not in an active state."]
    pub struct PartitionId {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "namespaceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If not empty, the ID of the namespace to which the entities belong."]
        pub namespace_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the project to which the entities belong."]
        pub project_id: ::std::option::Option<::std::string::String>,
    }
    impl PartitionId {
        pub fn builder() -> PartitionIdBuilder {
            PartitionIdBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A (kind, ID/name) pair used to construct a key path. If either name or ID is set, the element is complete. If neither is set, the element is incomplete."]
    pub struct PathElement {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The auto-allocated ID of the entity. Never equal to zero. Values less than zero are discouraged and may not be supported in the future."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The kind of the entity. A kind matching regex `__.*__` is reserved/read-only. A kind must not contain more than 1500 bytes when UTF-8 encoded. Cannot be `\"\"`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the entity. A name matching regex `__.*__` is reserved/read-only. A name must not be more than 1500 bytes when UTF-8 encoded. Cannot be `\"\"`."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl PathElement {
        pub fn builder() -> PathElementBuilder {
            PathElementBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A representation of a property in a projection."]
    pub struct Projection {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The property to project."]
        pub property: ::std::option::Option<::std::boxed::Box<PropertyReference>>,
    }
    impl Projection {
        pub fn builder() -> ProjectionBuilder {
            ProjectionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A filter on a specific property."]
    pub struct PropertyFilter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "op")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The operator to filter by."]
        pub op: ::std::option::Option<PropertyFilterOpEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The property to filter by."]
        pub property: ::std::option::Option<::std::boxed::Box<PropertyReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value to compare the property to."]
        pub value: ::std::option::Option<::std::boxed::Box<Value>>,
    }
    impl PropertyFilter {
        pub fn builder() -> PropertyFilterBuilder {
            PropertyFilterBuilder::default()
        }
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
    impl ::std::default::Default for PropertyFilterOpEnum {
        fn default() -> Self {
            Self::OperatorUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The desired order for a specific property."]
    pub struct PropertyOrder {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "direction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The direction to order by. Defaults to `ASCENDING`."]
        pub direction: ::std::option::Option<PropertyOrderDirectionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The property to order by."]
        pub property: ::std::option::Option<::std::boxed::Box<PropertyReference>>,
    }
    impl PropertyOrder {
        pub fn builder() -> PropertyOrderBuilder {
            PropertyOrderBuilder::default()
        }
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
    impl ::std::default::Default for PropertyOrderDirectionEnum {
        fn default() -> Self {
            Self::DirectionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A reference to a property relative to the kind expressions."]
    pub struct PropertyReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the property. If name includes \".\"s, it may be interpreted as a property name path."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl PropertyReference {
        pub fn builder() -> PropertyReferenceBuilder {
            PropertyReferenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A query for entities."]
    pub struct Query {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "distinctOn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The properties to make distinct. The query results will contain the first result for each distinct combination of values for the given properties (if empty, all results are returned)."]
        pub distinct_on:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PropertyReference>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endCursor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An ending point for the query results. Query cursors are returned in query result batches and [can only be used to limit the same query](https://cloud.google.com/datastore/docs/concepts/queries#cursors_limits_and_offsets)."]
        pub end_cursor: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The filter to apply."]
        pub filter: ::std::option::Option<::std::boxed::Box<Filter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The kinds to query (if empty, returns entities of all kinds). Currently at most 1 kind may be specified."]
        pub kind: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<KindExpression>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "limit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of results to return. Applies after all other constraints. Optional. Unspecified is interpreted as no limit. Must be >= 0 if specified."]
        pub limit: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of results to skip. Applies before limit, but after all other constraints. Optional. Must be >= 0 if specified."]
        pub offset: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "order")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The order to apply to the query results (if empty, order is unspecified)."]
        pub order: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PropertyOrder>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projection")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The projection to return. Defaults to returning all properties."]
        pub projection: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Projection>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startCursor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A starting point for the query results. Query cursors are returned in query result batches and [can only be used to continue the same query](https://cloud.google.com/datastore/docs/concepts/queries#cursors_limits_and_offsets)."]
        pub start_cursor: ::std::option::Option<::std::string::String>,
    }
    impl Query {
        pub fn builder() -> QueryBuilder {
            QueryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A batch of results produced by a query."]
    pub struct QueryResultBatch {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endCursor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A cursor that points to the position after the last result in the batch."]
        pub end_cursor: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityResultType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result type for every entity in `entity_results`."]
        pub entity_result_type: ::std::option::Option<QueryResultBatchEntityResultTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The results for this batch."]
        pub entity_results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EntityResult>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "moreResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state of the query after the current batch."]
        pub more_results: ::std::option::Option<QueryResultBatchMoreResultsEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "skippedCursor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A cursor that points to the position after the last skipped result. Will be set when `skipped_results` != 0."]
        pub skipped_cursor: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "skippedResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of results skipped, typically because of an offset."]
        pub skipped_results: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snapshotVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version number of the snapshot this batch was returned from. This applies to the range of results from the query's `start_cursor` (or the beginning of the query if no cursor was given) to this batch's `end_cursor` (not the query's `end_cursor`). In a single transaction, subsequent query result batches for the same query can have a greater snapshot version number. Each batch's snapshot version is valid for all preceding batches. The value will be zero for eventually consistent queries."]
        pub snapshot_version: ::std::option::Option<::std::string::String>,
    }
    impl QueryResultBatch {
        pub fn builder() -> QueryResultBatchBuilder {
            QueryResultBatchBuilder::default()
        }
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
    impl ::std::default::Default for QueryResultBatchEntityResultTypeEnum {
        fn default() -> Self {
            Self::ResultTypeUnspecified
        }
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
    impl ::std::default::Default for QueryResultBatchMoreResultsEnum {
        fn default() -> Self {
            Self::MoreResultsTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Options specific to read-only transactions."]
    pub struct ReadOnly {}
    impl ReadOnly {
        pub fn builder() -> ReadOnlyBuilder {
            ReadOnlyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The options shared by read requests."]
    pub struct ReadOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "readConsistency")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The non-transactional read consistency to use. Cannot be set to `STRONG` for global queries."]
        pub read_consistency: ::std::option::Option<ReadOptionsReadConsistencyEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transaction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier of the transaction in which to read. A transaction identifier is returned by a call to Datastore.BeginTransaction."]
        pub transaction: ::std::option::Option<::std::string::String>,
    }
    impl ReadOptions {
        pub fn builder() -> ReadOptionsBuilder {
            ReadOptionsBuilder::default()
        }
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
    impl ::std::default::Default for ReadOptionsReadConsistencyEnum {
        fn default() -> Self {
            Self::ReadConsistencyUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Options specific to read / write transactions."]
    pub struct ReadWrite {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previousTransaction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The transaction identifier of the transaction being retried."]
        pub previous_transaction: ::std::option::Option<::std::string::String>,
    }
    impl ReadWrite {
        pub fn builder() -> ReadWriteBuilder {
            ReadWriteBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request for Datastore.ReserveIds."]
    pub struct ReserveIdsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "databaseId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If not empty, the ID of the database against which to make the request."]
        pub database_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keys")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A list of keys with complete key paths whose numeric IDs should not be auto-allocated."]
        pub keys: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Key>>>,
    }
    impl ReserveIdsRequest {
        pub fn builder() -> ReserveIdsRequestBuilder {
            ReserveIdsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for Datastore.ReserveIds."]
    pub struct ReserveIdsResponse {}
    impl ReserveIdsResponse {
        pub fn builder() -> ReserveIdsResponseBuilder {
            ReserveIdsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request for Datastore.Rollback."]
    pub struct RollbackRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transaction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The transaction identifier, returned by a call to Datastore.BeginTransaction."]
        pub transaction: ::std::option::Option<::std::string::String>,
    }
    impl RollbackRequest {
        pub fn builder() -> RollbackRequestBuilder {
            RollbackRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for Datastore.Rollback. (an empty message)."]
    pub struct RollbackResponse {}
    impl RollbackResponse {
        pub fn builder() -> RollbackResponseBuilder {
            RollbackResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request for Datastore.RunQuery."]
    pub struct RunQueryRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gqlQuery")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The GQL query to run."]
        pub gql_query: ::std::option::Option<::std::boxed::Box<GqlQuery>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partitionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entities are partitioned into subsets, identified by a partition ID. Queries are scoped to a single partition. This partition ID is normalized with the standard default context partition ID."]
        pub partition_id: ::std::option::Option<::std::boxed::Box<PartitionId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "query")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The query to run."]
        pub query: ::std::option::Option<::std::boxed::Box<Query>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "readOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The options for this query."]
        pub read_options: ::std::option::Option<::std::boxed::Box<ReadOptions>>,
    }
    impl RunQueryRequest {
        pub fn builder() -> RunQueryRequestBuilder {
            RunQueryRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for Datastore.RunQuery."]
    pub struct RunQueryResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A batch of query results (always present)."]
        pub batch: ::std::option::Option<::std::boxed::Box<QueryResultBatch>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "query")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The parsed form of the `GqlQuery` from the request, if it was set."]
        pub query: ::std::option::Option<::std::boxed::Box<Query>>,
    }
    impl RunQueryResponse {
        pub fn builder() -> RunQueryResponseBuilder {
            RunQueryResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
    pub struct Status {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        pub code: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "details")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of messages that carry the error details. There is a common set of message types for APIs to use."]
        pub details: ::std::option::Option<
            ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client."]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl Status {
        pub fn builder() -> StatusBuilder {
            StatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Options for beginning a new transaction. Transactions can be created explicitly with calls to Datastore.BeginTransaction or implicitly by setting ReadOptions.new_transaction in read requests."]
    pub struct TransactionOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "readOnly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The transaction should only allow reads."]
        pub read_only: ::std::option::Option<::std::boxed::Box<ReadOnly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "readWrite")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The transaction should allow both reads and writes."]
        pub read_write: ::std::option::Option<::std::boxed::Box<ReadWrite>>,
    }
    impl TransactionOptions {
        pub fn builder() -> TransactionOptionsBuilder {
            TransactionOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A message that can hold any of the supported value types and associated metadata."]
    pub struct Value {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "arrayValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An array value. Cannot contain another array value. A `Value` instance that sets field `array_value` must not set fields `meaning` or `exclude_from_indexes`."]
        pub array_value: ::std::option::Option<::std::boxed::Box<ArrayValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blobValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A blob value. May have at most 1,000,000 bytes. When `exclude_from_indexes` is false, may have at most 1500 bytes. In JSON requests, must be base64-encoded."]
        pub blob_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "booleanValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A boolean value."]
        pub boolean_value: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "doubleValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A double value."]
        pub double_value: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An entity value. - May have no key. - May have a key with an incomplete key path. - May have a reserved/read-only key."]
        pub entity_value: ::std::option::Option<::std::boxed::Box<Entity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludeFromIndexes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the value should be excluded from all indexes including those defined explicitly."]
        pub exclude_from_indexes: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "geoPointValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A geo point value representing a point on the surface of Earth."]
        pub geo_point_value: ::std::option::Option<::std::boxed::Box<LatLng>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "integerValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An integer value."]
        pub integer_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A key value."]
        pub key_value: ::std::option::Option<::std::boxed::Box<Key>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "meaning")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `meaning` field should only be populated for backwards compatibility."]
        pub meaning: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nullValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A null value."]
        pub null_value: ::std::option::Option<ValueNullValueEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stringValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A UTF-8 encoded string value. When `exclude_from_indexes` is false (it is indexed) , may have at most 1500 bytes. Otherwise, may be set to at most 1,000,000 bytes."]
        pub string_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestampValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A timestamp value. When stored in the Datastore, precise only to microseconds; any additional precision is rounded down."]
        pub timestamp_value: ::std::option::Option<::std::string::String>,
    }
    impl Value {
        pub fn builder() -> ValueBuilder {
            ValueBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "A null value."]
    pub enum ValueNullValueEnum {
        #[serde(rename = "NULL_VALUE")]
        #[doc = "Null value."]
        NullValue,
    }
    impl ::std::default::Default for ValueNullValueEnum {
        fn default() -> Self {
            Self::NullValue
        }
    }
}
