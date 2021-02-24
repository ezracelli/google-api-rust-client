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
pub mod schemas {
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
    #[doc = "The request for google.datastore.admin.v1beta1.DatastoreAdmin.ExportEntities."]
    pub struct GoogleDatastoreAdminV1beta1ExportEntitiesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of what data from the project is included in the export."]
        pub entity_filter:
            ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1beta1EntityFilter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Client-assigned labels."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputUrlPrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location for the export metadata and data files. The full resource URL of the external storage location. Currently, only Google Cloud Storage is supported. So output_url_prefix should be of the form: `gs://BUCKET_NAME[/NAMESPACE_PATH]`, where `BUCKET_NAME` is the name of the Cloud Storage bucket and `NAMESPACE_PATH` is an optional Cloud Storage namespace path (this is not a Cloud Datastore namespace). For more information about Cloud Storage namespace paths, see [Object name considerations](https://cloud.google.com/storage/docs/naming#object-considerations). The resulting files will be nested deeper than the specified URL prefix. The final output URL will be provided in the google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url field. That value should be used for subsequent ImportEntities operations. By nesting the data files deeper, the same Cloud Storage bucket can be used in multiple ExportEntities operations without conflict."]
        pub output_url_prefix: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDatastoreAdminV1beta1ExportEntitiesRequest {
        pub fn builder() -> GoogleDatastoreAdminV1beta1ExportEntitiesRequestBuilder {
            GoogleDatastoreAdminV1beta1ExportEntitiesRequestBuilder::default()
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
    #[doc = "The request for google.datastore.admin.v1beta1.DatastoreAdmin.ImportEntities."]
    pub struct GoogleDatastoreAdminV1beta1ImportEntitiesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optionally specify which kinds/namespaces are to be imported. If provided, the list must be a subset of the EntityFilter used in creating the export, otherwise a FAILED_PRECONDITION error will be returned. If no filter is specified then all entities from the export are imported."]
        pub entity_filter:
            ::std::option::Option<::std::boxed::Box<GoogleDatastoreAdminV1beta1EntityFilter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full resource URL of the external storage location. Currently, only Google Cloud Storage is supported. So input_url should be of the form: `gs://BUCKET_NAME[/NAMESPACE_PATH]/OVERALL_EXPORT_METADATA_FILE`, where `BUCKET_NAME` is the name of the Cloud Storage bucket, `NAMESPACE_PATH` is an optional Cloud Storage namespace path (this is not a Cloud Datastore namespace), and `OVERALL_EXPORT_METADATA_FILE` is the metadata file written by the ExportEntities operation. For more information about Cloud Storage namespace paths, see [Object name considerations](https://cloud.google.com/storage/docs/naming#object-considerations). For more information, see google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url."]
        pub input_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Client-assigned labels."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl GoogleDatastoreAdminV1beta1ImportEntitiesRequest {
        pub fn builder() -> GoogleDatastoreAdminV1beta1ImportEntitiesRequestBuilder {
            GoogleDatastoreAdminV1beta1ImportEntitiesRequestBuilder::default()
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
