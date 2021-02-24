#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The batch translation request."]
pub struct BatchTranslateTextRequest {
    #[serde(rename = "glossaries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Glossaries to be applied for translation. It's keyed by target language code."]
    pub glossaries: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<TranslateTextGlossaryConfig>>,
    >,
    #[serde(rename = "inputConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Input configurations. The total number of files matched should be <= 100. The total content size should be <= 100M Unicode codepoints. The files must use UTF-8 encoding."]
    pub input_configs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<InputConfig>>>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The labels with user-defined metadata for the request. Label keys and values can be no longer than 63 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter. See https://cloud.google.com/translate/docs/labels for more information."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "models")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The models to use for translation. Map's key is target language code. Map's value is model name. Value can be a built-in general model, or an AutoML Translation model. The value format depends on model type: - AutoML Translation models: `projects/{project-number-or-id}/locations/{location-id}/models/{model-id}` - General (built-in) models: `projects/{project-number-or-id}/locations/{location-id}/models/general/nmt`, `projects/{project-number-or-id}/locations/{location-id}/models/general/base` If the map is empty or a specific model is not requested for a language pair, then default google model (nmt) is used."]
    pub models: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "outputConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Output configuration. If 2 input configs match to the same file (that is, same input path), we don't generate output for duplicate inputs."]
    pub output_config: ::std::option::Option<::std::boxed::Box<OutputConfig>>,
    #[serde(rename = "sourceLanguageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Source language code."]
    pub source_language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetLanguageCodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Specify up to 10 language codes here."]
    pub target_language_codes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for Operations.CancelOperation."]
pub struct CancelOperationRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for language detection."]
pub struct DetectLanguageRequest {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The content of the input stored as a string."]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The labels with user-defined metadata for the request. Label keys and values can be no longer than 63 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter. See https://cloud.google.com/translate/docs/labels for more information."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The format of the source text, for example, \"text/html\", \"text/plain\". If left blank, the MIME type defaults to \"text/html\"."]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "model")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The language detection model to be used. Format: `projects/{project-number-or-id}/locations/{location-id}/models/language-detection/{model-id}` Only one language detection model is currently supported: `projects/{project-number-or-id}/locations/{location-id}/models/language-detection/default`. If not specified, the default model is used."]
    pub model: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for language detection."]
pub struct DetectLanguageResponse {
    #[serde(rename = "languages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of detected languages sorted by detection confidence in descending order. The most probable language first."]
    pub languages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DetectedLanguage>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for language detection."]
pub struct DetectedLanguage {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The confidence of the detection result for this language."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The BCP-47 language code of source content in the request, detected automatically."]
    pub language_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The Google Cloud Storage location for the output content."]
pub struct GcsDestination {
    #[serde(rename = "outputUriPrefix")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. There must be no files under 'output_uri_prefix'. 'output_uri_prefix' must end with \"/\" and start with \"gs://\", otherwise an INVALID_ARGUMENT (400) error is returned."]
    pub output_uri_prefix: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The Google Cloud Storage location for the input content."]
pub struct GcsSource {
    #[serde(rename = "inputUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Source data URI. For example, `gs://my_bucket/my_object`."]
    pub input_uri: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a glossary built from user provided data."]
pub struct Glossary {
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. When the glossary creation was finished."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entryCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The number of entries defined in the glossary."]
    pub entry_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "inputConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Provides examples to build the glossary from. Total glossary must not exceed 10M Unicode codepoints."]
    pub input_config: ::std::option::Option<::std::boxed::Box<GlossaryInputConfig>>,
    #[serde(rename = "languageCodesSet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Used with equivalent term set glossaries."]
    pub language_codes_set: ::std::option::Option<::std::boxed::Box<LanguageCodesSet>>,
    #[serde(rename = "languagePair")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Used with unidirectional glossaries."]
    pub language_pair: ::std::option::Option<::std::boxed::Box<LanguageCodePair>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The resource name of the glossary. Glossary names have the form `projects/{project-number-or-id}/locations/{location-id}/glossaries/{glossary-id}`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "submitTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. When CreateGlossary was called."]
    pub submit_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Input configuration for glossaries."]
pub struct GlossaryInputConfig {
    #[serde(rename = "gcsSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Google Cloud Storage location of glossary data. File format is determined based on the filename extension. API returns [google.rpc.Code.INVALID_ARGUMENT] for unsupported URI-s and file formats. Wildcards are not allowed. This must be a single file in one of the following formats: For unidirectional glossaries: - TSV/CSV (`.tsv`/`.csv`): 2 column file, tab- or comma-separated. The first column is source text. The second column is target text. The file must not contain headers. That is, the first row is data, not column names. - TMX (`.tmx`): TMX file with parallel data defining source/target term pairs. For equivalent term sets glossaries: - CSV (`.csv`): Multi-column CSV file defining equivalent glossary terms in multiple languages. The format is defined for Google Translation Toolkit and documented in [Use a glossary](https://support.google.com/translatortoolkit/answer/6306379?hl=en)."]
    pub gcs_source: ::std::option::Option<::std::boxed::Box<GcsSource>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Input configuration for BatchTranslateText request."]
pub struct InputConfig {
    #[serde(rename = "gcsSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Google Cloud Storage location for the source input. This can be a single file (for example, `gs://translation-test/input.tsv`) or a wildcard (for example, `gs://translation-test/*`). If a file extension is `.tsv`, it can contain either one or two columns. The first column (optional) is the id of the text request. If the first column is missing, we use the row number (0-based) from the input file as the ID in the output file. The second column is the actual text to be translated. We recommend each row be <= 10K Unicode codepoints, otherwise an error might be returned. Note that the input tsv must be RFC 4180 compliant. You could use https://github.com/Clever/csvlint to check potential formatting errors in your tsv file. csvlint --delimiter='\\t' your_input_file.tsv The other supported file extensions are `.txt` or `.html`, which is treated as a single large chunk of text."]
    pub gcs_source: ::std::option::Option<::std::boxed::Box<GcsSource>>,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Can be \"text/plain\" or \"text/html\". For `.tsv`, \"text/html\" is used if mime_type is missing. For `.html`, this field must be \"text/html\" or empty. For `.txt`, this field must be \"text/plain\" or empty."]
    pub mime_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Used with unidirectional glossaries."]
pub struct LanguageCodePair {
    #[serde(rename = "sourceLanguageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The BCP-47 language code of the input text, for example, \"en-US\". Expected to be an exact match for GlossaryTerm.language_code."]
    pub source_language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetLanguageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The BCP-47 language code for translation output, for example, \"zh-CN\". Expected to be an exact match for GlossaryTerm.language_code."]
    pub target_language_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Used with equivalent term set glossaries."]
pub struct LanguageCodesSet {
    #[serde(rename = "languageCodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The BCP-47 language code(s) for terms defined in the glossary. All entries are unique. The list contains at least two entries. Expected to be an exact match for GlossaryTerm.language_code."]
    pub language_codes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ListGlossaries."]
pub struct ListGlossariesResponse {
    #[serde(rename = "glossaries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of glossaries for a project."]
    pub glossaries: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Glossary>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token to retrieve a page of results. Pass this value in the [ListGlossariesRequest.page_token] field in the subsequent call to `ListGlossaries` method to retrieve the next page of results."]
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
#[doc = "The response message for Operations.ListOperations."]
pub struct ListOperationsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The standard List next-page token."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of operations that matches the specified filter in the request."]
    pub operations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Operation>>>,
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
#[doc = "Output configuration for BatchTranslateText request."]
pub struct OutputConfig {
    #[serde(rename = "gcsDestination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Google Cloud Storage destination for output content. For every single input file (for example, gs://a/b/c.[extension]), we generate at most 2 * n output files. (n is the # of target_language_codes in the BatchTranslateTextRequest). Output files (tsv) generated are compliant with RFC 4180 except that record delimiters are '\\n' instead of '\\r\\n'. We don't provide any way to change record delimiters. While the input files are being processed, we write/update an index file 'index.csv' under 'output_uri_prefix' (for example, gs://translation-test/index.csv) The index file is generated/updated as new files are being translated. The format is: input_file,target_language_code,translations_file,errors_file, glossary_translations_file,glossary_errors_file input_file is one file we matched using gcs_source.input_uri. target_language_code is provided in the request. translations_file contains the translations. (details provided below) errors_file contains the errors during processing of the file. (details below). Both translations_file and errors_file could be empty strings if we have no content to output. glossary_translations_file and glossary_errors_file are always empty strings if the input_file is tsv. They could also be empty if we have no content to output. Once a row is present in index.csv, the input/output matching never changes. Callers should also expect all the content in input_file are processed and ready to be consumed (that is, no partial output file is written). Since index.csv will be keeping updated during the process, please make sure there is no custom retention policy applied on the output bucket that may avoid file updating. (https://cloud.google.com/storage/docs/bucket-lock?hl=en#retention-policy) The format of translations_file (for target language code 'trg') is: gs://translation_test/a_b_c_'trg'_translations.[extension] If the input file extension is tsv, the output has the following columns: Column 1: ID of the request provided in the input, if it's not provided in the input, then the input row number is used (0-based). Column 2: source sentence. Column 3: translation without applying a glossary. Empty string if there is an error. Column 4 (only present if a glossary is provided in the request): translation after applying the glossary. Empty string if there is an error applying the glossary. Could be same string as column 3 if there is no glossary applied. If input file extension is a txt or html, the translation is directly written to the output file. If glossary is requested, a separate glossary_translations_file has format of gs://translation_test/a_b_c_'trg'_glossary_translations.[extension] The format of errors file (for target language code 'trg') is: gs://translation_test/a_b_c_'trg'_errors.[extension] If the input file extension is tsv, errors_file contains the following: Column 1: ID of the request provided in the input, if it's not provided in the input, then the input row number is used (0-based). Column 2: source sentence. Column 3: Error detail for the translation. Could be empty. Column 4 (only present if a glossary is provided in the request): Error when applying the glossary. If the input file extension is txt or html, glossary_error_file will be generated that contains error details. glossary_error_file has format of gs://translation_test/a_b_c_'trg'_glossary_errors.[extension]"]
    pub gcs_destination: ::std::option::Option<::std::boxed::Box<GcsDestination>>,
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
#[doc = "A single supported language response corresponds to information related to one supported language."]
pub struct SupportedLanguage {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Human readable name of the language localized in the display language specified in the request."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Supported language code, generally consisting of its ISO 639-1 identifier, for example, 'en', 'ja'. In certain cases, BCP-47 codes including language and region identifiers are returned (for example, 'zh-TW' and 'zh-CN')"]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "supportSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Can be used as source language."]
    pub support_source: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "supportTarget")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Can be used as target language."]
    pub support_target: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for discovering supported languages."]
pub struct SupportedLanguages {
    #[serde(rename = "languages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of supported language responses. This list contains an entry for each language the Translation API supports."]
    pub languages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SupportedLanguage>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configures which glossary should be used for a specific target language, and defines options for applying that glossary."]
pub struct TranslateTextGlossaryConfig {
    #[serde(rename = "glossary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Specifies the glossary used for this translation. Use this format: projects/*/locations/*/glossaries/*"]
    pub glossary: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ignoreCase")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Indicates match is case-insensitive. Default value is false if missing."]
    pub ignore_case: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for synchronous translation."]
pub struct TranslateTextRequest {
    #[serde(rename = "contents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The content of the input in string format. We recommend the total content be less than 30k codepoints. The max length of this field is 1024. Use BatchTranslateText for larger text."]
    pub contents: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "glossaryConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Glossary to be applied. The glossary must be within the same region (have the same location-id) as the model, otherwise an INVALID_ARGUMENT (400) error is returned."]
    pub glossary_config: ::std::option::Option<::std::boxed::Box<TranslateTextGlossaryConfig>>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The labels with user-defined metadata for the request. Label keys and values can be no longer than 63 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter. See https://cloud.google.com/translate/docs/labels for more information."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The format of the source text, for example, \"text/html\", \"text/plain\". If left blank, the MIME type defaults to \"text/html\"."]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "model")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The `model` type requested for this translation. The format depends on model type: - AutoML Translation models: `projects/{project-number-or-id}/locations/{location-id}/models/{model-id}` - General (built-in) models: `projects/{project-number-or-id}/locations/{location-id}/models/general/nmt`, `projects/{project-number-or-id}/locations/{location-id}/models/general/base` For global (non-regionalized) requests, use `location-id` `global`. For example, `projects/{project-number-or-id}/locations/global/models/general/nmt`. If missing, the system decides which google base model to use."]
    pub model: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sourceLanguageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The BCP-47 language code of the input text if known, for example, \"en-US\" or \"sr-Latn\". Supported language codes are listed in Language Support. If the source language isn't specified, the API attempts to identify the source language automatically and returns the source language within the response."]
    pub source_language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetLanguageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The BCP-47 language code to use for translation of the input text, set to one of the language codes listed in Language Support."]
    pub target_language_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TranslateTextResponse {
    #[serde(rename = "glossaryTranslations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Text translation responses if a glossary is provided in the request. This can be the same as `translations` if no terms apply. This field has the same length as `contents`."]
    pub glossary_translations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Translation>>>,
    #[serde(rename = "translations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Text translation responses with no glossary applied. This field has the same length as `contents`."]
    pub translations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Translation>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A single translation response."]
pub struct Translation {
    #[serde(rename = "detectedLanguageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The BCP-47 language code of source text in the initial request, detected automatically, if no source language was passed within the initial request. If the source language was passed, auto-detection of the language does not occur and this field is empty."]
    pub detected_language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "glossaryConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `glossary_config` used for this translation."]
    pub glossary_config: ::std::option::Option<::std::boxed::Box<TranslateTextGlossaryConfig>>,
    #[serde(rename = "model")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only present when `model` is present in the request. `model` here is normalized to have project number. For example: If the `model` requested in TranslationTextRequest is `projects/{project-id}/locations/{location-id}/models/general/nmt` then `model` here would be normalized to `projects/{project-number}/locations/{location-id}/models/general/nmt`."]
    pub model: ::std::option::Option<::std::string::String>,
    #[serde(rename = "translatedText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Text translated into the target language."]
    pub translated_text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for Operations.WaitOperation."]
pub struct WaitOperationRequest {
    #[serde(rename = "timeout")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum duration to wait before timing out. If left blank, the wait will be at most the time permitted by the underlying HTTP/RPC protocol. If RPC context deadline is also specified, the shorter one will be used."]
    pub timeout: ::std::option::Option<::std::string::String>,
}
