#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for downloading a model to device."]
pub struct DownloadModelResponse {
    #[serde(rename = "downloadUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A download URI for the model/zip file."]
    pub download_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expireTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time that the download URI link expires. If the link has expired, the REST call must be repeated."]
    pub expire_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "modelFormat")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The format of the model being downloaded."]
    pub model_format: ::std::option::Option<DownloadModelResponseModelFormatEnum>,
    #[serde(rename = "sizeBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The size of the file(s), if this information is available."]
    pub size_bytes: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The format of the model being downloaded."]
pub enum DownloadModelResponseModelFormatEnum {
    #[serde(rename = "MODEL_FORMAT_UNSPECIFIED")]
    #[doc = "Unknown format"]
    ModelFormatUnspecified,
    #[serde(rename = "TFLITE")]
    #[doc = "TFLite model"]
    Tflite,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response for list models"]
pub struct ListModelsResponse {
    #[serde(rename = "models")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of models"]
    pub models: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Model>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An ML model hosted in Firebase ML"]
pub struct Model {
    #[serde(rename = "activeOperations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Lists operation ids associated with this model whose status is NOT done."]
    pub active_operations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Operation>>>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Timestamp when this model was created in Firebase ML."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the model to create. The name can be up to 32 characters long and can consist only of ASCII Latin letters A-Z and a-z, underscores(_) and ASCII digits 0-9. It must start with a letter."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. See RFC7232 https://tools.ietf.org/html/rfc7232#section-2.3"]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "modelHash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The model_hash will change if a new file is available for download."]
    pub model_hash: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the Model. Model names have the form `projects/{project_id}/models/{model_id}` The name is ignored when creating a model."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "State common to all model types. Includes publishing and validation information."]
    pub state: ::std::option::Option<::std::boxed::Box<ModelState>>,
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User defined tags which can be used to group/filter models during listing"]
    pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "tfliteModel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A TFLite Model"]
    pub tflite_model: ::std::option::Option<::std::boxed::Box<TfLiteModel>>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Timestamp when this model was updated in Firebase ML."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This is returned in the longrunning operations for create/update."]
pub struct ModelOperationMetadata {
    #[serde(rename = "basicOperationStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub basic_operation_status:
        ::std::option::Option<ModelOperationMetadataBasicOperationStatusEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the model we are creating/updating The name must have the form `projects/{project_id}/models/{model_id}`"]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum ModelOperationMetadataBasicOperationStatusEnum {
    #[serde(rename = "BASIC_OPERATION_STATUS_UNSPECIFIED")]
    #[doc = "The status is unspecified"]
    BasicOperationStatusUnspecified,
    #[serde(rename = "BASIC_OPERATION_STATUS_UPLOADING")]
    #[doc = "The model file is being uploaded"]
    BasicOperationStatusUploading,
    #[serde(rename = "BASIC_OPERATION_STATUS_VERIFYING")]
    #[doc = "The model file is being verified"]
    BasicOperationStatusVerifying,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "State common to all model types. Includes publishing and validation information."]
pub struct ModelState {
    #[serde(rename = "published")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates if this model has been published."]
    pub published: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "validationError")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Indicates the latest validation error on the model if any. A model may have validation errors if there were problems during the model creation/update. e.g. in the case of a TfLiteModel, if a tflite model file was missing or in the wrong format. This field will be empty for valid models."]
    pub validation_error: ::std::option::Option<::std::boxed::Box<Status>>,
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
#[doc = "Information that is specific to TfLite models."]
pub struct TfLiteModel {
    #[serde(rename = "automlModel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The AutoML model id referencing a model you created with the AutoML API. The name should have format 'projects//locations//models/' (This is the model resource name returned from the AutoML API)"]
    pub automl_model: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gcsTfliteUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The TfLite file containing the model. (Stored in Google Cloud). The gcs_tflite_uri should have form: gs://some-bucket/some-model.tflite Note: If you update the file in the original location, it is necessary to call UpdateModel for ML to pick up and validate the updated file."]
    pub gcs_tflite_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sizeBytes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The size of the TFLite model"]
    pub size_bytes: ::std::option::Option<::std::string::String>,
}
