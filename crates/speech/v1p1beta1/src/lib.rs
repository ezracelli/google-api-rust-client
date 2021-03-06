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
        serde_json::from_str(&"\"json\"").unwrap()
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "filter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The standard list filter."]
                    pub filter: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "name")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The name of the operation's parent resource."]
                    pub name: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The standard list page size."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
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
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An item of the class."]
    pub struct ClassItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The class item's value."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl ClassItem {
        pub fn builder() -> ClassItemBuilder {
            ClassItemBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A set of words or phrases that represents a common concept likely to appear in your audio, for example a list of passenger ship names. CustomClass items can be substituted into placeholders that you set in PhraseSet phrases."]
    pub struct CustomClass {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customClassId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If this custom class is a resource, the custom_class_id is the resource id of the CustomClass. Case sensitive."]
        pub custom_class_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A collection of class items."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ClassItem>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the custom class."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl CustomClass {
        pub fn builder() -> CustomClassBuilder {
            CustomClassBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Operations.ListOperations."]
    pub struct ListOperationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The standard List next-page token."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of operations that matches the specified filter in the request."]
        pub operations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Operation>>>,
    }
    impl ListOperationsResponse {
        pub fn builder() -> ListOperationsResponseBuilder {
            ListOperationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes the progress of a long-running `LongRunningRecognize` call. It is included in the `metadata` field of the `Operation` returned by the `GetOperation` call of the `google::longrunning::Operations` service."]
    pub struct LongRunningRecognizeMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastUpdateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time of the most recent processing update."]
        pub last_update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressPercent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Approximate percentage of audio processed thus far. Guaranteed to be 100 when the audio is fully processed and the results are available."]
        pub progress_percent: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time when the request was received."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The URI of the audio file being transcribed. Empty if the audio was sent as byte content."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl LongRunningRecognizeMetadata {
        pub fn builder() -> LongRunningRecognizeMetadataBuilder {
            LongRunningRecognizeMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The top-level message sent by the client for the `LongRunningRecognize` method."]
    pub struct LongRunningRecognizeRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The audio data to be recognized."]
        pub audio: ::std::option::Option<::std::boxed::Box<RecognitionAudio>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "config")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Provides information to the recognizer that specifies how to process the request."]
        pub config: ::std::option::Option<::std::boxed::Box<RecognitionConfig>>,
    }
    impl LongRunningRecognizeRequest {
        pub fn builder() -> LongRunningRecognizeRequestBuilder {
            LongRunningRecognizeRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The only message returned to the client by the `LongRunningRecognize` method. It contains the result as zero or more sequential `SpeechRecognitionResult` messages. It is included in the `result.response` field of the `Operation` returned by the `GetOperation` call of the `google::longrunning::Operations` service."]
    pub struct LongRunningRecognizeResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sequential list of transcription results corresponding to sequential portions of audio."]
        pub results:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SpeechRecognitionResult>>>,
    }
    impl LongRunningRecognizeResponse {
        pub fn builder() -> LongRunningRecognizeResponseBuilder {
            LongRunningRecognizeResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This resource represents a long-running operation that is the result of a network API call."]
    pub struct Operation {
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
    impl Operation {
        pub fn builder() -> OperationBuilder {
            OperationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A phrases containing words and phrase \"hints\" so that the speech recognition is more likely to recognize them. This can be used to improve the accuracy for specific words and phrases, for example, if specific commands are typically spoken by the user. This can also be used to add additional words to the vocabulary of the recognizer. See [usage limits](https://cloud.google.com/speech-to-text/quotas#content). List items can also include pre-built or custom classes containing groups of words that represent common concepts that occur in natural language. For example, rather than providing a phrase hint for every month of the year (e.g. \"i was born in january\", \"i was born in febuary\", ...), use the pre-built `$MONTH` class improves the likelihood of correctly transcribing audio that includes months (e.g. \"i was born in $month\"). To refer to pre-built classes, use the class' symbol prepended with `$` e.g. `$MONTH`. To refer to custom classes that were defined inline in the request, set the class's `custom_class_id` to a string unique to all class resources and inline classes. Then use the class' id wrapped in $`{...}` e.g. \"${my-months}\". To refer to custom classes resources, use the class' id wrapped in `${}` (e.g. `${my-months}`)."]
    pub struct Phrase {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boost")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hint Boost. Overrides the boost set at the phrase set level. Positive value will increase the probability that a specific phrase will be recognized over other similar sounding phrases. The higher the boost, the higher the chance of false positive recognition as well. Negative boost values would correspond to anti-biasing. Anti-biasing is not enabled, so negative boost will simply be ignored. Though `boost` can accept a wide range of positive values, most use cases are best served with values between 0 and 20. We recommend using a binary search approach to finding the optimal value for your use case. Speech recognition will skip PhraseSets with a boost value of 0."]
        pub boost: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The phrase itself."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl Phrase {
        pub fn builder() -> PhraseBuilder {
            PhraseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Provides \"hints\" to the speech recognizer to favor specific words and phrases in the results."]
    pub struct PhraseSet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boost")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hint Boost. Positive value will increase the probability that a specific phrase will be recognized over other similar sounding phrases. The higher the boost, the higher the chance of false positive recognition as well. Negative boost values would correspond to anti-biasing. Anti-biasing is not enabled, so negative boost will simply be ignored. Though `boost` can accept a wide range of positive values, most use cases are best served with values between 0 (exclusive) and 20. We recommend using a binary search approach to finding the optimal value for your use case. Speech recognition will skip PhraseSets with a boost value of 0."]
        pub boost: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the phrase set."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phrases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of word and phrases."]
        pub phrases: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Phrase>>>,
    }
    impl PhraseSet {
        pub fn builder() -> PhraseSetBuilder {
            PhraseSetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains audio data in the encoding specified in the `RecognitionConfig`. Either `content` or `uri` must be supplied. Supplying both or neither returns google.rpc.Code.INVALID_ARGUMENT. See [content limits](https://cloud.google.com/speech-to-text/quotas#content)."]
    pub struct RecognitionAudio {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The audio data bytes encoded as specified in `RecognitionConfig`. Note: as with all bytes fields, proto buffers use a pure binary representation, whereas JSON representations use base64."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URI that points to a file that contains audio data bytes as specified in `RecognitionConfig`. The file must not be compressed (for example, gzip). Currently, only Google Cloud Storage URIs are supported, which must be specified in the following format: `gs://bucket_name/object_name` (other URI formats return google.rpc.Code.INVALID_ARGUMENT). For more information, see [Request URIs](https://cloud.google.com/storage/docs/reference-uris)."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl RecognitionAudio {
        pub fn builder() -> RecognitionAudioBuilder {
            RecognitionAudioBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Provides information to the recognizer that specifies how to process the request."]
    pub struct RecognitionConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adaptation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Speech adaptation configuration improves the accuracy of speech recognition. When speech adaptation is set it supersedes the `speech_contexts` field. For more information, see the [speech adaptation](https://cloud.google.com/speech-to-text/docs/context-strength) documentation."]
        pub adaptation: ::std::option::Option<::std::boxed::Box<SpeechAdaptation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alternativeLanguageCodes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of up to 3 additional [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tags, listing possible alternative languages of the supplied audio. See [Language Support](https://cloud.google.com/speech-to-text/docs/languages) for a list of the currently supported language codes. If alternative languages are listed, recognition result will contain recognition in the most likely language detected including the main language_code. The recognition result will include the language tag of the language detected in the audio. Note: This feature is only supported for Voice Command and Voice Search use cases and performance may vary for other use cases (e.g., phone call transcription)."]
        pub alternative_language_codes:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audioChannelCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of channels in the input audio data. ONLY set this for MULTI-CHANNEL recognition. Valid values for LINEAR16 and FLAC are `1`-`8`. Valid values for OGG_OPUS are '1'-'254'. Valid value for MULAW, AMR, AMR_WB and SPEEX_WITH_HEADER_BYTE is only `1`. If `0` or omitted, defaults to one channel (mono). Note: We only recognize the first channel by default. To perform independent recognition on each channel set `enable_separate_recognition_per_channel` to 'true'."]
        pub audio_channel_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diarizationConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Config to enable speaker diarization and set additional parameters to make diarization better suited for your application. Note: When this is enabled, we send all the words from the beginning of the audio for the top alternative in every consecutive STREAMING responses. This is done in order to improve our speaker tags as our models learn to identify the speakers in the conversation over time. For non-streaming requests, the diarization results will be provided only in the top alternative of the FINAL SpeechRecognitionResult."]
        pub diarization_config: ::std::option::Option<::std::boxed::Box<SpeakerDiarizationConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diarizationSpeakerCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, specifies the estimated number of speakers in the conversation. Defaults to '2'. Ignored unless enable_speaker_diarization is set to true. Note: Use diarization_config instead."]
        pub diarization_speaker_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableAutomaticPunctuation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If 'true', adds punctuation to recognition result hypotheses. This feature is only available in select languages. Setting this for requests in other languages has no effect at all. The default 'false' value does not add punctuation to result hypotheses."]
        pub enable_automatic_punctuation: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableSeparateRecognitionPerChannel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This needs to be set to `true` explicitly and `audio_channel_count` > 1 to get each channel recognized separately. The recognition result will contain a `channel_tag` field to state which channel that result belongs to. If this is not true, we will only recognize the first channel. The request is billed cumulatively for all channels recognized: `audio_channel_count` multiplied by the length of the audio."]
        pub enable_separate_recognition_per_channel: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableSpeakerDiarization")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If 'true', enables speaker detection for each recognized word in the top alternative of the recognition result using a speaker_tag provided in the WordInfo. Note: Use diarization_config instead."]
        pub enable_speaker_diarization: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableWordConfidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If `true`, the top result includes a list of words and the confidence for those words. If `false`, no word-level confidence information is returned. The default is `false`."]
        pub enable_word_confidence: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableWordTimeOffsets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If `true`, the top result includes a list of words and the start and end time offsets (timestamps) for those words. If `false`, no word-level time offset information is returned. The default is `false`."]
        pub enable_word_time_offsets: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "encoding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Encoding of audio data sent in all `RecognitionAudio` messages. This field is optional for `FLAC` and `WAV` audio files and required for all other audio formats. For details, see AudioEncoding."]
        pub encoding: ::std::option::Option<RecognitionConfigEncodingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The language of the supplied audio as a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. Example: \"en-US\". See [Language Support](https://cloud.google.com/speech-to-text/docs/languages) for a list of the currently supported language codes."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxAlternatives")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum number of recognition hypotheses to be returned. Specifically, the maximum number of `SpeechRecognitionAlternative` messages within each `SpeechRecognitionResult`. The server may return fewer than `max_alternatives`. Valid values are `0`-`30`. A value of `0` or `1` will return a maximum of one. If omitted, will return a maximum of one."]
        pub max_alternatives: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata regarding this request."]
        pub metadata: ::std::option::Option<::std::boxed::Box<RecognitionMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "model")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Which model to select for the given request. Select the model best suited to your domain to get best results. If a model is not explicitly specified, then we auto-select a model based on the parameters in the RecognitionConfig. *Model* *Description* command_and_search Best for short queries such as voice commands or voice search. phone_call Best for audio that originated from a phone call (typically recorded at an 8khz sampling rate). video Best for audio that originated from from video or includes multiple speakers. Ideally the audio is recorded at a 16khz or greater sampling rate. This is a premium model that costs more than the standard rate. default Best for audio that is not one of the specific audio models. For example, long-form audio. Ideally the audio is high-fidelity, recorded at a 16khz or greater sampling rate. "]
        pub model: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profanityFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set to `true`, the server will attempt to filter out profanities, replacing all but the initial character in each filtered word with asterisks, e.g. \"f***\". If set to `false` or omitted, profanities won't be filtered out."]
        pub profanity_filter: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sampleRateHertz")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sample rate in Hertz of the audio data sent in all `RecognitionAudio` messages. Valid values are: 8000-48000. 16000 is optimal. For best results, set the sampling rate of the audio source to 16000 Hz. If that's not possible, use the native sample rate of the audio source (instead of re-sampling). This field is optional for FLAC and WAV audio files, but is required for all other audio formats. For details, see AudioEncoding."]
        pub sample_rate_hertz: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "speechContexts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Array of SpeechContext. A means to provide context to assist the speech recognition. For more information, see [speech adaptation](https://cloud.google.com/speech-to-text/docs/context-strength)."]
        pub speech_contexts:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SpeechContext>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "useEnhanced")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set to true to use an enhanced model for speech recognition. If `use_enhanced` is set to true and the `model` field is not set, then an appropriate enhanced model is chosen if an enhanced model exists for the audio. If `use_enhanced` is true and an enhanced version of the specified model does not exist, then the speech is recognized using the standard version of the specified model."]
        pub use_enhanced: ::std::option::Option<::std::primitive::bool>,
    }
    impl RecognitionConfig {
        pub fn builder() -> RecognitionConfigBuilder {
            RecognitionConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Encoding of audio data sent in all `RecognitionAudio` messages. This field is optional for `FLAC` and `WAV` audio files and required for all other audio formats. For details, see AudioEncoding."]
    pub enum RecognitionConfigEncodingEnum {
        #[serde(rename = "ENCODING_UNSPECIFIED")]
        #[doc = "Not specified."]
        EncodingUnspecified,
        #[serde(rename = "LINEAR16")]
        #[doc = "Uncompressed 16-bit signed little-endian samples (Linear PCM)."]
        Linear16,
        #[serde(rename = "FLAC")]
        #[doc = "`FLAC` (Free Lossless Audio Codec) is the recommended encoding because it is lossless--therefore recognition is not compromised--and requires only about half the bandwidth of `LINEAR16`. `FLAC` stream encoding supports 16-bit and 24-bit samples, however, not all fields in `STREAMINFO` are supported."]
        Flac,
        #[serde(rename = "MULAW")]
        #[doc = "8-bit samples that compand 14-bit audio samples using G.711 PCMU/mu-law."]
        Mulaw,
        #[serde(rename = "AMR")]
        #[doc = "Adaptive Multi-Rate Narrowband codec. `sample_rate_hertz` must be 8000."]
        Amr,
        #[serde(rename = "AMR_WB")]
        #[doc = "Adaptive Multi-Rate Wideband codec. `sample_rate_hertz` must be 16000."]
        AmrWb,
        #[serde(rename = "OGG_OPUS")]
        #[doc = "Opus encoded audio frames in Ogg container ([OggOpus](https://wiki.xiph.org/OggOpus)). `sample_rate_hertz` must be one of 8000, 12000, 16000, 24000, or 48000."]
        OggOpus,
        #[serde(rename = "SPEEX_WITH_HEADER_BYTE")]
        #[doc = "Although the use of lossy encodings is not recommended, if a very low bitrate encoding is required, `OGG_OPUS` is highly preferred over Speex encoding. The [Speex](https://speex.org/) encoding supported by Cloud Speech API has a header byte in each block, as in MIME type `audio/x-speex-with-header-byte`. It is a variant of the RTP Speex encoding defined in [RFC 5574](https://tools.ietf.org/html/rfc5574). The stream is a sequence of blocks, one block per RTP packet. Each block starts with a byte containing the length of the block, in bytes, followed by one or more frames of Speex data, padded to an integral number of bytes (octets) as specified in RFC 5574. In other words, each RTP header is replaced with a single byte containing the block length. Only Speex wideband is supported. `sample_rate_hertz` must be 16000."]
        SpeexWithHeaderByte,
        #[serde(rename = "MP3")]
        #[doc = "MP3 audio. MP3 encoding is a Beta feature and only available in v1p1beta1. Support all standard MP3 bitrates (which range from 32-320 kbps). When using this encoding, `sample_rate_hertz` has to match the sample rate of the file being used."]
        Mp3,
    }
    impl ::std::default::Default for RecognitionConfigEncodingEnum {
        fn default() -> Self {
            Self::EncodingUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Description of audio data to be recognized."]
    pub struct RecognitionMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audioTopic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the content. Eg. \"Recordings of federal supreme court hearings from 2012\"."]
        pub audio_topic: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "industryNaicsCodeOfAudio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The industry vertical to which this speech recognition request most closely applies. This is most indicative of the topics contained in the audio. Use the 6-digit NAICS code to identify the industry vertical - see https://www.naics.com/search/."]
        pub industry_naics_code_of_audio: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "interactionType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The use case most closely describing the audio content to be recognized."]
        pub interaction_type: ::std::option::Option<RecognitionMetadataInteractionTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "microphoneDistance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The audio type that most closely describes the audio being recognized."]
        pub microphone_distance: ::std::option::Option<RecognitionMetadataMicrophoneDistanceEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "obfuscatedId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Obfuscated (privacy-protected) ID of the user, to identify number of unique users using the service."]
        pub obfuscated_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalMediaType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The original media the speech was recorded on."]
        pub original_media_type: ::std::option::Option<RecognitionMetadataOriginalMediaTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalMimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mime type of the original audio file. For example `audio/m4a`, `audio/x-alaw-basic`, `audio/mp3`, `audio/3gpp`. A list of possible audio mime types is maintained at http://www.iana.org/assignments/media-types/media-types.xhtml#audio"]
        pub original_mime_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recordingDeviceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device used to make the recording. Examples 'Nexus 5X' or 'Polycom SoundStation IP 6000' or 'POTS' or 'VoIP' or 'Cardioid Microphone'."]
        pub recording_device_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recordingDeviceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of device the speech was recorded with."]
        pub recording_device_type:
            ::std::option::Option<RecognitionMetadataRecordingDeviceTypeEnum>,
    }
    impl RecognitionMetadata {
        pub fn builder() -> RecognitionMetadataBuilder {
            RecognitionMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The use case most closely describing the audio content to be recognized."]
    pub enum RecognitionMetadataInteractionTypeEnum {
        #[serde(rename = "INTERACTION_TYPE_UNSPECIFIED")]
        #[doc = "Use case is either unknown or is something other than one of the other values below."]
        InteractionTypeUnspecified,
        #[serde(rename = "DISCUSSION")]
        #[doc = "Multiple people in a conversation or discussion. For example in a meeting with two or more people actively participating. Typically all the primary people speaking would be in the same room (if not, see PHONE_CALL)"]
        Discussion,
        #[serde(rename = "PRESENTATION")]
        #[doc = "One or more persons lecturing or presenting to others, mostly uninterrupted."]
        Presentation,
        #[serde(rename = "PHONE_CALL")]
        #[doc = "A phone-call or video-conference in which two or more people, who are not in the same room, are actively participating."]
        PhoneCall,
        #[serde(rename = "VOICEMAIL")]
        #[doc = "A recorded message intended for another person to listen to."]
        Voicemail,
        #[serde(rename = "PROFESSIONALLY_PRODUCED")]
        #[doc = "Professionally produced audio (eg. TV Show, Podcast)."]
        ProfessionallyProduced,
        #[serde(rename = "VOICE_SEARCH")]
        #[doc = "Transcribe spoken questions and queries into text."]
        VoiceSearch,
        #[serde(rename = "VOICE_COMMAND")]
        #[doc = "Transcribe voice commands, such as for controlling a device."]
        VoiceCommand,
        #[serde(rename = "DICTATION")]
        #[doc = "Transcribe speech to text to create a written document, such as a text-message, email or report."]
        Dictation,
    }
    impl ::std::default::Default for RecognitionMetadataInteractionTypeEnum {
        fn default() -> Self {
            Self::InteractionTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The audio type that most closely describes the audio being recognized."]
    pub enum RecognitionMetadataMicrophoneDistanceEnum {
        #[serde(rename = "MICROPHONE_DISTANCE_UNSPECIFIED")]
        #[doc = "Audio type is not known."]
        MicrophoneDistanceUnspecified,
        #[serde(rename = "NEARFIELD")]
        #[doc = "The audio was captured from a closely placed microphone. Eg. phone, dictaphone, or handheld microphone. Generally if there speaker is within 1 meter of the microphone."]
        Nearfield,
        #[serde(rename = "MIDFIELD")]
        #[doc = "The speaker if within 3 meters of the microphone."]
        Midfield,
        #[serde(rename = "FARFIELD")]
        #[doc = "The speaker is more than 3 meters away from the microphone."]
        Farfield,
    }
    impl ::std::default::Default for RecognitionMetadataMicrophoneDistanceEnum {
        fn default() -> Self {
            Self::MicrophoneDistanceUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The original media the speech was recorded on."]
    pub enum RecognitionMetadataOriginalMediaTypeEnum {
        #[serde(rename = "ORIGINAL_MEDIA_TYPE_UNSPECIFIED")]
        #[doc = "Unknown original media type."]
        OriginalMediaTypeUnspecified,
        #[serde(rename = "AUDIO")]
        #[doc = "The speech data is an audio recording."]
        Audio,
        #[serde(rename = "VIDEO")]
        #[doc = "The speech data originally recorded on a video."]
        Video,
    }
    impl ::std::default::Default for RecognitionMetadataOriginalMediaTypeEnum {
        fn default() -> Self {
            Self::OriginalMediaTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of device the speech was recorded with."]
    pub enum RecognitionMetadataRecordingDeviceTypeEnum {
        #[serde(rename = "RECORDING_DEVICE_TYPE_UNSPECIFIED")]
        #[doc = "The recording device is unknown."]
        RecordingDeviceTypeUnspecified,
        #[serde(rename = "SMARTPHONE")]
        #[doc = "Speech was recorded on a smartphone."]
        Smartphone,
        #[serde(rename = "PC")]
        #[doc = "Speech was recorded using a personal computer or tablet."]
        Pc,
        #[serde(rename = "PHONE_LINE")]
        #[doc = "Speech was recorded over a phone line."]
        PhoneLine,
        #[serde(rename = "VEHICLE")]
        #[doc = "Speech was recorded in a vehicle."]
        Vehicle,
        #[serde(rename = "OTHER_OUTDOOR_DEVICE")]
        #[doc = "Speech was recorded outdoors."]
        OtherOutdoorDevice,
        #[serde(rename = "OTHER_INDOOR_DEVICE")]
        #[doc = "Speech was recorded indoors."]
        OtherIndoorDevice,
    }
    impl ::std::default::Default for RecognitionMetadataRecordingDeviceTypeEnum {
        fn default() -> Self {
            Self::RecordingDeviceTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The top-level message sent by the client for the `Recognize` method."]
    pub struct RecognizeRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The audio data to be recognized."]
        pub audio: ::std::option::Option<::std::boxed::Box<RecognitionAudio>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "config")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Provides information to the recognizer that specifies how to process the request."]
        pub config: ::std::option::Option<::std::boxed::Box<RecognitionConfig>>,
    }
    impl RecognizeRequest {
        pub fn builder() -> RecognizeRequestBuilder {
            RecognizeRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The only message returned to the client by the `Recognize` method. It contains the result as zero or more sequential `SpeechRecognitionResult` messages."]
    pub struct RecognizeResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sequential list of transcription results corresponding to sequential portions of audio."]
        pub results:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SpeechRecognitionResult>>>,
    }
    impl RecognizeResponse {
        pub fn builder() -> RecognizeResponseBuilder {
            RecognizeResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Config to enable speaker diarization."]
    pub struct SpeakerDiarizationConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableSpeakerDiarization")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If 'true', enables speaker detection for each recognized word in the top alternative of the recognition result using a speaker_tag provided in the WordInfo."]
        pub enable_speaker_diarization: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxSpeakerCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum number of speakers in the conversation. This range gives you more flexibility by allowing the system to automatically determine the correct number of speakers. If not set, the default value is 6."]
        pub max_speaker_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minSpeakerCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum number of speakers in the conversation. This range gives you more flexibility by allowing the system to automatically determine the correct number of speakers. If not set, the default value is 2."]
        pub min_speaker_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "speakerTag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Unused."]
        pub speaker_tag: ::std::option::Option<::std::primitive::i64>,
    }
    impl SpeakerDiarizationConfig {
        pub fn builder() -> SpeakerDiarizationConfigBuilder {
            SpeakerDiarizationConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Speech adaptation configuration."]
    pub struct SpeechAdaptation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customClasses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A collection of custom classes. To specify the classes inline, leave the class' `name` blank and fill in the rest of its fields, giving it a unique `custom_class_id`. Refer to the inline defined class in phrase hints by its `custom_class_id`."]
        pub custom_classes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomClass>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phraseSets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A collection of phrase sets. To specify the hints inline, leave the phrase set's `name` blank and fill in the rest of its fields. Any phrase set can use any custom class."]
        pub phrase_sets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PhraseSet>>>,
    }
    impl SpeechAdaptation {
        pub fn builder() -> SpeechAdaptationBuilder {
            SpeechAdaptationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Provides \"hints\" to the speech recognizer to favor specific words and phrases in the results."]
    pub struct SpeechContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boost")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hint Boost. Positive value will increase the probability that a specific phrase will be recognized over other similar sounding phrases. The higher the boost, the higher the chance of false positive recognition as well. Negative boost values would correspond to anti-biasing. Anti-biasing is not enabled, so negative boost will simply be ignored. Though `boost` can accept a wide range of positive values, most use cases are best served with values between 0 and 20. We recommend using a binary search approach to finding the optimal value for your use case."]
        pub boost: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phrases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of strings containing words and phrases \"hints\" so that the speech recognition is more likely to recognize them. This can be used to improve the accuracy for specific words and phrases, for example, if specific commands are typically spoken by the user. This can also be used to add additional words to the vocabulary of the recognizer. See [usage limits](https://cloud.google.com/speech-to-text/quotas#content). List items can also be set to classes for groups of words that represent common concepts that occur in natural language. For example, rather than providing phrase hints for every month of the year, using the $MONTH class improves the likelihood of correctly transcribing audio that includes months."]
        pub phrases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl SpeechContext {
        pub fn builder() -> SpeechContextBuilder {
            SpeechContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Alternative hypotheses (a.k.a. n-best list)."]
    pub struct SpeechRecognitionAlternative {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative of a non-streaming result or, of a streaming result where `is_final=true`. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transcript")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Transcript text representing the words that the user spoke."]
        pub transcript: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "words")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of word-specific information for each recognized word. Note: When `enable_speaker_diarization` is true, you will see all the words from the beginning of the audio."]
        pub words: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WordInfo>>>,
    }
    impl SpeechRecognitionAlternative {
        pub fn builder() -> SpeechRecognitionAlternativeBuilder {
            SpeechRecognitionAlternativeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A speech recognition result corresponding to a portion of the audio."]
    pub struct SpeechRecognitionResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alternatives")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "May contain one or more recognition hypotheses (up to the maximum specified in `max_alternatives`). These alternatives are ordered in terms of accuracy, with the top (first) alternative being the most probable, as ranked by the recognizer."]
        pub alternatives:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SpeechRecognitionAlternative>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelTag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For multi-channel audio, this is the channel number corresponding to the recognized result for the audio from that channel. For audio_channel_count = N, its output values can range from '1' to 'N'."]
        pub channel_tag: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag of the language in this result. This language code was detected to have the most likelihood of being spoken in the audio."]
        pub language_code: ::std::option::Option<::std::string::String>,
    }
    impl SpeechRecognitionResult {
        pub fn builder() -> SpeechRecognitionResultBuilder {
            SpeechRecognitionResultBuilder::default()
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
    #[doc = "Word-specific information for recognized words."]
    pub struct WordInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative of a non-streaming result or, of a streaming result where `is_final=true`. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time offset relative to the beginning of the audio, and corresponding to the end of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "speakerTag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A distinct integer value is assigned for every speaker within the audio. This field specifies which one of those speakers was detected to have spoken this word. Value ranges from '1' to diarization_speaker_count. speaker_tag is set if enable_speaker_diarization = 'true' and only in the top alternative."]
        pub speaker_tag: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time offset relative to the beginning of the audio, and corresponding to the start of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "word")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The word corresponding to this set of information."]
        pub word: ::std::option::Option<::std::string::String>,
    }
    impl WordInfo {
        pub fn builder() -> WordInfoBuilder {
            WordInfoBuilder::default()
        }
    }
}
