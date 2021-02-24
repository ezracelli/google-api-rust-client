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
            pub mod databases {
                pub mod resources {
                    pub mod collection_groups {
                        pub mod resources {
                            pub mod fields {
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
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The filter to apply to list results. Currently, FirestoreAdmin.ListFields only supports listing fields that have been explicitly overridden. To issue this query, call FirestoreAdmin.ListFields with the filter set to `indexConfig.usesAncestorConfig:false`."]
                                            pub filter:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The number of results to return."]
                                            pub page_size:
                                                ::std::option::Option<::std::primitive::i64>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "A page token, returned from a previous call to FirestoreAdmin.ListFields, that may be used to get the next page of results."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                    pub mod patch {
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
                                            #[serde(rename = "updateMask")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "A mask, relative to the field. If specified, only configuration specified by this field_mask will be updated in the field."]
                                            pub update_mask:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                }
                            }
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
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The filter to apply to list results."]
                                            pub filter:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The number of results to return."]
                                            pub page_size:
                                                ::std::option::Option<::std::primitive::i64>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "A page token, returned from a previous call to FirestoreAdmin.ListIndexes, that may be used to get the next page of results."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
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
            }
        }
    }
}
pub mod schemas {
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
    #[doc = "Metadata for google.longrunning.Operation results from FirestoreAdmin.ExportDocuments."]
    pub struct GoogleFirestoreAdminV1beta2ExportDocumentsMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "collectionIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Which collection ids are being exported."]
        pub collection_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time this operation completed. Will be unset if operation still in progress."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state of the export operation."]
        pub operation_state: ::std::option::Option<
            GoogleFirestoreAdminV1beta2ExportDocumentsMetadataOperationStateEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputUriPrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Where the entities are being exported to."]
        pub output_uri_prefix: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The progress, in bytes, of this operation."]
        pub progress_bytes:
            ::std::option::Option<::std::boxed::Box<GoogleFirestoreAdminV1beta2Progress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressDocuments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The progress, in documents, of this operation."]
        pub progress_documents:
            ::std::option::Option<::std::boxed::Box<GoogleFirestoreAdminV1beta2Progress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time this operation started."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleFirestoreAdminV1beta2ExportDocumentsMetadata {
        pub fn builder() -> GoogleFirestoreAdminV1beta2ExportDocumentsMetadataBuilder {
            GoogleFirestoreAdminV1beta2ExportDocumentsMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The state of the export operation."]
    pub enum GoogleFirestoreAdminV1beta2ExportDocumentsMetadataOperationStateEnum {
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
    impl ::std::default::Default
        for GoogleFirestoreAdminV1beta2ExportDocumentsMetadataOperationStateEnum
    {
        fn default() -> Self {
            Self::OperationStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request for FirestoreAdmin.ExportDocuments."]
    pub struct GoogleFirestoreAdminV1beta2ExportDocumentsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "collectionIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Which collection ids to export. Unspecified means all collections."]
        pub collection_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputUriPrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The output URI. Currently only supports Google Cloud Storage URIs of the form: `gs://BUCKET_NAME[/NAMESPACE_PATH]`, where `BUCKET_NAME` is the name of the Google Cloud Storage bucket and `NAMESPACE_PATH` is an optional Google Cloud Storage namespace path. When choosing a name, be sure to consider Google Cloud Storage naming guidelines: https://cloud.google.com/storage/docs/naming. If the URI is a bucket (without a namespace path), a prefix will be generated based on the start time."]
        pub output_uri_prefix: ::std::option::Option<::std::string::String>,
    }
    impl GoogleFirestoreAdminV1beta2ExportDocumentsRequest {
        pub fn builder() -> GoogleFirestoreAdminV1beta2ExportDocumentsRequestBuilder {
            GoogleFirestoreAdminV1beta2ExportDocumentsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Returned in the google.longrunning.Operation response field."]
    pub struct GoogleFirestoreAdminV1beta2ExportDocumentsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputUriPrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location of the output files. This can be used to begin an import into Cloud Firestore (this project or another project) after the operation completes successfully."]
        pub output_uri_prefix: ::std::option::Option<::std::string::String>,
    }
    impl GoogleFirestoreAdminV1beta2ExportDocumentsResponse {
        pub fn builder() -> GoogleFirestoreAdminV1beta2ExportDocumentsResponseBuilder {
            GoogleFirestoreAdminV1beta2ExportDocumentsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a single field in the database. Fields are grouped by their \"Collection Group\", which represent all collections in the database with the same id."]
    pub struct GoogleFirestoreAdminV1beta2Field {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "indexConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index configuration for this field. If unset, field indexing will revert to the configuration defined by the `ancestor_field`. To explicitly remove all indexes for this field, specify an index config with an empty list of indexes."]
        pub index_config:
            ::std::option::Option<::std::boxed::Box<GoogleFirestoreAdminV1beta2IndexConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A field name of the form `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/fields/{field_path}` A field path may be a simple field name, e.g. `address` or a path to fields within map_value , e.g. `address.city`, or a special field path. The only valid special field is `*`, which represents any field. Field paths may be quoted using ` (backtick). The only character that needs to be escaped within a quoted field path is the backtick character itself, escaped using a backslash. Special characters in field paths that must be quoted include: `*`, `.`, ``` (backtick), `[`, `]`, as well as any ascii symbolic characters. Examples: (Note: Comments here are written in markdown syntax, so there is an additional layer of backticks to represent a code block) `\\`address.city\\`` represents a field named `address.city`, not the map key `city` in the field `address`. `\\`*\\`` represents a field named `*`, not any field. A special `Field` contains the default indexing settings for all fields. This field's resource name is: `projects/{project_id}/databases/{database_id}/collectionGroups/__default__/fields/*` Indexes defined on this `Field` will be applied to all fields which do not have their own `Field` index configuration."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleFirestoreAdminV1beta2Field {
        pub fn builder() -> GoogleFirestoreAdminV1beta2FieldBuilder {
            GoogleFirestoreAdminV1beta2FieldBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for google.longrunning.Operation results from FirestoreAdmin.UpdateField."]
    pub struct GoogleFirestoreAdminV1beta2FieldOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bytesProgress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The progress, in bytes, of this operation."]
        pub bytes_progress:
            ::std::option::Option<::std::boxed::Box<GoogleFirestoreAdminV1beta2Progress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "documentProgress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The progress, in documents, of this operation."]
        pub document_progress:
            ::std::option::Option<::std::boxed::Box<GoogleFirestoreAdminV1beta2Progress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time this operation completed. Will be unset if operation still in progress."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "field")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The field resource that this operation is acting on. For example: `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/fields/{field_path}`"]
        pub field: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "indexConfigDeltas")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of IndexConfigDelta, which describe the intent of this operation."]
        pub index_config_deltas: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleFirestoreAdminV1beta2IndexConfigDelta>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time this operation started."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state of the operation."]
        pub state:
            ::std::option::Option<GoogleFirestoreAdminV1beta2FieldOperationMetadataStateEnum>,
    }
    impl GoogleFirestoreAdminV1beta2FieldOperationMetadata {
        pub fn builder() -> GoogleFirestoreAdminV1beta2FieldOperationMetadataBuilder {
            GoogleFirestoreAdminV1beta2FieldOperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The state of the operation."]
    pub enum GoogleFirestoreAdminV1beta2FieldOperationMetadataStateEnum {
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
    impl ::std::default::Default for GoogleFirestoreAdminV1beta2FieldOperationMetadataStateEnum {
        fn default() -> Self {
            Self::OperationStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for google.longrunning.Operation results from FirestoreAdmin.ImportDocuments."]
    pub struct GoogleFirestoreAdminV1beta2ImportDocumentsMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "collectionIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Which collection ids are being imported."]
        pub collection_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time this operation completed. Will be unset if operation still in progress."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputUriPrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location of the documents being imported."]
        pub input_uri_prefix: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state of the import operation."]
        pub operation_state: ::std::option::Option<
            GoogleFirestoreAdminV1beta2ImportDocumentsMetadataOperationStateEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The progress, in bytes, of this operation."]
        pub progress_bytes:
            ::std::option::Option<::std::boxed::Box<GoogleFirestoreAdminV1beta2Progress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressDocuments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The progress, in documents, of this operation."]
        pub progress_documents:
            ::std::option::Option<::std::boxed::Box<GoogleFirestoreAdminV1beta2Progress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time this operation started."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleFirestoreAdminV1beta2ImportDocumentsMetadata {
        pub fn builder() -> GoogleFirestoreAdminV1beta2ImportDocumentsMetadataBuilder {
            GoogleFirestoreAdminV1beta2ImportDocumentsMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The state of the import operation."]
    pub enum GoogleFirestoreAdminV1beta2ImportDocumentsMetadataOperationStateEnum {
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
    impl ::std::default::Default
        for GoogleFirestoreAdminV1beta2ImportDocumentsMetadataOperationStateEnum
    {
        fn default() -> Self {
            Self::OperationStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request for FirestoreAdmin.ImportDocuments."]
    pub struct GoogleFirestoreAdminV1beta2ImportDocumentsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "collectionIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Which collection ids to import. Unspecified means all collections included in the import."]
        pub collection_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputUriPrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location of the exported files. This must match the output_uri_prefix of an ExportDocumentsResponse from an export that has completed successfully. See: google.firestore.admin.v1beta2.ExportDocumentsResponse.output_uri_prefix."]
        pub input_uri_prefix: ::std::option::Option<::std::string::String>,
    }
    impl GoogleFirestoreAdminV1beta2ImportDocumentsRequest {
        pub fn builder() -> GoogleFirestoreAdminV1beta2ImportDocumentsRequestBuilder {
            GoogleFirestoreAdminV1beta2ImportDocumentsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Cloud Firestore indexes enable simple and complex queries against documents in a database."]
    pub struct GoogleFirestoreAdminV1beta2Index {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields supported by this index. For composite indexes, this is always 2 or more fields. The last field entry is always for the field path `__name__`. If, on creation, `__name__` was not specified as the last field, it will be added automatically with the same direction as that of the last field defined. If the final field in a composite index is not directional, the `__name__` will be ordered ASCENDING (unless explicitly specified). For single field indexes, this will always be exactly one entry with a field path equal to the field path of the associated field."]
        pub fields: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleFirestoreAdminV1beta2IndexField>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A server defined name for this index. The form of this name for composite indexes will be: `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/indexes/{composite_index_id}` For single field indexes, this field will be empty."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queryScope")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indexes with a collection query scope specified allow queries against a collection that is the child of a specific document, specified at query time, and that has the same collection id. Indexes with a collection group query scope specified allow queries against all collections descended from a specific document, specified at query time, and that have the same collection id as this index."]
        pub query_scope: ::std::option::Option<GoogleFirestoreAdminV1beta2IndexQueryScopeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The serving state of the index."]
        pub state: ::std::option::Option<GoogleFirestoreAdminV1beta2IndexStateEnum>,
    }
    impl GoogleFirestoreAdminV1beta2Index {
        pub fn builder() -> GoogleFirestoreAdminV1beta2IndexBuilder {
            GoogleFirestoreAdminV1beta2IndexBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Indexes with a collection query scope specified allow queries against a collection that is the child of a specific document, specified at query time, and that has the same collection id. Indexes with a collection group query scope specified allow queries against all collections descended from a specific document, specified at query time, and that have the same collection id as this index."]
    pub enum GoogleFirestoreAdminV1beta2IndexQueryScopeEnum {
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
    impl ::std::default::Default for GoogleFirestoreAdminV1beta2IndexQueryScopeEnum {
        fn default() -> Self {
            Self::QueryScopeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The serving state of the index."]
    pub enum GoogleFirestoreAdminV1beta2IndexStateEnum {
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
    impl ::std::default::Default for GoogleFirestoreAdminV1beta2IndexStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The index configuration for this field."]
    pub struct GoogleFirestoreAdminV1beta2IndexConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ancestorField")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Specifies the resource name of the `Field` from which this field's index configuration is set (when `uses_ancestor_config` is true), or from which it *would* be set if this field had no index configuration (when `uses_ancestor_config` is false)."]
        pub ancestor_field: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "indexes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The indexes supported for this field."]
        pub indexes: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleFirestoreAdminV1beta2Index>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reverting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only When true, the `Field`'s index configuration is in the process of being reverted. Once complete, the index config will transition to the same state as the field specified by `ancestor_field`, at which point `uses_ancestor_config` will be `true` and `reverting` will be `false`."]
        pub reverting: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "usesAncestorConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. When true, the `Field`'s index configuration is set from the configuration specified by the `ancestor_field`. When false, the `Field`'s index configuration is defined explicitly."]
        pub uses_ancestor_config: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleFirestoreAdminV1beta2IndexConfig {
        pub fn builder() -> GoogleFirestoreAdminV1beta2IndexConfigBuilder {
            GoogleFirestoreAdminV1beta2IndexConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about an index configuration change."]
    pub struct GoogleFirestoreAdminV1beta2IndexConfigDelta {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "changeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies how the index is changing."]
        pub change_type:
            ::std::option::Option<GoogleFirestoreAdminV1beta2IndexConfigDeltaChangeTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "index")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index being changed."]
        pub index: ::std::option::Option<::std::boxed::Box<GoogleFirestoreAdminV1beta2Index>>,
    }
    impl GoogleFirestoreAdminV1beta2IndexConfigDelta {
        pub fn builder() -> GoogleFirestoreAdminV1beta2IndexConfigDeltaBuilder {
            GoogleFirestoreAdminV1beta2IndexConfigDeltaBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies how the index is changing."]
    pub enum GoogleFirestoreAdminV1beta2IndexConfigDeltaChangeTypeEnum {
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
    impl ::std::default::Default for GoogleFirestoreAdminV1beta2IndexConfigDeltaChangeTypeEnum {
        fn default() -> Self {
            Self::ChangeTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A field in an index. The field_path describes which field is indexed, the value_mode describes how the field value is indexed."]
    pub struct GoogleFirestoreAdminV1beta2IndexField {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "arrayConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates that this field supports operations on `array_value`s."]
        pub array_config:
            ::std::option::Option<GoogleFirestoreAdminV1beta2IndexFieldArrayConfigEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldPath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Can be __name__. For single field indexes, this must match the name of the field or may be omitted."]
        pub field_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "order")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates that this field supports ordering by the specified order or comparing using =, <, <=, >, >=."]
        pub order: ::std::option::Option<GoogleFirestoreAdminV1beta2IndexFieldOrderEnum>,
    }
    impl GoogleFirestoreAdminV1beta2IndexField {
        pub fn builder() -> GoogleFirestoreAdminV1beta2IndexFieldBuilder {
            GoogleFirestoreAdminV1beta2IndexFieldBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Indicates that this field supports operations on `array_value`s."]
    pub enum GoogleFirestoreAdminV1beta2IndexFieldArrayConfigEnum {
        #[serde(rename = "ARRAY_CONFIG_UNSPECIFIED")]
        #[doc = "The index does not support additional array queries."]
        ArrayConfigUnspecified,
        #[serde(rename = "CONTAINS")]
        #[doc = "The index supports array containment queries."]
        Contains,
    }
    impl ::std::default::Default for GoogleFirestoreAdminV1beta2IndexFieldArrayConfigEnum {
        fn default() -> Self {
            Self::ArrayConfigUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Indicates that this field supports ordering by the specified order or comparing using =, <, <=, >, >=."]
    pub enum GoogleFirestoreAdminV1beta2IndexFieldOrderEnum {
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
    impl ::std::default::Default for GoogleFirestoreAdminV1beta2IndexFieldOrderEnum {
        fn default() -> Self {
            Self::OrderUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for google.longrunning.Operation results from FirestoreAdmin.CreateIndex."]
    pub struct GoogleFirestoreAdminV1beta2IndexOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time this operation completed. Will be unset if operation still in progress."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "index")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index resource that this operation is acting on. For example: `projects/{project_id}/databases/{database_id}/collectionGroups/{collection_id}/indexes/{index_id}`"]
        pub index: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The progress, in bytes, of this operation."]
        pub progress_bytes:
            ::std::option::Option<::std::boxed::Box<GoogleFirestoreAdminV1beta2Progress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressDocuments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The progress, in documents, of this operation."]
        pub progress_documents:
            ::std::option::Option<::std::boxed::Box<GoogleFirestoreAdminV1beta2Progress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time this operation started."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state of the operation."]
        pub state:
            ::std::option::Option<GoogleFirestoreAdminV1beta2IndexOperationMetadataStateEnum>,
    }
    impl GoogleFirestoreAdminV1beta2IndexOperationMetadata {
        pub fn builder() -> GoogleFirestoreAdminV1beta2IndexOperationMetadataBuilder {
            GoogleFirestoreAdminV1beta2IndexOperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The state of the operation."]
    pub enum GoogleFirestoreAdminV1beta2IndexOperationMetadataStateEnum {
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
    impl ::std::default::Default for GoogleFirestoreAdminV1beta2IndexOperationMetadataStateEnum {
        fn default() -> Self {
            Self::OperationStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for FirestoreAdmin.ListFields."]
    pub struct GoogleFirestoreAdminV1beta2ListFieldsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The requested fields."]
        pub fields: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleFirestoreAdminV1beta2Field>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A page token that may be used to request another page of results. If blank, this is the last page."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleFirestoreAdminV1beta2ListFieldsResponse {
        pub fn builder() -> GoogleFirestoreAdminV1beta2ListFieldsResponseBuilder {
            GoogleFirestoreAdminV1beta2ListFieldsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for FirestoreAdmin.ListIndexes."]
    pub struct GoogleFirestoreAdminV1beta2ListIndexesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "indexes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The requested indexes."]
        pub indexes: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleFirestoreAdminV1beta2Index>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A page token that may be used to request another page of results. If blank, this is the last page."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleFirestoreAdminV1beta2ListIndexesResponse {
        pub fn builder() -> GoogleFirestoreAdminV1beta2ListIndexesResponseBuilder {
            GoogleFirestoreAdminV1beta2ListIndexesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes the progress of the operation. Unit of work is generic and must be interpreted based on where Progress is used."]
    pub struct GoogleFirestoreAdminV1beta2Progress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "completedWork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of work completed."]
        pub completed_work: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "estimatedWork")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of work estimated."]
        pub estimated_work: ::std::option::Option<::std::string::String>,
    }
    impl GoogleFirestoreAdminV1beta2Progress {
        pub fn builder() -> GoogleFirestoreAdminV1beta2ProgressBuilder {
            GoogleFirestoreAdminV1beta2ProgressBuilder::default()
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
}
