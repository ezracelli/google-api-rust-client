#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video annotation progress. Included in the `metadata` field of the `Operation` returned by the `GetOperation` call of the `google::longrunning::Operations` service."]
pub struct GoogleCloudVideointelligenceV1AnnotateVideoProgress {
    #[serde(rename = "annotationProgress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Progress metadata for all videos specified in `AnnotateVideoRequest`."]
    pub annotation_progress: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1VideoAnnotationProgress>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video annotation response. Included in the `response` field of the `Operation` returned by the `GetOperation` call of the `google::longrunning::Operations` service."]
pub struct GoogleCloudVideointelligenceV1AnnotateVideoResponse {
    #[serde(rename = "annotationResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Annotation results for all videos specified in `AnnotateVideoRequest`."]
    pub annotation_results: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1VideoAnnotationResults>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic detected attribute represented by name in string format."]
pub struct GoogleCloudVideointelligenceV1DetectedAttribute {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detected attribute confidence. Range [0, 1]."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the attribute, for example, glasses, dark_glasses, mouth_open. A full list of supported type names will be provided in the document."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Text value of the detection result. For example, the value for \"HairColor\" can be \"black\", \"blonde\", etc."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic detected landmark represented by name in string format and a 2D location."]
pub struct GoogleCloudVideointelligenceV1DetectedLandmark {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The confidence score of the detected landmark. Range [0, 1]."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of this landmark, for example, left_hand, right_shoulder."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "point")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The 2D point of the detected landmark using the normalized image coordindate system. The normalized coordinates have the range from 0 to 1."]
    pub point:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1NormalizedVertex>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Detected entity from video analysis."]
pub struct GoogleCloudVideointelligenceV1Entity {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Textual description, e.g., `Fixed-gear bicycle`."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Opaque entity ID. Some IDs may be available in [Google Knowledge Graph Search API](https://developers.google.com/knowledge-graph/)."]
    pub entity_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Language code for `description` in BCP-47 format."]
    pub language_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Explicit content annotation (based on per-frame visual signals only). If no explicit content has been detected in a frame, no annotations are present for that frame."]
pub struct GoogleCloudVideointelligenceV1ExplicitContentAnnotation {
    #[serde(rename = "frames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video frames where explicit content was detected."]
    pub frames: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1ExplicitContentFrame>>,
    >,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video frame level annotation results for explicit content."]
pub struct GoogleCloudVideointelligenceV1ExplicitContentFrame {
    #[serde(rename = "pornographyLikelihood")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Likelihood of the pornography content.."]
    pub pornography_likelihood: ::std::option::Option<
        GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihoodEnum,
    >,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
    pub time_offset: ::std::option::Option<::std::string::String>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Deprecated. No effect."]
pub struct GoogleCloudVideointelligenceV1FaceAnnotation {
    #[serde(rename = "frames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video frames where a face was detected."]
    pub frames: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1FaceFrame>>,
    >,
    #[serde(rename = "segments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video segments where a face was detected."]
    pub segments: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1FaceSegment>>,
    >,
    #[serde(rename = "thumbnail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Thumbnail of a representative face view (in JPEG format)."]
    pub thumbnail: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Face detection annotation."]
pub struct GoogleCloudVideointelligenceV1FaceDetectionAnnotation {
    #[serde(rename = "thumbnail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The thumbnail of a person's face."]
    pub thumbnail: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tracks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The face tracks with attributes."]
    pub tracks: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1Track>>,
    >,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Deprecated. No effect."]
pub struct GoogleCloudVideointelligenceV1FaceFrame {
    #[serde(rename = "normalizedBoundingBoxes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Normalized Bounding boxes in a frame. There can be more than one boxes if the same face is detected in multiple locations within the current frame."]
    pub normalized_bounding_boxes: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1NormalizedBoundingBox>>,
    >,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
    pub time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video segment level annotation results for face detection."]
pub struct GoogleCloudVideointelligenceV1FaceSegment {
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video segment where a face was detected."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1VideoSegment>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Label annotation."]
pub struct GoogleCloudVideointelligenceV1LabelAnnotation {
    #[serde(rename = "categoryEntities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Common categories for the detected entity. For example, when the label is `Terrier`, the category is likely `dog`. And in some cases there might be more than one categories e.g., `Terrier` could also be a `pet`."]
    pub category_entities: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1Entity>>,
    >,
    #[serde(rename = "entity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detected entity."]
    pub entity: ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1Entity>>,
    #[serde(rename = "frames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video frames where a label was detected."]
    pub frames: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1LabelFrame>>,
    >,
    #[serde(rename = "segments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video segments where a label was detected."]
    pub segments: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1LabelSegment>>,
    >,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video frame level annotation results for label detection."]
pub struct GoogleCloudVideointelligenceV1LabelFrame {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
    pub time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video segment level annotation results for label detection."]
pub struct GoogleCloudVideointelligenceV1LabelSegment {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video segment where a label was detected."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1VideoSegment>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotation corresponding to one detected, tracked and recognized logo class."]
pub struct GoogleCloudVideointelligenceV1LogoRecognitionAnnotation {
    #[serde(rename = "entity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Entity category information to specify the logo class that all the logo tracks within this LogoRecognitionAnnotation are recognized as."]
    pub entity: ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1Entity>>,
    #[serde(rename = "segments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video segments where the recognized logo appears. There might be multiple instances of the same logo class appearing in one VideoSegment."]
    pub segments: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1VideoSegment>>,
    >,
    #[serde(rename = "tracks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All logo tracks where the recognized logo appears. Each track corresponds to one logo instance appearing in consecutive frames."]
    pub tracks: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1Track>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Normalized bounding box. The normalized vertex coordinates are relative to the original image. Range: [0, 1]."]
pub struct GoogleCloudVideointelligenceV1NormalizedBoundingBox {
    #[serde(rename = "bottom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Bottom Y coordinate."]
    pub bottom: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "left")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Left X coordinate."]
    pub left: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "right")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Right X coordinate."]
    pub right: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "top")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Top Y coordinate."]
    pub top: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Normalized bounding polygon for text (that might not be aligned with axis). Contains list of the corner points in clockwise order starting from top-left corner. For example, for a rectangular bounding box: When the text is horizontal it might look like: 0----1 | | 3----2 When it's clockwise rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3). Note that values can be less than 0, or greater than 1 due to trignometric calculations for location of the box."]
pub struct GoogleCloudVideointelligenceV1NormalizedBoundingPoly {
    #[serde(rename = "vertices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Normalized vertices of the bounding polygon."]
    pub vertices: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1NormalizedVertex>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A vertex represents a 2D point in the image. NOTE: the normalized vertex coordinates are relative to the original image and range from 0 to 1."]
pub struct GoogleCloudVideointelligenceV1NormalizedVertex {
    #[serde(rename = "x")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "X coordinate."]
    pub x: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "y")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Y coordinate."]
    pub y: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotations corresponding to one tracked object."]
pub struct GoogleCloudVideointelligenceV1ObjectTrackingAnnotation {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Object category's labeling confidence of this track."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "entity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Entity to specify the object category that this track is labeled as."]
    pub entity: ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1Entity>>,
    #[serde(rename = "frames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information corresponding to all frames where this object track appears. Non-streaming batch mode: it may be one or multiple ObjectTrackingFrame messages in frames. Streaming mode: it can only be one ObjectTrackingFrame message in frames."]
    pub frames: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1ObjectTrackingFrame>>,
    >,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Non-streaming batch mode ONLY. Each object track corresponds to one video segment where it appears."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1VideoSegment>>,
    #[serde(rename = "trackId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Streaming mode ONLY. In streaming mode, we do not know the end time of a tracked object before it is completed. Hence, there is no VideoSegment info returned. Instead, we provide a unique identifiable integer track_id so that the customers can correlate the results of the ongoing ObjectTrackAnnotation of the same track_id over time."]
    pub track_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video frame level annotations for object detection and tracking. This field stores per frame location, time offset, and confidence."]
pub struct GoogleCloudVideointelligenceV1ObjectTrackingFrame {
    #[serde(rename = "normalizedBoundingBox")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The normalized bounding box location of this object track for the frame."]
    pub normalized_bounding_box: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1NormalizedBoundingBox>,
    >,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The timestamp of the frame in microseconds."]
    pub time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Person detection annotation per video."]
pub struct GoogleCloudVideointelligenceV1PersonDetectionAnnotation {
    #[serde(rename = "tracks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The detected tracks of a person."]
    pub tracks: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1Track>>,
    >,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Alternative hypotheses (a.k.a. n-best list)."]
pub struct GoogleCloudVideointelligenceV1SpeechRecognitionAlternative {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "transcript")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Transcript text representing the words that the user spoke."]
    pub transcript: ::std::option::Option<::std::string::String>,
    #[serde(rename = "words")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A list of word-specific information for each recognized word. Note: When `enable_speaker_diarization` is set to true, you will see all the words from the beginning of the audio."]
    pub words: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1WordInfo>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A speech recognition result corresponding to a portion of the audio."]
pub struct GoogleCloudVideointelligenceV1SpeechTranscription {
    #[serde(rename = "alternatives")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "May contain one or more recognition hypotheses (up to the maximum specified in `max_alternatives`). These alternatives are ordered in terms of accuracy, with the top (first) alternative being the most probable, as ranked by the recognizer."]
    pub alternatives: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1SpeechRecognitionAlternative>,
        >,
    >,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag of the language in this result. This language code was detected to have the most likelihood of being spoken in the audio."]
    pub language_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotations related to one detected OCR text snippet. This will contain the corresponding text, confidence value, and frame level information for each detection."]
pub struct GoogleCloudVideointelligenceV1TextAnnotation {
    #[serde(rename = "segments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video segments where OCR detected text appears."]
    pub segments: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1TextSegment>>,
    >,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The detected text."]
    pub text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video frame level annotation results for text annotation (OCR). Contains information regarding timestamp and bounding box locations for the frames containing detected OCR text snippets."]
pub struct GoogleCloudVideointelligenceV1TextFrame {
    #[serde(rename = "rotatedBoundingBox")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Bounding polygon of the detected text for this frame."]
    pub rotated_bounding_box: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1NormalizedBoundingPoly>,
    >,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp of this frame."]
    pub time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video segment level annotation results for text detection."]
pub struct GoogleCloudVideointelligenceV1TextSegment {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Confidence for the track of detected text. It is calculated as the highest over all frames where OCR detected text appears."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "frames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information related to the frames where OCR detected text appears."]
    pub frames: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1TextFrame>>,
    >,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video segment where a text snippet was detected."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1VideoSegment>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "For tracking related features. An object at time_offset with attributes, and located with normalized_bounding_box."]
pub struct GoogleCloudVideointelligenceV1TimestampedObject {
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The attributes of the object in the bounding box."]
    pub attributes: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1DetectedAttribute>>,
    >,
    #[serde(rename = "landmarks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The detected landmarks."]
    pub landmarks: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1DetectedLandmark>>,
    >,
    #[serde(rename = "normalizedBoundingBox")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Normalized Bounding box in a frame, where the object is located."]
    pub normalized_bounding_box: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1NormalizedBoundingBox>,
    >,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this object."]
    pub time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A track of an object instance."]
pub struct GoogleCloudVideointelligenceV1Track {
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Attributes in the track level."]
    pub attributes: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1DetectedAttribute>>,
    >,
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The confidence score of the tracked object."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video segment of a track."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1VideoSegment>>,
    #[serde(rename = "timestampedObjects")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The object with timestamp and attributes per frame in the track."]
    pub timestamped_objects: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1TimestampedObject>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotation progress for a single video."]
pub struct GoogleCloudVideointelligenceV1VideoAnnotationProgress {
    #[serde(rename = "feature")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies which feature is being tracked if the request contains more than one feature."]
    pub feature:
        ::std::option::Option<GoogleCloudVideointelligenceV1VideoAnnotationProgressFeatureEnum>,
    #[serde(rename = "inputUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
    pub input_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progressPercent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Approximate percentage processed thus far. Guaranteed to be 100 when fully processed."]
    pub progress_percent: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies which segment is being tracked if the request contains more than one segment."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1VideoSegment>>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time when the request was received."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time of the most recent update."]
    pub update_time: ::std::option::Option<::std::string::String>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotation results for a single video."]
pub struct GoogleCloudVideointelligenceV1VideoAnnotationResults {
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, indicates an error. Note that for a single `AnnotateVideoRequest` some videos may succeed and some may fail."]
    pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
    #[serde(rename = "explicitAnnotation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Explicit content annotation."]
    pub explicit_annotation: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1ExplicitContentAnnotation>,
    >,
    #[serde(rename = "faceAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Please use `face_detection_annotations` instead."]
    pub face_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1FaceAnnotation>>,
    >,
    #[serde(rename = "faceDetectionAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Face detection annotations."]
    pub face_detection_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1FaceDetectionAnnotation>>,
    >,
    #[serde(rename = "frameLabelAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Label annotations on frame level. There is exactly one element for each unique label."]
    pub frame_label_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1LabelAnnotation>>,
    >,
    #[serde(rename = "inputUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
    pub input_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "logoRecognitionAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Annotations for list of logos detected, tracked and recognized in video."]
    pub logo_recognition_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1LogoRecognitionAnnotation>>,
    >,
    #[serde(rename = "objectAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Annotations for list of objects detected and tracked in video."]
    pub object_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1ObjectTrackingAnnotation>>,
    >,
    #[serde(rename = "personDetectionAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Person detection annotations."]
    pub person_detection_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1PersonDetectionAnnotation>>,
    >,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video segment on which the annotation is run."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1VideoSegment>>,
    #[serde(rename = "segmentLabelAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Topical label annotations on video level or user-specified segment level. There is exactly one element for each unique label."]
    pub segment_label_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1LabelAnnotation>>,
    >,
    #[serde(rename = "segmentPresenceLabelAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Presence label annotations on video level or user-specified segment level. There is exactly one element for each unique label. Compared to the existing topical `segment_label_annotations`, this field presents more fine-grained, segment-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to \"builtin/latest\" in the request."]
    pub segment_presence_label_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1LabelAnnotation>>,
    >,
    #[serde(rename = "shotAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shot annotations. Each shot is represented as a video segment."]
    pub shot_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1VideoSegment>>,
    >,
    #[serde(rename = "shotLabelAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Topical label annotations on shot level. There is exactly one element for each unique label."]
    pub shot_label_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1LabelAnnotation>>,
    >,
    #[serde(rename = "shotPresenceLabelAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Presence label annotations on shot level. There is exactly one element for each unique label. Compared to the existing topical `shot_label_annotations`, this field presents more fine-grained, shot-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to \"builtin/latest\" in the request."]
    pub shot_presence_label_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1LabelAnnotation>>,
    >,
    #[serde(rename = "speechTranscriptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Speech transcription."]
    pub speech_transcriptions: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1SpeechTranscription>>,
    >,
    #[serde(rename = "textAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OCR text detection and tracking. Annotations for list of detected text snippets. Each will have list of frame information associated with it."]
    pub text_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1TextAnnotation>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video segment."]
pub struct GoogleCloudVideointelligenceV1VideoSegment {
    #[serde(rename = "endTimeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the end of the segment (inclusive)."]
    pub end_time_offset: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTimeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the start of the segment (inclusive)."]
    pub start_time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Word-specific information for recognized words. Word information is only included in the response when certain request parameters are set, such as `enable_word_time_offsets`."]
pub struct GoogleCloudVideointelligenceV1WordInfo {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time offset relative to the beginning of the audio, and corresponding to the end of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "speakerTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A distinct integer value is assigned for every speaker within the audio. This field specifies which one of those speakers was detected to have spoken this word. Value ranges from 1 up to diarization_speaker_count, and is only set if speaker diarization is enabled."]
    pub speaker_tag: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time offset relative to the beginning of the audio, and corresponding to the start of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "word")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The word corresponding to this set of information."]
    pub word: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video annotation progress. Included in the `metadata` field of the `Operation` returned by the `GetOperation` call of the `google::longrunning::Operations` service."]
pub struct GoogleCloudVideointelligenceV1beta2AnnotateVideoProgress {
    #[serde(rename = "annotationProgress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Progress metadata for all videos specified in `AnnotateVideoRequest`."]
    pub annotation_progress: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2VideoAnnotationProgress>,
        >,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video annotation request."]
pub struct GoogleCloudVideointelligenceV1beta2AnnotateVideoRequest {
    #[serde(rename = "features")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Requested video annotation features."]
    pub features: ::std::option::Option<
        ::std::vec::Vec<GoogleCloudVideointelligenceV1beta2AnnotateVideoRequestFeaturesEnum>,
    >,
    #[serde(rename = "inputContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The video data bytes. If unset, the input video(s) should be specified via the `input_uri`. If set, `input_uri` must be unset."]
    pub input_content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inputUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Input video location. Currently, only [Cloud Storage](https://cloud.google.com/storage/) URIs are supported. URIs must be specified in the following format: `gs://bucket-id/object-id` (other URI formats return google.rpc.Code.INVALID_ARGUMENT). For more information, see [Request URIs](https://cloud.google.com/storage/docs/request-endpoints). To identify multiple videos, a video URI may include wildcards in the `object-id`. Supported wildcards: '*' to match 0 or more characters; '?' to match 1 character. If unset, the input video should be embedded in the request as `input_content`. If set, `input_content` must be unset."]
    pub input_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Cloud region where annotation should take place. Supported cloud regions are: `us-east1`, `us-west1`, `europe-west1`, `asia-east1`. If no region is specified, the region will be determined based on video file location."]
    pub location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "outputUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Location where the output (in JSON format) should be stored. Currently, only [Cloud Storage](https://cloud.google.com/storage/) URIs are supported. These must be specified in the following format: `gs://bucket-id/object-id` (other URI formats return google.rpc.Code.INVALID_ARGUMENT). For more information, see [Request URIs](https://cloud.google.com/storage/docs/request-endpoints)."]
    pub output_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "videoContext")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional video context and/or feature-specific parameters."]
    pub video_context:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2VideoContext>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum GoogleCloudVideointelligenceV1beta2AnnotateVideoRequestFeaturesEnum {
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video annotation response. Included in the `response` field of the `Operation` returned by the `GetOperation` call of the `google::longrunning::Operations` service."]
pub struct GoogleCloudVideointelligenceV1beta2AnnotateVideoResponse {
    #[serde(rename = "annotationResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Annotation results for all videos specified in `AnnotateVideoRequest`."]
    pub annotation_results: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2VideoAnnotationResults>,
        >,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic detected attribute represented by name in string format."]
pub struct GoogleCloudVideointelligenceV1beta2DetectedAttribute {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detected attribute confidence. Range [0, 1]."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the attribute, for example, glasses, dark_glasses, mouth_open. A full list of supported type names will be provided in the document."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Text value of the detection result. For example, the value for \"HairColor\" can be \"black\", \"blonde\", etc."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic detected landmark represented by name in string format and a 2D location."]
pub struct GoogleCloudVideointelligenceV1beta2DetectedLandmark {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The confidence score of the detected landmark. Range [0, 1]."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of this landmark, for example, left_hand, right_shoulder."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "point")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The 2D point of the detected landmark using the normalized image coordindate system. The normalized coordinates have the range from 0 to 1."]
    pub point: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2NormalizedVertex>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Detected entity from video analysis."]
pub struct GoogleCloudVideointelligenceV1beta2Entity {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Textual description, e.g., `Fixed-gear bicycle`."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Opaque entity ID. Some IDs may be available in [Google Knowledge Graph Search API](https://developers.google.com/knowledge-graph/)."]
    pub entity_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Language code for `description` in BCP-47 format."]
    pub language_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Explicit content annotation (based on per-frame visual signals only). If no explicit content has been detected in a frame, no annotations are present for that frame."]
pub struct GoogleCloudVideointelligenceV1beta2ExplicitContentAnnotation {
    #[serde(rename = "frames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video frames where explicit content was detected."]
    pub frames: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2ExplicitContentFrame>>,
    >,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Config for EXPLICIT_CONTENT_DETECTION."]
pub struct GoogleCloudVideointelligenceV1beta2ExplicitContentDetectionConfig {
    #[serde(rename = "model")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Model to use for explicit content detection. Supported values: \"builtin/stable\" (the default if unset) and \"builtin/latest\"."]
    pub model: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video frame level annotation results for explicit content."]
pub struct GoogleCloudVideointelligenceV1beta2ExplicitContentFrame {
    #[serde(rename = "pornographyLikelihood")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Likelihood of the pornography content.."]
    pub pornography_likelihood: ::std::option::Option<
        GoogleCloudVideointelligenceV1beta2ExplicitContentFramePornographyLikelihoodEnum,
    >,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
    pub time_offset: ::std::option::Option<::std::string::String>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Deprecated. No effect."]
pub struct GoogleCloudVideointelligenceV1beta2FaceAnnotation {
    #[serde(rename = "frames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video frames where a face was detected."]
    pub frames: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2FaceFrame>>,
    >,
    #[serde(rename = "segments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video segments where a face was detected."]
    pub segments: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2FaceSegment>>,
    >,
    #[serde(rename = "thumbnail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Thumbnail of a representative face view (in JPEG format)."]
    pub thumbnail: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Face detection annotation."]
pub struct GoogleCloudVideointelligenceV1beta2FaceDetectionAnnotation {
    #[serde(rename = "thumbnail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The thumbnail of a person's face."]
    pub thumbnail: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tracks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The face tracks with attributes."]
    pub tracks: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2Track>>,
    >,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Config for FACE_DETECTION."]
pub struct GoogleCloudVideointelligenceV1beta2FaceDetectionConfig {
    #[serde(rename = "includeAttributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to enable face attributes detection, such as glasses, dark_glasses, mouth_open etc. Ignored if 'include_bounding_boxes' is set to false."]
    pub include_attributes: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "includeBoundingBoxes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether bounding boxes are included in the face annotation output."]
    pub include_bounding_boxes: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "model")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Model to use for face detection. Supported values: \"builtin/stable\" (the default if unset) and \"builtin/latest\"."]
    pub model: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Deprecated. No effect."]
pub struct GoogleCloudVideointelligenceV1beta2FaceFrame {
    #[serde(rename = "normalizedBoundingBoxes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Normalized Bounding boxes in a frame. There can be more than one boxes if the same face is detected in multiple locations within the current frame."]
    pub normalized_bounding_boxes: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2NormalizedBoundingBox>,
        >,
    >,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
    pub time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video segment level annotation results for face detection."]
pub struct GoogleCloudVideointelligenceV1beta2FaceSegment {
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video segment where a face was detected."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2VideoSegment>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Label annotation."]
pub struct GoogleCloudVideointelligenceV1beta2LabelAnnotation {
    #[serde(rename = "categoryEntities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Common categories for the detected entity. For example, when the label is `Terrier`, the category is likely `dog`. And in some cases there might be more than one categories e.g., `Terrier` could also be a `pet`."]
    pub category_entities: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2Entity>>,
    >,
    #[serde(rename = "entity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detected entity."]
    pub entity: ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2Entity>>,
    #[serde(rename = "frames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video frames where a label was detected."]
    pub frames: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2LabelFrame>>,
    >,
    #[serde(rename = "segments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video segments where a label was detected."]
    pub segments: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2LabelSegment>>,
    >,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Config for LABEL_DETECTION."]
pub struct GoogleCloudVideointelligenceV1beta2LabelDetectionConfig {
    #[serde(rename = "frameConfidenceThreshold")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The confidence threshold we perform filtering on the labels from frame-level detection. If not set, it is set to 0.4 by default. The valid range for this threshold is [0.1, 0.9]. Any value set outside of this range will be clipped. Note: For best results, follow the default threshold. We will update the default threshold everytime when we release a new model."]
    pub frame_confidence_threshold: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "labelDetectionMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "What labels should be detected with LABEL_DETECTION, in addition to video-level labels or segment-level labels. If unspecified, defaults to `SHOT_MODE`."]
    pub label_detection_mode: ::std::option::Option<
        GoogleCloudVideointelligenceV1beta2LabelDetectionConfigLabelDetectionModeEnum,
    >,
    #[serde(rename = "model")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Model to use for label detection. Supported values: \"builtin/stable\" (the default if unset) and \"builtin/latest\"."]
    pub model: ::std::option::Option<::std::string::String>,
    #[serde(rename = "stationaryCamera")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the video has been shot from a stationary (i.e., non-moving) camera. When set to true, might improve detection accuracy for moving objects. Should be used with `SHOT_AND_FRAME_MODE` enabled."]
    pub stationary_camera: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "videoConfidenceThreshold")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The confidence threshold we perform filtering on the labels from video-level and shot-level detections. If not set, it's set to 0.3 by default. The valid range for this threshold is [0.1, 0.9]. Any value set outside of this range will be clipped. Note: For best results, follow the default threshold. We will update the default threshold everytime when we release a new model."]
    pub video_confidence_threshold: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "What labels should be detected with LABEL_DETECTION, in addition to video-level labels or segment-level labels. If unspecified, defaults to `SHOT_MODE`."]
pub enum GoogleCloudVideointelligenceV1beta2LabelDetectionConfigLabelDetectionModeEnum {
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video frame level annotation results for label detection."]
pub struct GoogleCloudVideointelligenceV1beta2LabelFrame {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
    pub time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video segment level annotation results for label detection."]
pub struct GoogleCloudVideointelligenceV1beta2LabelSegment {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video segment where a label was detected."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2VideoSegment>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotation corresponding to one detected, tracked and recognized logo class."]
pub struct GoogleCloudVideointelligenceV1beta2LogoRecognitionAnnotation {
    #[serde(rename = "entity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Entity category information to specify the logo class that all the logo tracks within this LogoRecognitionAnnotation are recognized as."]
    pub entity: ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2Entity>>,
    #[serde(rename = "segments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video segments where the recognized logo appears. There might be multiple instances of the same logo class appearing in one VideoSegment."]
    pub segments: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2VideoSegment>>,
    >,
    #[serde(rename = "tracks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All logo tracks where the recognized logo appears. Each track corresponds to one logo instance appearing in consecutive frames."]
    pub tracks: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2Track>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Normalized bounding box. The normalized vertex coordinates are relative to the original image. Range: [0, 1]."]
pub struct GoogleCloudVideointelligenceV1beta2NormalizedBoundingBox {
    #[serde(rename = "bottom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Bottom Y coordinate."]
    pub bottom: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "left")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Left X coordinate."]
    pub left: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "right")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Right X coordinate."]
    pub right: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "top")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Top Y coordinate."]
    pub top: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Normalized bounding polygon for text (that might not be aligned with axis). Contains list of the corner points in clockwise order starting from top-left corner. For example, for a rectangular bounding box: When the text is horizontal it might look like: 0----1 | | 3----2 When it's clockwise rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3). Note that values can be less than 0, or greater than 1 due to trignometric calculations for location of the box."]
pub struct GoogleCloudVideointelligenceV1beta2NormalizedBoundingPoly {
    #[serde(rename = "vertices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Normalized vertices of the bounding polygon."]
    pub vertices: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2NormalizedVertex>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A vertex represents a 2D point in the image. NOTE: the normalized vertex coordinates are relative to the original image and range from 0 to 1."]
pub struct GoogleCloudVideointelligenceV1beta2NormalizedVertex {
    #[serde(rename = "x")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "X coordinate."]
    pub x: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "y")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Y coordinate."]
    pub y: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotations corresponding to one tracked object."]
pub struct GoogleCloudVideointelligenceV1beta2ObjectTrackingAnnotation {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Object category's labeling confidence of this track."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "entity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Entity to specify the object category that this track is labeled as."]
    pub entity: ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2Entity>>,
    #[serde(rename = "frames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information corresponding to all frames where this object track appears. Non-streaming batch mode: it may be one or multiple ObjectTrackingFrame messages in frames. Streaming mode: it can only be one ObjectTrackingFrame message in frames."]
    pub frames: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2ObjectTrackingFrame>>,
    >,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Non-streaming batch mode ONLY. Each object track corresponds to one video segment where it appears."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2VideoSegment>>,
    #[serde(rename = "trackId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Streaming mode ONLY. In streaming mode, we do not know the end time of a tracked object before it is completed. Hence, there is no VideoSegment info returned. Instead, we provide a unique identifiable integer track_id so that the customers can correlate the results of the ongoing ObjectTrackAnnotation of the same track_id over time."]
    pub track_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Config for OBJECT_TRACKING."]
pub struct GoogleCloudVideointelligenceV1beta2ObjectTrackingConfig {
    #[serde(rename = "model")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Model to use for object tracking. Supported values: \"builtin/stable\" (the default if unset) and \"builtin/latest\"."]
    pub model: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video frame level annotations for object detection and tracking. This field stores per frame location, time offset, and confidence."]
pub struct GoogleCloudVideointelligenceV1beta2ObjectTrackingFrame {
    #[serde(rename = "normalizedBoundingBox")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The normalized bounding box location of this object track for the frame."]
    pub normalized_bounding_box: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2NormalizedBoundingBox>,
    >,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The timestamp of the frame in microseconds."]
    pub time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Person detection annotation per video."]
pub struct GoogleCloudVideointelligenceV1beta2PersonDetectionAnnotation {
    #[serde(rename = "tracks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The detected tracks of a person."]
    pub tracks: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2Track>>,
    >,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Config for PERSON_DETECTION."]
pub struct GoogleCloudVideointelligenceV1beta2PersonDetectionConfig {
    #[serde(rename = "includeAttributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to enable person attributes detection, such as cloth color (black, blue, etc), type (coat, dress, etc), pattern (plain, floral, etc), hair, etc. Ignored if 'include_bounding_boxes' is set to false."]
    pub include_attributes: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "includeBoundingBoxes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether bounding boxes are included in the person detection annotation output."]
    pub include_bounding_boxes: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "includePoseLandmarks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether to enable pose landmarks detection. Ignored if 'include_bounding_boxes' is set to false."]
    pub include_pose_landmarks: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Config for SHOT_CHANGE_DETECTION."]
pub struct GoogleCloudVideointelligenceV1beta2ShotChangeDetectionConfig {
    #[serde(rename = "model")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Model to use for shot change detection. Supported values: \"builtin/stable\" (the default if unset) and \"builtin/latest\"."]
    pub model: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Provides \"hints\" to the speech recognizer to favor specific words and phrases in the results."]
pub struct GoogleCloudVideointelligenceV1beta2SpeechContext {
    #[serde(rename = "phrases")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A list of strings containing words and phrases \"hints\" so that the speech recognition is more likely to recognize them. This can be used to improve the accuracy for specific words and phrases, for example, if specific commands are typically spoken by the user. This can also be used to add additional words to the vocabulary of the recognizer. See [usage limits](https://cloud.google.com/speech/limits#content)."]
    pub phrases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Alternative hypotheses (a.k.a. n-best list)."]
pub struct GoogleCloudVideointelligenceV1beta2SpeechRecognitionAlternative {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "transcript")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Transcript text representing the words that the user spoke."]
    pub transcript: ::std::option::Option<::std::string::String>,
    #[serde(rename = "words")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A list of word-specific information for each recognized word. Note: When `enable_speaker_diarization` is set to true, you will see all the words from the beginning of the audio."]
    pub words: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2WordInfo>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A speech recognition result corresponding to a portion of the audio."]
pub struct GoogleCloudVideointelligenceV1beta2SpeechTranscription {
    #[serde(rename = "alternatives")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "May contain one or more recognition hypotheses (up to the maximum specified in `max_alternatives`). These alternatives are ordered in terms of accuracy, with the top (first) alternative being the most probable, as ranked by the recognizer."]
    pub alternatives: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2SpeechRecognitionAlternative>,
        >,
    >,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag of the language in this result. This language code was detected to have the most likelihood of being spoken in the audio."]
    pub language_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Config for SPEECH_TRANSCRIPTION."]
pub struct GoogleCloudVideointelligenceV1beta2SpeechTranscriptionConfig {
    #[serde(rename = "audioTracks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. For file formats, such as MXF or MKV, supporting multiple audio tracks, specify up to two tracks. Default: track 0."]
    pub audio_tracks: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    #[serde(rename = "diarizationSpeakerCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If set, specifies the estimated number of speakers in the conversation. If not set, defaults to '2'. Ignored unless enable_speaker_diarization is set to true."]
    pub diarization_speaker_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "enableAutomaticPunctuation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If 'true', adds punctuation to recognition result hypotheses. This feature is only available in select languages. Setting this for requests in other languages has no effect at all. The default 'false' value does not add punctuation to result hypotheses. NOTE: \"This is currently offered as an experimental service, complimentary to all users. In the future this may be exclusively available as a premium feature.\""]
    pub enable_automatic_punctuation: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "enableSpeakerDiarization")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If 'true', enables speaker detection for each recognized word in the top alternative of the recognition result using a speaker_tag provided in the WordInfo. Note: When this is true, we send all the words from the beginning of the audio for the top alternative in every consecutive response. This is done in order to improve our speaker tags as our models learn to identify the speakers in the conversation over time."]
    pub enable_speaker_diarization: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "enableWordConfidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If `true`, the top result includes a list of words and the confidence for those words. If `false`, no word-level confidence information is returned. The default is `false`."]
    pub enable_word_confidence: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "filterProfanity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If set to `true`, the server will attempt to filter out profanities, replacing all but the initial character in each filtered word with asterisks, e.g. \"f***\". If set to `false` or omitted, profanities won't be filtered out."]
    pub filter_profanity: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. *Required* The language of the supplied audio as a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. Example: \"en-US\". See [Language Support](https://cloud.google.com/speech/docs/languages) for a list of the currently supported language codes."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maxAlternatives")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Maximum number of recognition hypotheses to be returned. Specifically, the maximum number of `SpeechRecognitionAlternative` messages within each `SpeechTranscription`. The server may return fewer than `max_alternatives`. Valid values are `0`-`30`. A value of `0` or `1` will return a maximum of one. If omitted, will return a maximum of one."]
    pub max_alternatives: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "speechContexts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A means to provide context to assist the speech recognition."]
    pub speech_contexts: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2SpeechContext>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotations related to one detected OCR text snippet. This will contain the corresponding text, confidence value, and frame level information for each detection."]
pub struct GoogleCloudVideointelligenceV1beta2TextAnnotation {
    #[serde(rename = "segments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video segments where OCR detected text appears."]
    pub segments: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2TextSegment>>,
    >,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The detected text."]
    pub text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Config for TEXT_DETECTION."]
pub struct GoogleCloudVideointelligenceV1beta2TextDetectionConfig {
    #[serde(rename = "languageHints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Language hint can be specified if the language to be detected is known a priori. It can increase the accuracy of the detection. Language hint must be language code in BCP-47 format. Automatic language detection is performed if no hint is provided."]
    pub language_hints: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "model")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Model to use for text detection. Supported values: \"builtin/stable\" (the default if unset) and \"builtin/latest\"."]
    pub model: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video frame level annotation results for text annotation (OCR). Contains information regarding timestamp and bounding box locations for the frames containing detected OCR text snippets."]
pub struct GoogleCloudVideointelligenceV1beta2TextFrame {
    #[serde(rename = "rotatedBoundingBox")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Bounding polygon of the detected text for this frame."]
    pub rotated_bounding_box: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2NormalizedBoundingPoly>,
    >,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp of this frame."]
    pub time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video segment level annotation results for text detection."]
pub struct GoogleCloudVideointelligenceV1beta2TextSegment {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Confidence for the track of detected text. It is calculated as the highest over all frames where OCR detected text appears."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "frames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information related to the frames where OCR detected text appears."]
    pub frames: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2TextFrame>>,
    >,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video segment where a text snippet was detected."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2VideoSegment>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "For tracking related features. An object at time_offset with attributes, and located with normalized_bounding_box."]
pub struct GoogleCloudVideointelligenceV1beta2TimestampedObject {
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The attributes of the object in the bounding box."]
    pub attributes: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2DetectedAttribute>>,
    >,
    #[serde(rename = "landmarks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The detected landmarks."]
    pub landmarks: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2DetectedLandmark>>,
    >,
    #[serde(rename = "normalizedBoundingBox")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Normalized Bounding box in a frame, where the object is located."]
    pub normalized_bounding_box: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2NormalizedBoundingBox>,
    >,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this object."]
    pub time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A track of an object instance."]
pub struct GoogleCloudVideointelligenceV1beta2Track {
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Attributes in the track level."]
    pub attributes: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2DetectedAttribute>>,
    >,
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The confidence score of the tracked object."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video segment of a track."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2VideoSegment>>,
    #[serde(rename = "timestampedObjects")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The object with timestamp and attributes per frame in the track."]
    pub timestamped_objects: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2TimestampedObject>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotation progress for a single video."]
pub struct GoogleCloudVideointelligenceV1beta2VideoAnnotationProgress {
    #[serde(rename = "feature")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies which feature is being tracked if the request contains more than one feature."]
    pub feature: ::std::option::Option<
        GoogleCloudVideointelligenceV1beta2VideoAnnotationProgressFeatureEnum,
    >,
    #[serde(rename = "inputUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
    pub input_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progressPercent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Approximate percentage processed thus far. Guaranteed to be 100 when fully processed."]
    pub progress_percent: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies which segment is being tracked if the request contains more than one segment."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2VideoSegment>>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time when the request was received."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time of the most recent update."]
    pub update_time: ::std::option::Option<::std::string::String>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotation results for a single video."]
pub struct GoogleCloudVideointelligenceV1beta2VideoAnnotationResults {
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, indicates an error. Note that for a single `AnnotateVideoRequest` some videos may succeed and some may fail."]
    pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
    #[serde(rename = "explicitAnnotation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Explicit content annotation."]
    pub explicit_annotation: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2ExplicitContentAnnotation>,
    >,
    #[serde(rename = "faceAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Please use `face_detection_annotations` instead."]
    pub face_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2FaceAnnotation>>,
    >,
    #[serde(rename = "faceDetectionAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Face detection annotations."]
    pub face_detection_annotations: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2FaceDetectionAnnotation>,
        >,
    >,
    #[serde(rename = "frameLabelAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Label annotations on frame level. There is exactly one element for each unique label."]
    pub frame_label_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2LabelAnnotation>>,
    >,
    #[serde(rename = "inputUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
    pub input_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "logoRecognitionAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Annotations for list of logos detected, tracked and recognized in video."]
    pub logo_recognition_annotations: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2LogoRecognitionAnnotation>,
        >,
    >,
    #[serde(rename = "objectAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Annotations for list of objects detected and tracked in video."]
    pub object_annotations: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2ObjectTrackingAnnotation>,
        >,
    >,
    #[serde(rename = "personDetectionAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Person detection annotations."]
    pub person_detection_annotations: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2PersonDetectionAnnotation>,
        >,
    >,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video segment on which the annotation is run."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2VideoSegment>>,
    #[serde(rename = "segmentLabelAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Topical label annotations on video level or user-specified segment level. There is exactly one element for each unique label."]
    pub segment_label_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2LabelAnnotation>>,
    >,
    #[serde(rename = "segmentPresenceLabelAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Presence label annotations on video level or user-specified segment level. There is exactly one element for each unique label. Compared to the existing topical `segment_label_annotations`, this field presents more fine-grained, segment-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to \"builtin/latest\" in the request."]
    pub segment_presence_label_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2LabelAnnotation>>,
    >,
    #[serde(rename = "shotAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shot annotations. Each shot is represented as a video segment."]
    pub shot_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2VideoSegment>>,
    >,
    #[serde(rename = "shotLabelAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Topical label annotations on shot level. There is exactly one element for each unique label."]
    pub shot_label_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2LabelAnnotation>>,
    >,
    #[serde(rename = "shotPresenceLabelAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Presence label annotations on shot level. There is exactly one element for each unique label. Compared to the existing topical `shot_label_annotations`, this field presents more fine-grained, shot-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to \"builtin/latest\" in the request."]
    pub shot_presence_label_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2LabelAnnotation>>,
    >,
    #[serde(rename = "speechTranscriptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Speech transcription."]
    pub speech_transcriptions: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2SpeechTranscription>>,
    >,
    #[serde(rename = "textAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OCR text detection and tracking. Annotations for list of detected text snippets. Each will have list of frame information associated with it."]
    pub text_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2TextAnnotation>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video context and/or feature-specific parameters."]
pub struct GoogleCloudVideointelligenceV1beta2VideoContext {
    #[serde(rename = "explicitContentDetectionConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Config for EXPLICIT_CONTENT_DETECTION."]
    pub explicit_content_detection_config: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2ExplicitContentDetectionConfig>,
    >,
    #[serde(rename = "faceDetectionConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Config for FACE_DETECTION."]
    pub face_detection_config: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2FaceDetectionConfig>,
    >,
    #[serde(rename = "labelDetectionConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Config for LABEL_DETECTION."]
    pub label_detection_config: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2LabelDetectionConfig>,
    >,
    #[serde(rename = "objectTrackingConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Config for OBJECT_TRACKING."]
    pub object_tracking_config: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2ObjectTrackingConfig>,
    >,
    #[serde(rename = "personDetectionConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Config for PERSON_DETECTION."]
    pub person_detection_config: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2PersonDetectionConfig>,
    >,
    #[serde(rename = "segments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video segments to annotate. The segments may overlap and are not required to be contiguous or span the whole video. If unspecified, each video is treated as a single segment."]
    pub segments: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1beta2VideoSegment>>,
    >,
    #[serde(rename = "shotChangeDetectionConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Config for SHOT_CHANGE_DETECTION."]
    pub shot_change_detection_config: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2ShotChangeDetectionConfig>,
    >,
    #[serde(rename = "speechTranscriptionConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Config for SPEECH_TRANSCRIPTION."]
    pub speech_transcription_config: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2SpeechTranscriptionConfig>,
    >,
    #[serde(rename = "textDetectionConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Config for TEXT_DETECTION."]
    pub text_detection_config: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1beta2TextDetectionConfig>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video segment."]
pub struct GoogleCloudVideointelligenceV1beta2VideoSegment {
    #[serde(rename = "endTimeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the end of the segment (inclusive)."]
    pub end_time_offset: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTimeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the start of the segment (inclusive)."]
    pub start_time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Word-specific information for recognized words. Word information is only included in the response when certain request parameters are set, such as `enable_word_time_offsets`."]
pub struct GoogleCloudVideointelligenceV1beta2WordInfo {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time offset relative to the beginning of the audio, and corresponding to the end of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "speakerTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A distinct integer value is assigned for every speaker within the audio. This field specifies which one of those speakers was detected to have spoken this word. Value ranges from 1 up to diarization_speaker_count, and is only set if speaker diarization is enabled."]
    pub speaker_tag: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time offset relative to the beginning of the audio, and corresponding to the start of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "word")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The word corresponding to this set of information."]
    pub word: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video annotation progress. Included in the `metadata` field of the `Operation` returned by the `GetOperation` call of the `google::longrunning::Operations` service."]
pub struct GoogleCloudVideointelligenceV1p1beta1AnnotateVideoProgress {
    #[serde(rename = "annotationProgress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Progress metadata for all videos specified in `AnnotateVideoRequest`."]
    pub annotation_progress: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1VideoAnnotationProgress>,
        >,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video annotation response. Included in the `response` field of the `Operation` returned by the `GetOperation` call of the `google::longrunning::Operations` service."]
pub struct GoogleCloudVideointelligenceV1p1beta1AnnotateVideoResponse {
    #[serde(rename = "annotationResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Annotation results for all videos specified in `AnnotateVideoRequest`."]
    pub annotation_results: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1VideoAnnotationResults>,
        >,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic detected attribute represented by name in string format."]
pub struct GoogleCloudVideointelligenceV1p1beta1DetectedAttribute {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detected attribute confidence. Range [0, 1]."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the attribute, for example, glasses, dark_glasses, mouth_open. A full list of supported type names will be provided in the document."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Text value of the detection result. For example, the value for \"HairColor\" can be \"black\", \"blonde\", etc."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic detected landmark represented by name in string format and a 2D location."]
pub struct GoogleCloudVideointelligenceV1p1beta1DetectedLandmark {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The confidence score of the detected landmark. Range [0, 1]."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of this landmark, for example, left_hand, right_shoulder."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "point")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The 2D point of the detected landmark using the normalized image coordindate system. The normalized coordinates have the range from 0 to 1."]
    pub point: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1NormalizedVertex>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Detected entity from video analysis."]
pub struct GoogleCloudVideointelligenceV1p1beta1Entity {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Textual description, e.g., `Fixed-gear bicycle`."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Opaque entity ID. Some IDs may be available in [Google Knowledge Graph Search API](https://developers.google.com/knowledge-graph/)."]
    pub entity_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Language code for `description` in BCP-47 format."]
    pub language_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Explicit content annotation (based on per-frame visual signals only). If no explicit content has been detected in a frame, no annotations are present for that frame."]
pub struct GoogleCloudVideointelligenceV1p1beta1ExplicitContentAnnotation {
    #[serde(rename = "frames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video frames where explicit content was detected."]
    pub frames: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1ExplicitContentFrame>,
        >,
    >,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video frame level annotation results for explicit content."]
pub struct GoogleCloudVideointelligenceV1p1beta1ExplicitContentFrame {
    #[serde(rename = "pornographyLikelihood")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Likelihood of the pornography content.."]
    pub pornography_likelihood: ::std::option::Option<
        GoogleCloudVideointelligenceV1p1beta1ExplicitContentFramePornographyLikelihoodEnum,
    >,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
    pub time_offset: ::std::option::Option<::std::string::String>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Deprecated. No effect."]
pub struct GoogleCloudVideointelligenceV1p1beta1FaceAnnotation {
    #[serde(rename = "frames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video frames where a face was detected."]
    pub frames: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1FaceFrame>>,
    >,
    #[serde(rename = "segments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video segments where a face was detected."]
    pub segments: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1FaceSegment>>,
    >,
    #[serde(rename = "thumbnail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Thumbnail of a representative face view (in JPEG format)."]
    pub thumbnail: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Face detection annotation."]
pub struct GoogleCloudVideointelligenceV1p1beta1FaceDetectionAnnotation {
    #[serde(rename = "thumbnail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The thumbnail of a person's face."]
    pub thumbnail: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tracks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The face tracks with attributes."]
    pub tracks: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1Track>>,
    >,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Deprecated. No effect."]
pub struct GoogleCloudVideointelligenceV1p1beta1FaceFrame {
    #[serde(rename = "normalizedBoundingBoxes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Normalized Bounding boxes in a frame. There can be more than one boxes if the same face is detected in multiple locations within the current frame."]
    pub normalized_bounding_boxes: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1NormalizedBoundingBox>,
        >,
    >,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
    pub time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video segment level annotation results for face detection."]
pub struct GoogleCloudVideointelligenceV1p1beta1FaceSegment {
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video segment where a face was detected."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1VideoSegment>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Label annotation."]
pub struct GoogleCloudVideointelligenceV1p1beta1LabelAnnotation {
    #[serde(rename = "categoryEntities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Common categories for the detected entity. For example, when the label is `Terrier`, the category is likely `dog`. And in some cases there might be more than one categories e.g., `Terrier` could also be a `pet`."]
    pub category_entities: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1Entity>>,
    >,
    #[serde(rename = "entity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detected entity."]
    pub entity:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1Entity>>,
    #[serde(rename = "frames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video frames where a label was detected."]
    pub frames: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1LabelFrame>>,
    >,
    #[serde(rename = "segments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video segments where a label was detected."]
    pub segments: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1LabelSegment>>,
    >,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video frame level annotation results for label detection."]
pub struct GoogleCloudVideointelligenceV1p1beta1LabelFrame {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
    pub time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video segment level annotation results for label detection."]
pub struct GoogleCloudVideointelligenceV1p1beta1LabelSegment {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video segment where a label was detected."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1VideoSegment>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotation corresponding to one detected, tracked and recognized logo class."]
pub struct GoogleCloudVideointelligenceV1p1beta1LogoRecognitionAnnotation {
    #[serde(rename = "entity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Entity category information to specify the logo class that all the logo tracks within this LogoRecognitionAnnotation are recognized as."]
    pub entity:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1Entity>>,
    #[serde(rename = "segments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video segments where the recognized logo appears. There might be multiple instances of the same logo class appearing in one VideoSegment."]
    pub segments: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1VideoSegment>>,
    >,
    #[serde(rename = "tracks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All logo tracks where the recognized logo appears. Each track corresponds to one logo instance appearing in consecutive frames."]
    pub tracks: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1Track>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Normalized bounding box. The normalized vertex coordinates are relative to the original image. Range: [0, 1]."]
pub struct GoogleCloudVideointelligenceV1p1beta1NormalizedBoundingBox {
    #[serde(rename = "bottom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Bottom Y coordinate."]
    pub bottom: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "left")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Left X coordinate."]
    pub left: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "right")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Right X coordinate."]
    pub right: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "top")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Top Y coordinate."]
    pub top: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Normalized bounding polygon for text (that might not be aligned with axis). Contains list of the corner points in clockwise order starting from top-left corner. For example, for a rectangular bounding box: When the text is horizontal it might look like: 0----1 | | 3----2 When it's clockwise rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3). Note that values can be less than 0, or greater than 1 due to trignometric calculations for location of the box."]
pub struct GoogleCloudVideointelligenceV1p1beta1NormalizedBoundingPoly {
    #[serde(rename = "vertices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Normalized vertices of the bounding polygon."]
    pub vertices: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1NormalizedVertex>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A vertex represents a 2D point in the image. NOTE: the normalized vertex coordinates are relative to the original image and range from 0 to 1."]
pub struct GoogleCloudVideointelligenceV1p1beta1NormalizedVertex {
    #[serde(rename = "x")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "X coordinate."]
    pub x: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "y")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Y coordinate."]
    pub y: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotations corresponding to one tracked object."]
pub struct GoogleCloudVideointelligenceV1p1beta1ObjectTrackingAnnotation {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Object category's labeling confidence of this track."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "entity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Entity to specify the object category that this track is labeled as."]
    pub entity:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1Entity>>,
    #[serde(rename = "frames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information corresponding to all frames where this object track appears. Non-streaming batch mode: it may be one or multiple ObjectTrackingFrame messages in frames. Streaming mode: it can only be one ObjectTrackingFrame message in frames."]
    pub frames: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1ObjectTrackingFrame>,
        >,
    >,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Non-streaming batch mode ONLY. Each object track corresponds to one video segment where it appears."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1VideoSegment>>,
    #[serde(rename = "trackId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Streaming mode ONLY. In streaming mode, we do not know the end time of a tracked object before it is completed. Hence, there is no VideoSegment info returned. Instead, we provide a unique identifiable integer track_id so that the customers can correlate the results of the ongoing ObjectTrackAnnotation of the same track_id over time."]
    pub track_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video frame level annotations for object detection and tracking. This field stores per frame location, time offset, and confidence."]
pub struct GoogleCloudVideointelligenceV1p1beta1ObjectTrackingFrame {
    #[serde(rename = "normalizedBoundingBox")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The normalized bounding box location of this object track for the frame."]
    pub normalized_bounding_box: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1NormalizedBoundingBox>,
    >,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The timestamp of the frame in microseconds."]
    pub time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Person detection annotation per video."]
pub struct GoogleCloudVideointelligenceV1p1beta1PersonDetectionAnnotation {
    #[serde(rename = "tracks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The detected tracks of a person."]
    pub tracks: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1Track>>,
    >,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Alternative hypotheses (a.k.a. n-best list)."]
pub struct GoogleCloudVideointelligenceV1p1beta1SpeechRecognitionAlternative {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "transcript")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Transcript text representing the words that the user spoke."]
    pub transcript: ::std::option::Option<::std::string::String>,
    #[serde(rename = "words")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A list of word-specific information for each recognized word. Note: When `enable_speaker_diarization` is set to true, you will see all the words from the beginning of the audio."]
    pub words: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1WordInfo>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A speech recognition result corresponding to a portion of the audio."]
pub struct GoogleCloudVideointelligenceV1p1beta1SpeechTranscription {
    #[serde(rename = "alternatives")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "May contain one or more recognition hypotheses (up to the maximum specified in `max_alternatives`). These alternatives are ordered in terms of accuracy, with the top (first) alternative being the most probable, as ranked by the recognizer."]
    pub alternatives: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1SpeechRecognitionAlternative>,
        >,
    >,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag of the language in this result. This language code was detected to have the most likelihood of being spoken in the audio."]
    pub language_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotations related to one detected OCR text snippet. This will contain the corresponding text, confidence value, and frame level information for each detection."]
pub struct GoogleCloudVideointelligenceV1p1beta1TextAnnotation {
    #[serde(rename = "segments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video segments where OCR detected text appears."]
    pub segments: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1TextSegment>>,
    >,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The detected text."]
    pub text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video frame level annotation results for text annotation (OCR). Contains information regarding timestamp and bounding box locations for the frames containing detected OCR text snippets."]
pub struct GoogleCloudVideointelligenceV1p1beta1TextFrame {
    #[serde(rename = "rotatedBoundingBox")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Bounding polygon of the detected text for this frame."]
    pub rotated_bounding_box: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1NormalizedBoundingPoly>,
    >,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp of this frame."]
    pub time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video segment level annotation results for text detection."]
pub struct GoogleCloudVideointelligenceV1p1beta1TextSegment {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Confidence for the track of detected text. It is calculated as the highest over all frames where OCR detected text appears."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "frames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information related to the frames where OCR detected text appears."]
    pub frames: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1TextFrame>>,
    >,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video segment where a text snippet was detected."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1VideoSegment>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "For tracking related features. An object at time_offset with attributes, and located with normalized_bounding_box."]
pub struct GoogleCloudVideointelligenceV1p1beta1TimestampedObject {
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The attributes of the object in the bounding box."]
    pub attributes: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1DetectedAttribute>>,
    >,
    #[serde(rename = "landmarks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The detected landmarks."]
    pub landmarks: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1DetectedLandmark>>,
    >,
    #[serde(rename = "normalizedBoundingBox")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Normalized Bounding box in a frame, where the object is located."]
    pub normalized_bounding_box: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1NormalizedBoundingBox>,
    >,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this object."]
    pub time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A track of an object instance."]
pub struct GoogleCloudVideointelligenceV1p1beta1Track {
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Attributes in the track level."]
    pub attributes: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1DetectedAttribute>>,
    >,
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The confidence score of the tracked object."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video segment of a track."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1VideoSegment>>,
    #[serde(rename = "timestampedObjects")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The object with timestamp and attributes per frame in the track."]
    pub timestamped_objects: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1TimestampedObject>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotation progress for a single video."]
pub struct GoogleCloudVideointelligenceV1p1beta1VideoAnnotationProgress {
    #[serde(rename = "feature")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies which feature is being tracked if the request contains more than one feature."]
    pub feature: ::std::option::Option<
        GoogleCloudVideointelligenceV1p1beta1VideoAnnotationProgressFeatureEnum,
    >,
    #[serde(rename = "inputUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
    pub input_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progressPercent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Approximate percentage processed thus far. Guaranteed to be 100 when fully processed."]
    pub progress_percent: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies which segment is being tracked if the request contains more than one segment."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1VideoSegment>>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time when the request was received."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time of the most recent update."]
    pub update_time: ::std::option::Option<::std::string::String>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotation results for a single video."]
pub struct GoogleCloudVideointelligenceV1p1beta1VideoAnnotationResults {
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, indicates an error. Note that for a single `AnnotateVideoRequest` some videos may succeed and some may fail."]
    pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
    #[serde(rename = "explicitAnnotation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Explicit content annotation."]
    pub explicit_annotation: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1ExplicitContentAnnotation>,
    >,
    #[serde(rename = "faceAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Please use `face_detection_annotations` instead."]
    pub face_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1FaceAnnotation>>,
    >,
    #[serde(rename = "faceDetectionAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Face detection annotations."]
    pub face_detection_annotations: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1FaceDetectionAnnotation>,
        >,
    >,
    #[serde(rename = "frameLabelAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Label annotations on frame level. There is exactly one element for each unique label."]
    pub frame_label_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1LabelAnnotation>>,
    >,
    #[serde(rename = "inputUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
    pub input_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "logoRecognitionAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Annotations for list of logos detected, tracked and recognized in video."]
    pub logo_recognition_annotations: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1LogoRecognitionAnnotation>,
        >,
    >,
    #[serde(rename = "objectAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Annotations for list of objects detected and tracked in video."]
    pub object_annotations: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1ObjectTrackingAnnotation>,
        >,
    >,
    #[serde(rename = "personDetectionAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Person detection annotations."]
    pub person_detection_annotations: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1PersonDetectionAnnotation>,
        >,
    >,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video segment on which the annotation is run."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1VideoSegment>>,
    #[serde(rename = "segmentLabelAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Topical label annotations on video level or user-specified segment level. There is exactly one element for each unique label."]
    pub segment_label_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1LabelAnnotation>>,
    >,
    #[serde(rename = "segmentPresenceLabelAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Presence label annotations on video level or user-specified segment level. There is exactly one element for each unique label. Compared to the existing topical `segment_label_annotations`, this field presents more fine-grained, segment-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to \"builtin/latest\" in the request."]
    pub segment_presence_label_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1LabelAnnotation>>,
    >,
    #[serde(rename = "shotAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shot annotations. Each shot is represented as a video segment."]
    pub shot_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1VideoSegment>>,
    >,
    #[serde(rename = "shotLabelAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Topical label annotations on shot level. There is exactly one element for each unique label."]
    pub shot_label_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1LabelAnnotation>>,
    >,
    #[serde(rename = "shotPresenceLabelAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Presence label annotations on shot level. There is exactly one element for each unique label. Compared to the existing topical `shot_label_annotations`, this field presents more fine-grained, shot-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to \"builtin/latest\" in the request."]
    pub shot_presence_label_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1LabelAnnotation>>,
    >,
    #[serde(rename = "speechTranscriptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Speech transcription."]
    pub speech_transcriptions: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1SpeechTranscription>,
        >,
    >,
    #[serde(rename = "textAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OCR text detection and tracking. Annotations for list of detected text snippets. Each will have list of frame information associated with it."]
    pub text_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p1beta1TextAnnotation>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video segment."]
pub struct GoogleCloudVideointelligenceV1p1beta1VideoSegment {
    #[serde(rename = "endTimeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the end of the segment (inclusive)."]
    pub end_time_offset: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTimeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the start of the segment (inclusive)."]
    pub start_time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Word-specific information for recognized words. Word information is only included in the response when certain request parameters are set, such as `enable_word_time_offsets`."]
pub struct GoogleCloudVideointelligenceV1p1beta1WordInfo {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time offset relative to the beginning of the audio, and corresponding to the end of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "speakerTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A distinct integer value is assigned for every speaker within the audio. This field specifies which one of those speakers was detected to have spoken this word. Value ranges from 1 up to diarization_speaker_count, and is only set if speaker diarization is enabled."]
    pub speaker_tag: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time offset relative to the beginning of the audio, and corresponding to the start of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "word")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The word corresponding to this set of information."]
    pub word: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video annotation progress. Included in the `metadata` field of the `Operation` returned by the `GetOperation` call of the `google::longrunning::Operations` service."]
pub struct GoogleCloudVideointelligenceV1p2beta1AnnotateVideoProgress {
    #[serde(rename = "annotationProgress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Progress metadata for all videos specified in `AnnotateVideoRequest`."]
    pub annotation_progress: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1VideoAnnotationProgress>,
        >,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video annotation response. Included in the `response` field of the `Operation` returned by the `GetOperation` call of the `google::longrunning::Operations` service."]
pub struct GoogleCloudVideointelligenceV1p2beta1AnnotateVideoResponse {
    #[serde(rename = "annotationResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Annotation results for all videos specified in `AnnotateVideoRequest`."]
    pub annotation_results: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1VideoAnnotationResults>,
        >,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic detected attribute represented by name in string format."]
pub struct GoogleCloudVideointelligenceV1p2beta1DetectedAttribute {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detected attribute confidence. Range [0, 1]."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the attribute, for example, glasses, dark_glasses, mouth_open. A full list of supported type names will be provided in the document."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Text value of the detection result. For example, the value for \"HairColor\" can be \"black\", \"blonde\", etc."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic detected landmark represented by name in string format and a 2D location."]
pub struct GoogleCloudVideointelligenceV1p2beta1DetectedLandmark {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The confidence score of the detected landmark. Range [0, 1]."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of this landmark, for example, left_hand, right_shoulder."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "point")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The 2D point of the detected landmark using the normalized image coordindate system. The normalized coordinates have the range from 0 to 1."]
    pub point: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1NormalizedVertex>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Detected entity from video analysis."]
pub struct GoogleCloudVideointelligenceV1p2beta1Entity {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Textual description, e.g., `Fixed-gear bicycle`."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Opaque entity ID. Some IDs may be available in [Google Knowledge Graph Search API](https://developers.google.com/knowledge-graph/)."]
    pub entity_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Language code for `description` in BCP-47 format."]
    pub language_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Explicit content annotation (based on per-frame visual signals only). If no explicit content has been detected in a frame, no annotations are present for that frame."]
pub struct GoogleCloudVideointelligenceV1p2beta1ExplicitContentAnnotation {
    #[serde(rename = "frames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video frames where explicit content was detected."]
    pub frames: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1ExplicitContentFrame>,
        >,
    >,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video frame level annotation results for explicit content."]
pub struct GoogleCloudVideointelligenceV1p2beta1ExplicitContentFrame {
    #[serde(rename = "pornographyLikelihood")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Likelihood of the pornography content.."]
    pub pornography_likelihood: ::std::option::Option<
        GoogleCloudVideointelligenceV1p2beta1ExplicitContentFramePornographyLikelihoodEnum,
    >,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
    pub time_offset: ::std::option::Option<::std::string::String>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Deprecated. No effect."]
pub struct GoogleCloudVideointelligenceV1p2beta1FaceAnnotation {
    #[serde(rename = "frames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video frames where a face was detected."]
    pub frames: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1FaceFrame>>,
    >,
    #[serde(rename = "segments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video segments where a face was detected."]
    pub segments: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1FaceSegment>>,
    >,
    #[serde(rename = "thumbnail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Thumbnail of a representative face view (in JPEG format)."]
    pub thumbnail: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Face detection annotation."]
pub struct GoogleCloudVideointelligenceV1p2beta1FaceDetectionAnnotation {
    #[serde(rename = "thumbnail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The thumbnail of a person's face."]
    pub thumbnail: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tracks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The face tracks with attributes."]
    pub tracks: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1Track>>,
    >,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Deprecated. No effect."]
pub struct GoogleCloudVideointelligenceV1p2beta1FaceFrame {
    #[serde(rename = "normalizedBoundingBoxes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Normalized Bounding boxes in a frame. There can be more than one boxes if the same face is detected in multiple locations within the current frame."]
    pub normalized_bounding_boxes: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1NormalizedBoundingBox>,
        >,
    >,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
    pub time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video segment level annotation results for face detection."]
pub struct GoogleCloudVideointelligenceV1p2beta1FaceSegment {
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video segment where a face was detected."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1VideoSegment>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Label annotation."]
pub struct GoogleCloudVideointelligenceV1p2beta1LabelAnnotation {
    #[serde(rename = "categoryEntities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Common categories for the detected entity. For example, when the label is `Terrier`, the category is likely `dog`. And in some cases there might be more than one categories e.g., `Terrier` could also be a `pet`."]
    pub category_entities: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1Entity>>,
    >,
    #[serde(rename = "entity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detected entity."]
    pub entity:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1Entity>>,
    #[serde(rename = "frames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video frames where a label was detected."]
    pub frames: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1LabelFrame>>,
    >,
    #[serde(rename = "segments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video segments where a label was detected."]
    pub segments: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1LabelSegment>>,
    >,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video frame level annotation results for label detection."]
pub struct GoogleCloudVideointelligenceV1p2beta1LabelFrame {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
    pub time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video segment level annotation results for label detection."]
pub struct GoogleCloudVideointelligenceV1p2beta1LabelSegment {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video segment where a label was detected."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1VideoSegment>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotation corresponding to one detected, tracked and recognized logo class."]
pub struct GoogleCloudVideointelligenceV1p2beta1LogoRecognitionAnnotation {
    #[serde(rename = "entity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Entity category information to specify the logo class that all the logo tracks within this LogoRecognitionAnnotation are recognized as."]
    pub entity:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1Entity>>,
    #[serde(rename = "segments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video segments where the recognized logo appears. There might be multiple instances of the same logo class appearing in one VideoSegment."]
    pub segments: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1VideoSegment>>,
    >,
    #[serde(rename = "tracks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All logo tracks where the recognized logo appears. Each track corresponds to one logo instance appearing in consecutive frames."]
    pub tracks: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1Track>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Normalized bounding box. The normalized vertex coordinates are relative to the original image. Range: [0, 1]."]
pub struct GoogleCloudVideointelligenceV1p2beta1NormalizedBoundingBox {
    #[serde(rename = "bottom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Bottom Y coordinate."]
    pub bottom: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "left")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Left X coordinate."]
    pub left: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "right")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Right X coordinate."]
    pub right: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "top")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Top Y coordinate."]
    pub top: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Normalized bounding polygon for text (that might not be aligned with axis). Contains list of the corner points in clockwise order starting from top-left corner. For example, for a rectangular bounding box: When the text is horizontal it might look like: 0----1 | | 3----2 When it's clockwise rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3). Note that values can be less than 0, or greater than 1 due to trignometric calculations for location of the box."]
pub struct GoogleCloudVideointelligenceV1p2beta1NormalizedBoundingPoly {
    #[serde(rename = "vertices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Normalized vertices of the bounding polygon."]
    pub vertices: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1NormalizedVertex>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A vertex represents a 2D point in the image. NOTE: the normalized vertex coordinates are relative to the original image and range from 0 to 1."]
pub struct GoogleCloudVideointelligenceV1p2beta1NormalizedVertex {
    #[serde(rename = "x")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "X coordinate."]
    pub x: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "y")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Y coordinate."]
    pub y: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotations corresponding to one tracked object."]
pub struct GoogleCloudVideointelligenceV1p2beta1ObjectTrackingAnnotation {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Object category's labeling confidence of this track."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "entity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Entity to specify the object category that this track is labeled as."]
    pub entity:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1Entity>>,
    #[serde(rename = "frames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information corresponding to all frames where this object track appears. Non-streaming batch mode: it may be one or multiple ObjectTrackingFrame messages in frames. Streaming mode: it can only be one ObjectTrackingFrame message in frames."]
    pub frames: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1ObjectTrackingFrame>,
        >,
    >,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Non-streaming batch mode ONLY. Each object track corresponds to one video segment where it appears."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1VideoSegment>>,
    #[serde(rename = "trackId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Streaming mode ONLY. In streaming mode, we do not know the end time of a tracked object before it is completed. Hence, there is no VideoSegment info returned. Instead, we provide a unique identifiable integer track_id so that the customers can correlate the results of the ongoing ObjectTrackAnnotation of the same track_id over time."]
    pub track_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video frame level annotations for object detection and tracking. This field stores per frame location, time offset, and confidence."]
pub struct GoogleCloudVideointelligenceV1p2beta1ObjectTrackingFrame {
    #[serde(rename = "normalizedBoundingBox")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The normalized bounding box location of this object track for the frame."]
    pub normalized_bounding_box: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1NormalizedBoundingBox>,
    >,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The timestamp of the frame in microseconds."]
    pub time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Person detection annotation per video."]
pub struct GoogleCloudVideointelligenceV1p2beta1PersonDetectionAnnotation {
    #[serde(rename = "tracks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The detected tracks of a person."]
    pub tracks: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1Track>>,
    >,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Alternative hypotheses (a.k.a. n-best list)."]
pub struct GoogleCloudVideointelligenceV1p2beta1SpeechRecognitionAlternative {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "transcript")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Transcript text representing the words that the user spoke."]
    pub transcript: ::std::option::Option<::std::string::String>,
    #[serde(rename = "words")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A list of word-specific information for each recognized word. Note: When `enable_speaker_diarization` is set to true, you will see all the words from the beginning of the audio."]
    pub words: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1WordInfo>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A speech recognition result corresponding to a portion of the audio."]
pub struct GoogleCloudVideointelligenceV1p2beta1SpeechTranscription {
    #[serde(rename = "alternatives")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "May contain one or more recognition hypotheses (up to the maximum specified in `max_alternatives`). These alternatives are ordered in terms of accuracy, with the top (first) alternative being the most probable, as ranked by the recognizer."]
    pub alternatives: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1SpeechRecognitionAlternative>,
        >,
    >,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag of the language in this result. This language code was detected to have the most likelihood of being spoken in the audio."]
    pub language_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotations related to one detected OCR text snippet. This will contain the corresponding text, confidence value, and frame level information for each detection."]
pub struct GoogleCloudVideointelligenceV1p2beta1TextAnnotation {
    #[serde(rename = "segments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video segments where OCR detected text appears."]
    pub segments: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1TextSegment>>,
    >,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The detected text."]
    pub text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video frame level annotation results for text annotation (OCR). Contains information regarding timestamp and bounding box locations for the frames containing detected OCR text snippets."]
pub struct GoogleCloudVideointelligenceV1p2beta1TextFrame {
    #[serde(rename = "rotatedBoundingBox")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Bounding polygon of the detected text for this frame."]
    pub rotated_bounding_box: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1NormalizedBoundingPoly>,
    >,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp of this frame."]
    pub time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video segment level annotation results for text detection."]
pub struct GoogleCloudVideointelligenceV1p2beta1TextSegment {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Confidence for the track of detected text. It is calculated as the highest over all frames where OCR detected text appears."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "frames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information related to the frames where OCR detected text appears."]
    pub frames: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1TextFrame>>,
    >,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video segment where a text snippet was detected."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1VideoSegment>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "For tracking related features. An object at time_offset with attributes, and located with normalized_bounding_box."]
pub struct GoogleCloudVideointelligenceV1p2beta1TimestampedObject {
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The attributes of the object in the bounding box."]
    pub attributes: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1DetectedAttribute>>,
    >,
    #[serde(rename = "landmarks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The detected landmarks."]
    pub landmarks: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1DetectedLandmark>>,
    >,
    #[serde(rename = "normalizedBoundingBox")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Normalized Bounding box in a frame, where the object is located."]
    pub normalized_bounding_box: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1NormalizedBoundingBox>,
    >,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this object."]
    pub time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A track of an object instance."]
pub struct GoogleCloudVideointelligenceV1p2beta1Track {
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Attributes in the track level."]
    pub attributes: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1DetectedAttribute>>,
    >,
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The confidence score of the tracked object."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video segment of a track."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1VideoSegment>>,
    #[serde(rename = "timestampedObjects")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The object with timestamp and attributes per frame in the track."]
    pub timestamped_objects: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1TimestampedObject>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotation progress for a single video."]
pub struct GoogleCloudVideointelligenceV1p2beta1VideoAnnotationProgress {
    #[serde(rename = "feature")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies which feature is being tracked if the request contains more than one feature."]
    pub feature: ::std::option::Option<
        GoogleCloudVideointelligenceV1p2beta1VideoAnnotationProgressFeatureEnum,
    >,
    #[serde(rename = "inputUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
    pub input_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progressPercent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Approximate percentage processed thus far. Guaranteed to be 100 when fully processed."]
    pub progress_percent: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies which segment is being tracked if the request contains more than one segment."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1VideoSegment>>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time when the request was received."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time of the most recent update."]
    pub update_time: ::std::option::Option<::std::string::String>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotation results for a single video."]
pub struct GoogleCloudVideointelligenceV1p2beta1VideoAnnotationResults {
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, indicates an error. Note that for a single `AnnotateVideoRequest` some videos may succeed and some may fail."]
    pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
    #[serde(rename = "explicitAnnotation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Explicit content annotation."]
    pub explicit_annotation: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1ExplicitContentAnnotation>,
    >,
    #[serde(rename = "faceAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Please use `face_detection_annotations` instead."]
    pub face_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1FaceAnnotation>>,
    >,
    #[serde(rename = "faceDetectionAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Face detection annotations."]
    pub face_detection_annotations: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1FaceDetectionAnnotation>,
        >,
    >,
    #[serde(rename = "frameLabelAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Label annotations on frame level. There is exactly one element for each unique label."]
    pub frame_label_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1LabelAnnotation>>,
    >,
    #[serde(rename = "inputUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
    pub input_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "logoRecognitionAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Annotations for list of logos detected, tracked and recognized in video."]
    pub logo_recognition_annotations: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1LogoRecognitionAnnotation>,
        >,
    >,
    #[serde(rename = "objectAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Annotations for list of objects detected and tracked in video."]
    pub object_annotations: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1ObjectTrackingAnnotation>,
        >,
    >,
    #[serde(rename = "personDetectionAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Person detection annotations."]
    pub person_detection_annotations: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1PersonDetectionAnnotation>,
        >,
    >,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video segment on which the annotation is run."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1VideoSegment>>,
    #[serde(rename = "segmentLabelAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Topical label annotations on video level or user-specified segment level. There is exactly one element for each unique label."]
    pub segment_label_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1LabelAnnotation>>,
    >,
    #[serde(rename = "segmentPresenceLabelAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Presence label annotations on video level or user-specified segment level. There is exactly one element for each unique label. Compared to the existing topical `segment_label_annotations`, this field presents more fine-grained, segment-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to \"builtin/latest\" in the request."]
    pub segment_presence_label_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1LabelAnnotation>>,
    >,
    #[serde(rename = "shotAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shot annotations. Each shot is represented as a video segment."]
    pub shot_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1VideoSegment>>,
    >,
    #[serde(rename = "shotLabelAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Topical label annotations on shot level. There is exactly one element for each unique label."]
    pub shot_label_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1LabelAnnotation>>,
    >,
    #[serde(rename = "shotPresenceLabelAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Presence label annotations on shot level. There is exactly one element for each unique label. Compared to the existing topical `shot_label_annotations`, this field presents more fine-grained, shot-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to \"builtin/latest\" in the request."]
    pub shot_presence_label_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1LabelAnnotation>>,
    >,
    #[serde(rename = "speechTranscriptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Speech transcription."]
    pub speech_transcriptions: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1SpeechTranscription>,
        >,
    >,
    #[serde(rename = "textAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OCR text detection and tracking. Annotations for list of detected text snippets. Each will have list of frame information associated with it."]
    pub text_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p2beta1TextAnnotation>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video segment."]
pub struct GoogleCloudVideointelligenceV1p2beta1VideoSegment {
    #[serde(rename = "endTimeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the end of the segment (inclusive)."]
    pub end_time_offset: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTimeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the start of the segment (inclusive)."]
    pub start_time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Word-specific information for recognized words. Word information is only included in the response when certain request parameters are set, such as `enable_word_time_offsets`."]
pub struct GoogleCloudVideointelligenceV1p2beta1WordInfo {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time offset relative to the beginning of the audio, and corresponding to the end of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "speakerTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A distinct integer value is assigned for every speaker within the audio. This field specifies which one of those speakers was detected to have spoken this word. Value ranges from 1 up to diarization_speaker_count, and is only set if speaker diarization is enabled."]
    pub speaker_tag: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time offset relative to the beginning of the audio, and corresponding to the start of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "word")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The word corresponding to this set of information."]
    pub word: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video annotation progress. Included in the `metadata` field of the `Operation` returned by the `GetOperation` call of the `google::longrunning::Operations` service."]
pub struct GoogleCloudVideointelligenceV1p3beta1AnnotateVideoProgress {
    #[serde(rename = "annotationProgress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Progress metadata for all videos specified in `AnnotateVideoRequest`."]
    pub annotation_progress: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1VideoAnnotationProgress>,
        >,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video annotation response. Included in the `response` field of the `Operation` returned by the `GetOperation` call of the `google::longrunning::Operations` service."]
pub struct GoogleCloudVideointelligenceV1p3beta1AnnotateVideoResponse {
    #[serde(rename = "annotationResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Annotation results for all videos specified in `AnnotateVideoRequest`."]
    pub annotation_results: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1VideoAnnotationResults>,
        >,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Celebrity definition."]
pub struct GoogleCloudVideointelligenceV1p3beta1Celebrity {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Textual description of additional information about the celebrity, if applicable."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The celebrity name."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the celebrity. Have the format `video-intelligence/kg-mid` indicates a celebrity from preloaded gallery. kg-mid is the id in Google knowledge graph, which is unique for the celebrity."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Celebrity recognition annotation per video."]
pub struct GoogleCloudVideointelligenceV1p3beta1CelebrityRecognitionAnnotation {
    #[serde(rename = "celebrityTracks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The tracks detected from the input video, including recognized celebrities and other detected faces in the video."]
    pub celebrity_tracks: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1CelebrityTrack>>,
    >,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The annotation result of a celebrity face track. RecognizedCelebrity field could be empty if the face track does not have any matched celebrities."]
pub struct GoogleCloudVideointelligenceV1p3beta1CelebrityTrack {
    #[serde(rename = "celebrities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Top N match of the celebrities for the face in this track."]
    pub celebrities: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1RecognizedCelebrity>,
        >,
    >,
    #[serde(rename = "faceTrack")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A track of a person's face."]
    pub face_track:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1Track>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic detected attribute represented by name in string format."]
pub struct GoogleCloudVideointelligenceV1p3beta1DetectedAttribute {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detected attribute confidence. Range [0, 1]."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the attribute, for example, glasses, dark_glasses, mouth_open. A full list of supported type names will be provided in the document."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Text value of the detection result. For example, the value for \"HairColor\" can be \"black\", \"blonde\", etc."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic detected landmark represented by name in string format and a 2D location."]
pub struct GoogleCloudVideointelligenceV1p3beta1DetectedLandmark {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The confidence score of the detected landmark. Range [0, 1]."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of this landmark, for example, left_hand, right_shoulder."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "point")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The 2D point of the detected landmark using the normalized image coordindate system. The normalized coordinates have the range from 0 to 1."]
    pub point: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1NormalizedVertex>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Detected entity from video analysis."]
pub struct GoogleCloudVideointelligenceV1p3beta1Entity {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Textual description, e.g., `Fixed-gear bicycle`."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Opaque entity ID. Some IDs may be available in [Google Knowledge Graph Search API](https://developers.google.com/knowledge-graph/)."]
    pub entity_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Language code for `description` in BCP-47 format."]
    pub language_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Explicit content annotation (based on per-frame visual signals only). If no explicit content has been detected in a frame, no annotations are present for that frame."]
pub struct GoogleCloudVideointelligenceV1p3beta1ExplicitContentAnnotation {
    #[serde(rename = "frames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video frames where explicit content was detected."]
    pub frames: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1ExplicitContentFrame>,
        >,
    >,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video frame level annotation results for explicit content."]
pub struct GoogleCloudVideointelligenceV1p3beta1ExplicitContentFrame {
    #[serde(rename = "pornographyLikelihood")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Likelihood of the pornography content.."]
    pub pornography_likelihood: ::std::option::Option<
        GoogleCloudVideointelligenceV1p3beta1ExplicitContentFramePornographyLikelihoodEnum,
    >,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
    pub time_offset: ::std::option::Option<::std::string::String>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Deprecated. No effect."]
pub struct GoogleCloudVideointelligenceV1p3beta1FaceAnnotation {
    #[serde(rename = "frames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video frames where a face was detected."]
    pub frames: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1FaceFrame>>,
    >,
    #[serde(rename = "segments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video segments where a face was detected."]
    pub segments: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1FaceSegment>>,
    >,
    #[serde(rename = "thumbnail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Thumbnail of a representative face view (in JPEG format)."]
    pub thumbnail: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Face detection annotation."]
pub struct GoogleCloudVideointelligenceV1p3beta1FaceDetectionAnnotation {
    #[serde(rename = "thumbnail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The thumbnail of a person's face."]
    pub thumbnail: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tracks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The face tracks with attributes."]
    pub tracks: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1Track>>,
    >,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Deprecated. No effect."]
pub struct GoogleCloudVideointelligenceV1p3beta1FaceFrame {
    #[serde(rename = "normalizedBoundingBoxes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Normalized Bounding boxes in a frame. There can be more than one boxes if the same face is detected in multiple locations within the current frame."]
    pub normalized_bounding_boxes: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1NormalizedBoundingBox>,
        >,
    >,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
    pub time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video segment level annotation results for face detection."]
pub struct GoogleCloudVideointelligenceV1p3beta1FaceSegment {
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video segment where a face was detected."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1VideoSegment>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Label annotation."]
pub struct GoogleCloudVideointelligenceV1p3beta1LabelAnnotation {
    #[serde(rename = "categoryEntities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Common categories for the detected entity. For example, when the label is `Terrier`, the category is likely `dog`. And in some cases there might be more than one categories e.g., `Terrier` could also be a `pet`."]
    pub category_entities: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1Entity>>,
    >,
    #[serde(rename = "entity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detected entity."]
    pub entity:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1Entity>>,
    #[serde(rename = "frames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video frames where a label was detected."]
    pub frames: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1LabelFrame>>,
    >,
    #[serde(rename = "segments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video segments where a label was detected."]
    pub segments: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1LabelSegment>>,
    >,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video frame level annotation results for label detection."]
pub struct GoogleCloudVideointelligenceV1p3beta1LabelFrame {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
    pub time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video segment level annotation results for label detection."]
pub struct GoogleCloudVideointelligenceV1p3beta1LabelSegment {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Confidence that the label is accurate. Range: [0, 1]."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video segment where a label was detected."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1VideoSegment>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotation corresponding to one detected, tracked and recognized logo class."]
pub struct GoogleCloudVideointelligenceV1p3beta1LogoRecognitionAnnotation {
    #[serde(rename = "entity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Entity category information to specify the logo class that all the logo tracks within this LogoRecognitionAnnotation are recognized as."]
    pub entity:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1Entity>>,
    #[serde(rename = "segments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video segments where the recognized logo appears. There might be multiple instances of the same logo class appearing in one VideoSegment."]
    pub segments: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1VideoSegment>>,
    >,
    #[serde(rename = "tracks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All logo tracks where the recognized logo appears. Each track corresponds to one logo instance appearing in consecutive frames."]
    pub tracks: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1Track>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Normalized bounding box. The normalized vertex coordinates are relative to the original image. Range: [0, 1]."]
pub struct GoogleCloudVideointelligenceV1p3beta1NormalizedBoundingBox {
    #[serde(rename = "bottom")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Bottom Y coordinate."]
    pub bottom: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "left")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Left X coordinate."]
    pub left: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "right")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Right X coordinate."]
    pub right: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "top")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Top Y coordinate."]
    pub top: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Normalized bounding polygon for text (that might not be aligned with axis). Contains list of the corner points in clockwise order starting from top-left corner. For example, for a rectangular bounding box: When the text is horizontal it might look like: 0----1 | | 3----2 When it's clockwise rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3). Note that values can be less than 0, or greater than 1 due to trignometric calculations for location of the box."]
pub struct GoogleCloudVideointelligenceV1p3beta1NormalizedBoundingPoly {
    #[serde(rename = "vertices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Normalized vertices of the bounding polygon."]
    pub vertices: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1NormalizedVertex>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A vertex represents a 2D point in the image. NOTE: the normalized vertex coordinates are relative to the original image and range from 0 to 1."]
pub struct GoogleCloudVideointelligenceV1p3beta1NormalizedVertex {
    #[serde(rename = "x")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "X coordinate."]
    pub x: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "y")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Y coordinate."]
    pub y: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotations corresponding to one tracked object."]
pub struct GoogleCloudVideointelligenceV1p3beta1ObjectTrackingAnnotation {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Object category's labeling confidence of this track."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "entity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Entity to specify the object category that this track is labeled as."]
    pub entity:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1Entity>>,
    #[serde(rename = "frames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information corresponding to all frames where this object track appears. Non-streaming batch mode: it may be one or multiple ObjectTrackingFrame messages in frames. Streaming mode: it can only be one ObjectTrackingFrame message in frames."]
    pub frames: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1ObjectTrackingFrame>,
        >,
    >,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Non-streaming batch mode ONLY. Each object track corresponds to one video segment where it appears."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1VideoSegment>>,
    #[serde(rename = "trackId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Streaming mode ONLY. In streaming mode, we do not know the end time of a tracked object before it is completed. Hence, there is no VideoSegment info returned. Instead, we provide a unique identifiable integer track_id so that the customers can correlate the results of the ongoing ObjectTrackAnnotation of the same track_id over time."]
    pub track_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video frame level annotations for object detection and tracking. This field stores per frame location, time offset, and confidence."]
pub struct GoogleCloudVideointelligenceV1p3beta1ObjectTrackingFrame {
    #[serde(rename = "normalizedBoundingBox")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The normalized bounding box location of this object track for the frame."]
    pub normalized_bounding_box: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1NormalizedBoundingBox>,
    >,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The timestamp of the frame in microseconds."]
    pub time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Person detection annotation per video."]
pub struct GoogleCloudVideointelligenceV1p3beta1PersonDetectionAnnotation {
    #[serde(rename = "tracks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The detected tracks of a person."]
    pub tracks: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1Track>>,
    >,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The recognized celebrity with confidence score."]
pub struct GoogleCloudVideointelligenceV1p3beta1RecognizedCelebrity {
    #[serde(rename = "celebrity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The recognized celebrity."]
    pub celebrity:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1Celebrity>>,
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Recognition confidence. Range [0, 1]."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Alternative hypotheses (a.k.a. n-best list)."]
pub struct GoogleCloudVideointelligenceV1p3beta1SpeechRecognitionAlternative {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "transcript")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Transcript text representing the words that the user spoke."]
    pub transcript: ::std::option::Option<::std::string::String>,
    #[serde(rename = "words")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A list of word-specific information for each recognized word. Note: When `enable_speaker_diarization` is set to true, you will see all the words from the beginning of the audio."]
    pub words: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1WordInfo>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A speech recognition result corresponding to a portion of the audio."]
pub struct GoogleCloudVideointelligenceV1p3beta1SpeechTranscription {
    #[serde(rename = "alternatives")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "May contain one or more recognition hypotheses (up to the maximum specified in `max_alternatives`). These alternatives are ordered in terms of accuracy, with the top (first) alternative being the most probable, as ranked by the recognizer."]
    pub alternatives: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1SpeechRecognitionAlternative>,
        >,
    >,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag of the language in this result. This language code was detected to have the most likelihood of being spoken in the audio."]
    pub language_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "`StreamingAnnotateVideoResponse` is the only message returned to the client by `StreamingAnnotateVideo`. A series of zero or more `StreamingAnnotateVideoResponse` messages are streamed back to the client."]
pub struct GoogleCloudVideointelligenceV1p3beta1StreamingAnnotateVideoResponse {
    #[serde(rename = "annotationResults")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Streaming annotation results."]
    pub annotation_results: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1StreamingVideoAnnotationResults>,
    >,
    #[serde(rename = "annotationResultsUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Google Cloud Storage URI that stores annotation results of one streaming session in JSON format. It is the annotation_result_storage_directory from the request followed by '/cloud_project_number-session_id'."]
    pub annotation_results_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, returns a google.rpc.Status message that specifies the error for the operation."]
    pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Streaming annotation results corresponding to a portion of the video that is currently being processed. Only ONE type of annotation will be specified in the response."]
pub struct GoogleCloudVideointelligenceV1p3beta1StreamingVideoAnnotationResults {
    #[serde(rename = "explicitAnnotation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Explicit content annotation results."]
    pub explicit_annotation: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1ExplicitContentAnnotation>,
    >,
    #[serde(rename = "frameTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp of the processed frame in microseconds."]
    pub frame_timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labelAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Label annotation results."]
    pub label_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1LabelAnnotation>>,
    >,
    #[serde(rename = "objectAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Object tracking results."]
    pub object_annotations: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1ObjectTrackingAnnotation>,
        >,
    >,
    #[serde(rename = "shotAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shot annotation results. Each shot is represented as a video segment."]
    pub shot_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1VideoSegment>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotations related to one detected OCR text snippet. This will contain the corresponding text, confidence value, and frame level information for each detection."]
pub struct GoogleCloudVideointelligenceV1p3beta1TextAnnotation {
    #[serde(rename = "segments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All video segments where OCR detected text appears."]
    pub segments: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1TextSegment>>,
    >,
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The detected text."]
    pub text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Feature version."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video frame level annotation results for text annotation (OCR). Contains information regarding timestamp and bounding box locations for the frames containing detected OCR text snippets."]
pub struct GoogleCloudVideointelligenceV1p3beta1TextFrame {
    #[serde(rename = "rotatedBoundingBox")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Bounding polygon of the detected text for this frame."]
    pub rotated_bounding_box: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1NormalizedBoundingPoly>,
    >,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp of this frame."]
    pub time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video segment level annotation results for text detection."]
pub struct GoogleCloudVideointelligenceV1p3beta1TextSegment {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Confidence for the track of detected text. It is calculated as the highest over all frames where OCR detected text appears."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "frames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information related to the frames where OCR detected text appears."]
    pub frames: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1TextFrame>>,
    >,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video segment where a text snippet was detected."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1VideoSegment>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "For tracking related features. An object at time_offset with attributes, and located with normalized_bounding_box."]
pub struct GoogleCloudVideointelligenceV1p3beta1TimestampedObject {
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The attributes of the object in the bounding box."]
    pub attributes: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1DetectedAttribute>>,
    >,
    #[serde(rename = "landmarks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The detected landmarks."]
    pub landmarks: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1DetectedLandmark>>,
    >,
    #[serde(rename = "normalizedBoundingBox")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Normalized Bounding box in a frame, where the object is located."]
    pub normalized_bounding_box: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1NormalizedBoundingBox>,
    >,
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this object."]
    pub time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A track of an object instance."]
pub struct GoogleCloudVideointelligenceV1p3beta1Track {
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Attributes in the track level."]
    pub attributes: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1DetectedAttribute>>,
    >,
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The confidence score of the tracked object."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video segment of a track."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1VideoSegment>>,
    #[serde(rename = "timestampedObjects")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The object with timestamp and attributes per frame in the track."]
    pub timestamped_objects: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1TimestampedObject>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotation progress for a single video."]
pub struct GoogleCloudVideointelligenceV1p3beta1VideoAnnotationProgress {
    #[serde(rename = "feature")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies which feature is being tracked if the request contains more than one feature."]
    pub feature: ::std::option::Option<
        GoogleCloudVideointelligenceV1p3beta1VideoAnnotationProgressFeatureEnum,
    >,
    #[serde(rename = "inputUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
    pub input_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progressPercent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Approximate percentage processed thus far. Guaranteed to be 100 when fully processed."]
    pub progress_percent: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies which segment is being tracked if the request contains more than one segment."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1VideoSegment>>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time when the request was received."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time of the most recent update."]
    pub update_time: ::std::option::Option<::std::string::String>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Annotation results for a single video."]
pub struct GoogleCloudVideointelligenceV1p3beta1VideoAnnotationResults {
    #[serde(rename = "celebrityRecognitionAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Celebrity recognition annotations."]
    pub celebrity_recognition_annotations: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1CelebrityRecognitionAnnotation>,
    >,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, indicates an error. Note that for a single `AnnotateVideoRequest` some videos may succeed and some may fail."]
    pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
    #[serde(rename = "explicitAnnotation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Explicit content annotation."]
    pub explicit_annotation: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1ExplicitContentAnnotation>,
    >,
    #[serde(rename = "faceAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Please use `face_detection_annotations` instead."]
    pub face_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1FaceAnnotation>>,
    >,
    #[serde(rename = "faceDetectionAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Face detection annotations."]
    pub face_detection_annotations: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1FaceDetectionAnnotation>,
        >,
    >,
    #[serde(rename = "frameLabelAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Label annotations on frame level. There is exactly one element for each unique label."]
    pub frame_label_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1LabelAnnotation>>,
    >,
    #[serde(rename = "inputUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
    pub input_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "logoRecognitionAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Annotations for list of logos detected, tracked and recognized in video."]
    pub logo_recognition_annotations: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1LogoRecognitionAnnotation>,
        >,
    >,
    #[serde(rename = "objectAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Annotations for list of objects detected and tracked in video."]
    pub object_annotations: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1ObjectTrackingAnnotation>,
        >,
    >,
    #[serde(rename = "personDetectionAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Person detection annotations."]
    pub person_detection_annotations: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1PersonDetectionAnnotation>,
        >,
    >,
    #[serde(rename = "segment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video segment on which the annotation is run."]
    pub segment:
        ::std::option::Option<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1VideoSegment>>,
    #[serde(rename = "segmentLabelAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Topical label annotations on video level or user-specified segment level. There is exactly one element for each unique label."]
    pub segment_label_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1LabelAnnotation>>,
    >,
    #[serde(rename = "segmentPresenceLabelAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Presence label annotations on video level or user-specified segment level. There is exactly one element for each unique label. Compared to the existing topical `segment_label_annotations`, this field presents more fine-grained, segment-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to \"builtin/latest\" in the request."]
    pub segment_presence_label_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1LabelAnnotation>>,
    >,
    #[serde(rename = "shotAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shot annotations. Each shot is represented as a video segment."]
    pub shot_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1VideoSegment>>,
    >,
    #[serde(rename = "shotLabelAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Topical label annotations on shot level. There is exactly one element for each unique label."]
    pub shot_label_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1LabelAnnotation>>,
    >,
    #[serde(rename = "shotPresenceLabelAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Presence label annotations on shot level. There is exactly one element for each unique label. Compared to the existing topical `shot_label_annotations`, this field presents more fine-grained, shot-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to \"builtin/latest\" in the request."]
    pub shot_presence_label_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1LabelAnnotation>>,
    >,
    #[serde(rename = "speechTranscriptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Speech transcription."]
    pub speech_transcriptions: ::std::option::Option<
        ::std::vec::Vec<
            ::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1SpeechTranscription>,
        >,
    >,
    #[serde(rename = "textAnnotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OCR text detection and tracking. Annotations for list of detected text snippets. Each will have list of frame information associated with it."]
    pub text_annotations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudVideointelligenceV1p3beta1TextAnnotation>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Video segment."]
pub struct GoogleCloudVideointelligenceV1p3beta1VideoSegment {
    #[serde(rename = "endTimeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the end of the segment (inclusive)."]
    pub end_time_offset: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTimeOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time-offset, relative to the beginning of the video, corresponding to the start of the segment (inclusive)."]
    pub start_time_offset: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Word-specific information for recognized words. Word information is only included in the response when certain request parameters are set, such as `enable_word_time_offsets`."]
pub struct GoogleCloudVideointelligenceV1p3beta1WordInfo {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time offset relative to the beginning of the audio, and corresponding to the end of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "speakerTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A distinct integer value is assigned for every speaker within the audio. This field specifies which one of those speakers was detected to have spoken this word. Value ranges from 1 up to diarization_speaker_count, and is only set if speaker diarization is enabled."]
    pub speaker_tag: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time offset relative to the beginning of the audio, and corresponding to the start of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "word")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The word corresponding to this set of information."]
    pub word: ::std::option::Option<::std::string::String>,
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
    pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
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
pub struct GoogleRpcStatus {
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
