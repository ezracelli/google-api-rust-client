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
    pub mod projects {
        pub mod resources {
            pub mod locations {
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
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "filter")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The standard list filter."]
                                    pub filter: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The standard list page size."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
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
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video annotation progress. Included in the `metadata` field of the `Operation` returned by the `GetOperation` call of the `google::longrunning::Operations` service."]
    pub struct GoogleCloudVideointelligenceV1AnnotateVideoProgress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationProgress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Progress metadata for all videos specified in `AnnotateVideoRequest`."]
        pub annotation_progress: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1VideoAnnotationProgress>,
            >,
        >,
    }
    impl GoogleCloudVideointelligenceV1AnnotateVideoProgress {
        pub fn builder() -> GoogleCloudVideointelligenceV1AnnotateVideoProgressBuilder {
            GoogleCloudVideointelligenceV1AnnotateVideoProgressBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video annotation request."]
    pub struct GoogleCloudVideointelligenceV1AnnotateVideoRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "features")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Requested video annotation features."]
        pub features: ::std::option::Option<
            ::std::vec::Vec<GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video data bytes. If unset, the input video(s) should be specified via the `input_uri`. If set, `input_uri` must be unset."]
        pub input_content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Input video location. Currently, only [Cloud Storage](https://cloud.google.com/storage/) URIs are supported. URIs must be specified in the following format: `gs://bucket-id/object-id` (other URI formats return google.rpc.Code.INVALID_ARGUMENT). For more information, see [Request URIs](https://cloud.google.com/storage/docs/request-endpoints). To identify multiple videos, a video URI may include wildcards in the `object-id`. Supported wildcards: '*' to match 0 or more characters; '?' to match 1 character. If unset, the input video should be embedded in the request as `input_content`. If set, `input_content` must be unset."]
        pub input_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Cloud region where annotation should take place. Supported cloud regions are: `us-east1`, `us-west1`, `europe-west1`, `asia-east1`. If no region is specified, the region will be determined based on video file location."]
        pub location_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Location where the output (in JSON format) should be stored. Currently, only [Cloud Storage](https://cloud.google.com/storage/) URIs are supported. These must be specified in the following format: `gs://bucket-id/object-id` (other URI formats return google.rpc.Code.INVALID_ARGUMENT). For more information, see [Request URIs](https://cloud.google.com/storage/docs/request-endpoints)."]
        pub output_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional video context and/or feature-specific parameters."]
        pub video_context:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1VideoContext>>,
    }
    impl GoogleCloudVideointelligenceV1AnnotateVideoRequest {
        pub fn builder() -> GoogleCloudVideointelligenceV1AnnotateVideoRequestBuilder {
            GoogleCloudVideointelligenceV1AnnotateVideoRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum {
        #[serde(rename = "FEATURE_UNSPECIFIED")]
        #[doc = "Unspecified."]
        FeatureUnspecified,
        #[serde(rename = "LABEL_DETECTION")]
        #[doc = "Label detection. Detect objects, such as dog or flower."]
        LabelDetection,
        #[serde(rename = "SHOT_CHANGE_DETECTION")]
        #[doc = "Shot change detection."]
        ShotChangeDetection,
        #[serde(rename = "EXPLICIT_CONTENT_DETECTION")]
        #[doc = "Explicit content detection."]
        ExplicitContentDetection,
        #[serde(rename = "FACE_DETECTION")]
        #[doc = "Human face detection."]
        FaceDetection,
        #[serde(rename = "SPEECH_TRANSCRIPTION")]
        #[doc = "Speech transcription."]
        SpeechTranscription,
        #[serde(rename = "TEXT_DETECTION")]
        #[doc = "OCR text detection and tracking."]
        TextDetection,
        #[serde(rename = "OBJECT_TRACKING")]
        #[doc = "Object detection and tracking."]
        ObjectTracking,
        #[serde(rename = "LOGO_RECOGNITION")]
        #[doc = "Logo detection, tracking, and recognition."]
        LogoRecognition,
        #[serde(rename = "PERSON_DETECTION")]
        #[doc = "Person detection."]
        PersonDetection,
    }
    impl ::std::default::Default for GoogleCloudVideointelligenceV1AnnotateVideoRequestFeaturesEnum {
        fn default() -> Self {
            Self::FeatureUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video annotation response. Included in the `response` field of the `Operation` returned by the `GetOperation` call of the `google::longrunning::Operations` service."]
    pub struct GoogleCloudVideointelligenceV1AnnotateVideoResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Annotation results for all videos specified in `AnnotateVideoRequest`."]
        pub annotation_results: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1VideoAnnotationResults>,
            >,
        >,
    }
    impl GoogleCloudVideointelligenceV1AnnotateVideoResponse {
        pub fn builder() -> GoogleCloudVideointelligenceV1AnnotateVideoResponseBuilder {
            GoogleCloudVideointelligenceV1AnnotateVideoResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic detected attribute represented by name in string format."]
    pub struct GoogleCloudVideointelligenceV1DetectedAttribute {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected attribute confidence. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the attribute, for example, glasses, dark_glasses, mouth_open. A full list of supported type names will be provided in the document."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text value of the detection result. For example, the value for \"HairColor\" can be \"black\", \"blonde\", etc."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1DetectedAttribute {
        pub fn builder() -> GoogleCloudVideointelligenceV1DetectedAttributeBuilder {
            GoogleCloudVideointelligenceV1DetectedAttributeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic detected landmark represented by name in string format and a 2D location."]
    pub struct GoogleCloudVideointelligenceV1DetectedLandmark {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The confidence score of the detected landmark. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of this landmark, for example, left_hand, right_shoulder."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "point")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The 2D point of the detected landmark using the normalized image coordindate system. The normalized coordinates have the range from 0 to 1."]
        pub point: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1NormalizedVertex>,
        >,
    }
    impl GoogleCloudVideointelligenceV1DetectedLandmark {
        pub fn builder() -> GoogleCloudVideointelligenceV1DetectedLandmarkBuilder {
            GoogleCloudVideointelligenceV1DetectedLandmarkBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected entity from video analysis."]
    pub struct GoogleCloudVideointelligenceV1Entity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Textual description, e.g., `Fixed-gear bicycle`."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Opaque entity ID. Some IDs may be available in [Google Knowledge Graph Search API](https://developers.google.com/knowledge-graph/)."]
        pub entity_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Language code for `description` in BCP-47 format."]
        pub language_code: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1Entity {
        pub fn builder() -> GoogleCloudVideointelligenceV1EntityBuilder {
            GoogleCloudVideointelligenceV1EntityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Explicit content annotation (based on per-frame visual signals only). If no explicit content has been detected in a frame, no annotations are present for that frame."]
    pub struct GoogleCloudVideointelligenceV1ExplicitContentAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video frames where explicit content was detected."]
        pub frames: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1ExplicitContentFrame>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1ExplicitContentAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1ExplicitContentAnnotationBuilder {
            GoogleCloudVideointelligenceV1ExplicitContentAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Config for EXPLICIT_CONTENT_DETECTION."]
    pub struct GoogleCloudVideointelligenceV1ExplicitContentDetectionConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "model")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Model to use for explicit content detection. Supported values: \"builtin/stable\" (the default if unset) and \"builtin/latest\"."]
        pub model: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1ExplicitContentDetectionConfig {
        pub fn builder() -> GoogleCloudVideointelligenceV1ExplicitContentDetectionConfigBuilder {
            GoogleCloudVideointelligenceV1ExplicitContentDetectionConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video frame level annotation results for explicit content."]
    pub struct GoogleCloudVideointelligenceV1ExplicitContentFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pornographyLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Likelihood of the pornography content.."]
        pub pornography_likelihood: ::std::option::Option<
            GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihoodEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1ExplicitContentFrame {
        pub fn builder() -> GoogleCloudVideointelligenceV1ExplicitContentFrameBuilder {
            GoogleCloudVideointelligenceV1ExplicitContentFrameBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Likelihood of the pornography content.."]
    pub enum GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihoodEnum {
        #[serde(rename = "LIKELIHOOD_UNSPECIFIED")]
        #[doc = "Unspecified likelihood."]
        LikelihoodUnspecified,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "Very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "Unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "Possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "Likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "Very likely."]
        VeryLikely,
    }
    impl ::std::default::Default
        for GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihoodEnum
    {
        fn default() -> Self {
            Self::LikelihoodUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deprecated. No effect."]
    pub struct GoogleCloudVideointelligenceV1FaceAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video frames where a face was detected."]
        pub frames: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1FaceFrame>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video segments where a face was detected."]
        pub segments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1FaceSegment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Thumbnail of a representative face view (in JPEG format)."]
        pub thumbnail: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1FaceAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1FaceAnnotationBuilder {
            GoogleCloudVideointelligenceV1FaceAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Face detection annotation."]
    pub struct GoogleCloudVideointelligenceV1FaceDetectionAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The thumbnail of a person's face."]
        pub thumbnail: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tracks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The face tracks with attributes."]
        pub tracks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1Track>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1FaceDetectionAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1FaceDetectionAnnotationBuilder {
            GoogleCloudVideointelligenceV1FaceDetectionAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Config for FACE_DETECTION."]
    pub struct GoogleCloudVideointelligenceV1FaceDetectionConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeAttributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to enable face attributes detection, such as glasses, dark_glasses, mouth_open etc. Ignored if 'include_bounding_boxes' is set to false."]
        pub include_attributes: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeBoundingBoxes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether bounding boxes are included in the face annotation output."]
        pub include_bounding_boxes: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "model")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Model to use for face detection. Supported values: \"builtin/stable\" (the default if unset) and \"builtin/latest\"."]
        pub model: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1FaceDetectionConfig {
        pub fn builder() -> GoogleCloudVideointelligenceV1FaceDetectionConfigBuilder {
            GoogleCloudVideointelligenceV1FaceDetectionConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deprecated. No effect."]
    pub struct GoogleCloudVideointelligenceV1FaceFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedBoundingBoxes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Normalized Bounding boxes in a frame. There can be more than one boxes if the same face is detected in multiple locations within the current frame."]
        pub normalized_bounding_boxes: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1NormalizedBoundingBox>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1FaceFrame {
        pub fn builder() -> GoogleCloudVideointelligenceV1FaceFrameBuilder {
            GoogleCloudVideointelligenceV1FaceFrameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video segment level annotation results for face detection."]
    pub struct GoogleCloudVideointelligenceV1FaceSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video segment where a face was detected."]
        pub segment:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1VideoSegment>>,
    }
    impl GoogleCloudVideointelligenceV1FaceSegment {
        pub fn builder() -> GoogleCloudVideointelligenceV1FaceSegmentBuilder {
            GoogleCloudVideointelligenceV1FaceSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Label annotation."]
    pub struct GoogleCloudVideointelligenceV1LabelAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "categoryEntities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Common categories for the detected entity. For example, when the label is `Terrier`, the category is likely `dog`. And in some cases there might be more than one categories e.g., `Terrier` could also be a `pet`."]
        pub category_entities: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1Entity>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected entity."]
        pub entity: ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1Entity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video frames where a label was detected."]
        pub frames: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1LabelFrame>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video segments where a label was detected."]
        pub segments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1LabelSegment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1LabelAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1LabelAnnotationBuilder {
            GoogleCloudVideointelligenceV1LabelAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Config for LABEL_DETECTION."]
    pub struct GoogleCloudVideointelligenceV1LabelDetectionConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frameConfidenceThreshold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The confidence threshold we perform filtering on the labels from frame-level detection. If not set, it is set to 0.4 by default. The valid range for this threshold is [0.1, 0.9]. Any value set outside of this range will be clipped. Note: For best results, follow the default threshold. We will update the default threshold everytime when we release a new model."]
        pub frame_confidence_threshold: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelDetectionMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "What labels should be detected with LABEL_DETECTION, in addition to video-level labels or segment-level labels. If unspecified, defaults to `SHOT_MODE`."]
        pub label_detection_mode: ::std::option::Option<
            GoogleCloudVideointelligenceV1LabelDetectionConfigLabelDetectionModeEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "model")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Model to use for label detection. Supported values: \"builtin/stable\" (the default if unset) and \"builtin/latest\"."]
        pub model: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stationaryCamera")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the video has been shot from a stationary (i.e., non-moving) camera. When set to true, might improve detection accuracy for moving objects. Should be used with `SHOT_AND_FRAME_MODE` enabled."]
        pub stationary_camera: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoConfidenceThreshold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The confidence threshold we perform filtering on the labels from video-level and shot-level detections. If not set, it's set to 0.3 by default. The valid range for this threshold is [0.1, 0.9]. Any value set outside of this range will be clipped. Note: For best results, follow the default threshold. We will update the default threshold everytime when we release a new model."]
        pub video_confidence_threshold: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVideointelligenceV1LabelDetectionConfig {
        pub fn builder() -> GoogleCloudVideointelligenceV1LabelDetectionConfigBuilder {
            GoogleCloudVideointelligenceV1LabelDetectionConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "What labels should be detected with LABEL_DETECTION, in addition to video-level labels or segment-level labels. If unspecified, defaults to `SHOT_MODE`."]
    pub enum GoogleCloudVideointelligenceV1LabelDetectionConfigLabelDetectionModeEnum {
        #[serde(rename = "LABEL_DETECTION_MODE_UNSPECIFIED")]
        #[doc = "Unspecified."]
        LabelDetectionModeUnspecified,
        #[serde(rename = "SHOT_MODE")]
        #[doc = "Detect shot-level labels."]
        ShotMode,
        #[serde(rename = "FRAME_MODE")]
        #[doc = "Detect frame-level labels."]
        FrameMode,
        #[serde(rename = "SHOT_AND_FRAME_MODE")]
        #[doc = "Detect both shot-level and frame-level labels."]
        ShotAndFrameMode,
    }
    impl ::std::default::Default
        for GoogleCloudVideointelligenceV1LabelDetectionConfigLabelDetectionModeEnum
    {
        fn default() -> Self {
            Self::LabelDetectionModeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video frame level annotation results for label detection."]
    pub struct GoogleCloudVideointelligenceV1LabelFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1LabelFrame {
        pub fn builder() -> GoogleCloudVideointelligenceV1LabelFrameBuilder {
            GoogleCloudVideointelligenceV1LabelFrameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video segment level annotation results for label detection."]
    pub struct GoogleCloudVideointelligenceV1LabelSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video segment where a label was detected."]
        pub segment:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1VideoSegment>>,
    }
    impl GoogleCloudVideointelligenceV1LabelSegment {
        pub fn builder() -> GoogleCloudVideointelligenceV1LabelSegmentBuilder {
            GoogleCloudVideointelligenceV1LabelSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotation corresponding to one detected, tracked and recognized logo class."]
    pub struct GoogleCloudVideointelligenceV1LogoRecognitionAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entity category information to specify the logo class that all the logo tracks within this LogoRecognitionAnnotation are recognized as."]
        pub entity: ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1Entity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video segments where the recognized logo appears. There might be multiple instances of the same logo class appearing in one VideoSegment."]
        pub segments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1VideoSegment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tracks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All logo tracks where the recognized logo appears. Each track corresponds to one logo instance appearing in consecutive frames."]
        pub tracks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1Track>>,
        >,
    }
    impl GoogleCloudVideointelligenceV1LogoRecognitionAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1LogoRecognitionAnnotationBuilder {
            GoogleCloudVideointelligenceV1LogoRecognitionAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Normalized bounding box. The normalized vertex coordinates are relative to the original image. Range: [0, 1]."]
    pub struct GoogleCloudVideointelligenceV1NormalizedBoundingBox {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bottom")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Bottom Y coordinate."]
        pub bottom: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "left")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Left X coordinate."]
        pub left: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "right")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Right X coordinate."]
        pub right: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "top")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Top Y coordinate."]
        pub top: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVideointelligenceV1NormalizedBoundingBox {
        pub fn builder() -> GoogleCloudVideointelligenceV1NormalizedBoundingBoxBuilder {
            GoogleCloudVideointelligenceV1NormalizedBoundingBoxBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Normalized bounding polygon for text (that might not be aligned with axis). Contains list of the corner points in clockwise order starting from top-left corner. For example, for a rectangular bounding box: When the text is horizontal it might look like: 0----1 | | 3----2 When it's clockwise rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3). Note that values can be less than 0, or greater than 1 due to trignometric calculations for location of the box."]
    pub struct GoogleCloudVideointelligenceV1NormalizedBoundingPoly {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vertices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Normalized vertices of the bounding polygon."]
        pub vertices: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1NormalizedVertex>>,
        >,
    }
    impl GoogleCloudVideointelligenceV1NormalizedBoundingPoly {
        pub fn builder() -> GoogleCloudVideointelligenceV1NormalizedBoundingPolyBuilder {
            GoogleCloudVideointelligenceV1NormalizedBoundingPolyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A vertex represents a 2D point in the image. NOTE: the normalized vertex coordinates are relative to the original image and range from 0 to 1."]
    pub struct GoogleCloudVideointelligenceV1NormalizedVertex {
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
    impl GoogleCloudVideointelligenceV1NormalizedVertex {
        pub fn builder() -> GoogleCloudVideointelligenceV1NormalizedVertexBuilder {
            GoogleCloudVideointelligenceV1NormalizedVertexBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotations corresponding to one tracked object."]
    pub struct GoogleCloudVideointelligenceV1ObjectTrackingAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object category's labeling confidence of this track."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entity to specify the object category that this track is labeled as."]
        pub entity: ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1Entity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information corresponding to all frames where this object track appears. Non-streaming batch mode: it may be one or multiple ObjectTrackingFrame messages in frames. Streaming mode: it can only be one ObjectTrackingFrame message in frames."]
        pub frames: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1ObjectTrackingFrame>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Non-streaming batch mode ONLY. Each object track corresponds to one video segment where it appears."]
        pub segment:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1VideoSegment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trackId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Streaming mode ONLY. In streaming mode, we do not know the end time of a tracked object before it is completed. Hence, there is no VideoSegment info returned. Instead, we provide a unique identifiable integer track_id so that the customers can correlate the results of the ongoing ObjectTrackAnnotation of the same track_id over time."]
        pub track_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1ObjectTrackingAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1ObjectTrackingAnnotationBuilder {
            GoogleCloudVideointelligenceV1ObjectTrackingAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Config for OBJECT_TRACKING."]
    pub struct GoogleCloudVideointelligenceV1ObjectTrackingConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "model")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Model to use for object tracking. Supported values: \"builtin/stable\" (the default if unset) and \"builtin/latest\"."]
        pub model: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1ObjectTrackingConfig {
        pub fn builder() -> GoogleCloudVideointelligenceV1ObjectTrackingConfigBuilder {
            GoogleCloudVideointelligenceV1ObjectTrackingConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video frame level annotations for object detection and tracking. This field stores per frame location, time offset, and confidence."]
    pub struct GoogleCloudVideointelligenceV1ObjectTrackingFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedBoundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The normalized bounding box location of this object track for the frame."]
        pub normalized_bounding_box: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1NormalizedBoundingBox>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp of the frame in microseconds."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1ObjectTrackingFrame {
        pub fn builder() -> GoogleCloudVideointelligenceV1ObjectTrackingFrameBuilder {
            GoogleCloudVideointelligenceV1ObjectTrackingFrameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Person detection annotation per video."]
    pub struct GoogleCloudVideointelligenceV1PersonDetectionAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tracks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The detected tracks of a person."]
        pub tracks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1Track>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1PersonDetectionAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1PersonDetectionAnnotationBuilder {
            GoogleCloudVideointelligenceV1PersonDetectionAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Config for PERSON_DETECTION."]
    pub struct GoogleCloudVideointelligenceV1PersonDetectionConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeAttributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to enable person attributes detection, such as cloth color (black, blue, etc), type (coat, dress, etc), pattern (plain, floral, etc), hair, etc. Ignored if 'include_bounding_boxes' is set to false."]
        pub include_attributes: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeBoundingBoxes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether bounding boxes are included in the person detection annotation output."]
        pub include_bounding_boxes: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includePoseLandmarks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to enable pose landmarks detection. Ignored if 'include_bounding_boxes' is set to false."]
        pub include_pose_landmarks: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudVideointelligenceV1PersonDetectionConfig {
        pub fn builder() -> GoogleCloudVideointelligenceV1PersonDetectionConfigBuilder {
            GoogleCloudVideointelligenceV1PersonDetectionConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Config for SHOT_CHANGE_DETECTION."]
    pub struct GoogleCloudVideointelligenceV1ShotChangeDetectionConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "model")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Model to use for shot change detection. Supported values: \"builtin/stable\" (the default if unset) and \"builtin/latest\"."]
        pub model: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1ShotChangeDetectionConfig {
        pub fn builder() -> GoogleCloudVideointelligenceV1ShotChangeDetectionConfigBuilder {
            GoogleCloudVideointelligenceV1ShotChangeDetectionConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Provides \"hints\" to the speech recognizer to favor specific words and phrases in the results."]
    pub struct GoogleCloudVideointelligenceV1SpeechContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phrases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A list of strings containing words and phrases \"hints\" so that the speech recognition is more likely to recognize them. This can be used to improve the accuracy for specific words and phrases, for example, if specific commands are typically spoken by the user. This can also be used to add additional words to the vocabulary of the recognizer. See [usage limits](https://cloud.google.com/speech/limits#content)."]
        pub phrases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudVideointelligenceV1SpeechContext {
        pub fn builder() -> GoogleCloudVideointelligenceV1SpeechContextBuilder {
            GoogleCloudVideointelligenceV1SpeechContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Alternative hypotheses (a.k.a. n-best list)."]
    pub struct GoogleCloudVideointelligenceV1SpeechRecognitionAlternative {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transcript")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Transcript text representing the words that the user spoke."]
        pub transcript: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "words")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A list of word-specific information for each recognized word. Note: When `enable_speaker_diarization` is set to true, you will see all the words from the beginning of the audio."]
        pub words: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1WordInfo>>,
        >,
    }
    impl GoogleCloudVideointelligenceV1SpeechRecognitionAlternative {
        pub fn builder() -> GoogleCloudVideointelligenceV1SpeechRecognitionAlternativeBuilder {
            GoogleCloudVideointelligenceV1SpeechRecognitionAlternativeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A speech recognition result corresponding to a portion of the audio."]
    pub struct GoogleCloudVideointelligenceV1SpeechTranscription {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alternatives")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "May contain one or more recognition hypotheses (up to the maximum specified in `max_alternatives`). These alternatives are ordered in terms of accuracy, with the top (first) alternative being the most probable, as ranked by the recognizer."]
        pub alternatives: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1SpeechRecognitionAlternative>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag of the language in this result. This language code was detected to have the most likelihood of being spoken in the audio."]
        pub language_code: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1SpeechTranscription {
        pub fn builder() -> GoogleCloudVideointelligenceV1SpeechTranscriptionBuilder {
            GoogleCloudVideointelligenceV1SpeechTranscriptionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Config for SPEECH_TRANSCRIPTION."]
    pub struct GoogleCloudVideointelligenceV1SpeechTranscriptionConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audioTracks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. For file formats, such as MXF or MKV, supporting multiple audio tracks, specify up to two tracks. Default: track 0."]
        pub audio_tracks: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diarizationSpeakerCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If set, specifies the estimated number of speakers in the conversation. If not set, defaults to '2'. Ignored unless enable_speaker_diarization is set to true."]
        pub diarization_speaker_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableAutomaticPunctuation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If 'true', adds punctuation to recognition result hypotheses. This feature is only available in select languages. Setting this for requests in other languages has no effect at all. The default 'false' value does not add punctuation to result hypotheses. NOTE: \"This is currently offered as an experimental service, complimentary to all users. In the future this may be exclusively available as a premium feature.\""]
        pub enable_automatic_punctuation: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableSpeakerDiarization")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If 'true', enables speaker detection for each recognized word in the top alternative of the recognition result using a speaker_tag provided in the WordInfo. Note: When this is true, we send all the words from the beginning of the audio for the top alternative in every consecutive response. This is done in order to improve our speaker tags as our models learn to identify the speakers in the conversation over time."]
        pub enable_speaker_diarization: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableWordConfidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If `true`, the top result includes a list of words and the confidence for those words. If `false`, no word-level confidence information is returned. The default is `false`."]
        pub enable_word_confidence: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filterProfanity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If set to `true`, the server will attempt to filter out profanities, replacing all but the initial character in each filtered word with asterisks, e.g. \"f***\". If set to `false` or omitted, profanities won't be filtered out."]
        pub filter_profanity: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. *Required* The language of the supplied audio as a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. Example: \"en-US\". See [Language Support](https://cloud.google.com/speech/docs/languages) for a list of the currently supported language codes."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxAlternatives")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Maximum number of recognition hypotheses to be returned. Specifically, the maximum number of `SpeechRecognitionAlternative` messages within each `SpeechTranscription`. The server may return fewer than `max_alternatives`. Valid values are `0`-`30`. A value of `0` or `1` will return a maximum of one. If omitted, will return a maximum of one."]
        pub max_alternatives: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "speechContexts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A means to provide context to assist the speech recognition."]
        pub speech_contexts: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1SpeechContext>>,
        >,
    }
    impl GoogleCloudVideointelligenceV1SpeechTranscriptionConfig {
        pub fn builder() -> GoogleCloudVideointelligenceV1SpeechTranscriptionConfigBuilder {
            GoogleCloudVideointelligenceV1SpeechTranscriptionConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotations related to one detected OCR text snippet. This will contain the corresponding text, confidence value, and frame level information for each detection."]
    pub struct GoogleCloudVideointelligenceV1TextAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video segments where OCR detected text appears."]
        pub segments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1TextSegment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The detected text."]
        pub text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1TextAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1TextAnnotationBuilder {
            GoogleCloudVideointelligenceV1TextAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Config for TEXT_DETECTION."]
    pub struct GoogleCloudVideointelligenceV1TextDetectionConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageHints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Language hint can be specified if the language to be detected is known a priori. It can increase the accuracy of the detection. Language hint must be language code in BCP-47 format. Automatic language detection is performed if no hint is provided."]
        pub language_hints: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "model")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Model to use for text detection. Supported values: \"builtin/stable\" (the default if unset) and \"builtin/latest\"."]
        pub model: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1TextDetectionConfig {
        pub fn builder() -> GoogleCloudVideointelligenceV1TextDetectionConfigBuilder {
            GoogleCloudVideointelligenceV1TextDetectionConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video frame level annotation results for text annotation (OCR). Contains information regarding timestamp and bounding box locations for the frames containing detected OCR text snippets."]
    pub struct GoogleCloudVideointelligenceV1TextFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rotatedBoundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Bounding polygon of the detected text for this frame."]
        pub rotated_bounding_box: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1NormalizedBoundingPoly>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp of this frame."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1TextFrame {
        pub fn builder() -> GoogleCloudVideointelligenceV1TextFrameBuilder {
            GoogleCloudVideointelligenceV1TextFrameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video segment level annotation results for text detection."]
    pub struct GoogleCloudVideointelligenceV1TextSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence for the track of detected text. It is calculated as the highest over all frames where OCR detected text appears."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information related to the frames where OCR detected text appears."]
        pub frames: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1TextFrame>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video segment where a text snippet was detected."]
        pub segment:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1VideoSegment>>,
    }
    impl GoogleCloudVideointelligenceV1TextSegment {
        pub fn builder() -> GoogleCloudVideointelligenceV1TextSegmentBuilder {
            GoogleCloudVideointelligenceV1TextSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "For tracking related features. An object at time_offset with attributes, and located with normalized_bounding_box."]
    pub struct GoogleCloudVideointelligenceV1TimestampedObject {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The attributes of the object in the bounding box."]
        pub attributes: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1DetectedAttribute>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "landmarks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The detected landmarks."]
        pub landmarks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1DetectedLandmark>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedBoundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Normalized Bounding box in a frame, where the object is located."]
        pub normalized_bounding_box: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1NormalizedBoundingBox>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this object."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1TimestampedObject {
        pub fn builder() -> GoogleCloudVideointelligenceV1TimestampedObjectBuilder {
            GoogleCloudVideointelligenceV1TimestampedObjectBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A track of an object instance."]
    pub struct GoogleCloudVideointelligenceV1Track {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Attributes in the track level."]
        pub attributes: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1DetectedAttribute>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The confidence score of the tracked object."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video segment of a track."]
        pub segment:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1VideoSegment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestampedObjects")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object with timestamp and attributes per frame in the track."]
        pub timestamped_objects: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1TimestampedObject>>,
        >,
    }
    impl GoogleCloudVideointelligenceV1Track {
        pub fn builder() -> GoogleCloudVideointelligenceV1TrackBuilder {
            GoogleCloudVideointelligenceV1TrackBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotation progress for a single video."]
    pub struct GoogleCloudVideointelligenceV1VideoAnnotationProgress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "feature")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies which feature is being tracked if the request contains more than one feature."]
        pub feature:
            ::std::option::Option<GoogleCloudVideointelligenceV1VideoAnnotationProgressFeatureEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
        pub input_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressPercent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Approximate percentage processed thus far. Guaranteed to be 100 when fully processed."]
        pub progress_percent: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies which segment is being tracked if the request contains more than one segment."]
        pub segment:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1VideoSegment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time when the request was received."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time of the most recent update."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1VideoAnnotationProgress {
        pub fn builder() -> GoogleCloudVideointelligenceV1VideoAnnotationProgressBuilder {
            GoogleCloudVideointelligenceV1VideoAnnotationProgressBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies which feature is being tracked if the request contains more than one feature."]
    pub enum GoogleCloudVideointelligenceV1VideoAnnotationProgressFeatureEnum {
        #[serde(rename = "FEATURE_UNSPECIFIED")]
        #[doc = "Unspecified."]
        FeatureUnspecified,
        #[serde(rename = "LABEL_DETECTION")]
        #[doc = "Label detection. Detect objects, such as dog or flower."]
        LabelDetection,
        #[serde(rename = "SHOT_CHANGE_DETECTION")]
        #[doc = "Shot change detection."]
        ShotChangeDetection,
        #[serde(rename = "EXPLICIT_CONTENT_DETECTION")]
        #[doc = "Explicit content detection."]
        ExplicitContentDetection,
        #[serde(rename = "FACE_DETECTION")]
        #[doc = "Human face detection."]
        FaceDetection,
        #[serde(rename = "SPEECH_TRANSCRIPTION")]
        #[doc = "Speech transcription."]
        SpeechTranscription,
        #[serde(rename = "TEXT_DETECTION")]
        #[doc = "OCR text detection and tracking."]
        TextDetection,
        #[serde(rename = "OBJECT_TRACKING")]
        #[doc = "Object detection and tracking."]
        ObjectTracking,
        #[serde(rename = "LOGO_RECOGNITION")]
        #[doc = "Logo detection, tracking, and recognition."]
        LogoRecognition,
        #[serde(rename = "PERSON_DETECTION")]
        #[doc = "Person detection."]
        PersonDetection,
    }
    impl ::std::default::Default for GoogleCloudVideointelligenceV1VideoAnnotationProgressFeatureEnum {
        fn default() -> Self {
            Self::FeatureUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotation results for a single video."]
    pub struct GoogleCloudVideointelligenceV1VideoAnnotationResults {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, indicates an error. Note that for a single `AnnotateVideoRequest` some videos may succeed and some may fail."]
        pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "explicitAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Explicit content annotation."]
        pub explicit_annotation: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1ExplicitContentAnnotation>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "faceAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Please use `face_detection_annotations` instead."]
        pub face_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1FaceAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "faceDetectionAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Face detection annotations."]
        pub face_detection_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1FaceDetectionAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frameLabelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label annotations on frame level. There is exactly one element for each unique label."]
        pub frame_label_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1LabelAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
        pub input_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logoRecognitionAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Annotations for list of logos detected, tracked and recognized in video."]
        pub logo_recognition_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1LogoRecognitionAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Annotations for list of objects detected and tracked in video."]
        pub object_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1ObjectTrackingAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "personDetectionAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Person detection annotations."]
        pub person_detection_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1PersonDetectionAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video segment on which the annotation is run."]
        pub segment:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1VideoSegment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segmentLabelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Topical label annotations on video level or user-specified segment level. There is exactly one element for each unique label."]
        pub segment_label_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1LabelAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segmentPresenceLabelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Presence label annotations on video level or user-specified segment level. There is exactly one element for each unique label. Compared to the existing topical `segment_label_annotations`, this field presents more fine-grained, segment-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to \"builtin/latest\" in the request."]
        pub segment_presence_label_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1LabelAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shotAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Shot annotations. Each shot is represented as a video segment."]
        pub shot_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1VideoSegment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shotLabelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Topical label annotations on shot level. There is exactly one element for each unique label."]
        pub shot_label_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1LabelAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shotPresenceLabelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Presence label annotations on shot level. There is exactly one element for each unique label. Compared to the existing topical `shot_label_annotations`, this field presents more fine-grained, shot-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to \"builtin/latest\" in the request."]
        pub shot_presence_label_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1LabelAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "speechTranscriptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Speech transcription."]
        pub speech_transcriptions: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1SpeechTranscription>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OCR text detection and tracking. Annotations for list of detected text snippets. Each will have list of frame information associated with it."]
        pub text_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1TextAnnotation>>,
        >,
    }
    impl GoogleCloudVideointelligenceV1VideoAnnotationResults {
        pub fn builder() -> GoogleCloudVideointelligenceV1VideoAnnotationResultsBuilder {
            GoogleCloudVideointelligenceV1VideoAnnotationResultsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video context and/or feature-specific parameters."]
    pub struct GoogleCloudVideointelligenceV1VideoContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "explicitContentDetectionConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Config for EXPLICIT_CONTENT_DETECTION."]
        pub explicit_content_detection_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1ExplicitContentDetectionConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "faceDetectionConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Config for FACE_DETECTION."]
        pub face_detection_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1FaceDetectionConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelDetectionConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Config for LABEL_DETECTION."]
        pub label_detection_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1LabelDetectionConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectTrackingConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Config for OBJECT_TRACKING."]
        pub object_tracking_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1ObjectTrackingConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "personDetectionConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Config for PERSON_DETECTION."]
        pub person_detection_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1PersonDetectionConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video segments to annotate. The segments may overlap and are not required to be contiguous or span the whole video. If unspecified, each video is treated as a single segment."]
        pub segments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1VideoSegment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shotChangeDetectionConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Config for SHOT_CHANGE_DETECTION."]
        pub shot_change_detection_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1ShotChangeDetectionConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "speechTranscriptionConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Config for SPEECH_TRANSCRIPTION."]
        pub speech_transcription_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1SpeechTranscriptionConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textDetectionConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Config for TEXT_DETECTION."]
        pub text_detection_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1TextDetectionConfig>,
        >,
    }
    impl GoogleCloudVideointelligenceV1VideoContext {
        pub fn builder() -> GoogleCloudVideointelligenceV1VideoContextBuilder {
            GoogleCloudVideointelligenceV1VideoContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video segment."]
    pub struct GoogleCloudVideointelligenceV1VideoSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTimeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the end of the segment (inclusive)."]
        pub end_time_offset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTimeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the start of the segment (inclusive)."]
        pub start_time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1VideoSegment {
        pub fn builder() -> GoogleCloudVideointelligenceV1VideoSegmentBuilder {
            GoogleCloudVideointelligenceV1VideoSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Word-specific information for recognized words. Word information is only included in the response when certain request parameters are set, such as `enable_word_time_offsets`."]
    pub struct GoogleCloudVideointelligenceV1WordInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time offset relative to the beginning of the audio, and corresponding to the end of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "speakerTag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A distinct integer value is assigned for every speaker within the audio. This field specifies which one of those speakers was detected to have spoken this word. Value ranges from 1 up to diarization_speaker_count, and is only set if speaker diarization is enabled."]
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
    impl GoogleCloudVideointelligenceV1WordInfo {
        pub fn builder() -> GoogleCloudVideointelligenceV1WordInfoBuilder {
            GoogleCloudVideointelligenceV1WordInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video annotation progress. Included in the `metadata` field of the `Operation` returned by the `GetOperation` call of the `google::longrunning::Operations` service."]
    pub struct GoogleCloudVideointelligenceV1beta2AnnotateVideoProgress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationProgress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Progress metadata for all videos specified in `AnnotateVideoRequest`."]
        pub annotation_progress: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2VideoAnnotationProgress>,
            >,
        >,
    }
    impl GoogleCloudVideointelligenceV1beta2AnnotateVideoProgress {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2AnnotateVideoProgressBuilder {
            GoogleCloudVideointelligenceV1beta2AnnotateVideoProgressBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video annotation response. Included in the `response` field of the `Operation` returned by the `GetOperation` call of the `google::longrunning::Operations` service."]
    pub struct GoogleCloudVideointelligenceV1beta2AnnotateVideoResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Annotation results for all videos specified in `AnnotateVideoRequest`."]
        pub annotation_results: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2VideoAnnotationResults>,
            >,
        >,
    }
    impl GoogleCloudVideointelligenceV1beta2AnnotateVideoResponse {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2AnnotateVideoResponseBuilder {
            GoogleCloudVideointelligenceV1beta2AnnotateVideoResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic detected attribute represented by name in string format."]
    pub struct GoogleCloudVideointelligenceV1beta2DetectedAttribute {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected attribute confidence. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the attribute, for example, glasses, dark_glasses, mouth_open. A full list of supported type names will be provided in the document."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text value of the detection result. For example, the value for \"HairColor\" can be \"black\", \"blonde\", etc."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1beta2DetectedAttribute {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2DetectedAttributeBuilder {
            GoogleCloudVideointelligenceV1beta2DetectedAttributeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic detected landmark represented by name in string format and a 2D location."]
    pub struct GoogleCloudVideointelligenceV1beta2DetectedLandmark {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The confidence score of the detected landmark. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of this landmark, for example, left_hand, right_shoulder."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "point")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The 2D point of the detected landmark using the normalized image coordindate system. The normalized coordinates have the range from 0 to 1."]
        pub point: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2NormalizedVertex>,
        >,
    }
    impl GoogleCloudVideointelligenceV1beta2DetectedLandmark {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2DetectedLandmarkBuilder {
            GoogleCloudVideointelligenceV1beta2DetectedLandmarkBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected entity from video analysis."]
    pub struct GoogleCloudVideointelligenceV1beta2Entity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Textual description, e.g., `Fixed-gear bicycle`."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Opaque entity ID. Some IDs may be available in [Google Knowledge Graph Search API](https://developers.google.com/knowledge-graph/)."]
        pub entity_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Language code for `description` in BCP-47 format."]
        pub language_code: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1beta2Entity {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2EntityBuilder {
            GoogleCloudVideointelligenceV1beta2EntityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Explicit content annotation (based on per-frame visual signals only). If no explicit content has been detected in a frame, no annotations are present for that frame."]
    pub struct GoogleCloudVideointelligenceV1beta2ExplicitContentAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video frames where explicit content was detected."]
        pub frames: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2ExplicitContentFrame>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1beta2ExplicitContentAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2ExplicitContentAnnotationBuilder {
            GoogleCloudVideointelligenceV1beta2ExplicitContentAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video frame level annotation results for explicit content."]
    pub struct GoogleCloudVideointelligenceV1beta2ExplicitContentFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pornographyLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Likelihood of the pornography content.."]
        pub pornography_likelihood: ::std::option::Option<
            GoogleCloudVideointelligenceV1beta2ExplicitContentFramePornographyLikelihoodEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1beta2ExplicitContentFrame {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2ExplicitContentFrameBuilder {
            GoogleCloudVideointelligenceV1beta2ExplicitContentFrameBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Likelihood of the pornography content.."]
    pub enum GoogleCloudVideointelligenceV1beta2ExplicitContentFramePornographyLikelihoodEnum {
        #[serde(rename = "LIKELIHOOD_UNSPECIFIED")]
        #[doc = "Unspecified likelihood."]
        LikelihoodUnspecified,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "Very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "Unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "Possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "Likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "Very likely."]
        VeryLikely,
    }
    impl ::std::default::Default
        for GoogleCloudVideointelligenceV1beta2ExplicitContentFramePornographyLikelihoodEnum
    {
        fn default() -> Self {
            Self::LikelihoodUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deprecated. No effect."]
    pub struct GoogleCloudVideointelligenceV1beta2FaceAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video frames where a face was detected."]
        pub frames: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2FaceFrame>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video segments where a face was detected."]
        pub segments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2FaceSegment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Thumbnail of a representative face view (in JPEG format)."]
        pub thumbnail: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1beta2FaceAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2FaceAnnotationBuilder {
            GoogleCloudVideointelligenceV1beta2FaceAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Face detection annotation."]
    pub struct GoogleCloudVideointelligenceV1beta2FaceDetectionAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The thumbnail of a person's face."]
        pub thumbnail: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tracks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The face tracks with attributes."]
        pub tracks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2Track>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1beta2FaceDetectionAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2FaceDetectionAnnotationBuilder {
            GoogleCloudVideointelligenceV1beta2FaceDetectionAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deprecated. No effect."]
    pub struct GoogleCloudVideointelligenceV1beta2FaceFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedBoundingBoxes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Normalized Bounding boxes in a frame. There can be more than one boxes if the same face is detected in multiple locations within the current frame."]
        pub normalized_bounding_boxes: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2NormalizedBoundingBox>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1beta2FaceFrame {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2FaceFrameBuilder {
            GoogleCloudVideointelligenceV1beta2FaceFrameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video segment level annotation results for face detection."]
    pub struct GoogleCloudVideointelligenceV1beta2FaceSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video segment where a face was detected."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2VideoSegment>,
        >,
    }
    impl GoogleCloudVideointelligenceV1beta2FaceSegment {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2FaceSegmentBuilder {
            GoogleCloudVideointelligenceV1beta2FaceSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Label annotation."]
    pub struct GoogleCloudVideointelligenceV1beta2LabelAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "categoryEntities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Common categories for the detected entity. For example, when the label is `Terrier`, the category is likely `dog`. And in some cases there might be more than one categories e.g., `Terrier` could also be a `pet`."]
        pub category_entities: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2Entity>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected entity."]
        pub entity:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2Entity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video frames where a label was detected."]
        pub frames: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2LabelFrame>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video segments where a label was detected."]
        pub segments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2LabelSegment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1beta2LabelAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2LabelAnnotationBuilder {
            GoogleCloudVideointelligenceV1beta2LabelAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video frame level annotation results for label detection."]
    pub struct GoogleCloudVideointelligenceV1beta2LabelFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1beta2LabelFrame {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2LabelFrameBuilder {
            GoogleCloudVideointelligenceV1beta2LabelFrameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video segment level annotation results for label detection."]
    pub struct GoogleCloudVideointelligenceV1beta2LabelSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video segment where a label was detected."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2VideoSegment>,
        >,
    }
    impl GoogleCloudVideointelligenceV1beta2LabelSegment {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2LabelSegmentBuilder {
            GoogleCloudVideointelligenceV1beta2LabelSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotation corresponding to one detected, tracked and recognized logo class."]
    pub struct GoogleCloudVideointelligenceV1beta2LogoRecognitionAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entity category information to specify the logo class that all the logo tracks within this LogoRecognitionAnnotation are recognized as."]
        pub entity:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2Entity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video segments where the recognized logo appears. There might be multiple instances of the same logo class appearing in one VideoSegment."]
        pub segments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2VideoSegment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tracks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All logo tracks where the recognized logo appears. Each track corresponds to one logo instance appearing in consecutive frames."]
        pub tracks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2Track>>,
        >,
    }
    impl GoogleCloudVideointelligenceV1beta2LogoRecognitionAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2LogoRecognitionAnnotationBuilder {
            GoogleCloudVideointelligenceV1beta2LogoRecognitionAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Normalized bounding box. The normalized vertex coordinates are relative to the original image. Range: [0, 1]."]
    pub struct GoogleCloudVideointelligenceV1beta2NormalizedBoundingBox {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bottom")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Bottom Y coordinate."]
        pub bottom: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "left")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Left X coordinate."]
        pub left: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "right")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Right X coordinate."]
        pub right: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "top")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Top Y coordinate."]
        pub top: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVideointelligenceV1beta2NormalizedBoundingBox {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2NormalizedBoundingBoxBuilder {
            GoogleCloudVideointelligenceV1beta2NormalizedBoundingBoxBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Normalized bounding polygon for text (that might not be aligned with axis). Contains list of the corner points in clockwise order starting from top-left corner. For example, for a rectangular bounding box: When the text is horizontal it might look like: 0----1 | | 3----2 When it's clockwise rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3). Note that values can be less than 0, or greater than 1 due to trignometric calculations for location of the box."]
    pub struct GoogleCloudVideointelligenceV1beta2NormalizedBoundingPoly {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vertices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Normalized vertices of the bounding polygon."]
        pub vertices: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2NormalizedVertex>>,
        >,
    }
    impl GoogleCloudVideointelligenceV1beta2NormalizedBoundingPoly {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2NormalizedBoundingPolyBuilder {
            GoogleCloudVideointelligenceV1beta2NormalizedBoundingPolyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A vertex represents a 2D point in the image. NOTE: the normalized vertex coordinates are relative to the original image and range from 0 to 1."]
    pub struct GoogleCloudVideointelligenceV1beta2NormalizedVertex {
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
    impl GoogleCloudVideointelligenceV1beta2NormalizedVertex {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2NormalizedVertexBuilder {
            GoogleCloudVideointelligenceV1beta2NormalizedVertexBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotations corresponding to one tracked object."]
    pub struct GoogleCloudVideointelligenceV1beta2ObjectTrackingAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object category's labeling confidence of this track."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entity to specify the object category that this track is labeled as."]
        pub entity:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2Entity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information corresponding to all frames where this object track appears. Non-streaming batch mode: it may be one or multiple ObjectTrackingFrame messages in frames. Streaming mode: it can only be one ObjectTrackingFrame message in frames."]
        pub frames: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2ObjectTrackingFrame>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Non-streaming batch mode ONLY. Each object track corresponds to one video segment where it appears."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2VideoSegment>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trackId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Streaming mode ONLY. In streaming mode, we do not know the end time of a tracked object before it is completed. Hence, there is no VideoSegment info returned. Instead, we provide a unique identifiable integer track_id so that the customers can correlate the results of the ongoing ObjectTrackAnnotation of the same track_id over time."]
        pub track_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1beta2ObjectTrackingAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2ObjectTrackingAnnotationBuilder {
            GoogleCloudVideointelligenceV1beta2ObjectTrackingAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video frame level annotations for object detection and tracking. This field stores per frame location, time offset, and confidence."]
    pub struct GoogleCloudVideointelligenceV1beta2ObjectTrackingFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedBoundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The normalized bounding box location of this object track for the frame."]
        pub normalized_bounding_box: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2NormalizedBoundingBox>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp of the frame in microseconds."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1beta2ObjectTrackingFrame {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2ObjectTrackingFrameBuilder {
            GoogleCloudVideointelligenceV1beta2ObjectTrackingFrameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Person detection annotation per video."]
    pub struct GoogleCloudVideointelligenceV1beta2PersonDetectionAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tracks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The detected tracks of a person."]
        pub tracks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2Track>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1beta2PersonDetectionAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2PersonDetectionAnnotationBuilder {
            GoogleCloudVideointelligenceV1beta2PersonDetectionAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Alternative hypotheses (a.k.a. n-best list)."]
    pub struct GoogleCloudVideointelligenceV1beta2SpeechRecognitionAlternative {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transcript")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Transcript text representing the words that the user spoke."]
        pub transcript: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "words")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A list of word-specific information for each recognized word. Note: When `enable_speaker_diarization` is set to true, you will see all the words from the beginning of the audio."]
        pub words: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2WordInfo>>,
        >,
    }
    impl GoogleCloudVideointelligenceV1beta2SpeechRecognitionAlternative {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2SpeechRecognitionAlternativeBuilder {
            GoogleCloudVideointelligenceV1beta2SpeechRecognitionAlternativeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A speech recognition result corresponding to a portion of the audio."]
    pub struct GoogleCloudVideointelligenceV1beta2SpeechTranscription {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alternatives")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "May contain one or more recognition hypotheses (up to the maximum specified in `max_alternatives`). These alternatives are ordered in terms of accuracy, with the top (first) alternative being the most probable, as ranked by the recognizer."]
        pub alternatives: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2SpeechRecognitionAlternative>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag of the language in this result. This language code was detected to have the most likelihood of being spoken in the audio."]
        pub language_code: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1beta2SpeechTranscription {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2SpeechTranscriptionBuilder {
            GoogleCloudVideointelligenceV1beta2SpeechTranscriptionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotations related to one detected OCR text snippet. This will contain the corresponding text, confidence value, and frame level information for each detection."]
    pub struct GoogleCloudVideointelligenceV1beta2TextAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video segments where OCR detected text appears."]
        pub segments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2TextSegment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The detected text."]
        pub text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1beta2TextAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2TextAnnotationBuilder {
            GoogleCloudVideointelligenceV1beta2TextAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video frame level annotation results for text annotation (OCR). Contains information regarding timestamp and bounding box locations for the frames containing detected OCR text snippets."]
    pub struct GoogleCloudVideointelligenceV1beta2TextFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rotatedBoundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Bounding polygon of the detected text for this frame."]
        pub rotated_bounding_box: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2NormalizedBoundingPoly>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp of this frame."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1beta2TextFrame {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2TextFrameBuilder {
            GoogleCloudVideointelligenceV1beta2TextFrameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video segment level annotation results for text detection."]
    pub struct GoogleCloudVideointelligenceV1beta2TextSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence for the track of detected text. It is calculated as the highest over all frames where OCR detected text appears."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information related to the frames where OCR detected text appears."]
        pub frames: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2TextFrame>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video segment where a text snippet was detected."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2VideoSegment>,
        >,
    }
    impl GoogleCloudVideointelligenceV1beta2TextSegment {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2TextSegmentBuilder {
            GoogleCloudVideointelligenceV1beta2TextSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "For tracking related features. An object at time_offset with attributes, and located with normalized_bounding_box."]
    pub struct GoogleCloudVideointelligenceV1beta2TimestampedObject {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The attributes of the object in the bounding box."]
        pub attributes: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2DetectedAttribute>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "landmarks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The detected landmarks."]
        pub landmarks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2DetectedLandmark>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedBoundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Normalized Bounding box in a frame, where the object is located."]
        pub normalized_bounding_box: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2NormalizedBoundingBox>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this object."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1beta2TimestampedObject {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2TimestampedObjectBuilder {
            GoogleCloudVideointelligenceV1beta2TimestampedObjectBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A track of an object instance."]
    pub struct GoogleCloudVideointelligenceV1beta2Track {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Attributes in the track level."]
        pub attributes: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2DetectedAttribute>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The confidence score of the tracked object."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video segment of a track."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2VideoSegment>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestampedObjects")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object with timestamp and attributes per frame in the track."]
        pub timestamped_objects: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2TimestampedObject>,
            >,
        >,
    }
    impl GoogleCloudVideointelligenceV1beta2Track {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2TrackBuilder {
            GoogleCloudVideointelligenceV1beta2TrackBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotation progress for a single video."]
    pub struct GoogleCloudVideointelligenceV1beta2VideoAnnotationProgress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "feature")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies which feature is being tracked if the request contains more than one feature."]
        pub feature: ::std::option::Option<
            GoogleCloudVideointelligenceV1beta2VideoAnnotationProgressFeatureEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
        pub input_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressPercent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Approximate percentage processed thus far. Guaranteed to be 100 when fully processed."]
        pub progress_percent: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies which segment is being tracked if the request contains more than one segment."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2VideoSegment>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time when the request was received."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time of the most recent update."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1beta2VideoAnnotationProgress {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2VideoAnnotationProgressBuilder {
            GoogleCloudVideointelligenceV1beta2VideoAnnotationProgressBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies which feature is being tracked if the request contains more than one feature."]
    pub enum GoogleCloudVideointelligenceV1beta2VideoAnnotationProgressFeatureEnum {
        #[serde(rename = "FEATURE_UNSPECIFIED")]
        #[doc = "Unspecified."]
        FeatureUnspecified,
        #[serde(rename = "LABEL_DETECTION")]
        #[doc = "Label detection. Detect objects, such as dog or flower."]
        LabelDetection,
        #[serde(rename = "SHOT_CHANGE_DETECTION")]
        #[doc = "Shot change detection."]
        ShotChangeDetection,
        #[serde(rename = "EXPLICIT_CONTENT_DETECTION")]
        #[doc = "Explicit content detection."]
        ExplicitContentDetection,
        #[serde(rename = "FACE_DETECTION")]
        #[doc = "Human face detection."]
        FaceDetection,
        #[serde(rename = "SPEECH_TRANSCRIPTION")]
        #[doc = "Speech transcription."]
        SpeechTranscription,
        #[serde(rename = "TEXT_DETECTION")]
        #[doc = "OCR text detection and tracking."]
        TextDetection,
        #[serde(rename = "OBJECT_TRACKING")]
        #[doc = "Object detection and tracking."]
        ObjectTracking,
        #[serde(rename = "LOGO_RECOGNITION")]
        #[doc = "Logo detection, tracking, and recognition."]
        LogoRecognition,
        #[serde(rename = "PERSON_DETECTION")]
        #[doc = "Person detection."]
        PersonDetection,
    }
    impl ::std::default::Default
        for GoogleCloudVideointelligenceV1beta2VideoAnnotationProgressFeatureEnum
    {
        fn default() -> Self {
            Self::FeatureUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotation results for a single video."]
    pub struct GoogleCloudVideointelligenceV1beta2VideoAnnotationResults {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, indicates an error. Note that for a single `AnnotateVideoRequest` some videos may succeed and some may fail."]
        pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "explicitAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Explicit content annotation."]
        pub explicit_annotation: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2ExplicitContentAnnotation>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "faceAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Please use `face_detection_annotations` instead."]
        pub face_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2FaceAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "faceDetectionAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Face detection annotations."]
        pub face_detection_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2FaceDetectionAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frameLabelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label annotations on frame level. There is exactly one element for each unique label."]
        pub frame_label_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2LabelAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
        pub input_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logoRecognitionAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Annotations for list of logos detected, tracked and recognized in video."]
        pub logo_recognition_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2LogoRecognitionAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Annotations for list of objects detected and tracked in video."]
        pub object_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2ObjectTrackingAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "personDetectionAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Person detection annotations."]
        pub person_detection_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2PersonDetectionAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video segment on which the annotation is run."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2VideoSegment>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segmentLabelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Topical label annotations on video level or user-specified segment level. There is exactly one element for each unique label."]
        pub segment_label_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2LabelAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segmentPresenceLabelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Presence label annotations on video level or user-specified segment level. There is exactly one element for each unique label. Compared to the existing topical `segment_label_annotations`, this field presents more fine-grained, segment-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to \"builtin/latest\" in the request."]
        pub segment_presence_label_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2LabelAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shotAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Shot annotations. Each shot is represented as a video segment."]
        pub shot_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2VideoSegment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shotLabelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Topical label annotations on shot level. There is exactly one element for each unique label."]
        pub shot_label_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2LabelAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shotPresenceLabelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Presence label annotations on shot level. There is exactly one element for each unique label. Compared to the existing topical `shot_label_annotations`, this field presents more fine-grained, shot-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to \"builtin/latest\" in the request."]
        pub shot_presence_label_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2LabelAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "speechTranscriptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Speech transcription."]
        pub speech_transcriptions: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2SpeechTranscription>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OCR text detection and tracking. Annotations for list of detected text snippets. Each will have list of frame information associated with it."]
        pub text_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2TextAnnotation>>,
        >,
    }
    impl GoogleCloudVideointelligenceV1beta2VideoAnnotationResults {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2VideoAnnotationResultsBuilder {
            GoogleCloudVideointelligenceV1beta2VideoAnnotationResultsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video segment."]
    pub struct GoogleCloudVideointelligenceV1beta2VideoSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTimeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the end of the segment (inclusive)."]
        pub end_time_offset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTimeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the start of the segment (inclusive)."]
        pub start_time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1beta2VideoSegment {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2VideoSegmentBuilder {
            GoogleCloudVideointelligenceV1beta2VideoSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Word-specific information for recognized words. Word information is only included in the response when certain request parameters are set, such as `enable_word_time_offsets`."]
    pub struct GoogleCloudVideointelligenceV1beta2WordInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time offset relative to the beginning of the audio, and corresponding to the end of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "speakerTag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A distinct integer value is assigned for every speaker within the audio. This field specifies which one of those speakers was detected to have spoken this word. Value ranges from 1 up to diarization_speaker_count, and is only set if speaker diarization is enabled."]
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
    impl GoogleCloudVideointelligenceV1beta2WordInfo {
        pub fn builder() -> GoogleCloudVideointelligenceV1beta2WordInfoBuilder {
            GoogleCloudVideointelligenceV1beta2WordInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video annotation progress. Included in the `metadata` field of the `Operation` returned by the `GetOperation` call of the `google::longrunning::Operations` service."]
    pub struct GoogleCloudVideointelligenceV1p1beta1AnnotateVideoProgress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationProgress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Progress metadata for all videos specified in `AnnotateVideoRequest`."]
        pub annotation_progress: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1VideoAnnotationProgress>,
            >,
        >,
    }
    impl GoogleCloudVideointelligenceV1p1beta1AnnotateVideoProgress {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1AnnotateVideoProgressBuilder {
            GoogleCloudVideointelligenceV1p1beta1AnnotateVideoProgressBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video annotation response. Included in the `response` field of the `Operation` returned by the `GetOperation` call of the `google::longrunning::Operations` service."]
    pub struct GoogleCloudVideointelligenceV1p1beta1AnnotateVideoResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Annotation results for all videos specified in `AnnotateVideoRequest`."]
        pub annotation_results: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1VideoAnnotationResults>,
            >,
        >,
    }
    impl GoogleCloudVideointelligenceV1p1beta1AnnotateVideoResponse {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1AnnotateVideoResponseBuilder {
            GoogleCloudVideointelligenceV1p1beta1AnnotateVideoResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic detected attribute represented by name in string format."]
    pub struct GoogleCloudVideointelligenceV1p1beta1DetectedAttribute {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected attribute confidence. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the attribute, for example, glasses, dark_glasses, mouth_open. A full list of supported type names will be provided in the document."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text value of the detection result. For example, the value for \"HairColor\" can be \"black\", \"blonde\", etc."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p1beta1DetectedAttribute {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1DetectedAttributeBuilder {
            GoogleCloudVideointelligenceV1p1beta1DetectedAttributeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic detected landmark represented by name in string format and a 2D location."]
    pub struct GoogleCloudVideointelligenceV1p1beta1DetectedLandmark {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The confidence score of the detected landmark. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of this landmark, for example, left_hand, right_shoulder."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "point")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The 2D point of the detected landmark using the normalized image coordindate system. The normalized coordinates have the range from 0 to 1."]
        pub point: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1NormalizedVertex>,
        >,
    }
    impl GoogleCloudVideointelligenceV1p1beta1DetectedLandmark {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1DetectedLandmarkBuilder {
            GoogleCloudVideointelligenceV1p1beta1DetectedLandmarkBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected entity from video analysis."]
    pub struct GoogleCloudVideointelligenceV1p1beta1Entity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Textual description, e.g., `Fixed-gear bicycle`."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Opaque entity ID. Some IDs may be available in [Google Knowledge Graph Search API](https://developers.google.com/knowledge-graph/)."]
        pub entity_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Language code for `description` in BCP-47 format."]
        pub language_code: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p1beta1Entity {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1EntityBuilder {
            GoogleCloudVideointelligenceV1p1beta1EntityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Explicit content annotation (based on per-frame visual signals only). If no explicit content has been detected in a frame, no annotations are present for that frame."]
    pub struct GoogleCloudVideointelligenceV1p1beta1ExplicitContentAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video frames where explicit content was detected."]
        pub frames: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1ExplicitContentFrame>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p1beta1ExplicitContentAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1ExplicitContentAnnotationBuilder {
            GoogleCloudVideointelligenceV1p1beta1ExplicitContentAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video frame level annotation results for explicit content."]
    pub struct GoogleCloudVideointelligenceV1p1beta1ExplicitContentFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pornographyLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Likelihood of the pornography content.."]
        pub pornography_likelihood: ::std::option::Option<
            GoogleCloudVideointelligenceV1p1beta1ExplicitContentFramePornographyLikelihoodEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p1beta1ExplicitContentFrame {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1ExplicitContentFrameBuilder {
            GoogleCloudVideointelligenceV1p1beta1ExplicitContentFrameBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Likelihood of the pornography content.."]
    pub enum GoogleCloudVideointelligenceV1p1beta1ExplicitContentFramePornographyLikelihoodEnum {
        #[serde(rename = "LIKELIHOOD_UNSPECIFIED")]
        #[doc = "Unspecified likelihood."]
        LikelihoodUnspecified,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "Very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "Unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "Possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "Likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "Very likely."]
        VeryLikely,
    }
    impl ::std::default::Default
        for GoogleCloudVideointelligenceV1p1beta1ExplicitContentFramePornographyLikelihoodEnum
    {
        fn default() -> Self {
            Self::LikelihoodUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deprecated. No effect."]
    pub struct GoogleCloudVideointelligenceV1p1beta1FaceAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video frames where a face was detected."]
        pub frames: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1FaceFrame>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video segments where a face was detected."]
        pub segments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1FaceSegment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Thumbnail of a representative face view (in JPEG format)."]
        pub thumbnail: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p1beta1FaceAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1FaceAnnotationBuilder {
            GoogleCloudVideointelligenceV1p1beta1FaceAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Face detection annotation."]
    pub struct GoogleCloudVideointelligenceV1p1beta1FaceDetectionAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The thumbnail of a person's face."]
        pub thumbnail: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tracks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The face tracks with attributes."]
        pub tracks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1Track>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p1beta1FaceDetectionAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1FaceDetectionAnnotationBuilder {
            GoogleCloudVideointelligenceV1p1beta1FaceDetectionAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deprecated. No effect."]
    pub struct GoogleCloudVideointelligenceV1p1beta1FaceFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedBoundingBoxes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Normalized Bounding boxes in a frame. There can be more than one boxes if the same face is detected in multiple locations within the current frame."]
        pub normalized_bounding_boxes: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1NormalizedBoundingBox>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p1beta1FaceFrame {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1FaceFrameBuilder {
            GoogleCloudVideointelligenceV1p1beta1FaceFrameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video segment level annotation results for face detection."]
    pub struct GoogleCloudVideointelligenceV1p1beta1FaceSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video segment where a face was detected."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1VideoSegment>,
        >,
    }
    impl GoogleCloudVideointelligenceV1p1beta1FaceSegment {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1FaceSegmentBuilder {
            GoogleCloudVideointelligenceV1p1beta1FaceSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Label annotation."]
    pub struct GoogleCloudVideointelligenceV1p1beta1LabelAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "categoryEntities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Common categories for the detected entity. For example, when the label is `Terrier`, the category is likely `dog`. And in some cases there might be more than one categories e.g., `Terrier` could also be a `pet`."]
        pub category_entities: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1Entity>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected entity."]
        pub entity:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1Entity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video frames where a label was detected."]
        pub frames: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1LabelFrame>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video segments where a label was detected."]
        pub segments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1LabelSegment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p1beta1LabelAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1LabelAnnotationBuilder {
            GoogleCloudVideointelligenceV1p1beta1LabelAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video frame level annotation results for label detection."]
    pub struct GoogleCloudVideointelligenceV1p1beta1LabelFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p1beta1LabelFrame {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1LabelFrameBuilder {
            GoogleCloudVideointelligenceV1p1beta1LabelFrameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video segment level annotation results for label detection."]
    pub struct GoogleCloudVideointelligenceV1p1beta1LabelSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video segment where a label was detected."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1VideoSegment>,
        >,
    }
    impl GoogleCloudVideointelligenceV1p1beta1LabelSegment {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1LabelSegmentBuilder {
            GoogleCloudVideointelligenceV1p1beta1LabelSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotation corresponding to one detected, tracked and recognized logo class."]
    pub struct GoogleCloudVideointelligenceV1p1beta1LogoRecognitionAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entity category information to specify the logo class that all the logo tracks within this LogoRecognitionAnnotation are recognized as."]
        pub entity:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1Entity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video segments where the recognized logo appears. There might be multiple instances of the same logo class appearing in one VideoSegment."]
        pub segments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1VideoSegment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tracks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All logo tracks where the recognized logo appears. Each track corresponds to one logo instance appearing in consecutive frames."]
        pub tracks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1Track>>,
        >,
    }
    impl GoogleCloudVideointelligenceV1p1beta1LogoRecognitionAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1LogoRecognitionAnnotationBuilder {
            GoogleCloudVideointelligenceV1p1beta1LogoRecognitionAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Normalized bounding box. The normalized vertex coordinates are relative to the original image. Range: [0, 1]."]
    pub struct GoogleCloudVideointelligenceV1p1beta1NormalizedBoundingBox {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bottom")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Bottom Y coordinate."]
        pub bottom: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "left")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Left X coordinate."]
        pub left: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "right")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Right X coordinate."]
        pub right: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "top")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Top Y coordinate."]
        pub top: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVideointelligenceV1p1beta1NormalizedBoundingBox {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1NormalizedBoundingBoxBuilder {
            GoogleCloudVideointelligenceV1p1beta1NormalizedBoundingBoxBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Normalized bounding polygon for text (that might not be aligned with axis). Contains list of the corner points in clockwise order starting from top-left corner. For example, for a rectangular bounding box: When the text is horizontal it might look like: 0----1 | | 3----2 When it's clockwise rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3). Note that values can be less than 0, or greater than 1 due to trignometric calculations for location of the box."]
    pub struct GoogleCloudVideointelligenceV1p1beta1NormalizedBoundingPoly {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vertices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Normalized vertices of the bounding polygon."]
        pub vertices: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1NormalizedVertex>,
            >,
        >,
    }
    impl GoogleCloudVideointelligenceV1p1beta1NormalizedBoundingPoly {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1NormalizedBoundingPolyBuilder {
            GoogleCloudVideointelligenceV1p1beta1NormalizedBoundingPolyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A vertex represents a 2D point in the image. NOTE: the normalized vertex coordinates are relative to the original image and range from 0 to 1."]
    pub struct GoogleCloudVideointelligenceV1p1beta1NormalizedVertex {
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
    impl GoogleCloudVideointelligenceV1p1beta1NormalizedVertex {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1NormalizedVertexBuilder {
            GoogleCloudVideointelligenceV1p1beta1NormalizedVertexBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotations corresponding to one tracked object."]
    pub struct GoogleCloudVideointelligenceV1p1beta1ObjectTrackingAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object category's labeling confidence of this track."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entity to specify the object category that this track is labeled as."]
        pub entity:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1Entity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information corresponding to all frames where this object track appears. Non-streaming batch mode: it may be one or multiple ObjectTrackingFrame messages in frames. Streaming mode: it can only be one ObjectTrackingFrame message in frames."]
        pub frames: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1ObjectTrackingFrame>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Non-streaming batch mode ONLY. Each object track corresponds to one video segment where it appears."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1VideoSegment>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trackId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Streaming mode ONLY. In streaming mode, we do not know the end time of a tracked object before it is completed. Hence, there is no VideoSegment info returned. Instead, we provide a unique identifiable integer track_id so that the customers can correlate the results of the ongoing ObjectTrackAnnotation of the same track_id over time."]
        pub track_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p1beta1ObjectTrackingAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1ObjectTrackingAnnotationBuilder {
            GoogleCloudVideointelligenceV1p1beta1ObjectTrackingAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video frame level annotations for object detection and tracking. This field stores per frame location, time offset, and confidence."]
    pub struct GoogleCloudVideointelligenceV1p1beta1ObjectTrackingFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedBoundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The normalized bounding box location of this object track for the frame."]
        pub normalized_bounding_box: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1NormalizedBoundingBox>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp of the frame in microseconds."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p1beta1ObjectTrackingFrame {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1ObjectTrackingFrameBuilder {
            GoogleCloudVideointelligenceV1p1beta1ObjectTrackingFrameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Person detection annotation per video."]
    pub struct GoogleCloudVideointelligenceV1p1beta1PersonDetectionAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tracks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The detected tracks of a person."]
        pub tracks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1Track>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p1beta1PersonDetectionAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1PersonDetectionAnnotationBuilder {
            GoogleCloudVideointelligenceV1p1beta1PersonDetectionAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Alternative hypotheses (a.k.a. n-best list)."]
    pub struct GoogleCloudVideointelligenceV1p1beta1SpeechRecognitionAlternative {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transcript")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Transcript text representing the words that the user spoke."]
        pub transcript: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "words")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A list of word-specific information for each recognized word. Note: When `enable_speaker_diarization` is set to true, you will see all the words from the beginning of the audio."]
        pub words: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1WordInfo>>,
        >,
    }
    impl GoogleCloudVideointelligenceV1p1beta1SpeechRecognitionAlternative {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1SpeechRecognitionAlternativeBuilder
        {
            GoogleCloudVideointelligenceV1p1beta1SpeechRecognitionAlternativeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A speech recognition result corresponding to a portion of the audio."]
    pub struct GoogleCloudVideointelligenceV1p1beta1SpeechTranscription {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alternatives")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "May contain one or more recognition hypotheses (up to the maximum specified in `max_alternatives`). These alternatives are ordered in terms of accuracy, with the top (first) alternative being the most probable, as ranked by the recognizer."]
        pub alternatives: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<
                    GoogleCloudVideointelligenceV1p1beta1SpeechRecognitionAlternative,
                >,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag of the language in this result. This language code was detected to have the most likelihood of being spoken in the audio."]
        pub language_code: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p1beta1SpeechTranscription {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1SpeechTranscriptionBuilder {
            GoogleCloudVideointelligenceV1p1beta1SpeechTranscriptionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotations related to one detected OCR text snippet. This will contain the corresponding text, confidence value, and frame level information for each detection."]
    pub struct GoogleCloudVideointelligenceV1p1beta1TextAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video segments where OCR detected text appears."]
        pub segments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1TextSegment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The detected text."]
        pub text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p1beta1TextAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1TextAnnotationBuilder {
            GoogleCloudVideointelligenceV1p1beta1TextAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video frame level annotation results for text annotation (OCR). Contains information regarding timestamp and bounding box locations for the frames containing detected OCR text snippets."]
    pub struct GoogleCloudVideointelligenceV1p1beta1TextFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rotatedBoundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Bounding polygon of the detected text for this frame."]
        pub rotated_bounding_box: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1NormalizedBoundingPoly>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp of this frame."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p1beta1TextFrame {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1TextFrameBuilder {
            GoogleCloudVideointelligenceV1p1beta1TextFrameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video segment level annotation results for text detection."]
    pub struct GoogleCloudVideointelligenceV1p1beta1TextSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence for the track of detected text. It is calculated as the highest over all frames where OCR detected text appears."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information related to the frames where OCR detected text appears."]
        pub frames: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1TextFrame>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video segment where a text snippet was detected."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1VideoSegment>,
        >,
    }
    impl GoogleCloudVideointelligenceV1p1beta1TextSegment {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1TextSegmentBuilder {
            GoogleCloudVideointelligenceV1p1beta1TextSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "For tracking related features. An object at time_offset with attributes, and located with normalized_bounding_box."]
    pub struct GoogleCloudVideointelligenceV1p1beta1TimestampedObject {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The attributes of the object in the bounding box."]
        pub attributes: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1DetectedAttribute>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "landmarks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The detected landmarks."]
        pub landmarks: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1DetectedLandmark>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedBoundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Normalized Bounding box in a frame, where the object is located."]
        pub normalized_bounding_box: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1NormalizedBoundingBox>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this object."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p1beta1TimestampedObject {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1TimestampedObjectBuilder {
            GoogleCloudVideointelligenceV1p1beta1TimestampedObjectBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A track of an object instance."]
    pub struct GoogleCloudVideointelligenceV1p1beta1Track {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Attributes in the track level."]
        pub attributes: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1DetectedAttribute>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The confidence score of the tracked object."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video segment of a track."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1VideoSegment>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestampedObjects")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object with timestamp and attributes per frame in the track."]
        pub timestamped_objects: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1TimestampedObject>,
            >,
        >,
    }
    impl GoogleCloudVideointelligenceV1p1beta1Track {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1TrackBuilder {
            GoogleCloudVideointelligenceV1p1beta1TrackBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotation progress for a single video."]
    pub struct GoogleCloudVideointelligenceV1p1beta1VideoAnnotationProgress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "feature")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies which feature is being tracked if the request contains more than one feature."]
        pub feature: ::std::option::Option<
            GoogleCloudVideointelligenceV1p1beta1VideoAnnotationProgressFeatureEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
        pub input_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressPercent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Approximate percentage processed thus far. Guaranteed to be 100 when fully processed."]
        pub progress_percent: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies which segment is being tracked if the request contains more than one segment."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1VideoSegment>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time when the request was received."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time of the most recent update."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p1beta1VideoAnnotationProgress {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1VideoAnnotationProgressBuilder {
            GoogleCloudVideointelligenceV1p1beta1VideoAnnotationProgressBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies which feature is being tracked if the request contains more than one feature."]
    pub enum GoogleCloudVideointelligenceV1p1beta1VideoAnnotationProgressFeatureEnum {
        #[serde(rename = "FEATURE_UNSPECIFIED")]
        #[doc = "Unspecified."]
        FeatureUnspecified,
        #[serde(rename = "LABEL_DETECTION")]
        #[doc = "Label detection. Detect objects, such as dog or flower."]
        LabelDetection,
        #[serde(rename = "SHOT_CHANGE_DETECTION")]
        #[doc = "Shot change detection."]
        ShotChangeDetection,
        #[serde(rename = "EXPLICIT_CONTENT_DETECTION")]
        #[doc = "Explicit content detection."]
        ExplicitContentDetection,
        #[serde(rename = "FACE_DETECTION")]
        #[doc = "Human face detection."]
        FaceDetection,
        #[serde(rename = "SPEECH_TRANSCRIPTION")]
        #[doc = "Speech transcription."]
        SpeechTranscription,
        #[serde(rename = "TEXT_DETECTION")]
        #[doc = "OCR text detection and tracking."]
        TextDetection,
        #[serde(rename = "OBJECT_TRACKING")]
        #[doc = "Object detection and tracking."]
        ObjectTracking,
        #[serde(rename = "LOGO_RECOGNITION")]
        #[doc = "Logo detection, tracking, and recognition."]
        LogoRecognition,
        #[serde(rename = "PERSON_DETECTION")]
        #[doc = "Person detection."]
        PersonDetection,
    }
    impl ::std::default::Default
        for GoogleCloudVideointelligenceV1p1beta1VideoAnnotationProgressFeatureEnum
    {
        fn default() -> Self {
            Self::FeatureUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotation results for a single video."]
    pub struct GoogleCloudVideointelligenceV1p1beta1VideoAnnotationResults {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, indicates an error. Note that for a single `AnnotateVideoRequest` some videos may succeed and some may fail."]
        pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "explicitAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Explicit content annotation."]
        pub explicit_annotation: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1ExplicitContentAnnotation>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "faceAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Please use `face_detection_annotations` instead."]
        pub face_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1FaceAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "faceDetectionAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Face detection annotations."]
        pub face_detection_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1FaceDetectionAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frameLabelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label annotations on frame level. There is exactly one element for each unique label."]
        pub frame_label_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1LabelAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
        pub input_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logoRecognitionAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Annotations for list of logos detected, tracked and recognized in video."]
        pub logo_recognition_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1LogoRecognitionAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Annotations for list of objects detected and tracked in video."]
        pub object_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1ObjectTrackingAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "personDetectionAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Person detection annotations."]
        pub person_detection_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1PersonDetectionAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video segment on which the annotation is run."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1VideoSegment>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segmentLabelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Topical label annotations on video level or user-specified segment level. There is exactly one element for each unique label."]
        pub segment_label_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1LabelAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segmentPresenceLabelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Presence label annotations on video level or user-specified segment level. There is exactly one element for each unique label. Compared to the existing topical `segment_label_annotations`, this field presents more fine-grained, segment-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to \"builtin/latest\" in the request."]
        pub segment_presence_label_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1LabelAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shotAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Shot annotations. Each shot is represented as a video segment."]
        pub shot_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1VideoSegment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shotLabelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Topical label annotations on shot level. There is exactly one element for each unique label."]
        pub shot_label_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1LabelAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shotPresenceLabelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Presence label annotations on shot level. There is exactly one element for each unique label. Compared to the existing topical `shot_label_annotations`, this field presents more fine-grained, shot-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to \"builtin/latest\" in the request."]
        pub shot_presence_label_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1LabelAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "speechTranscriptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Speech transcription."]
        pub speech_transcriptions: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1SpeechTranscription>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OCR text detection and tracking. Annotations for list of detected text snippets. Each will have list of frame information associated with it."]
        pub text_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1TextAnnotation>>,
        >,
    }
    impl GoogleCloudVideointelligenceV1p1beta1VideoAnnotationResults {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1VideoAnnotationResultsBuilder {
            GoogleCloudVideointelligenceV1p1beta1VideoAnnotationResultsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video segment."]
    pub struct GoogleCloudVideointelligenceV1p1beta1VideoSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTimeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the end of the segment (inclusive)."]
        pub end_time_offset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTimeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the start of the segment (inclusive)."]
        pub start_time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p1beta1VideoSegment {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1VideoSegmentBuilder {
            GoogleCloudVideointelligenceV1p1beta1VideoSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Word-specific information for recognized words. Word information is only included in the response when certain request parameters are set, such as `enable_word_time_offsets`."]
    pub struct GoogleCloudVideointelligenceV1p1beta1WordInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time offset relative to the beginning of the audio, and corresponding to the end of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "speakerTag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A distinct integer value is assigned for every speaker within the audio. This field specifies which one of those speakers was detected to have spoken this word. Value ranges from 1 up to diarization_speaker_count, and is only set if speaker diarization is enabled."]
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
    impl GoogleCloudVideointelligenceV1p1beta1WordInfo {
        pub fn builder() -> GoogleCloudVideointelligenceV1p1beta1WordInfoBuilder {
            GoogleCloudVideointelligenceV1p1beta1WordInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video annotation progress. Included in the `metadata` field of the `Operation` returned by the `GetOperation` call of the `google::longrunning::Operations` service."]
    pub struct GoogleCloudVideointelligenceV1p2beta1AnnotateVideoProgress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationProgress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Progress metadata for all videos specified in `AnnotateVideoRequest`."]
        pub annotation_progress: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1VideoAnnotationProgress>,
            >,
        >,
    }
    impl GoogleCloudVideointelligenceV1p2beta1AnnotateVideoProgress {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1AnnotateVideoProgressBuilder {
            GoogleCloudVideointelligenceV1p2beta1AnnotateVideoProgressBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video annotation response. Included in the `response` field of the `Operation` returned by the `GetOperation` call of the `google::longrunning::Operations` service."]
    pub struct GoogleCloudVideointelligenceV1p2beta1AnnotateVideoResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Annotation results for all videos specified in `AnnotateVideoRequest`."]
        pub annotation_results: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1VideoAnnotationResults>,
            >,
        >,
    }
    impl GoogleCloudVideointelligenceV1p2beta1AnnotateVideoResponse {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1AnnotateVideoResponseBuilder {
            GoogleCloudVideointelligenceV1p2beta1AnnotateVideoResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic detected attribute represented by name in string format."]
    pub struct GoogleCloudVideointelligenceV1p2beta1DetectedAttribute {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected attribute confidence. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the attribute, for example, glasses, dark_glasses, mouth_open. A full list of supported type names will be provided in the document."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text value of the detection result. For example, the value for \"HairColor\" can be \"black\", \"blonde\", etc."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p2beta1DetectedAttribute {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1DetectedAttributeBuilder {
            GoogleCloudVideointelligenceV1p2beta1DetectedAttributeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic detected landmark represented by name in string format and a 2D location."]
    pub struct GoogleCloudVideointelligenceV1p2beta1DetectedLandmark {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The confidence score of the detected landmark. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of this landmark, for example, left_hand, right_shoulder."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "point")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The 2D point of the detected landmark using the normalized image coordindate system. The normalized coordinates have the range from 0 to 1."]
        pub point: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1NormalizedVertex>,
        >,
    }
    impl GoogleCloudVideointelligenceV1p2beta1DetectedLandmark {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1DetectedLandmarkBuilder {
            GoogleCloudVideointelligenceV1p2beta1DetectedLandmarkBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected entity from video analysis."]
    pub struct GoogleCloudVideointelligenceV1p2beta1Entity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Textual description, e.g., `Fixed-gear bicycle`."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Opaque entity ID. Some IDs may be available in [Google Knowledge Graph Search API](https://developers.google.com/knowledge-graph/)."]
        pub entity_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Language code for `description` in BCP-47 format."]
        pub language_code: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p2beta1Entity {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1EntityBuilder {
            GoogleCloudVideointelligenceV1p2beta1EntityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Explicit content annotation (based on per-frame visual signals only). If no explicit content has been detected in a frame, no annotations are present for that frame."]
    pub struct GoogleCloudVideointelligenceV1p2beta1ExplicitContentAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video frames where explicit content was detected."]
        pub frames: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1ExplicitContentFrame>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p2beta1ExplicitContentAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1ExplicitContentAnnotationBuilder {
            GoogleCloudVideointelligenceV1p2beta1ExplicitContentAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video frame level annotation results for explicit content."]
    pub struct GoogleCloudVideointelligenceV1p2beta1ExplicitContentFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pornographyLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Likelihood of the pornography content.."]
        pub pornography_likelihood: ::std::option::Option<
            GoogleCloudVideointelligenceV1p2beta1ExplicitContentFramePornographyLikelihoodEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p2beta1ExplicitContentFrame {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1ExplicitContentFrameBuilder {
            GoogleCloudVideointelligenceV1p2beta1ExplicitContentFrameBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Likelihood of the pornography content.."]
    pub enum GoogleCloudVideointelligenceV1p2beta1ExplicitContentFramePornographyLikelihoodEnum {
        #[serde(rename = "LIKELIHOOD_UNSPECIFIED")]
        #[doc = "Unspecified likelihood."]
        LikelihoodUnspecified,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "Very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "Unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "Possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "Likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "Very likely."]
        VeryLikely,
    }
    impl ::std::default::Default
        for GoogleCloudVideointelligenceV1p2beta1ExplicitContentFramePornographyLikelihoodEnum
    {
        fn default() -> Self {
            Self::LikelihoodUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deprecated. No effect."]
    pub struct GoogleCloudVideointelligenceV1p2beta1FaceAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video frames where a face was detected."]
        pub frames: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1FaceFrame>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video segments where a face was detected."]
        pub segments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1FaceSegment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Thumbnail of a representative face view (in JPEG format)."]
        pub thumbnail: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p2beta1FaceAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1FaceAnnotationBuilder {
            GoogleCloudVideointelligenceV1p2beta1FaceAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Face detection annotation."]
    pub struct GoogleCloudVideointelligenceV1p2beta1FaceDetectionAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The thumbnail of a person's face."]
        pub thumbnail: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tracks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The face tracks with attributes."]
        pub tracks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1Track>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p2beta1FaceDetectionAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1FaceDetectionAnnotationBuilder {
            GoogleCloudVideointelligenceV1p2beta1FaceDetectionAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deprecated. No effect."]
    pub struct GoogleCloudVideointelligenceV1p2beta1FaceFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedBoundingBoxes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Normalized Bounding boxes in a frame. There can be more than one boxes if the same face is detected in multiple locations within the current frame."]
        pub normalized_bounding_boxes: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1NormalizedBoundingBox>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p2beta1FaceFrame {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1FaceFrameBuilder {
            GoogleCloudVideointelligenceV1p2beta1FaceFrameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video segment level annotation results for face detection."]
    pub struct GoogleCloudVideointelligenceV1p2beta1FaceSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video segment where a face was detected."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1VideoSegment>,
        >,
    }
    impl GoogleCloudVideointelligenceV1p2beta1FaceSegment {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1FaceSegmentBuilder {
            GoogleCloudVideointelligenceV1p2beta1FaceSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Label annotation."]
    pub struct GoogleCloudVideointelligenceV1p2beta1LabelAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "categoryEntities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Common categories for the detected entity. For example, when the label is `Terrier`, the category is likely `dog`. And in some cases there might be more than one categories e.g., `Terrier` could also be a `pet`."]
        pub category_entities: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1Entity>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected entity."]
        pub entity:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1Entity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video frames where a label was detected."]
        pub frames: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1LabelFrame>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video segments where a label was detected."]
        pub segments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1LabelSegment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p2beta1LabelAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1LabelAnnotationBuilder {
            GoogleCloudVideointelligenceV1p2beta1LabelAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video frame level annotation results for label detection."]
    pub struct GoogleCloudVideointelligenceV1p2beta1LabelFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p2beta1LabelFrame {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1LabelFrameBuilder {
            GoogleCloudVideointelligenceV1p2beta1LabelFrameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video segment level annotation results for label detection."]
    pub struct GoogleCloudVideointelligenceV1p2beta1LabelSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video segment where a label was detected."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1VideoSegment>,
        >,
    }
    impl GoogleCloudVideointelligenceV1p2beta1LabelSegment {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1LabelSegmentBuilder {
            GoogleCloudVideointelligenceV1p2beta1LabelSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotation corresponding to one detected, tracked and recognized logo class."]
    pub struct GoogleCloudVideointelligenceV1p2beta1LogoRecognitionAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entity category information to specify the logo class that all the logo tracks within this LogoRecognitionAnnotation are recognized as."]
        pub entity:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1Entity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video segments where the recognized logo appears. There might be multiple instances of the same logo class appearing in one VideoSegment."]
        pub segments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1VideoSegment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tracks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All logo tracks where the recognized logo appears. Each track corresponds to one logo instance appearing in consecutive frames."]
        pub tracks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1Track>>,
        >,
    }
    impl GoogleCloudVideointelligenceV1p2beta1LogoRecognitionAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1LogoRecognitionAnnotationBuilder {
            GoogleCloudVideointelligenceV1p2beta1LogoRecognitionAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Normalized bounding box. The normalized vertex coordinates are relative to the original image. Range: [0, 1]."]
    pub struct GoogleCloudVideointelligenceV1p2beta1NormalizedBoundingBox {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bottom")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Bottom Y coordinate."]
        pub bottom: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "left")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Left X coordinate."]
        pub left: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "right")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Right X coordinate."]
        pub right: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "top")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Top Y coordinate."]
        pub top: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVideointelligenceV1p2beta1NormalizedBoundingBox {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1NormalizedBoundingBoxBuilder {
            GoogleCloudVideointelligenceV1p2beta1NormalizedBoundingBoxBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Normalized bounding polygon for text (that might not be aligned with axis). Contains list of the corner points in clockwise order starting from top-left corner. For example, for a rectangular bounding box: When the text is horizontal it might look like: 0----1 | | 3----2 When it's clockwise rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3). Note that values can be less than 0, or greater than 1 due to trignometric calculations for location of the box."]
    pub struct GoogleCloudVideointelligenceV1p2beta1NormalizedBoundingPoly {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vertices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Normalized vertices of the bounding polygon."]
        pub vertices: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1NormalizedVertex>,
            >,
        >,
    }
    impl GoogleCloudVideointelligenceV1p2beta1NormalizedBoundingPoly {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1NormalizedBoundingPolyBuilder {
            GoogleCloudVideointelligenceV1p2beta1NormalizedBoundingPolyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A vertex represents a 2D point in the image. NOTE: the normalized vertex coordinates are relative to the original image and range from 0 to 1."]
    pub struct GoogleCloudVideointelligenceV1p2beta1NormalizedVertex {
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
    impl GoogleCloudVideointelligenceV1p2beta1NormalizedVertex {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1NormalizedVertexBuilder {
            GoogleCloudVideointelligenceV1p2beta1NormalizedVertexBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotations corresponding to one tracked object."]
    pub struct GoogleCloudVideointelligenceV1p2beta1ObjectTrackingAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object category's labeling confidence of this track."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entity to specify the object category that this track is labeled as."]
        pub entity:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1Entity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information corresponding to all frames where this object track appears. Non-streaming batch mode: it may be one or multiple ObjectTrackingFrame messages in frames. Streaming mode: it can only be one ObjectTrackingFrame message in frames."]
        pub frames: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1ObjectTrackingFrame>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Non-streaming batch mode ONLY. Each object track corresponds to one video segment where it appears."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1VideoSegment>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trackId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Streaming mode ONLY. In streaming mode, we do not know the end time of a tracked object before it is completed. Hence, there is no VideoSegment info returned. Instead, we provide a unique identifiable integer track_id so that the customers can correlate the results of the ongoing ObjectTrackAnnotation of the same track_id over time."]
        pub track_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p2beta1ObjectTrackingAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1ObjectTrackingAnnotationBuilder {
            GoogleCloudVideointelligenceV1p2beta1ObjectTrackingAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video frame level annotations for object detection and tracking. This field stores per frame location, time offset, and confidence."]
    pub struct GoogleCloudVideointelligenceV1p2beta1ObjectTrackingFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedBoundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The normalized bounding box location of this object track for the frame."]
        pub normalized_bounding_box: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1NormalizedBoundingBox>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp of the frame in microseconds."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p2beta1ObjectTrackingFrame {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1ObjectTrackingFrameBuilder {
            GoogleCloudVideointelligenceV1p2beta1ObjectTrackingFrameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Person detection annotation per video."]
    pub struct GoogleCloudVideointelligenceV1p2beta1PersonDetectionAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tracks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The detected tracks of a person."]
        pub tracks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1Track>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p2beta1PersonDetectionAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1PersonDetectionAnnotationBuilder {
            GoogleCloudVideointelligenceV1p2beta1PersonDetectionAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Alternative hypotheses (a.k.a. n-best list)."]
    pub struct GoogleCloudVideointelligenceV1p2beta1SpeechRecognitionAlternative {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transcript")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Transcript text representing the words that the user spoke."]
        pub transcript: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "words")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A list of word-specific information for each recognized word. Note: When `enable_speaker_diarization` is set to true, you will see all the words from the beginning of the audio."]
        pub words: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1WordInfo>>,
        >,
    }
    impl GoogleCloudVideointelligenceV1p2beta1SpeechRecognitionAlternative {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1SpeechRecognitionAlternativeBuilder
        {
            GoogleCloudVideointelligenceV1p2beta1SpeechRecognitionAlternativeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A speech recognition result corresponding to a portion of the audio."]
    pub struct GoogleCloudVideointelligenceV1p2beta1SpeechTranscription {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alternatives")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "May contain one or more recognition hypotheses (up to the maximum specified in `max_alternatives`). These alternatives are ordered in terms of accuracy, with the top (first) alternative being the most probable, as ranked by the recognizer."]
        pub alternatives: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<
                    GoogleCloudVideointelligenceV1p2beta1SpeechRecognitionAlternative,
                >,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag of the language in this result. This language code was detected to have the most likelihood of being spoken in the audio."]
        pub language_code: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p2beta1SpeechTranscription {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1SpeechTranscriptionBuilder {
            GoogleCloudVideointelligenceV1p2beta1SpeechTranscriptionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotations related to one detected OCR text snippet. This will contain the corresponding text, confidence value, and frame level information for each detection."]
    pub struct GoogleCloudVideointelligenceV1p2beta1TextAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video segments where OCR detected text appears."]
        pub segments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1TextSegment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The detected text."]
        pub text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p2beta1TextAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1TextAnnotationBuilder {
            GoogleCloudVideointelligenceV1p2beta1TextAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video frame level annotation results for text annotation (OCR). Contains information regarding timestamp and bounding box locations for the frames containing detected OCR text snippets."]
    pub struct GoogleCloudVideointelligenceV1p2beta1TextFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rotatedBoundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Bounding polygon of the detected text for this frame."]
        pub rotated_bounding_box: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1NormalizedBoundingPoly>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp of this frame."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p2beta1TextFrame {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1TextFrameBuilder {
            GoogleCloudVideointelligenceV1p2beta1TextFrameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video segment level annotation results for text detection."]
    pub struct GoogleCloudVideointelligenceV1p2beta1TextSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence for the track of detected text. It is calculated as the highest over all frames where OCR detected text appears."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information related to the frames where OCR detected text appears."]
        pub frames: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1TextFrame>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video segment where a text snippet was detected."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1VideoSegment>,
        >,
    }
    impl GoogleCloudVideointelligenceV1p2beta1TextSegment {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1TextSegmentBuilder {
            GoogleCloudVideointelligenceV1p2beta1TextSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "For tracking related features. An object at time_offset with attributes, and located with normalized_bounding_box."]
    pub struct GoogleCloudVideointelligenceV1p2beta1TimestampedObject {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The attributes of the object in the bounding box."]
        pub attributes: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1DetectedAttribute>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "landmarks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The detected landmarks."]
        pub landmarks: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1DetectedLandmark>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedBoundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Normalized Bounding box in a frame, where the object is located."]
        pub normalized_bounding_box: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1NormalizedBoundingBox>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this object."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p2beta1TimestampedObject {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1TimestampedObjectBuilder {
            GoogleCloudVideointelligenceV1p2beta1TimestampedObjectBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A track of an object instance."]
    pub struct GoogleCloudVideointelligenceV1p2beta1Track {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Attributes in the track level."]
        pub attributes: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1DetectedAttribute>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The confidence score of the tracked object."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video segment of a track."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1VideoSegment>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestampedObjects")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object with timestamp and attributes per frame in the track."]
        pub timestamped_objects: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1TimestampedObject>,
            >,
        >,
    }
    impl GoogleCloudVideointelligenceV1p2beta1Track {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1TrackBuilder {
            GoogleCloudVideointelligenceV1p2beta1TrackBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotation progress for a single video."]
    pub struct GoogleCloudVideointelligenceV1p2beta1VideoAnnotationProgress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "feature")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies which feature is being tracked if the request contains more than one feature."]
        pub feature: ::std::option::Option<
            GoogleCloudVideointelligenceV1p2beta1VideoAnnotationProgressFeatureEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
        pub input_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressPercent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Approximate percentage processed thus far. Guaranteed to be 100 when fully processed."]
        pub progress_percent: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies which segment is being tracked if the request contains more than one segment."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1VideoSegment>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time when the request was received."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time of the most recent update."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p2beta1VideoAnnotationProgress {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1VideoAnnotationProgressBuilder {
            GoogleCloudVideointelligenceV1p2beta1VideoAnnotationProgressBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies which feature is being tracked if the request contains more than one feature."]
    pub enum GoogleCloudVideointelligenceV1p2beta1VideoAnnotationProgressFeatureEnum {
        #[serde(rename = "FEATURE_UNSPECIFIED")]
        #[doc = "Unspecified."]
        FeatureUnspecified,
        #[serde(rename = "LABEL_DETECTION")]
        #[doc = "Label detection. Detect objects, such as dog or flower."]
        LabelDetection,
        #[serde(rename = "SHOT_CHANGE_DETECTION")]
        #[doc = "Shot change detection."]
        ShotChangeDetection,
        #[serde(rename = "EXPLICIT_CONTENT_DETECTION")]
        #[doc = "Explicit content detection."]
        ExplicitContentDetection,
        #[serde(rename = "FACE_DETECTION")]
        #[doc = "Human face detection."]
        FaceDetection,
        #[serde(rename = "SPEECH_TRANSCRIPTION")]
        #[doc = "Speech transcription."]
        SpeechTranscription,
        #[serde(rename = "TEXT_DETECTION")]
        #[doc = "OCR text detection and tracking."]
        TextDetection,
        #[serde(rename = "OBJECT_TRACKING")]
        #[doc = "Object detection and tracking."]
        ObjectTracking,
        #[serde(rename = "LOGO_RECOGNITION")]
        #[doc = "Logo detection, tracking, and recognition."]
        LogoRecognition,
        #[serde(rename = "PERSON_DETECTION")]
        #[doc = "Person detection."]
        PersonDetection,
    }
    impl ::std::default::Default
        for GoogleCloudVideointelligenceV1p2beta1VideoAnnotationProgressFeatureEnum
    {
        fn default() -> Self {
            Self::FeatureUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotation results for a single video."]
    pub struct GoogleCloudVideointelligenceV1p2beta1VideoAnnotationResults {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, indicates an error. Note that for a single `AnnotateVideoRequest` some videos may succeed and some may fail."]
        pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "explicitAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Explicit content annotation."]
        pub explicit_annotation: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1ExplicitContentAnnotation>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "faceAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Please use `face_detection_annotations` instead."]
        pub face_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1FaceAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "faceDetectionAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Face detection annotations."]
        pub face_detection_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1FaceDetectionAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frameLabelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label annotations on frame level. There is exactly one element for each unique label."]
        pub frame_label_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1LabelAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
        pub input_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logoRecognitionAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Annotations for list of logos detected, tracked and recognized in video."]
        pub logo_recognition_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1LogoRecognitionAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Annotations for list of objects detected and tracked in video."]
        pub object_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1ObjectTrackingAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "personDetectionAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Person detection annotations."]
        pub person_detection_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1PersonDetectionAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video segment on which the annotation is run."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1VideoSegment>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segmentLabelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Topical label annotations on video level or user-specified segment level. There is exactly one element for each unique label."]
        pub segment_label_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1LabelAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segmentPresenceLabelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Presence label annotations on video level or user-specified segment level. There is exactly one element for each unique label. Compared to the existing topical `segment_label_annotations`, this field presents more fine-grained, segment-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to \"builtin/latest\" in the request."]
        pub segment_presence_label_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1LabelAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shotAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Shot annotations. Each shot is represented as a video segment."]
        pub shot_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1VideoSegment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shotLabelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Topical label annotations on shot level. There is exactly one element for each unique label."]
        pub shot_label_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1LabelAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shotPresenceLabelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Presence label annotations on shot level. There is exactly one element for each unique label. Compared to the existing topical `shot_label_annotations`, this field presents more fine-grained, shot-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to \"builtin/latest\" in the request."]
        pub shot_presence_label_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1LabelAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "speechTranscriptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Speech transcription."]
        pub speech_transcriptions: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1SpeechTranscription>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OCR text detection and tracking. Annotations for list of detected text snippets. Each will have list of frame information associated with it."]
        pub text_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1TextAnnotation>>,
        >,
    }
    impl GoogleCloudVideointelligenceV1p2beta1VideoAnnotationResults {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1VideoAnnotationResultsBuilder {
            GoogleCloudVideointelligenceV1p2beta1VideoAnnotationResultsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video segment."]
    pub struct GoogleCloudVideointelligenceV1p2beta1VideoSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTimeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the end of the segment (inclusive)."]
        pub end_time_offset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTimeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the start of the segment (inclusive)."]
        pub start_time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p2beta1VideoSegment {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1VideoSegmentBuilder {
            GoogleCloudVideointelligenceV1p2beta1VideoSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Word-specific information for recognized words. Word information is only included in the response when certain request parameters are set, such as `enable_word_time_offsets`."]
    pub struct GoogleCloudVideointelligenceV1p2beta1WordInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time offset relative to the beginning of the audio, and corresponding to the end of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "speakerTag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A distinct integer value is assigned for every speaker within the audio. This field specifies which one of those speakers was detected to have spoken this word. Value ranges from 1 up to diarization_speaker_count, and is only set if speaker diarization is enabled."]
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
    impl GoogleCloudVideointelligenceV1p2beta1WordInfo {
        pub fn builder() -> GoogleCloudVideointelligenceV1p2beta1WordInfoBuilder {
            GoogleCloudVideointelligenceV1p2beta1WordInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video annotation progress. Included in the `metadata` field of the `Operation` returned by the `GetOperation` call of the `google::longrunning::Operations` service."]
    pub struct GoogleCloudVideointelligenceV1p3beta1AnnotateVideoProgress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationProgress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Progress metadata for all videos specified in `AnnotateVideoRequest`."]
        pub annotation_progress: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1VideoAnnotationProgress>,
            >,
        >,
    }
    impl GoogleCloudVideointelligenceV1p3beta1AnnotateVideoProgress {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1AnnotateVideoProgressBuilder {
            GoogleCloudVideointelligenceV1p3beta1AnnotateVideoProgressBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video annotation response. Included in the `response` field of the `Operation` returned by the `GetOperation` call of the `google::longrunning::Operations` service."]
    pub struct GoogleCloudVideointelligenceV1p3beta1AnnotateVideoResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Annotation results for all videos specified in `AnnotateVideoRequest`."]
        pub annotation_results: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1VideoAnnotationResults>,
            >,
        >,
    }
    impl GoogleCloudVideointelligenceV1p3beta1AnnotateVideoResponse {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1AnnotateVideoResponseBuilder {
            GoogleCloudVideointelligenceV1p3beta1AnnotateVideoResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Celebrity definition."]
    pub struct GoogleCloudVideointelligenceV1p3beta1Celebrity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Textual description of additional information about the celebrity, if applicable."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The celebrity name."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the celebrity. Have the format `video-intelligence/kg-mid` indicates a celebrity from preloaded gallery. kg-mid is the id in Google knowledge graph, which is unique for the celebrity."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p3beta1Celebrity {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1CelebrityBuilder {
            GoogleCloudVideointelligenceV1p3beta1CelebrityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Celebrity recognition annotation per video."]
    pub struct GoogleCloudVideointelligenceV1p3beta1CelebrityRecognitionAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "celebrityTracks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The tracks detected from the input video, including recognized celebrities and other detected faces in the video."]
        pub celebrity_tracks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1CelebrityTrack>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p3beta1CelebrityRecognitionAnnotation {
        pub fn builder(
        ) -> GoogleCloudVideointelligenceV1p3beta1CelebrityRecognitionAnnotationBuilder {
            GoogleCloudVideointelligenceV1p3beta1CelebrityRecognitionAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The annotation result of a celebrity face track. RecognizedCelebrity field could be empty if the face track does not have any matched celebrities."]
    pub struct GoogleCloudVideointelligenceV1p3beta1CelebrityTrack {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "celebrities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Top N match of the celebrities for the face in this track."]
        pub celebrities: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1RecognizedCelebrity>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "faceTrack")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A track of a person's face."]
        pub face_track:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1Track>>,
    }
    impl GoogleCloudVideointelligenceV1p3beta1CelebrityTrack {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1CelebrityTrackBuilder {
            GoogleCloudVideointelligenceV1p3beta1CelebrityTrackBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic detected attribute represented by name in string format."]
    pub struct GoogleCloudVideointelligenceV1p3beta1DetectedAttribute {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected attribute confidence. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the attribute, for example, glasses, dark_glasses, mouth_open. A full list of supported type names will be provided in the document."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text value of the detection result. For example, the value for \"HairColor\" can be \"black\", \"blonde\", etc."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p3beta1DetectedAttribute {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1DetectedAttributeBuilder {
            GoogleCloudVideointelligenceV1p3beta1DetectedAttributeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic detected landmark represented by name in string format and a 2D location."]
    pub struct GoogleCloudVideointelligenceV1p3beta1DetectedLandmark {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The confidence score of the detected landmark. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of this landmark, for example, left_hand, right_shoulder."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "point")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The 2D point of the detected landmark using the normalized image coordindate system. The normalized coordinates have the range from 0 to 1."]
        pub point: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1NormalizedVertex>,
        >,
    }
    impl GoogleCloudVideointelligenceV1p3beta1DetectedLandmark {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1DetectedLandmarkBuilder {
            GoogleCloudVideointelligenceV1p3beta1DetectedLandmarkBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected entity from video analysis."]
    pub struct GoogleCloudVideointelligenceV1p3beta1Entity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Textual description, e.g., `Fixed-gear bicycle`."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Opaque entity ID. Some IDs may be available in [Google Knowledge Graph Search API](https://developers.google.com/knowledge-graph/)."]
        pub entity_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Language code for `description` in BCP-47 format."]
        pub language_code: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p3beta1Entity {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1EntityBuilder {
            GoogleCloudVideointelligenceV1p3beta1EntityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Explicit content annotation (based on per-frame visual signals only). If no explicit content has been detected in a frame, no annotations are present for that frame."]
    pub struct GoogleCloudVideointelligenceV1p3beta1ExplicitContentAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video frames where explicit content was detected."]
        pub frames: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1ExplicitContentFrame>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p3beta1ExplicitContentAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1ExplicitContentAnnotationBuilder {
            GoogleCloudVideointelligenceV1p3beta1ExplicitContentAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video frame level annotation results for explicit content."]
    pub struct GoogleCloudVideointelligenceV1p3beta1ExplicitContentFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pornographyLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Likelihood of the pornography content.."]
        pub pornography_likelihood: ::std::option::Option<
            GoogleCloudVideointelligenceV1p3beta1ExplicitContentFramePornographyLikelihoodEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p3beta1ExplicitContentFrame {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1ExplicitContentFrameBuilder {
            GoogleCloudVideointelligenceV1p3beta1ExplicitContentFrameBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Likelihood of the pornography content.."]
    pub enum GoogleCloudVideointelligenceV1p3beta1ExplicitContentFramePornographyLikelihoodEnum {
        #[serde(rename = "LIKELIHOOD_UNSPECIFIED")]
        #[doc = "Unspecified likelihood."]
        LikelihoodUnspecified,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "Very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "Unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "Possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "Likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "Very likely."]
        VeryLikely,
    }
    impl ::std::default::Default
        for GoogleCloudVideointelligenceV1p3beta1ExplicitContentFramePornographyLikelihoodEnum
    {
        fn default() -> Self {
            Self::LikelihoodUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deprecated. No effect."]
    pub struct GoogleCloudVideointelligenceV1p3beta1FaceAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video frames where a face was detected."]
        pub frames: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1FaceFrame>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video segments where a face was detected."]
        pub segments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1FaceSegment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Thumbnail of a representative face view (in JPEG format)."]
        pub thumbnail: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p3beta1FaceAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1FaceAnnotationBuilder {
            GoogleCloudVideointelligenceV1p3beta1FaceAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Face detection annotation."]
    pub struct GoogleCloudVideointelligenceV1p3beta1FaceDetectionAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The thumbnail of a person's face."]
        pub thumbnail: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tracks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The face tracks with attributes."]
        pub tracks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1Track>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p3beta1FaceDetectionAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1FaceDetectionAnnotationBuilder {
            GoogleCloudVideointelligenceV1p3beta1FaceDetectionAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deprecated. No effect."]
    pub struct GoogleCloudVideointelligenceV1p3beta1FaceFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedBoundingBoxes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Normalized Bounding boxes in a frame. There can be more than one boxes if the same face is detected in multiple locations within the current frame."]
        pub normalized_bounding_boxes: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1NormalizedBoundingBox>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p3beta1FaceFrame {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1FaceFrameBuilder {
            GoogleCloudVideointelligenceV1p3beta1FaceFrameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video segment level annotation results for face detection."]
    pub struct GoogleCloudVideointelligenceV1p3beta1FaceSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video segment where a face was detected."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1VideoSegment>,
        >,
    }
    impl GoogleCloudVideointelligenceV1p3beta1FaceSegment {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1FaceSegmentBuilder {
            GoogleCloudVideointelligenceV1p3beta1FaceSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Label annotation."]
    pub struct GoogleCloudVideointelligenceV1p3beta1LabelAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "categoryEntities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Common categories for the detected entity. For example, when the label is `Terrier`, the category is likely `dog`. And in some cases there might be more than one categories e.g., `Terrier` could also be a `pet`."]
        pub category_entities: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1Entity>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected entity."]
        pub entity:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1Entity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video frames where a label was detected."]
        pub frames: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1LabelFrame>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video segments where a label was detected."]
        pub segments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1LabelSegment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p3beta1LabelAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1LabelAnnotationBuilder {
            GoogleCloudVideointelligenceV1p3beta1LabelAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video frame level annotation results for label detection."]
    pub struct GoogleCloudVideointelligenceV1p3beta1LabelFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p3beta1LabelFrame {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1LabelFrameBuilder {
            GoogleCloudVideointelligenceV1p3beta1LabelFrameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video segment level annotation results for label detection."]
    pub struct GoogleCloudVideointelligenceV1p3beta1LabelSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video segment where a label was detected."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1VideoSegment>,
        >,
    }
    impl GoogleCloudVideointelligenceV1p3beta1LabelSegment {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1LabelSegmentBuilder {
            GoogleCloudVideointelligenceV1p3beta1LabelSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotation corresponding to one detected, tracked and recognized logo class."]
    pub struct GoogleCloudVideointelligenceV1p3beta1LogoRecognitionAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entity category information to specify the logo class that all the logo tracks within this LogoRecognitionAnnotation are recognized as."]
        pub entity:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1Entity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video segments where the recognized logo appears. There might be multiple instances of the same logo class appearing in one VideoSegment."]
        pub segments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1VideoSegment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tracks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All logo tracks where the recognized logo appears. Each track corresponds to one logo instance appearing in consecutive frames."]
        pub tracks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1Track>>,
        >,
    }
    impl GoogleCloudVideointelligenceV1p3beta1LogoRecognitionAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1LogoRecognitionAnnotationBuilder {
            GoogleCloudVideointelligenceV1p3beta1LogoRecognitionAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Normalized bounding box. The normalized vertex coordinates are relative to the original image. Range: [0, 1]."]
    pub struct GoogleCloudVideointelligenceV1p3beta1NormalizedBoundingBox {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bottom")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Bottom Y coordinate."]
        pub bottom: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "left")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Left X coordinate."]
        pub left: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "right")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Right X coordinate."]
        pub right: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "top")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Top Y coordinate."]
        pub top: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVideointelligenceV1p3beta1NormalizedBoundingBox {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1NormalizedBoundingBoxBuilder {
            GoogleCloudVideointelligenceV1p3beta1NormalizedBoundingBoxBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Normalized bounding polygon for text (that might not be aligned with axis). Contains list of the corner points in clockwise order starting from top-left corner. For example, for a rectangular bounding box: When the text is horizontal it might look like: 0----1 | | 3----2 When it's clockwise rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3). Note that values can be less than 0, or greater than 1 due to trignometric calculations for location of the box."]
    pub struct GoogleCloudVideointelligenceV1p3beta1NormalizedBoundingPoly {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vertices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Normalized vertices of the bounding polygon."]
        pub vertices: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1NormalizedVertex>,
            >,
        >,
    }
    impl GoogleCloudVideointelligenceV1p3beta1NormalizedBoundingPoly {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1NormalizedBoundingPolyBuilder {
            GoogleCloudVideointelligenceV1p3beta1NormalizedBoundingPolyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A vertex represents a 2D point in the image. NOTE: the normalized vertex coordinates are relative to the original image and range from 0 to 1."]
    pub struct GoogleCloudVideointelligenceV1p3beta1NormalizedVertex {
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
    impl GoogleCloudVideointelligenceV1p3beta1NormalizedVertex {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1NormalizedVertexBuilder {
            GoogleCloudVideointelligenceV1p3beta1NormalizedVertexBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotations corresponding to one tracked object."]
    pub struct GoogleCloudVideointelligenceV1p3beta1ObjectTrackingAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object category's labeling confidence of this track."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entity to specify the object category that this track is labeled as."]
        pub entity:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1Entity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information corresponding to all frames where this object track appears. Non-streaming batch mode: it may be one or multiple ObjectTrackingFrame messages in frames. Streaming mode: it can only be one ObjectTrackingFrame message in frames."]
        pub frames: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1ObjectTrackingFrame>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Non-streaming batch mode ONLY. Each object track corresponds to one video segment where it appears."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1VideoSegment>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trackId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Streaming mode ONLY. In streaming mode, we do not know the end time of a tracked object before it is completed. Hence, there is no VideoSegment info returned. Instead, we provide a unique identifiable integer track_id so that the customers can correlate the results of the ongoing ObjectTrackAnnotation of the same track_id over time."]
        pub track_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p3beta1ObjectTrackingAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1ObjectTrackingAnnotationBuilder {
            GoogleCloudVideointelligenceV1p3beta1ObjectTrackingAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video frame level annotations for object detection and tracking. This field stores per frame location, time offset, and confidence."]
    pub struct GoogleCloudVideointelligenceV1p3beta1ObjectTrackingFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedBoundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The normalized bounding box location of this object track for the frame."]
        pub normalized_bounding_box: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1NormalizedBoundingBox>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp of the frame in microseconds."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p3beta1ObjectTrackingFrame {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1ObjectTrackingFrameBuilder {
            GoogleCloudVideointelligenceV1p3beta1ObjectTrackingFrameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Person detection annotation per video."]
    pub struct GoogleCloudVideointelligenceV1p3beta1PersonDetectionAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tracks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The detected tracks of a person."]
        pub tracks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1Track>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p3beta1PersonDetectionAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1PersonDetectionAnnotationBuilder {
            GoogleCloudVideointelligenceV1p3beta1PersonDetectionAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The recognized celebrity with confidence score."]
    pub struct GoogleCloudVideointelligenceV1p3beta1RecognizedCelebrity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "celebrity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The recognized celebrity."]
        pub celebrity: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1Celebrity>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Recognition confidence. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVideointelligenceV1p3beta1RecognizedCelebrity {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1RecognizedCelebrityBuilder {
            GoogleCloudVideointelligenceV1p3beta1RecognizedCelebrityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Alternative hypotheses (a.k.a. n-best list)."]
    pub struct GoogleCloudVideointelligenceV1p3beta1SpeechRecognitionAlternative {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transcript")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Transcript text representing the words that the user spoke."]
        pub transcript: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "words")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A list of word-specific information for each recognized word. Note: When `enable_speaker_diarization` is set to true, you will see all the words from the beginning of the audio."]
        pub words: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1WordInfo>>,
        >,
    }
    impl GoogleCloudVideointelligenceV1p3beta1SpeechRecognitionAlternative {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1SpeechRecognitionAlternativeBuilder
        {
            GoogleCloudVideointelligenceV1p3beta1SpeechRecognitionAlternativeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A speech recognition result corresponding to a portion of the audio."]
    pub struct GoogleCloudVideointelligenceV1p3beta1SpeechTranscription {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alternatives")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "May contain one or more recognition hypotheses (up to the maximum specified in `max_alternatives`). These alternatives are ordered in terms of accuracy, with the top (first) alternative being the most probable, as ranked by the recognizer."]
        pub alternatives: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<
                    GoogleCloudVideointelligenceV1p3beta1SpeechRecognitionAlternative,
                >,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag of the language in this result. This language code was detected to have the most likelihood of being spoken in the audio."]
        pub language_code: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p3beta1SpeechTranscription {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1SpeechTranscriptionBuilder {
            GoogleCloudVideointelligenceV1p3beta1SpeechTranscriptionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "`StreamingAnnotateVideoResponse` is the only message returned to the client by `StreamingAnnotateVideo`. A series of zero or more `StreamingAnnotateVideoResponse` messages are streamed back to the client."]
    pub struct GoogleCloudVideointelligenceV1p3beta1StreamingAnnotateVideoResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Streaming annotation results."]
        pub annotation_results: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1StreamingVideoAnnotationResults>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationResultsUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage URI that stores annotation results of one streaming session in JSON format. It is the annotation_result_storage_directory from the request followed by '/cloud_project_number-session_id'."]
        pub annotation_results_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, returns a google.rpc.Status message that specifies the error for the operation."]
        pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
    }
    impl GoogleCloudVideointelligenceV1p3beta1StreamingAnnotateVideoResponse {
        pub fn builder(
        ) -> GoogleCloudVideointelligenceV1p3beta1StreamingAnnotateVideoResponseBuilder {
            GoogleCloudVideointelligenceV1p3beta1StreamingAnnotateVideoResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Streaming annotation results corresponding to a portion of the video that is currently being processed. Only ONE type of annotation will be specified in the response."]
    pub struct GoogleCloudVideointelligenceV1p3beta1StreamingVideoAnnotationResults {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "explicitAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Explicit content annotation results."]
        pub explicit_annotation: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1ExplicitContentAnnotation>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frameTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp of the processed frame in microseconds."]
        pub frame_timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label annotation results."]
        pub label_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1LabelAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object tracking results."]
        pub object_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1ObjectTrackingAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shotAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Shot annotation results. Each shot is represented as a video segment."]
        pub shot_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1VideoSegment>>,
        >,
    }
    impl GoogleCloudVideointelligenceV1p3beta1StreamingVideoAnnotationResults {
        pub fn builder(
        ) -> GoogleCloudVideointelligenceV1p3beta1StreamingVideoAnnotationResultsBuilder {
            GoogleCloudVideointelligenceV1p3beta1StreamingVideoAnnotationResultsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotations related to one detected OCR text snippet. This will contain the corresponding text, confidence value, and frame level information for each detection."]
    pub struct GoogleCloudVideointelligenceV1p3beta1TextAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All video segments where OCR detected text appears."]
        pub segments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1TextSegment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The detected text."]
        pub text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Feature version."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p3beta1TextAnnotation {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1TextAnnotationBuilder {
            GoogleCloudVideointelligenceV1p3beta1TextAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video frame level annotation results for text annotation (OCR). Contains information regarding timestamp and bounding box locations for the frames containing detected OCR text snippets."]
    pub struct GoogleCloudVideointelligenceV1p3beta1TextFrame {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rotatedBoundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Bounding polygon of the detected text for this frame."]
        pub rotated_bounding_box: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1NormalizedBoundingPoly>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp of this frame."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p3beta1TextFrame {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1TextFrameBuilder {
            GoogleCloudVideointelligenceV1p3beta1TextFrameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video segment level annotation results for text detection."]
    pub struct GoogleCloudVideointelligenceV1p3beta1TextSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence for the track of detected text. It is calculated as the highest over all frames where OCR detected text appears."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information related to the frames where OCR detected text appears."]
        pub frames: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1TextFrame>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video segment where a text snippet was detected."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1VideoSegment>,
        >,
    }
    impl GoogleCloudVideointelligenceV1p3beta1TextSegment {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1TextSegmentBuilder {
            GoogleCloudVideointelligenceV1p3beta1TextSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "For tracking related features. An object at time_offset with attributes, and located with normalized_bounding_box."]
    pub struct GoogleCloudVideointelligenceV1p3beta1TimestampedObject {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The attributes of the object in the bounding box."]
        pub attributes: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1DetectedAttribute>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "landmarks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The detected landmarks."]
        pub landmarks: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1DetectedLandmark>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedBoundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Normalized Bounding box in a frame, where the object is located."]
        pub normalized_bounding_box: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1NormalizedBoundingBox>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this object."]
        pub time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p3beta1TimestampedObject {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1TimestampedObjectBuilder {
            GoogleCloudVideointelligenceV1p3beta1TimestampedObjectBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A track of an object instance."]
    pub struct GoogleCloudVideointelligenceV1p3beta1Track {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Attributes in the track level."]
        pub attributes: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1DetectedAttribute>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The confidence score of the tracked object."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video segment of a track."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1VideoSegment>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestampedObjects")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object with timestamp and attributes per frame in the track."]
        pub timestamped_objects: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1TimestampedObject>,
            >,
        >,
    }
    impl GoogleCloudVideointelligenceV1p3beta1Track {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1TrackBuilder {
            GoogleCloudVideointelligenceV1p3beta1TrackBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotation progress for a single video."]
    pub struct GoogleCloudVideointelligenceV1p3beta1VideoAnnotationProgress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "feature")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies which feature is being tracked if the request contains more than one feature."]
        pub feature: ::std::option::Option<
            GoogleCloudVideointelligenceV1p3beta1VideoAnnotationProgressFeatureEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
        pub input_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressPercent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Approximate percentage processed thus far. Guaranteed to be 100 when fully processed."]
        pub progress_percent: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies which segment is being tracked if the request contains more than one segment."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1VideoSegment>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time when the request was received."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time of the most recent update."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p3beta1VideoAnnotationProgress {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1VideoAnnotationProgressBuilder {
            GoogleCloudVideointelligenceV1p3beta1VideoAnnotationProgressBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies which feature is being tracked if the request contains more than one feature."]
    pub enum GoogleCloudVideointelligenceV1p3beta1VideoAnnotationProgressFeatureEnum {
        #[serde(rename = "FEATURE_UNSPECIFIED")]
        #[doc = "Unspecified."]
        FeatureUnspecified,
        #[serde(rename = "LABEL_DETECTION")]
        #[doc = "Label detection. Detect objects, such as dog or flower."]
        LabelDetection,
        #[serde(rename = "SHOT_CHANGE_DETECTION")]
        #[doc = "Shot change detection."]
        ShotChangeDetection,
        #[serde(rename = "EXPLICIT_CONTENT_DETECTION")]
        #[doc = "Explicit content detection."]
        ExplicitContentDetection,
        #[serde(rename = "FACE_DETECTION")]
        #[doc = "Human face detection."]
        FaceDetection,
        #[serde(rename = "SPEECH_TRANSCRIPTION")]
        #[doc = "Speech transcription."]
        SpeechTranscription,
        #[serde(rename = "TEXT_DETECTION")]
        #[doc = "OCR text detection and tracking."]
        TextDetection,
        #[serde(rename = "OBJECT_TRACKING")]
        #[doc = "Object detection and tracking."]
        ObjectTracking,
        #[serde(rename = "LOGO_RECOGNITION")]
        #[doc = "Logo detection, tracking, and recognition."]
        LogoRecognition,
        #[serde(rename = "CELEBRITY_RECOGNITION")]
        #[doc = "Celebrity recognition."]
        CelebrityRecognition,
        #[serde(rename = "PERSON_DETECTION")]
        #[doc = "Person detection."]
        PersonDetection,
    }
    impl ::std::default::Default
        for GoogleCloudVideointelligenceV1p3beta1VideoAnnotationProgressFeatureEnum
    {
        fn default() -> Self {
            Self::FeatureUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Annotation results for a single video."]
    pub struct GoogleCloudVideointelligenceV1p3beta1VideoAnnotationResults {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "celebrityRecognitionAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Celebrity recognition annotations."]
        pub celebrity_recognition_annotations: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1CelebrityRecognitionAnnotation>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, indicates an error. Note that for a single `AnnotateVideoRequest` some videos may succeed and some may fail."]
        pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "explicitAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Explicit content annotation."]
        pub explicit_annotation: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1ExplicitContentAnnotation>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "faceAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Please use `face_detection_annotations` instead."]
        pub face_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1FaceAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "faceDetectionAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Face detection annotations."]
        pub face_detection_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1FaceDetectionAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frameLabelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label annotations on frame level. There is exactly one element for each unique label."]
        pub frame_label_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1LabelAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
        pub input_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logoRecognitionAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Annotations for list of logos detected, tracked and recognized in video."]
        pub logo_recognition_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1LogoRecognitionAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Annotations for list of objects detected and tracked in video."]
        pub object_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1ObjectTrackingAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "personDetectionAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Person detection annotations."]
        pub person_detection_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1PersonDetectionAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video segment on which the annotation is run."]
        pub segment: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1VideoSegment>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segmentLabelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Topical label annotations on video level or user-specified segment level. There is exactly one element for each unique label."]
        pub segment_label_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1LabelAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segmentPresenceLabelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Presence label annotations on video level or user-specified segment level. There is exactly one element for each unique label. Compared to the existing topical `segment_label_annotations`, this field presents more fine-grained, segment-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to \"builtin/latest\" in the request."]
        pub segment_presence_label_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1LabelAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shotAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Shot annotations. Each shot is represented as a video segment."]
        pub shot_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1VideoSegment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shotLabelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Topical label annotations on shot level. There is exactly one element for each unique label."]
        pub shot_label_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1LabelAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shotPresenceLabelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Presence label annotations on shot level. There is exactly one element for each unique label. Compared to the existing topical `shot_label_annotations`, this field presents more fine-grained, shot-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to \"builtin/latest\" in the request."]
        pub shot_presence_label_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1LabelAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "speechTranscriptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Speech transcription."]
        pub speech_transcriptions: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1SpeechTranscription>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OCR text detection and tracking. Annotations for list of detected text snippets. Each will have list of frame information associated with it."]
        pub text_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1TextAnnotation>>,
        >,
    }
    impl GoogleCloudVideointelligenceV1p3beta1VideoAnnotationResults {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1VideoAnnotationResultsBuilder {
            GoogleCloudVideointelligenceV1p3beta1VideoAnnotationResultsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video segment."]
    pub struct GoogleCloudVideointelligenceV1p3beta1VideoSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTimeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the end of the segment (inclusive)."]
        pub end_time_offset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTimeOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the start of the segment (inclusive)."]
        pub start_time_offset: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVideointelligenceV1p3beta1VideoSegment {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1VideoSegmentBuilder {
            GoogleCloudVideointelligenceV1p3beta1VideoSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Word-specific information for recognized words. Word information is only included in the response when certain request parameters are set, such as `enable_word_time_offsets`."]
    pub struct GoogleCloudVideointelligenceV1p3beta1WordInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time offset relative to the beginning of the audio, and corresponding to the end of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "speakerTag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A distinct integer value is assigned for every speaker within the audio. This field specifies which one of those speakers was detected to have spoken this word. Value ranges from 1 up to diarization_speaker_count, and is only set if speaker diarization is enabled."]
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
    impl GoogleCloudVideointelligenceV1p3beta1WordInfo {
        pub fn builder() -> GoogleCloudVideointelligenceV1p3beta1WordInfoBuilder {
            GoogleCloudVideointelligenceV1p3beta1WordInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for Operations.CancelOperation."]
    pub struct GoogleLongrunningCancelOperationRequest {}
    impl GoogleLongrunningCancelOperationRequest {
        pub fn builder() -> GoogleLongrunningCancelOperationRequestBuilder {
            GoogleLongrunningCancelOperationRequestBuilder::default()
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
}
