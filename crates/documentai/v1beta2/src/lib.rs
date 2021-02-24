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
    #[doc = "The common metadata for long running operations."]
    pub struct GoogleCloudDocumentaiUiv1beta3CommonOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creation time of the operation."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state of the operation."]
        pub state:
            ::std::option::Option<GoogleCloudDocumentaiUiv1beta3CommonOperationMetadataStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A message providing more details about the current state of processing."]
        pub state_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last update time of the operation."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiUiv1beta3CommonOperationMetadata {
        pub fn builder() -> GoogleCloudDocumentaiUiv1beta3CommonOperationMetadataBuilder {
            GoogleCloudDocumentaiUiv1beta3CommonOperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The state of the operation."]
    pub enum GoogleCloudDocumentaiUiv1beta3CommonOperationMetadataStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Unspecified state."]
        StateUnspecified,
        #[serde(rename = "RUNNING")]
        #[doc = "Operation is still running."]
        Running,
        #[serde(rename = "CANCELLING")]
        #[doc = "Operation is being cancelled."]
        Cancelling,
        #[serde(rename = "SUCCEEDED")]
        #[doc = "Operation succeeded."]
        Succeeded,
        #[serde(rename = "FAILED")]
        #[doc = "Operation failed."]
        Failed,
        #[serde(rename = "CANCELLED")]
        #[doc = "Operation is cancelled."]
        Cancelled,
    }
    impl ::std::default::Default for GoogleCloudDocumentaiUiv1beta3CommonOperationMetadataStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The long running operation metadata for CreateLabelerPool."]
    pub struct GoogleCloudDocumentaiUiv1beta3CreateLabelerPoolOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commonMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The basic metadata of the long running operation."]
        pub common_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiUiv1beta3CommonOperationMetadata>,
        >,
    }
    impl GoogleCloudDocumentaiUiv1beta3CreateLabelerPoolOperationMetadata {
        pub fn builder() -> GoogleCloudDocumentaiUiv1beta3CreateLabelerPoolOperationMetadataBuilder
        {
            GoogleCloudDocumentaiUiv1beta3CreateLabelerPoolOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The long running operation metadata for DeleteLabelerPool."]
    pub struct GoogleCloudDocumentaiUiv1beta3DeleteLabelerPoolOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commonMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The basic metadata of the long running operation."]
        pub common_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiUiv1beta3CommonOperationMetadata>,
        >,
    }
    impl GoogleCloudDocumentaiUiv1beta3DeleteLabelerPoolOperationMetadata {
        pub fn builder() -> GoogleCloudDocumentaiUiv1beta3DeleteLabelerPoolOperationMetadataBuilder
        {
            GoogleCloudDocumentaiUiv1beta3DeleteLabelerPoolOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The long running operation metadata for delete processor method."]
    pub struct GoogleCloudDocumentaiUiv1beta3DeleteProcessorMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commonMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The basic metadata of the long running operation."]
        pub common_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiUiv1beta3CommonOperationMetadata>,
        >,
    }
    impl GoogleCloudDocumentaiUiv1beta3DeleteProcessorMetadata {
        pub fn builder() -> GoogleCloudDocumentaiUiv1beta3DeleteProcessorMetadataBuilder {
            GoogleCloudDocumentaiUiv1beta3DeleteProcessorMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The long running operation metadata for delete processor version method."]
    pub struct GoogleCloudDocumentaiUiv1beta3DeleteProcessorVersionMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commonMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The basic metadata of the long running operation."]
        pub common_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiUiv1beta3CommonOperationMetadata>,
        >,
    }
    impl GoogleCloudDocumentaiUiv1beta3DeleteProcessorVersionMetadata {
        pub fn builder() -> GoogleCloudDocumentaiUiv1beta3DeleteProcessorVersionMetadataBuilder {
            GoogleCloudDocumentaiUiv1beta3DeleteProcessorVersionMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The long running operation metadata for deploy processor version method."]
    pub struct GoogleCloudDocumentaiUiv1beta3DeployProcessorVersionMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commonMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The basic metadata of the long running operation."]
        pub common_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiUiv1beta3CommonOperationMetadata>,
        >,
    }
    impl GoogleCloudDocumentaiUiv1beta3DeployProcessorVersionMetadata {
        pub fn builder() -> GoogleCloudDocumentaiUiv1beta3DeployProcessorVersionMetadataBuilder {
            GoogleCloudDocumentaiUiv1beta3DeployProcessorVersionMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for the deploy processor version method."]
    pub struct GoogleCloudDocumentaiUiv1beta3DeployProcessorVersionResponse {}
    impl GoogleCloudDocumentaiUiv1beta3DeployProcessorVersionResponse {
        pub fn builder() -> GoogleCloudDocumentaiUiv1beta3DeployProcessorVersionResponseBuilder {
            GoogleCloudDocumentaiUiv1beta3DeployProcessorVersionResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The long running operation metadata for disable processor method."]
    pub struct GoogleCloudDocumentaiUiv1beta3DisableProcessorMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commonMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The basic metadata of the long running operation."]
        pub common_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiUiv1beta3CommonOperationMetadata>,
        >,
    }
    impl GoogleCloudDocumentaiUiv1beta3DisableProcessorMetadata {
        pub fn builder() -> GoogleCloudDocumentaiUiv1beta3DisableProcessorMetadataBuilder {
            GoogleCloudDocumentaiUiv1beta3DisableProcessorMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for the disable processor method. Intentionally empty proto for adding fields in future."]
    pub struct GoogleCloudDocumentaiUiv1beta3DisableProcessorResponse {}
    impl GoogleCloudDocumentaiUiv1beta3DisableProcessorResponse {
        pub fn builder() -> GoogleCloudDocumentaiUiv1beta3DisableProcessorResponseBuilder {
            GoogleCloudDocumentaiUiv1beta3DisableProcessorResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The long running operation metadata for enable processor method."]
    pub struct GoogleCloudDocumentaiUiv1beta3EnableProcessorMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commonMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The basic metadata of the long running operation."]
        pub common_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiUiv1beta3CommonOperationMetadata>,
        >,
    }
    impl GoogleCloudDocumentaiUiv1beta3EnableProcessorMetadata {
        pub fn builder() -> GoogleCloudDocumentaiUiv1beta3EnableProcessorMetadataBuilder {
            GoogleCloudDocumentaiUiv1beta3EnableProcessorMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for the enable processor method. Intentionally empty proto for adding fields in future."]
    pub struct GoogleCloudDocumentaiUiv1beta3EnableProcessorResponse {}
    impl GoogleCloudDocumentaiUiv1beta3EnableProcessorResponse {
        pub fn builder() -> GoogleCloudDocumentaiUiv1beta3EnableProcessorResponseBuilder {
            GoogleCloudDocumentaiUiv1beta3EnableProcessorResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata of the EvaluateProcessorVersion method."]
    pub struct GoogleCloudDocumentaiUiv1beta3EvaluateProcessorVersionMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commonMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The basic metadata of the long running operation."]
        pub common_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiUiv1beta3CommonOperationMetadata>,
        >,
    }
    impl GoogleCloudDocumentaiUiv1beta3EvaluateProcessorVersionMetadata {
        pub fn builder() -> GoogleCloudDocumentaiUiv1beta3EvaluateProcessorVersionMetadataBuilder {
            GoogleCloudDocumentaiUiv1beta3EvaluateProcessorVersionMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata of the EvaluateProcessorVersion method."]
    pub struct GoogleCloudDocumentaiUiv1beta3EvaluateProcessorVersionResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "evaluation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the created evaluation."]
        pub evaluation: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiUiv1beta3EvaluateProcessorVersionResponse {
        pub fn builder() -> GoogleCloudDocumentaiUiv1beta3EvaluateProcessorVersionResponseBuilder {
            GoogleCloudDocumentaiUiv1beta3EvaluateProcessorVersionResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The metadata that represents a processor version being created."]
    pub struct GoogleCloudDocumentaiUiv1beta3TrainProcessorVersionMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commonMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The basic metadata of the long running operation."]
        pub common_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiUiv1beta3CommonOperationMetadata>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testDatasetValidation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The test dataset validation information."]
        pub test_dataset_validation: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDocumentaiUiv1beta3TrainProcessorVersionMetadataDatasetValidation,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trainingDatasetValidation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The training dataset validation information."]
        pub training_dataset_validation: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDocumentaiUiv1beta3TrainProcessorVersionMetadataDatasetValidation,
            >,
        >,
    }
    impl GoogleCloudDocumentaiUiv1beta3TrainProcessorVersionMetadata {
        pub fn builder() -> GoogleCloudDocumentaiUiv1beta3TrainProcessorVersionMetadataBuilder {
            GoogleCloudDocumentaiUiv1beta3TrainProcessorVersionMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The dataset validation information. This includes any and all errors with documents and the dataset."]
    pub struct GoogleCloudDocumentaiUiv1beta3TrainProcessorVersionMetadataDatasetValidation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datasetErrors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Error information for the dataset as a whole. A maximum of 10 dataset errors will be returned. A single dataset error is terminal for training."]
        pub dataset_errors:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "documentErrors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Error information pertaining to specific documents. A maximum of 10 document errors will be returned. Any document with errors will not be used throughout training."]
        pub document_errors:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
    }
    impl GoogleCloudDocumentaiUiv1beta3TrainProcessorVersionMetadataDatasetValidation {
        pub fn builder(
        ) -> GoogleCloudDocumentaiUiv1beta3TrainProcessorVersionMetadataDatasetValidationBuilder
        {
            GoogleCloudDocumentaiUiv1beta3TrainProcessorVersionMetadataDatasetValidationBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for the TrainProcessorVersion method."]
    pub struct GoogleCloudDocumentaiUiv1beta3TrainProcessorVersionResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "processorVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the processor version produced by training."]
        pub processor_version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiUiv1beta3TrainProcessorVersionResponse {
        pub fn builder() -> GoogleCloudDocumentaiUiv1beta3TrainProcessorVersionResponseBuilder {
            GoogleCloudDocumentaiUiv1beta3TrainProcessorVersionResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The long running operation metadata for the undeploy processor version method."]
    pub struct GoogleCloudDocumentaiUiv1beta3UndeployProcessorVersionMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commonMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The basic metadata of the long running operation."]
        pub common_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiUiv1beta3CommonOperationMetadata>,
        >,
    }
    impl GoogleCloudDocumentaiUiv1beta3UndeployProcessorVersionMetadata {
        pub fn builder() -> GoogleCloudDocumentaiUiv1beta3UndeployProcessorVersionMetadataBuilder {
            GoogleCloudDocumentaiUiv1beta3UndeployProcessorVersionMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for the undeploy processor version method."]
    pub struct GoogleCloudDocumentaiUiv1beta3UndeployProcessorVersionResponse {}
    impl GoogleCloudDocumentaiUiv1beta3UndeployProcessorVersionResponse {
        pub fn builder() -> GoogleCloudDocumentaiUiv1beta3UndeployProcessorVersionResponseBuilder {
            GoogleCloudDocumentaiUiv1beta3UndeployProcessorVersionResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The long running operation metadata for updating the human review configuration."]
    pub struct GoogleCloudDocumentaiUiv1beta3UpdateHumanReviewConfigMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commonMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The basic metadata of the long running operation."]
        pub common_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiUiv1beta3CommonOperationMetadata>,
        >,
    }
    impl GoogleCloudDocumentaiUiv1beta3UpdateHumanReviewConfigMetadata {
        pub fn builder() -> GoogleCloudDocumentaiUiv1beta3UpdateHumanReviewConfigMetadataBuilder {
            GoogleCloudDocumentaiUiv1beta3UpdateHumanReviewConfigMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The long running operation metadata for UpdateLabelerPool."]
    pub struct GoogleCloudDocumentaiUiv1beta3UpdateLabelerPoolOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commonMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The basic metadata of the long running operation."]
        pub common_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiUiv1beta3CommonOperationMetadata>,
        >,
    }
    impl GoogleCloudDocumentaiUiv1beta3UpdateLabelerPoolOperationMetadata {
        pub fn builder() -> GoogleCloudDocumentaiUiv1beta3UpdateLabelerPoolOperationMetadataBuilder
        {
            GoogleCloudDocumentaiUiv1beta3UpdateLabelerPoolOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The long running operation metadata for batch process method."]
    pub struct GoogleCloudDocumentaiV1BatchProcessMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creation time of the operation."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "individualProcessStatuses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of response details of each document."]
        pub individual_process_statuses: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<
                    GoogleCloudDocumentaiV1BatchProcessMetadataIndividualProcessStatus,
                >,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state of the current batch processing."]
        pub state: ::std::option::Option<GoogleCloudDocumentaiV1BatchProcessMetadataStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A message providing more details about the current state of processing. For example, the error message if the operation is failed."]
        pub state_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last update time of the operation."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1BatchProcessMetadata {
        pub fn builder() -> GoogleCloudDocumentaiV1BatchProcessMetadataBuilder {
            GoogleCloudDocumentaiV1BatchProcessMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The state of the current batch processing."]
    pub enum GoogleCloudDocumentaiV1BatchProcessMetadataStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "The default value. This value is used if the state is omitted."]
        StateUnspecified,
        #[serde(rename = "WAITING")]
        #[doc = "Request operation is waiting for scheduling."]
        Waiting,
        #[serde(rename = "RUNNING")]
        #[doc = "Request is being processed."]
        Running,
        #[serde(rename = "SUCCEEDED")]
        #[doc = "The batch processing completed successfully."]
        Succeeded,
        #[serde(rename = "CANCELLING")]
        #[doc = "The batch processing was being cancelled."]
        Cancelling,
        #[serde(rename = "CANCELLED")]
        #[doc = "The batch processing was cancelled."]
        Cancelled,
        #[serde(rename = "FAILED")]
        #[doc = "The batch processing has failed."]
        Failed,
    }
    impl ::std::default::Default for GoogleCloudDocumentaiV1BatchProcessMetadataStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The status of a each individual document in the batch process."]
    pub struct GoogleCloudDocumentaiV1BatchProcessMetadataIndividualProcessStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "humanReviewStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of human review on the processed document."]
        pub human_review_status:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDocumentaiV1HumanReviewStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputGcsSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source of the document, same as the [input_gcs_source] field in the request when the batch process started. The batch process is started by take snapshot of that document, since a user can move or change that document during the process."]
        pub input_gcs_source: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputGcsDestination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The output_gcs_destination (in the request as 'output_gcs_destination') of the processed document if it was successful, otherwise empty."]
        pub output_gcs_destination: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the processing of the document."]
        pub status: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
    }
    impl GoogleCloudDocumentaiV1BatchProcessMetadataIndividualProcessStatus {
        pub fn builder() -> GoogleCloudDocumentaiV1BatchProcessMetadataIndividualProcessStatusBuilder
        {
            GoogleCloudDocumentaiV1BatchProcessMetadataIndividualProcessStatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for batch process document method."]
    pub struct GoogleCloudDocumentaiV1BatchProcessResponse {}
    impl GoogleCloudDocumentaiV1BatchProcessResponse {
        pub fn builder() -> GoogleCloudDocumentaiV1BatchProcessResponseBuilder {
            GoogleCloudDocumentaiV1BatchProcessResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The common metadata for long running operations."]
    pub struct GoogleCloudDocumentaiV1CommonOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creation time of the operation."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state of the operation."]
        pub state: ::std::option::Option<GoogleCloudDocumentaiV1CommonOperationMetadataStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A message providing more details about the current state of processing."]
        pub state_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last update time of the operation."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1CommonOperationMetadata {
        pub fn builder() -> GoogleCloudDocumentaiV1CommonOperationMetadataBuilder {
            GoogleCloudDocumentaiV1CommonOperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The state of the operation."]
    pub enum GoogleCloudDocumentaiV1CommonOperationMetadataStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Unspecified state."]
        StateUnspecified,
        #[serde(rename = "RUNNING")]
        #[doc = "Operation is still running."]
        Running,
        #[serde(rename = "CANCELLING")]
        #[doc = "Operation is being cancelled."]
        Cancelling,
        #[serde(rename = "SUCCEEDED")]
        #[doc = "Operation succeeded."]
        Succeeded,
        #[serde(rename = "FAILED")]
        #[doc = "Operation failed."]
        Failed,
        #[serde(rename = "CANCELLED")]
        #[doc = "Operation is cancelled."]
        Cancelled,
    }
    impl ::std::default::Default for GoogleCloudDocumentaiV1CommonOperationMetadataStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The status of human review on a processed document."]
    pub struct GoogleCloudDocumentaiV1HumanReviewStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "humanReviewOperation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the operation triggered by the processed document. This field is populated only when the [state] is [HUMAN_REVIEW_IN_PROGRESS]. It has the same response type and metadata as the long running operation returned by [ReviewDocument] method."]
        pub human_review_operation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state of human review on the processing request."]
        pub state: ::std::option::Option<GoogleCloudDocumentaiV1HumanReviewStatusStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A message providing more details about the human review state."]
        pub state_message: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1HumanReviewStatus {
        pub fn builder() -> GoogleCloudDocumentaiV1HumanReviewStatusBuilder {
            GoogleCloudDocumentaiV1HumanReviewStatusBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The state of human review on the processing request."]
    pub enum GoogleCloudDocumentaiV1HumanReviewStatusStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Human review state is unspecified. Most likely due to an internal error."]
        StateUnspecified,
        #[serde(rename = "HUMAN_REVIEW_SKIPPED")]
        #[doc = "Human review is skipped for the document. This can happen because human review is not enabled on the processor or the processing request has been set to skip this document."]
        HumanReviewSkipped,
        #[serde(rename = "HUMAN_REVIEW_VALIDATION_PASSED")]
        #[doc = "Human review validation is triggered and passed, so no review is needed."]
        HumanReviewValidationPassed,
        #[serde(rename = "HUMAN_REVIEW_IN_PROGRESS")]
        #[doc = "Human review validation is triggered and the document is under review."]
        HumanReviewInProgress,
        #[serde(rename = "HUMAN_REVIEW_ERROR")]
        #[doc = "Some error happened during triggering human review, see the [state_message] for details."]
        HumanReviewError,
    }
    impl ::std::default::Default for GoogleCloudDocumentaiV1HumanReviewStatusStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The long running operation metadata for review document method."]
    pub struct GoogleCloudDocumentaiV1ReviewDocumentOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commonMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The basic metadata of the long running operation."]
        pub common_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1CommonOperationMetadata>,
        >,
    }
    impl GoogleCloudDocumentaiV1ReviewDocumentOperationMetadata {
        pub fn builder() -> GoogleCloudDocumentaiV1ReviewDocumentOperationMetadataBuilder {
            GoogleCloudDocumentaiV1ReviewDocumentOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for review document method."]
    pub struct GoogleCloudDocumentaiV1ReviewDocumentResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsDestination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Cloud Storage uri for the human reviewed document."]
        pub gcs_destination: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1ReviewDocumentResponse {
        pub fn builder() -> GoogleCloudDocumentaiV1ReviewDocumentResponseBuilder {
            GoogleCloudDocumentaiV1ReviewDocumentResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to an batch document processing request. This is returned in the LRO Operation after the operation is complete."]
    pub struct GoogleCloudDocumentaiV1beta1BatchProcessDocumentsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Responses for each individual document."]
        pub responses: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta1ProcessDocumentResponse>>,
        >,
    }
    impl GoogleCloudDocumentaiV1beta1BatchProcessDocumentsResponse {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1BatchProcessDocumentsResponseBuilder {
            GoogleCloudDocumentaiV1beta1BatchProcessDocumentsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A bounding polygon for the detected image annotation."]
    pub struct GoogleCloudDocumentaiV1beta1BoundingPoly {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedVertices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon normalized vertices."]
        pub normalized_vertices: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta1NormalizedVertex>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vertices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon vertices."]
        pub vertices: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta1Vertex>>,
        >,
    }
    impl GoogleCloudDocumentaiV1beta1BoundingPoly {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1BoundingPolyBuilder {
            GoogleCloudDocumentaiV1beta1BoundingPolyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Document represents the canonical document resource in Document Understanding AI. It is an interchange format that provides insights into documents and allows for collaboration between users and Document Understanding AI to iterate and optimize for quality."]
    pub struct GoogleCloudDocumentaiV1beta1Document {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Inline document content, represented as a stream of bytes. Note: As with all `bytes` fields, protobuffers use a pure binary representation, whereas JSON representations use base64."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of entities detected on Document.text. For document shards, entities in this list may cross shard boundaries."]
        pub entities: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentEntity>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityRelations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Relationship among Document.entities."]
        pub entity_relations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentEntityRelation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Any error that occurred while processing this document."]
        pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An IANA published MIME type (also referred to as media type). For more information, see https://www.iana.org/assignments/media-types/media-types.xhtml."]
        pub mime_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Visual page layout for the Document."]
        pub pages: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revisions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Revision history of this document."]
        pub revisions: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentRevision>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shardInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about the sharding if this document is sharded part of a larger document. If the document is not sharded, this message is not specified."]
        pub shard_info:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentShardInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "UTF-8 encoded text in reading order from the document."]
        pub text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textChanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of text corrections made to [Document.text]. This is usually used for annotating corrections to OCR mistakes. Text changes for a given revision may not overlap with each other."]
        pub text_changes: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentTextChange>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textStyles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Styles for the Document.text."]
        pub text_styles: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentStyle>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "translations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of translations on Document.text. For document shards, translations in this list may cross shard boundaries."]
        pub translations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentTranslation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Currently supports Google Cloud Storage URI of the form `gs://bucket_name/object_name`. Object versioning is not supported. See [Google Cloud Storage Request URIs](https://cloud.google.com/storage/docs/reference-uris) for more info."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta1Document {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentBuilder {
            GoogleCloudDocumentaiV1beta1DocumentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A phrase in the text that is a known entity type, such as a person, an organization, or location."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentEntity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Confidence of detected Schema entity. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Canonical id. This will be a unique value in the entity list for this document."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mentionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Use `id` field instead."]
        pub mention_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mentionText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text value in the document e.g. `1600 Amphitheatre Pkwy`."]
        pub mention_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Normalized entity value. Absent if the extracted value could not be converted or the type (e.g. address) is not supported for certain parsers. This field is also only populated for certain supported document types."]
        pub normalized_value: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentEntityNormalizedValue>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageAnchor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Represents the provenance of this entity wrt. the location on the page where it was found."]
        pub page_anchor: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageAnchor>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Entities can be nested to form a hierarchical data structure representing the content in the document."]
        pub properties: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentEntity>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provenance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The history of this annotation."]
        pub provenance: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentProvenance>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "redacted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Whether the entity will be redacted for de-identification purposes."]
        pub redacted: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textAnchor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Provenance of the entity. Text anchor indexing into the Document.text."]
        pub text_anchor: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentTextAnchor>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entity type from a schema e.g. `Address`."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentEntity {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentEntityBuilder {
            GoogleCloudDocumentaiV1beta1DocumentEntityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parsed and normalized entity value."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentEntityNormalizedValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addressValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Postal address. See also: https://github.com/googleapis/googleapis/blob/master/google/type/postal_address.proto"]
        pub address_value: ::std::option::Option<::std::boxed::Box<GoogleTypePostalAddress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dateValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date value. Includes year, month, day. See also: https://github.com/googleapis/googleapis/blob/master/google/type/date.proto"]
        pub date_value: ::std::option::Option<::std::boxed::Box<GoogleTypeDate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datetimeValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DateTime value. Includes date, time, and timezone. See also: https://github.com/googleapis/googleapis/blob/master/google/type/datetime.proto"]
        pub datetime_value: ::std::option::Option<::std::boxed::Box<GoogleTypeDateTime>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "moneyValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Money value. See also: https://github.com/googleapis/googleapis/blob/master/google/type/money.proto"]
        pub money_value: ::std::option::Option<::std::boxed::Box<GoogleTypeMoney>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Normalized entity value stored as a string. This field is populated for supported document type (e.g. Invoice). For some entity types, one of respective 'structured_value' fields may also be populated. - Money/Currency type (`money_value`) is in the ISO 4217 text format. - Date type (`date_value`) is in the ISO 8601 text format. - Datetime type (`datetime_value`) is in the ISO 8601 text format."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentEntityNormalizedValue {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentEntityNormalizedValueBuilder {
            GoogleCloudDocumentaiV1beta1DocumentEntityNormalizedValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Relationship between Entities."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentEntityRelation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object entity id."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Relationship description."]
        pub relation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subjectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Subject entity id."]
        pub subject_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentEntityRelation {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentEntityRelationBuilder {
            GoogleCloudDocumentaiV1beta1DocumentEntityRelationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A page in a Document."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentPage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blocks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of visually detected text blocks on the page. A block has a set of lines (collected into paragraphs) that have a common line-spacing and orientation."]
        pub blocks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageBlock>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected languages together with confidence."]
        pub detected_languages: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageDetectedLanguage>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Physical dimension of the page."]
        pub dimension: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageDimension>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formFields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of visually detected form fields on the page."]
        pub form_fields: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageFormField>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rendered image for this page. This image is preprocessed to remove any skew, rotation, and distortions such that the annotation bounding boxes can be upright and axis-aligned."]
        pub image:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageImage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Layout for the page."]
        pub layout: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageLayout>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lines")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of visually detected text lines on the page. A collection of tokens that a human would perceive as a line."]
        pub lines: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageLine>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "1-based index for current Page in a parent Document. Useful when a page is taken out of a Document for individual processing."]
        pub page_number: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paragraphs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of visually detected text paragraphs on the page. A collection of lines that a human would perceive as a paragraph."]
        pub paragraphs: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageParagraph>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tables")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of visually detected tables on the page."]
        pub tables: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageTable>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tokens")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of visually detected tokens on the page."]
        pub tokens: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageToken>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transforms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Transformation matrices that were applied to the original document image to produce Page.image."]
        pub transforms: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageMatrix>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visualElements")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected non-text visual elements e.g. checkbox, signature etc. on the page."]
        pub visual_elements: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageVisualElement>,
            >,
        >,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentPage {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentPageBuilder {
            GoogleCloudDocumentaiV1beta1DocumentPageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Referencing the visual context of the entity in the Document.pages. Page anchors can be cross-page, consist of multiple bounding polygons and optionally reference specific layout element types."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentPageAnchor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageRefs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "One or more references to visual page elements"]
        pub page_refs: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageAnchorPageRef>,
            >,
        >,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentPageAnchor {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentPageAnchorBuilder {
            GoogleCloudDocumentaiV1beta1DocumentPageAnchorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a weak reference to a page element within a document."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentPageAnchorPageRef {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Identifies the bounding polygon of a layout element on the page."]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDocumentaiV1beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layoutId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Deprecated. Use PageRef.bounding_poly instead."]
        pub layout_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layoutType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The type of the layout element that is being referenced if any."]
        pub layout_type: ::std::option::Option<
            GoogleCloudDocumentaiV1beta1DocumentPageAnchorPageRefLayoutTypeEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "page")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Index into the Document.pages element, for example using Document.pages to locate the related page element."]
        pub page: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentPageAnchorPageRef {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentPageAnchorPageRefBuilder {
            GoogleCloudDocumentaiV1beta1DocumentPageAnchorPageRefBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. The type of the layout element that is being referenced if any."]
    pub enum GoogleCloudDocumentaiV1beta1DocumentPageAnchorPageRefLayoutTypeEnum {
        #[serde(rename = "LAYOUT_TYPE_UNSPECIFIED")]
        #[doc = "Layout Unspecified."]
        LayoutTypeUnspecified,
        #[serde(rename = "BLOCK")]
        #[doc = "References a Page.blocks element."]
        Block,
        #[serde(rename = "PARAGRAPH")]
        #[doc = "References a Page.paragraphs element."]
        Paragraph,
        #[serde(rename = "LINE")]
        #[doc = "References a Page.lines element."]
        Line,
        #[serde(rename = "TOKEN")]
        #[doc = "References a Page.tokens element."]
        Token,
        #[serde(rename = "VISUAL_ELEMENT")]
        #[doc = "References a Page.visual_elements element."]
        VisualElement,
        #[serde(rename = "TABLE")]
        #[doc = "Refrrences a Page.tables element."]
        Table,
        #[serde(rename = "FORM_FIELD")]
        #[doc = "References a Page.form_fields element."]
        FormField,
    }
    impl ::std::default::Default
        for GoogleCloudDocumentaiV1beta1DocumentPageAnchorPageRefLayoutTypeEnum
    {
        fn default() -> Self {
            Self::LayoutTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A block has a set of lines (collected into paragraphs) that have a common line-spacing and orientation."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentPageBlock {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected languages together with confidence."]
        pub detected_languages: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageDetectedLanguage>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Layout for Block."]
        pub layout: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageLayout>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provenance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The history of this annotation."]
        pub provenance: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentProvenance>,
        >,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentPageBlock {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentPageBlockBuilder {
            GoogleCloudDocumentaiV1beta1DocumentPageBlockBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected language for a structural component."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentPageDetectedLanguage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of detected language. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        pub language_code: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentPageDetectedLanguage {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentPageDetectedLanguageBuilder {
            GoogleCloudDocumentaiV1beta1DocumentPageDetectedLanguageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Dimension for the page."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentPageDimension {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page height."]
        pub height: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Dimension unit."]
        pub unit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page width."]
        pub width: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentPageDimension {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentPageDimensionBuilder {
            GoogleCloudDocumentaiV1beta1DocumentPageDimensionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A form field detected on the page."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentPageFormField {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Layout for the FormField name. e.g. `Address`, `Email`, `Grand total`, `Phone number`, etc."]
        pub field_name: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageLayout>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Layout for the FormField value."]
        pub field_value: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageLayout>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nameDetectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected languages for name together with confidence."]
        pub name_detected_languages: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageDetectedLanguage>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueDetectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected languages for value together with confidence."]
        pub value_detected_languages: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageDetectedLanguage>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the value is non-textual, this field represents the type. Current valid values are: - blank (this indicates the field_value is normal text) - \"unfilled_checkbox\" - \"filled_checkbox\""]
        pub value_type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentPageFormField {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentPageFormFieldBuilder {
            GoogleCloudDocumentaiV1beta1DocumentPageFormFieldBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Rendered image contents for this page."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentPageImage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Raw byte content of the image."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Height of the image in pixels."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Encoding mime type for the image."]
        pub mime_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Width of the image in pixels."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentPageImage {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentPageImageBuilder {
            GoogleCloudDocumentaiV1beta1DocumentPageImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Visual element describing a layout unit on a page."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentPageLayout {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon for the Layout."]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDocumentaiV1beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the current Layout within context of the object this layout is for. e.g. confidence can be for a single token, a table, a visual element, etc. depending on context. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orientation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected orientation for the Layout."]
        pub orientation:
            ::std::option::Option<GoogleCloudDocumentaiV1beta1DocumentPageLayoutOrientationEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textAnchor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text anchor indexing into the Document.text."]
        pub text_anchor: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentTextAnchor>,
        >,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentPageLayout {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentPageLayoutBuilder {
            GoogleCloudDocumentaiV1beta1DocumentPageLayoutBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Detected orientation for the Layout."]
    pub enum GoogleCloudDocumentaiV1beta1DocumentPageLayoutOrientationEnum {
        #[serde(rename = "ORIENTATION_UNSPECIFIED")]
        #[doc = "Unspecified orientation."]
        OrientationUnspecified,
        #[serde(rename = "PAGE_UP")]
        #[doc = "Orientation is aligned with page up."]
        PageUp,
        #[serde(rename = "PAGE_RIGHT")]
        #[doc = "Orientation is aligned with page right. Turn the head 90 degrees clockwise from upright to read."]
        PageRight,
        #[serde(rename = "PAGE_DOWN")]
        #[doc = "Orientation is aligned with page down. Turn the head 180 degrees from upright to read."]
        PageDown,
        #[serde(rename = "PAGE_LEFT")]
        #[doc = "Orientation is aligned with page left. Turn the head 90 degrees counterclockwise from upright to read."]
        PageLeft,
    }
    impl ::std::default::Default for GoogleCloudDocumentaiV1beta1DocumentPageLayoutOrientationEnum {
        fn default() -> Self {
            Self::OrientationUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A collection of tokens that a human would perceive as a line. Does not cross column boundaries, can be horizontal, vertical, etc."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentPageLine {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected languages together with confidence."]
        pub detected_languages: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageDetectedLanguage>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Layout for Line."]
        pub layout: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageLayout>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provenance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The history of this annotation."]
        pub provenance: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentProvenance>,
        >,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentPageLine {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentPageLineBuilder {
            GoogleCloudDocumentaiV1beta1DocumentPageLineBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Representation for transformation matrix, intended to be compatible and used with OpenCV format for image manipulation."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentPageMatrix {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cols")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of columns in the matrix."]
        pub cols: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The matrix data."]
        pub data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of rows in the matrix."]
        pub rows: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This encodes information about what data type the matrix uses. For example, 0 (CV_8U) is an unsigned 8-bit image. For the full list of OpenCV primitive data types, please refer to https://docs.opencv.org/4.3.0/d1/d1b/group__core__hal__interface.html"]
        pub _type: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentPageMatrix {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentPageMatrixBuilder {
            GoogleCloudDocumentaiV1beta1DocumentPageMatrixBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A collection of lines that a human would perceive as a paragraph."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentPageParagraph {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected languages together with confidence."]
        pub detected_languages: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageDetectedLanguage>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Layout for Paragraph."]
        pub layout: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageLayout>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provenance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The history of this annotation."]
        pub provenance: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentProvenance>,
        >,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentPageParagraph {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentPageParagraphBuilder {
            GoogleCloudDocumentaiV1beta1DocumentPageParagraphBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A table representation similar to HTML table structure."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentPageTable {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bodyRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Body rows of the table."]
        pub body_rows: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageTableTableRow>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected languages together with confidence."]
        pub detected_languages: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageDetectedLanguage>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headerRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Header rows of the table."]
        pub header_rows: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageTableTableRow>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Layout for Table."]
        pub layout: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageLayout>,
        >,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentPageTable {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentPageTableBuilder {
            GoogleCloudDocumentaiV1beta1DocumentPageTableBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A cell representation inside the table."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentPageTableTableCell {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colSpan")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How many columns this cell spans."]
        pub col_span: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected languages together with confidence."]
        pub detected_languages: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageDetectedLanguage>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Layout for TableCell."]
        pub layout: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageLayout>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowSpan")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How many rows this cell spans."]
        pub row_span: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentPageTableTableCell {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentPageTableTableCellBuilder {
            GoogleCloudDocumentaiV1beta1DocumentPageTableTableCellBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A row of table cells."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentPageTableTableRow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cells")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cells that make up this row."]
        pub cells: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageTableTableCell>,
            >,
        >,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentPageTableTableRow {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentPageTableTableRowBuilder {
            GoogleCloudDocumentaiV1beta1DocumentPageTableTableRowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A detected token."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentPageToken {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedBreak")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected break at the end of a Token."]
        pub detected_break: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageTokenDetectedBreak>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected languages together with confidence."]
        pub detected_languages: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageDetectedLanguage>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Layout for Token."]
        pub layout: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageLayout>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provenance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The history of this annotation."]
        pub provenance: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentProvenance>,
        >,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentPageToken {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentPageTokenBuilder {
            GoogleCloudDocumentaiV1beta1DocumentPageTokenBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected break at the end of a Token."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentPageTokenDetectedBreak {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected break type."]
        pub _type: ::std::option::Option<
            GoogleCloudDocumentaiV1beta1DocumentPageTokenDetectedBreakTypeEnum,
        >,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentPageTokenDetectedBreak {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentPageTokenDetectedBreakBuilder {
            GoogleCloudDocumentaiV1beta1DocumentPageTokenDetectedBreakBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Detected break type."]
    pub enum GoogleCloudDocumentaiV1beta1DocumentPageTokenDetectedBreakTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "Unspecified break type."]
        TypeUnspecified,
        #[serde(rename = "SPACE")]
        #[doc = "A single whitespace."]
        Space,
        #[serde(rename = "WIDE_SPACE")]
        #[doc = "A wider whitespace."]
        WideSpace,
        #[serde(rename = "HYPHEN")]
        #[doc = "A hyphen that indicates that a token has been split across lines."]
        Hyphen,
    }
    impl ::std::default::Default
        for GoogleCloudDocumentaiV1beta1DocumentPageTokenDetectedBreakTypeEnum
    {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected non-text visual elements e.g. checkbox, signature etc. on the page."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentPageVisualElement {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected languages together with confidence."]
        pub detected_languages: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageDetectedLanguage>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Layout for VisualElement."]
        pub layout: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentPageLayout>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of the VisualElement."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentPageVisualElement {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentPageVisualElementBuilder {
            GoogleCloudDocumentaiV1beta1DocumentPageVisualElementBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Structure to identify provenance relationships between annotations in different revisions."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentProvenance {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Id of this operation. Needs to be unique within the scope of the revision."]
        pub id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "References to the original elements that are replaced."]
        pub parents: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentProvenanceParent>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index of the revision that produced this element."]
        pub revision: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of provenance operation."]
        pub _type: ::std::option::Option<GoogleCloudDocumentaiV1beta1DocumentProvenanceTypeEnum>,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentProvenance {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentProvenanceBuilder {
            GoogleCloudDocumentaiV1beta1DocumentProvenanceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of provenance operation."]
    pub enum GoogleCloudDocumentaiV1beta1DocumentProvenanceTypeEnum {
        #[serde(rename = "OPERATION_TYPE_UNSPECIFIED")]
        #[doc = "Operation type unspecified."]
        OperationTypeUnspecified,
        #[serde(rename = "ADD")]
        #[doc = "Add an element. Implicit if no `parents` are set for the provenance."]
        Add,
        #[serde(rename = "REMOVE")]
        #[doc = "The element is removed. No `parents` should be set."]
        Remove,
        #[serde(rename = "REPLACE")]
        #[doc = "Explicitly replaces the element(s) identified by `parents`."]
        Replace,
        #[serde(rename = "EVAL_REQUESTED")]
        #[doc = "Element is requested for human review."]
        EvalRequested,
        #[serde(rename = "EVAL_APPROVED")]
        #[doc = "Element is review and approved at human review, confidence will be set to 1.0"]
        EvalApproved,
    }
    impl ::std::default::Default for GoogleCloudDocumentaiV1beta1DocumentProvenanceTypeEnum {
        fn default() -> Self {
            Self::OperationTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Structure for referencing parent provenances. When an element replaces one of more other elements parent references identify the elements that are replaced."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentProvenanceParent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The id of the parent provenance."]
        pub id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index of the [Document.revisions] identifying the parent revision."]
        pub revision: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentProvenanceParent {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentProvenanceParentBuilder {
            GoogleCloudDocumentaiV1beta1DocumentProvenanceParentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains past or forward revisions of this document."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentRevision {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "agent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the change was made by a person specify the name or id of that person."]
        pub agent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time that the revision was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "humanReview")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human Review information of this revision."]
        pub human_review: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentRevisionHumanReview>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Id of the revision. Unique within the context of the document."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The revisions that this revision is based on. This can include one or more parent (when documents are merged.) This field represents the index into the `revisions` field."]
        pub parent: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "processor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the annotation was made by processor identify the processor by its resource name."]
        pub processor: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentRevision {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentRevisionBuilder {
            GoogleCloudDocumentaiV1beta1DocumentRevisionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Human Review information of the document."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentRevisionHumanReview {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human review state. e.g. `requested`, `succeeded`, `rejected`."]
        pub state: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A message providing more details about the current state of processing. For example, the rejection reason when the state is `rejected`."]
        pub state_message: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentRevisionHumanReview {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentRevisionHumanReviewBuilder {
            GoogleCloudDocumentaiV1beta1DocumentRevisionHumanReviewBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "For a large document, sharding may be performed to produce several document shards. Each document shard contains this field to detail which shard it is."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentShardInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shardCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of shards."]
        pub shard_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shardIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The 0-based index of this shard."]
        pub shard_index: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index of the first character in Document.text in the overall document global text."]
        pub text_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentShardInfo {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentShardInfoBuilder {
            GoogleCloudDocumentaiV1beta1DocumentShardInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotation for common text style attributes. This adheres to CSS conventions as much as possible."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentStyle {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backgroundColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text background color."]
        pub background_color: ::std::option::Option<::std::boxed::Box<GoogleTypeColor>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "color")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text color."]
        pub color: ::std::option::Option<::std::boxed::Box<GoogleTypeColor>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fontSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Font size."]
        pub font_size: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentStyleFontSize>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fontWeight")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Font weight. Possible values are normal, bold, bolder, and lighter. https://www.w3schools.com/cssref/pr_font_weight.asp"]
        pub font_weight: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textAnchor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text anchor indexing into the Document.text."]
        pub text_anchor: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentTextAnchor>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textDecoration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text decoration. Follows CSS standard. https://www.w3schools.com/cssref/pr_text_text-decoration.asp"]
        pub text_decoration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text style. Possible values are normal, italic, and oblique. https://www.w3schools.com/cssref/pr_font_font-style.asp"]
        pub text_style: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentStyle {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentStyleBuilder {
            GoogleCloudDocumentaiV1beta1DocumentStyleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Font size with unit."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentStyleFontSize {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Font size for the text."]
        pub size: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unit for the font size. Follows CSS naming (in, px, pt, etc.)."]
        pub unit: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentStyleFontSize {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentStyleFontSizeBuilder {
            GoogleCloudDocumentaiV1beta1DocumentStyleFontSizeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Text reference indexing into the Document.text."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentTextAnchor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains the content of the text span so that users do not have to look it up in the text_segments."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textSegments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text segments from the Document.text."]
        pub text_segments: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentTextAnchorTextSegment>,
            >,
        >,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentTextAnchor {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentTextAnchorBuilder {
            GoogleCloudDocumentaiV1beta1DocumentTextAnchorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A text segment in the Document.text. The indices may be out of bounds which indicate that the text extends into another document shard for large sharded documents. See ShardInfo.text_offset"]
    pub struct GoogleCloudDocumentaiV1beta1DocumentTextAnchorTextSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "TextSegment half open end UTF-8 char index in the Document.text."]
        pub end_index: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "TextSegment start UTF-8 char index in the Document.text."]
        pub start_index: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentTextAnchorTextSegment {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentTextAnchorTextSegmentBuilder {
            GoogleCloudDocumentaiV1beta1DocumentTextAnchorTextSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This message is used for text changes aka. OCR corrections."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentTextChange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "changedText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text that replaces the text identified in the `text_anchor`."]
        pub changed_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provenance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The history of this annotation."]
        pub provenance: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentProvenance>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textAnchor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Provenance of the correction. Text anchor indexing into the Document.text. There can only be a single `TextAnchor.text_segments` element. If the start and end index of the text segment are the same, the text change is inserted before that index."]
        pub text_anchor: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentTextAnchor>,
        >,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentTextChange {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentTextChangeBuilder {
            GoogleCloudDocumentaiV1beta1DocumentTextChangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A translation of the text segment."]
    pub struct GoogleCloudDocumentaiV1beta1DocumentTranslation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provenance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The history of this annotation."]
        pub provenance: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentProvenance>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textAnchor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Provenance of the translation. Text anchor indexing into the Document.text. There can only be a single `TextAnchor.text_segments` element. If the start and end index of the text segment are the same, the text change is inserted before that index."]
        pub text_anchor: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta1DocumentTextAnchor>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "translatedText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text translated into the target language."]
        pub translated_text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta1DocumentTranslation {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1DocumentTranslationBuilder {
            GoogleCloudDocumentaiV1beta1DocumentTranslationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Google Cloud Storage location where the output file will be written to."]
    pub struct GoogleCloudDocumentaiV1beta1GcsDestination {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta1GcsDestination {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1GcsDestinationBuilder {
            GoogleCloudDocumentaiV1beta1GcsDestinationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Google Cloud Storage location where the input file will be read from."]
    pub struct GoogleCloudDocumentaiV1beta1GcsSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta1GcsSource {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1GcsSourceBuilder {
            GoogleCloudDocumentaiV1beta1GcsSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The desired input location and metadata."]
    pub struct GoogleCloudDocumentaiV1beta1InputConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Google Cloud Storage location to read the input from. This must be a single file."]
        pub gcs_source:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDocumentaiV1beta1GcsSource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Mimetype of the input. Current supported mimetypes are application/pdf, image/tiff, and image/gif. In addition, application/json type is supported for requests with ProcessDocumentRequest.automl_params field set. The JSON file needs to be in Document format."]
        pub mime_type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta1InputConfig {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1InputConfigBuilder {
            GoogleCloudDocumentaiV1beta1InputConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A vertex represents a 2D point in the image. NOTE: the normalized vertex coordinates are relative to the original image and range from 0 to 1."]
    pub struct GoogleCloudDocumentaiV1beta1NormalizedVertex {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "x")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "X coordinate."]
        pub x: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "y")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Y coordinate."]
        pub y: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudDocumentaiV1beta1NormalizedVertex {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1NormalizedVertexBuilder {
            GoogleCloudDocumentaiV1beta1NormalizedVertexBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains metadata for the BatchProcessDocuments operation."]
    pub struct GoogleCloudDocumentaiV1beta1OperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creation time of the operation."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state of the current batch processing."]
        pub state: ::std::option::Option<GoogleCloudDocumentaiV1beta1OperationMetadataStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A message providing more details about the current state of processing."]
        pub state_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last update time of the operation."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta1OperationMetadata {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1OperationMetadataBuilder {
            GoogleCloudDocumentaiV1beta1OperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The state of the current batch processing."]
    pub enum GoogleCloudDocumentaiV1beta1OperationMetadataStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "The default value. This value is used if the state is omitted."]
        StateUnspecified,
        #[serde(rename = "ACCEPTED")]
        #[doc = "Request is received."]
        Accepted,
        #[serde(rename = "WAITING")]
        #[doc = "Request operation is waiting for scheduling."]
        Waiting,
        #[serde(rename = "RUNNING")]
        #[doc = "Request is being processed."]
        Running,
        #[serde(rename = "SUCCEEDED")]
        #[doc = "The batch processing completed successfully."]
        Succeeded,
        #[serde(rename = "CANCELLED")]
        #[doc = "The batch processing was cancelled."]
        Cancelled,
        #[serde(rename = "FAILED")]
        #[doc = "The batch processing has failed."]
        Failed,
    }
    impl ::std::default::Default for GoogleCloudDocumentaiV1beta1OperationMetadataStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The desired output location and metadata."]
    pub struct GoogleCloudDocumentaiV1beta1OutputConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsDestination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Google Cloud Storage location to write the output to."]
        pub gcs_destination:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDocumentaiV1beta1GcsDestination>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pagesPerShard")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The max number of pages to include into each output Document shard JSON on Google Cloud Storage. The valid range is [1, 100]. If not specified, the default value is 20. For example, for one pdf file with 100 pages, 100 parsed pages will be produced. If `pages_per_shard` = 20, then 5 Document shard JSON files each containing 20 parsed pages will be written under the prefix OutputConfig.gcs_destination.uri and suffix pages-x-to-y.json where x and y are 1-indexed page numbers. Example GCS outputs with 157 pages and pages_per_shard = 50: pages-001-to-050.json pages-051-to-100.json pages-101-to-150.json pages-151-to-157.json"]
        pub pages_per_shard: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDocumentaiV1beta1OutputConfig {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1OutputConfigBuilder {
            GoogleCloudDocumentaiV1beta1OutputConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to a single document processing request."]
    pub struct GoogleCloudDocumentaiV1beta1ProcessDocumentResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about the input file. This is the same as the corresponding input config in the request."]
        pub input_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDocumentaiV1beta1InputConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The output location of the parsed responses. The responses are written to this location as JSON-serialized `Document` objects."]
        pub output_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDocumentaiV1beta1OutputConfig>>,
    }
    impl GoogleCloudDocumentaiV1beta1ProcessDocumentResponse {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1ProcessDocumentResponseBuilder {
            GoogleCloudDocumentaiV1beta1ProcessDocumentResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A vertex represents a 2D point in the image. NOTE: the vertex coordinates are in the same scale as the original image."]
    pub struct GoogleCloudDocumentaiV1beta1Vertex {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "x")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "X coordinate."]
        pub x: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "y")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Y coordinate."]
        pub y: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDocumentaiV1beta1Vertex {
        pub fn builder() -> GoogleCloudDocumentaiV1beta1VertexBuilder {
            GoogleCloudDocumentaiV1beta1VertexBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parameters to control AutoML model prediction behavior."]
    pub struct GoogleCloudDocumentaiV1beta2AutoMlParams {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "model")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name of the AutoML model. Format: `projects/{project-id}/locations/{location-id}/models/{model-id}`."]
        pub model: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta2AutoMlParams {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2AutoMlParamsBuilder {
            GoogleCloudDocumentaiV1beta2AutoMlParamsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to batch process documents as an asynchronous operation. The output is written to Cloud Storage as JSON in the [Document] format."]
    pub struct GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requests")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Individual requests for each document."]
        pub requests: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta2ProcessDocumentRequest>>,
        >,
    }
    impl GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequest {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequestBuilder {
            GoogleCloudDocumentaiV1beta2BatchProcessDocumentsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to an batch document processing request. This is returned in the LRO Operation after the operation is complete."]
    pub struct GoogleCloudDocumentaiV1beta2BatchProcessDocumentsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Responses for each individual document."]
        pub responses: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta2ProcessDocumentResponse>>,
        >,
    }
    impl GoogleCloudDocumentaiV1beta2BatchProcessDocumentsResponse {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2BatchProcessDocumentsResponseBuilder {
            GoogleCloudDocumentaiV1beta2BatchProcessDocumentsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A bounding polygon for the detected image annotation."]
    pub struct GoogleCloudDocumentaiV1beta2BoundingPoly {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedVertices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon normalized vertices."]
        pub normalized_vertices: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta2NormalizedVertex>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vertices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon vertices."]
        pub vertices: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta2Vertex>>,
        >,
    }
    impl GoogleCloudDocumentaiV1beta2BoundingPoly {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2BoundingPolyBuilder {
            GoogleCloudDocumentaiV1beta2BoundingPolyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Document represents the canonical document resource in Document Understanding AI. It is an interchange format that provides insights into documents and allows for collaboration between users and Document Understanding AI to iterate and optimize for quality."]
    pub struct GoogleCloudDocumentaiV1beta2Document {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Inline document content, represented as a stream of bytes. Note: As with all `bytes` fields, protobuffers use a pure binary representation, whereas JSON representations use base64."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of entities detected on Document.text. For document shards, entities in this list may cross shard boundaries."]
        pub entities: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentEntity>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityRelations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Relationship among Document.entities."]
        pub entity_relations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentEntityRelation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Any error that occurred while processing this document."]
        pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Labels for this document."]
        pub labels: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentLabel>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An IANA published MIME type (also referred to as media type). For more information, see https://www.iana.org/assignments/media-types/media-types.xhtml."]
        pub mime_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Visual page layout for the Document."]
        pub pages: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revisions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Revision history of this document."]
        pub revisions: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentRevision>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shardInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about the sharding if this document is sharded part of a larger document. If the document is not sharded, this message is not specified."]
        pub shard_info:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentShardInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "UTF-8 encoded text in reading order from the document."]
        pub text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textChanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of text corrections made to [Document.text]. This is usually used for annotating corrections to OCR mistakes. Text changes for a given revision may not overlap with each other."]
        pub text_changes: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentTextChange>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textStyles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Styles for the Document.text."]
        pub text_styles: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentStyle>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "translations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of translations on Document.text. For document shards, translations in this list may cross shard boundaries."]
        pub translations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentTranslation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Currently supports Google Cloud Storage URI of the form `gs://bucket_name/object_name`. Object versioning is not supported. See [Google Cloud Storage Request URIs](https://cloud.google.com/storage/docs/reference-uris) for more info."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta2Document {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentBuilder {
            GoogleCloudDocumentaiV1beta2DocumentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A phrase in the text that is a known entity type, such as a person, an organization, or location."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentEntity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Confidence of detected Schema entity. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Canonical id. This will be a unique value in the entity list for this document."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mentionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Use `id` field instead."]
        pub mention_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mentionText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text value in the document e.g. `1600 Amphitheatre Pkwy`."]
        pub mention_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Normalized entity value. Absent if the extracted value could not be converted or the type (e.g. address) is not supported for certain parsers. This field is also only populated for certain supported document types."]
        pub normalized_value: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentEntityNormalizedValue>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageAnchor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Represents the provenance of this entity wrt. the location on the page where it was found."]
        pub page_anchor: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageAnchor>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Entities can be nested to form a hierarchical data structure representing the content in the document."]
        pub properties: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentEntity>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provenance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The history of this annotation."]
        pub provenance: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentProvenance>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "redacted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Whether the entity will be redacted for de-identification purposes."]
        pub redacted: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textAnchor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Provenance of the entity. Text anchor indexing into the Document.text."]
        pub text_anchor: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentTextAnchor>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entity type from a schema e.g. `Address`."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentEntity {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentEntityBuilder {
            GoogleCloudDocumentaiV1beta2DocumentEntityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parsed and normalized entity value."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentEntityNormalizedValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addressValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Postal address. See also: https://github.com/googleapis/googleapis/blob/master/google/type/postal_address.proto"]
        pub address_value: ::std::option::Option<::std::boxed::Box<GoogleTypePostalAddress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dateValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date value. Includes year, month, day. See also: https://github.com/googleapis/googleapis/blob/master/google/type/date.proto"]
        pub date_value: ::std::option::Option<::std::boxed::Box<GoogleTypeDate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datetimeValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DateTime value. Includes date, time, and timezone. See also: https://github.com/googleapis/googleapis/blob/master/google/type/datetime.proto"]
        pub datetime_value: ::std::option::Option<::std::boxed::Box<GoogleTypeDateTime>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "moneyValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Money value. See also: https://github.com/googleapis/googleapis/blob/master/google/type/money.proto"]
        pub money_value: ::std::option::Option<::std::boxed::Box<GoogleTypeMoney>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Normalized entity value stored as a string. This field is populated for supported document type (e.g. Invoice). For some entity types, one of respective 'structured_value' fields may also be populated. - Money/Currency type (`money_value`) is in the ISO 4217 text format. - Date type (`date_value`) is in the ISO 8601 text format. - Datetime type (`datetime_value`) is in the ISO 8601 text format."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentEntityNormalizedValue {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentEntityNormalizedValueBuilder {
            GoogleCloudDocumentaiV1beta2DocumentEntityNormalizedValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Relationship between Entities."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentEntityRelation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object entity id."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Relationship description."]
        pub relation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subjectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Subject entity id."]
        pub subject_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentEntityRelation {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentEntityRelationBuilder {
            GoogleCloudDocumentaiV1beta2DocumentEntityRelationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Label attaches schema information and/or other metadata to segments within a Document. Multiple Labels on a single field can denote either different labels, different instances of the same label created at different times, or some combination of both."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentLabel {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "automlModel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label is generated AutoML model. This field stores the full resource name of the AutoML model. Format: `projects/{project-id}/locations/{location-id}/models/{model-id}`"]
        pub automl_model: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence score between 0 and 1 for label assignment."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the label. When the label is generated from AutoML Text Classification model, this field represents the name of the category."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentLabel {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentLabelBuilder {
            GoogleCloudDocumentaiV1beta2DocumentLabelBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A page in a Document."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentPage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blocks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of visually detected text blocks on the page. A block has a set of lines (collected into paragraphs) that have a common line-spacing and orientation."]
        pub blocks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageBlock>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected languages together with confidence."]
        pub detected_languages: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Physical dimension of the page."]
        pub dimension: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageDimension>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formFields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of visually detected form fields on the page."]
        pub form_fields: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageFormField>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rendered image for this page. This image is preprocessed to remove any skew, rotation, and distortions such that the annotation bounding boxes can be upright and axis-aligned."]
        pub image:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageImage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Layout for the page."]
        pub layout: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lines")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of visually detected text lines on the page. A collection of tokens that a human would perceive as a line."]
        pub lines: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageLine>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "1-based index for current Page in a parent Document. Useful when a page is taken out of a Document for individual processing."]
        pub page_number: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paragraphs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of visually detected text paragraphs on the page. A collection of lines that a human would perceive as a paragraph."]
        pub paragraphs: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageParagraph>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tables")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of visually detected tables on the page."]
        pub tables: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageTable>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tokens")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of visually detected tokens on the page."]
        pub tokens: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageToken>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transforms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Transformation matrices that were applied to the original document image to produce Page.image."]
        pub transforms: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageMatrix>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visualElements")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected non-text visual elements e.g. checkbox, signature etc. on the page."]
        pub visual_elements: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageVisualElement>,
            >,
        >,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentPage {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentPageBuilder {
            GoogleCloudDocumentaiV1beta2DocumentPageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Referencing the visual context of the entity in the Document.pages. Page anchors can be cross-page, consist of multiple bounding polygons and optionally reference specific layout element types."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentPageAnchor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageRefs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "One or more references to visual page elements"]
        pub page_refs: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRef>,
            >,
        >,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentPageAnchor {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentPageAnchorBuilder {
            GoogleCloudDocumentaiV1beta2DocumentPageAnchorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a weak reference to a page element within a document."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRef {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Identifies the bounding polygon of a layout element on the page."]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDocumentaiV1beta2BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layoutId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Deprecated. Use PageRef.bounding_poly instead."]
        pub layout_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layoutType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The type of the layout element that is being referenced if any."]
        pub layout_type: ::std::option::Option<
            GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRefLayoutTypeEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "page")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Index into the Document.pages element, for example using Document.pages to locate the related page element."]
        pub page: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRef {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRefBuilder {
            GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRefBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. The type of the layout element that is being referenced if any."]
    pub enum GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRefLayoutTypeEnum {
        #[serde(rename = "LAYOUT_TYPE_UNSPECIFIED")]
        #[doc = "Layout Unspecified."]
        LayoutTypeUnspecified,
        #[serde(rename = "BLOCK")]
        #[doc = "References a Page.blocks element."]
        Block,
        #[serde(rename = "PARAGRAPH")]
        #[doc = "References a Page.paragraphs element."]
        Paragraph,
        #[serde(rename = "LINE")]
        #[doc = "References a Page.lines element."]
        Line,
        #[serde(rename = "TOKEN")]
        #[doc = "References a Page.tokens element."]
        Token,
        #[serde(rename = "VISUAL_ELEMENT")]
        #[doc = "References a Page.visual_elements element."]
        VisualElement,
        #[serde(rename = "TABLE")]
        #[doc = "Refrrences a Page.tables element."]
        Table,
        #[serde(rename = "FORM_FIELD")]
        #[doc = "References a Page.form_fields element."]
        FormField,
    }
    impl ::std::default::Default
        for GoogleCloudDocumentaiV1beta2DocumentPageAnchorPageRefLayoutTypeEnum
    {
        fn default() -> Self {
            Self::LayoutTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A block has a set of lines (collected into paragraphs) that have a common line-spacing and orientation."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentPageBlock {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected languages together with confidence."]
        pub detected_languages: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Layout for Block."]
        pub layout: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provenance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The history of this annotation."]
        pub provenance: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentProvenance>,
        >,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentPageBlock {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentPageBlockBuilder {
            GoogleCloudDocumentaiV1beta2DocumentPageBlockBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected language for a structural component."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of detected language. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        pub language_code: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguageBuilder {
            GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Dimension for the page."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentPageDimension {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page height."]
        pub height: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Dimension unit."]
        pub unit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page width."]
        pub width: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentPageDimension {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentPageDimensionBuilder {
            GoogleCloudDocumentaiV1beta2DocumentPageDimensionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A form field detected on the page."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentPageFormField {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Layout for the FormField name. e.g. `Address`, `Email`, `Grand total`, `Phone number`, etc."]
        pub field_name: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Layout for the FormField value."]
        pub field_value: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nameDetectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected languages for name together with confidence."]
        pub name_detected_languages: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueDetectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected languages for value together with confidence."]
        pub value_detected_languages: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the value is non-textual, this field represents the type. Current valid values are: - blank (this indicates the field_value is normal text) - \"unfilled_checkbox\" - \"filled_checkbox\""]
        pub value_type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentPageFormField {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentPageFormFieldBuilder {
            GoogleCloudDocumentaiV1beta2DocumentPageFormFieldBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Rendered image contents for this page."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentPageImage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Raw byte content of the image."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Height of the image in pixels."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Encoding mime type for the image."]
        pub mime_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Width of the image in pixels."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentPageImage {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentPageImageBuilder {
            GoogleCloudDocumentaiV1beta2DocumentPageImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Visual element describing a layout unit on a page."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentPageLayout {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon for the Layout."]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDocumentaiV1beta2BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the current Layout within context of the object this layout is for. e.g. confidence can be for a single token, a table, a visual element, etc. depending on context. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orientation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected orientation for the Layout."]
        pub orientation:
            ::std::option::Option<GoogleCloudDocumentaiV1beta2DocumentPageLayoutOrientationEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textAnchor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text anchor indexing into the Document.text."]
        pub text_anchor: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentTextAnchor>,
        >,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentPageLayout {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentPageLayoutBuilder {
            GoogleCloudDocumentaiV1beta2DocumentPageLayoutBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Detected orientation for the Layout."]
    pub enum GoogleCloudDocumentaiV1beta2DocumentPageLayoutOrientationEnum {
        #[serde(rename = "ORIENTATION_UNSPECIFIED")]
        #[doc = "Unspecified orientation."]
        OrientationUnspecified,
        #[serde(rename = "PAGE_UP")]
        #[doc = "Orientation is aligned with page up."]
        PageUp,
        #[serde(rename = "PAGE_RIGHT")]
        #[doc = "Orientation is aligned with page right. Turn the head 90 degrees clockwise from upright to read."]
        PageRight,
        #[serde(rename = "PAGE_DOWN")]
        #[doc = "Orientation is aligned with page down. Turn the head 180 degrees from upright to read."]
        PageDown,
        #[serde(rename = "PAGE_LEFT")]
        #[doc = "Orientation is aligned with page left. Turn the head 90 degrees counterclockwise from upright to read."]
        PageLeft,
    }
    impl ::std::default::Default for GoogleCloudDocumentaiV1beta2DocumentPageLayoutOrientationEnum {
        fn default() -> Self {
            Self::OrientationUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A collection of tokens that a human would perceive as a line. Does not cross column boundaries, can be horizontal, vertical, etc."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentPageLine {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected languages together with confidence."]
        pub detected_languages: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Layout for Line."]
        pub layout: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provenance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The history of this annotation."]
        pub provenance: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentProvenance>,
        >,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentPageLine {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentPageLineBuilder {
            GoogleCloudDocumentaiV1beta2DocumentPageLineBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Representation for transformation matrix, intended to be compatible and used with OpenCV format for image manipulation."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentPageMatrix {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cols")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of columns in the matrix."]
        pub cols: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The matrix data."]
        pub data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of rows in the matrix."]
        pub rows: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This encodes information about what data type the matrix uses. For example, 0 (CV_8U) is an unsigned 8-bit image. For the full list of OpenCV primitive data types, please refer to https://docs.opencv.org/4.3.0/d1/d1b/group__core__hal__interface.html"]
        pub _type: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentPageMatrix {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentPageMatrixBuilder {
            GoogleCloudDocumentaiV1beta2DocumentPageMatrixBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A collection of lines that a human would perceive as a paragraph."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentPageParagraph {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected languages together with confidence."]
        pub detected_languages: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Layout for Paragraph."]
        pub layout: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provenance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The history of this annotation."]
        pub provenance: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentProvenance>,
        >,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentPageParagraph {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentPageParagraphBuilder {
            GoogleCloudDocumentaiV1beta2DocumentPageParagraphBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A table representation similar to HTML table structure."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentPageTable {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bodyRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Body rows of the table."]
        pub body_rows: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageTableTableRow>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected languages together with confidence."]
        pub detected_languages: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headerRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Header rows of the table."]
        pub header_rows: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageTableTableRow>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Layout for Table."]
        pub layout: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
        >,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentPageTable {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentPageTableBuilder {
            GoogleCloudDocumentaiV1beta2DocumentPageTableBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A cell representation inside the table."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentPageTableTableCell {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colSpan")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How many columns this cell spans."]
        pub col_span: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected languages together with confidence."]
        pub detected_languages: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Layout for TableCell."]
        pub layout: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowSpan")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How many rows this cell spans."]
        pub row_span: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentPageTableTableCell {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentPageTableTableCellBuilder {
            GoogleCloudDocumentaiV1beta2DocumentPageTableTableCellBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A row of table cells."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentPageTableTableRow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cells")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cells that make up this row."]
        pub cells: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageTableTableCell>,
            >,
        >,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentPageTableTableRow {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentPageTableTableRowBuilder {
            GoogleCloudDocumentaiV1beta2DocumentPageTableTableRowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A detected token."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentPageToken {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedBreak")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected break at the end of a Token."]
        pub detected_break: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreak>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected languages together with confidence."]
        pub detected_languages: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Layout for Token."]
        pub layout: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provenance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The history of this annotation."]
        pub provenance: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentProvenance>,
        >,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentPageToken {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentPageTokenBuilder {
            GoogleCloudDocumentaiV1beta2DocumentPageTokenBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected break at the end of a Token."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreak {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected break type."]
        pub _type: ::std::option::Option<
            GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreakTypeEnum,
        >,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreak {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreakBuilder {
            GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreakBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Detected break type."]
    pub enum GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreakTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "Unspecified break type."]
        TypeUnspecified,
        #[serde(rename = "SPACE")]
        #[doc = "A single whitespace."]
        Space,
        #[serde(rename = "WIDE_SPACE")]
        #[doc = "A wider whitespace."]
        WideSpace,
        #[serde(rename = "HYPHEN")]
        #[doc = "A hyphen that indicates that a token has been split across lines."]
        Hyphen,
    }
    impl ::std::default::Default
        for GoogleCloudDocumentaiV1beta2DocumentPageTokenDetectedBreakTypeEnum
    {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected non-text visual elements e.g. checkbox, signature etc. on the page."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentPageVisualElement {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected languages together with confidence."]
        pub detected_languages: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageDetectedLanguage>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Layout for VisualElement."]
        pub layout: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentPageLayout>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of the VisualElement."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentPageVisualElement {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentPageVisualElementBuilder {
            GoogleCloudDocumentaiV1beta2DocumentPageVisualElementBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Structure to identify provenance relationships between annotations in different revisions."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentProvenance {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Id of this operation. Needs to be unique within the scope of the revision."]
        pub id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "References to the original elements that are replaced."]
        pub parents: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentProvenanceParent>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index of the revision that produced this element."]
        pub revision: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of provenance operation."]
        pub _type: ::std::option::Option<GoogleCloudDocumentaiV1beta2DocumentProvenanceTypeEnum>,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentProvenance {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentProvenanceBuilder {
            GoogleCloudDocumentaiV1beta2DocumentProvenanceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of provenance operation."]
    pub enum GoogleCloudDocumentaiV1beta2DocumentProvenanceTypeEnum {
        #[serde(rename = "OPERATION_TYPE_UNSPECIFIED")]
        #[doc = "Operation type unspecified."]
        OperationTypeUnspecified,
        #[serde(rename = "ADD")]
        #[doc = "Add an element. Implicit if no `parents` are set for the provenance."]
        Add,
        #[serde(rename = "REMOVE")]
        #[doc = "The element is removed. No `parents` should be set."]
        Remove,
        #[serde(rename = "REPLACE")]
        #[doc = "Explicitly replaces the element(s) identified by `parents`."]
        Replace,
        #[serde(rename = "EVAL_REQUESTED")]
        #[doc = "Element is requested for human review."]
        EvalRequested,
        #[serde(rename = "EVAL_APPROVED")]
        #[doc = "Element is review and approved at human review, confidence will be set to 1.0"]
        EvalApproved,
    }
    impl ::std::default::Default for GoogleCloudDocumentaiV1beta2DocumentProvenanceTypeEnum {
        fn default() -> Self {
            Self::OperationTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Structure for referencing parent provenances. When an element replaces one of more other elements parent references identify the elements that are replaced."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentProvenanceParent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The id of the parent provenance."]
        pub id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index of the [Document.revisions] identifying the parent revision."]
        pub revision: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentProvenanceParent {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentProvenanceParentBuilder {
            GoogleCloudDocumentaiV1beta2DocumentProvenanceParentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains past or forward revisions of this document."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentRevision {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "agent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the change was made by a person specify the name or id of that person."]
        pub agent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time that the revision was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "humanReview")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human Review information of this revision."]
        pub human_review: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentRevisionHumanReview>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Id of the revision. Unique within the context of the document."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The revisions that this revision is based on. This can include one or more parent (when documents are merged.) This field represents the index into the `revisions` field."]
        pub parent: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "processor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the annotation was made by processor identify the processor by its resource name."]
        pub processor: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentRevision {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentRevisionBuilder {
            GoogleCloudDocumentaiV1beta2DocumentRevisionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Human Review information of the document."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentRevisionHumanReview {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human review state. e.g. `requested`, `succeeded`, `rejected`."]
        pub state: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A message providing more details about the current state of processing. For example, the rejection reason when the state is `rejected`."]
        pub state_message: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentRevisionHumanReview {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentRevisionHumanReviewBuilder {
            GoogleCloudDocumentaiV1beta2DocumentRevisionHumanReviewBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "For a large document, sharding may be performed to produce several document shards. Each document shard contains this field to detail which shard it is."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentShardInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shardCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of shards."]
        pub shard_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shardIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The 0-based index of this shard."]
        pub shard_index: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index of the first character in Document.text in the overall document global text."]
        pub text_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentShardInfo {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentShardInfoBuilder {
            GoogleCloudDocumentaiV1beta2DocumentShardInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotation for common text style attributes. This adheres to CSS conventions as much as possible."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentStyle {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backgroundColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text background color."]
        pub background_color: ::std::option::Option<::std::boxed::Box<GoogleTypeColor>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "color")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text color."]
        pub color: ::std::option::Option<::std::boxed::Box<GoogleTypeColor>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fontSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Font size."]
        pub font_size: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentStyleFontSize>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fontWeight")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Font weight. Possible values are normal, bold, bolder, and lighter. https://www.w3schools.com/cssref/pr_font_weight.asp"]
        pub font_weight: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textAnchor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text anchor indexing into the Document.text."]
        pub text_anchor: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentTextAnchor>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textDecoration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text decoration. Follows CSS standard. https://www.w3schools.com/cssref/pr_text_text-decoration.asp"]
        pub text_decoration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text style. Possible values are normal, italic, and oblique. https://www.w3schools.com/cssref/pr_font_font-style.asp"]
        pub text_style: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentStyle {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentStyleBuilder {
            GoogleCloudDocumentaiV1beta2DocumentStyleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Font size with unit."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentStyleFontSize {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Font size for the text."]
        pub size: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unit for the font size. Follows CSS naming (in, px, pt, etc.)."]
        pub unit: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentStyleFontSize {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentStyleFontSizeBuilder {
            GoogleCloudDocumentaiV1beta2DocumentStyleFontSizeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Text reference indexing into the Document.text."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentTextAnchor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains the content of the text span so that users do not have to look it up in the text_segments."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textSegments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text segments from the Document.text."]
        pub text_segments: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentTextAnchorTextSegment>,
            >,
        >,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentTextAnchor {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentTextAnchorBuilder {
            GoogleCloudDocumentaiV1beta2DocumentTextAnchorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A text segment in the Document.text. The indices may be out of bounds which indicate that the text extends into another document shard for large sharded documents. See ShardInfo.text_offset"]
    pub struct GoogleCloudDocumentaiV1beta2DocumentTextAnchorTextSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "TextSegment half open end UTF-8 char index in the Document.text."]
        pub end_index: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "TextSegment start UTF-8 char index in the Document.text."]
        pub start_index: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentTextAnchorTextSegment {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentTextAnchorTextSegmentBuilder {
            GoogleCloudDocumentaiV1beta2DocumentTextAnchorTextSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This message is used for text changes aka. OCR corrections."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentTextChange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "changedText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text that replaces the text identified in the `text_anchor`."]
        pub changed_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provenance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The history of this annotation."]
        pub provenance: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentProvenance>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textAnchor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Provenance of the correction. Text anchor indexing into the Document.text. There can only be a single `TextAnchor.text_segments` element. If the start and end index of the text segment are the same, the text change is inserted before that index."]
        pub text_anchor: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentTextAnchor>,
        >,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentTextChange {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentTextChangeBuilder {
            GoogleCloudDocumentaiV1beta2DocumentTextChangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A translation of the text segment."]
    pub struct GoogleCloudDocumentaiV1beta2DocumentTranslation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provenance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The history of this annotation."]
        pub provenance: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentProvenance>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textAnchor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Provenance of the translation. Text anchor indexing into the Document.text. There can only be a single `TextAnchor.text_segments` element. If the start and end index of the text segment are the same, the text change is inserted before that index."]
        pub text_anchor: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2DocumentTextAnchor>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "translatedText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text translated into the target language."]
        pub translated_text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta2DocumentTranslation {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2DocumentTranslationBuilder {
            GoogleCloudDocumentaiV1beta2DocumentTranslationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parameters to control entity extraction behavior."]
    pub struct GoogleCloudDocumentaiV1beta2EntityExtractionParams {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to enable entity extraction."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modelVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Model version of the entity extraction. Default is \"builtin/stable\". Specify \"builtin/latest\" for the latest model."]
        pub model_version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta2EntityExtractionParams {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2EntityExtractionParamsBuilder {
            GoogleCloudDocumentaiV1beta2EntityExtractionParamsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parameters to control form extraction behavior."]
    pub struct GoogleCloudDocumentaiV1beta2FormExtractionParams {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to enable form extraction."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyValuePairHints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reserved for future use."]
        pub key_value_pair_hints: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta2KeyValuePairHint>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modelVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Model version of the form extraction system. Default is \"builtin/stable\". Specify \"builtin/latest\" for the latest model. For custom form models, specify: “custom/{model_name}\". Model name format is \"bucket_name/path/to/modeldir\" corresponding to \"gs://bucket_name/path/to/modeldir\" where annotated examples are stored."]
        pub model_version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta2FormExtractionParams {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2FormExtractionParamsBuilder {
            GoogleCloudDocumentaiV1beta2FormExtractionParamsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Google Cloud Storage location where the output file will be written to."]
    pub struct GoogleCloudDocumentaiV1beta2GcsDestination {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta2GcsDestination {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2GcsDestinationBuilder {
            GoogleCloudDocumentaiV1beta2GcsDestinationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Google Cloud Storage location where the input file will be read from."]
    pub struct GoogleCloudDocumentaiV1beta2GcsSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta2GcsSource {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2GcsSourceBuilder {
            GoogleCloudDocumentaiV1beta2GcsSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The desired input location and metadata."]
    pub struct GoogleCloudDocumentaiV1beta2InputConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Content in bytes, represented as a stream of bytes. Note: As with all `bytes` fields, proto buffer messages use a pure binary representation, whereas JSON representations use base64. This field only works for synchronous ProcessDocument method."]
        pub contents: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Google Cloud Storage location to read the input from. This must be a single file."]
        pub gcs_source:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDocumentaiV1beta2GcsSource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Mimetype of the input. Current supported mimetypes are application/pdf, image/tiff, and image/gif. In addition, application/json type is supported for requests with ProcessDocumentRequest.automl_params field set. The JSON file needs to be in Document format."]
        pub mime_type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta2InputConfig {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2InputConfigBuilder {
            GoogleCloudDocumentaiV1beta2InputConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Reserved for future use."]
    pub struct GoogleCloudDocumentaiV1beta2KeyValuePairHint {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The key text for the hint."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of the value. This is case-insensitive, and could be one of: ADDRESS, LOCATION, ORGANIZATION, PERSON, PHONE_NUMBER, ID, NUMBER, EMAIL, PRICE, TERMS, DATE, NAME. Types not in this list will be ignored."]
        pub value_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudDocumentaiV1beta2KeyValuePairHint {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2KeyValuePairHintBuilder {
            GoogleCloudDocumentaiV1beta2KeyValuePairHintBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A vertex represents a 2D point in the image. NOTE: the normalized vertex coordinates are relative to the original image and range from 0 to 1."]
    pub struct GoogleCloudDocumentaiV1beta2NormalizedVertex {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "x")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "X coordinate."]
        pub x: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "y")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Y coordinate."]
        pub y: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudDocumentaiV1beta2NormalizedVertex {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2NormalizedVertexBuilder {
            GoogleCloudDocumentaiV1beta2NormalizedVertexBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parameters to control Optical Character Recognition (OCR) behavior."]
    pub struct GoogleCloudDocumentaiV1beta2OcrParams {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageHints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of languages to use for OCR. In most cases, an empty value yields the best results since it enables automatic language detection. For languages based on the Latin alphabet, setting `language_hints` is not needed. In rare cases, when the language of the text in the image is known, setting a hint will help get better results (although it will be a significant hindrance if the hint is wrong). Document processing returns an error if one or more of the specified languages is not one of the supported languages."]
        pub language_hints: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudDocumentaiV1beta2OcrParams {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2OcrParamsBuilder {
            GoogleCloudDocumentaiV1beta2OcrParamsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains metadata for the BatchProcessDocuments operation."]
    pub struct GoogleCloudDocumentaiV1beta2OperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creation time of the operation."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state of the current batch processing."]
        pub state: ::std::option::Option<GoogleCloudDocumentaiV1beta2OperationMetadataStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A message providing more details about the current state of processing."]
        pub state_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last update time of the operation."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta2OperationMetadata {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2OperationMetadataBuilder {
            GoogleCloudDocumentaiV1beta2OperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The state of the current batch processing."]
    pub enum GoogleCloudDocumentaiV1beta2OperationMetadataStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "The default value. This value is used if the state is omitted."]
        StateUnspecified,
        #[serde(rename = "ACCEPTED")]
        #[doc = "Request is received."]
        Accepted,
        #[serde(rename = "WAITING")]
        #[doc = "Request operation is waiting for scheduling."]
        Waiting,
        #[serde(rename = "RUNNING")]
        #[doc = "Request is being processed."]
        Running,
        #[serde(rename = "SUCCEEDED")]
        #[doc = "The batch processing completed successfully."]
        Succeeded,
        #[serde(rename = "CANCELLED")]
        #[doc = "The batch processing was cancelled."]
        Cancelled,
        #[serde(rename = "FAILED")]
        #[doc = "The batch processing has failed."]
        Failed,
    }
    impl ::std::default::Default for GoogleCloudDocumentaiV1beta2OperationMetadataStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The desired output location and metadata."]
    pub struct GoogleCloudDocumentaiV1beta2OutputConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsDestination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Google Cloud Storage location to write the output to."]
        pub gcs_destination:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDocumentaiV1beta2GcsDestination>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pagesPerShard")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The max number of pages to include into each output Document shard JSON on Google Cloud Storage. The valid range is [1, 100]. If not specified, the default value is 20. For example, for one pdf file with 100 pages, 100 parsed pages will be produced. If `pages_per_shard` = 20, then 5 Document shard JSON files each containing 20 parsed pages will be written under the prefix OutputConfig.gcs_destination.uri and suffix pages-x-to-y.json where x and y are 1-indexed page numbers. Example GCS outputs with 157 pages and pages_per_shard = 50: pages-001-to-050.json pages-051-to-100.json pages-101-to-150.json pages-151-to-157.json"]
        pub pages_per_shard: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDocumentaiV1beta2OutputConfig {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2OutputConfigBuilder {
            GoogleCloudDocumentaiV1beta2OutputConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to process one document."]
    pub struct GoogleCloudDocumentaiV1beta2ProcessDocumentRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "automlParams")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Controls AutoML model prediction behavior. AutoMlParams cannot be used together with other Params."]
        pub automl_params:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDocumentaiV1beta2AutoMlParams>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "documentType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies a known document type for deeper structure detection. Valid values are currently \"general\" and \"invoice\". If not provided, \"general\"\\ is used as default. If any other value is given, the request is rejected."]
        pub document_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityExtractionParams")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Controls entity extraction behavior. If not specified, the system will decide reasonable defaults."]
        pub entity_extraction_params: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2EntityExtractionParams>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formExtractionParams")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Controls form extraction behavior. If not specified, the system will decide reasonable defaults."]
        pub form_extraction_params: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2FormExtractionParams>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Information about the input file."]
        pub input_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDocumentaiV1beta2InputConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ocrParams")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Controls OCR behavior. If not specified, the system will decide reasonable defaults."]
        pub ocr_params:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDocumentaiV1beta2OcrParams>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The desired output location. This field is only needed in BatchProcessDocumentsRequest."]
        pub output_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDocumentaiV1beta2OutputConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no location is specified, a region will be chosen automatically. This field is only populated when used in ProcessDocument method."]
        pub parent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableExtractionParams")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Controls table extraction behavior. If not specified, the system will decide reasonable defaults."]
        pub table_extraction_params: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta2TableExtractionParams>,
        >,
    }
    impl GoogleCloudDocumentaiV1beta2ProcessDocumentRequest {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2ProcessDocumentRequestBuilder {
            GoogleCloudDocumentaiV1beta2ProcessDocumentRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to a single document processing request."]
    pub struct GoogleCloudDocumentaiV1beta2ProcessDocumentResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about the input file. This is the same as the corresponding input config in the request."]
        pub input_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDocumentaiV1beta2InputConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The output location of the parsed responses. The responses are written to this location as JSON-serialized `Document` objects."]
        pub output_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDocumentaiV1beta2OutputConfig>>,
    }
    impl GoogleCloudDocumentaiV1beta2ProcessDocumentResponse {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2ProcessDocumentResponseBuilder {
            GoogleCloudDocumentaiV1beta2ProcessDocumentResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A hint for a table bounding box on the page for table parsing."]
    pub struct GoogleCloudDocumentaiV1beta2TableBoundHint {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Bounding box hint for a table on this page. The coordinates must be normalized to [0,1] and the bounding box must be an axis-aligned rectangle."]
        pub bounding_box:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDocumentaiV1beta2BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Page number for multi-paged inputs this hint applies to. If not provided, this hint will apply to all pages by default. This value is 1-based."]
        pub page_number: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDocumentaiV1beta2TableBoundHint {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2TableBoundHintBuilder {
            GoogleCloudDocumentaiV1beta2TableBoundHintBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parameters to control table extraction behavior."]
    pub struct GoogleCloudDocumentaiV1beta2TableExtractionParams {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to enable table extraction."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headerHints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Reserved for future use."]
        pub header_hints: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modelVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Model version of the table extraction system. Default is \"builtin/stable\". Specify \"builtin/latest\" for the latest model."]
        pub model_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableBoundHints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Table bounding box hints that can be provided to complex cases which our algorithm cannot locate the table(s) in."]
        pub table_bound_hints: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDocumentaiV1beta2TableBoundHint>>,
        >,
    }
    impl GoogleCloudDocumentaiV1beta2TableExtractionParams {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2TableExtractionParamsBuilder {
            GoogleCloudDocumentaiV1beta2TableExtractionParamsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A vertex represents a 2D point in the image. NOTE: the vertex coordinates are in the same scale as the original image."]
    pub struct GoogleCloudDocumentaiV1beta2Vertex {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "x")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "X coordinate."]
        pub x: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "y")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Y coordinate."]
        pub y: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDocumentaiV1beta2Vertex {
        pub fn builder() -> GoogleCloudDocumentaiV1beta2VertexBuilder {
            GoogleCloudDocumentaiV1beta2VertexBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The long running operation metadata for batch process method."]
    pub struct GoogleCloudDocumentaiV1beta3BatchProcessMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creation time of the operation."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "individualProcessStatuses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of response details of each document."]
        pub individual_process_statuses: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<
                    GoogleCloudDocumentaiV1beta3BatchProcessMetadataIndividualProcessStatus,
                >,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state of the current batch processing."]
        pub state: ::std::option::Option<GoogleCloudDocumentaiV1beta3BatchProcessMetadataStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A message providing more details about the current state of processing. For example, the error message if the operation is failed."]
        pub state_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last update time of the operation."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta3BatchProcessMetadata {
        pub fn builder() -> GoogleCloudDocumentaiV1beta3BatchProcessMetadataBuilder {
            GoogleCloudDocumentaiV1beta3BatchProcessMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The state of the current batch processing."]
    pub enum GoogleCloudDocumentaiV1beta3BatchProcessMetadataStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "The default value. This value is used if the state is omitted."]
        StateUnspecified,
        #[serde(rename = "WAITING")]
        #[doc = "Request operation is waiting for scheduling."]
        Waiting,
        #[serde(rename = "RUNNING")]
        #[doc = "Request is being processed."]
        Running,
        #[serde(rename = "SUCCEEDED")]
        #[doc = "The batch processing completed successfully."]
        Succeeded,
        #[serde(rename = "CANCELLING")]
        #[doc = "The batch processing was being cancelled."]
        Cancelling,
        #[serde(rename = "CANCELLED")]
        #[doc = "The batch processing was cancelled."]
        Cancelled,
        #[serde(rename = "FAILED")]
        #[doc = "The batch processing has failed."]
        Failed,
    }
    impl ::std::default::Default for GoogleCloudDocumentaiV1beta3BatchProcessMetadataStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The status of a each individual document in the batch process."]
    pub struct GoogleCloudDocumentaiV1beta3BatchProcessMetadataIndividualProcessStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "humanReviewOperation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the operation triggered by the processed document. If the human review process is not triggered, this field will be empty. It has the same response type and metadata as the long running operation returned by ReviewDocument method."]
        pub human_review_operation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "humanReviewStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of human review on the processed document."]
        pub human_review_status:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDocumentaiV1beta3HumanReviewStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputGcsSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source of the document, same as the [input_gcs_source] field in the request when the batch process started. The batch process is started by take snapshot of that document, since a user can move or change that document during the process."]
        pub input_gcs_source: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputGcsDestination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The output_gcs_destination (in the request as 'output_gcs_destination') of the processed document if it was successful, otherwise empty."]
        pub output_gcs_destination: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the processing of the document."]
        pub status: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
    }
    impl GoogleCloudDocumentaiV1beta3BatchProcessMetadataIndividualProcessStatus {
        pub fn builder(
        ) -> GoogleCloudDocumentaiV1beta3BatchProcessMetadataIndividualProcessStatusBuilder
        {
            GoogleCloudDocumentaiV1beta3BatchProcessMetadataIndividualProcessStatusBuilder::default(
            )
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for batch process document method."]
    pub struct GoogleCloudDocumentaiV1beta3BatchProcessResponse {}
    impl GoogleCloudDocumentaiV1beta3BatchProcessResponse {
        pub fn builder() -> GoogleCloudDocumentaiV1beta3BatchProcessResponseBuilder {
            GoogleCloudDocumentaiV1beta3BatchProcessResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The common metadata for long running operations."]
    pub struct GoogleCloudDocumentaiV1beta3CommonOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creation time of the operation."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state of the operation."]
        pub state:
            ::std::option::Option<GoogleCloudDocumentaiV1beta3CommonOperationMetadataStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A message providing more details about the current state of processing."]
        pub state_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last update time of the operation."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta3CommonOperationMetadata {
        pub fn builder() -> GoogleCloudDocumentaiV1beta3CommonOperationMetadataBuilder {
            GoogleCloudDocumentaiV1beta3CommonOperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The state of the operation."]
    pub enum GoogleCloudDocumentaiV1beta3CommonOperationMetadataStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Unspecified state."]
        StateUnspecified,
        #[serde(rename = "RUNNING")]
        #[doc = "Operation is still running."]
        Running,
        #[serde(rename = "CANCELLING")]
        #[doc = "Operation is being cancelled."]
        Cancelling,
        #[serde(rename = "SUCCEEDED")]
        #[doc = "Operation succeeded."]
        Succeeded,
        #[serde(rename = "FAILED")]
        #[doc = "Operation failed."]
        Failed,
        #[serde(rename = "CANCELLED")]
        #[doc = "Operation is cancelled."]
        Cancelled,
    }
    impl ::std::default::Default for GoogleCloudDocumentaiV1beta3CommonOperationMetadataStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The status of human review on a processed document."]
    pub struct GoogleCloudDocumentaiV1beta3HumanReviewStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "humanReviewOperation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the operation triggered by the processed document. This field is populated only when the [state] is [HUMAN_REVIEW_IN_PROGRESS]. It has the same response type and metadata as the long running operation returned by [ReviewDocument] method."]
        pub human_review_operation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state of human review on the processing request."]
        pub state: ::std::option::Option<GoogleCloudDocumentaiV1beta3HumanReviewStatusStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A message providing more details about the human review state."]
        pub state_message: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta3HumanReviewStatus {
        pub fn builder() -> GoogleCloudDocumentaiV1beta3HumanReviewStatusBuilder {
            GoogleCloudDocumentaiV1beta3HumanReviewStatusBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The state of human review on the processing request."]
    pub enum GoogleCloudDocumentaiV1beta3HumanReviewStatusStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Human review state is unspecified. Most likely due to an internal error."]
        StateUnspecified,
        #[serde(rename = "HUMAN_REVIEW_SKIPPED")]
        #[doc = "Human review is skipped for the document. This can happen because human review is not enabled on the processor or the processing request has been set to skip this document."]
        HumanReviewSkipped,
        #[serde(rename = "HUMAN_REVIEW_VALIDATION_PASSED")]
        #[doc = "Human review validation is triggered and passed, so no review is needed."]
        HumanReviewValidationPassed,
        #[serde(rename = "HUMAN_REVIEW_IN_PROGRESS")]
        #[doc = "Human review validation is triggered and the document is under review."]
        HumanReviewInProgress,
        #[serde(rename = "HUMAN_REVIEW_ERROR")]
        #[doc = "Some error happened during triggering human review, see the [state_message] for details."]
        HumanReviewError,
    }
    impl ::std::default::Default for GoogleCloudDocumentaiV1beta3HumanReviewStatusStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The long running operation metadata for review document method."]
    pub struct GoogleCloudDocumentaiV1beta3ReviewDocumentOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commonMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The basic metadata of the long running operation."]
        pub common_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDocumentaiV1beta3CommonOperationMetadata>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creation time of the operation."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Used only when Operation.done is false."]
        pub state: ::std::option::Option<
            GoogleCloudDocumentaiV1beta3ReviewDocumentOperationMetadataStateEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A message providing more details about the current state of processing. For example, the error message if the operation is failed."]
        pub state_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last update time of the operation."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta3ReviewDocumentOperationMetadata {
        pub fn builder() -> GoogleCloudDocumentaiV1beta3ReviewDocumentOperationMetadataBuilder {
            GoogleCloudDocumentaiV1beta3ReviewDocumentOperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Used only when Operation.done is false."]
    pub enum GoogleCloudDocumentaiV1beta3ReviewDocumentOperationMetadataStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Unspecified state."]
        StateUnspecified,
        #[serde(rename = "RUNNING")]
        #[doc = "Operation is still running."]
        Running,
        #[serde(rename = "CANCELLING")]
        #[doc = "Operation is being cancelled."]
        Cancelling,
        #[serde(rename = "SUCCEEDED")]
        #[doc = "Operation succeeded."]
        Succeeded,
        #[serde(rename = "FAILED")]
        #[doc = "Operation failed."]
        Failed,
        #[serde(rename = "CANCELLED")]
        #[doc = "Operation is cancelled."]
        Cancelled,
    }
    impl ::std::default::Default
        for GoogleCloudDocumentaiV1beta3ReviewDocumentOperationMetadataStateEnum
    {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for review document method."]
    pub struct GoogleCloudDocumentaiV1beta3ReviewDocumentResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsDestination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Cloud Storage uri for the human reviewed document."]
        pub gcs_destination: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDocumentaiV1beta3ReviewDocumentResponse {
        pub fn builder() -> GoogleCloudDocumentaiV1beta3ReviewDocumentResponseBuilder {
            GoogleCloudDocumentaiV1beta3ReviewDocumentResponseBuilder::default()
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
        pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
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
    #[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
    pub struct GoogleProtobufEmpty {}
    impl GoogleProtobufEmpty {
        pub fn builder() -> GoogleProtobufEmptyBuilder {
            GoogleProtobufEmptyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
    pub struct GoogleRpcStatus {
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
    impl GoogleRpcStatus {
        pub fn builder() -> GoogleRpcStatusBuilder {
            GoogleRpcStatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a color in the RGBA color space. This representation is designed for simplicity of conversion to/from color representations in various languages over compactness; for example, the fields of this representation can be trivially provided to the constructor of \"java.awt.Color\" in Java; it can also be trivially provided to UIColor's \"+colorWithRed:green:blue:alpha\" method in iOS; and, with just a little work, it can be easily formatted into a CSS \"rgba()\" string in JavaScript, as well. Note: this proto does not carry information about the absolute color space that should be used to interpret the RGB value (e.g. sRGB, Adobe RGB, DCI-P3, BT.2020, etc.). By default, applications SHOULD assume the sRGB color space. Note: when color equality needs to be decided, implementations, unless documented otherwise, will treat two colors to be equal if all their red, green, blue and alpha values each differ by at most 1e-5. Example (Java): import com.google.type.Color; // ... public static java.awt.Color fromProto(Color protocolor) { float alpha = protocolor.hasAlpha() ? protocolor.getAlpha().getValue() : 1.0; return new java.awt.Color( protocolor.getRed(), protocolor.getGreen(), protocolor.getBlue(), alpha); } public static Color toProto(java.awt.Color color) { float red = (float) color.getRed(); float green = (float) color.getGreen(); float blue = (float) color.getBlue(); float denominator = 255.0; Color.Builder resultBuilder = Color .newBuilder() .setRed(red / denominator) .setGreen(green / denominator) .setBlue(blue / denominator); int alpha = color.getAlpha(); if (alpha != 255) { result.setAlpha( FloatValue .newBuilder() .setValue(((float) alpha) / denominator) .build()); } return resultBuilder.build(); } // ... Example (iOS / Obj-C): // ... static UIColor* fromProto(Color* protocolor) { float red = [protocolor red]; float green = [protocolor green]; float blue = [protocolor blue]; FloatValue* alpha_wrapper = [protocolor alpha]; float alpha = 1.0; if (alpha_wrapper != nil) { alpha = [alpha_wrapper value]; } return [UIColor colorWithRed:red green:green blue:blue alpha:alpha]; } static Color* toProto(UIColor* color) { CGFloat red, green, blue, alpha; if (![color getRed:&red green:&green blue:&blue alpha:&alpha]) { return nil; } Color* result = [[Color alloc] init]; [result setRed:red]; [result setGreen:green]; [result setBlue:blue]; if (alpha <= 0.9999) { [result setAlpha:floatWrapperWithValue(alpha)]; } [result autorelease]; return result; } // ... Example (JavaScript): // ... var protoToCssColor = function(rgb_color) { var redFrac = rgb_color.red || 0.0; var greenFrac = rgb_color.green || 0.0; var blueFrac = rgb_color.blue || 0.0; var red = Math.floor(redFrac * 255); var green = Math.floor(greenFrac * 255); var blue = Math.floor(blueFrac * 255); if (!('alpha' in rgb_color)) { return rgbToCssColor_(red, green, blue); } var alphaFrac = rgb_color.alpha.value || 0.0; var rgbParams = [red, green, blue].join(','); return ['rgba(', rgbParams, ',', alphaFrac, ')'].join(''); }; var rgbToCssColor_ = function(red, green, blue) { var rgbNumber = new Number((red << 16) | (green << 8) | blue); var hexString = rgbNumber.toString(16); var missingZeros = 6 - hexString.length; var resultBuilder = ['#']; for (var i = 0; i < missingZeros; i++) { resultBuilder.push('0'); } resultBuilder.push(hexString); return resultBuilder.join(''); }; // ..."]
    pub struct GoogleTypeColor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alpha")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fraction of this color that should be applied to the pixel. That is, the final pixel color is defined by the equation: pixel color = alpha * (this color) + (1.0 - alpha) * (background color) This means that a value of 1.0 corresponds to a solid color, whereas a value of 0.0 corresponds to a completely transparent color. This uses a wrapper message rather than a simple float scalar so that it is possible to distinguish between a default value and the value being unset. If omitted, this color object is to be rendered as a solid color (as if the alpha value had been explicitly given with a value of 1.0)."]
        pub alpha: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of blue in the color as a value in the interval [0, 1]."]
        pub blue: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "green")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of green in the color as a value in the interval [0, 1]."]
        pub green: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "red")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of red in the color as a value in the interval [0, 1]."]
        pub red: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleTypeColor {
        pub fn builder() -> GoogleTypeColorBuilder {
            GoogleTypeColorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values * A month and day value, with a zero year, such as an anniversary * A year on its own, with zero month and day values * A year and month value, with a zero day, such as a credit card expiration date Related types are google.type.TimeOfDay and `google.protobuf.Timestamp`."]
    pub struct GoogleTypeDate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "day")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
        pub day: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "month")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
        pub month: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "year")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
        pub year: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleTypeDate {
        pub fn builder() -> GoogleTypeDateBuilder {
            GoogleTypeDateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents civil time (or occasionally physical time). This type can represent a civil time in one of a few possible ways: * When utc_offset is set and time_zone is unset: a civil time on a calendar day with a particular offset from UTC. * When time_zone is set and utc_offset is unset: a civil time on a calendar day in a particular time zone. * When neither time_zone nor utc_offset is set: a civil time on a calendar day in local time. The date is relative to the Proleptic Gregorian Calendar. If year is 0, the DateTime is considered not to have a specific year. month and day must have valid, non-zero values. This type may also be used to represent a physical time if all the date and time fields are set and either case of the `time_offset` oneof is set. Consider using `Timestamp` message for physical time instead. If your use case also would like to store the user's timezone, that can be done in another field. This type is more flexible than some applications may want. Make sure to document and validate your application's limitations."]
    pub struct GoogleTypeDateTime {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "day")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Day of month. Must be from 1 to 31 and valid for the year and month."]
        pub day: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hours")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Hours of day in 24 hour format. Should be from 0 to 23. An API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
        pub hours: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minutes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Minutes of hour of day. Must be from 0 to 59."]
        pub minutes: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "month")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Month of year. Must be from 1 to 12."]
        pub month: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nanos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
        pub nanos: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "seconds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Seconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds."]
        pub seconds: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time zone."]
        pub time_zone: ::std::option::Option<::std::boxed::Box<GoogleTypeTimeZone>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "utcOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "UTC offset. Must be whole seconds, between -18 hours and +18 hours. For example, a UTC offset of -4:00 would be represented as { seconds: -14400 }."]
        pub utc_offset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "year")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Year of date. Must be from 1 to 9999, or 0 if specifying a datetime without a year."]
        pub year: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleTypeDateTime {
        pub fn builder() -> GoogleTypeDateTimeBuilder {
            GoogleTypeDateTimeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents an amount of money with its currency type."]
    pub struct GoogleTypeMoney {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currencyCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The three-letter currency code defined in ISO 4217."]
        pub currency_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nanos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000."]
        pub nanos: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "units")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The whole units of the amount. For example if `currencyCode` is `\"USD\"`, then 1 unit is one US dollar."]
        pub units: ::std::option::Option<::std::string::String>,
    }
    impl GoogleTypeMoney {
        pub fn builder() -> GoogleTypeMoneyBuilder {
            GoogleTypeMoneyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a postal address, e.g. for postal delivery or payments addresses. Given a postal address, a postal service can deliver items to a premise, P.O. Box or similar. It is not intended to model geographical locations (roads, towns, mountains). In typical usage an address would be created via user input or from importing existing data, depending on the type of process. Advice on address input / editing: - Use an i18n-ready address widget such as https://github.com/google/libaddressinput) - Users should not be presented with UI elements for input or editing of fields outside countries where that field is used. For more guidance on how to use this schema, please see: https://support.google.com/business/answer/6397478"]
    pub struct GoogleTypePostalAddress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addressLines")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unstructured address lines describing the lower levels of an address. Because values in address_lines do not have type information and may sometimes contain multiple values in a single field (e.g. \"Austin, TX\"), it is important that the line order is clear. The order of address lines should be \"envelope order\" for the country/region of the address. In places where this can vary (e.g. Japan), address_language is used to make it explicit (e.g. \"ja\" for large-to-small ordering and \"ja-Latn\" or \"en\" for small-to-large). This way, the most specific line of an address can be selected based on the language. The minimum permitted structural representation of an address consists of a region_code with all remaining information placed in the address_lines. It would be possible to format such an address very approximately without geocoding, but no semantic reasoning could be made about any of the address components until it was at least partially resolved. Creating an address only containing a region_code and address_lines, and then geocoding is the recommended way to handle completely unstructured addresses (as opposed to guessing which parts of the address should be localities or administrative areas)."]
        pub address_lines: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "administrativeArea")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Highest administrative subdivision which is used for postal addresses of a country or region. For example, this can be a state, a province, an oblast, or a prefecture. Specifically, for Spain this is the province and not the autonomous community (e.g. \"Barcelona\" and not \"Catalonia\"). Many countries don't use an administrative area in postal addresses. E.g. in Switzerland this should be left unpopulated."]
        pub administrative_area: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. BCP-47 language code of the contents of this address (if known). This is often the UI language of the input form or is expected to match one of the languages used in the address' country/region, or their transliterated equivalents. This can affect formatting in certain countries, but is not critical to the correctness of the data and will never affect any validation or other non-formatting related operations. If this value is not known, it should be omitted (rather than specifying a possibly incorrect default). Examples: \"zh-Hant\", \"ja\", \"ja-Latn\", \"en\"."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locality")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Generally refers to the city/town portion of the address. Examples: US city, IT comune, UK post town. In regions of the world where localities are not well defined or do not fit into this structure well, leave locality empty and use address_lines."]
        pub locality: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "organization")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The name of the organization at the address."]
        pub organization: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postalCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Postal code of the address. Not all countries use or require postal codes to be present, but where they are used, they may trigger additional validation with other parts of the address (e.g. state/zip validation in the U.S.A.)."]
        pub postal_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recipients")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The recipient at the address. This field may, under certain circumstances, contain multiline information. For example, it might contain \"care of\" information."]
        pub recipients: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "regionCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. CLDR region code of the country/region of the address. This is never inferred and it is up to the user to ensure the value is correct. See http://cldr.unicode.org/ and http://www.unicode.org/cldr/charts/30/supplemental/territory_information.html for details. Example: \"CH\" for Switzerland."]
        pub region_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The schema revision of the `PostalAddress`. This must be set to 0, which is the latest revision. All new revisions **must** be backward compatible with old revisions."]
        pub revision: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sortingCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Additional, country-specific, sorting code. This is not used in most regions. Where it is used, the value is either a string like \"CEDEX\", optionally followed by a number (e.g. \"CEDEX 7\"), or just a number alone, representing the \"sector code\" (Jamaica), \"delivery area indicator\" (Malawi) or \"post office indicator\" (e.g. Côte d'Ivoire)."]
        pub sorting_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sublocality")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Sublocality of the address. For example, this can be neighborhoods, boroughs, districts."]
        pub sublocality: ::std::option::Option<::std::string::String>,
    }
    impl GoogleTypePostalAddress {
        pub fn builder() -> GoogleTypePostalAddressBuilder {
            GoogleTypePostalAddressBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a time zone from the [IANA Time Zone Database](https://www.iana.org/time-zones)."]
    pub struct GoogleTypeTimeZone {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "IANA Time Zone Database time zone, e.g. \"America/New_York\"."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. IANA Time Zone Database version number, e.g. \"2019a\"."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleTypeTimeZone {
        pub fn builder() -> GoogleTypeTimeZoneBuilder {
            GoogleTypeTimeZoneBuilder::default()
        }
    }
}
