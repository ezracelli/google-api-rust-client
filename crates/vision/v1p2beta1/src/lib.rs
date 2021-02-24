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
    #[doc = "Response to a single file annotation request. A file may contain one or more images, which individually have their own responses."]
    pub struct AnnotateFileResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, represents the error message for the failed request. The `responses` field will not be set in this case."]
        pub error: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about the file for which this response is generated."]
        pub input_config: ::std::option::Option<::std::boxed::Box<InputConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Individual responses to images found within the file. This field will be empty if the `error` field is set."]
        pub responses:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AnnotateImageResponse>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalPages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field gives the total number of pages in the file."]
        pub total_pages: ::std::option::Option<::std::primitive::i64>,
    }
    impl AnnotateFileResponse {
        pub fn builder() -> AnnotateFileResponseBuilder {
            AnnotateFileResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to an image annotation request."]
    pub struct AnnotateImageResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "context")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, contextual information is needed to understand where this image comes from."]
        pub context: ::std::option::Option<::std::boxed::Box<ImageAnnotationContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cropHintsAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, crop hints have completed successfully."]
        pub crop_hints_annotation: ::std::option::Option<::std::boxed::Box<CropHintsAnnotation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, represents the error message for the operation. Note that filled-in image annotations are guaranteed to be correct, even when `error` is set."]
        pub error: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "faceAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, face detection has completed successfully."]
        pub face_annotations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FaceAnnotation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullTextAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, text (OCR) detection or document (OCR) text detection has completed successfully. This annotation provides the structural hierarchy for the OCR detected text."]
        pub full_text_annotation: ::std::option::Option<::std::boxed::Box<TextAnnotation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imagePropertiesAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, image properties were extracted successfully."]
        pub image_properties_annotation: ::std::option::Option<::std::boxed::Box<ImageProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, label detection has completed successfully."]
        pub label_annotations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EntityAnnotation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "landmarkAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, landmark detection has completed successfully."]
        pub landmark_annotations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EntityAnnotation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localizedObjectAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, localized object detection has completed successfully. This will be sorted descending by confidence score."]
        pub localized_object_annotations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LocalizedObjectAnnotation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logoAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, logo detection has completed successfully."]
        pub logo_annotations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EntityAnnotation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productSearchResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, product search has completed successfully."]
        pub product_search_results: ::std::option::Option<::std::boxed::Box<ProductSearchResults>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "safeSearchAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, safe-search annotation has completed successfully."]
        pub safe_search_annotation: ::std::option::Option<::std::boxed::Box<SafeSearchAnnotation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, text (OCR) detection has completed successfully."]
        pub text_annotations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EntityAnnotation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webDetection")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, web detection has completed successfully."]
        pub web_detection: ::std::option::Option<::std::boxed::Box<WebDetection>>,
    }
    impl AnnotateImageResponse {
        pub fn builder() -> AnnotateImageResponseBuilder {
            AnnotateImageResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for a single offline file annotation request."]
    pub struct AsyncAnnotateFileResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The output location and metadata from AsyncAnnotateFileRequest."]
        pub output_config: ::std::option::Option<::std::boxed::Box<OutputConfig>>,
    }
    impl AsyncAnnotateFileResponse {
        pub fn builder() -> AsyncAnnotateFileResponseBuilder {
            AsyncAnnotateFileResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to an async batch file annotation request."]
    pub struct AsyncBatchAnnotateFilesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of file annotation responses, one for each request in AsyncBatchAnnotateFilesRequest."]
        pub responses:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AsyncAnnotateFileResponse>>>,
    }
    impl AsyncBatchAnnotateFilesResponse {
        pub fn builder() -> AsyncBatchAnnotateFilesResponseBuilder {
            AsyncBatchAnnotateFilesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to an async batch image annotation request."]
    pub struct AsyncBatchAnnotateImagesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The output location and metadata from AsyncBatchAnnotateImagesRequest."]
        pub output_config: ::std::option::Option<::std::boxed::Box<OutputConfig>>,
    }
    impl AsyncBatchAnnotateImagesResponse {
        pub fn builder() -> AsyncBatchAnnotateImagesResponseBuilder {
            AsyncBatchAnnotateImagesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of file annotation responses."]
    pub struct BatchAnnotateFilesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of file annotation responses, each response corresponding to each AnnotateFileRequest in BatchAnnotateFilesRequest."]
        pub responses:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AnnotateFileResponse>>>,
    }
    impl BatchAnnotateFilesResponse {
        pub fn builder() -> BatchAnnotateFilesResponseBuilder {
            BatchAnnotateFilesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for the batch operations such as the current state. This is included in the `metadata` field of the `Operation` returned by the `GetOperation` call of the `google::longrunning::Operations` service."]
    pub struct BatchOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when the batch request is finished and google.longrunning.Operation.done is set to true."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current state of the batch operation."]
        pub state: ::std::option::Option<BatchOperationMetadataStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "submitTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when the batch request was submitted to the server."]
        pub submit_time: ::std::option::Option<::std::string::String>,
    }
    impl BatchOperationMetadata {
        pub fn builder() -> BatchOperationMetadataBuilder {
            BatchOperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The current state of the batch operation."]
    pub enum BatchOperationMetadataStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Invalid."]
        StateUnspecified,
        #[serde(rename = "PROCESSING")]
        #[doc = "Request is actively being processed."]
        Processing,
        #[serde(rename = "SUCCESSFUL")]
        #[doc = "The request is done and at least one item has been successfully processed."]
        Successful,
        #[serde(rename = "FAILED")]
        #[doc = "The request is done and no item has been successfully processed."]
        Failed,
        #[serde(rename = "CANCELLED")]
        #[doc = "The request is done after the longrunning.Operations.CancelOperation has been called by the user. Any records that were processed before the cancel command are output as specified in the request."]
        Cancelled,
    }
    impl ::std::default::Default for BatchOperationMetadataStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Logical element on the page."]
    pub struct Block {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blockType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected block type (text, image etc) for this block."]
        pub block_type: ::std::option::Option<BlockBlockTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding box for the block. The vertices are in the order of top-left, top-right, bottom-right, bottom-left. When a rotation of the bounding box is detected the rotation is represented as around the top-left corner as defined when the text is read in the 'natural' orientation. For example: * when the text is horizontal it might look like: 0----1 | | 3----2 * when it's rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3)."]
        pub bounding_box: ::std::option::Option<::std::boxed::Box<BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the OCR results on the block. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paragraphs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of paragraphs in this block (if this blocks is of type text)."]
        pub paragraphs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Paragraph>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information detected for the block."]
        pub property: ::std::option::Option<::std::boxed::Box<TextProperty>>,
    }
    impl Block {
        pub fn builder() -> BlockBuilder {
            BlockBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Detected block type (text, image etc) for this block."]
    pub enum BlockBlockTypeEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown block type."]
        Unknown,
        #[serde(rename = "TEXT")]
        #[doc = "Regular text block."]
        Text,
        #[serde(rename = "TABLE")]
        #[doc = "Table block."]
        Table,
        #[serde(rename = "PICTURE")]
        #[doc = "Image block."]
        Picture,
        #[serde(rename = "RULER")]
        #[doc = "Horizontal/vertical line box."]
        Ruler,
        #[serde(rename = "BARCODE")]
        #[doc = "Barcode block."]
        Barcode,
    }
    impl ::std::default::Default for BlockBlockTypeEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A bounding polygon for the detected image annotation."]
    pub struct BoundingPoly {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedVertices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon normalized vertices."]
        pub normalized_vertices:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NormalizedVertex>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vertices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon vertices."]
        pub vertices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Vertex>>>,
    }
    impl BoundingPoly {
        pub fn builder() -> BoundingPolyBuilder {
            BoundingPolyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a color in the RGBA color space. This representation is designed for simplicity of conversion to/from color representations in various languages over compactness; for example, the fields of this representation can be trivially provided to the constructor of \"java.awt.Color\" in Java; it can also be trivially provided to UIColor's \"+colorWithRed:green:blue:alpha\" method in iOS; and, with just a little work, it can be easily formatted into a CSS \"rgba()\" string in JavaScript, as well. Note: this proto does not carry information about the absolute color space that should be used to interpret the RGB value (e.g. sRGB, Adobe RGB, DCI-P3, BT.2020, etc.). By default, applications SHOULD assume the sRGB color space. Note: when color equality needs to be decided, implementations, unless documented otherwise, will treat two colors to be equal if all their red, green, blue and alpha values each differ by at most 1e-5. Example (Java): import com.google.type.Color; // ... public static java.awt.Color fromProto(Color protocolor) { float alpha = protocolor.hasAlpha() ? protocolor.getAlpha().getValue() : 1.0; return new java.awt.Color( protocolor.getRed(), protocolor.getGreen(), protocolor.getBlue(), alpha); } public static Color toProto(java.awt.Color color) { float red = (float) color.getRed(); float green = (float) color.getGreen(); float blue = (float) color.getBlue(); float denominator = 255.0; Color.Builder resultBuilder = Color .newBuilder() .setRed(red / denominator) .setGreen(green / denominator) .setBlue(blue / denominator); int alpha = color.getAlpha(); if (alpha != 255) { result.setAlpha( FloatValue .newBuilder() .setValue(((float) alpha) / denominator) .build()); } return resultBuilder.build(); } // ... Example (iOS / Obj-C): // ... static UIColor* fromProto(Color* protocolor) { float red = [protocolor red]; float green = [protocolor green]; float blue = [protocolor blue]; FloatValue* alpha_wrapper = [protocolor alpha]; float alpha = 1.0; if (alpha_wrapper != nil) { alpha = [alpha_wrapper value]; } return [UIColor colorWithRed:red green:green blue:blue alpha:alpha]; } static Color* toProto(UIColor* color) { CGFloat red, green, blue, alpha; if (![color getRed:&red green:&green blue:&blue alpha:&alpha]) { return nil; } Color* result = [[Color alloc] init]; [result setRed:red]; [result setGreen:green]; [result setBlue:blue]; if (alpha <= 0.9999) { [result setAlpha:floatWrapperWithValue(alpha)]; } [result autorelease]; return result; } // ... Example (JavaScript): // ... var protoToCssColor = function(rgb_color) { var redFrac = rgb_color.red || 0.0; var greenFrac = rgb_color.green || 0.0; var blueFrac = rgb_color.blue || 0.0; var red = Math.floor(redFrac * 255); var green = Math.floor(greenFrac * 255); var blue = Math.floor(blueFrac * 255); if (!('alpha' in rgb_color)) { return rgbToCssColor_(red, green, blue); } var alphaFrac = rgb_color.alpha.value || 0.0; var rgbParams = [red, green, blue].join(','); return ['rgba(', rgbParams, ',', alphaFrac, ')'].join(''); }; var rgbToCssColor_ = function(red, green, blue) { var rgbNumber = new Number((red << 16) | (green << 8) | blue); var hexString = rgbNumber.toString(16); var missingZeros = 6 - hexString.length; var resultBuilder = ['#']; for (var i = 0; i < missingZeros; i++) { resultBuilder.push('0'); } resultBuilder.push(hexString); return resultBuilder.join(''); }; // ..."]
    pub struct Color {
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
    impl Color {
        pub fn builder() -> ColorBuilder {
            ColorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Color information consists of RGB channels, score, and the fraction of the image that the color occupies in the image."]
    pub struct ColorInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "color")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "RGB components of the color."]
        pub color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pixelFraction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fraction of pixels the color occupies in the image. Value in range [0, 1]."]
        pub pixel_fraction: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image-specific score for this color. Value in range [0, 1]."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl ColorInfo {
        pub fn builder() -> ColorInfoBuilder {
            ColorInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Single crop hint that is used to generate a new crop when serving an image."]
    pub struct CropHint {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon for the crop region. The coordinates of the bounding box are in the original image's scale."]
        pub bounding_poly: ::std::option::Option<::std::boxed::Box<BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of this being a salient region. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importanceFraction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fraction of importance of this salient region with respect to the original image."]
        pub importance_fraction: ::std::option::Option<::std::primitive::f64>,
    }
    impl CropHint {
        pub fn builder() -> CropHintBuilder {
            CropHintBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set of crop hints that are used to generate new crops when serving images."]
    pub struct CropHintsAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cropHints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Crop hint results."]
        pub crop_hints: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CropHint>>>,
    }
    impl CropHintsAnnotation {
        pub fn builder() -> CropHintsAnnotationBuilder {
            CropHintsAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected start or end of a structural component."]
    pub struct DetectedBreak {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isPrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if break prepends the element."]
        pub is_prefix: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected break type."]
        pub _type: ::std::option::Option<DetectedBreakTypeEnum>,
    }
    impl DetectedBreak {
        pub fn builder() -> DetectedBreakBuilder {
            DetectedBreakBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Detected break type."]
    pub enum DetectedBreakTypeEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown break label type."]
        Unknown,
        #[serde(rename = "SPACE")]
        #[doc = "Regular space."]
        Space,
        #[serde(rename = "SURE_SPACE")]
        #[doc = "Sure space (very wide)."]
        SureSpace,
        #[serde(rename = "EOL_SURE_SPACE")]
        #[doc = "Line-wrapping break."]
        EolSureSpace,
        #[serde(rename = "HYPHEN")]
        #[doc = "End-line hyphen that is not present in text; does not co-occur with `SPACE`, `LEADER_SPACE`, or `LINE_BREAK`."]
        Hyphen,
        #[serde(rename = "LINE_BREAK")]
        #[doc = "Line break that ends a paragraph."]
        LineBreak,
    }
    impl ::std::default::Default for DetectedBreakTypeEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected language for a structural component."]
    pub struct DetectedLanguage {
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
    impl DetectedLanguage {
        pub fn builder() -> DetectedLanguageBuilder {
            DetectedLanguageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set of dominant colors and their corresponding scores."]
    pub struct DominantColorsAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "RGB color values with their score and pixel fraction."]
        pub colors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ColorInfo>>>,
    }
    impl DominantColorsAnnotation {
        pub fn builder() -> DominantColorsAnnotationBuilder {
            DominantColorsAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set of detected entity features."]
    pub struct EntityAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image region to which this entity belongs. Not produced for `LABEL_DETECTION` features."]
        pub bounding_poly: ::std::option::Option<::std::boxed::Box<BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "**Deprecated. Use `score` instead.** The accuracy of the entity detection in an image. For example, for an image in which the \"Eiffel Tower\" entity is detected, this field represents the confidence that there is a tower in the query image. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entity textual description, expressed in its `locale` language."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locale")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language code for the locale in which the entity textual `description` is expressed."]
        pub locale: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location information for the detected entity. Multiple `LocationInfo` elements can be present because one location may indicate the location of the scene in the image, and another location may indicate the location of the place where the image was taken. Location information is usually present for landmarks."]
        pub locations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LocationInfo>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Opaque entity ID. Some IDs may be available in [Google Knowledge Graph Search API](https://developers.google.com/knowledge-graph/)."]
        pub mid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Some entities may have optional user-supplied `Property` (name/value) fields, such a score or string that qualifies the entity."]
        pub properties: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Property>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Overall score of the result. Range [0, 1]."]
        pub score: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topicality")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The relevancy of the ICA (Image Content Annotation) label to the image. For example, the relevancy of \"tower\" is likely higher to an image containing the detected \"Eiffel Tower\" than to an image containing a detected distant towering building, even though the confidence that there is a tower in each image may be the same. Range [0, 1]."]
        pub topicality: ::std::option::Option<::std::primitive::f64>,
    }
    impl EntityAnnotation {
        pub fn builder() -> EntityAnnotationBuilder {
            EntityAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A face annotation object contains the results of face detection."]
    pub struct FaceAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "angerLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Anger likelihood."]
        pub anger_likelihood: ::std::option::Option<FaceAnnotationAngerLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blurredLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Blurred likelihood."]
        pub blurred_likelihood: ::std::option::Option<FaceAnnotationBlurredLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon around the face. The coordinates of the bounding box are in the original image's scale. The bounding box is computed to \"frame\" the face in accordance with human expectations. It is based on the landmarker results. Note that one or more x and/or y coordinates may not be generated in the `BoundingPoly` (the polygon will be unbounded) if only a partial face appears in the image to be annotated."]
        pub bounding_poly: ::std::option::Option<::std::boxed::Box<BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectionConfidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detection confidence. Range [0, 1]."]
        pub detection_confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fdBoundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `fd_bounding_poly` bounding polygon is tighter than the `boundingPoly`, and encloses only the skin part of the face. Typically, it is used to eliminate the face from any image analysis that detects the \"amount of skin\" visible in an image. It is not based on the landmarker results, only on the initial face detection, hence the fd (face detection) prefix."]
        pub fd_bounding_poly: ::std::option::Option<::std::boxed::Box<BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headwearLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Headwear likelihood."]
        pub headwear_likelihood: ::std::option::Option<FaceAnnotationHeadwearLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "joyLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Joy likelihood."]
        pub joy_likelihood: ::std::option::Option<FaceAnnotationJoyLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "landmarkingConfidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Face landmarking confidence. Range [0, 1]."]
        pub landmarking_confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "landmarks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected face landmarks."]
        pub landmarks: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Landmark>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "panAngle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Yaw angle, which indicates the leftward/rightward angle that the face is pointing relative to the vertical plane perpendicular to the image. Range [-180,180]."]
        pub pan_angle: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rollAngle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Roll angle, which indicates the amount of clockwise/anti-clockwise rotation of the face relative to the image vertical about the axis perpendicular to the face. Range [-180,180]."]
        pub roll_angle: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sorrowLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sorrow likelihood."]
        pub sorrow_likelihood: ::std::option::Option<FaceAnnotationSorrowLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "surpriseLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Surprise likelihood."]
        pub surprise_likelihood: ::std::option::Option<FaceAnnotationSurpriseLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tiltAngle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pitch angle, which indicates the upwards/downwards angle that the face is pointing relative to the image's horizontal plane. Range [-180,180]."]
        pub tilt_angle: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "underExposedLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Under-exposed likelihood."]
        pub under_exposed_likelihood:
            ::std::option::Option<FaceAnnotationUnderExposedLikelihoodEnum>,
    }
    impl FaceAnnotation {
        pub fn builder() -> FaceAnnotationBuilder {
            FaceAnnotationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Anger likelihood."]
    pub enum FaceAnnotationAngerLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for FaceAnnotationAngerLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Blurred likelihood."]
    pub enum FaceAnnotationBlurredLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for FaceAnnotationBlurredLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Headwear likelihood."]
    pub enum FaceAnnotationHeadwearLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for FaceAnnotationHeadwearLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Joy likelihood."]
    pub enum FaceAnnotationJoyLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for FaceAnnotationJoyLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Sorrow likelihood."]
    pub enum FaceAnnotationSorrowLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for FaceAnnotationSorrowLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Surprise likelihood."]
    pub enum FaceAnnotationSurpriseLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for FaceAnnotationSurpriseLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Under-exposed likelihood."]
    pub enum FaceAnnotationUnderExposedLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for FaceAnnotationUnderExposedLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Google Cloud Storage location where the output will be written to."]
    pub struct GcsDestination {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage URI prefix where the results will be stored. Results will be in JSON format and preceded by its corresponding input URI prefix. This field can either represent a gcs file prefix or gcs directory. In either case, the uri should be unique because in order to get all of the output files, you will need to do a wildcard gcs search on the uri prefix you provide. Examples: * File Prefix: gs://bucket-name/here/filenameprefix The output files will be created in gs://bucket-name/here/ and the names of the output files will begin with \"filenameprefix\". * Directory Prefix: gs://bucket-name/some/location/ The output files will be created in gs://bucket-name/some/location/ and the names of the output files could be anything because there was no filename prefix specified. If multiple outputs, each response is still AnnotateFileResponse, each of which contains some subset of the full list of AnnotateImageResponse. Multiple outputs can happen if, for example, the output JSON is too large and overflows into multiple sharded files."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GcsDestination {
        pub fn builder() -> GcsDestinationBuilder {
            GcsDestinationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Google Cloud Storage location where the input will be read from."]
    pub struct GcsSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage URI for the input file. This must only be a Google Cloud Storage object. Wildcards are not currently supported."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GcsSource {
        pub fn builder() -> GcsSourceBuilder {
            GcsSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to a single file annotation request. A file may contain one or more images, which individually have their own responses."]
    pub struct GoogleCloudVisionV1p1beta1AnnotateFileResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, represents the error message for the failed request. The `responses` field will not be set in this case."]
        pub error: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about the file for which this response is generated."]
        pub input_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p1beta1InputConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Individual responses to images found within the file. This field will be empty if the `error` field is set."]
        pub responses: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1AnnotateImageResponse>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalPages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field gives the total number of pages in the file."]
        pub total_pages: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudVisionV1p1beta1AnnotateFileResponse {
        pub fn builder() -> GoogleCloudVisionV1p1beta1AnnotateFileResponseBuilder {
            GoogleCloudVisionV1p1beta1AnnotateFileResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to an image annotation request."]
    pub struct GoogleCloudVisionV1p1beta1AnnotateImageResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "context")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, contextual information is needed to understand where this image comes from."]
        pub context: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p1beta1ImageAnnotationContext>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cropHintsAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, crop hints have completed successfully."]
        pub crop_hints_annotation:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p1beta1CropHintsAnnotation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, represents the error message for the operation. Note that filled-in image annotations are guaranteed to be correct, even when `error` is set."]
        pub error: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "faceAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, face detection has completed successfully."]
        pub face_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1FaceAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullTextAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, text (OCR) detection or document (OCR) text detection has completed successfully. This annotation provides the structural hierarchy for the OCR detected text."]
        pub full_text_annotation:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p1beta1TextAnnotation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imagePropertiesAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, image properties were extracted successfully."]
        pub image_properties_annotation:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p1beta1ImageProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, label detection has completed successfully."]
        pub label_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1EntityAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "landmarkAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, landmark detection has completed successfully."]
        pub landmark_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1EntityAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localizedObjectAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, localized object detection has completed successfully. This will be sorted descending by confidence score."]
        pub localized_object_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1LocalizedObjectAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logoAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, logo detection has completed successfully."]
        pub logo_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1EntityAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productSearchResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, product search has completed successfully."]
        pub product_search_results: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p1beta1ProductSearchResults>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "safeSearchAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, safe-search annotation has completed successfully."]
        pub safe_search_annotation: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p1beta1SafeSearchAnnotation>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, text (OCR) detection has completed successfully."]
        pub text_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1EntityAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webDetection")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, web detection has completed successfully."]
        pub web_detection:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p1beta1WebDetection>>,
    }
    impl GoogleCloudVisionV1p1beta1AnnotateImageResponse {
        pub fn builder() -> GoogleCloudVisionV1p1beta1AnnotateImageResponseBuilder {
            GoogleCloudVisionV1p1beta1AnnotateImageResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for a single offline file annotation request."]
    pub struct GoogleCloudVisionV1p1beta1AsyncAnnotateFileResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The output location and metadata from AsyncAnnotateFileRequest."]
        pub output_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p1beta1OutputConfig>>,
    }
    impl GoogleCloudVisionV1p1beta1AsyncAnnotateFileResponse {
        pub fn builder() -> GoogleCloudVisionV1p1beta1AsyncAnnotateFileResponseBuilder {
            GoogleCloudVisionV1p1beta1AsyncAnnotateFileResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to an async batch file annotation request."]
    pub struct GoogleCloudVisionV1p1beta1AsyncBatchAnnotateFilesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of file annotation responses, one for each request in AsyncBatchAnnotateFilesRequest."]
        pub responses: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1AsyncAnnotateFileResponse>>,
        >,
    }
    impl GoogleCloudVisionV1p1beta1AsyncBatchAnnotateFilesResponse {
        pub fn builder() -> GoogleCloudVisionV1p1beta1AsyncBatchAnnotateFilesResponseBuilder {
            GoogleCloudVisionV1p1beta1AsyncBatchAnnotateFilesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Logical element on the page."]
    pub struct GoogleCloudVisionV1p1beta1Block {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blockType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected block type (text, image etc) for this block."]
        pub block_type: ::std::option::Option<GoogleCloudVisionV1p1beta1BlockBlockTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding box for the block. The vertices are in the order of top-left, top-right, bottom-right, bottom-left. When a rotation of the bounding box is detected the rotation is represented as around the top-left corner as defined when the text is read in the 'natural' orientation. For example: * when the text is horizontal it might look like: 0----1 | | 3----2 * when it's rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3)."]
        pub bounding_box:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p1beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the OCR results on the block. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paragraphs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of paragraphs in this block (if this blocks is of type text)."]
        pub paragraphs: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1Paragraph>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information detected for the block."]
        pub property: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p1beta1TextAnnotationTextProperty>,
        >,
    }
    impl GoogleCloudVisionV1p1beta1Block {
        pub fn builder() -> GoogleCloudVisionV1p1beta1BlockBuilder {
            GoogleCloudVisionV1p1beta1BlockBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Detected block type (text, image etc) for this block."]
    pub enum GoogleCloudVisionV1p1beta1BlockBlockTypeEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown block type."]
        Unknown,
        #[serde(rename = "TEXT")]
        #[doc = "Regular text block."]
        Text,
        #[serde(rename = "TABLE")]
        #[doc = "Table block."]
        Table,
        #[serde(rename = "PICTURE")]
        #[doc = "Image block."]
        Picture,
        #[serde(rename = "RULER")]
        #[doc = "Horizontal/vertical line box."]
        Ruler,
        #[serde(rename = "BARCODE")]
        #[doc = "Barcode block."]
        Barcode,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p1beta1BlockBlockTypeEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A bounding polygon for the detected image annotation."]
    pub struct GoogleCloudVisionV1p1beta1BoundingPoly {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedVertices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon normalized vertices."]
        pub normalized_vertices: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1NormalizedVertex>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vertices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon vertices."]
        pub vertices: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1Vertex>>,
        >,
    }
    impl GoogleCloudVisionV1p1beta1BoundingPoly {
        pub fn builder() -> GoogleCloudVisionV1p1beta1BoundingPolyBuilder {
            GoogleCloudVisionV1p1beta1BoundingPolyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Color information consists of RGB channels, score, and the fraction of the image that the color occupies in the image."]
    pub struct GoogleCloudVisionV1p1beta1ColorInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "color")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "RGB components of the color."]
        pub color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pixelFraction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fraction of pixels the color occupies in the image. Value in range [0, 1]."]
        pub pixel_fraction: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image-specific score for this color. Value in range [0, 1]."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p1beta1ColorInfo {
        pub fn builder() -> GoogleCloudVisionV1p1beta1ColorInfoBuilder {
            GoogleCloudVisionV1p1beta1ColorInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Single crop hint that is used to generate a new crop when serving an image."]
    pub struct GoogleCloudVisionV1p1beta1CropHint {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon for the crop region. The coordinates of the bounding box are in the original image's scale."]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p1beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of this being a salient region. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importanceFraction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fraction of importance of this salient region with respect to the original image."]
        pub importance_fraction: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p1beta1CropHint {
        pub fn builder() -> GoogleCloudVisionV1p1beta1CropHintBuilder {
            GoogleCloudVisionV1p1beta1CropHintBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set of crop hints that are used to generate new crops when serving images."]
    pub struct GoogleCloudVisionV1p1beta1CropHintsAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cropHints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Crop hint results."]
        pub crop_hints: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1CropHint>>,
        >,
    }
    impl GoogleCloudVisionV1p1beta1CropHintsAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p1beta1CropHintsAnnotationBuilder {
            GoogleCloudVisionV1p1beta1CropHintsAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set of dominant colors and their corresponding scores."]
    pub struct GoogleCloudVisionV1p1beta1DominantColorsAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "RGB color values with their score and pixel fraction."]
        pub colors: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1ColorInfo>>,
        >,
    }
    impl GoogleCloudVisionV1p1beta1DominantColorsAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p1beta1DominantColorsAnnotationBuilder {
            GoogleCloudVisionV1p1beta1DominantColorsAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set of detected entity features."]
    pub struct GoogleCloudVisionV1p1beta1EntityAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image region to which this entity belongs. Not produced for `LABEL_DETECTION` features."]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p1beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "**Deprecated. Use `score` instead.** The accuracy of the entity detection in an image. For example, for an image in which the \"Eiffel Tower\" entity is detected, this field represents the confidence that there is a tower in the query image. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entity textual description, expressed in its `locale` language."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locale")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language code for the locale in which the entity textual `description` is expressed."]
        pub locale: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location information for the detected entity. Multiple `LocationInfo` elements can be present because one location may indicate the location of the scene in the image, and another location may indicate the location of the place where the image was taken. Location information is usually present for landmarks."]
        pub locations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1LocationInfo>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Opaque entity ID. Some IDs may be available in [Google Knowledge Graph Search API](https://developers.google.com/knowledge-graph/)."]
        pub mid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Some entities may have optional user-supplied `Property` (name/value) fields, such a score or string that qualifies the entity."]
        pub properties: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1Property>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Overall score of the result. Range [0, 1]."]
        pub score: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topicality")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The relevancy of the ICA (Image Content Annotation) label to the image. For example, the relevancy of \"tower\" is likely higher to an image containing the detected \"Eiffel Tower\" than to an image containing a detected distant towering building, even though the confidence that there is a tower in each image may be the same. Range [0, 1]."]
        pub topicality: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p1beta1EntityAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p1beta1EntityAnnotationBuilder {
            GoogleCloudVisionV1p1beta1EntityAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A face annotation object contains the results of face detection."]
    pub struct GoogleCloudVisionV1p1beta1FaceAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "angerLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Anger likelihood."]
        pub anger_likelihood:
            ::std::option::Option<GoogleCloudVisionV1p1beta1FaceAnnotationAngerLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blurredLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Blurred likelihood."]
        pub blurred_likelihood:
            ::std::option::Option<GoogleCloudVisionV1p1beta1FaceAnnotationBlurredLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon around the face. The coordinates of the bounding box are in the original image's scale. The bounding box is computed to \"frame\" the face in accordance with human expectations. It is based on the landmarker results. Note that one or more x and/or y coordinates may not be generated in the `BoundingPoly` (the polygon will be unbounded) if only a partial face appears in the image to be annotated."]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p1beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectionConfidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detection confidence. Range [0, 1]."]
        pub detection_confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fdBoundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `fd_bounding_poly` bounding polygon is tighter than the `boundingPoly`, and encloses only the skin part of the face. Typically, it is used to eliminate the face from any image analysis that detects the \"amount of skin\" visible in an image. It is not based on the landmarker results, only on the initial face detection, hence the fd (face detection) prefix."]
        pub fd_bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p1beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headwearLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Headwear likelihood."]
        pub headwear_likelihood:
            ::std::option::Option<GoogleCloudVisionV1p1beta1FaceAnnotationHeadwearLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "joyLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Joy likelihood."]
        pub joy_likelihood:
            ::std::option::Option<GoogleCloudVisionV1p1beta1FaceAnnotationJoyLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "landmarkingConfidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Face landmarking confidence. Range [0, 1]."]
        pub landmarking_confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "landmarks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected face landmarks."]
        pub landmarks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1FaceAnnotationLandmark>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "panAngle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Yaw angle, which indicates the leftward/rightward angle that the face is pointing relative to the vertical plane perpendicular to the image. Range [-180,180]."]
        pub pan_angle: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rollAngle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Roll angle, which indicates the amount of clockwise/anti-clockwise rotation of the face relative to the image vertical about the axis perpendicular to the face. Range [-180,180]."]
        pub roll_angle: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sorrowLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sorrow likelihood."]
        pub sorrow_likelihood:
            ::std::option::Option<GoogleCloudVisionV1p1beta1FaceAnnotationSorrowLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "surpriseLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Surprise likelihood."]
        pub surprise_likelihood:
            ::std::option::Option<GoogleCloudVisionV1p1beta1FaceAnnotationSurpriseLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tiltAngle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pitch angle, which indicates the upwards/downwards angle that the face is pointing relative to the image's horizontal plane. Range [-180,180]."]
        pub tilt_angle: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "underExposedLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Under-exposed likelihood."]
        pub under_exposed_likelihood: ::std::option::Option<
            GoogleCloudVisionV1p1beta1FaceAnnotationUnderExposedLikelihoodEnum,
        >,
    }
    impl GoogleCloudVisionV1p1beta1FaceAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p1beta1FaceAnnotationBuilder {
            GoogleCloudVisionV1p1beta1FaceAnnotationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Anger likelihood."]
    pub enum GoogleCloudVisionV1p1beta1FaceAnnotationAngerLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p1beta1FaceAnnotationAngerLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Blurred likelihood."]
    pub enum GoogleCloudVisionV1p1beta1FaceAnnotationBlurredLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p1beta1FaceAnnotationBlurredLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Headwear likelihood."]
    pub enum GoogleCloudVisionV1p1beta1FaceAnnotationHeadwearLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p1beta1FaceAnnotationHeadwearLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Joy likelihood."]
    pub enum GoogleCloudVisionV1p1beta1FaceAnnotationJoyLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p1beta1FaceAnnotationJoyLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Sorrow likelihood."]
    pub enum GoogleCloudVisionV1p1beta1FaceAnnotationSorrowLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p1beta1FaceAnnotationSorrowLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Surprise likelihood."]
    pub enum GoogleCloudVisionV1p1beta1FaceAnnotationSurpriseLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p1beta1FaceAnnotationSurpriseLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Under-exposed likelihood."]
    pub enum GoogleCloudVisionV1p1beta1FaceAnnotationUnderExposedLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default
        for GoogleCloudVisionV1p1beta1FaceAnnotationUnderExposedLikelihoodEnum
    {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A face-specific landmark (for example, a face feature)."]
    pub struct GoogleCloudVisionV1p1beta1FaceAnnotationLandmark {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "position")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Face landmark position."]
        pub position: ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p1beta1Position>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Face landmark type."]
        pub _type: ::std::option::Option<GoogleCloudVisionV1p1beta1FaceAnnotationLandmarkTypeEnum>,
    }
    impl GoogleCloudVisionV1p1beta1FaceAnnotationLandmark {
        pub fn builder() -> GoogleCloudVisionV1p1beta1FaceAnnotationLandmarkBuilder {
            GoogleCloudVisionV1p1beta1FaceAnnotationLandmarkBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Face landmark type."]
    pub enum GoogleCloudVisionV1p1beta1FaceAnnotationLandmarkTypeEnum {
        #[serde(rename = "UNKNOWN_LANDMARK")]
        #[doc = "Unknown face landmark detected. Should not be filled."]
        UnknownLandmark,
        #[serde(rename = "LEFT_EYE")]
        #[doc = "Left eye."]
        LeftEye,
        #[serde(rename = "RIGHT_EYE")]
        #[doc = "Right eye."]
        RightEye,
        #[serde(rename = "LEFT_OF_LEFT_EYEBROW")]
        #[doc = "Left of left eyebrow."]
        LeftOfLeftEyebrow,
        #[serde(rename = "RIGHT_OF_LEFT_EYEBROW")]
        #[doc = "Right of left eyebrow."]
        RightOfLeftEyebrow,
        #[serde(rename = "LEFT_OF_RIGHT_EYEBROW")]
        #[doc = "Left of right eyebrow."]
        LeftOfRightEyebrow,
        #[serde(rename = "RIGHT_OF_RIGHT_EYEBROW")]
        #[doc = "Right of right eyebrow."]
        RightOfRightEyebrow,
        #[serde(rename = "MIDPOINT_BETWEEN_EYES")]
        #[doc = "Midpoint between eyes."]
        MidpointBetweenEyes,
        #[serde(rename = "NOSE_TIP")]
        #[doc = "Nose tip."]
        NoseTip,
        #[serde(rename = "UPPER_LIP")]
        #[doc = "Upper lip."]
        UpperLip,
        #[serde(rename = "LOWER_LIP")]
        #[doc = "Lower lip."]
        LowerLip,
        #[serde(rename = "MOUTH_LEFT")]
        #[doc = "Mouth left."]
        MouthLeft,
        #[serde(rename = "MOUTH_RIGHT")]
        #[doc = "Mouth right."]
        MouthRight,
        #[serde(rename = "MOUTH_CENTER")]
        #[doc = "Mouth center."]
        MouthCenter,
        #[serde(rename = "NOSE_BOTTOM_RIGHT")]
        #[doc = "Nose, bottom right."]
        NoseBottomRight,
        #[serde(rename = "NOSE_BOTTOM_LEFT")]
        #[doc = "Nose, bottom left."]
        NoseBottomLeft,
        #[serde(rename = "NOSE_BOTTOM_CENTER")]
        #[doc = "Nose, bottom center."]
        NoseBottomCenter,
        #[serde(rename = "LEFT_EYE_TOP_BOUNDARY")]
        #[doc = "Left eye, top boundary."]
        LeftEyeTopBoundary,
        #[serde(rename = "LEFT_EYE_RIGHT_CORNER")]
        #[doc = "Left eye, right corner."]
        LeftEyeRightCorner,
        #[serde(rename = "LEFT_EYE_BOTTOM_BOUNDARY")]
        #[doc = "Left eye, bottom boundary."]
        LeftEyeBottomBoundary,
        #[serde(rename = "LEFT_EYE_LEFT_CORNER")]
        #[doc = "Left eye, left corner."]
        LeftEyeLeftCorner,
        #[serde(rename = "RIGHT_EYE_TOP_BOUNDARY")]
        #[doc = "Right eye, top boundary."]
        RightEyeTopBoundary,
        #[serde(rename = "RIGHT_EYE_RIGHT_CORNER")]
        #[doc = "Right eye, right corner."]
        RightEyeRightCorner,
        #[serde(rename = "RIGHT_EYE_BOTTOM_BOUNDARY")]
        #[doc = "Right eye, bottom boundary."]
        RightEyeBottomBoundary,
        #[serde(rename = "RIGHT_EYE_LEFT_CORNER")]
        #[doc = "Right eye, left corner."]
        RightEyeLeftCorner,
        #[serde(rename = "LEFT_EYEBROW_UPPER_MIDPOINT")]
        #[doc = "Left eyebrow, upper midpoint."]
        LeftEyebrowUpperMidpoint,
        #[serde(rename = "RIGHT_EYEBROW_UPPER_MIDPOINT")]
        #[doc = "Right eyebrow, upper midpoint."]
        RightEyebrowUpperMidpoint,
        #[serde(rename = "LEFT_EAR_TRAGION")]
        #[doc = "Left ear tragion."]
        LeftEarTragion,
        #[serde(rename = "RIGHT_EAR_TRAGION")]
        #[doc = "Right ear tragion."]
        RightEarTragion,
        #[serde(rename = "LEFT_EYE_PUPIL")]
        #[doc = "Left eye pupil."]
        LeftEyePupil,
        #[serde(rename = "RIGHT_EYE_PUPIL")]
        #[doc = "Right eye pupil."]
        RightEyePupil,
        #[serde(rename = "FOREHEAD_GLABELLA")]
        #[doc = "Forehead glabella."]
        ForeheadGlabella,
        #[serde(rename = "CHIN_GNATHION")]
        #[doc = "Chin gnathion."]
        ChinGnathion,
        #[serde(rename = "CHIN_LEFT_GONION")]
        #[doc = "Chin left gonion."]
        ChinLeftGonion,
        #[serde(rename = "CHIN_RIGHT_GONION")]
        #[doc = "Chin right gonion."]
        ChinRightGonion,
        #[serde(rename = "LEFT_CHEEK_CENTER")]
        #[doc = "Left cheek center."]
        LeftCheekCenter,
        #[serde(rename = "RIGHT_CHEEK_CENTER")]
        #[doc = "Right cheek center."]
        RightCheekCenter,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p1beta1FaceAnnotationLandmarkTypeEnum {
        fn default() -> Self {
            Self::UnknownLandmark
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Google Cloud Storage location where the output will be written to."]
    pub struct GoogleCloudVisionV1p1beta1GcsDestination {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage URI prefix where the results will be stored. Results will be in JSON format and preceded by its corresponding input URI prefix. This field can either represent a gcs file prefix or gcs directory. In either case, the uri should be unique because in order to get all of the output files, you will need to do a wildcard gcs search on the uri prefix you provide. Examples: * File Prefix: gs://bucket-name/here/filenameprefix The output files will be created in gs://bucket-name/here/ and the names of the output files will begin with \"filenameprefix\". * Directory Prefix: gs://bucket-name/some/location/ The output files will be created in gs://bucket-name/some/location/ and the names of the output files could be anything because there was no filename prefix specified. If multiple outputs, each response is still AnnotateFileResponse, each of which contains some subset of the full list of AnnotateImageResponse. Multiple outputs can happen if, for example, the output JSON is too large and overflows into multiple sharded files."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p1beta1GcsDestination {
        pub fn builder() -> GoogleCloudVisionV1p1beta1GcsDestinationBuilder {
            GoogleCloudVisionV1p1beta1GcsDestinationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Google Cloud Storage location where the input will be read from."]
    pub struct GoogleCloudVisionV1p1beta1GcsSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage URI for the input file. This must only be a Google Cloud Storage object. Wildcards are not currently supported."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p1beta1GcsSource {
        pub fn builder() -> GoogleCloudVisionV1p1beta1GcsSourceBuilder {
            GoogleCloudVisionV1p1beta1GcsSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "If an image was produced from a file (e.g. a PDF), this message gives information about the source of that image."]
    pub struct GoogleCloudVisionV1p1beta1ImageAnnotationContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the file was a PDF or TIFF, this field gives the page number within the file used to produce the image."]
        pub page_number: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI of the file used to produce the image."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p1beta1ImageAnnotationContext {
        pub fn builder() -> GoogleCloudVisionV1p1beta1ImageAnnotationContextBuilder {
            GoogleCloudVisionV1p1beta1ImageAnnotationContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Stores image properties, such as dominant colors."]
    pub struct GoogleCloudVisionV1p1beta1ImageProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dominantColors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, dominant colors completed successfully."]
        pub dominant_colors: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p1beta1DominantColorsAnnotation>,
        >,
    }
    impl GoogleCloudVisionV1p1beta1ImageProperties {
        pub fn builder() -> GoogleCloudVisionV1p1beta1ImagePropertiesBuilder {
            GoogleCloudVisionV1p1beta1ImagePropertiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The desired input location and metadata."]
    pub struct GoogleCloudVisionV1p1beta1InputConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "File content, represented as a stream of bytes. Note: As with all `bytes` fields, protobuffers use a pure binary representation, whereas JSON representations use base64. Currently, this field only works for BatchAnnotateFiles requests. It does not work for AsyncBatchAnnotateFiles requests."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Google Cloud Storage location to read the input from."]
        pub gcs_source:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p1beta1GcsSource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the file. Currently only \"application/pdf\", \"image/tiff\" and \"image/gif\" are supported. Wildcards are not supported."]
        pub mime_type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p1beta1InputConfig {
        pub fn builder() -> GoogleCloudVisionV1p1beta1InputConfigBuilder {
            GoogleCloudVisionV1p1beta1InputConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set of detected objects with bounding boxes."]
    pub struct GoogleCloudVisionV1p1beta1LocalizedObjectAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image region to which this object belongs. This must be populated."]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p1beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object ID that should align with EntityAnnotation mid."]
        pub mid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object name, expressed in its `language_code` language."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Score of the result. Range [0, 1]."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p1beta1LocalizedObjectAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p1beta1LocalizedObjectAnnotationBuilder {
            GoogleCloudVisionV1p1beta1LocalizedObjectAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected entity location information."]
    pub struct GoogleCloudVisionV1p1beta1LocationInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "latLng")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "lat/long location coordinates."]
        pub lat_lng: ::std::option::Option<::std::boxed::Box<LatLng>>,
    }
    impl GoogleCloudVisionV1p1beta1LocationInfo {
        pub fn builder() -> GoogleCloudVisionV1p1beta1LocationInfoBuilder {
            GoogleCloudVisionV1p1beta1LocationInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A vertex represents a 2D point in the image. NOTE: the normalized vertex coordinates are relative to the original image and range from 0 to 1."]
    pub struct GoogleCloudVisionV1p1beta1NormalizedVertex {
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
    impl GoogleCloudVisionV1p1beta1NormalizedVertex {
        pub fn builder() -> GoogleCloudVisionV1p1beta1NormalizedVertexBuilder {
            GoogleCloudVisionV1p1beta1NormalizedVertexBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains metadata for the BatchAnnotateImages operation."]
    pub struct GoogleCloudVisionV1p1beta1OperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when the batch request was received."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Current state of the batch operation."]
        pub state: ::std::option::Option<GoogleCloudVisionV1p1beta1OperationMetadataStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when the operation result was last updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p1beta1OperationMetadata {
        pub fn builder() -> GoogleCloudVisionV1p1beta1OperationMetadataBuilder {
            GoogleCloudVisionV1p1beta1OperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Current state of the batch operation."]
    pub enum GoogleCloudVisionV1p1beta1OperationMetadataStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Invalid."]
        StateUnspecified,
        #[serde(rename = "CREATED")]
        #[doc = "Request is received."]
        Created,
        #[serde(rename = "RUNNING")]
        #[doc = "Request is actively being processed."]
        Running,
        #[serde(rename = "DONE")]
        #[doc = "The batch processing is done."]
        Done,
        #[serde(rename = "CANCELLED")]
        #[doc = "The batch processing was cancelled."]
        Cancelled,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p1beta1OperationMetadataStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The desired output location and metadata."]
    pub struct GoogleCloudVisionV1p1beta1OutputConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The max number of response protos to put into each output JSON file on Google Cloud Storage. The valid range is [1, 100]. If not specified, the default value is 20. For example, for one pdf file with 100 pages, 100 response protos will be generated. If `batch_size` = 20, then 5 json files each containing 20 response protos will be written under the prefix `gcs_destination`.`uri`. Currently, batch_size only applies to GcsDestination, with potential future support for other output configurations."]
        pub batch_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsDestination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Google Cloud Storage location to write the output(s) to."]
        pub gcs_destination:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p1beta1GcsDestination>>,
    }
    impl GoogleCloudVisionV1p1beta1OutputConfig {
        pub fn builder() -> GoogleCloudVisionV1p1beta1OutputConfigBuilder {
            GoogleCloudVisionV1p1beta1OutputConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected page from OCR."]
    pub struct GoogleCloudVisionV1p1beta1Page {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blocks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of blocks of text, images etc on this page."]
        pub blocks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1Block>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the OCR results on the page. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page height. For PDFs the unit is points. For images (including TIFFs) the unit is pixels."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information detected on the page."]
        pub property: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p1beta1TextAnnotationTextProperty>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page width. For PDFs the unit is points. For images (including TIFFs) the unit is pixels."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudVisionV1p1beta1Page {
        pub fn builder() -> GoogleCloudVisionV1p1beta1PageBuilder {
            GoogleCloudVisionV1p1beta1PageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Structural unit of text representing a number of words in certain order."]
    pub struct GoogleCloudVisionV1p1beta1Paragraph {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding box for the paragraph. The vertices are in the order of top-left, top-right, bottom-right, bottom-left. When a rotation of the bounding box is detected the rotation is represented as around the top-left corner as defined when the text is read in the 'natural' orientation. For example: * when the text is horizontal it might look like: 0----1 | | 3----2 * when it's rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3)."]
        pub bounding_box:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p1beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the OCR results for the paragraph. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information detected for the paragraph."]
        pub property: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p1beta1TextAnnotationTextProperty>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "words")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of all words in this paragraph."]
        pub words: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1Word>>,
        >,
    }
    impl GoogleCloudVisionV1p1beta1Paragraph {
        pub fn builder() -> GoogleCloudVisionV1p1beta1ParagraphBuilder {
            GoogleCloudVisionV1p1beta1ParagraphBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A 3D position in the image, used primarily for Face detection landmarks. A valid Position must have both x and y coordinates. The position coordinates are in the same scale as the original image."]
    pub struct GoogleCloudVisionV1p1beta1Position {
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
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "z")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Z coordinate (or depth)."]
        pub z: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p1beta1Position {
        pub fn builder() -> GoogleCloudVisionV1p1beta1PositionBuilder {
            GoogleCloudVisionV1p1beta1PositionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Product contains ReferenceImages."]
    pub struct GoogleCloudVisionV1p1beta1Product {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User-provided metadata to be stored with this product. Must be at most 4096 characters long."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user-provided name for this Product. Must not be empty. Must be at most 4096 characters long."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the product. Format is: `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`. This field is ignored when creating a product."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productCategory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Immutable. The category for the product identified by the reference image. This should be one of \"homegoods-v2\", \"apparel-v2\", \"toys-v2\", \"packagedgoods-v1\" or \"general-v1\". The legacy categories \"homegoods\", \"apparel\", and \"toys\" are still supported, but these should not be used for new products."]
        pub product_category: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Key-value pairs that can be attached to a product. At query time, constraints can be specified based on the product_labels. Note that integer values can be provided as strings, e.g. \"1199\". Only strings with integer values can match a range-based restriction which is to be supported soon. Multiple values can be assigned to the same key. One product may have up to 500 product_labels. Notice that the total number of distinct product_labels over all products in one ProductSet cannot exceed 1M, otherwise the product search pipeline will refuse to work for that ProductSet."]
        pub product_labels: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1ProductKeyValue>>,
        >,
    }
    impl GoogleCloudVisionV1p1beta1Product {
        pub fn builder() -> GoogleCloudVisionV1p1beta1ProductBuilder {
            GoogleCloudVisionV1p1beta1ProductBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A product label represented as a key-value pair."]
    pub struct GoogleCloudVisionV1p1beta1ProductKeyValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The key of the label attached to the product. Cannot be empty and cannot exceed 128 bytes."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value of the label attached to the product. Cannot be empty and cannot exceed 128 bytes."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p1beta1ProductKeyValue {
        pub fn builder() -> GoogleCloudVisionV1p1beta1ProductKeyValueBuilder {
            GoogleCloudVisionV1p1beta1ProductKeyValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Results for a product search request."]
    pub struct GoogleCloudVisionV1p1beta1ProductSearchResults {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "indexTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp of the index which provided these results. Products added to the product set and products removed from the product set after this time are not reflected in the current results."]
        pub index_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productGroupedResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of results grouped by products detected in the query image. Each entry corresponds to one bounding polygon in the query image, and contains the matching products specific to that region. There may be duplicate product matches in the union of all the per-product results."]
        pub product_grouped_results: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVisionV1p1beta1ProductSearchResultsGroupedResult>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of results, one for each product match."]
        pub results: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVisionV1p1beta1ProductSearchResultsResult>,
            >,
        >,
    }
    impl GoogleCloudVisionV1p1beta1ProductSearchResults {
        pub fn builder() -> GoogleCloudVisionV1p1beta1ProductSearchResultsBuilder {
            GoogleCloudVisionV1p1beta1ProductSearchResultsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about the products similar to a single product in a query image."]
    pub struct GoogleCloudVisionV1p1beta1ProductSearchResultsGroupedResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon around the product detected in the query image."]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p1beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of generic predictions for the object in the bounding box."]
        pub object_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVisionV1p1beta1ProductSearchResultsObjectAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of results, one for each product match."]
        pub results: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVisionV1p1beta1ProductSearchResultsResult>,
            >,
        >,
    }
    impl GoogleCloudVisionV1p1beta1ProductSearchResultsGroupedResult {
        pub fn builder() -> GoogleCloudVisionV1p1beta1ProductSearchResultsGroupedResultBuilder {
            GoogleCloudVisionV1p1beta1ProductSearchResultsGroupedResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Prediction for what the object in the bounding box is."]
    pub struct GoogleCloudVisionV1p1beta1ProductSearchResultsObjectAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object ID that should align with EntityAnnotation mid."]
        pub mid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object name, expressed in its `language_code` language."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Score of the result. Range [0, 1]."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p1beta1ProductSearchResultsObjectAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p1beta1ProductSearchResultsObjectAnnotationBuilder {
            GoogleCloudVisionV1p1beta1ProductSearchResultsObjectAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a product."]
    pub struct GoogleCloudVisionV1p1beta1ProductSearchResultsResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the image from the product that is the closest match to the query."]
        pub image: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "product")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Product."]
        pub product: ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p1beta1Product>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A confidence level on the match, ranging from 0 (no confidence) to 1 (full confidence)."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p1beta1ProductSearchResultsResult {
        pub fn builder() -> GoogleCloudVisionV1p1beta1ProductSearchResultsResultBuilder {
            GoogleCloudVisionV1p1beta1ProductSearchResultsResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A `Property` consists of a user-supplied name/value pair."]
    pub struct GoogleCloudVisionV1p1beta1Property {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the property."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uint64Value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value of numeric properties."]
        pub uint64_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value of the property."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p1beta1Property {
        pub fn builder() -> GoogleCloudVisionV1p1beta1PropertyBuilder {
            GoogleCloudVisionV1p1beta1PropertyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set of features pertaining to the image, computed by computer vision methods over safe-search verticals (for example, adult, spoof, medical, violence)."]
    pub struct GoogleCloudVisionV1p1beta1SafeSearchAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents the adult content likelihood for the image. Adult content may contain elements such as nudity, pornographic images or cartoons, or sexual activities."]
        pub adult: ::std::option::Option<GoogleCloudVisionV1p1beta1SafeSearchAnnotationAdultEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "medical")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Likelihood that this is a medical image."]
        pub medical:
            ::std::option::Option<GoogleCloudVisionV1p1beta1SafeSearchAnnotationMedicalEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "racy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Likelihood that the request image contains racy content. Racy content may include (but is not limited to) skimpy or sheer clothing, strategically covered nudity, lewd or provocative poses, or close-ups of sensitive body areas."]
        pub racy: ::std::option::Option<GoogleCloudVisionV1p1beta1SafeSearchAnnotationRacyEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spoof")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Spoof likelihood. The likelihood that an modification was made to the image's canonical version to make it appear funny or offensive."]
        pub spoof: ::std::option::Option<GoogleCloudVisionV1p1beta1SafeSearchAnnotationSpoofEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "violence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Likelihood that this image contains violent content."]
        pub violence:
            ::std::option::Option<GoogleCloudVisionV1p1beta1SafeSearchAnnotationViolenceEnum>,
    }
    impl GoogleCloudVisionV1p1beta1SafeSearchAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p1beta1SafeSearchAnnotationBuilder {
            GoogleCloudVisionV1p1beta1SafeSearchAnnotationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Represents the adult content likelihood for the image. Adult content may contain elements such as nudity, pornographic images or cartoons, or sexual activities."]
    pub enum GoogleCloudVisionV1p1beta1SafeSearchAnnotationAdultEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p1beta1SafeSearchAnnotationAdultEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Likelihood that this is a medical image."]
    pub enum GoogleCloudVisionV1p1beta1SafeSearchAnnotationMedicalEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p1beta1SafeSearchAnnotationMedicalEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Likelihood that the request image contains racy content. Racy content may include (but is not limited to) skimpy or sheer clothing, strategically covered nudity, lewd or provocative poses, or close-ups of sensitive body areas."]
    pub enum GoogleCloudVisionV1p1beta1SafeSearchAnnotationRacyEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p1beta1SafeSearchAnnotationRacyEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Spoof likelihood. The likelihood that an modification was made to the image's canonical version to make it appear funny or offensive."]
    pub enum GoogleCloudVisionV1p1beta1SafeSearchAnnotationSpoofEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p1beta1SafeSearchAnnotationSpoofEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Likelihood that this image contains violent content."]
    pub enum GoogleCloudVisionV1p1beta1SafeSearchAnnotationViolenceEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p1beta1SafeSearchAnnotationViolenceEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single symbol representation."]
    pub struct GoogleCloudVisionV1p1beta1Symbol {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding box for the symbol. The vertices are in the order of top-left, top-right, bottom-right, bottom-left. When a rotation of the bounding box is detected the rotation is represented as around the top-left corner as defined when the text is read in the 'natural' orientation. For example: * when the text is horizontal it might look like: 0----1 | | 3----2 * when it's rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3)."]
        pub bounding_box:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p1beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the OCR results for the symbol. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information detected for the symbol."]
        pub property: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p1beta1TextAnnotationTextProperty>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The actual UTF-8 representation of the symbol."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p1beta1Symbol {
        pub fn builder() -> GoogleCloudVisionV1p1beta1SymbolBuilder {
            GoogleCloudVisionV1p1beta1SymbolBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "TextAnnotation contains a structured representation of OCR extracted text. The hierarchy of an OCR extracted text structure is like this: TextAnnotation -> Page -> Block -> Paragraph -> Word -> Symbol Each structural component, starting from Page, may further have their own properties. Properties describe detected languages, breaks etc.. Please refer to the TextAnnotation.TextProperty message definition below for more detail."]
    pub struct GoogleCloudVisionV1p1beta1TextAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of pages detected by OCR."]
        pub pages: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1Page>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "UTF-8 text detected on the pages."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p1beta1TextAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p1beta1TextAnnotationBuilder {
            GoogleCloudVisionV1p1beta1TextAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected start or end of a structural component."]
    pub struct GoogleCloudVisionV1p1beta1TextAnnotationDetectedBreak {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isPrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if break prepends the element."]
        pub is_prefix: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected break type."]
        pub _type:
            ::std::option::Option<GoogleCloudVisionV1p1beta1TextAnnotationDetectedBreakTypeEnum>,
    }
    impl GoogleCloudVisionV1p1beta1TextAnnotationDetectedBreak {
        pub fn builder() -> GoogleCloudVisionV1p1beta1TextAnnotationDetectedBreakBuilder {
            GoogleCloudVisionV1p1beta1TextAnnotationDetectedBreakBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Detected break type."]
    pub enum GoogleCloudVisionV1p1beta1TextAnnotationDetectedBreakTypeEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown break label type."]
        Unknown,
        #[serde(rename = "SPACE")]
        #[doc = "Regular space."]
        Space,
        #[serde(rename = "SURE_SPACE")]
        #[doc = "Sure space (very wide)."]
        SureSpace,
        #[serde(rename = "EOL_SURE_SPACE")]
        #[doc = "Line-wrapping break."]
        EolSureSpace,
        #[serde(rename = "HYPHEN")]
        #[doc = "End-line hyphen that is not present in text; does not co-occur with `SPACE`, `LEADER_SPACE`, or `LINE_BREAK`."]
        Hyphen,
        #[serde(rename = "LINE_BREAK")]
        #[doc = "Line break that ends a paragraph."]
        LineBreak,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p1beta1TextAnnotationDetectedBreakTypeEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected language for a structural component."]
    pub struct GoogleCloudVisionV1p1beta1TextAnnotationDetectedLanguage {
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
    impl GoogleCloudVisionV1p1beta1TextAnnotationDetectedLanguage {
        pub fn builder() -> GoogleCloudVisionV1p1beta1TextAnnotationDetectedLanguageBuilder {
            GoogleCloudVisionV1p1beta1TextAnnotationDetectedLanguageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional information detected on the structural component."]
    pub struct GoogleCloudVisionV1p1beta1TextAnnotationTextProperty {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedBreak")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected start or end of a text segment."]
        pub detected_break: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p1beta1TextAnnotationDetectedBreak>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected languages together with confidence."]
        pub detected_languages: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVisionV1p1beta1TextAnnotationDetectedLanguage>,
            >,
        >,
    }
    impl GoogleCloudVisionV1p1beta1TextAnnotationTextProperty {
        pub fn builder() -> GoogleCloudVisionV1p1beta1TextAnnotationTextPropertyBuilder {
            GoogleCloudVisionV1p1beta1TextAnnotationTextPropertyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A vertex represents a 2D point in the image. NOTE: the vertex coordinates are in the same scale as the original image."]
    pub struct GoogleCloudVisionV1p1beta1Vertex {
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
    impl GoogleCloudVisionV1p1beta1Vertex {
        pub fn builder() -> GoogleCloudVisionV1p1beta1VertexBuilder {
            GoogleCloudVisionV1p1beta1VertexBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Relevant information for the image from the Internet."]
    pub struct GoogleCloudVisionV1p1beta1WebDetection {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bestGuessLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The service's best guess as to the topic of the request image. Inferred from similar images on the open web."]
        pub best_guess_labels: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1WebDetectionWebLabel>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullMatchingImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fully matching images from the Internet. Can include resized copies of the query image."]
        pub full_matching_images: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1WebDetectionWebImage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pagesWithMatchingImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web pages containing the matching images from the Internet."]
        pub pages_with_matching_images: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1WebDetectionWebPage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialMatchingImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Partial matching images from the Internet. Those images are similar enough to share some key-point features. For example an original image will likely have partial matching for its crops."]
        pub partial_matching_images: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1WebDetectionWebImage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visuallySimilarImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visually similar image results."]
        pub visually_similar_images: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1WebDetectionWebImage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webEntities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deduced entities from similar images on the Internet."]
        pub web_entities: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1WebDetectionWebEntity>>,
        >,
    }
    impl GoogleCloudVisionV1p1beta1WebDetection {
        pub fn builder() -> GoogleCloudVisionV1p1beta1WebDetectionBuilder {
            GoogleCloudVisionV1p1beta1WebDetectionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Entity deduced from similar images on the Internet."]
    pub struct GoogleCloudVisionV1p1beta1WebDetectionWebEntity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Canonical description of the entity, in English."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Opaque entity ID."]
        pub entity_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Overall relevancy score for the entity. Not normalized and not comparable across different image queries."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p1beta1WebDetectionWebEntity {
        pub fn builder() -> GoogleCloudVisionV1p1beta1WebDetectionWebEntityBuilder {
            GoogleCloudVisionV1p1beta1WebDetectionWebEntityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for online images."]
    pub struct GoogleCloudVisionV1p1beta1WebDetectionWebImage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "(Deprecated) Overall relevancy score for the image."]
        pub score: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result image URL."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p1beta1WebDetectionWebImage {
        pub fn builder() -> GoogleCloudVisionV1p1beta1WebDetectionWebImageBuilder {
            GoogleCloudVisionV1p1beta1WebDetectionWebImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Label to provide extra metadata for the web detection."]
    pub struct GoogleCloudVisionV1p1beta1WebDetectionWebLabel {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "label")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label for extra metadata."]
        pub label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The BCP-47 language code for `label`, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        pub language_code: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p1beta1WebDetectionWebLabel {
        pub fn builder() -> GoogleCloudVisionV1p1beta1WebDetectionWebLabelBuilder {
            GoogleCloudVisionV1p1beta1WebDetectionWebLabelBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for web pages."]
    pub struct GoogleCloudVisionV1p1beta1WebDetectionWebPage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullMatchingImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fully matching images on the page. Can include resized copies of the query image."]
        pub full_matching_images: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1WebDetectionWebImage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title for the web page, may contain HTML markups."]
        pub page_title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialMatchingImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Partial matching images on the page. Those images are similar enough to share some key-point features. For example an original image will likely have partial matching for its crops."]
        pub partial_matching_images: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1WebDetectionWebImage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "(Deprecated) Overall relevancy score for the web page."]
        pub score: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result web page URL."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p1beta1WebDetectionWebPage {
        pub fn builder() -> GoogleCloudVisionV1p1beta1WebDetectionWebPageBuilder {
            GoogleCloudVisionV1p1beta1WebDetectionWebPageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A word representation."]
    pub struct GoogleCloudVisionV1p1beta1Word {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding box for the word. The vertices are in the order of top-left, top-right, bottom-right, bottom-left. When a rotation of the bounding box is detected the rotation is represented as around the top-left corner as defined when the text is read in the 'natural' orientation. For example: * when the text is horizontal it might look like: 0----1 | | 3----2 * when it's rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3)."]
        pub bounding_box:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p1beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the OCR results for the word. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information detected for the word."]
        pub property: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p1beta1TextAnnotationTextProperty>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "symbols")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of symbols in the word. The order of the symbols follows the natural reading order."]
        pub symbols: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p1beta1Symbol>>,
        >,
    }
    impl GoogleCloudVisionV1p1beta1Word {
        pub fn builder() -> GoogleCloudVisionV1p1beta1WordBuilder {
            GoogleCloudVisionV1p1beta1WordBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to annotate one single file, e.g. a PDF, TIFF or GIF file."]
    pub struct GoogleCloudVisionV1p2beta1AnnotateFileRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "features")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Requested features."]
        pub features: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1Feature>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional context that may accompany the image(s) in the file."]
        pub image_context:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1ImageContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Information about the input file."]
        pub input_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1InputConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pages of the file to perform image annotation. Pages starts from 1, we assume the first page of the file is page 1. At most 5 pages are supported per request. Pages can be negative. Page 1 means the first page. Page 2 means the second page. Page -1 means the last page. Page -2 means the second to the last page. If the file is GIF instead of PDF or TIFF, page refers to GIF frames. If this field is empty, by default the service performs image annotation for the first 5 pages of the file."]
        pub pages: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    }
    impl GoogleCloudVisionV1p2beta1AnnotateFileRequest {
        pub fn builder() -> GoogleCloudVisionV1p2beta1AnnotateFileRequestBuilder {
            GoogleCloudVisionV1p2beta1AnnotateFileRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to a single file annotation request. A file may contain one or more images, which individually have their own responses."]
    pub struct GoogleCloudVisionV1p2beta1AnnotateFileResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, represents the error message for the failed request. The `responses` field will not be set in this case."]
        pub error: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about the file for which this response is generated."]
        pub input_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1InputConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Individual responses to images found within the file. This field will be empty if the `error` field is set."]
        pub responses: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1AnnotateImageResponse>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalPages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field gives the total number of pages in the file."]
        pub total_pages: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudVisionV1p2beta1AnnotateFileResponse {
        pub fn builder() -> GoogleCloudVisionV1p2beta1AnnotateFileResponseBuilder {
            GoogleCloudVisionV1p2beta1AnnotateFileResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for performing Google Cloud Vision API tasks over a user-provided image, with user-requested features, and with context information."]
    pub struct GoogleCloudVisionV1p2beta1AnnotateImageRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "features")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Requested features."]
        pub features: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1Feature>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The image to be processed."]
        pub image: ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1Image>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional context that may accompany the image."]
        pub image_context:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1ImageContext>>,
    }
    impl GoogleCloudVisionV1p2beta1AnnotateImageRequest {
        pub fn builder() -> GoogleCloudVisionV1p2beta1AnnotateImageRequestBuilder {
            GoogleCloudVisionV1p2beta1AnnotateImageRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to an image annotation request."]
    pub struct GoogleCloudVisionV1p2beta1AnnotateImageResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "context")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, contextual information is needed to understand where this image comes from."]
        pub context: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p2beta1ImageAnnotationContext>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cropHintsAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, crop hints have completed successfully."]
        pub crop_hints_annotation:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1CropHintsAnnotation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, represents the error message for the operation. Note that filled-in image annotations are guaranteed to be correct, even when `error` is set."]
        pub error: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "faceAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, face detection has completed successfully."]
        pub face_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1FaceAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullTextAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, text (OCR) detection or document (OCR) text detection has completed successfully. This annotation provides the structural hierarchy for the OCR detected text."]
        pub full_text_annotation:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1TextAnnotation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imagePropertiesAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, image properties were extracted successfully."]
        pub image_properties_annotation:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1ImageProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, label detection has completed successfully."]
        pub label_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1EntityAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "landmarkAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, landmark detection has completed successfully."]
        pub landmark_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1EntityAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localizedObjectAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, localized object detection has completed successfully. This will be sorted descending by confidence score."]
        pub localized_object_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1LocalizedObjectAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logoAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, logo detection has completed successfully."]
        pub logo_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1EntityAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productSearchResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, product search has completed successfully."]
        pub product_search_results: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p2beta1ProductSearchResults>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "safeSearchAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, safe-search annotation has completed successfully."]
        pub safe_search_annotation: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p2beta1SafeSearchAnnotation>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, text (OCR) detection has completed successfully."]
        pub text_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1EntityAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webDetection")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, web detection has completed successfully."]
        pub web_detection:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1WebDetection>>,
    }
    impl GoogleCloudVisionV1p2beta1AnnotateImageResponse {
        pub fn builder() -> GoogleCloudVisionV1p2beta1AnnotateImageResponseBuilder {
            GoogleCloudVisionV1p2beta1AnnotateImageResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An offline file annotation request."]
    pub struct GoogleCloudVisionV1p2beta1AsyncAnnotateFileRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "features")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Requested features."]
        pub features: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1Feature>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional context that may accompany the image(s) in the file."]
        pub image_context:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1ImageContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Information about the input file."]
        pub input_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1InputConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The desired output location and metadata (e.g. format)."]
        pub output_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1OutputConfig>>,
    }
    impl GoogleCloudVisionV1p2beta1AsyncAnnotateFileRequest {
        pub fn builder() -> GoogleCloudVisionV1p2beta1AsyncAnnotateFileRequestBuilder {
            GoogleCloudVisionV1p2beta1AsyncAnnotateFileRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for a single offline file annotation request."]
    pub struct GoogleCloudVisionV1p2beta1AsyncAnnotateFileResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The output location and metadata from AsyncAnnotateFileRequest."]
        pub output_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1OutputConfig>>,
    }
    impl GoogleCloudVisionV1p2beta1AsyncAnnotateFileResponse {
        pub fn builder() -> GoogleCloudVisionV1p2beta1AsyncAnnotateFileResponseBuilder {
            GoogleCloudVisionV1p2beta1AsyncAnnotateFileResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Multiple async file annotation requests are batched into a single service call."]
    pub struct GoogleCloudVisionV1p2beta1AsyncBatchAnnotateFilesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no parent is specified, a region will be chosen automatically. Supported location-ids: `us`: USA country only, `asia`: East asia areas, like Japan, Taiwan, `eu`: The European Union. Example: `projects/project-A/locations/eu`."]
        pub parent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requests")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Individual async file annotation requests for this batch."]
        pub requests: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1AsyncAnnotateFileRequest>>,
        >,
    }
    impl GoogleCloudVisionV1p2beta1AsyncBatchAnnotateFilesRequest {
        pub fn builder() -> GoogleCloudVisionV1p2beta1AsyncBatchAnnotateFilesRequestBuilder {
            GoogleCloudVisionV1p2beta1AsyncBatchAnnotateFilesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to an async batch file annotation request."]
    pub struct GoogleCloudVisionV1p2beta1AsyncBatchAnnotateFilesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of file annotation responses, one for each request in AsyncBatchAnnotateFilesRequest."]
        pub responses: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1AsyncAnnotateFileResponse>>,
        >,
    }
    impl GoogleCloudVisionV1p2beta1AsyncBatchAnnotateFilesResponse {
        pub fn builder() -> GoogleCloudVisionV1p2beta1AsyncBatchAnnotateFilesResponseBuilder {
            GoogleCloudVisionV1p2beta1AsyncBatchAnnotateFilesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for async image annotation for a list of images."]
    pub struct GoogleCloudVisionV1p2beta1AsyncBatchAnnotateImagesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The desired output location and metadata (e.g. format)."]
        pub output_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1OutputConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no parent is specified, a region will be chosen automatically. Supported location-ids: `us`: USA country only, `asia`: East asia areas, like Japan, Taiwan, `eu`: The European Union. Example: `projects/project-A/locations/eu`."]
        pub parent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requests")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Individual image annotation requests for this batch."]
        pub requests: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1AnnotateImageRequest>>,
        >,
    }
    impl GoogleCloudVisionV1p2beta1AsyncBatchAnnotateImagesRequest {
        pub fn builder() -> GoogleCloudVisionV1p2beta1AsyncBatchAnnotateImagesRequestBuilder {
            GoogleCloudVisionV1p2beta1AsyncBatchAnnotateImagesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of requests to annotate files using the BatchAnnotateFiles API."]
    pub struct GoogleCloudVisionV1p2beta1BatchAnnotateFilesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no parent is specified, a region will be chosen automatically. Supported location-ids: `us`: USA country only, `asia`: East asia areas, like Japan, Taiwan, `eu`: The European Union. Example: `projects/project-A/locations/eu`."]
        pub parent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requests")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The list of file annotation requests. Right now we support only one AnnotateFileRequest in BatchAnnotateFilesRequest."]
        pub requests: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1AnnotateFileRequest>>,
        >,
    }
    impl GoogleCloudVisionV1p2beta1BatchAnnotateFilesRequest {
        pub fn builder() -> GoogleCloudVisionV1p2beta1BatchAnnotateFilesRequestBuilder {
            GoogleCloudVisionV1p2beta1BatchAnnotateFilesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of file annotation responses."]
    pub struct GoogleCloudVisionV1p2beta1BatchAnnotateFilesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of file annotation responses, each response corresponding to each AnnotateFileRequest in BatchAnnotateFilesRequest."]
        pub responses: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1AnnotateFileResponse>>,
        >,
    }
    impl GoogleCloudVisionV1p2beta1BatchAnnotateFilesResponse {
        pub fn builder() -> GoogleCloudVisionV1p2beta1BatchAnnotateFilesResponseBuilder {
            GoogleCloudVisionV1p2beta1BatchAnnotateFilesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Multiple image annotation requests are batched into a single service call."]
    pub struct GoogleCloudVisionV1p2beta1BatchAnnotateImagesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no parent is specified, a region will be chosen automatically. Supported location-ids: `us`: USA country only, `asia`: East asia areas, like Japan, Taiwan, `eu`: The European Union. Example: `projects/project-A/locations/eu`."]
        pub parent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requests")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Individual image annotation requests for this batch."]
        pub requests: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1AnnotateImageRequest>>,
        >,
    }
    impl GoogleCloudVisionV1p2beta1BatchAnnotateImagesRequest {
        pub fn builder() -> GoogleCloudVisionV1p2beta1BatchAnnotateImagesRequestBuilder {
            GoogleCloudVisionV1p2beta1BatchAnnotateImagesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to a batch image annotation request."]
    pub struct GoogleCloudVisionV1p2beta1BatchAnnotateImagesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Individual responses to image annotation requests within the batch."]
        pub responses: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1AnnotateImageResponse>>,
        >,
    }
    impl GoogleCloudVisionV1p2beta1BatchAnnotateImagesResponse {
        pub fn builder() -> GoogleCloudVisionV1p2beta1BatchAnnotateImagesResponseBuilder {
            GoogleCloudVisionV1p2beta1BatchAnnotateImagesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Logical element on the page."]
    pub struct GoogleCloudVisionV1p2beta1Block {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blockType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected block type (text, image etc) for this block."]
        pub block_type: ::std::option::Option<GoogleCloudVisionV1p2beta1BlockBlockTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding box for the block. The vertices are in the order of top-left, top-right, bottom-right, bottom-left. When a rotation of the bounding box is detected the rotation is represented as around the top-left corner as defined when the text is read in the 'natural' orientation. For example: * when the text is horizontal it might look like: 0----1 | | 3----2 * when it's rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3)."]
        pub bounding_box:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the OCR results on the block. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paragraphs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of paragraphs in this block (if this blocks is of type text)."]
        pub paragraphs: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1Paragraph>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information detected for the block."]
        pub property: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p2beta1TextAnnotationTextProperty>,
        >,
    }
    impl GoogleCloudVisionV1p2beta1Block {
        pub fn builder() -> GoogleCloudVisionV1p2beta1BlockBuilder {
            GoogleCloudVisionV1p2beta1BlockBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Detected block type (text, image etc) for this block."]
    pub enum GoogleCloudVisionV1p2beta1BlockBlockTypeEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown block type."]
        Unknown,
        #[serde(rename = "TEXT")]
        #[doc = "Regular text block."]
        Text,
        #[serde(rename = "TABLE")]
        #[doc = "Table block."]
        Table,
        #[serde(rename = "PICTURE")]
        #[doc = "Image block."]
        Picture,
        #[serde(rename = "RULER")]
        #[doc = "Horizontal/vertical line box."]
        Ruler,
        #[serde(rename = "BARCODE")]
        #[doc = "Barcode block."]
        Barcode,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p2beta1BlockBlockTypeEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A bounding polygon for the detected image annotation."]
    pub struct GoogleCloudVisionV1p2beta1BoundingPoly {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedVertices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon normalized vertices."]
        pub normalized_vertices: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1NormalizedVertex>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vertices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon vertices."]
        pub vertices: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1Vertex>>,
        >,
    }
    impl GoogleCloudVisionV1p2beta1BoundingPoly {
        pub fn builder() -> GoogleCloudVisionV1p2beta1BoundingPolyBuilder {
            GoogleCloudVisionV1p2beta1BoundingPolyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Color information consists of RGB channels, score, and the fraction of the image that the color occupies in the image."]
    pub struct GoogleCloudVisionV1p2beta1ColorInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "color")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "RGB components of the color."]
        pub color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pixelFraction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fraction of pixels the color occupies in the image. Value in range [0, 1]."]
        pub pixel_fraction: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image-specific score for this color. Value in range [0, 1]."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p2beta1ColorInfo {
        pub fn builder() -> GoogleCloudVisionV1p2beta1ColorInfoBuilder {
            GoogleCloudVisionV1p2beta1ColorInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Single crop hint that is used to generate a new crop when serving an image."]
    pub struct GoogleCloudVisionV1p2beta1CropHint {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon for the crop region. The coordinates of the bounding box are in the original image's scale."]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of this being a salient region. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importanceFraction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fraction of importance of this salient region with respect to the original image."]
        pub importance_fraction: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p2beta1CropHint {
        pub fn builder() -> GoogleCloudVisionV1p2beta1CropHintBuilder {
            GoogleCloudVisionV1p2beta1CropHintBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set of crop hints that are used to generate new crops when serving images."]
    pub struct GoogleCloudVisionV1p2beta1CropHintsAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cropHints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Crop hint results."]
        pub crop_hints: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1CropHint>>,
        >,
    }
    impl GoogleCloudVisionV1p2beta1CropHintsAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p2beta1CropHintsAnnotationBuilder {
            GoogleCloudVisionV1p2beta1CropHintsAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parameters for crop hints annotation request."]
    pub struct GoogleCloudVisionV1p2beta1CropHintsParams {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aspectRatios")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Aspect ratios in floats, representing the ratio of the width to the height of the image. For example, if the desired aspect ratio is 4/3, the corresponding float value should be 1.33333. If not specified, the best possible crop is returned. The number of provided aspect ratios is limited to a maximum of 16; any aspect ratios provided after the 16th are ignored."]
        pub aspect_ratios: ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
    }
    impl GoogleCloudVisionV1p2beta1CropHintsParams {
        pub fn builder() -> GoogleCloudVisionV1p2beta1CropHintsParamsBuilder {
            GoogleCloudVisionV1p2beta1CropHintsParamsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set of dominant colors and their corresponding scores."]
    pub struct GoogleCloudVisionV1p2beta1DominantColorsAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "RGB color values with their score and pixel fraction."]
        pub colors: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1ColorInfo>>,
        >,
    }
    impl GoogleCloudVisionV1p2beta1DominantColorsAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p2beta1DominantColorsAnnotationBuilder {
            GoogleCloudVisionV1p2beta1DominantColorsAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set of detected entity features."]
    pub struct GoogleCloudVisionV1p2beta1EntityAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image region to which this entity belongs. Not produced for `LABEL_DETECTION` features."]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "**Deprecated. Use `score` instead.** The accuracy of the entity detection in an image. For example, for an image in which the \"Eiffel Tower\" entity is detected, this field represents the confidence that there is a tower in the query image. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entity textual description, expressed in its `locale` language."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locale")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language code for the locale in which the entity textual `description` is expressed."]
        pub locale: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location information for the detected entity. Multiple `LocationInfo` elements can be present because one location may indicate the location of the scene in the image, and another location may indicate the location of the place where the image was taken. Location information is usually present for landmarks."]
        pub locations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1LocationInfo>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Opaque entity ID. Some IDs may be available in [Google Knowledge Graph Search API](https://developers.google.com/knowledge-graph/)."]
        pub mid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Some entities may have optional user-supplied `Property` (name/value) fields, such a score or string that qualifies the entity."]
        pub properties: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1Property>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Overall score of the result. Range [0, 1]."]
        pub score: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topicality")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The relevancy of the ICA (Image Content Annotation) label to the image. For example, the relevancy of \"tower\" is likely higher to an image containing the detected \"Eiffel Tower\" than to an image containing a detected distant towering building, even though the confidence that there is a tower in each image may be the same. Range [0, 1]."]
        pub topicality: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p2beta1EntityAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p2beta1EntityAnnotationBuilder {
            GoogleCloudVisionV1p2beta1EntityAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A face annotation object contains the results of face detection."]
    pub struct GoogleCloudVisionV1p2beta1FaceAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "angerLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Anger likelihood."]
        pub anger_likelihood:
            ::std::option::Option<GoogleCloudVisionV1p2beta1FaceAnnotationAngerLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blurredLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Blurred likelihood."]
        pub blurred_likelihood:
            ::std::option::Option<GoogleCloudVisionV1p2beta1FaceAnnotationBlurredLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon around the face. The coordinates of the bounding box are in the original image's scale. The bounding box is computed to \"frame\" the face in accordance with human expectations. It is based on the landmarker results. Note that one or more x and/or y coordinates may not be generated in the `BoundingPoly` (the polygon will be unbounded) if only a partial face appears in the image to be annotated."]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectionConfidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detection confidence. Range [0, 1]."]
        pub detection_confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fdBoundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `fd_bounding_poly` bounding polygon is tighter than the `boundingPoly`, and encloses only the skin part of the face. Typically, it is used to eliminate the face from any image analysis that detects the \"amount of skin\" visible in an image. It is not based on the landmarker results, only on the initial face detection, hence the fd (face detection) prefix."]
        pub fd_bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headwearLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Headwear likelihood."]
        pub headwear_likelihood:
            ::std::option::Option<GoogleCloudVisionV1p2beta1FaceAnnotationHeadwearLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "joyLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Joy likelihood."]
        pub joy_likelihood:
            ::std::option::Option<GoogleCloudVisionV1p2beta1FaceAnnotationJoyLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "landmarkingConfidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Face landmarking confidence. Range [0, 1]."]
        pub landmarking_confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "landmarks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected face landmarks."]
        pub landmarks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1FaceAnnotationLandmark>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "panAngle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Yaw angle, which indicates the leftward/rightward angle that the face is pointing relative to the vertical plane perpendicular to the image. Range [-180,180]."]
        pub pan_angle: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rollAngle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Roll angle, which indicates the amount of clockwise/anti-clockwise rotation of the face relative to the image vertical about the axis perpendicular to the face. Range [-180,180]."]
        pub roll_angle: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sorrowLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sorrow likelihood."]
        pub sorrow_likelihood:
            ::std::option::Option<GoogleCloudVisionV1p2beta1FaceAnnotationSorrowLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "surpriseLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Surprise likelihood."]
        pub surprise_likelihood:
            ::std::option::Option<GoogleCloudVisionV1p2beta1FaceAnnotationSurpriseLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tiltAngle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pitch angle, which indicates the upwards/downwards angle that the face is pointing relative to the image's horizontal plane. Range [-180,180]."]
        pub tilt_angle: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "underExposedLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Under-exposed likelihood."]
        pub under_exposed_likelihood: ::std::option::Option<
            GoogleCloudVisionV1p2beta1FaceAnnotationUnderExposedLikelihoodEnum,
        >,
    }
    impl GoogleCloudVisionV1p2beta1FaceAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p2beta1FaceAnnotationBuilder {
            GoogleCloudVisionV1p2beta1FaceAnnotationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Anger likelihood."]
    pub enum GoogleCloudVisionV1p2beta1FaceAnnotationAngerLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p2beta1FaceAnnotationAngerLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Blurred likelihood."]
    pub enum GoogleCloudVisionV1p2beta1FaceAnnotationBlurredLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p2beta1FaceAnnotationBlurredLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Headwear likelihood."]
    pub enum GoogleCloudVisionV1p2beta1FaceAnnotationHeadwearLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p2beta1FaceAnnotationHeadwearLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Joy likelihood."]
    pub enum GoogleCloudVisionV1p2beta1FaceAnnotationJoyLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p2beta1FaceAnnotationJoyLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Sorrow likelihood."]
    pub enum GoogleCloudVisionV1p2beta1FaceAnnotationSorrowLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p2beta1FaceAnnotationSorrowLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Surprise likelihood."]
    pub enum GoogleCloudVisionV1p2beta1FaceAnnotationSurpriseLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p2beta1FaceAnnotationSurpriseLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Under-exposed likelihood."]
    pub enum GoogleCloudVisionV1p2beta1FaceAnnotationUnderExposedLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default
        for GoogleCloudVisionV1p2beta1FaceAnnotationUnderExposedLikelihoodEnum
    {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A face-specific landmark (for example, a face feature)."]
    pub struct GoogleCloudVisionV1p2beta1FaceAnnotationLandmark {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "position")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Face landmark position."]
        pub position: ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1Position>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Face landmark type."]
        pub _type: ::std::option::Option<GoogleCloudVisionV1p2beta1FaceAnnotationLandmarkTypeEnum>,
    }
    impl GoogleCloudVisionV1p2beta1FaceAnnotationLandmark {
        pub fn builder() -> GoogleCloudVisionV1p2beta1FaceAnnotationLandmarkBuilder {
            GoogleCloudVisionV1p2beta1FaceAnnotationLandmarkBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Face landmark type."]
    pub enum GoogleCloudVisionV1p2beta1FaceAnnotationLandmarkTypeEnum {
        #[serde(rename = "UNKNOWN_LANDMARK")]
        #[doc = "Unknown face landmark detected. Should not be filled."]
        UnknownLandmark,
        #[serde(rename = "LEFT_EYE")]
        #[doc = "Left eye."]
        LeftEye,
        #[serde(rename = "RIGHT_EYE")]
        #[doc = "Right eye."]
        RightEye,
        #[serde(rename = "LEFT_OF_LEFT_EYEBROW")]
        #[doc = "Left of left eyebrow."]
        LeftOfLeftEyebrow,
        #[serde(rename = "RIGHT_OF_LEFT_EYEBROW")]
        #[doc = "Right of left eyebrow."]
        RightOfLeftEyebrow,
        #[serde(rename = "LEFT_OF_RIGHT_EYEBROW")]
        #[doc = "Left of right eyebrow."]
        LeftOfRightEyebrow,
        #[serde(rename = "RIGHT_OF_RIGHT_EYEBROW")]
        #[doc = "Right of right eyebrow."]
        RightOfRightEyebrow,
        #[serde(rename = "MIDPOINT_BETWEEN_EYES")]
        #[doc = "Midpoint between eyes."]
        MidpointBetweenEyes,
        #[serde(rename = "NOSE_TIP")]
        #[doc = "Nose tip."]
        NoseTip,
        #[serde(rename = "UPPER_LIP")]
        #[doc = "Upper lip."]
        UpperLip,
        #[serde(rename = "LOWER_LIP")]
        #[doc = "Lower lip."]
        LowerLip,
        #[serde(rename = "MOUTH_LEFT")]
        #[doc = "Mouth left."]
        MouthLeft,
        #[serde(rename = "MOUTH_RIGHT")]
        #[doc = "Mouth right."]
        MouthRight,
        #[serde(rename = "MOUTH_CENTER")]
        #[doc = "Mouth center."]
        MouthCenter,
        #[serde(rename = "NOSE_BOTTOM_RIGHT")]
        #[doc = "Nose, bottom right."]
        NoseBottomRight,
        #[serde(rename = "NOSE_BOTTOM_LEFT")]
        #[doc = "Nose, bottom left."]
        NoseBottomLeft,
        #[serde(rename = "NOSE_BOTTOM_CENTER")]
        #[doc = "Nose, bottom center."]
        NoseBottomCenter,
        #[serde(rename = "LEFT_EYE_TOP_BOUNDARY")]
        #[doc = "Left eye, top boundary."]
        LeftEyeTopBoundary,
        #[serde(rename = "LEFT_EYE_RIGHT_CORNER")]
        #[doc = "Left eye, right corner."]
        LeftEyeRightCorner,
        #[serde(rename = "LEFT_EYE_BOTTOM_BOUNDARY")]
        #[doc = "Left eye, bottom boundary."]
        LeftEyeBottomBoundary,
        #[serde(rename = "LEFT_EYE_LEFT_CORNER")]
        #[doc = "Left eye, left corner."]
        LeftEyeLeftCorner,
        #[serde(rename = "RIGHT_EYE_TOP_BOUNDARY")]
        #[doc = "Right eye, top boundary."]
        RightEyeTopBoundary,
        #[serde(rename = "RIGHT_EYE_RIGHT_CORNER")]
        #[doc = "Right eye, right corner."]
        RightEyeRightCorner,
        #[serde(rename = "RIGHT_EYE_BOTTOM_BOUNDARY")]
        #[doc = "Right eye, bottom boundary."]
        RightEyeBottomBoundary,
        #[serde(rename = "RIGHT_EYE_LEFT_CORNER")]
        #[doc = "Right eye, left corner."]
        RightEyeLeftCorner,
        #[serde(rename = "LEFT_EYEBROW_UPPER_MIDPOINT")]
        #[doc = "Left eyebrow, upper midpoint."]
        LeftEyebrowUpperMidpoint,
        #[serde(rename = "RIGHT_EYEBROW_UPPER_MIDPOINT")]
        #[doc = "Right eyebrow, upper midpoint."]
        RightEyebrowUpperMidpoint,
        #[serde(rename = "LEFT_EAR_TRAGION")]
        #[doc = "Left ear tragion."]
        LeftEarTragion,
        #[serde(rename = "RIGHT_EAR_TRAGION")]
        #[doc = "Right ear tragion."]
        RightEarTragion,
        #[serde(rename = "LEFT_EYE_PUPIL")]
        #[doc = "Left eye pupil."]
        LeftEyePupil,
        #[serde(rename = "RIGHT_EYE_PUPIL")]
        #[doc = "Right eye pupil."]
        RightEyePupil,
        #[serde(rename = "FOREHEAD_GLABELLA")]
        #[doc = "Forehead glabella."]
        ForeheadGlabella,
        #[serde(rename = "CHIN_GNATHION")]
        #[doc = "Chin gnathion."]
        ChinGnathion,
        #[serde(rename = "CHIN_LEFT_GONION")]
        #[doc = "Chin left gonion."]
        ChinLeftGonion,
        #[serde(rename = "CHIN_RIGHT_GONION")]
        #[doc = "Chin right gonion."]
        ChinRightGonion,
        #[serde(rename = "LEFT_CHEEK_CENTER")]
        #[doc = "Left cheek center."]
        LeftCheekCenter,
        #[serde(rename = "RIGHT_CHEEK_CENTER")]
        #[doc = "Right cheek center."]
        RightCheekCenter,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p2beta1FaceAnnotationLandmarkTypeEnum {
        fn default() -> Self {
            Self::UnknownLandmark
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The type of Google Cloud Vision API detection to perform, and the maximum number of results to return for that type. Multiple `Feature` objects can be specified in the `features` list."]
    pub struct GoogleCloudVisionV1p2beta1Feature {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum number of results of this type. Does not apply to `TEXT_DETECTION`, `DOCUMENT_TEXT_DETECTION`, or `CROP_HINTS`."]
        pub max_results: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "model")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Model to use for the feature. Supported values: \"builtin/stable\" (the default if unset) and \"builtin/latest\"."]
        pub model: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The feature type."]
        pub _type: ::std::option::Option<GoogleCloudVisionV1p2beta1FeatureTypeEnum>,
    }
    impl GoogleCloudVisionV1p2beta1Feature {
        pub fn builder() -> GoogleCloudVisionV1p2beta1FeatureBuilder {
            GoogleCloudVisionV1p2beta1FeatureBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The feature type."]
    pub enum GoogleCloudVisionV1p2beta1FeatureTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "Unspecified feature type."]
        TypeUnspecified,
        #[serde(rename = "FACE_DETECTION")]
        #[doc = "Run face detection."]
        FaceDetection,
        #[serde(rename = "LANDMARK_DETECTION")]
        #[doc = "Run landmark detection."]
        LandmarkDetection,
        #[serde(rename = "LOGO_DETECTION")]
        #[doc = "Run logo detection."]
        LogoDetection,
        #[serde(rename = "LABEL_DETECTION")]
        #[doc = "Run label detection."]
        LabelDetection,
        #[serde(rename = "TEXT_DETECTION")]
        #[doc = "Run text detection / optical character recognition (OCR). Text detection is optimized for areas of text within a larger image; if the image is a document, use `DOCUMENT_TEXT_DETECTION` instead."]
        TextDetection,
        #[serde(rename = "DOCUMENT_TEXT_DETECTION")]
        #[doc = "Run dense text document OCR. Takes precedence when both `DOCUMENT_TEXT_DETECTION` and `TEXT_DETECTION` are present."]
        DocumentTextDetection,
        #[serde(rename = "SAFE_SEARCH_DETECTION")]
        #[doc = "Run Safe Search to detect potentially unsafe or undesirable content."]
        SafeSearchDetection,
        #[serde(rename = "IMAGE_PROPERTIES")]
        #[doc = "Compute a set of image properties, such as the image's dominant colors."]
        ImageProperties,
        #[serde(rename = "CROP_HINTS")]
        #[doc = "Run crop hints."]
        CropHints,
        #[serde(rename = "WEB_DETECTION")]
        #[doc = "Run web detection."]
        WebDetection,
        #[serde(rename = "PRODUCT_SEARCH")]
        #[doc = "Run Product Search."]
        ProductSearch,
        #[serde(rename = "OBJECT_LOCALIZATION")]
        #[doc = "Run localizer for object detection."]
        ObjectLocalization,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p2beta1FeatureTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Google Cloud Storage location where the output will be written to."]
    pub struct GoogleCloudVisionV1p2beta1GcsDestination {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage URI prefix where the results will be stored. Results will be in JSON format and preceded by its corresponding input URI prefix. This field can either represent a gcs file prefix or gcs directory. In either case, the uri should be unique because in order to get all of the output files, you will need to do a wildcard gcs search on the uri prefix you provide. Examples: * File Prefix: gs://bucket-name/here/filenameprefix The output files will be created in gs://bucket-name/here/ and the names of the output files will begin with \"filenameprefix\". * Directory Prefix: gs://bucket-name/some/location/ The output files will be created in gs://bucket-name/some/location/ and the names of the output files could be anything because there was no filename prefix specified. If multiple outputs, each response is still AnnotateFileResponse, each of which contains some subset of the full list of AnnotateImageResponse. Multiple outputs can happen if, for example, the output JSON is too large and overflows into multiple sharded files."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p2beta1GcsDestination {
        pub fn builder() -> GoogleCloudVisionV1p2beta1GcsDestinationBuilder {
            GoogleCloudVisionV1p2beta1GcsDestinationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Google Cloud Storage location where the input will be read from."]
    pub struct GoogleCloudVisionV1p2beta1GcsSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage URI for the input file. This must only be a Google Cloud Storage object. Wildcards are not currently supported."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p2beta1GcsSource {
        pub fn builder() -> GoogleCloudVisionV1p2beta1GcsSourceBuilder {
            GoogleCloudVisionV1p2beta1GcsSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Client image to perform Google Cloud Vision API tasks over."]
    pub struct GoogleCloudVisionV1p2beta1Image {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image content, represented as a stream of bytes. Note: As with all `bytes` fields, protobuffers use a pure binary representation, whereas JSON representations use base64. Currently, this field only works for BatchAnnotateImages requests. It does not work for AsyncBatchAnnotateImages requests."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage image location, or publicly-accessible image URL. If both `content` and `source` are provided for an image, `content` takes precedence and is used to perform the image annotation request."]
        pub source: ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1ImageSource>>,
    }
    impl GoogleCloudVisionV1p2beta1Image {
        pub fn builder() -> GoogleCloudVisionV1p2beta1ImageBuilder {
            GoogleCloudVisionV1p2beta1ImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "If an image was produced from a file (e.g. a PDF), this message gives information about the source of that image."]
    pub struct GoogleCloudVisionV1p2beta1ImageAnnotationContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the file was a PDF or TIFF, this field gives the page number within the file used to produce the image."]
        pub page_number: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI of the file used to produce the image."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p2beta1ImageAnnotationContext {
        pub fn builder() -> GoogleCloudVisionV1p2beta1ImageAnnotationContextBuilder {
            GoogleCloudVisionV1p2beta1ImageAnnotationContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Image context and/or feature-specific parameters."]
    pub struct GoogleCloudVisionV1p2beta1ImageContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cropHintsParams")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parameters for crop hints annotation request."]
        pub crop_hints_params:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1CropHintsParams>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageHints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of languages to use for TEXT_DETECTION. In most cases, an empty value yields the best results since it enables automatic language detection. For languages based on the Latin alphabet, setting `language_hints` is not needed. In rare cases, when the language of the text in the image is known, setting a hint will help get better results (although it will be a significant hindrance if the hint is wrong). Text detection returns an error if one or more of the specified languages is not one of the [supported languages](https://cloud.google.com/vision/docs/languages)."]
        pub language_hints: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "latLongRect")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Not used."]
        pub lat_long_rect:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1LatLongRect>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productSearchParams")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parameters for product search."]
        pub product_search_params:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1ProductSearchParams>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textDetectionParams")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parameters for text detection and document text detection."]
        pub text_detection_params:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1TextDetectionParams>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webDetectionParams")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parameters for web detection."]
        pub web_detection_params:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1WebDetectionParams>>,
    }
    impl GoogleCloudVisionV1p2beta1ImageContext {
        pub fn builder() -> GoogleCloudVisionV1p2beta1ImageContextBuilder {
            GoogleCloudVisionV1p2beta1ImageContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Stores image properties, such as dominant colors."]
    pub struct GoogleCloudVisionV1p2beta1ImageProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dominantColors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, dominant colors completed successfully."]
        pub dominant_colors: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p2beta1DominantColorsAnnotation>,
        >,
    }
    impl GoogleCloudVisionV1p2beta1ImageProperties {
        pub fn builder() -> GoogleCloudVisionV1p2beta1ImagePropertiesBuilder {
            GoogleCloudVisionV1p2beta1ImagePropertiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "External image source (Google Cloud Storage or web URL image location)."]
    pub struct GoogleCloudVisionV1p2beta1ImageSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsImageUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "**Use `image_uri` instead.** The Google Cloud Storage URI of the form `gs://bucket_name/object_name`. Object versioning is not supported. See [Google Cloud Storage Request URIs](https://cloud.google.com/storage/docs/reference-uris) for more info."]
        pub gcs_image_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI of the source image. Can be either: 1. A Google Cloud Storage URI of the form `gs://bucket_name/object_name`. Object versioning is not supported. See [Google Cloud Storage Request URIs](https://cloud.google.com/storage/docs/reference-uris) for more info. 2. A publicly-accessible image HTTP/HTTPS URL. When fetching images from HTTP/HTTPS URLs, Google cannot guarantee that the request will be completed. Your request may fail if the specified host denies the request (e.g. due to request throttling or DOS prevention), or if Google throttles requests to the site for abuse prevention. You should not depend on externally-hosted images for production applications. When both `gcs_image_uri` and `image_uri` are specified, `image_uri` takes precedence."]
        pub image_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p2beta1ImageSource {
        pub fn builder() -> GoogleCloudVisionV1p2beta1ImageSourceBuilder {
            GoogleCloudVisionV1p2beta1ImageSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The desired input location and metadata."]
    pub struct GoogleCloudVisionV1p2beta1InputConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "File content, represented as a stream of bytes. Note: As with all `bytes` fields, protobuffers use a pure binary representation, whereas JSON representations use base64. Currently, this field only works for BatchAnnotateFiles requests. It does not work for AsyncBatchAnnotateFiles requests."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Google Cloud Storage location to read the input from."]
        pub gcs_source:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1GcsSource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the file. Currently only \"application/pdf\", \"image/tiff\" and \"image/gif\" are supported. Wildcards are not supported."]
        pub mime_type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p2beta1InputConfig {
        pub fn builder() -> GoogleCloudVisionV1p2beta1InputConfigBuilder {
            GoogleCloudVisionV1p2beta1InputConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Rectangle determined by min and max `LatLng` pairs."]
    pub struct GoogleCloudVisionV1p2beta1LatLongRect {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxLatLng")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Max lat/long pair."]
        pub max_lat_lng: ::std::option::Option<::std::boxed::Box<LatLng>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minLatLng")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Min lat/long pair."]
        pub min_lat_lng: ::std::option::Option<::std::boxed::Box<LatLng>>,
    }
    impl GoogleCloudVisionV1p2beta1LatLongRect {
        pub fn builder() -> GoogleCloudVisionV1p2beta1LatLongRectBuilder {
            GoogleCloudVisionV1p2beta1LatLongRectBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set of detected objects with bounding boxes."]
    pub struct GoogleCloudVisionV1p2beta1LocalizedObjectAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image region to which this object belongs. This must be populated."]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object ID that should align with EntityAnnotation mid."]
        pub mid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object name, expressed in its `language_code` language."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Score of the result. Range [0, 1]."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p2beta1LocalizedObjectAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p2beta1LocalizedObjectAnnotationBuilder {
            GoogleCloudVisionV1p2beta1LocalizedObjectAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected entity location information."]
    pub struct GoogleCloudVisionV1p2beta1LocationInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "latLng")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "lat/long location coordinates."]
        pub lat_lng: ::std::option::Option<::std::boxed::Box<LatLng>>,
    }
    impl GoogleCloudVisionV1p2beta1LocationInfo {
        pub fn builder() -> GoogleCloudVisionV1p2beta1LocationInfoBuilder {
            GoogleCloudVisionV1p2beta1LocationInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A vertex represents a 2D point in the image. NOTE: the normalized vertex coordinates are relative to the original image and range from 0 to 1."]
    pub struct GoogleCloudVisionV1p2beta1NormalizedVertex {
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
    impl GoogleCloudVisionV1p2beta1NormalizedVertex {
        pub fn builder() -> GoogleCloudVisionV1p2beta1NormalizedVertexBuilder {
            GoogleCloudVisionV1p2beta1NormalizedVertexBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains metadata for the BatchAnnotateImages operation."]
    pub struct GoogleCloudVisionV1p2beta1OperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when the batch request was received."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Current state of the batch operation."]
        pub state: ::std::option::Option<GoogleCloudVisionV1p2beta1OperationMetadataStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when the operation result was last updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p2beta1OperationMetadata {
        pub fn builder() -> GoogleCloudVisionV1p2beta1OperationMetadataBuilder {
            GoogleCloudVisionV1p2beta1OperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Current state of the batch operation."]
    pub enum GoogleCloudVisionV1p2beta1OperationMetadataStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Invalid."]
        StateUnspecified,
        #[serde(rename = "CREATED")]
        #[doc = "Request is received."]
        Created,
        #[serde(rename = "RUNNING")]
        #[doc = "Request is actively being processed."]
        Running,
        #[serde(rename = "DONE")]
        #[doc = "The batch processing is done."]
        Done,
        #[serde(rename = "CANCELLED")]
        #[doc = "The batch processing was cancelled."]
        Cancelled,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p2beta1OperationMetadataStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The desired output location and metadata."]
    pub struct GoogleCloudVisionV1p2beta1OutputConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The max number of response protos to put into each output JSON file on Google Cloud Storage. The valid range is [1, 100]. If not specified, the default value is 20. For example, for one pdf file with 100 pages, 100 response protos will be generated. If `batch_size` = 20, then 5 json files each containing 20 response protos will be written under the prefix `gcs_destination`.`uri`. Currently, batch_size only applies to GcsDestination, with potential future support for other output configurations."]
        pub batch_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsDestination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Google Cloud Storage location to write the output(s) to."]
        pub gcs_destination:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1GcsDestination>>,
    }
    impl GoogleCloudVisionV1p2beta1OutputConfig {
        pub fn builder() -> GoogleCloudVisionV1p2beta1OutputConfigBuilder {
            GoogleCloudVisionV1p2beta1OutputConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected page from OCR."]
    pub struct GoogleCloudVisionV1p2beta1Page {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blocks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of blocks of text, images etc on this page."]
        pub blocks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1Block>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the OCR results on the page. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page height. For PDFs the unit is points. For images (including TIFFs) the unit is pixels."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information detected on the page."]
        pub property: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p2beta1TextAnnotationTextProperty>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page width. For PDFs the unit is points. For images (including TIFFs) the unit is pixels."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudVisionV1p2beta1Page {
        pub fn builder() -> GoogleCloudVisionV1p2beta1PageBuilder {
            GoogleCloudVisionV1p2beta1PageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Structural unit of text representing a number of words in certain order."]
    pub struct GoogleCloudVisionV1p2beta1Paragraph {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding box for the paragraph. The vertices are in the order of top-left, top-right, bottom-right, bottom-left. When a rotation of the bounding box is detected the rotation is represented as around the top-left corner as defined when the text is read in the 'natural' orientation. For example: * when the text is horizontal it might look like: 0----1 | | 3----2 * when it's rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3)."]
        pub bounding_box:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the OCR results for the paragraph. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information detected for the paragraph."]
        pub property: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p2beta1TextAnnotationTextProperty>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "words")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of all words in this paragraph."]
        pub words: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1Word>>,
        >,
    }
    impl GoogleCloudVisionV1p2beta1Paragraph {
        pub fn builder() -> GoogleCloudVisionV1p2beta1ParagraphBuilder {
            GoogleCloudVisionV1p2beta1ParagraphBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A 3D position in the image, used primarily for Face detection landmarks. A valid Position must have both x and y coordinates. The position coordinates are in the same scale as the original image."]
    pub struct GoogleCloudVisionV1p2beta1Position {
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
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "z")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Z coordinate (or depth)."]
        pub z: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p2beta1Position {
        pub fn builder() -> GoogleCloudVisionV1p2beta1PositionBuilder {
            GoogleCloudVisionV1p2beta1PositionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Product contains ReferenceImages."]
    pub struct GoogleCloudVisionV1p2beta1Product {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User-provided metadata to be stored with this product. Must be at most 4096 characters long."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user-provided name for this Product. Must not be empty. Must be at most 4096 characters long."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the product. Format is: `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`. This field is ignored when creating a product."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productCategory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Immutable. The category for the product identified by the reference image. This should be one of \"homegoods-v2\", \"apparel-v2\", \"toys-v2\", \"packagedgoods-v1\" or \"general-v1\". The legacy categories \"homegoods\", \"apparel\", and \"toys\" are still supported, but these should not be used for new products."]
        pub product_category: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Key-value pairs that can be attached to a product. At query time, constraints can be specified based on the product_labels. Note that integer values can be provided as strings, e.g. \"1199\". Only strings with integer values can match a range-based restriction which is to be supported soon. Multiple values can be assigned to the same key. One product may have up to 500 product_labels. Notice that the total number of distinct product_labels over all products in one ProductSet cannot exceed 1M, otherwise the product search pipeline will refuse to work for that ProductSet."]
        pub product_labels: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1ProductKeyValue>>,
        >,
    }
    impl GoogleCloudVisionV1p2beta1Product {
        pub fn builder() -> GoogleCloudVisionV1p2beta1ProductBuilder {
            GoogleCloudVisionV1p2beta1ProductBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A product label represented as a key-value pair."]
    pub struct GoogleCloudVisionV1p2beta1ProductKeyValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The key of the label attached to the product. Cannot be empty and cannot exceed 128 bytes."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value of the label attached to the product. Cannot be empty and cannot exceed 128 bytes."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p2beta1ProductKeyValue {
        pub fn builder() -> GoogleCloudVisionV1p2beta1ProductKeyValueBuilder {
            GoogleCloudVisionV1p2beta1ProductKeyValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parameters for a product search request."]
    pub struct GoogleCloudVisionV1p2beta1ProductSearchParams {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon around the area of interest in the image. If it is not specified, system discretion will be applied."]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The filtering expression. This can be used to restrict search results based on Product labels. We currently support an AND of OR of key-value expressions, where each expression within an OR must have the same key. An '=' should be used to connect the key and value. For example, \"(color = red OR color = blue) AND brand = Google\" is acceptable, but \"(color = red OR brand = Google)\" is not acceptable. \"color: red\" is not acceptable because it uses a ':' instead of an '='."]
        pub filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productCategories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of product categories to search in. Currently, we only consider the first category, and either \"homegoods-v2\", \"apparel-v2\", \"toys-v2\", \"packagedgoods-v1\", or \"general-v1\" should be specified. The legacy categories \"homegoods\", \"apparel\", and \"toys\" are still supported but will be deprecated. For new products, please use \"homegoods-v2\", \"apparel-v2\", or \"toys-v2\" for better product search accuracy. It is recommended to migrate existing products to these categories as well."]
        pub product_categories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productSet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of a ProductSet to be searched for similar images. Format is: `projects/PROJECT_ID/locations/LOC_ID/productSets/PRODUCT_SET_ID`."]
        pub product_set: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p2beta1ProductSearchParams {
        pub fn builder() -> GoogleCloudVisionV1p2beta1ProductSearchParamsBuilder {
            GoogleCloudVisionV1p2beta1ProductSearchParamsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Results for a product search request."]
    pub struct GoogleCloudVisionV1p2beta1ProductSearchResults {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "indexTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp of the index which provided these results. Products added to the product set and products removed from the product set after this time are not reflected in the current results."]
        pub index_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productGroupedResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of results grouped by products detected in the query image. Each entry corresponds to one bounding polygon in the query image, and contains the matching products specific to that region. There may be duplicate product matches in the union of all the per-product results."]
        pub product_grouped_results: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVisionV1p2beta1ProductSearchResultsGroupedResult>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of results, one for each product match."]
        pub results: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVisionV1p2beta1ProductSearchResultsResult>,
            >,
        >,
    }
    impl GoogleCloudVisionV1p2beta1ProductSearchResults {
        pub fn builder() -> GoogleCloudVisionV1p2beta1ProductSearchResultsBuilder {
            GoogleCloudVisionV1p2beta1ProductSearchResultsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about the products similar to a single product in a query image."]
    pub struct GoogleCloudVisionV1p2beta1ProductSearchResultsGroupedResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon around the product detected in the query image."]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of generic predictions for the object in the bounding box."]
        pub object_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVisionV1p2beta1ProductSearchResultsObjectAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of results, one for each product match."]
        pub results: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVisionV1p2beta1ProductSearchResultsResult>,
            >,
        >,
    }
    impl GoogleCloudVisionV1p2beta1ProductSearchResultsGroupedResult {
        pub fn builder() -> GoogleCloudVisionV1p2beta1ProductSearchResultsGroupedResultBuilder {
            GoogleCloudVisionV1p2beta1ProductSearchResultsGroupedResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Prediction for what the object in the bounding box is."]
    pub struct GoogleCloudVisionV1p2beta1ProductSearchResultsObjectAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object ID that should align with EntityAnnotation mid."]
        pub mid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object name, expressed in its `language_code` language."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Score of the result. Range [0, 1]."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p2beta1ProductSearchResultsObjectAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p2beta1ProductSearchResultsObjectAnnotationBuilder {
            GoogleCloudVisionV1p2beta1ProductSearchResultsObjectAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a product."]
    pub struct GoogleCloudVisionV1p2beta1ProductSearchResultsResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the image from the product that is the closest match to the query."]
        pub image: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "product")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Product."]
        pub product: ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1Product>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A confidence level on the match, ranging from 0 (no confidence) to 1 (full confidence)."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p2beta1ProductSearchResultsResult {
        pub fn builder() -> GoogleCloudVisionV1p2beta1ProductSearchResultsResultBuilder {
            GoogleCloudVisionV1p2beta1ProductSearchResultsResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A `Property` consists of a user-supplied name/value pair."]
    pub struct GoogleCloudVisionV1p2beta1Property {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the property."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uint64Value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value of numeric properties."]
        pub uint64_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value of the property."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p2beta1Property {
        pub fn builder() -> GoogleCloudVisionV1p2beta1PropertyBuilder {
            GoogleCloudVisionV1p2beta1PropertyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set of features pertaining to the image, computed by computer vision methods over safe-search verticals (for example, adult, spoof, medical, violence)."]
    pub struct GoogleCloudVisionV1p2beta1SafeSearchAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents the adult content likelihood for the image. Adult content may contain elements such as nudity, pornographic images or cartoons, or sexual activities."]
        pub adult: ::std::option::Option<GoogleCloudVisionV1p2beta1SafeSearchAnnotationAdultEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "medical")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Likelihood that this is a medical image."]
        pub medical:
            ::std::option::Option<GoogleCloudVisionV1p2beta1SafeSearchAnnotationMedicalEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "racy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Likelihood that the request image contains racy content. Racy content may include (but is not limited to) skimpy or sheer clothing, strategically covered nudity, lewd or provocative poses, or close-ups of sensitive body areas."]
        pub racy: ::std::option::Option<GoogleCloudVisionV1p2beta1SafeSearchAnnotationRacyEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spoof")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Spoof likelihood. The likelihood that an modification was made to the image's canonical version to make it appear funny or offensive."]
        pub spoof: ::std::option::Option<GoogleCloudVisionV1p2beta1SafeSearchAnnotationSpoofEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "violence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Likelihood that this image contains violent content."]
        pub violence:
            ::std::option::Option<GoogleCloudVisionV1p2beta1SafeSearchAnnotationViolenceEnum>,
    }
    impl GoogleCloudVisionV1p2beta1SafeSearchAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p2beta1SafeSearchAnnotationBuilder {
            GoogleCloudVisionV1p2beta1SafeSearchAnnotationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Represents the adult content likelihood for the image. Adult content may contain elements such as nudity, pornographic images or cartoons, or sexual activities."]
    pub enum GoogleCloudVisionV1p2beta1SafeSearchAnnotationAdultEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p2beta1SafeSearchAnnotationAdultEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Likelihood that this is a medical image."]
    pub enum GoogleCloudVisionV1p2beta1SafeSearchAnnotationMedicalEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p2beta1SafeSearchAnnotationMedicalEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Likelihood that the request image contains racy content. Racy content may include (but is not limited to) skimpy or sheer clothing, strategically covered nudity, lewd or provocative poses, or close-ups of sensitive body areas."]
    pub enum GoogleCloudVisionV1p2beta1SafeSearchAnnotationRacyEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p2beta1SafeSearchAnnotationRacyEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Spoof likelihood. The likelihood that an modification was made to the image's canonical version to make it appear funny or offensive."]
    pub enum GoogleCloudVisionV1p2beta1SafeSearchAnnotationSpoofEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p2beta1SafeSearchAnnotationSpoofEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Likelihood that this image contains violent content."]
    pub enum GoogleCloudVisionV1p2beta1SafeSearchAnnotationViolenceEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p2beta1SafeSearchAnnotationViolenceEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single symbol representation."]
    pub struct GoogleCloudVisionV1p2beta1Symbol {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding box for the symbol. The vertices are in the order of top-left, top-right, bottom-right, bottom-left. When a rotation of the bounding box is detected the rotation is represented as around the top-left corner as defined when the text is read in the 'natural' orientation. For example: * when the text is horizontal it might look like: 0----1 | | 3----2 * when it's rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3)."]
        pub bounding_box:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the OCR results for the symbol. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information detected for the symbol."]
        pub property: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p2beta1TextAnnotationTextProperty>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The actual UTF-8 representation of the symbol."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p2beta1Symbol {
        pub fn builder() -> GoogleCloudVisionV1p2beta1SymbolBuilder {
            GoogleCloudVisionV1p2beta1SymbolBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "TextAnnotation contains a structured representation of OCR extracted text. The hierarchy of an OCR extracted text structure is like this: TextAnnotation -> Page -> Block -> Paragraph -> Word -> Symbol Each structural component, starting from Page, may further have their own properties. Properties describe detected languages, breaks etc.. Please refer to the TextAnnotation.TextProperty message definition below for more detail."]
    pub struct GoogleCloudVisionV1p2beta1TextAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of pages detected by OCR."]
        pub pages: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1Page>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "UTF-8 text detected on the pages."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p2beta1TextAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p2beta1TextAnnotationBuilder {
            GoogleCloudVisionV1p2beta1TextAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected start or end of a structural component."]
    pub struct GoogleCloudVisionV1p2beta1TextAnnotationDetectedBreak {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isPrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if break prepends the element."]
        pub is_prefix: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected break type."]
        pub _type:
            ::std::option::Option<GoogleCloudVisionV1p2beta1TextAnnotationDetectedBreakTypeEnum>,
    }
    impl GoogleCloudVisionV1p2beta1TextAnnotationDetectedBreak {
        pub fn builder() -> GoogleCloudVisionV1p2beta1TextAnnotationDetectedBreakBuilder {
            GoogleCloudVisionV1p2beta1TextAnnotationDetectedBreakBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Detected break type."]
    pub enum GoogleCloudVisionV1p2beta1TextAnnotationDetectedBreakTypeEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown break label type."]
        Unknown,
        #[serde(rename = "SPACE")]
        #[doc = "Regular space."]
        Space,
        #[serde(rename = "SURE_SPACE")]
        #[doc = "Sure space (very wide)."]
        SureSpace,
        #[serde(rename = "EOL_SURE_SPACE")]
        #[doc = "Line-wrapping break."]
        EolSureSpace,
        #[serde(rename = "HYPHEN")]
        #[doc = "End-line hyphen that is not present in text; does not co-occur with `SPACE`, `LEADER_SPACE`, or `LINE_BREAK`."]
        Hyphen,
        #[serde(rename = "LINE_BREAK")]
        #[doc = "Line break that ends a paragraph."]
        LineBreak,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p2beta1TextAnnotationDetectedBreakTypeEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected language for a structural component."]
    pub struct GoogleCloudVisionV1p2beta1TextAnnotationDetectedLanguage {
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
    impl GoogleCloudVisionV1p2beta1TextAnnotationDetectedLanguage {
        pub fn builder() -> GoogleCloudVisionV1p2beta1TextAnnotationDetectedLanguageBuilder {
            GoogleCloudVisionV1p2beta1TextAnnotationDetectedLanguageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional information detected on the structural component."]
    pub struct GoogleCloudVisionV1p2beta1TextAnnotationTextProperty {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedBreak")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected start or end of a text segment."]
        pub detected_break: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p2beta1TextAnnotationDetectedBreak>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected languages together with confidence."]
        pub detected_languages: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVisionV1p2beta1TextAnnotationDetectedLanguage>,
            >,
        >,
    }
    impl GoogleCloudVisionV1p2beta1TextAnnotationTextProperty {
        pub fn builder() -> GoogleCloudVisionV1p2beta1TextAnnotationTextPropertyBuilder {
            GoogleCloudVisionV1p2beta1TextAnnotationTextPropertyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parameters for text detections. This is used to control TEXT_DETECTION and DOCUMENT_TEXT_DETECTION features."]
    pub struct GoogleCloudVisionV1p2beta1TextDetectionParams {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableTextDetectionConfidenceScore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "By default, Cloud Vision API only includes confidence score for DOCUMENT_TEXT_DETECTION result. Set the flag to true to include confidence score for TEXT_DETECTION as well."]
        pub enable_text_detection_confidence_score: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudVisionV1p2beta1TextDetectionParams {
        pub fn builder() -> GoogleCloudVisionV1p2beta1TextDetectionParamsBuilder {
            GoogleCloudVisionV1p2beta1TextDetectionParamsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A vertex represents a 2D point in the image. NOTE: the vertex coordinates are in the same scale as the original image."]
    pub struct GoogleCloudVisionV1p2beta1Vertex {
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
    impl GoogleCloudVisionV1p2beta1Vertex {
        pub fn builder() -> GoogleCloudVisionV1p2beta1VertexBuilder {
            GoogleCloudVisionV1p2beta1VertexBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Relevant information for the image from the Internet."]
    pub struct GoogleCloudVisionV1p2beta1WebDetection {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bestGuessLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The service's best guess as to the topic of the request image. Inferred from similar images on the open web."]
        pub best_guess_labels: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1WebDetectionWebLabel>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullMatchingImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fully matching images from the Internet. Can include resized copies of the query image."]
        pub full_matching_images: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1WebDetectionWebImage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pagesWithMatchingImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web pages containing the matching images from the Internet."]
        pub pages_with_matching_images: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1WebDetectionWebPage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialMatchingImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Partial matching images from the Internet. Those images are similar enough to share some key-point features. For example an original image will likely have partial matching for its crops."]
        pub partial_matching_images: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1WebDetectionWebImage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visuallySimilarImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visually similar image results."]
        pub visually_similar_images: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1WebDetectionWebImage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webEntities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deduced entities from similar images on the Internet."]
        pub web_entities: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1WebDetectionWebEntity>>,
        >,
    }
    impl GoogleCloudVisionV1p2beta1WebDetection {
        pub fn builder() -> GoogleCloudVisionV1p2beta1WebDetectionBuilder {
            GoogleCloudVisionV1p2beta1WebDetectionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parameters for web detection request."]
    pub struct GoogleCloudVisionV1p2beta1WebDetectionParams {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeGeoResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to include results derived from the geo information in the image."]
        pub include_geo_results: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudVisionV1p2beta1WebDetectionParams {
        pub fn builder() -> GoogleCloudVisionV1p2beta1WebDetectionParamsBuilder {
            GoogleCloudVisionV1p2beta1WebDetectionParamsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Entity deduced from similar images on the Internet."]
    pub struct GoogleCloudVisionV1p2beta1WebDetectionWebEntity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Canonical description of the entity, in English."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Opaque entity ID."]
        pub entity_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Overall relevancy score for the entity. Not normalized and not comparable across different image queries."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p2beta1WebDetectionWebEntity {
        pub fn builder() -> GoogleCloudVisionV1p2beta1WebDetectionWebEntityBuilder {
            GoogleCloudVisionV1p2beta1WebDetectionWebEntityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for online images."]
    pub struct GoogleCloudVisionV1p2beta1WebDetectionWebImage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "(Deprecated) Overall relevancy score for the image."]
        pub score: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result image URL."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p2beta1WebDetectionWebImage {
        pub fn builder() -> GoogleCloudVisionV1p2beta1WebDetectionWebImageBuilder {
            GoogleCloudVisionV1p2beta1WebDetectionWebImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Label to provide extra metadata for the web detection."]
    pub struct GoogleCloudVisionV1p2beta1WebDetectionWebLabel {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "label")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label for extra metadata."]
        pub label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The BCP-47 language code for `label`, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        pub language_code: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p2beta1WebDetectionWebLabel {
        pub fn builder() -> GoogleCloudVisionV1p2beta1WebDetectionWebLabelBuilder {
            GoogleCloudVisionV1p2beta1WebDetectionWebLabelBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for web pages."]
    pub struct GoogleCloudVisionV1p2beta1WebDetectionWebPage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullMatchingImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fully matching images on the page. Can include resized copies of the query image."]
        pub full_matching_images: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1WebDetectionWebImage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title for the web page, may contain HTML markups."]
        pub page_title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialMatchingImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Partial matching images on the page. Those images are similar enough to share some key-point features. For example an original image will likely have partial matching for its crops."]
        pub partial_matching_images: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1WebDetectionWebImage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "(Deprecated) Overall relevancy score for the web page."]
        pub score: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result web page URL."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p2beta1WebDetectionWebPage {
        pub fn builder() -> GoogleCloudVisionV1p2beta1WebDetectionWebPageBuilder {
            GoogleCloudVisionV1p2beta1WebDetectionWebPageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A word representation."]
    pub struct GoogleCloudVisionV1p2beta1Word {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding box for the word. The vertices are in the order of top-left, top-right, bottom-right, bottom-left. When a rotation of the bounding box is detected the rotation is represented as around the top-left corner as defined when the text is read in the 'natural' orientation. For example: * when the text is horizontal it might look like: 0----1 | | 3----2 * when it's rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3)."]
        pub bounding_box:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p2beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the OCR results for the word. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information detected for the word."]
        pub property: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p2beta1TextAnnotationTextProperty>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "symbols")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of symbols in the word. The order of the symbols follows the natural reading order."]
        pub symbols: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p2beta1Symbol>>,
        >,
    }
    impl GoogleCloudVisionV1p2beta1Word {
        pub fn builder() -> GoogleCloudVisionV1p2beta1WordBuilder {
            GoogleCloudVisionV1p2beta1WordBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to a single file annotation request. A file may contain one or more images, which individually have their own responses."]
    pub struct GoogleCloudVisionV1p3beta1AnnotateFileResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, represents the error message for the failed request. The `responses` field will not be set in this case."]
        pub error: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about the file for which this response is generated."]
        pub input_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p3beta1InputConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Individual responses to images found within the file. This field will be empty if the `error` field is set."]
        pub responses: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1AnnotateImageResponse>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalPages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field gives the total number of pages in the file."]
        pub total_pages: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudVisionV1p3beta1AnnotateFileResponse {
        pub fn builder() -> GoogleCloudVisionV1p3beta1AnnotateFileResponseBuilder {
            GoogleCloudVisionV1p3beta1AnnotateFileResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to an image annotation request."]
    pub struct GoogleCloudVisionV1p3beta1AnnotateImageResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "context")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, contextual information is needed to understand where this image comes from."]
        pub context: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p3beta1ImageAnnotationContext>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cropHintsAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, crop hints have completed successfully."]
        pub crop_hints_annotation:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p3beta1CropHintsAnnotation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, represents the error message for the operation. Note that filled-in image annotations are guaranteed to be correct, even when `error` is set."]
        pub error: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "faceAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, face detection has completed successfully."]
        pub face_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1FaceAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullTextAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, text (OCR) detection or document (OCR) text detection has completed successfully. This annotation provides the structural hierarchy for the OCR detected text."]
        pub full_text_annotation:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p3beta1TextAnnotation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imagePropertiesAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, image properties were extracted successfully."]
        pub image_properties_annotation:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p3beta1ImageProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, label detection has completed successfully."]
        pub label_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1EntityAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "landmarkAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, landmark detection has completed successfully."]
        pub landmark_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1EntityAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localizedObjectAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, localized object detection has completed successfully. This will be sorted descending by confidence score."]
        pub localized_object_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1LocalizedObjectAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logoAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, logo detection has completed successfully."]
        pub logo_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1EntityAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productSearchResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, product search has completed successfully."]
        pub product_search_results: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p3beta1ProductSearchResults>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "safeSearchAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, safe-search annotation has completed successfully."]
        pub safe_search_annotation: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p3beta1SafeSearchAnnotation>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, text (OCR) detection has completed successfully."]
        pub text_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1EntityAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webDetection")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, web detection has completed successfully."]
        pub web_detection:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p3beta1WebDetection>>,
    }
    impl GoogleCloudVisionV1p3beta1AnnotateImageResponse {
        pub fn builder() -> GoogleCloudVisionV1p3beta1AnnotateImageResponseBuilder {
            GoogleCloudVisionV1p3beta1AnnotateImageResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for a single offline file annotation request."]
    pub struct GoogleCloudVisionV1p3beta1AsyncAnnotateFileResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The output location and metadata from AsyncAnnotateFileRequest."]
        pub output_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p3beta1OutputConfig>>,
    }
    impl GoogleCloudVisionV1p3beta1AsyncAnnotateFileResponse {
        pub fn builder() -> GoogleCloudVisionV1p3beta1AsyncAnnotateFileResponseBuilder {
            GoogleCloudVisionV1p3beta1AsyncAnnotateFileResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to an async batch file annotation request."]
    pub struct GoogleCloudVisionV1p3beta1AsyncBatchAnnotateFilesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of file annotation responses, one for each request in AsyncBatchAnnotateFilesRequest."]
        pub responses: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1AsyncAnnotateFileResponse>>,
        >,
    }
    impl GoogleCloudVisionV1p3beta1AsyncBatchAnnotateFilesResponse {
        pub fn builder() -> GoogleCloudVisionV1p3beta1AsyncBatchAnnotateFilesResponseBuilder {
            GoogleCloudVisionV1p3beta1AsyncBatchAnnotateFilesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for the batch operations such as the current state. This is included in the `metadata` field of the `Operation` returned by the `GetOperation` call of the `google::longrunning::Operations` service."]
    pub struct GoogleCloudVisionV1p3beta1BatchOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when the batch request is finished and google.longrunning.Operation.done is set to true."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current state of the batch operation."]
        pub state: ::std::option::Option<GoogleCloudVisionV1p3beta1BatchOperationMetadataStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "submitTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when the batch request was submitted to the server."]
        pub submit_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p3beta1BatchOperationMetadata {
        pub fn builder() -> GoogleCloudVisionV1p3beta1BatchOperationMetadataBuilder {
            GoogleCloudVisionV1p3beta1BatchOperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The current state of the batch operation."]
    pub enum GoogleCloudVisionV1p3beta1BatchOperationMetadataStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Invalid."]
        StateUnspecified,
        #[serde(rename = "PROCESSING")]
        #[doc = "Request is actively being processed."]
        Processing,
        #[serde(rename = "SUCCESSFUL")]
        #[doc = "The request is done and at least one item has been successfully processed."]
        Successful,
        #[serde(rename = "FAILED")]
        #[doc = "The request is done and no item has been successfully processed."]
        Failed,
        #[serde(rename = "CANCELLED")]
        #[doc = "The request is done after the longrunning.Operations.CancelOperation has been called by the user. Any records that were processed before the cancel command are output as specified in the request."]
        Cancelled,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p3beta1BatchOperationMetadataStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Logical element on the page."]
    pub struct GoogleCloudVisionV1p3beta1Block {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blockType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected block type (text, image etc) for this block."]
        pub block_type: ::std::option::Option<GoogleCloudVisionV1p3beta1BlockBlockTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding box for the block. The vertices are in the order of top-left, top-right, bottom-right, bottom-left. When a rotation of the bounding box is detected the rotation is represented as around the top-left corner as defined when the text is read in the 'natural' orientation. For example: * when the text is horizontal it might look like: 0----1 | | 3----2 * when it's rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3)."]
        pub bounding_box:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p3beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the OCR results on the block. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paragraphs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of paragraphs in this block (if this blocks is of type text)."]
        pub paragraphs: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1Paragraph>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information detected for the block."]
        pub property: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p3beta1TextAnnotationTextProperty>,
        >,
    }
    impl GoogleCloudVisionV1p3beta1Block {
        pub fn builder() -> GoogleCloudVisionV1p3beta1BlockBuilder {
            GoogleCloudVisionV1p3beta1BlockBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Detected block type (text, image etc) for this block."]
    pub enum GoogleCloudVisionV1p3beta1BlockBlockTypeEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown block type."]
        Unknown,
        #[serde(rename = "TEXT")]
        #[doc = "Regular text block."]
        Text,
        #[serde(rename = "TABLE")]
        #[doc = "Table block."]
        Table,
        #[serde(rename = "PICTURE")]
        #[doc = "Image block."]
        Picture,
        #[serde(rename = "RULER")]
        #[doc = "Horizontal/vertical line box."]
        Ruler,
        #[serde(rename = "BARCODE")]
        #[doc = "Barcode block."]
        Barcode,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p3beta1BlockBlockTypeEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A bounding polygon for the detected image annotation."]
    pub struct GoogleCloudVisionV1p3beta1BoundingPoly {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedVertices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon normalized vertices."]
        pub normalized_vertices: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1NormalizedVertex>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vertices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon vertices."]
        pub vertices: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1Vertex>>,
        >,
    }
    impl GoogleCloudVisionV1p3beta1BoundingPoly {
        pub fn builder() -> GoogleCloudVisionV1p3beta1BoundingPolyBuilder {
            GoogleCloudVisionV1p3beta1BoundingPolyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Color information consists of RGB channels, score, and the fraction of the image that the color occupies in the image."]
    pub struct GoogleCloudVisionV1p3beta1ColorInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "color")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "RGB components of the color."]
        pub color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pixelFraction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fraction of pixels the color occupies in the image. Value in range [0, 1]."]
        pub pixel_fraction: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image-specific score for this color. Value in range [0, 1]."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p3beta1ColorInfo {
        pub fn builder() -> GoogleCloudVisionV1p3beta1ColorInfoBuilder {
            GoogleCloudVisionV1p3beta1ColorInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Single crop hint that is used to generate a new crop when serving an image."]
    pub struct GoogleCloudVisionV1p3beta1CropHint {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon for the crop region. The coordinates of the bounding box are in the original image's scale."]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p3beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of this being a salient region. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importanceFraction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fraction of importance of this salient region with respect to the original image."]
        pub importance_fraction: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p3beta1CropHint {
        pub fn builder() -> GoogleCloudVisionV1p3beta1CropHintBuilder {
            GoogleCloudVisionV1p3beta1CropHintBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set of crop hints that are used to generate new crops when serving images."]
    pub struct GoogleCloudVisionV1p3beta1CropHintsAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cropHints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Crop hint results."]
        pub crop_hints: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1CropHint>>,
        >,
    }
    impl GoogleCloudVisionV1p3beta1CropHintsAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p3beta1CropHintsAnnotationBuilder {
            GoogleCloudVisionV1p3beta1CropHintsAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set of dominant colors and their corresponding scores."]
    pub struct GoogleCloudVisionV1p3beta1DominantColorsAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "RGB color values with their score and pixel fraction."]
        pub colors: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1ColorInfo>>,
        >,
    }
    impl GoogleCloudVisionV1p3beta1DominantColorsAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p3beta1DominantColorsAnnotationBuilder {
            GoogleCloudVisionV1p3beta1DominantColorsAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set of detected entity features."]
    pub struct GoogleCloudVisionV1p3beta1EntityAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image region to which this entity belongs. Not produced for `LABEL_DETECTION` features."]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p3beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "**Deprecated. Use `score` instead.** The accuracy of the entity detection in an image. For example, for an image in which the \"Eiffel Tower\" entity is detected, this field represents the confidence that there is a tower in the query image. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entity textual description, expressed in its `locale` language."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locale")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language code for the locale in which the entity textual `description` is expressed."]
        pub locale: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location information for the detected entity. Multiple `LocationInfo` elements can be present because one location may indicate the location of the scene in the image, and another location may indicate the location of the place where the image was taken. Location information is usually present for landmarks."]
        pub locations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1LocationInfo>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Opaque entity ID. Some IDs may be available in [Google Knowledge Graph Search API](https://developers.google.com/knowledge-graph/)."]
        pub mid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Some entities may have optional user-supplied `Property` (name/value) fields, such a score or string that qualifies the entity."]
        pub properties: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1Property>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Overall score of the result. Range [0, 1]."]
        pub score: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topicality")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The relevancy of the ICA (Image Content Annotation) label to the image. For example, the relevancy of \"tower\" is likely higher to an image containing the detected \"Eiffel Tower\" than to an image containing a detected distant towering building, even though the confidence that there is a tower in each image may be the same. Range [0, 1]."]
        pub topicality: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p3beta1EntityAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p3beta1EntityAnnotationBuilder {
            GoogleCloudVisionV1p3beta1EntityAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A face annotation object contains the results of face detection."]
    pub struct GoogleCloudVisionV1p3beta1FaceAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "angerLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Anger likelihood."]
        pub anger_likelihood:
            ::std::option::Option<GoogleCloudVisionV1p3beta1FaceAnnotationAngerLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blurredLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Blurred likelihood."]
        pub blurred_likelihood:
            ::std::option::Option<GoogleCloudVisionV1p3beta1FaceAnnotationBlurredLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon around the face. The coordinates of the bounding box are in the original image's scale. The bounding box is computed to \"frame\" the face in accordance with human expectations. It is based on the landmarker results. Note that one or more x and/or y coordinates may not be generated in the `BoundingPoly` (the polygon will be unbounded) if only a partial face appears in the image to be annotated."]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p3beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectionConfidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detection confidence. Range [0, 1]."]
        pub detection_confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fdBoundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `fd_bounding_poly` bounding polygon is tighter than the `boundingPoly`, and encloses only the skin part of the face. Typically, it is used to eliminate the face from any image analysis that detects the \"amount of skin\" visible in an image. It is not based on the landmarker results, only on the initial face detection, hence the fd (face detection) prefix."]
        pub fd_bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p3beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headwearLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Headwear likelihood."]
        pub headwear_likelihood:
            ::std::option::Option<GoogleCloudVisionV1p3beta1FaceAnnotationHeadwearLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "joyLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Joy likelihood."]
        pub joy_likelihood:
            ::std::option::Option<GoogleCloudVisionV1p3beta1FaceAnnotationJoyLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "landmarkingConfidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Face landmarking confidence. Range [0, 1]."]
        pub landmarking_confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "landmarks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected face landmarks."]
        pub landmarks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1FaceAnnotationLandmark>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "panAngle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Yaw angle, which indicates the leftward/rightward angle that the face is pointing relative to the vertical plane perpendicular to the image. Range [-180,180]."]
        pub pan_angle: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rollAngle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Roll angle, which indicates the amount of clockwise/anti-clockwise rotation of the face relative to the image vertical about the axis perpendicular to the face. Range [-180,180]."]
        pub roll_angle: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sorrowLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sorrow likelihood."]
        pub sorrow_likelihood:
            ::std::option::Option<GoogleCloudVisionV1p3beta1FaceAnnotationSorrowLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "surpriseLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Surprise likelihood."]
        pub surprise_likelihood:
            ::std::option::Option<GoogleCloudVisionV1p3beta1FaceAnnotationSurpriseLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tiltAngle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pitch angle, which indicates the upwards/downwards angle that the face is pointing relative to the image's horizontal plane. Range [-180,180]."]
        pub tilt_angle: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "underExposedLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Under-exposed likelihood."]
        pub under_exposed_likelihood: ::std::option::Option<
            GoogleCloudVisionV1p3beta1FaceAnnotationUnderExposedLikelihoodEnum,
        >,
    }
    impl GoogleCloudVisionV1p3beta1FaceAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p3beta1FaceAnnotationBuilder {
            GoogleCloudVisionV1p3beta1FaceAnnotationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Anger likelihood."]
    pub enum GoogleCloudVisionV1p3beta1FaceAnnotationAngerLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p3beta1FaceAnnotationAngerLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Blurred likelihood."]
    pub enum GoogleCloudVisionV1p3beta1FaceAnnotationBlurredLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p3beta1FaceAnnotationBlurredLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Headwear likelihood."]
    pub enum GoogleCloudVisionV1p3beta1FaceAnnotationHeadwearLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p3beta1FaceAnnotationHeadwearLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Joy likelihood."]
    pub enum GoogleCloudVisionV1p3beta1FaceAnnotationJoyLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p3beta1FaceAnnotationJoyLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Sorrow likelihood."]
    pub enum GoogleCloudVisionV1p3beta1FaceAnnotationSorrowLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p3beta1FaceAnnotationSorrowLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Surprise likelihood."]
    pub enum GoogleCloudVisionV1p3beta1FaceAnnotationSurpriseLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p3beta1FaceAnnotationSurpriseLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Under-exposed likelihood."]
    pub enum GoogleCloudVisionV1p3beta1FaceAnnotationUnderExposedLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default
        for GoogleCloudVisionV1p3beta1FaceAnnotationUnderExposedLikelihoodEnum
    {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A face-specific landmark (for example, a face feature)."]
    pub struct GoogleCloudVisionV1p3beta1FaceAnnotationLandmark {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "position")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Face landmark position."]
        pub position: ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p3beta1Position>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Face landmark type."]
        pub _type: ::std::option::Option<GoogleCloudVisionV1p3beta1FaceAnnotationLandmarkTypeEnum>,
    }
    impl GoogleCloudVisionV1p3beta1FaceAnnotationLandmark {
        pub fn builder() -> GoogleCloudVisionV1p3beta1FaceAnnotationLandmarkBuilder {
            GoogleCloudVisionV1p3beta1FaceAnnotationLandmarkBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Face landmark type."]
    pub enum GoogleCloudVisionV1p3beta1FaceAnnotationLandmarkTypeEnum {
        #[serde(rename = "UNKNOWN_LANDMARK")]
        #[doc = "Unknown face landmark detected. Should not be filled."]
        UnknownLandmark,
        #[serde(rename = "LEFT_EYE")]
        #[doc = "Left eye."]
        LeftEye,
        #[serde(rename = "RIGHT_EYE")]
        #[doc = "Right eye."]
        RightEye,
        #[serde(rename = "LEFT_OF_LEFT_EYEBROW")]
        #[doc = "Left of left eyebrow."]
        LeftOfLeftEyebrow,
        #[serde(rename = "RIGHT_OF_LEFT_EYEBROW")]
        #[doc = "Right of left eyebrow."]
        RightOfLeftEyebrow,
        #[serde(rename = "LEFT_OF_RIGHT_EYEBROW")]
        #[doc = "Left of right eyebrow."]
        LeftOfRightEyebrow,
        #[serde(rename = "RIGHT_OF_RIGHT_EYEBROW")]
        #[doc = "Right of right eyebrow."]
        RightOfRightEyebrow,
        #[serde(rename = "MIDPOINT_BETWEEN_EYES")]
        #[doc = "Midpoint between eyes."]
        MidpointBetweenEyes,
        #[serde(rename = "NOSE_TIP")]
        #[doc = "Nose tip."]
        NoseTip,
        #[serde(rename = "UPPER_LIP")]
        #[doc = "Upper lip."]
        UpperLip,
        #[serde(rename = "LOWER_LIP")]
        #[doc = "Lower lip."]
        LowerLip,
        #[serde(rename = "MOUTH_LEFT")]
        #[doc = "Mouth left."]
        MouthLeft,
        #[serde(rename = "MOUTH_RIGHT")]
        #[doc = "Mouth right."]
        MouthRight,
        #[serde(rename = "MOUTH_CENTER")]
        #[doc = "Mouth center."]
        MouthCenter,
        #[serde(rename = "NOSE_BOTTOM_RIGHT")]
        #[doc = "Nose, bottom right."]
        NoseBottomRight,
        #[serde(rename = "NOSE_BOTTOM_LEFT")]
        #[doc = "Nose, bottom left."]
        NoseBottomLeft,
        #[serde(rename = "NOSE_BOTTOM_CENTER")]
        #[doc = "Nose, bottom center."]
        NoseBottomCenter,
        #[serde(rename = "LEFT_EYE_TOP_BOUNDARY")]
        #[doc = "Left eye, top boundary."]
        LeftEyeTopBoundary,
        #[serde(rename = "LEFT_EYE_RIGHT_CORNER")]
        #[doc = "Left eye, right corner."]
        LeftEyeRightCorner,
        #[serde(rename = "LEFT_EYE_BOTTOM_BOUNDARY")]
        #[doc = "Left eye, bottom boundary."]
        LeftEyeBottomBoundary,
        #[serde(rename = "LEFT_EYE_LEFT_CORNER")]
        #[doc = "Left eye, left corner."]
        LeftEyeLeftCorner,
        #[serde(rename = "RIGHT_EYE_TOP_BOUNDARY")]
        #[doc = "Right eye, top boundary."]
        RightEyeTopBoundary,
        #[serde(rename = "RIGHT_EYE_RIGHT_CORNER")]
        #[doc = "Right eye, right corner."]
        RightEyeRightCorner,
        #[serde(rename = "RIGHT_EYE_BOTTOM_BOUNDARY")]
        #[doc = "Right eye, bottom boundary."]
        RightEyeBottomBoundary,
        #[serde(rename = "RIGHT_EYE_LEFT_CORNER")]
        #[doc = "Right eye, left corner."]
        RightEyeLeftCorner,
        #[serde(rename = "LEFT_EYEBROW_UPPER_MIDPOINT")]
        #[doc = "Left eyebrow, upper midpoint."]
        LeftEyebrowUpperMidpoint,
        #[serde(rename = "RIGHT_EYEBROW_UPPER_MIDPOINT")]
        #[doc = "Right eyebrow, upper midpoint."]
        RightEyebrowUpperMidpoint,
        #[serde(rename = "LEFT_EAR_TRAGION")]
        #[doc = "Left ear tragion."]
        LeftEarTragion,
        #[serde(rename = "RIGHT_EAR_TRAGION")]
        #[doc = "Right ear tragion."]
        RightEarTragion,
        #[serde(rename = "LEFT_EYE_PUPIL")]
        #[doc = "Left eye pupil."]
        LeftEyePupil,
        #[serde(rename = "RIGHT_EYE_PUPIL")]
        #[doc = "Right eye pupil."]
        RightEyePupil,
        #[serde(rename = "FOREHEAD_GLABELLA")]
        #[doc = "Forehead glabella."]
        ForeheadGlabella,
        #[serde(rename = "CHIN_GNATHION")]
        #[doc = "Chin gnathion."]
        ChinGnathion,
        #[serde(rename = "CHIN_LEFT_GONION")]
        #[doc = "Chin left gonion."]
        ChinLeftGonion,
        #[serde(rename = "CHIN_RIGHT_GONION")]
        #[doc = "Chin right gonion."]
        ChinRightGonion,
        #[serde(rename = "LEFT_CHEEK_CENTER")]
        #[doc = "Left cheek center."]
        LeftCheekCenter,
        #[serde(rename = "RIGHT_CHEEK_CENTER")]
        #[doc = "Right cheek center."]
        RightCheekCenter,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p3beta1FaceAnnotationLandmarkTypeEnum {
        fn default() -> Self {
            Self::UnknownLandmark
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Google Cloud Storage location where the output will be written to."]
    pub struct GoogleCloudVisionV1p3beta1GcsDestination {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage URI prefix where the results will be stored. Results will be in JSON format and preceded by its corresponding input URI prefix. This field can either represent a gcs file prefix or gcs directory. In either case, the uri should be unique because in order to get all of the output files, you will need to do a wildcard gcs search on the uri prefix you provide. Examples: * File Prefix: gs://bucket-name/here/filenameprefix The output files will be created in gs://bucket-name/here/ and the names of the output files will begin with \"filenameprefix\". * Directory Prefix: gs://bucket-name/some/location/ The output files will be created in gs://bucket-name/some/location/ and the names of the output files could be anything because there was no filename prefix specified. If multiple outputs, each response is still AnnotateFileResponse, each of which contains some subset of the full list of AnnotateImageResponse. Multiple outputs can happen if, for example, the output JSON is too large and overflows into multiple sharded files."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p3beta1GcsDestination {
        pub fn builder() -> GoogleCloudVisionV1p3beta1GcsDestinationBuilder {
            GoogleCloudVisionV1p3beta1GcsDestinationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Google Cloud Storage location where the input will be read from."]
    pub struct GoogleCloudVisionV1p3beta1GcsSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage URI for the input file. This must only be a Google Cloud Storage object. Wildcards are not currently supported."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p3beta1GcsSource {
        pub fn builder() -> GoogleCloudVisionV1p3beta1GcsSourceBuilder {
            GoogleCloudVisionV1p3beta1GcsSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "If an image was produced from a file (e.g. a PDF), this message gives information about the source of that image."]
    pub struct GoogleCloudVisionV1p3beta1ImageAnnotationContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the file was a PDF or TIFF, this field gives the page number within the file used to produce the image."]
        pub page_number: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI of the file used to produce the image."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p3beta1ImageAnnotationContext {
        pub fn builder() -> GoogleCloudVisionV1p3beta1ImageAnnotationContextBuilder {
            GoogleCloudVisionV1p3beta1ImageAnnotationContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Stores image properties, such as dominant colors."]
    pub struct GoogleCloudVisionV1p3beta1ImageProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dominantColors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, dominant colors completed successfully."]
        pub dominant_colors: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p3beta1DominantColorsAnnotation>,
        >,
    }
    impl GoogleCloudVisionV1p3beta1ImageProperties {
        pub fn builder() -> GoogleCloudVisionV1p3beta1ImagePropertiesBuilder {
            GoogleCloudVisionV1p3beta1ImagePropertiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for the `ImportProductSets` method. This message is returned by the google.longrunning.Operations.GetOperation method in the returned google.longrunning.Operation.response field."]
    pub struct GoogleCloudVisionV1p3beta1ImportProductSetsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "referenceImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of reference_images that are imported successfully."]
        pub reference_images: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1ReferenceImage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statuses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The rpc status for each ImportProductSet request, including both successes and errors. The number of statuses here matches the number of lines in the csv file, and statuses[i] stores the success or failure status of processing the i-th line of the csv, starting from line 0."]
        pub statuses: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Status>>>,
    }
    impl GoogleCloudVisionV1p3beta1ImportProductSetsResponse {
        pub fn builder() -> GoogleCloudVisionV1p3beta1ImportProductSetsResponseBuilder {
            GoogleCloudVisionV1p3beta1ImportProductSetsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The desired input location and metadata."]
    pub struct GoogleCloudVisionV1p3beta1InputConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "File content, represented as a stream of bytes. Note: As with all `bytes` fields, protobuffers use a pure binary representation, whereas JSON representations use base64. Currently, this field only works for BatchAnnotateFiles requests. It does not work for AsyncBatchAnnotateFiles requests."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Google Cloud Storage location to read the input from."]
        pub gcs_source:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p3beta1GcsSource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the file. Currently only \"application/pdf\", \"image/tiff\" and \"image/gif\" are supported. Wildcards are not supported."]
        pub mime_type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p3beta1InputConfig {
        pub fn builder() -> GoogleCloudVisionV1p3beta1InputConfigBuilder {
            GoogleCloudVisionV1p3beta1InputConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set of detected objects with bounding boxes."]
    pub struct GoogleCloudVisionV1p3beta1LocalizedObjectAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image region to which this object belongs. This must be populated."]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p3beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object ID that should align with EntityAnnotation mid."]
        pub mid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object name, expressed in its `language_code` language."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Score of the result. Range [0, 1]."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p3beta1LocalizedObjectAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p3beta1LocalizedObjectAnnotationBuilder {
            GoogleCloudVisionV1p3beta1LocalizedObjectAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected entity location information."]
    pub struct GoogleCloudVisionV1p3beta1LocationInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "latLng")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "lat/long location coordinates."]
        pub lat_lng: ::std::option::Option<::std::boxed::Box<LatLng>>,
    }
    impl GoogleCloudVisionV1p3beta1LocationInfo {
        pub fn builder() -> GoogleCloudVisionV1p3beta1LocationInfoBuilder {
            GoogleCloudVisionV1p3beta1LocationInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A vertex represents a 2D point in the image. NOTE: the normalized vertex coordinates are relative to the original image and range from 0 to 1."]
    pub struct GoogleCloudVisionV1p3beta1NormalizedVertex {
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
    impl GoogleCloudVisionV1p3beta1NormalizedVertex {
        pub fn builder() -> GoogleCloudVisionV1p3beta1NormalizedVertexBuilder {
            GoogleCloudVisionV1p3beta1NormalizedVertexBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains metadata for the BatchAnnotateImages operation."]
    pub struct GoogleCloudVisionV1p3beta1OperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when the batch request was received."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Current state of the batch operation."]
        pub state: ::std::option::Option<GoogleCloudVisionV1p3beta1OperationMetadataStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when the operation result was last updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p3beta1OperationMetadata {
        pub fn builder() -> GoogleCloudVisionV1p3beta1OperationMetadataBuilder {
            GoogleCloudVisionV1p3beta1OperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Current state of the batch operation."]
    pub enum GoogleCloudVisionV1p3beta1OperationMetadataStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Invalid."]
        StateUnspecified,
        #[serde(rename = "CREATED")]
        #[doc = "Request is received."]
        Created,
        #[serde(rename = "RUNNING")]
        #[doc = "Request is actively being processed."]
        Running,
        #[serde(rename = "DONE")]
        #[doc = "The batch processing is done."]
        Done,
        #[serde(rename = "CANCELLED")]
        #[doc = "The batch processing was cancelled."]
        Cancelled,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p3beta1OperationMetadataStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The desired output location and metadata."]
    pub struct GoogleCloudVisionV1p3beta1OutputConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The max number of response protos to put into each output JSON file on Google Cloud Storage. The valid range is [1, 100]. If not specified, the default value is 20. For example, for one pdf file with 100 pages, 100 response protos will be generated. If `batch_size` = 20, then 5 json files each containing 20 response protos will be written under the prefix `gcs_destination`.`uri`. Currently, batch_size only applies to GcsDestination, with potential future support for other output configurations."]
        pub batch_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsDestination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Google Cloud Storage location to write the output(s) to."]
        pub gcs_destination:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p3beta1GcsDestination>>,
    }
    impl GoogleCloudVisionV1p3beta1OutputConfig {
        pub fn builder() -> GoogleCloudVisionV1p3beta1OutputConfigBuilder {
            GoogleCloudVisionV1p3beta1OutputConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected page from OCR."]
    pub struct GoogleCloudVisionV1p3beta1Page {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blocks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of blocks of text, images etc on this page."]
        pub blocks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1Block>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the OCR results on the page. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page height. For PDFs the unit is points. For images (including TIFFs) the unit is pixels."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information detected on the page."]
        pub property: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p3beta1TextAnnotationTextProperty>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page width. For PDFs the unit is points. For images (including TIFFs) the unit is pixels."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudVisionV1p3beta1Page {
        pub fn builder() -> GoogleCloudVisionV1p3beta1PageBuilder {
            GoogleCloudVisionV1p3beta1PageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Structural unit of text representing a number of words in certain order."]
    pub struct GoogleCloudVisionV1p3beta1Paragraph {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding box for the paragraph. The vertices are in the order of top-left, top-right, bottom-right, bottom-left. When a rotation of the bounding box is detected the rotation is represented as around the top-left corner as defined when the text is read in the 'natural' orientation. For example: * when the text is horizontal it might look like: 0----1 | | 3----2 * when it's rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3)."]
        pub bounding_box:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p3beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the OCR results for the paragraph. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information detected for the paragraph."]
        pub property: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p3beta1TextAnnotationTextProperty>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "words")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of all words in this paragraph."]
        pub words: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1Word>>,
        >,
    }
    impl GoogleCloudVisionV1p3beta1Paragraph {
        pub fn builder() -> GoogleCloudVisionV1p3beta1ParagraphBuilder {
            GoogleCloudVisionV1p3beta1ParagraphBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A 3D position in the image, used primarily for Face detection landmarks. A valid Position must have both x and y coordinates. The position coordinates are in the same scale as the original image."]
    pub struct GoogleCloudVisionV1p3beta1Position {
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
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "z")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Z coordinate (or depth)."]
        pub z: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p3beta1Position {
        pub fn builder() -> GoogleCloudVisionV1p3beta1PositionBuilder {
            GoogleCloudVisionV1p3beta1PositionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Product contains ReferenceImages."]
    pub struct GoogleCloudVisionV1p3beta1Product {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User-provided metadata to be stored with this product. Must be at most 4096 characters long."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user-provided name for this Product. Must not be empty. Must be at most 4096 characters long."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the product. Format is: `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`. This field is ignored when creating a product."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productCategory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Immutable. The category for the product identified by the reference image. This should be one of \"homegoods-v2\", \"apparel-v2\", \"toys-v2\", \"packagedgoods-v1\" or \"general-v1\". The legacy categories \"homegoods\", \"apparel\", and \"toys\" are still supported, but these should not be used for new products."]
        pub product_category: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Key-value pairs that can be attached to a product. At query time, constraints can be specified based on the product_labels. Note that integer values can be provided as strings, e.g. \"1199\". Only strings with integer values can match a range-based restriction which is to be supported soon. Multiple values can be assigned to the same key. One product may have up to 500 product_labels. Notice that the total number of distinct product_labels over all products in one ProductSet cannot exceed 1M, otherwise the product search pipeline will refuse to work for that ProductSet."]
        pub product_labels: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1ProductKeyValue>>,
        >,
    }
    impl GoogleCloudVisionV1p3beta1Product {
        pub fn builder() -> GoogleCloudVisionV1p3beta1ProductBuilder {
            GoogleCloudVisionV1p3beta1ProductBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A product label represented as a key-value pair."]
    pub struct GoogleCloudVisionV1p3beta1ProductKeyValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The key of the label attached to the product. Cannot be empty and cannot exceed 128 bytes."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value of the label attached to the product. Cannot be empty and cannot exceed 128 bytes."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p3beta1ProductKeyValue {
        pub fn builder() -> GoogleCloudVisionV1p3beta1ProductKeyValueBuilder {
            GoogleCloudVisionV1p3beta1ProductKeyValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Results for a product search request."]
    pub struct GoogleCloudVisionV1p3beta1ProductSearchResults {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "indexTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp of the index which provided these results. Products added to the product set and products removed from the product set after this time are not reflected in the current results."]
        pub index_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productGroupedResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of results grouped by products detected in the query image. Each entry corresponds to one bounding polygon in the query image, and contains the matching products specific to that region. There may be duplicate product matches in the union of all the per-product results."]
        pub product_grouped_results: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVisionV1p3beta1ProductSearchResultsGroupedResult>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of results, one for each product match."]
        pub results: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVisionV1p3beta1ProductSearchResultsResult>,
            >,
        >,
    }
    impl GoogleCloudVisionV1p3beta1ProductSearchResults {
        pub fn builder() -> GoogleCloudVisionV1p3beta1ProductSearchResultsBuilder {
            GoogleCloudVisionV1p3beta1ProductSearchResultsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about the products similar to a single product in a query image."]
    pub struct GoogleCloudVisionV1p3beta1ProductSearchResultsGroupedResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon around the product detected in the query image."]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p3beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of generic predictions for the object in the bounding box."]
        pub object_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVisionV1p3beta1ProductSearchResultsObjectAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of results, one for each product match."]
        pub results: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVisionV1p3beta1ProductSearchResultsResult>,
            >,
        >,
    }
    impl GoogleCloudVisionV1p3beta1ProductSearchResultsGroupedResult {
        pub fn builder() -> GoogleCloudVisionV1p3beta1ProductSearchResultsGroupedResultBuilder {
            GoogleCloudVisionV1p3beta1ProductSearchResultsGroupedResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Prediction for what the object in the bounding box is."]
    pub struct GoogleCloudVisionV1p3beta1ProductSearchResultsObjectAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object ID that should align with EntityAnnotation mid."]
        pub mid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object name, expressed in its `language_code` language."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Score of the result. Range [0, 1]."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p3beta1ProductSearchResultsObjectAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p3beta1ProductSearchResultsObjectAnnotationBuilder {
            GoogleCloudVisionV1p3beta1ProductSearchResultsObjectAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a product."]
    pub struct GoogleCloudVisionV1p3beta1ProductSearchResultsResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the image from the product that is the closest match to the query."]
        pub image: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "product")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Product."]
        pub product: ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p3beta1Product>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A confidence level on the match, ranging from 0 (no confidence) to 1 (full confidence)."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p3beta1ProductSearchResultsResult {
        pub fn builder() -> GoogleCloudVisionV1p3beta1ProductSearchResultsResultBuilder {
            GoogleCloudVisionV1p3beta1ProductSearchResultsResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A `Property` consists of a user-supplied name/value pair."]
    pub struct GoogleCloudVisionV1p3beta1Property {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the property."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uint64Value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value of numeric properties."]
        pub uint64_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value of the property."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p3beta1Property {
        pub fn builder() -> GoogleCloudVisionV1p3beta1PropertyBuilder {
            GoogleCloudVisionV1p3beta1PropertyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A `ReferenceImage` represents a product image and its associated metadata, such as bounding boxes."]
    pub struct GoogleCloudVisionV1p3beta1ReferenceImage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPolys")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Bounding polygons around the areas of interest in the reference image. If this field is empty, the system will try to detect regions of interest. At most 10 bounding polygons will be used. The provided shape is converted into a non-rotated rectangle. Once converted, the small edge of the rectangle must be greater than or equal to 300 pixels. The aspect ratio must be 1:4 or less (i.e. 1:3 is ok; 1:5 is not)."]
        pub bounding_polys: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1BoundingPoly>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the reference image. Format is: `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID/referenceImages/IMAGE_ID`. This field is ignored when creating a reference image."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The Google Cloud Storage URI of the reference image. The URI must start with `gs://`."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p3beta1ReferenceImage {
        pub fn builder() -> GoogleCloudVisionV1p3beta1ReferenceImageBuilder {
            GoogleCloudVisionV1p3beta1ReferenceImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set of features pertaining to the image, computed by computer vision methods over safe-search verticals (for example, adult, spoof, medical, violence)."]
    pub struct GoogleCloudVisionV1p3beta1SafeSearchAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents the adult content likelihood for the image. Adult content may contain elements such as nudity, pornographic images or cartoons, or sexual activities."]
        pub adult: ::std::option::Option<GoogleCloudVisionV1p3beta1SafeSearchAnnotationAdultEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "medical")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Likelihood that this is a medical image."]
        pub medical:
            ::std::option::Option<GoogleCloudVisionV1p3beta1SafeSearchAnnotationMedicalEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "racy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Likelihood that the request image contains racy content. Racy content may include (but is not limited to) skimpy or sheer clothing, strategically covered nudity, lewd or provocative poses, or close-ups of sensitive body areas."]
        pub racy: ::std::option::Option<GoogleCloudVisionV1p3beta1SafeSearchAnnotationRacyEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spoof")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Spoof likelihood. The likelihood that an modification was made to the image's canonical version to make it appear funny or offensive."]
        pub spoof: ::std::option::Option<GoogleCloudVisionV1p3beta1SafeSearchAnnotationSpoofEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "violence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Likelihood that this image contains violent content."]
        pub violence:
            ::std::option::Option<GoogleCloudVisionV1p3beta1SafeSearchAnnotationViolenceEnum>,
    }
    impl GoogleCloudVisionV1p3beta1SafeSearchAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p3beta1SafeSearchAnnotationBuilder {
            GoogleCloudVisionV1p3beta1SafeSearchAnnotationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Represents the adult content likelihood for the image. Adult content may contain elements such as nudity, pornographic images or cartoons, or sexual activities."]
    pub enum GoogleCloudVisionV1p3beta1SafeSearchAnnotationAdultEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p3beta1SafeSearchAnnotationAdultEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Likelihood that this is a medical image."]
    pub enum GoogleCloudVisionV1p3beta1SafeSearchAnnotationMedicalEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p3beta1SafeSearchAnnotationMedicalEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Likelihood that the request image contains racy content. Racy content may include (but is not limited to) skimpy or sheer clothing, strategically covered nudity, lewd or provocative poses, or close-ups of sensitive body areas."]
    pub enum GoogleCloudVisionV1p3beta1SafeSearchAnnotationRacyEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p3beta1SafeSearchAnnotationRacyEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Spoof likelihood. The likelihood that an modification was made to the image's canonical version to make it appear funny or offensive."]
    pub enum GoogleCloudVisionV1p3beta1SafeSearchAnnotationSpoofEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p3beta1SafeSearchAnnotationSpoofEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Likelihood that this image contains violent content."]
    pub enum GoogleCloudVisionV1p3beta1SafeSearchAnnotationViolenceEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p3beta1SafeSearchAnnotationViolenceEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single symbol representation."]
    pub struct GoogleCloudVisionV1p3beta1Symbol {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding box for the symbol. The vertices are in the order of top-left, top-right, bottom-right, bottom-left. When a rotation of the bounding box is detected the rotation is represented as around the top-left corner as defined when the text is read in the 'natural' orientation. For example: * when the text is horizontal it might look like: 0----1 | | 3----2 * when it's rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3)."]
        pub bounding_box:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p3beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the OCR results for the symbol. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information detected for the symbol."]
        pub property: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p3beta1TextAnnotationTextProperty>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The actual UTF-8 representation of the symbol."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p3beta1Symbol {
        pub fn builder() -> GoogleCloudVisionV1p3beta1SymbolBuilder {
            GoogleCloudVisionV1p3beta1SymbolBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "TextAnnotation contains a structured representation of OCR extracted text. The hierarchy of an OCR extracted text structure is like this: TextAnnotation -> Page -> Block -> Paragraph -> Word -> Symbol Each structural component, starting from Page, may further have their own properties. Properties describe detected languages, breaks etc.. Please refer to the TextAnnotation.TextProperty message definition below for more detail."]
    pub struct GoogleCloudVisionV1p3beta1TextAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of pages detected by OCR."]
        pub pages: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1Page>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "UTF-8 text detected on the pages."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p3beta1TextAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p3beta1TextAnnotationBuilder {
            GoogleCloudVisionV1p3beta1TextAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected start or end of a structural component."]
    pub struct GoogleCloudVisionV1p3beta1TextAnnotationDetectedBreak {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isPrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if break prepends the element."]
        pub is_prefix: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected break type."]
        pub _type:
            ::std::option::Option<GoogleCloudVisionV1p3beta1TextAnnotationDetectedBreakTypeEnum>,
    }
    impl GoogleCloudVisionV1p3beta1TextAnnotationDetectedBreak {
        pub fn builder() -> GoogleCloudVisionV1p3beta1TextAnnotationDetectedBreakBuilder {
            GoogleCloudVisionV1p3beta1TextAnnotationDetectedBreakBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Detected break type."]
    pub enum GoogleCloudVisionV1p3beta1TextAnnotationDetectedBreakTypeEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown break label type."]
        Unknown,
        #[serde(rename = "SPACE")]
        #[doc = "Regular space."]
        Space,
        #[serde(rename = "SURE_SPACE")]
        #[doc = "Sure space (very wide)."]
        SureSpace,
        #[serde(rename = "EOL_SURE_SPACE")]
        #[doc = "Line-wrapping break."]
        EolSureSpace,
        #[serde(rename = "HYPHEN")]
        #[doc = "End-line hyphen that is not present in text; does not co-occur with `SPACE`, `LEADER_SPACE`, or `LINE_BREAK`."]
        Hyphen,
        #[serde(rename = "LINE_BREAK")]
        #[doc = "Line break that ends a paragraph."]
        LineBreak,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p3beta1TextAnnotationDetectedBreakTypeEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected language for a structural component."]
    pub struct GoogleCloudVisionV1p3beta1TextAnnotationDetectedLanguage {
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
    impl GoogleCloudVisionV1p3beta1TextAnnotationDetectedLanguage {
        pub fn builder() -> GoogleCloudVisionV1p3beta1TextAnnotationDetectedLanguageBuilder {
            GoogleCloudVisionV1p3beta1TextAnnotationDetectedLanguageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional information detected on the structural component."]
    pub struct GoogleCloudVisionV1p3beta1TextAnnotationTextProperty {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedBreak")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected start or end of a text segment."]
        pub detected_break: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p3beta1TextAnnotationDetectedBreak>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected languages together with confidence."]
        pub detected_languages: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVisionV1p3beta1TextAnnotationDetectedLanguage>,
            >,
        >,
    }
    impl GoogleCloudVisionV1p3beta1TextAnnotationTextProperty {
        pub fn builder() -> GoogleCloudVisionV1p3beta1TextAnnotationTextPropertyBuilder {
            GoogleCloudVisionV1p3beta1TextAnnotationTextPropertyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A vertex represents a 2D point in the image. NOTE: the vertex coordinates are in the same scale as the original image."]
    pub struct GoogleCloudVisionV1p3beta1Vertex {
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
    impl GoogleCloudVisionV1p3beta1Vertex {
        pub fn builder() -> GoogleCloudVisionV1p3beta1VertexBuilder {
            GoogleCloudVisionV1p3beta1VertexBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Relevant information for the image from the Internet."]
    pub struct GoogleCloudVisionV1p3beta1WebDetection {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bestGuessLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The service's best guess as to the topic of the request image. Inferred from similar images on the open web."]
        pub best_guess_labels: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1WebDetectionWebLabel>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullMatchingImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fully matching images from the Internet. Can include resized copies of the query image."]
        pub full_matching_images: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1WebDetectionWebImage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pagesWithMatchingImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web pages containing the matching images from the Internet."]
        pub pages_with_matching_images: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1WebDetectionWebPage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialMatchingImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Partial matching images from the Internet. Those images are similar enough to share some key-point features. For example an original image will likely have partial matching for its crops."]
        pub partial_matching_images: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1WebDetectionWebImage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visuallySimilarImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visually similar image results."]
        pub visually_similar_images: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1WebDetectionWebImage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webEntities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deduced entities from similar images on the Internet."]
        pub web_entities: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1WebDetectionWebEntity>>,
        >,
    }
    impl GoogleCloudVisionV1p3beta1WebDetection {
        pub fn builder() -> GoogleCloudVisionV1p3beta1WebDetectionBuilder {
            GoogleCloudVisionV1p3beta1WebDetectionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Entity deduced from similar images on the Internet."]
    pub struct GoogleCloudVisionV1p3beta1WebDetectionWebEntity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Canonical description of the entity, in English."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Opaque entity ID."]
        pub entity_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Overall relevancy score for the entity. Not normalized and not comparable across different image queries."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p3beta1WebDetectionWebEntity {
        pub fn builder() -> GoogleCloudVisionV1p3beta1WebDetectionWebEntityBuilder {
            GoogleCloudVisionV1p3beta1WebDetectionWebEntityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for online images."]
    pub struct GoogleCloudVisionV1p3beta1WebDetectionWebImage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "(Deprecated) Overall relevancy score for the image."]
        pub score: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result image URL."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p3beta1WebDetectionWebImage {
        pub fn builder() -> GoogleCloudVisionV1p3beta1WebDetectionWebImageBuilder {
            GoogleCloudVisionV1p3beta1WebDetectionWebImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Label to provide extra metadata for the web detection."]
    pub struct GoogleCloudVisionV1p3beta1WebDetectionWebLabel {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "label")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label for extra metadata."]
        pub label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The BCP-47 language code for `label`, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        pub language_code: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p3beta1WebDetectionWebLabel {
        pub fn builder() -> GoogleCloudVisionV1p3beta1WebDetectionWebLabelBuilder {
            GoogleCloudVisionV1p3beta1WebDetectionWebLabelBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for web pages."]
    pub struct GoogleCloudVisionV1p3beta1WebDetectionWebPage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullMatchingImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fully matching images on the page. Can include resized copies of the query image."]
        pub full_matching_images: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1WebDetectionWebImage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title for the web page, may contain HTML markups."]
        pub page_title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialMatchingImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Partial matching images on the page. Those images are similar enough to share some key-point features. For example an original image will likely have partial matching for its crops."]
        pub partial_matching_images: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1WebDetectionWebImage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "(Deprecated) Overall relevancy score for the web page."]
        pub score: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result web page URL."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p3beta1WebDetectionWebPage {
        pub fn builder() -> GoogleCloudVisionV1p3beta1WebDetectionWebPageBuilder {
            GoogleCloudVisionV1p3beta1WebDetectionWebPageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A word representation."]
    pub struct GoogleCloudVisionV1p3beta1Word {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding box for the word. The vertices are in the order of top-left, top-right, bottom-right, bottom-left. When a rotation of the bounding box is detected the rotation is represented as around the top-left corner as defined when the text is read in the 'natural' orientation. For example: * when the text is horizontal it might look like: 0----1 | | 3----2 * when it's rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3)."]
        pub bounding_box:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p3beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the OCR results for the word. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information detected for the word."]
        pub property: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p3beta1TextAnnotationTextProperty>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "symbols")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of symbols in the word. The order of the symbols follows the natural reading order."]
        pub symbols: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p3beta1Symbol>>,
        >,
    }
    impl GoogleCloudVisionV1p3beta1Word {
        pub fn builder() -> GoogleCloudVisionV1p3beta1WordBuilder {
            GoogleCloudVisionV1p3beta1WordBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to a single file annotation request. A file may contain one or more images, which individually have their own responses."]
    pub struct GoogleCloudVisionV1p4beta1AnnotateFileResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, represents the error message for the failed request. The `responses` field will not be set in this case."]
        pub error: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about the file for which this response is generated."]
        pub input_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p4beta1InputConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Individual responses to images found within the file. This field will be empty if the `error` field is set."]
        pub responses: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1AnnotateImageResponse>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalPages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field gives the total number of pages in the file."]
        pub total_pages: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudVisionV1p4beta1AnnotateFileResponse {
        pub fn builder() -> GoogleCloudVisionV1p4beta1AnnotateFileResponseBuilder {
            GoogleCloudVisionV1p4beta1AnnotateFileResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to an image annotation request."]
    pub struct GoogleCloudVisionV1p4beta1AnnotateImageResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "context")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, contextual information is needed to understand where this image comes from."]
        pub context: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p4beta1ImageAnnotationContext>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cropHintsAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, crop hints have completed successfully."]
        pub crop_hints_annotation:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p4beta1CropHintsAnnotation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, represents the error message for the operation. Note that filled-in image annotations are guaranteed to be correct, even when `error` is set."]
        pub error: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "faceAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, face detection has completed successfully."]
        pub face_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1FaceAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullTextAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, text (OCR) detection or document (OCR) text detection has completed successfully. This annotation provides the structural hierarchy for the OCR detected text."]
        pub full_text_annotation:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p4beta1TextAnnotation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imagePropertiesAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, image properties were extracted successfully."]
        pub image_properties_annotation:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p4beta1ImageProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, label detection has completed successfully."]
        pub label_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1EntityAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "landmarkAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, landmark detection has completed successfully."]
        pub landmark_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1EntityAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localizedObjectAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, localized object detection has completed successfully. This will be sorted descending by confidence score."]
        pub localized_object_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1LocalizedObjectAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logoAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, logo detection has completed successfully."]
        pub logo_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1EntityAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productSearchResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, product search has completed successfully."]
        pub product_search_results: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p4beta1ProductSearchResults>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "safeSearchAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, safe-search annotation has completed successfully."]
        pub safe_search_annotation: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p4beta1SafeSearchAnnotation>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, text (OCR) detection has completed successfully."]
        pub text_annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1EntityAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webDetection")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, web detection has completed successfully."]
        pub web_detection:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p4beta1WebDetection>>,
    }
    impl GoogleCloudVisionV1p4beta1AnnotateImageResponse {
        pub fn builder() -> GoogleCloudVisionV1p4beta1AnnotateImageResponseBuilder {
            GoogleCloudVisionV1p4beta1AnnotateImageResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response for a single offline file annotation request."]
    pub struct GoogleCloudVisionV1p4beta1AsyncAnnotateFileResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The output location and metadata from AsyncAnnotateFileRequest."]
        pub output_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p4beta1OutputConfig>>,
    }
    impl GoogleCloudVisionV1p4beta1AsyncAnnotateFileResponse {
        pub fn builder() -> GoogleCloudVisionV1p4beta1AsyncAnnotateFileResponseBuilder {
            GoogleCloudVisionV1p4beta1AsyncAnnotateFileResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to an async batch file annotation request."]
    pub struct GoogleCloudVisionV1p4beta1AsyncBatchAnnotateFilesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of file annotation responses, one for each request in AsyncBatchAnnotateFilesRequest."]
        pub responses: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1AsyncAnnotateFileResponse>>,
        >,
    }
    impl GoogleCloudVisionV1p4beta1AsyncBatchAnnotateFilesResponse {
        pub fn builder() -> GoogleCloudVisionV1p4beta1AsyncBatchAnnotateFilesResponseBuilder {
            GoogleCloudVisionV1p4beta1AsyncBatchAnnotateFilesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to an async batch image annotation request."]
    pub struct GoogleCloudVisionV1p4beta1AsyncBatchAnnotateImagesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The output location and metadata from AsyncBatchAnnotateImagesRequest."]
        pub output_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p4beta1OutputConfig>>,
    }
    impl GoogleCloudVisionV1p4beta1AsyncBatchAnnotateImagesResponse {
        pub fn builder() -> GoogleCloudVisionV1p4beta1AsyncBatchAnnotateImagesResponseBuilder {
            GoogleCloudVisionV1p4beta1AsyncBatchAnnotateImagesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of file annotation responses."]
    pub struct GoogleCloudVisionV1p4beta1BatchAnnotateFilesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of file annotation responses, each response corresponding to each AnnotateFileRequest in BatchAnnotateFilesRequest."]
        pub responses: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1AnnotateFileResponse>>,
        >,
    }
    impl GoogleCloudVisionV1p4beta1BatchAnnotateFilesResponse {
        pub fn builder() -> GoogleCloudVisionV1p4beta1BatchAnnotateFilesResponseBuilder {
            GoogleCloudVisionV1p4beta1BatchAnnotateFilesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for the batch operations such as the current state. This is included in the `metadata` field of the `Operation` returned by the `GetOperation` call of the `google::longrunning::Operations` service."]
    pub struct GoogleCloudVisionV1p4beta1BatchOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when the batch request is finished and google.longrunning.Operation.done is set to true."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current state of the batch operation."]
        pub state: ::std::option::Option<GoogleCloudVisionV1p4beta1BatchOperationMetadataStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "submitTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when the batch request was submitted to the server."]
        pub submit_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p4beta1BatchOperationMetadata {
        pub fn builder() -> GoogleCloudVisionV1p4beta1BatchOperationMetadataBuilder {
            GoogleCloudVisionV1p4beta1BatchOperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The current state of the batch operation."]
    pub enum GoogleCloudVisionV1p4beta1BatchOperationMetadataStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Invalid."]
        StateUnspecified,
        #[serde(rename = "PROCESSING")]
        #[doc = "Request is actively being processed."]
        Processing,
        #[serde(rename = "SUCCESSFUL")]
        #[doc = "The request is done and at least one item has been successfully processed."]
        Successful,
        #[serde(rename = "FAILED")]
        #[doc = "The request is done and no item has been successfully processed."]
        Failed,
        #[serde(rename = "CANCELLED")]
        #[doc = "The request is done after the longrunning.Operations.CancelOperation has been called by the user. Any records that were processed before the cancel command are output as specified in the request."]
        Cancelled,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p4beta1BatchOperationMetadataStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Logical element on the page."]
    pub struct GoogleCloudVisionV1p4beta1Block {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blockType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected block type (text, image etc) for this block."]
        pub block_type: ::std::option::Option<GoogleCloudVisionV1p4beta1BlockBlockTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding box for the block. The vertices are in the order of top-left, top-right, bottom-right, bottom-left. When a rotation of the bounding box is detected the rotation is represented as around the top-left corner as defined when the text is read in the 'natural' orientation. For example: * when the text is horizontal it might look like: 0----1 | | 3----2 * when it's rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3)."]
        pub bounding_box:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p4beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the OCR results on the block. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paragraphs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of paragraphs in this block (if this blocks is of type text)."]
        pub paragraphs: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1Paragraph>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information detected for the block."]
        pub property: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p4beta1TextAnnotationTextProperty>,
        >,
    }
    impl GoogleCloudVisionV1p4beta1Block {
        pub fn builder() -> GoogleCloudVisionV1p4beta1BlockBuilder {
            GoogleCloudVisionV1p4beta1BlockBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Detected block type (text, image etc) for this block."]
    pub enum GoogleCloudVisionV1p4beta1BlockBlockTypeEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown block type."]
        Unknown,
        #[serde(rename = "TEXT")]
        #[doc = "Regular text block."]
        Text,
        #[serde(rename = "TABLE")]
        #[doc = "Table block."]
        Table,
        #[serde(rename = "PICTURE")]
        #[doc = "Image block."]
        Picture,
        #[serde(rename = "RULER")]
        #[doc = "Horizontal/vertical line box."]
        Ruler,
        #[serde(rename = "BARCODE")]
        #[doc = "Barcode block."]
        Barcode,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p4beta1BlockBlockTypeEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A bounding polygon for the detected image annotation."]
    pub struct GoogleCloudVisionV1p4beta1BoundingPoly {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "normalizedVertices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon normalized vertices."]
        pub normalized_vertices: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1NormalizedVertex>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vertices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon vertices."]
        pub vertices: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1Vertex>>,
        >,
    }
    impl GoogleCloudVisionV1p4beta1BoundingPoly {
        pub fn builder() -> GoogleCloudVisionV1p4beta1BoundingPolyBuilder {
            GoogleCloudVisionV1p4beta1BoundingPolyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Celebrity is a group of Faces with an identity."]
    pub struct GoogleCloudVisionV1p4beta1Celebrity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Celebrity's description."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Celebrity's display name."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the preloaded Celebrity. Has the format `builtin/{mid}`."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p4beta1Celebrity {
        pub fn builder() -> GoogleCloudVisionV1p4beta1CelebrityBuilder {
            GoogleCloudVisionV1p4beta1CelebrityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Color information consists of RGB channels, score, and the fraction of the image that the color occupies in the image."]
    pub struct GoogleCloudVisionV1p4beta1ColorInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "color")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "RGB components of the color."]
        pub color: ::std::option::Option<::std::boxed::Box<Color>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pixelFraction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fraction of pixels the color occupies in the image. Value in range [0, 1]."]
        pub pixel_fraction: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image-specific score for this color. Value in range [0, 1]."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p4beta1ColorInfo {
        pub fn builder() -> GoogleCloudVisionV1p4beta1ColorInfoBuilder {
            GoogleCloudVisionV1p4beta1ColorInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Single crop hint that is used to generate a new crop when serving an image."]
    pub struct GoogleCloudVisionV1p4beta1CropHint {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon for the crop region. The coordinates of the bounding box are in the original image's scale."]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p4beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of this being a salient region. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importanceFraction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fraction of importance of this salient region with respect to the original image."]
        pub importance_fraction: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p4beta1CropHint {
        pub fn builder() -> GoogleCloudVisionV1p4beta1CropHintBuilder {
            GoogleCloudVisionV1p4beta1CropHintBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set of crop hints that are used to generate new crops when serving images."]
    pub struct GoogleCloudVisionV1p4beta1CropHintsAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cropHints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Crop hint results."]
        pub crop_hints: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1CropHint>>,
        >,
    }
    impl GoogleCloudVisionV1p4beta1CropHintsAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p4beta1CropHintsAnnotationBuilder {
            GoogleCloudVisionV1p4beta1CropHintsAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set of dominant colors and their corresponding scores."]
    pub struct GoogleCloudVisionV1p4beta1DominantColorsAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "RGB color values with their score and pixel fraction."]
        pub colors: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1ColorInfo>>,
        >,
    }
    impl GoogleCloudVisionV1p4beta1DominantColorsAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p4beta1DominantColorsAnnotationBuilder {
            GoogleCloudVisionV1p4beta1DominantColorsAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set of detected entity features."]
    pub struct GoogleCloudVisionV1p4beta1EntityAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image region to which this entity belongs. Not produced for `LABEL_DETECTION` features."]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p4beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "**Deprecated. Use `score` instead.** The accuracy of the entity detection in an image. For example, for an image in which the \"Eiffel Tower\" entity is detected, this field represents the confidence that there is a tower in the query image. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entity textual description, expressed in its `locale` language."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locale")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language code for the locale in which the entity textual `description` is expressed."]
        pub locale: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location information for the detected entity. Multiple `LocationInfo` elements can be present because one location may indicate the location of the scene in the image, and another location may indicate the location of the place where the image was taken. Location information is usually present for landmarks."]
        pub locations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1LocationInfo>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Opaque entity ID. Some IDs may be available in [Google Knowledge Graph Search API](https://developers.google.com/knowledge-graph/)."]
        pub mid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "properties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Some entities may have optional user-supplied `Property` (name/value) fields, such a score or string that qualifies the entity."]
        pub properties: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1Property>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Overall score of the result. Range [0, 1]."]
        pub score: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topicality")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The relevancy of the ICA (Image Content Annotation) label to the image. For example, the relevancy of \"tower\" is likely higher to an image containing the detected \"Eiffel Tower\" than to an image containing a detected distant towering building, even though the confidence that there is a tower in each image may be the same. Range [0, 1]."]
        pub topicality: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p4beta1EntityAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p4beta1EntityAnnotationBuilder {
            GoogleCloudVisionV1p4beta1EntityAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A face annotation object contains the results of face detection."]
    pub struct GoogleCloudVisionV1p4beta1FaceAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "angerLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Anger likelihood."]
        pub anger_likelihood:
            ::std::option::Option<GoogleCloudVisionV1p4beta1FaceAnnotationAngerLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blurredLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Blurred likelihood."]
        pub blurred_likelihood:
            ::std::option::Option<GoogleCloudVisionV1p4beta1FaceAnnotationBlurredLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon around the face. The coordinates of the bounding box are in the original image's scale. The bounding box is computed to \"frame\" the face in accordance with human expectations. It is based on the landmarker results. Note that one or more x and/or y coordinates may not be generated in the `BoundingPoly` (the polygon will be unbounded) if only a partial face appears in the image to be annotated."]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p4beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectionConfidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detection confidence. Range [0, 1]."]
        pub detection_confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fdBoundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `fd_bounding_poly` bounding polygon is tighter than the `boundingPoly`, and encloses only the skin part of the face. Typically, it is used to eliminate the face from any image analysis that detects the \"amount of skin\" visible in an image. It is not based on the landmarker results, only on the initial face detection, hence the fd (face detection) prefix."]
        pub fd_bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p4beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headwearLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Headwear likelihood."]
        pub headwear_likelihood:
            ::std::option::Option<GoogleCloudVisionV1p4beta1FaceAnnotationHeadwearLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "joyLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Joy likelihood."]
        pub joy_likelihood:
            ::std::option::Option<GoogleCloudVisionV1p4beta1FaceAnnotationJoyLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "landmarkingConfidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Face landmarking confidence. Range [0, 1]."]
        pub landmarking_confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "landmarks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected face landmarks."]
        pub landmarks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1FaceAnnotationLandmark>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "panAngle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Yaw angle, which indicates the leftward/rightward angle that the face is pointing relative to the vertical plane perpendicular to the image. Range [-180,180]."]
        pub pan_angle: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recognitionResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional recognition information. Only computed if image_context.face_recognition_params is provided, **and** a match is found to a Celebrity in the input CelebritySet. This field is sorted in order of decreasing confidence values."]
        pub recognition_result: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1FaceRecognitionResult>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rollAngle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Roll angle, which indicates the amount of clockwise/anti-clockwise rotation of the face relative to the image vertical about the axis perpendicular to the face. Range [-180,180]."]
        pub roll_angle: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sorrowLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sorrow likelihood."]
        pub sorrow_likelihood:
            ::std::option::Option<GoogleCloudVisionV1p4beta1FaceAnnotationSorrowLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "surpriseLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Surprise likelihood."]
        pub surprise_likelihood:
            ::std::option::Option<GoogleCloudVisionV1p4beta1FaceAnnotationSurpriseLikelihoodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tiltAngle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pitch angle, which indicates the upwards/downwards angle that the face is pointing relative to the image's horizontal plane. Range [-180,180]."]
        pub tilt_angle: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "underExposedLikelihood")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Under-exposed likelihood."]
        pub under_exposed_likelihood: ::std::option::Option<
            GoogleCloudVisionV1p4beta1FaceAnnotationUnderExposedLikelihoodEnum,
        >,
    }
    impl GoogleCloudVisionV1p4beta1FaceAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p4beta1FaceAnnotationBuilder {
            GoogleCloudVisionV1p4beta1FaceAnnotationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Anger likelihood."]
    pub enum GoogleCloudVisionV1p4beta1FaceAnnotationAngerLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p4beta1FaceAnnotationAngerLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Blurred likelihood."]
    pub enum GoogleCloudVisionV1p4beta1FaceAnnotationBlurredLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p4beta1FaceAnnotationBlurredLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Headwear likelihood."]
    pub enum GoogleCloudVisionV1p4beta1FaceAnnotationHeadwearLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p4beta1FaceAnnotationHeadwearLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Joy likelihood."]
    pub enum GoogleCloudVisionV1p4beta1FaceAnnotationJoyLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p4beta1FaceAnnotationJoyLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Sorrow likelihood."]
    pub enum GoogleCloudVisionV1p4beta1FaceAnnotationSorrowLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p4beta1FaceAnnotationSorrowLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Surprise likelihood."]
    pub enum GoogleCloudVisionV1p4beta1FaceAnnotationSurpriseLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p4beta1FaceAnnotationSurpriseLikelihoodEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Under-exposed likelihood."]
    pub enum GoogleCloudVisionV1p4beta1FaceAnnotationUnderExposedLikelihoodEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default
        for GoogleCloudVisionV1p4beta1FaceAnnotationUnderExposedLikelihoodEnum
    {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A face-specific landmark (for example, a face feature)."]
    pub struct GoogleCloudVisionV1p4beta1FaceAnnotationLandmark {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "position")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Face landmark position."]
        pub position: ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p4beta1Position>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Face landmark type."]
        pub _type: ::std::option::Option<GoogleCloudVisionV1p4beta1FaceAnnotationLandmarkTypeEnum>,
    }
    impl GoogleCloudVisionV1p4beta1FaceAnnotationLandmark {
        pub fn builder() -> GoogleCloudVisionV1p4beta1FaceAnnotationLandmarkBuilder {
            GoogleCloudVisionV1p4beta1FaceAnnotationLandmarkBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Face landmark type."]
    pub enum GoogleCloudVisionV1p4beta1FaceAnnotationLandmarkTypeEnum {
        #[serde(rename = "UNKNOWN_LANDMARK")]
        #[doc = "Unknown face landmark detected. Should not be filled."]
        UnknownLandmark,
        #[serde(rename = "LEFT_EYE")]
        #[doc = "Left eye."]
        LeftEye,
        #[serde(rename = "RIGHT_EYE")]
        #[doc = "Right eye."]
        RightEye,
        #[serde(rename = "LEFT_OF_LEFT_EYEBROW")]
        #[doc = "Left of left eyebrow."]
        LeftOfLeftEyebrow,
        #[serde(rename = "RIGHT_OF_LEFT_EYEBROW")]
        #[doc = "Right of left eyebrow."]
        RightOfLeftEyebrow,
        #[serde(rename = "LEFT_OF_RIGHT_EYEBROW")]
        #[doc = "Left of right eyebrow."]
        LeftOfRightEyebrow,
        #[serde(rename = "RIGHT_OF_RIGHT_EYEBROW")]
        #[doc = "Right of right eyebrow."]
        RightOfRightEyebrow,
        #[serde(rename = "MIDPOINT_BETWEEN_EYES")]
        #[doc = "Midpoint between eyes."]
        MidpointBetweenEyes,
        #[serde(rename = "NOSE_TIP")]
        #[doc = "Nose tip."]
        NoseTip,
        #[serde(rename = "UPPER_LIP")]
        #[doc = "Upper lip."]
        UpperLip,
        #[serde(rename = "LOWER_LIP")]
        #[doc = "Lower lip."]
        LowerLip,
        #[serde(rename = "MOUTH_LEFT")]
        #[doc = "Mouth left."]
        MouthLeft,
        #[serde(rename = "MOUTH_RIGHT")]
        #[doc = "Mouth right."]
        MouthRight,
        #[serde(rename = "MOUTH_CENTER")]
        #[doc = "Mouth center."]
        MouthCenter,
        #[serde(rename = "NOSE_BOTTOM_RIGHT")]
        #[doc = "Nose, bottom right."]
        NoseBottomRight,
        #[serde(rename = "NOSE_BOTTOM_LEFT")]
        #[doc = "Nose, bottom left."]
        NoseBottomLeft,
        #[serde(rename = "NOSE_BOTTOM_CENTER")]
        #[doc = "Nose, bottom center."]
        NoseBottomCenter,
        #[serde(rename = "LEFT_EYE_TOP_BOUNDARY")]
        #[doc = "Left eye, top boundary."]
        LeftEyeTopBoundary,
        #[serde(rename = "LEFT_EYE_RIGHT_CORNER")]
        #[doc = "Left eye, right corner."]
        LeftEyeRightCorner,
        #[serde(rename = "LEFT_EYE_BOTTOM_BOUNDARY")]
        #[doc = "Left eye, bottom boundary."]
        LeftEyeBottomBoundary,
        #[serde(rename = "LEFT_EYE_LEFT_CORNER")]
        #[doc = "Left eye, left corner."]
        LeftEyeLeftCorner,
        #[serde(rename = "RIGHT_EYE_TOP_BOUNDARY")]
        #[doc = "Right eye, top boundary."]
        RightEyeTopBoundary,
        #[serde(rename = "RIGHT_EYE_RIGHT_CORNER")]
        #[doc = "Right eye, right corner."]
        RightEyeRightCorner,
        #[serde(rename = "RIGHT_EYE_BOTTOM_BOUNDARY")]
        #[doc = "Right eye, bottom boundary."]
        RightEyeBottomBoundary,
        #[serde(rename = "RIGHT_EYE_LEFT_CORNER")]
        #[doc = "Right eye, left corner."]
        RightEyeLeftCorner,
        #[serde(rename = "LEFT_EYEBROW_UPPER_MIDPOINT")]
        #[doc = "Left eyebrow, upper midpoint."]
        LeftEyebrowUpperMidpoint,
        #[serde(rename = "RIGHT_EYEBROW_UPPER_MIDPOINT")]
        #[doc = "Right eyebrow, upper midpoint."]
        RightEyebrowUpperMidpoint,
        #[serde(rename = "LEFT_EAR_TRAGION")]
        #[doc = "Left ear tragion."]
        LeftEarTragion,
        #[serde(rename = "RIGHT_EAR_TRAGION")]
        #[doc = "Right ear tragion."]
        RightEarTragion,
        #[serde(rename = "LEFT_EYE_PUPIL")]
        #[doc = "Left eye pupil."]
        LeftEyePupil,
        #[serde(rename = "RIGHT_EYE_PUPIL")]
        #[doc = "Right eye pupil."]
        RightEyePupil,
        #[serde(rename = "FOREHEAD_GLABELLA")]
        #[doc = "Forehead glabella."]
        ForeheadGlabella,
        #[serde(rename = "CHIN_GNATHION")]
        #[doc = "Chin gnathion."]
        ChinGnathion,
        #[serde(rename = "CHIN_LEFT_GONION")]
        #[doc = "Chin left gonion."]
        ChinLeftGonion,
        #[serde(rename = "CHIN_RIGHT_GONION")]
        #[doc = "Chin right gonion."]
        ChinRightGonion,
        #[serde(rename = "LEFT_CHEEK_CENTER")]
        #[doc = "Left cheek center."]
        LeftCheekCenter,
        #[serde(rename = "RIGHT_CHEEK_CENTER")]
        #[doc = "Right cheek center."]
        RightCheekCenter,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p4beta1FaceAnnotationLandmarkTypeEnum {
        fn default() -> Self {
            Self::UnknownLandmark
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a face's identity."]
    pub struct GoogleCloudVisionV1p4beta1FaceRecognitionResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "celebrity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Celebrity that this face was matched to."]
        pub celebrity:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p4beta1Celebrity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Recognition confidence. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p4beta1FaceRecognitionResult {
        pub fn builder() -> GoogleCloudVisionV1p4beta1FaceRecognitionResultBuilder {
            GoogleCloudVisionV1p4beta1FaceRecognitionResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Google Cloud Storage location where the output will be written to."]
    pub struct GoogleCloudVisionV1p4beta1GcsDestination {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage URI prefix where the results will be stored. Results will be in JSON format and preceded by its corresponding input URI prefix. This field can either represent a gcs file prefix or gcs directory. In either case, the uri should be unique because in order to get all of the output files, you will need to do a wildcard gcs search on the uri prefix you provide. Examples: * File Prefix: gs://bucket-name/here/filenameprefix The output files will be created in gs://bucket-name/here/ and the names of the output files will begin with \"filenameprefix\". * Directory Prefix: gs://bucket-name/some/location/ The output files will be created in gs://bucket-name/some/location/ and the names of the output files could be anything because there was no filename prefix specified. If multiple outputs, each response is still AnnotateFileResponse, each of which contains some subset of the full list of AnnotateImageResponse. Multiple outputs can happen if, for example, the output JSON is too large and overflows into multiple sharded files."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p4beta1GcsDestination {
        pub fn builder() -> GoogleCloudVisionV1p4beta1GcsDestinationBuilder {
            GoogleCloudVisionV1p4beta1GcsDestinationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Google Cloud Storage location where the input will be read from."]
    pub struct GoogleCloudVisionV1p4beta1GcsSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage URI for the input file. This must only be a Google Cloud Storage object. Wildcards are not currently supported."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p4beta1GcsSource {
        pub fn builder() -> GoogleCloudVisionV1p4beta1GcsSourceBuilder {
            GoogleCloudVisionV1p4beta1GcsSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "If an image was produced from a file (e.g. a PDF), this message gives information about the source of that image."]
    pub struct GoogleCloudVisionV1p4beta1ImageAnnotationContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the file was a PDF or TIFF, this field gives the page number within the file used to produce the image."]
        pub page_number: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI of the file used to produce the image."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p4beta1ImageAnnotationContext {
        pub fn builder() -> GoogleCloudVisionV1p4beta1ImageAnnotationContextBuilder {
            GoogleCloudVisionV1p4beta1ImageAnnotationContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Stores image properties, such as dominant colors."]
    pub struct GoogleCloudVisionV1p4beta1ImageProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dominantColors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, dominant colors completed successfully."]
        pub dominant_colors: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p4beta1DominantColorsAnnotation>,
        >,
    }
    impl GoogleCloudVisionV1p4beta1ImageProperties {
        pub fn builder() -> GoogleCloudVisionV1p4beta1ImagePropertiesBuilder {
            GoogleCloudVisionV1p4beta1ImagePropertiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for the `ImportProductSets` method. This message is returned by the google.longrunning.Operations.GetOperation method in the returned google.longrunning.Operation.response field."]
    pub struct GoogleCloudVisionV1p4beta1ImportProductSetsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "referenceImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of reference_images that are imported successfully."]
        pub reference_images: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1ReferenceImage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statuses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The rpc status for each ImportProductSet request, including both successes and errors. The number of statuses here matches the number of lines in the csv file, and statuses[i] stores the success or failure status of processing the i-th line of the csv, starting from line 0."]
        pub statuses: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Status>>>,
    }
    impl GoogleCloudVisionV1p4beta1ImportProductSetsResponse {
        pub fn builder() -> GoogleCloudVisionV1p4beta1ImportProductSetsResponseBuilder {
            GoogleCloudVisionV1p4beta1ImportProductSetsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The desired input location and metadata."]
    pub struct GoogleCloudVisionV1p4beta1InputConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "File content, represented as a stream of bytes. Note: As with all `bytes` fields, protobuffers use a pure binary representation, whereas JSON representations use base64. Currently, this field only works for BatchAnnotateFiles requests. It does not work for AsyncBatchAnnotateFiles requests."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Google Cloud Storage location to read the input from."]
        pub gcs_source:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p4beta1GcsSource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the file. Currently only \"application/pdf\", \"image/tiff\" and \"image/gif\" are supported. Wildcards are not supported."]
        pub mime_type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p4beta1InputConfig {
        pub fn builder() -> GoogleCloudVisionV1p4beta1InputConfigBuilder {
            GoogleCloudVisionV1p4beta1InputConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set of detected objects with bounding boxes."]
    pub struct GoogleCloudVisionV1p4beta1LocalizedObjectAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image region to which this object belongs. This must be populated."]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p4beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object ID that should align with EntityAnnotation mid."]
        pub mid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object name, expressed in its `language_code` language."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Score of the result. Range [0, 1]."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p4beta1LocalizedObjectAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p4beta1LocalizedObjectAnnotationBuilder {
            GoogleCloudVisionV1p4beta1LocalizedObjectAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected entity location information."]
    pub struct GoogleCloudVisionV1p4beta1LocationInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "latLng")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "lat/long location coordinates."]
        pub lat_lng: ::std::option::Option<::std::boxed::Box<LatLng>>,
    }
    impl GoogleCloudVisionV1p4beta1LocationInfo {
        pub fn builder() -> GoogleCloudVisionV1p4beta1LocationInfoBuilder {
            GoogleCloudVisionV1p4beta1LocationInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A vertex represents a 2D point in the image. NOTE: the normalized vertex coordinates are relative to the original image and range from 0 to 1."]
    pub struct GoogleCloudVisionV1p4beta1NormalizedVertex {
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
    impl GoogleCloudVisionV1p4beta1NormalizedVertex {
        pub fn builder() -> GoogleCloudVisionV1p4beta1NormalizedVertexBuilder {
            GoogleCloudVisionV1p4beta1NormalizedVertexBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains metadata for the BatchAnnotateImages operation."]
    pub struct GoogleCloudVisionV1p4beta1OperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when the batch request was received."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Current state of the batch operation."]
        pub state: ::std::option::Option<GoogleCloudVisionV1p4beta1OperationMetadataStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when the operation result was last updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p4beta1OperationMetadata {
        pub fn builder() -> GoogleCloudVisionV1p4beta1OperationMetadataBuilder {
            GoogleCloudVisionV1p4beta1OperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Current state of the batch operation."]
    pub enum GoogleCloudVisionV1p4beta1OperationMetadataStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Invalid."]
        StateUnspecified,
        #[serde(rename = "CREATED")]
        #[doc = "Request is received."]
        Created,
        #[serde(rename = "RUNNING")]
        #[doc = "Request is actively being processed."]
        Running,
        #[serde(rename = "DONE")]
        #[doc = "The batch processing is done."]
        Done,
        #[serde(rename = "CANCELLED")]
        #[doc = "The batch processing was cancelled."]
        Cancelled,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p4beta1OperationMetadataStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The desired output location and metadata."]
    pub struct GoogleCloudVisionV1p4beta1OutputConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The max number of response protos to put into each output JSON file on Google Cloud Storage. The valid range is [1, 100]. If not specified, the default value is 20. For example, for one pdf file with 100 pages, 100 response protos will be generated. If `batch_size` = 20, then 5 json files each containing 20 response protos will be written under the prefix `gcs_destination`.`uri`. Currently, batch_size only applies to GcsDestination, with potential future support for other output configurations."]
        pub batch_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsDestination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Google Cloud Storage location to write the output(s) to."]
        pub gcs_destination:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p4beta1GcsDestination>>,
    }
    impl GoogleCloudVisionV1p4beta1OutputConfig {
        pub fn builder() -> GoogleCloudVisionV1p4beta1OutputConfigBuilder {
            GoogleCloudVisionV1p4beta1OutputConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected page from OCR."]
    pub struct GoogleCloudVisionV1p4beta1Page {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blocks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of blocks of text, images etc on this page."]
        pub blocks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1Block>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the OCR results on the page. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page height. For PDFs the unit is points. For images (including TIFFs) the unit is pixels."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information detected on the page."]
        pub property: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p4beta1TextAnnotationTextProperty>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page width. For PDFs the unit is points. For images (including TIFFs) the unit is pixels."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudVisionV1p4beta1Page {
        pub fn builder() -> GoogleCloudVisionV1p4beta1PageBuilder {
            GoogleCloudVisionV1p4beta1PageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Structural unit of text representing a number of words in certain order."]
    pub struct GoogleCloudVisionV1p4beta1Paragraph {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding box for the paragraph. The vertices are in the order of top-left, top-right, bottom-right, bottom-left. When a rotation of the bounding box is detected the rotation is represented as around the top-left corner as defined when the text is read in the 'natural' orientation. For example: * when the text is horizontal it might look like: 0----1 | | 3----2 * when it's rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3)."]
        pub bounding_box:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p4beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the OCR results for the paragraph. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information detected for the paragraph."]
        pub property: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p4beta1TextAnnotationTextProperty>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "words")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of all words in this paragraph."]
        pub words: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1Word>>,
        >,
    }
    impl GoogleCloudVisionV1p4beta1Paragraph {
        pub fn builder() -> GoogleCloudVisionV1p4beta1ParagraphBuilder {
            GoogleCloudVisionV1p4beta1ParagraphBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A 3D position in the image, used primarily for Face detection landmarks. A valid Position must have both x and y coordinates. The position coordinates are in the same scale as the original image."]
    pub struct GoogleCloudVisionV1p4beta1Position {
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
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "z")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Z coordinate (or depth)."]
        pub z: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p4beta1Position {
        pub fn builder() -> GoogleCloudVisionV1p4beta1PositionBuilder {
            GoogleCloudVisionV1p4beta1PositionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Product contains ReferenceImages."]
    pub struct GoogleCloudVisionV1p4beta1Product {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User-provided metadata to be stored with this product. Must be at most 4096 characters long."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user-provided name for this Product. Must not be empty. Must be at most 4096 characters long."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the product. Format is: `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`. This field is ignored when creating a product."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productCategory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Immutable. The category for the product identified by the reference image. This should be one of \"homegoods-v2\", \"apparel-v2\", \"toys-v2\", \"packagedgoods-v1\" or \"general-v1\". The legacy categories \"homegoods\", \"apparel\", and \"toys\" are still supported, but these should not be used for new products."]
        pub product_category: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Key-value pairs that can be attached to a product. At query time, constraints can be specified based on the product_labels. Note that integer values can be provided as strings, e.g. \"1199\". Only strings with integer values can match a range-based restriction which is to be supported soon. Multiple values can be assigned to the same key. One product may have up to 500 product_labels. Notice that the total number of distinct product_labels over all products in one ProductSet cannot exceed 1M, otherwise the product search pipeline will refuse to work for that ProductSet."]
        pub product_labels: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1ProductKeyValue>>,
        >,
    }
    impl GoogleCloudVisionV1p4beta1Product {
        pub fn builder() -> GoogleCloudVisionV1p4beta1ProductBuilder {
            GoogleCloudVisionV1p4beta1ProductBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A product label represented as a key-value pair."]
    pub struct GoogleCloudVisionV1p4beta1ProductKeyValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The key of the label attached to the product. Cannot be empty and cannot exceed 128 bytes."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value of the label attached to the product. Cannot be empty and cannot exceed 128 bytes."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p4beta1ProductKeyValue {
        pub fn builder() -> GoogleCloudVisionV1p4beta1ProductKeyValueBuilder {
            GoogleCloudVisionV1p4beta1ProductKeyValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Results for a product search request."]
    pub struct GoogleCloudVisionV1p4beta1ProductSearchResults {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "indexTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp of the index which provided these results. Products added to the product set and products removed from the product set after this time are not reflected in the current results."]
        pub index_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productGroupedResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of results grouped by products detected in the query image. Each entry corresponds to one bounding polygon in the query image, and contains the matching products specific to that region. There may be duplicate product matches in the union of all the per-product results."]
        pub product_grouped_results: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVisionV1p4beta1ProductSearchResultsGroupedResult>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of results, one for each product match."]
        pub results: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVisionV1p4beta1ProductSearchResultsResult>,
            >,
        >,
    }
    impl GoogleCloudVisionV1p4beta1ProductSearchResults {
        pub fn builder() -> GoogleCloudVisionV1p4beta1ProductSearchResultsBuilder {
            GoogleCloudVisionV1p4beta1ProductSearchResultsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about the products similar to a single product in a query image."]
    pub struct GoogleCloudVisionV1p4beta1ProductSearchResultsGroupedResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon around the product detected in the query image."]
        pub bounding_poly:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p4beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of generic predictions for the object in the bounding box."]
        pub object_annotations: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVisionV1p4beta1ProductSearchResultsObjectAnnotation>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of results, one for each product match."]
        pub results: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVisionV1p4beta1ProductSearchResultsResult>,
            >,
        >,
    }
    impl GoogleCloudVisionV1p4beta1ProductSearchResultsGroupedResult {
        pub fn builder() -> GoogleCloudVisionV1p4beta1ProductSearchResultsGroupedResultBuilder {
            GoogleCloudVisionV1p4beta1ProductSearchResultsGroupedResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Prediction for what the object in the bounding box is."]
    pub struct GoogleCloudVisionV1p4beta1ProductSearchResultsObjectAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object ID that should align with EntityAnnotation mid."]
        pub mid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object name, expressed in its `language_code` language."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Score of the result. Range [0, 1]."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p4beta1ProductSearchResultsObjectAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p4beta1ProductSearchResultsObjectAnnotationBuilder {
            GoogleCloudVisionV1p4beta1ProductSearchResultsObjectAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a product."]
    pub struct GoogleCloudVisionV1p4beta1ProductSearchResultsResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the image from the product that is the closest match to the query."]
        pub image: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "product")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Product."]
        pub product: ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p4beta1Product>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A confidence level on the match, ranging from 0 (no confidence) to 1 (full confidence)."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p4beta1ProductSearchResultsResult {
        pub fn builder() -> GoogleCloudVisionV1p4beta1ProductSearchResultsResultBuilder {
            GoogleCloudVisionV1p4beta1ProductSearchResultsResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A `Property` consists of a user-supplied name/value pair."]
    pub struct GoogleCloudVisionV1p4beta1Property {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the property."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uint64Value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value of numeric properties."]
        pub uint64_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value of the property."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p4beta1Property {
        pub fn builder() -> GoogleCloudVisionV1p4beta1PropertyBuilder {
            GoogleCloudVisionV1p4beta1PropertyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A `ReferenceImage` represents a product image and its associated metadata, such as bounding boxes."]
    pub struct GoogleCloudVisionV1p4beta1ReferenceImage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPolys")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Bounding polygons around the areas of interest in the reference image. If this field is empty, the system will try to detect regions of interest. At most 10 bounding polygons will be used. The provided shape is converted into a non-rotated rectangle. Once converted, the small edge of the rectangle must be greater than or equal to 300 pixels. The aspect ratio must be 1:4 or less (i.e. 1:3 is ok; 1:5 is not)."]
        pub bounding_polys: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1BoundingPoly>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the reference image. Format is: `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID/referenceImages/IMAGE_ID`. This field is ignored when creating a reference image."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The Google Cloud Storage URI of the reference image. The URI must start with `gs://`."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p4beta1ReferenceImage {
        pub fn builder() -> GoogleCloudVisionV1p4beta1ReferenceImageBuilder {
            GoogleCloudVisionV1p4beta1ReferenceImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set of features pertaining to the image, computed by computer vision methods over safe-search verticals (for example, adult, spoof, medical, violence)."]
    pub struct GoogleCloudVisionV1p4beta1SafeSearchAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents the adult content likelihood for the image. Adult content may contain elements such as nudity, pornographic images or cartoons, or sexual activities."]
        pub adult: ::std::option::Option<GoogleCloudVisionV1p4beta1SafeSearchAnnotationAdultEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "medical")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Likelihood that this is a medical image."]
        pub medical:
            ::std::option::Option<GoogleCloudVisionV1p4beta1SafeSearchAnnotationMedicalEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "racy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Likelihood that the request image contains racy content. Racy content may include (but is not limited to) skimpy or sheer clothing, strategically covered nudity, lewd or provocative poses, or close-ups of sensitive body areas."]
        pub racy: ::std::option::Option<GoogleCloudVisionV1p4beta1SafeSearchAnnotationRacyEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spoof")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Spoof likelihood. The likelihood that an modification was made to the image's canonical version to make it appear funny or offensive."]
        pub spoof: ::std::option::Option<GoogleCloudVisionV1p4beta1SafeSearchAnnotationSpoofEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "violence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Likelihood that this image contains violent content."]
        pub violence:
            ::std::option::Option<GoogleCloudVisionV1p4beta1SafeSearchAnnotationViolenceEnum>,
    }
    impl GoogleCloudVisionV1p4beta1SafeSearchAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p4beta1SafeSearchAnnotationBuilder {
            GoogleCloudVisionV1p4beta1SafeSearchAnnotationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Represents the adult content likelihood for the image. Adult content may contain elements such as nudity, pornographic images or cartoons, or sexual activities."]
    pub enum GoogleCloudVisionV1p4beta1SafeSearchAnnotationAdultEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p4beta1SafeSearchAnnotationAdultEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Likelihood that this is a medical image."]
    pub enum GoogleCloudVisionV1p4beta1SafeSearchAnnotationMedicalEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p4beta1SafeSearchAnnotationMedicalEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Likelihood that the request image contains racy content. Racy content may include (but is not limited to) skimpy or sheer clothing, strategically covered nudity, lewd or provocative poses, or close-ups of sensitive body areas."]
    pub enum GoogleCloudVisionV1p4beta1SafeSearchAnnotationRacyEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p4beta1SafeSearchAnnotationRacyEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Spoof likelihood. The likelihood that an modification was made to the image's canonical version to make it appear funny or offensive."]
    pub enum GoogleCloudVisionV1p4beta1SafeSearchAnnotationSpoofEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p4beta1SafeSearchAnnotationSpoofEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Likelihood that this image contains violent content."]
    pub enum GoogleCloudVisionV1p4beta1SafeSearchAnnotationViolenceEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p4beta1SafeSearchAnnotationViolenceEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single symbol representation."]
    pub struct GoogleCloudVisionV1p4beta1Symbol {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding box for the symbol. The vertices are in the order of top-left, top-right, bottom-right, bottom-left. When a rotation of the bounding box is detected the rotation is represented as around the top-left corner as defined when the text is read in the 'natural' orientation. For example: * when the text is horizontal it might look like: 0----1 | | 3----2 * when it's rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3)."]
        pub bounding_box:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p4beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the OCR results for the symbol. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information detected for the symbol."]
        pub property: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p4beta1TextAnnotationTextProperty>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The actual UTF-8 representation of the symbol."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p4beta1Symbol {
        pub fn builder() -> GoogleCloudVisionV1p4beta1SymbolBuilder {
            GoogleCloudVisionV1p4beta1SymbolBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "TextAnnotation contains a structured representation of OCR extracted text. The hierarchy of an OCR extracted text structure is like this: TextAnnotation -> Page -> Block -> Paragraph -> Word -> Symbol Each structural component, starting from Page, may further have their own properties. Properties describe detected languages, breaks etc.. Please refer to the TextAnnotation.TextProperty message definition below for more detail."]
    pub struct GoogleCloudVisionV1p4beta1TextAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of pages detected by OCR."]
        pub pages: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1Page>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "UTF-8 text detected on the pages."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p4beta1TextAnnotation {
        pub fn builder() -> GoogleCloudVisionV1p4beta1TextAnnotationBuilder {
            GoogleCloudVisionV1p4beta1TextAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected start or end of a structural component."]
    pub struct GoogleCloudVisionV1p4beta1TextAnnotationDetectedBreak {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isPrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if break prepends the element."]
        pub is_prefix: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected break type."]
        pub _type:
            ::std::option::Option<GoogleCloudVisionV1p4beta1TextAnnotationDetectedBreakTypeEnum>,
    }
    impl GoogleCloudVisionV1p4beta1TextAnnotationDetectedBreak {
        pub fn builder() -> GoogleCloudVisionV1p4beta1TextAnnotationDetectedBreakBuilder {
            GoogleCloudVisionV1p4beta1TextAnnotationDetectedBreakBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Detected break type."]
    pub enum GoogleCloudVisionV1p4beta1TextAnnotationDetectedBreakTypeEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown break label type."]
        Unknown,
        #[serde(rename = "SPACE")]
        #[doc = "Regular space."]
        Space,
        #[serde(rename = "SURE_SPACE")]
        #[doc = "Sure space (very wide)."]
        SureSpace,
        #[serde(rename = "EOL_SURE_SPACE")]
        #[doc = "Line-wrapping break."]
        EolSureSpace,
        #[serde(rename = "HYPHEN")]
        #[doc = "End-line hyphen that is not present in text; does not co-occur with `SPACE`, `LEADER_SPACE`, or `LINE_BREAK`."]
        Hyphen,
        #[serde(rename = "LINE_BREAK")]
        #[doc = "Line break that ends a paragraph."]
        LineBreak,
    }
    impl ::std::default::Default for GoogleCloudVisionV1p4beta1TextAnnotationDetectedBreakTypeEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected language for a structural component."]
    pub struct GoogleCloudVisionV1p4beta1TextAnnotationDetectedLanguage {
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
    impl GoogleCloudVisionV1p4beta1TextAnnotationDetectedLanguage {
        pub fn builder() -> GoogleCloudVisionV1p4beta1TextAnnotationDetectedLanguageBuilder {
            GoogleCloudVisionV1p4beta1TextAnnotationDetectedLanguageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional information detected on the structural component."]
    pub struct GoogleCloudVisionV1p4beta1TextAnnotationTextProperty {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedBreak")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected start or end of a text segment."]
        pub detected_break: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p4beta1TextAnnotationDetectedBreak>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected languages together with confidence."]
        pub detected_languages: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudVisionV1p4beta1TextAnnotationDetectedLanguage>,
            >,
        >,
    }
    impl GoogleCloudVisionV1p4beta1TextAnnotationTextProperty {
        pub fn builder() -> GoogleCloudVisionV1p4beta1TextAnnotationTextPropertyBuilder {
            GoogleCloudVisionV1p4beta1TextAnnotationTextPropertyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A vertex represents a 2D point in the image. NOTE: the vertex coordinates are in the same scale as the original image."]
    pub struct GoogleCloudVisionV1p4beta1Vertex {
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
    impl GoogleCloudVisionV1p4beta1Vertex {
        pub fn builder() -> GoogleCloudVisionV1p4beta1VertexBuilder {
            GoogleCloudVisionV1p4beta1VertexBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Relevant information for the image from the Internet."]
    pub struct GoogleCloudVisionV1p4beta1WebDetection {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bestGuessLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The service's best guess as to the topic of the request image. Inferred from similar images on the open web."]
        pub best_guess_labels: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1WebDetectionWebLabel>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullMatchingImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fully matching images from the Internet. Can include resized copies of the query image."]
        pub full_matching_images: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1WebDetectionWebImage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pagesWithMatchingImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web pages containing the matching images from the Internet."]
        pub pages_with_matching_images: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1WebDetectionWebPage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialMatchingImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Partial matching images from the Internet. Those images are similar enough to share some key-point features. For example an original image will likely have partial matching for its crops."]
        pub partial_matching_images: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1WebDetectionWebImage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visuallySimilarImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visually similar image results."]
        pub visually_similar_images: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1WebDetectionWebImage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webEntities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deduced entities from similar images on the Internet."]
        pub web_entities: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1WebDetectionWebEntity>>,
        >,
    }
    impl GoogleCloudVisionV1p4beta1WebDetection {
        pub fn builder() -> GoogleCloudVisionV1p4beta1WebDetectionBuilder {
            GoogleCloudVisionV1p4beta1WebDetectionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Entity deduced from similar images on the Internet."]
    pub struct GoogleCloudVisionV1p4beta1WebDetectionWebEntity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Canonical description of the entity, in English."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Opaque entity ID."]
        pub entity_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Overall relevancy score for the entity. Not normalized and not comparable across different image queries."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudVisionV1p4beta1WebDetectionWebEntity {
        pub fn builder() -> GoogleCloudVisionV1p4beta1WebDetectionWebEntityBuilder {
            GoogleCloudVisionV1p4beta1WebDetectionWebEntityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for online images."]
    pub struct GoogleCloudVisionV1p4beta1WebDetectionWebImage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "(Deprecated) Overall relevancy score for the image."]
        pub score: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result image URL."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p4beta1WebDetectionWebImage {
        pub fn builder() -> GoogleCloudVisionV1p4beta1WebDetectionWebImageBuilder {
            GoogleCloudVisionV1p4beta1WebDetectionWebImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Label to provide extra metadata for the web detection."]
    pub struct GoogleCloudVisionV1p4beta1WebDetectionWebLabel {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "label")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label for extra metadata."]
        pub label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The BCP-47 language code for `label`, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        pub language_code: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p4beta1WebDetectionWebLabel {
        pub fn builder() -> GoogleCloudVisionV1p4beta1WebDetectionWebLabelBuilder {
            GoogleCloudVisionV1p4beta1WebDetectionWebLabelBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for web pages."]
    pub struct GoogleCloudVisionV1p4beta1WebDetectionWebPage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullMatchingImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fully matching images on the page. Can include resized copies of the query image."]
        pub full_matching_images: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1WebDetectionWebImage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title for the web page, may contain HTML markups."]
        pub page_title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialMatchingImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Partial matching images on the page. Those images are similar enough to share some key-point features. For example an original image will likely have partial matching for its crops."]
        pub partial_matching_images: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1WebDetectionWebImage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "(Deprecated) Overall relevancy score for the web page."]
        pub score: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result web page URL."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudVisionV1p4beta1WebDetectionWebPage {
        pub fn builder() -> GoogleCloudVisionV1p4beta1WebDetectionWebPageBuilder {
            GoogleCloudVisionV1p4beta1WebDetectionWebPageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A word representation."]
    pub struct GoogleCloudVisionV1p4beta1Word {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding box for the word. The vertices are in the order of top-left, top-right, bottom-right, bottom-left. When a rotation of the bounding box is detected the rotation is represented as around the top-left corner as defined when the text is read in the 'natural' orientation. For example: * when the text is horizontal it might look like: 0----1 | | 3----2 * when it's rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3)."]
        pub bounding_box:
            ::std::option::Option<::std::boxed::Box<GoogleCloudVisionV1p4beta1BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the OCR results for the word. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information detected for the word."]
        pub property: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudVisionV1p4beta1TextAnnotationTextProperty>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "symbols")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of symbols in the word. The order of the symbols follows the natural reading order."]
        pub symbols: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudVisionV1p4beta1Symbol>>,
        >,
    }
    impl GoogleCloudVisionV1p4beta1Word {
        pub fn builder() -> GoogleCloudVisionV1p4beta1WordBuilder {
            GoogleCloudVisionV1p4beta1WordBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about the products similar to a single product in a query image."]
    pub struct GroupedResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding polygon around the product detected in the query image."]
        pub bounding_poly: ::std::option::Option<::std::boxed::Box<BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectAnnotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of generic predictions for the object in the bounding box."]
        pub object_annotations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ObjectAnnotation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of results, one for each product match."]
        pub results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Result>>>,
    }
    impl GroupedResult {
        pub fn builder() -> GroupedResultBuilder {
            GroupedResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "If an image was produced from a file (e.g. a PDF), this message gives information about the source of that image."]
    pub struct ImageAnnotationContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the file was a PDF or TIFF, this field gives the page number within the file used to produce the image."]
        pub page_number: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI of the file used to produce the image."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl ImageAnnotationContext {
        pub fn builder() -> ImageAnnotationContextBuilder {
            ImageAnnotationContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Stores image properties, such as dominant colors."]
    pub struct ImageProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dominantColors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, dominant colors completed successfully."]
        pub dominant_colors: ::std::option::Option<::std::boxed::Box<DominantColorsAnnotation>>,
    }
    impl ImageProperties {
        pub fn builder() -> ImagePropertiesBuilder {
            ImagePropertiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for the `ImportProductSets` method. This message is returned by the google.longrunning.Operations.GetOperation method in the returned google.longrunning.Operation.response field."]
    pub struct ImportProductSetsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "referenceImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of reference_images that are imported successfully."]
        pub reference_images:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ReferenceImage>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statuses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The rpc status for each ImportProductSet request, including both successes and errors. The number of statuses here matches the number of lines in the csv file, and statuses[i] stores the success or failure status of processing the i-th line of the csv, starting from line 0."]
        pub statuses: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Status>>>,
    }
    impl ImportProductSetsResponse {
        pub fn builder() -> ImportProductSetsResponseBuilder {
            ImportProductSetsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The desired input location and metadata."]
    pub struct InputConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "File content, represented as a stream of bytes. Note: As with all `bytes` fields, protobuffers use a pure binary representation, whereas JSON representations use base64. Currently, this field only works for BatchAnnotateFiles requests. It does not work for AsyncBatchAnnotateFiles requests."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Google Cloud Storage location to read the input from."]
        pub gcs_source: ::std::option::Option<::std::boxed::Box<GcsSource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the file. Currently only \"application/pdf\", \"image/tiff\" and \"image/gif\" are supported. Wildcards are not supported."]
        pub mime_type: ::std::option::Option<::std::string::String>,
    }
    impl InputConfig {
        pub fn builder() -> InputConfigBuilder {
            InputConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A product label represented as a key-value pair."]
    pub struct KeyValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The key of the label attached to the product. Cannot be empty and cannot exceed 128 bytes."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value of the label attached to the product. Cannot be empty and cannot exceed 128 bytes."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl KeyValue {
        pub fn builder() -> KeyValueBuilder {
            KeyValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A face-specific landmark (for example, a face feature)."]
    pub struct Landmark {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "position")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Face landmark position."]
        pub position: ::std::option::Option<::std::boxed::Box<Position>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Face landmark type."]
        pub _type: ::std::option::Option<LandmarkTypeEnum>,
    }
    impl Landmark {
        pub fn builder() -> LandmarkBuilder {
            LandmarkBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Face landmark type."]
    pub enum LandmarkTypeEnum {
        #[serde(rename = "UNKNOWN_LANDMARK")]
        #[doc = "Unknown face landmark detected. Should not be filled."]
        UnknownLandmark,
        #[serde(rename = "LEFT_EYE")]
        #[doc = "Left eye."]
        LeftEye,
        #[serde(rename = "RIGHT_EYE")]
        #[doc = "Right eye."]
        RightEye,
        #[serde(rename = "LEFT_OF_LEFT_EYEBROW")]
        #[doc = "Left of left eyebrow."]
        LeftOfLeftEyebrow,
        #[serde(rename = "RIGHT_OF_LEFT_EYEBROW")]
        #[doc = "Right of left eyebrow."]
        RightOfLeftEyebrow,
        #[serde(rename = "LEFT_OF_RIGHT_EYEBROW")]
        #[doc = "Left of right eyebrow."]
        LeftOfRightEyebrow,
        #[serde(rename = "RIGHT_OF_RIGHT_EYEBROW")]
        #[doc = "Right of right eyebrow."]
        RightOfRightEyebrow,
        #[serde(rename = "MIDPOINT_BETWEEN_EYES")]
        #[doc = "Midpoint between eyes."]
        MidpointBetweenEyes,
        #[serde(rename = "NOSE_TIP")]
        #[doc = "Nose tip."]
        NoseTip,
        #[serde(rename = "UPPER_LIP")]
        #[doc = "Upper lip."]
        UpperLip,
        #[serde(rename = "LOWER_LIP")]
        #[doc = "Lower lip."]
        LowerLip,
        #[serde(rename = "MOUTH_LEFT")]
        #[doc = "Mouth left."]
        MouthLeft,
        #[serde(rename = "MOUTH_RIGHT")]
        #[doc = "Mouth right."]
        MouthRight,
        #[serde(rename = "MOUTH_CENTER")]
        #[doc = "Mouth center."]
        MouthCenter,
        #[serde(rename = "NOSE_BOTTOM_RIGHT")]
        #[doc = "Nose, bottom right."]
        NoseBottomRight,
        #[serde(rename = "NOSE_BOTTOM_LEFT")]
        #[doc = "Nose, bottom left."]
        NoseBottomLeft,
        #[serde(rename = "NOSE_BOTTOM_CENTER")]
        #[doc = "Nose, bottom center."]
        NoseBottomCenter,
        #[serde(rename = "LEFT_EYE_TOP_BOUNDARY")]
        #[doc = "Left eye, top boundary."]
        LeftEyeTopBoundary,
        #[serde(rename = "LEFT_EYE_RIGHT_CORNER")]
        #[doc = "Left eye, right corner."]
        LeftEyeRightCorner,
        #[serde(rename = "LEFT_EYE_BOTTOM_BOUNDARY")]
        #[doc = "Left eye, bottom boundary."]
        LeftEyeBottomBoundary,
        #[serde(rename = "LEFT_EYE_LEFT_CORNER")]
        #[doc = "Left eye, left corner."]
        LeftEyeLeftCorner,
        #[serde(rename = "RIGHT_EYE_TOP_BOUNDARY")]
        #[doc = "Right eye, top boundary."]
        RightEyeTopBoundary,
        #[serde(rename = "RIGHT_EYE_RIGHT_CORNER")]
        #[doc = "Right eye, right corner."]
        RightEyeRightCorner,
        #[serde(rename = "RIGHT_EYE_BOTTOM_BOUNDARY")]
        #[doc = "Right eye, bottom boundary."]
        RightEyeBottomBoundary,
        #[serde(rename = "RIGHT_EYE_LEFT_CORNER")]
        #[doc = "Right eye, left corner."]
        RightEyeLeftCorner,
        #[serde(rename = "LEFT_EYEBROW_UPPER_MIDPOINT")]
        #[doc = "Left eyebrow, upper midpoint."]
        LeftEyebrowUpperMidpoint,
        #[serde(rename = "RIGHT_EYEBROW_UPPER_MIDPOINT")]
        #[doc = "Right eyebrow, upper midpoint."]
        RightEyebrowUpperMidpoint,
        #[serde(rename = "LEFT_EAR_TRAGION")]
        #[doc = "Left ear tragion."]
        LeftEarTragion,
        #[serde(rename = "RIGHT_EAR_TRAGION")]
        #[doc = "Right ear tragion."]
        RightEarTragion,
        #[serde(rename = "LEFT_EYE_PUPIL")]
        #[doc = "Left eye pupil."]
        LeftEyePupil,
        #[serde(rename = "RIGHT_EYE_PUPIL")]
        #[doc = "Right eye pupil."]
        RightEyePupil,
        #[serde(rename = "FOREHEAD_GLABELLA")]
        #[doc = "Forehead glabella."]
        ForeheadGlabella,
        #[serde(rename = "CHIN_GNATHION")]
        #[doc = "Chin gnathion."]
        ChinGnathion,
        #[serde(rename = "CHIN_LEFT_GONION")]
        #[doc = "Chin left gonion."]
        ChinLeftGonion,
        #[serde(rename = "CHIN_RIGHT_GONION")]
        #[doc = "Chin right gonion."]
        ChinRightGonion,
        #[serde(rename = "LEFT_CHEEK_CENTER")]
        #[doc = "Left cheek center."]
        LeftCheekCenter,
        #[serde(rename = "RIGHT_CHEEK_CENTER")]
        #[doc = "Right cheek center."]
        RightCheekCenter,
    }
    impl ::std::default::Default for LandmarkTypeEnum {
        fn default() -> Self {
            Self::UnknownLandmark
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An object that represents a latitude/longitude pair. This is expressed as a pair of doubles to represent degrees latitude and degrees longitude. Unless specified otherwise, this must conform to the WGS84 standard. Values must be within normalized ranges."]
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
    #[doc = "Set of detected objects with bounding boxes."]
    pub struct LocalizedObjectAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPoly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image region to which this object belongs. This must be populated."]
        pub bounding_poly: ::std::option::Option<::std::boxed::Box<BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object ID that should align with EntityAnnotation mid."]
        pub mid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object name, expressed in its `language_code` language."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Score of the result. Range [0, 1]."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl LocalizedObjectAnnotation {
        pub fn builder() -> LocalizedObjectAnnotationBuilder {
            LocalizedObjectAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected entity location information."]
    pub struct LocationInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "latLng")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "lat/long location coordinates."]
        pub lat_lng: ::std::option::Option<::std::boxed::Box<LatLng>>,
    }
    impl LocationInfo {
        pub fn builder() -> LocationInfoBuilder {
            LocationInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A vertex represents a 2D point in the image. NOTE: the normalized vertex coordinates are relative to the original image and range from 0 to 1."]
    pub struct NormalizedVertex {
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
    impl NormalizedVertex {
        pub fn builder() -> NormalizedVertexBuilder {
            NormalizedVertexBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Prediction for what the object in the bounding box is."]
    pub struct ObjectAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object ID that should align with EntityAnnotation mid."]
        pub mid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Object name, expressed in its `language_code` language."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Score of the result. Range [0, 1]."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl ObjectAnnotation {
        pub fn builder() -> ObjectAnnotationBuilder {
            ObjectAnnotationBuilder::default()
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
    #[doc = "Contains metadata for the BatchAnnotateImages operation."]
    pub struct OperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when the batch request was received."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Current state of the batch operation."]
        pub state: ::std::option::Option<OperationMetadataStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when the operation result was last updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl OperationMetadata {
        pub fn builder() -> OperationMetadataBuilder {
            OperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Current state of the batch operation."]
    pub enum OperationMetadataStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Invalid."]
        StateUnspecified,
        #[serde(rename = "CREATED")]
        #[doc = "Request is received."]
        Created,
        #[serde(rename = "RUNNING")]
        #[doc = "Request is actively being processed."]
        Running,
        #[serde(rename = "DONE")]
        #[doc = "The batch processing is done."]
        Done,
        #[serde(rename = "CANCELLED")]
        #[doc = "The batch processing was cancelled."]
        Cancelled,
    }
    impl ::std::default::Default for OperationMetadataStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The desired output location and metadata."]
    pub struct OutputConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The max number of response protos to put into each output JSON file on Google Cloud Storage. The valid range is [1, 100]. If not specified, the default value is 20. For example, for one pdf file with 100 pages, 100 response protos will be generated. If `batch_size` = 20, then 5 json files each containing 20 response protos will be written under the prefix `gcs_destination`.`uri`. Currently, batch_size only applies to GcsDestination, with potential future support for other output configurations."]
        pub batch_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsDestination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Google Cloud Storage location to write the output(s) to."]
        pub gcs_destination: ::std::option::Option<::std::boxed::Box<GcsDestination>>,
    }
    impl OutputConfig {
        pub fn builder() -> OutputConfigBuilder {
            OutputConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected page from OCR."]
    pub struct Page {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blocks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of blocks of text, images etc on this page."]
        pub blocks: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Block>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the OCR results on the page. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page height. For PDFs the unit is points. For images (including TIFFs) the unit is pixels."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information detected on the page."]
        pub property: ::std::option::Option<::std::boxed::Box<TextProperty>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page width. For PDFs the unit is points. For images (including TIFFs) the unit is pixels."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl Page {
        pub fn builder() -> PageBuilder {
            PageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Structural unit of text representing a number of words in certain order."]
    pub struct Paragraph {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding box for the paragraph. The vertices are in the order of top-left, top-right, bottom-right, bottom-left. When a rotation of the bounding box is detected the rotation is represented as around the top-left corner as defined when the text is read in the 'natural' orientation. For example: * when the text is horizontal it might look like: 0----1 | | 3----2 * when it's rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3)."]
        pub bounding_box: ::std::option::Option<::std::boxed::Box<BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the OCR results for the paragraph. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information detected for the paragraph."]
        pub property: ::std::option::Option<::std::boxed::Box<TextProperty>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "words")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of all words in this paragraph."]
        pub words: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Word>>>,
    }
    impl Paragraph {
        pub fn builder() -> ParagraphBuilder {
            ParagraphBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A 3D position in the image, used primarily for Face detection landmarks. A valid Position must have both x and y coordinates. The position coordinates are in the same scale as the original image."]
    pub struct Position {
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
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "z")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Z coordinate (or depth)."]
        pub z: ::std::option::Option<::std::primitive::f64>,
    }
    impl Position {
        pub fn builder() -> PositionBuilder {
            PositionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Product contains ReferenceImages."]
    pub struct Product {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User-provided metadata to be stored with this product. Must be at most 4096 characters long."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user-provided name for this Product. Must not be empty. Must be at most 4096 characters long."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the product. Format is: `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`. This field is ignored when creating a product."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productCategory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Immutable. The category for the product identified by the reference image. This should be one of \"homegoods-v2\", \"apparel-v2\", \"toys-v2\", \"packagedgoods-v1\" or \"general-v1\". The legacy categories \"homegoods\", \"apparel\", and \"toys\" are still supported, but these should not be used for new products."]
        pub product_category: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Key-value pairs that can be attached to a product. At query time, constraints can be specified based on the product_labels. Note that integer values can be provided as strings, e.g. \"1199\". Only strings with integer values can match a range-based restriction which is to be supported soon. Multiple values can be assigned to the same key. One product may have up to 500 product_labels. Notice that the total number of distinct product_labels over all products in one ProductSet cannot exceed 1M, otherwise the product search pipeline will refuse to work for that ProductSet."]
        pub product_labels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<KeyValue>>>,
    }
    impl Product {
        pub fn builder() -> ProductBuilder {
            ProductBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Results for a product search request."]
    pub struct ProductSearchResults {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "indexTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp of the index which provided these results. Products added to the product set and products removed from the product set after this time are not reflected in the current results."]
        pub index_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productGroupedResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of results grouped by products detected in the query image. Each entry corresponds to one bounding polygon in the query image, and contains the matching products specific to that region. There may be duplicate product matches in the union of all the per-product results."]
        pub product_grouped_results:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GroupedResult>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of results, one for each product match."]
        pub results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Result>>>,
    }
    impl ProductSearchResults {
        pub fn builder() -> ProductSearchResultsBuilder {
            ProductSearchResultsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A `Property` consists of a user-supplied name/value pair."]
    pub struct Property {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the property."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uint64Value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value of numeric properties."]
        pub uint64_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value of the property."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl Property {
        pub fn builder() -> PropertyBuilder {
            PropertyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A `ReferenceImage` represents a product image and its associated metadata, such as bounding boxes."]
    pub struct ReferenceImage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingPolys")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Bounding polygons around the areas of interest in the reference image. If this field is empty, the system will try to detect regions of interest. At most 10 bounding polygons will be used. The provided shape is converted into a non-rotated rectangle. Once converted, the small edge of the rectangle must be greater than or equal to 300 pixels. The aspect ratio must be 1:4 or less (i.e. 1:3 is ok; 1:5 is not)."]
        pub bounding_polys: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BoundingPoly>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the reference image. Format is: `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID/referenceImages/IMAGE_ID`. This field is ignored when creating a reference image."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The Google Cloud Storage URI of the reference image. The URI must start with `gs://`."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl ReferenceImage {
        pub fn builder() -> ReferenceImageBuilder {
            ReferenceImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a product."]
    pub struct Result {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the image from the product that is the closest match to the query."]
        pub image: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "product")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Product."]
        pub product: ::std::option::Option<::std::boxed::Box<Product>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A confidence level on the match, ranging from 0 (no confidence) to 1 (full confidence)."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl Result {
        pub fn builder() -> ResultBuilder {
            ResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Set of features pertaining to the image, computed by computer vision methods over safe-search verticals (for example, adult, spoof, medical, violence)."]
    pub struct SafeSearchAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents the adult content likelihood for the image. Adult content may contain elements such as nudity, pornographic images or cartoons, or sexual activities."]
        pub adult: ::std::option::Option<SafeSearchAnnotationAdultEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "medical")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Likelihood that this is a medical image."]
        pub medical: ::std::option::Option<SafeSearchAnnotationMedicalEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "racy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Likelihood that the request image contains racy content. Racy content may include (but is not limited to) skimpy or sheer clothing, strategically covered nudity, lewd or provocative poses, or close-ups of sensitive body areas."]
        pub racy: ::std::option::Option<SafeSearchAnnotationRacyEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spoof")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Spoof likelihood. The likelihood that an modification was made to the image's canonical version to make it appear funny or offensive."]
        pub spoof: ::std::option::Option<SafeSearchAnnotationSpoofEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "violence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Likelihood that this image contains violent content."]
        pub violence: ::std::option::Option<SafeSearchAnnotationViolenceEnum>,
    }
    impl SafeSearchAnnotation {
        pub fn builder() -> SafeSearchAnnotationBuilder {
            SafeSearchAnnotationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Represents the adult content likelihood for the image. Adult content may contain elements such as nudity, pornographic images or cartoons, or sexual activities."]
    pub enum SafeSearchAnnotationAdultEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for SafeSearchAnnotationAdultEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Likelihood that this is a medical image."]
    pub enum SafeSearchAnnotationMedicalEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for SafeSearchAnnotationMedicalEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Likelihood that the request image contains racy content. Racy content may include (but is not limited to) skimpy or sheer clothing, strategically covered nudity, lewd or provocative poses, or close-ups of sensitive body areas."]
    pub enum SafeSearchAnnotationRacyEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for SafeSearchAnnotationRacyEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Spoof likelihood. The likelihood that an modification was made to the image's canonical version to make it appear funny or offensive."]
    pub enum SafeSearchAnnotationSpoofEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for SafeSearchAnnotationSpoofEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Likelihood that this image contains violent content."]
    pub enum SafeSearchAnnotationViolenceEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unknown likelihood."]
        Unknown,
        #[serde(rename = "VERY_UNLIKELY")]
        #[doc = "It is very unlikely."]
        VeryUnlikely,
        #[serde(rename = "UNLIKELY")]
        #[doc = "It is unlikely."]
        Unlikely,
        #[serde(rename = "POSSIBLE")]
        #[doc = "It is possible."]
        Possible,
        #[serde(rename = "LIKELY")]
        #[doc = "It is likely."]
        Likely,
        #[serde(rename = "VERY_LIKELY")]
        #[doc = "It is very likely."]
        VeryLikely,
    }
    impl ::std::default::Default for SafeSearchAnnotationViolenceEnum {
        fn default() -> Self {
            Self::Unknown
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
    #[doc = "A single symbol representation."]
    pub struct Symbol {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding box for the symbol. The vertices are in the order of top-left, top-right, bottom-right, bottom-left. When a rotation of the bounding box is detected the rotation is represented as around the top-left corner as defined when the text is read in the 'natural' orientation. For example: * when the text is horizontal it might look like: 0----1 | | 3----2 * when it's rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3)."]
        pub bounding_box: ::std::option::Option<::std::boxed::Box<BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the OCR results for the symbol. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information detected for the symbol."]
        pub property: ::std::option::Option<::std::boxed::Box<TextProperty>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The actual UTF-8 representation of the symbol."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl Symbol {
        pub fn builder() -> SymbolBuilder {
            SymbolBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "TextAnnotation contains a structured representation of OCR extracted text. The hierarchy of an OCR extracted text structure is like this: TextAnnotation -> Page -> Block -> Paragraph -> Word -> Symbol Each structural component, starting from Page, may further have their own properties. Properties describe detected languages, breaks etc.. Please refer to the TextAnnotation.TextProperty message definition below for more detail."]
    pub struct TextAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of pages detected by OCR."]
        pub pages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Page>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "UTF-8 text detected on the pages."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl TextAnnotation {
        pub fn builder() -> TextAnnotationBuilder {
            TextAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional information detected on the structural component."]
    pub struct TextProperty {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedBreak")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detected start or end of a text segment."]
        pub detected_break: ::std::option::Option<::std::boxed::Box<DetectedBreak>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of detected languages together with confidence."]
        pub detected_languages:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DetectedLanguage>>>,
    }
    impl TextProperty {
        pub fn builder() -> TextPropertyBuilder {
            TextPropertyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A vertex represents a 2D point in the image. NOTE: the vertex coordinates are in the same scale as the original image."]
    pub struct Vertex {
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
    impl Vertex {
        pub fn builder() -> VertexBuilder {
            VertexBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Relevant information for the image from the Internet."]
    pub struct WebDetection {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bestGuessLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The service's best guess as to the topic of the request image. Inferred from similar images on the open web."]
        pub best_guess_labels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WebLabel>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullMatchingImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fully matching images from the Internet. Can include resized copies of the query image."]
        pub full_matching_images:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WebImage>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pagesWithMatchingImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web pages containing the matching images from the Internet."]
        pub pages_with_matching_images:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WebPage>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialMatchingImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Partial matching images from the Internet. Those images are similar enough to share some key-point features. For example an original image will likely have partial matching for its crops."]
        pub partial_matching_images:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WebImage>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visuallySimilarImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visually similar image results."]
        pub visually_similar_images:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WebImage>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webEntities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deduced entities from similar images on the Internet."]
        pub web_entities: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WebEntity>>>,
    }
    impl WebDetection {
        pub fn builder() -> WebDetectionBuilder {
            WebDetectionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Entity deduced from similar images on the Internet."]
    pub struct WebEntity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Canonical description of the entity, in English."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Opaque entity ID."]
        pub entity_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Overall relevancy score for the entity. Not normalized and not comparable across different image queries."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl WebEntity {
        pub fn builder() -> WebEntityBuilder {
            WebEntityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for online images."]
    pub struct WebImage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "(Deprecated) Overall relevancy score for the image."]
        pub score: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result image URL."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl WebImage {
        pub fn builder() -> WebImageBuilder {
            WebImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Label to provide extra metadata for the web detection."]
    pub struct WebLabel {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "label")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label for extra metadata."]
        pub label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The BCP-47 language code for `label`, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        pub language_code: ::std::option::Option<::std::string::String>,
    }
    impl WebLabel {
        pub fn builder() -> WebLabelBuilder {
            WebLabelBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for web pages."]
    pub struct WebPage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullMatchingImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fully matching images on the page. Can include resized copies of the query image."]
        pub full_matching_images:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WebImage>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title for the web page, may contain HTML markups."]
        pub page_title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partialMatchingImages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Partial matching images on the page. Those images are similar enough to share some key-point features. For example an original image will likely have partial matching for its crops."]
        pub partial_matching_images:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<WebImage>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "(Deprecated) Overall relevancy score for the web page."]
        pub score: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result web page URL."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl WebPage {
        pub fn builder() -> WebPageBuilder {
            WebPageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A word representation."]
    pub struct Word {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundingBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bounding box for the word. The vertices are in the order of top-left, top-right, bottom-right, bottom-left. When a rotation of the bounding box is detected the rotation is represented as around the top-left corner as defined when the text is read in the 'natural' orientation. For example: * when the text is horizontal it might look like: 0----1 | | 3----2 * when it's rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3)."]
        pub bounding_box: ::std::option::Option<::std::boxed::Box<BoundingPoly>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Confidence of the OCR results for the word. Range [0, 1]."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information detected for the word."]
        pub property: ::std::option::Option<::std::boxed::Box<TextProperty>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "symbols")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of symbols in the word. The order of the symbols follows the natural reading order."]
        pub symbols: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Symbol>>>,
    }
    impl Word {
        pub fn builder() -> WordBuilder {
            WordBuilder::default()
        }
    }
}
