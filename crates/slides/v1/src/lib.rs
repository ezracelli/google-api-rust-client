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
    pub mod presentations {
        pub mod resources {
            pub mod pages {
                pub mod methods {
                    pub mod get_thumbnail {
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
                            #[serde(rename = "thumbnailProperties.mimeType")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The optional mime type of the thumbnail image. If you don't specify the mime type, the mime type defaults to PNG."]
                            pub thumbnail_properties_mime_type: ::std::option::Option<
                                QueryParametersThumbnailPropertiesMimeTypeEnum,
                            >,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "thumbnailProperties.thumbnailSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The optional thumbnail image size. If you don't specify the size, the server chooses a default size of the image."]
                            pub thumbnail_properties_thumbnail_size: ::std::option::Option<
                                QueryParametersThumbnailPropertiesThumbnailSizeEnum,
                            >,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "The optional mime type of the thumbnail image. If you don't specify the mime type, the mime type defaults to PNG."]
                        pub enum QueryParametersThumbnailPropertiesMimeTypeEnum {
                            #[serde(rename = "PNG")]
                            #[doc = "The default mime type."]
                            Png,
                        }
                        impl ::std::default::Default for QueryParametersThumbnailPropertiesMimeTypeEnum {
                            fn default() -> Self {
                                Self::Png
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "The optional thumbnail image size. If you don't specify the size, the server chooses a default size of the image."]
                        pub enum QueryParametersThumbnailPropertiesThumbnailSizeEnum {
                            #[serde(rename = "THUMBNAIL_SIZE_UNSPECIFIED")]
                            #[doc = "The default thumbnail image size. The unspecified thumbnail size implies that the server chooses the size of the image in a way that might vary in the future."]
                            ThumbnailSizeUnspecified,
                            #[serde(rename = "LARGE")]
                            #[doc = "The thumbnail image width of 1600px."]
                            Large,
                            #[serde(rename = "MEDIUM")]
                            #[doc = "The thumbnail image width of 800px."]
                            Medium,
                            #[serde(rename = "SMALL")]
                            #[doc = "The thumbnail image width of 200px."]
                            Small,
                        }
                        impl ::std::default::Default for QueryParametersThumbnailPropertiesThumbnailSizeEnum {
                            fn default() -> Self {
                                Self::ThumbnailSizeUnspecified
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
    #[doc = "AffineTransform uses a 3x3 matrix with an implied last row of [ 0 0 1 ] to transform source coordinates (x,y) into destination coordinates (x', y') according to: x' x = shear_y scale_y translate_y 1 [ 1 ] After transformation, x' = scale_x * x + shear_x * y + translate_x; y' = scale_y * y + shear_y * x + translate_y; This message is therefore composed of these six matrix elements."]
    pub struct AffineTransform {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scaleX")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The X coordinate scaling element."]
        pub scale_x: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scaleY")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Y coordinate scaling element."]
        pub scale_y: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shearX")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The X coordinate shearing element."]
        pub shear_x: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shearY")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Y coordinate shearing element."]
        pub shear_y: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "translateX")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The X coordinate translation element."]
        pub translate_x: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "translateY")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Y coordinate translation element."]
        pub translate_y: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The units for translate elements."]
        pub unit: ::std::option::Option<AffineTransformUnitEnum>,
    }
    impl AffineTransform {
        pub fn builder() -> AffineTransformBuilder {
            AffineTransformBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The units for translate elements."]
    pub enum AffineTransformUnitEnum {
        #[serde(rename = "UNIT_UNSPECIFIED")]
        #[doc = "The units are unknown."]
        UnitUnspecified,
        #[serde(rename = "EMU")]
        #[doc = "An English Metric Unit (EMU) is defined as 1/360,000 of a centimeter and thus there are 914,400 EMUs per inch, and 12,700 EMUs per point."]
        Emu,
        #[serde(rename = "PT")]
        #[doc = "A point, 1/72 of an inch."]
        Pt,
    }
    impl ::std::default::Default for AffineTransformUnitEnum {
        fn default() -> Self {
            Self::UnitUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A TextElement kind that represents auto text."]
    pub struct AutoText {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The rendered content of this auto text, if available."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "style")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The styling applied to this auto text."]
        pub style: ::std::option::Option<::std::boxed::Box<TextStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of this auto text."]
        pub _type: ::std::option::Option<AutoTextTypeEnum>,
    }
    impl AutoText {
        pub fn builder() -> AutoTextBuilder {
            AutoTextBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of this auto text."]
    pub enum AutoTextTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "An unspecified autotext type."]
        TypeUnspecified,
        #[serde(rename = "SLIDE_NUMBER")]
        #[doc = "Type for autotext that represents the current slide number."]
        SlideNumber,
    }
    impl ::std::default::Default for AutoTextTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The autofit properties of a Shape."]
    pub struct Autofit {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autofitType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The autofit type of the shape. If unspecified, the autofit type is inherited from a parent placeholder if it exists. The field will be automatically set to NONE if a request is made that may affect text fitting within its bounding text box. In this case the font_scale will be applied to the font_size and the line_spacing_reduction will be applied to the line_spacing. Both properties would also be reset to default values."]
        pub autofit_type: ::std::option::Option<AutofitAutofitTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fontScale")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The font scale applied to the shape. For shapes with autofit_type NONE or SHAPE_AUTOFIT, this value will be the default value of 1. For TEXT_AUTOFIT, this value multiplied by the font_size will give the font size that is rendered in the editor. This property is read-only."]
        pub font_scale: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineSpacingReduction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The line spacing reduction applied to the shape. For shapes with autofit_type NONE or SHAPE_AUTOFIT, this value will be the default value of 0. For TEXT_AUTOFIT, this value subtracted from the line_spacing will give the line spacing that is rendered in the editor. This property is read-only."]
        pub line_spacing_reduction: ::std::option::Option<::std::primitive::f64>,
    }
    impl Autofit {
        pub fn builder() -> AutofitBuilder {
            AutofitBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The autofit type of the shape. If unspecified, the autofit type is inherited from a parent placeholder if it exists. The field will be automatically set to NONE if a request is made that may affect text fitting within its bounding text box. In this case the font_scale will be applied to the font_size and the line_spacing_reduction will be applied to the line_spacing. Both properties would also be reset to default values."]
    pub enum AutofitAutofitTypeEnum {
        #[serde(rename = "AUTOFIT_TYPE_UNSPECIFIED")]
        #[doc = "The autofit type is unspecified."]
        AutofitTypeUnspecified,
        #[serde(rename = "NONE")]
        #[doc = "Do not autofit."]
        None,
        #[serde(rename = "TEXT_AUTOFIT")]
        #[doc = "Shrink text on overflow to fit shape."]
        TextAutofit,
        #[serde(rename = "SHAPE_AUTOFIT")]
        #[doc = "Resize shape to fit text."]
        ShapeAutofit,
    }
    impl ::std::default::Default for AutofitAutofitTypeEnum {
        fn default() -> Self {
            Self::AutofitTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for PresentationsService.BatchUpdatePresentation."]
    pub struct BatchUpdatePresentationRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requests")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of updates to apply to the presentation."]
        pub requests: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Request>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "writeControl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Provides control over how write requests are executed."]
        pub write_control: ::std::option::Option<::std::boxed::Box<WriteControl>>,
    }
    impl BatchUpdatePresentationRequest {
        pub fn builder() -> BatchUpdatePresentationRequestBuilder {
            BatchUpdatePresentationRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message from a batch update."]
    pub struct BatchUpdatePresentationResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "presentationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The presentation the updates were applied to."]
        pub presentation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reply of the updates. This maps 1:1 with the updates, although replies to some requests may be empty."]
        pub replies: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Response>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "writeControl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The updated write control after applying the request."]
        pub write_control: ::std::option::Option<::std::boxed::Box<WriteControl>>,
    }
    impl BatchUpdatePresentationResponse {
        pub fn builder() -> BatchUpdatePresentationResponseBuilder {
            BatchUpdatePresentationResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes the bullet of a paragraph."]
    pub struct Bullet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bulletStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The paragraph specific text style applied to this bullet."]
        pub bullet_style: ::std::option::Option<::std::boxed::Box<TextStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "glyph")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The rendered bullet glyph for this paragraph."]
        pub glyph: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "listId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the list this paragraph belongs to."]
        pub list_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nestingLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The nesting level of this paragraph in the list."]
        pub nesting_level: ::std::option::Option<::std::primitive::i64>,
    }
    impl Bullet {
        pub fn builder() -> BulletBuilder {
            BulletBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The palette of predefined colors for a page."]
    pub struct ColorScheme {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ThemeColorType and corresponding concrete color pairs."]
        pub colors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ThemeColorPair>>>,
    }
    impl ColorScheme {
        pub fn builder() -> ColorSchemeBuilder {
            ColorSchemeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A color and position in a gradient band."]
    pub struct ColorStop {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alpha")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The alpha value of this color in the gradient band. Defaults to 1.0, fully opaque."]
        pub alpha: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "color")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the gradient stop."]
        pub color: ::std::option::Option<::std::boxed::Box<OpaqueColor>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "position")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The relative position of the color stop in the gradient band measured in percentage. The value should be in the interval [0.0, 1.0]."]
        pub position: ::std::option::Option<::std::primitive::f64>,
    }
    impl ColorStop {
        pub fn builder() -> ColorStopBuilder {
            ColorStopBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Creates an image."]
    pub struct CreateImageRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "elementProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The element properties for the image. When the aspect ratio of the provided size does not match the image aspect ratio, the image is scaled and centered with respect to the size in order to maintain aspect ratio. The provided transform is applied after this operation. The PageElementProperties.size property is optional. If you don't specify the size, the default size of the image is used. The PageElementProperties.transform property is optional. If you don't specify a transform, the image will be placed at the top left corner of the page."]
        pub element_properties: ::std::option::Option<::std::boxed::Box<PageElementProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A user-supplied object ID. If you specify an ID, it must be unique among all pages and page elements in the presentation. The ID must start with an alphanumeric character or an underscore (matches regex `[a-zA-Z0-9_]`); remaining characters may include those as well as a hyphen or colon (matches regex `[a-zA-Z0-9_-:]`). The length of the ID must not be less than 5 or greater than 50. If you don't specify an ID, a unique one is generated."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The image URL. The image is fetched once at insertion time and a copy is stored for display inside the presentation. Images must be less than 50MB in size, cannot exceed 25 megapixels, and must be in one of PNG, JPEG, or GIF format. The provided URL can be at most 2 kB in length. The URL itself is saved with the image, and exposed via the Image.source_url field."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl CreateImageRequest {
        pub fn builder() -> CreateImageRequestBuilder {
            CreateImageRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of creating an image."]
    pub struct CreateImageResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the created image."]
        pub object_id: ::std::option::Option<::std::string::String>,
    }
    impl CreateImageResponse {
        pub fn builder() -> CreateImageResponseBuilder {
            CreateImageResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Creates a line."]
    pub struct CreateLineRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "category")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The category of the line to be created. The exact line type created is determined based on the category and how it's routed to connect to other page elements. If you specify both a `category` and a `line_category`, the `category` takes precedence. If you do not specify a value for `category`, but specify a value for `line_category`, then the specified `line_category` value is used. If you do not specify either, then STRAIGHT is used."]
        pub category: ::std::option::Option<CreateLineRequestCategoryEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "elementProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The element properties for the line."]
        pub element_properties: ::std::option::Option<::std::boxed::Box<PageElementProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineCategory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The category of the line to be created. *Deprecated*: use `category` instead. The exact line type created is determined based on the category and how it's routed to connect to other page elements. If you specify both a `category` and a `line_category`, the `category` takes precedence."]
        pub line_category: ::std::option::Option<CreateLineRequestLineCategoryEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A user-supplied object ID. If you specify an ID, it must be unique among all pages and page elements in the presentation. The ID must start with an alphanumeric character or an underscore (matches regex `[a-zA-Z0-9_]`); remaining characters may include those as well as a hyphen or colon (matches regex `[a-zA-Z0-9_-:]`). The length of the ID must not be less than 5 or greater than 50. If you don't specify an ID, a unique one is generated."]
        pub object_id: ::std::option::Option<::std::string::String>,
    }
    impl CreateLineRequest {
        pub fn builder() -> CreateLineRequestBuilder {
            CreateLineRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The category of the line to be created. The exact line type created is determined based on the category and how it's routed to connect to other page elements. If you specify both a `category` and a `line_category`, the `category` takes precedence. If you do not specify a value for `category`, but specify a value for `line_category`, then the specified `line_category` value is used. If you do not specify either, then STRAIGHT is used."]
    pub enum CreateLineRequestCategoryEnum {
        #[serde(rename = "LINE_CATEGORY_UNSPECIFIED")]
        #[doc = "Unspecified line category."]
        LineCategoryUnspecified,
        #[serde(rename = "STRAIGHT")]
        #[doc = "Straight connectors, including straight connector 1."]
        Straight,
        #[serde(rename = "BENT")]
        #[doc = "Bent connectors, including bent connector 2 to 5."]
        Bent,
        #[serde(rename = "CURVED")]
        #[doc = "Curved connectors, including curved connector 2 to 5."]
        Curved,
    }
    impl ::std::default::Default for CreateLineRequestCategoryEnum {
        fn default() -> Self {
            Self::LineCategoryUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The category of the line to be created. *Deprecated*: use `category` instead. The exact line type created is determined based on the category and how it's routed to connect to other page elements. If you specify both a `category` and a `line_category`, the `category` takes precedence."]
    pub enum CreateLineRequestLineCategoryEnum {
        #[serde(rename = "STRAIGHT")]
        #[doc = "Straight connectors, including straight connector 1. The is the default category when one is not specified."]
        Straight,
        #[serde(rename = "BENT")]
        #[doc = "Bent connectors, including bent connector 2 to 5."]
        Bent,
        #[serde(rename = "CURVED")]
        #[doc = "Curved connectors, including curved connector 2 to 5."]
        Curved,
    }
    impl ::std::default::Default for CreateLineRequestLineCategoryEnum {
        fn default() -> Self {
            Self::Straight
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of creating a line."]
    pub struct CreateLineResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the created line."]
        pub object_id: ::std::option::Option<::std::string::String>,
    }
    impl CreateLineResponse {
        pub fn builder() -> CreateLineResponseBuilder {
            CreateLineResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Creates bullets for all of the paragraphs that overlap with the given text index range. The nesting level of each paragraph will be determined by counting leading tabs in front of each paragraph. To avoid excess space between the bullet and the corresponding paragraph, these leading tabs are removed by this request. This may change the indices of parts of the text. If the paragraph immediately before paragraphs being updated is in a list with a matching preset, the paragraphs being updated are added to that preceding list."]
    pub struct CreateParagraphBulletsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bulletPreset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The kinds of bullet glyphs to be used. Defaults to the `BULLET_DISC_CIRCLE_SQUARE` preset."]
        pub bullet_preset: ::std::option::Option<CreateParagraphBulletsRequestBulletPresetEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cellLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The optional table cell location if the text to be modified is in a table cell. If present, the object_id must refer to a table."]
        pub cell_location: ::std::option::Option<::std::boxed::Box<TableCellLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the shape or table containing the text to add bullets to."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range of text to apply the bullet presets to, based on TextElement indexes."]
        pub text_range: ::std::option::Option<::std::boxed::Box<Range>>,
    }
    impl CreateParagraphBulletsRequest {
        pub fn builder() -> CreateParagraphBulletsRequestBuilder {
            CreateParagraphBulletsRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The kinds of bullet glyphs to be used. Defaults to the `BULLET_DISC_CIRCLE_SQUARE` preset."]
    pub enum CreateParagraphBulletsRequestBulletPresetEnum {
        #[serde(rename = "BULLET_DISC_CIRCLE_SQUARE")]
        #[doc = "A bulleted list with a `DISC`, `CIRCLE` and `SQUARE` bullet glyph for the first 3 list nesting levels."]
        BulletDiscCircleSquare,
        #[serde(rename = "BULLET_DIAMONDX_ARROW3D_SQUARE")]
        #[doc = "A bulleted list with a `DIAMONDX`, `ARROW3D` and `SQUARE` bullet glyph for the first 3 list nesting levels."]
        BulletDiamondxArrow3DSquare,
        #[serde(rename = "BULLET_CHECKBOX")]
        #[doc = "A bulleted list with `CHECKBOX` bullet glyphs for all list nesting levels."]
        BulletCheckbox,
        #[serde(rename = "BULLET_ARROW_DIAMOND_DISC")]
        #[doc = "A bulleted list with a `ARROW`, `DIAMOND` and `DISC` bullet glyph for the first 3 list nesting levels."]
        BulletArrowDiamondDisc,
        #[serde(rename = "BULLET_STAR_CIRCLE_SQUARE")]
        #[doc = "A bulleted list with a `STAR`, `CIRCLE` and `SQUARE` bullet glyph for the first 3 list nesting levels."]
        BulletStarCircleSquare,
        #[serde(rename = "BULLET_ARROW3D_CIRCLE_SQUARE")]
        #[doc = "A bulleted list with a `ARROW3D`, `CIRCLE` and `SQUARE` bullet glyph for the first 3 list nesting levels."]
        BulletArrow3DCircleSquare,
        #[serde(rename = "BULLET_LEFTTRIANGLE_DIAMOND_DISC")]
        #[doc = "A bulleted list with a `LEFTTRIANGLE`, `DIAMOND` and `DISC` bullet glyph for the first 3 list nesting levels."]
        BulletLefttriangleDiamondDisc,
        #[serde(rename = "BULLET_DIAMONDX_HOLLOWDIAMOND_SQUARE")]
        #[doc = "A bulleted list with a `DIAMONDX`, `HOLLOWDIAMOND` and `SQUARE` bullet glyph for the first 3 list nesting levels."]
        BulletDiamondxHollowdiamondSquare,
        #[serde(rename = "BULLET_DIAMOND_CIRCLE_SQUARE")]
        #[doc = "A bulleted list with a `DIAMOND`, `CIRCLE` and `SQUARE` bullet glyph for the first 3 list nesting levels."]
        BulletDiamondCircleSquare,
        #[serde(rename = "NUMBERED_DIGIT_ALPHA_ROMAN")]
        #[doc = "A numbered list with `DIGIT`, `ALPHA` and `ROMAN` numeric glyphs for the first 3 list nesting levels, followed by periods."]
        NumberedDigitAlphaRoman,
        #[serde(rename = "NUMBERED_DIGIT_ALPHA_ROMAN_PARENS")]
        #[doc = "A numbered list with `DIGIT`, `ALPHA` and `ROMAN` numeric glyphs for the first 3 list nesting levels, followed by parenthesis."]
        NumberedDigitAlphaRomanParens,
        #[serde(rename = "NUMBERED_DIGIT_NESTED")]
        #[doc = "A numbered list with `DIGIT` numeric glyphs separated by periods, where each nesting level uses the previous nesting level's glyph as a prefix. For example: '1.', '1.1.', '2.', '2.2.'."]
        NumberedDigitNested,
        #[serde(rename = "NUMBERED_UPPERALPHA_ALPHA_ROMAN")]
        #[doc = "A numbered list with `UPPERALPHA`, `ALPHA` and `ROMAN` numeric glyphs for the first 3 list nesting levels, followed by periods."]
        NumberedUpperalphaAlphaRoman,
        #[serde(rename = "NUMBERED_UPPERROMAN_UPPERALPHA_DIGIT")]
        #[doc = "A numbered list with `UPPERROMAN`, `UPPERALPHA` and `DIGIT` numeric glyphs for the first 3 list nesting levels, followed by periods."]
        NumberedUpperromanUpperalphaDigit,
        #[serde(rename = "NUMBERED_ZERODIGIT_ALPHA_ROMAN")]
        #[doc = "A numbered list with `ZERODIGIT`, `ALPHA` and `ROMAN` numeric glyphs for the first 3 list nesting levels, followed by periods."]
        NumberedZerodigitAlphaRoman,
    }
    impl ::std::default::Default for CreateParagraphBulletsRequestBulletPresetEnum {
        fn default() -> Self {
            Self::BulletDiscCircleSquare
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Creates a new shape."]
    pub struct CreateShapeRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "elementProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The element properties for the shape."]
        pub element_properties: ::std::option::Option<::std::boxed::Box<PageElementProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A user-supplied object ID. If you specify an ID, it must be unique among all pages and page elements in the presentation. The ID must start with an alphanumeric character or an underscore (matches regex `[a-zA-Z0-9_]`); remaining characters may include those as well as a hyphen or colon (matches regex `[a-zA-Z0-9_-:]`). The length of the ID must not be less than 5 or greater than 50. If empty, a unique identifier will be generated."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shapeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The shape type."]
        pub shape_type: ::std::option::Option<CreateShapeRequestShapeTypeEnum>,
    }
    impl CreateShapeRequest {
        pub fn builder() -> CreateShapeRequestBuilder {
            CreateShapeRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The shape type."]
    pub enum CreateShapeRequestShapeTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "The shape type that is not predefined."]
        TypeUnspecified,
        #[serde(rename = "TEXT_BOX")]
        #[doc = "Text box shape."]
        TextBox,
        #[serde(rename = "RECTANGLE")]
        #[doc = "Rectangle shape. Corresponds to ECMA-376 ST_ShapeType 'rect'."]
        Rectangle,
        #[serde(rename = "ROUND_RECTANGLE")]
        #[doc = "Round corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType 'roundRect'"]
        RoundRectangle,
        #[serde(rename = "ELLIPSE")]
        #[doc = "Ellipse shape. Corresponds to ECMA-376 ST_ShapeType 'ellipse'"]
        Ellipse,
        #[serde(rename = "ARC")]
        #[doc = "Curved arc shape. Corresponds to ECMA-376 ST_ShapeType 'arc'"]
        Arc,
        #[serde(rename = "BENT_ARROW")]
        #[doc = "Bent arrow shape. Corresponds to ECMA-376 ST_ShapeType 'bentArrow'"]
        BentArrow,
        #[serde(rename = "BENT_UP_ARROW")]
        #[doc = "Bent up arrow shape. Corresponds to ECMA-376 ST_ShapeType 'bentUpArrow'"]
        BentUpArrow,
        #[serde(rename = "BEVEL")]
        #[doc = "Bevel shape. Corresponds to ECMA-376 ST_ShapeType 'bevel'"]
        Bevel,
        #[serde(rename = "BLOCK_ARC")]
        #[doc = "Block arc shape. Corresponds to ECMA-376 ST_ShapeType 'blockArc'"]
        BlockArc,
        #[serde(rename = "BRACE_PAIR")]
        #[doc = "Brace pair shape. Corresponds to ECMA-376 ST_ShapeType 'bracePair'"]
        BracePair,
        #[serde(rename = "BRACKET_PAIR")]
        #[doc = "Bracket pair shape. Corresponds to ECMA-376 ST_ShapeType 'bracketPair'"]
        BracketPair,
        #[serde(rename = "CAN")]
        #[doc = "Can shape. Corresponds to ECMA-376 ST_ShapeType 'can'"]
        Can,
        #[serde(rename = "CHEVRON")]
        #[doc = "Chevron shape. Corresponds to ECMA-376 ST_ShapeType 'chevron'"]
        Chevron,
        #[serde(rename = "CHORD")]
        #[doc = "Chord shape. Corresponds to ECMA-376 ST_ShapeType 'chord'"]
        Chord,
        #[serde(rename = "CLOUD")]
        #[doc = "Cloud shape. Corresponds to ECMA-376 ST_ShapeType 'cloud'"]
        Cloud,
        #[serde(rename = "CORNER")]
        #[doc = "Corner shape. Corresponds to ECMA-376 ST_ShapeType 'corner'"]
        Corner,
        #[serde(rename = "CUBE")]
        #[doc = "Cube shape. Corresponds to ECMA-376 ST_ShapeType 'cube'"]
        Cube,
        #[serde(rename = "CURVED_DOWN_ARROW")]
        #[doc = "Curved down arrow shape. Corresponds to ECMA-376 ST_ShapeType 'curvedDownArrow'"]
        CurvedDownArrow,
        #[serde(rename = "CURVED_LEFT_ARROW")]
        #[doc = "Curved left arrow shape. Corresponds to ECMA-376 ST_ShapeType 'curvedLeftArrow'"]
        CurvedLeftArrow,
        #[serde(rename = "CURVED_RIGHT_ARROW")]
        #[doc = "Curved right arrow shape. Corresponds to ECMA-376 ST_ShapeType 'curvedRightArrow'"]
        CurvedRightArrow,
        #[serde(rename = "CURVED_UP_ARROW")]
        #[doc = "Curved up arrow shape. Corresponds to ECMA-376 ST_ShapeType 'curvedUpArrow'"]
        CurvedUpArrow,
        #[serde(rename = "DECAGON")]
        #[doc = "Decagon shape. Corresponds to ECMA-376 ST_ShapeType 'decagon'"]
        Decagon,
        #[serde(rename = "DIAGONAL_STRIPE")]
        #[doc = "Diagonal stripe shape. Corresponds to ECMA-376 ST_ShapeType 'diagStripe'"]
        DiagonalStripe,
        #[serde(rename = "DIAMOND")]
        #[doc = "Diamond shape. Corresponds to ECMA-376 ST_ShapeType 'diamond'"]
        Diamond,
        #[serde(rename = "DODECAGON")]
        #[doc = "Dodecagon shape. Corresponds to ECMA-376 ST_ShapeType 'dodecagon'"]
        Dodecagon,
        #[serde(rename = "DONUT")]
        #[doc = "Donut shape. Corresponds to ECMA-376 ST_ShapeType 'donut'"]
        Donut,
        #[serde(rename = "DOUBLE_WAVE")]
        #[doc = "Double wave shape. Corresponds to ECMA-376 ST_ShapeType 'doubleWave'"]
        DoubleWave,
        #[serde(rename = "DOWN_ARROW")]
        #[doc = "Down arrow shape. Corresponds to ECMA-376 ST_ShapeType 'downArrow'"]
        DownArrow,
        #[serde(rename = "DOWN_ARROW_CALLOUT")]
        #[doc = "Callout down arrow shape. Corresponds to ECMA-376 ST_ShapeType 'downArrowCallout'"]
        DownArrowCallout,
        #[serde(rename = "FOLDED_CORNER")]
        #[doc = "Folded corner shape. Corresponds to ECMA-376 ST_ShapeType 'foldedCorner'"]
        FoldedCorner,
        #[serde(rename = "FRAME")]
        #[doc = "Frame shape. Corresponds to ECMA-376 ST_ShapeType 'frame'"]
        Frame,
        #[serde(rename = "HALF_FRAME")]
        #[doc = "Half frame shape. Corresponds to ECMA-376 ST_ShapeType 'halfFrame'"]
        HalfFrame,
        #[serde(rename = "HEART")]
        #[doc = "Heart shape. Corresponds to ECMA-376 ST_ShapeType 'heart'"]
        Heart,
        #[serde(rename = "HEPTAGON")]
        #[doc = "Heptagon shape. Corresponds to ECMA-376 ST_ShapeType 'heptagon'"]
        Heptagon,
        #[serde(rename = "HEXAGON")]
        #[doc = "Hexagon shape. Corresponds to ECMA-376 ST_ShapeType 'hexagon'"]
        Hexagon,
        #[serde(rename = "HOME_PLATE")]
        #[doc = "Home plate shape. Corresponds to ECMA-376 ST_ShapeType 'homePlate'"]
        HomePlate,
        #[serde(rename = "HORIZONTAL_SCROLL")]
        #[doc = "Horizontal scroll shape. Corresponds to ECMA-376 ST_ShapeType 'horizontalScroll'"]
        HorizontalScroll,
        #[serde(rename = "IRREGULAR_SEAL_1")]
        #[doc = "Irregular seal 1 shape. Corresponds to ECMA-376 ST_ShapeType 'irregularSeal1'"]
        IrregularSeal1,
        #[serde(rename = "IRREGULAR_SEAL_2")]
        #[doc = "Irregular seal 2 shape. Corresponds to ECMA-376 ST_ShapeType 'irregularSeal2'"]
        IrregularSeal2,
        #[serde(rename = "LEFT_ARROW")]
        #[doc = "Left arrow shape. Corresponds to ECMA-376 ST_ShapeType 'leftArrow'"]
        LeftArrow,
        #[serde(rename = "LEFT_ARROW_CALLOUT")]
        #[doc = "Callout left arrow shape. Corresponds to ECMA-376 ST_ShapeType 'leftArrowCallout'"]
        LeftArrowCallout,
        #[serde(rename = "LEFT_BRACE")]
        #[doc = "Left brace shape. Corresponds to ECMA-376 ST_ShapeType 'leftBrace'"]
        LeftBrace,
        #[serde(rename = "LEFT_BRACKET")]
        #[doc = "Left bracket shape. Corresponds to ECMA-376 ST_ShapeType 'leftBracket'"]
        LeftBracket,
        #[serde(rename = "LEFT_RIGHT_ARROW")]
        #[doc = "Left right arrow shape. Corresponds to ECMA-376 ST_ShapeType 'leftRightArrow'"]
        LeftRightArrow,
        #[serde(rename = "LEFT_RIGHT_ARROW_CALLOUT")]
        #[doc = "Callout left right arrow shape. Corresponds to ECMA-376 ST_ShapeType 'leftRightArrowCallout'"]
        LeftRightArrowCallout,
        #[serde(rename = "LEFT_RIGHT_UP_ARROW")]
        #[doc = "Left right up arrow shape. Corresponds to ECMA-376 ST_ShapeType 'leftRightUpArrow'"]
        LeftRightUpArrow,
        #[serde(rename = "LEFT_UP_ARROW")]
        #[doc = "Left up arrow shape. Corresponds to ECMA-376 ST_ShapeType 'leftUpArrow'"]
        LeftUpArrow,
        #[serde(rename = "LIGHTNING_BOLT")]
        #[doc = "Lightning bolt shape. Corresponds to ECMA-376 ST_ShapeType 'lightningBolt'"]
        LightningBolt,
        #[serde(rename = "MATH_DIVIDE")]
        #[doc = "Divide math shape. Corresponds to ECMA-376 ST_ShapeType 'mathDivide'"]
        MathDivide,
        #[serde(rename = "MATH_EQUAL")]
        #[doc = "Equal math shape. Corresponds to ECMA-376 ST_ShapeType 'mathEqual'"]
        MathEqual,
        #[serde(rename = "MATH_MINUS")]
        #[doc = "Minus math shape. Corresponds to ECMA-376 ST_ShapeType 'mathMinus'"]
        MathMinus,
        #[serde(rename = "MATH_MULTIPLY")]
        #[doc = "Multiply math shape. Corresponds to ECMA-376 ST_ShapeType 'mathMultiply'"]
        MathMultiply,
        #[serde(rename = "MATH_NOT_EQUAL")]
        #[doc = "Not equal math shape. Corresponds to ECMA-376 ST_ShapeType 'mathNotEqual'"]
        MathNotEqual,
        #[serde(rename = "MATH_PLUS")]
        #[doc = "Plus math shape. Corresponds to ECMA-376 ST_ShapeType 'mathPlus'"]
        MathPlus,
        #[serde(rename = "MOON")]
        #[doc = "Moon shape. Corresponds to ECMA-376 ST_ShapeType 'moon'"]
        Moon,
        #[serde(rename = "NO_SMOKING")]
        #[doc = "No smoking shape. Corresponds to ECMA-376 ST_ShapeType 'noSmoking'"]
        NoSmoking,
        #[serde(rename = "NOTCHED_RIGHT_ARROW")]
        #[doc = "Notched right arrow shape. Corresponds to ECMA-376 ST_ShapeType 'notchedRightArrow'"]
        NotchedRightArrow,
        #[serde(rename = "OCTAGON")]
        #[doc = "Octagon shape. Corresponds to ECMA-376 ST_ShapeType 'octagon'"]
        Octagon,
        #[serde(rename = "PARALLELOGRAM")]
        #[doc = "Parallelogram shape. Corresponds to ECMA-376 ST_ShapeType 'parallelogram'"]
        Parallelogram,
        #[serde(rename = "PENTAGON")]
        #[doc = "Pentagon shape. Corresponds to ECMA-376 ST_ShapeType 'pentagon'"]
        Pentagon,
        #[serde(rename = "PIE")]
        #[doc = "Pie shape. Corresponds to ECMA-376 ST_ShapeType 'pie'"]
        Pie,
        #[serde(rename = "PLAQUE")]
        #[doc = "Plaque shape. Corresponds to ECMA-376 ST_ShapeType 'plaque'"]
        Plaque,
        #[serde(rename = "PLUS")]
        #[doc = "Plus shape. Corresponds to ECMA-376 ST_ShapeType 'plus'"]
        Plus,
        #[serde(rename = "QUAD_ARROW")]
        #[doc = "Quad-arrow shape. Corresponds to ECMA-376 ST_ShapeType 'quadArrow'"]
        QuadArrow,
        #[serde(rename = "QUAD_ARROW_CALLOUT")]
        #[doc = "Callout quad-arrow shape. Corresponds to ECMA-376 ST_ShapeType 'quadArrowCallout'"]
        QuadArrowCallout,
        #[serde(rename = "RIBBON")]
        #[doc = "Ribbon shape. Corresponds to ECMA-376 ST_ShapeType 'ribbon'"]
        Ribbon,
        #[serde(rename = "RIBBON_2")]
        #[doc = "Ribbon 2 shape. Corresponds to ECMA-376 ST_ShapeType 'ribbon2'"]
        Ribbon2,
        #[serde(rename = "RIGHT_ARROW")]
        #[doc = "Right arrow shape. Corresponds to ECMA-376 ST_ShapeType 'rightArrow'"]
        RightArrow,
        #[serde(rename = "RIGHT_ARROW_CALLOUT")]
        #[doc = "Callout right arrow shape. Corresponds to ECMA-376 ST_ShapeType 'rightArrowCallout'"]
        RightArrowCallout,
        #[serde(rename = "RIGHT_BRACE")]
        #[doc = "Right brace shape. Corresponds to ECMA-376 ST_ShapeType 'rightBrace'"]
        RightBrace,
        #[serde(rename = "RIGHT_BRACKET")]
        #[doc = "Right bracket shape. Corresponds to ECMA-376 ST_ShapeType 'rightBracket'"]
        RightBracket,
        #[serde(rename = "ROUND_1_RECTANGLE")]
        #[doc = "One round corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType 'round1Rect'"]
        Round1Rectangle,
        #[serde(rename = "ROUND_2_DIAGONAL_RECTANGLE")]
        #[doc = "Two diagonal round corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType 'round2DiagRect'"]
        Round2DiagonalRectangle,
        #[serde(rename = "ROUND_2_SAME_RECTANGLE")]
        #[doc = "Two same-side round corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType 'round2SameRect'"]
        Round2SameRectangle,
        #[serde(rename = "RIGHT_TRIANGLE")]
        #[doc = "Right triangle shape. Corresponds to ECMA-376 ST_ShapeType 'rtTriangle'"]
        RightTriangle,
        #[serde(rename = "SMILEY_FACE")]
        #[doc = "Smiley face shape. Corresponds to ECMA-376 ST_ShapeType 'smileyFace'"]
        SmileyFace,
        #[serde(rename = "SNIP_1_RECTANGLE")]
        #[doc = "One snip corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType 'snip1Rect'"]
        Snip1Rectangle,
        #[serde(rename = "SNIP_2_DIAGONAL_RECTANGLE")]
        #[doc = "Two diagonal snip corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType 'snip2DiagRect'"]
        Snip2DiagonalRectangle,
        #[serde(rename = "SNIP_2_SAME_RECTANGLE")]
        #[doc = "Two same-side snip corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType 'snip2SameRect'"]
        Snip2SameRectangle,
        #[serde(rename = "SNIP_ROUND_RECTANGLE")]
        #[doc = "One snip one round corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType 'snipRoundRect'"]
        SnipRoundRectangle,
        #[serde(rename = "STAR_10")]
        #[doc = "Ten pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star10'"]
        Star10,
        #[serde(rename = "STAR_12")]
        #[doc = "Twelve pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star12'"]
        Star12,
        #[serde(rename = "STAR_16")]
        #[doc = "Sixteen pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star16'"]
        Star16,
        #[serde(rename = "STAR_24")]
        #[doc = "Twenty four pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star24'"]
        Star24,
        #[serde(rename = "STAR_32")]
        #[doc = "Thirty two pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star32'"]
        Star32,
        #[serde(rename = "STAR_4")]
        #[doc = "Four pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star4'"]
        Star4,
        #[serde(rename = "STAR_5")]
        #[doc = "Five pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star5'"]
        Star5,
        #[serde(rename = "STAR_6")]
        #[doc = "Six pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star6'"]
        Star6,
        #[serde(rename = "STAR_7")]
        #[doc = "Seven pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star7'"]
        Star7,
        #[serde(rename = "STAR_8")]
        #[doc = "Eight pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star8'"]
        Star8,
        #[serde(rename = "STRIPED_RIGHT_ARROW")]
        #[doc = "Striped right arrow shape. Corresponds to ECMA-376 ST_ShapeType 'stripedRightArrow'"]
        StripedRightArrow,
        #[serde(rename = "SUN")]
        #[doc = "Sun shape. Corresponds to ECMA-376 ST_ShapeType 'sun'"]
        Sun,
        #[serde(rename = "TRAPEZOID")]
        #[doc = "Trapezoid shape. Corresponds to ECMA-376 ST_ShapeType 'trapezoid'"]
        Trapezoid,
        #[serde(rename = "TRIANGLE")]
        #[doc = "Triangle shape. Corresponds to ECMA-376 ST_ShapeType 'triangle'"]
        Triangle,
        #[serde(rename = "UP_ARROW")]
        #[doc = "Up arrow shape. Corresponds to ECMA-376 ST_ShapeType 'upArrow'"]
        UpArrow,
        #[serde(rename = "UP_ARROW_CALLOUT")]
        #[doc = "Callout up arrow shape. Corresponds to ECMA-376 ST_ShapeType 'upArrowCallout'"]
        UpArrowCallout,
        #[serde(rename = "UP_DOWN_ARROW")]
        #[doc = "Up down arrow shape. Corresponds to ECMA-376 ST_ShapeType 'upDownArrow'"]
        UpDownArrow,
        #[serde(rename = "UTURN_ARROW")]
        #[doc = "U-turn arrow shape. Corresponds to ECMA-376 ST_ShapeType 'uturnArrow'"]
        UturnArrow,
        #[serde(rename = "VERTICAL_SCROLL")]
        #[doc = "Vertical scroll shape. Corresponds to ECMA-376 ST_ShapeType 'verticalScroll'"]
        VerticalScroll,
        #[serde(rename = "WAVE")]
        #[doc = "Wave shape. Corresponds to ECMA-376 ST_ShapeType 'wave'"]
        Wave,
        #[serde(rename = "WEDGE_ELLIPSE_CALLOUT")]
        #[doc = "Callout wedge ellipse shape. Corresponds to ECMA-376 ST_ShapeType 'wedgeEllipseCallout'"]
        WedgeEllipseCallout,
        #[serde(rename = "WEDGE_RECTANGLE_CALLOUT")]
        #[doc = "Callout wedge rectangle shape. Corresponds to ECMA-376 ST_ShapeType 'wedgeRectCallout'"]
        WedgeRectangleCallout,
        #[serde(rename = "WEDGE_ROUND_RECTANGLE_CALLOUT")]
        #[doc = "Callout wedge round rectangle shape. Corresponds to ECMA-376 ST_ShapeType 'wedgeRoundRectCallout'"]
        WedgeRoundRectangleCallout,
        #[serde(rename = "FLOW_CHART_ALTERNATE_PROCESS")]
        #[doc = "Alternate process flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartAlternateProcess'"]
        FlowChartAlternateProcess,
        #[serde(rename = "FLOW_CHART_COLLATE")]
        #[doc = "Collate flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartCollate'"]
        FlowChartCollate,
        #[serde(rename = "FLOW_CHART_CONNECTOR")]
        #[doc = "Connector flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartConnector'"]
        FlowChartConnector,
        #[serde(rename = "FLOW_CHART_DECISION")]
        #[doc = "Decision flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartDecision'"]
        FlowChartDecision,
        #[serde(rename = "FLOW_CHART_DELAY")]
        #[doc = "Delay flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartDelay'"]
        FlowChartDelay,
        #[serde(rename = "FLOW_CHART_DISPLAY")]
        #[doc = "Display flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartDisplay'"]
        FlowChartDisplay,
        #[serde(rename = "FLOW_CHART_DOCUMENT")]
        #[doc = "Document flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartDocument'"]
        FlowChartDocument,
        #[serde(rename = "FLOW_CHART_EXTRACT")]
        #[doc = "Extract flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartExtract'"]
        FlowChartExtract,
        #[serde(rename = "FLOW_CHART_INPUT_OUTPUT")]
        #[doc = "Input output flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartInputOutput'"]
        FlowChartInputOutput,
        #[serde(rename = "FLOW_CHART_INTERNAL_STORAGE")]
        #[doc = "Internal storage flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartInternalStorage'"]
        FlowChartInternalStorage,
        #[serde(rename = "FLOW_CHART_MAGNETIC_DISK")]
        #[doc = "Magnetic disk flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartMagneticDisk'"]
        FlowChartMagneticDisk,
        #[serde(rename = "FLOW_CHART_MAGNETIC_DRUM")]
        #[doc = "Magnetic drum flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartMagneticDrum'"]
        FlowChartMagneticDrum,
        #[serde(rename = "FLOW_CHART_MAGNETIC_TAPE")]
        #[doc = "Magnetic tape flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartMagneticTape'"]
        FlowChartMagneticTape,
        #[serde(rename = "FLOW_CHART_MANUAL_INPUT")]
        #[doc = "Manual input flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartManualInput'"]
        FlowChartManualInput,
        #[serde(rename = "FLOW_CHART_MANUAL_OPERATION")]
        #[doc = "Manual operation flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartManualOperation'"]
        FlowChartManualOperation,
        #[serde(rename = "FLOW_CHART_MERGE")]
        #[doc = "Merge flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartMerge'"]
        FlowChartMerge,
        #[serde(rename = "FLOW_CHART_MULTIDOCUMENT")]
        #[doc = "Multi-document flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartMultidocument'"]
        FlowChartMultidocument,
        #[serde(rename = "FLOW_CHART_OFFLINE_STORAGE")]
        #[doc = "Offline storage flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartOfflineStorage'"]
        FlowChartOfflineStorage,
        #[serde(rename = "FLOW_CHART_OFFPAGE_CONNECTOR")]
        #[doc = "Off-page connector flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartOffpageConnector'"]
        FlowChartOffpageConnector,
        #[serde(rename = "FLOW_CHART_ONLINE_STORAGE")]
        #[doc = "Online storage flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartOnlineStorage'"]
        FlowChartOnlineStorage,
        #[serde(rename = "FLOW_CHART_OR")]
        #[doc = "Or flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartOr'"]
        FlowChartOr,
        #[serde(rename = "FLOW_CHART_PREDEFINED_PROCESS")]
        #[doc = "Predefined process flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartPredefinedProcess'"]
        FlowChartPredefinedProcess,
        #[serde(rename = "FLOW_CHART_PREPARATION")]
        #[doc = "Preparation flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartPreparation'"]
        FlowChartPreparation,
        #[serde(rename = "FLOW_CHART_PROCESS")]
        #[doc = "Process flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartProcess'"]
        FlowChartProcess,
        #[serde(rename = "FLOW_CHART_PUNCHED_CARD")]
        #[doc = "Punched card flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartPunchedCard'"]
        FlowChartPunchedCard,
        #[serde(rename = "FLOW_CHART_PUNCHED_TAPE")]
        #[doc = "Punched tape flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartPunchedTape'"]
        FlowChartPunchedTape,
        #[serde(rename = "FLOW_CHART_SORT")]
        #[doc = "Sort flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartSort'"]
        FlowChartSort,
        #[serde(rename = "FLOW_CHART_SUMMING_JUNCTION")]
        #[doc = "Summing junction flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartSummingJunction'"]
        FlowChartSummingJunction,
        #[serde(rename = "FLOW_CHART_TERMINATOR")]
        #[doc = "Terminator flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartTerminator'"]
        FlowChartTerminator,
        #[serde(rename = "ARROW_EAST")]
        #[doc = "East arrow shape."]
        ArrowEast,
        #[serde(rename = "ARROW_NORTH_EAST")]
        #[doc = "Northeast arrow shape."]
        ArrowNorthEast,
        #[serde(rename = "ARROW_NORTH")]
        #[doc = "North arrow shape."]
        ArrowNorth,
        #[serde(rename = "SPEECH")]
        #[doc = "Speech shape."]
        Speech,
        #[serde(rename = "STARBURST")]
        #[doc = "Star burst shape."]
        Starburst,
        #[serde(rename = "TEARDROP")]
        #[doc = "Teardrop shape. Corresponds to ECMA-376 ST_ShapeType 'teardrop'"]
        Teardrop,
        #[serde(rename = "ELLIPSE_RIBBON")]
        #[doc = "Ellipse ribbon shape. Corresponds to ECMA-376 ST_ShapeType 'ellipseRibbon'"]
        EllipseRibbon,
        #[serde(rename = "ELLIPSE_RIBBON_2")]
        #[doc = "Ellipse ribbon 2 shape. Corresponds to ECMA-376 ST_ShapeType 'ellipseRibbon2'"]
        EllipseRibbon2,
        #[serde(rename = "CLOUD_CALLOUT")]
        #[doc = "Callout cloud shape. Corresponds to ECMA-376 ST_ShapeType 'cloudCallout'"]
        CloudCallout,
        #[serde(rename = "CUSTOM")]
        #[doc = "Custom shape."]
        Custom,
    }
    impl ::std::default::Default for CreateShapeRequestShapeTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of creating a shape."]
    pub struct CreateShapeResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the created shape."]
        pub object_id: ::std::option::Option<::std::string::String>,
    }
    impl CreateShapeResponse {
        pub fn builder() -> CreateShapeResponseBuilder {
            CreateShapeResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Creates an embedded Google Sheets chart. NOTE: Chart creation requires at least one of the spreadsheets.readonly, spreadsheets, drive.readonly, drive.file, or drive OAuth scopes."]
    pub struct CreateSheetsChartRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "chartId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the specific chart in the Google Sheets spreadsheet."]
        pub chart_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "elementProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The element properties for the chart. When the aspect ratio of the provided size does not match the chart aspect ratio, the chart is scaled and centered with respect to the size in order to maintain aspect ratio. The provided transform is applied after this operation."]
        pub element_properties: ::std::option::Option<::std::boxed::Box<PageElementProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linkingMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The mode with which the chart is linked to the source spreadsheet. When not specified, the chart will be an image that is not linked."]
        pub linking_mode: ::std::option::Option<CreateSheetsChartRequestLinkingModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A user-supplied object ID. If specified, the ID must be unique among all pages and page elements in the presentation. The ID should start with a word character [a-zA-Z0-9_] and then followed by any number of the following characters [a-zA-Z0-9_-:]. The length of the ID should not be less than 5 or greater than 50. If empty, a unique identifier will be generated."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spreadsheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the Google Sheets spreadsheet that contains the chart."]
        pub spreadsheet_id: ::std::option::Option<::std::string::String>,
    }
    impl CreateSheetsChartRequest {
        pub fn builder() -> CreateSheetsChartRequestBuilder {
            CreateSheetsChartRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The mode with which the chart is linked to the source spreadsheet. When not specified, the chart will be an image that is not linked."]
    pub enum CreateSheetsChartRequestLinkingModeEnum {
        #[serde(rename = "NOT_LINKED_IMAGE")]
        #[doc = "The chart is not associated with the source spreadsheet and cannot be updated. A chart that is not linked will be inserted as an image."]
        NotLinkedImage,
        #[serde(rename = "LINKED")]
        #[doc = "Linking the chart allows it to be updated, and other collaborators will see a link to the spreadsheet."]
        Linked,
    }
    impl ::std::default::Default for CreateSheetsChartRequestLinkingModeEnum {
        fn default() -> Self {
            Self::NotLinkedImage
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of creating an embedded Google Sheets chart."]
    pub struct CreateSheetsChartResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the created chart."]
        pub object_id: ::std::option::Option<::std::string::String>,
    }
    impl CreateSheetsChartResponse {
        pub fn builder() -> CreateSheetsChartResponseBuilder {
            CreateSheetsChartResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Creates a new slide."]
    pub struct CreateSlideRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertionIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The optional zero-based index indicating where to insert the slides. If you don't specify an index, the new slide is created at the end."]
        pub insertion_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A user-supplied object ID. If you specify an ID, it must be unique among all pages and page elements in the presentation. The ID must start with an alphanumeric character or an underscore (matches regex `[a-zA-Z0-9_]`); remaining characters may include those as well as a hyphen or colon (matches regex `[a-zA-Z0-9_-:]`). The length of the ID must not be less than 5 or greater than 50. If you don't specify an ID, a unique one is generated."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "placeholderIdMappings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional list of object ID mappings from the placeholder(s) on the layout to the placeholder(s) that will be created on the new slide from that specified layout. Can only be used when `slide_layout_reference` is specified."]
        pub placeholder_id_mappings:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LayoutPlaceholderIdMapping>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "slideLayoutReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Layout reference of the slide to be inserted, based on the *current master*, which is one of the following: - The master of the previous slide index. - The master of the first slide, if the insertion_index is zero. - The first master in the presentation, if there are no slides. If the LayoutReference is not found in the current master, a 400 bad request error is returned. If you don't specify a layout reference, then the new slide will use the predefined layout `BLANK`."]
        pub slide_layout_reference: ::std::option::Option<::std::boxed::Box<LayoutReference>>,
    }
    impl CreateSlideRequest {
        pub fn builder() -> CreateSlideRequestBuilder {
            CreateSlideRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of creating a slide."]
    pub struct CreateSlideResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the created slide."]
        pub object_id: ::std::option::Option<::std::string::String>,
    }
    impl CreateSlideResponse {
        pub fn builder() -> CreateSlideResponseBuilder {
            CreateSlideResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Creates a new table."]
    pub struct CreateTableRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of columns in the table."]
        pub columns: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "elementProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The element properties for the table. The table will be created at the provided size, subject to a minimum size. If no size is provided, the table will be automatically sized. Table transforms must have a scale of 1 and no shear components. If no transform is provided, the table will be centered on the page."]
        pub element_properties: ::std::option::Option<::std::boxed::Box<PageElementProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A user-supplied object ID. If you specify an ID, it must be unique among all pages and page elements in the presentation. The ID must start with an alphanumeric character or an underscore (matches regex `[a-zA-Z0-9_]`); remaining characters may include those as well as a hyphen or colon (matches regex `[a-zA-Z0-9_-:]`). The length of the ID must not be less than 5 or greater than 50. If you don't specify an ID, a unique one is generated."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of rows in the table."]
        pub rows: ::std::option::Option<::std::primitive::i64>,
    }
    impl CreateTableRequest {
        pub fn builder() -> CreateTableRequestBuilder {
            CreateTableRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of creating a table."]
    pub struct CreateTableResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the created table."]
        pub object_id: ::std::option::Option<::std::string::String>,
    }
    impl CreateTableResponse {
        pub fn builder() -> CreateTableResponseBuilder {
            CreateTableResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Creates a video. NOTE: Creating a video from Google Drive requires that the requesting app have at least one of the drive, drive.readonly, or drive.file OAuth scopes."]
    pub struct CreateVideoRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "elementProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The element properties for the video. The PageElementProperties.size property is optional. If you don't specify a size, a default size is chosen by the server. The PageElementProperties.transform property is optional. The transform must not have shear components. If you don't specify a transform, the video will be placed at the top left corner of the page."]
        pub element_properties: ::std::option::Option<::std::boxed::Box<PageElementProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video source's unique identifier for this video. e.g. For YouTube video https://www.youtube.com/watch?v=7U3axjORYZ0, the ID is 7U3axjORYZ0. For a Google Drive video https://drive.google.com/file/d/1xCgQLFTJi5_Xl8DgW_lcUYq5e-q6Hi5Q the ID is 1xCgQLFTJi5_Xl8DgW_lcUYq5e-q6Hi5Q."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A user-supplied object ID. If you specify an ID, it must be unique among all pages and page elements in the presentation. The ID must start with an alphanumeric character or an underscore (matches regex `[a-zA-Z0-9_]`); remaining characters may include those as well as a hyphen or colon (matches regex `[a-zA-Z0-9_-:]`). The length of the ID must not be less than 5 or greater than 50. If you don't specify an ID, a unique one is generated."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video source."]
        pub source: ::std::option::Option<CreateVideoRequestSourceEnum>,
    }
    impl CreateVideoRequest {
        pub fn builder() -> CreateVideoRequestBuilder {
            CreateVideoRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video source."]
    pub enum CreateVideoRequestSourceEnum {
        #[serde(rename = "SOURCE_UNSPECIFIED")]
        #[doc = "The video source is unspecified."]
        SourceUnspecified,
        #[serde(rename = "YOUTUBE")]
        #[doc = "The video source is YouTube."]
        Youtube,
        #[serde(rename = "DRIVE")]
        #[doc = "The video source is Google Drive."]
        Drive,
    }
    impl ::std::default::Default for CreateVideoRequestSourceEnum {
        fn default() -> Self {
            Self::SourceUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of creating a video."]
    pub struct CreateVideoResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the created video."]
        pub object_id: ::std::option::Option<::std::string::String>,
    }
    impl CreateVideoResponse {
        pub fn builder() -> CreateVideoResponseBuilder {
            CreateVideoResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The crop properties of an object enclosed in a container. For example, an Image. The crop properties is represented by the offsets of four edges which define a crop rectangle. The offsets are measured in percentage from the corresponding edges of the object's original bounding rectangle towards inside, relative to the object's original dimensions. - If the offset is in the interval (0, 1), the corresponding edge of crop rectangle is positioned inside of the object's original bounding rectangle. - If the offset is negative or greater than 1, the corresponding edge of crop rectangle is positioned outside of the object's original bounding rectangle. - If the left edge of the crop rectangle is on the right side of its right edge, the object will be flipped horizontally. - If the top edge of the crop rectangle is below its bottom edge, the object will be flipped vertically. - If all offsets and rotation angle is 0, the object is not cropped. After cropping, the content in the crop rectangle will be stretched to fit its container."]
    pub struct CropProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "angle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The rotation angle of the crop window around its center, in radians. Rotation angle is applied after the offset."]
        pub angle: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bottomOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The offset specifies the bottom edge of the crop rectangle that is located above the original bounding rectangle bottom edge, relative to the object's original height."]
        pub bottom_offset: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "leftOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The offset specifies the left edge of the crop rectangle that is located to the right of the original bounding rectangle left edge, relative to the object's original width."]
        pub left_offset: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rightOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The offset specifies the right edge of the crop rectangle that is located to the left of the original bounding rectangle right edge, relative to the object's original width."]
        pub right_offset: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The offset specifies the top edge of the crop rectangle that is located below the original bounding rectangle top edge, relative to the object's original height."]
        pub top_offset: ::std::option::Option<::std::primitive::f64>,
    }
    impl CropProperties {
        pub fn builder() -> CropPropertiesBuilder {
            CropPropertiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deletes an object, either pages or page elements, from the presentation."]
    pub struct DeleteObjectRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the page or page element to delete. If after a delete operation a group contains only 1 or no page elements, the group is also deleted. If a placeholder is deleted on a layout, any empty inheriting shapes are also deleted."]
        pub object_id: ::std::option::Option<::std::string::String>,
    }
    impl DeleteObjectRequest {
        pub fn builder() -> DeleteObjectRequestBuilder {
            DeleteObjectRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deletes bullets from all of the paragraphs that overlap with the given text index range. The nesting level of each paragraph will be visually preserved by adding indent to the start of the corresponding paragraph."]
    pub struct DeleteParagraphBulletsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cellLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The optional table cell location if the text to be modified is in a table cell. If present, the object_id must refer to a table."]
        pub cell_location: ::std::option::Option<::std::boxed::Box<TableCellLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the shape or table containing the text to delete bullets from."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range of text to delete bullets from, based on TextElement indexes."]
        pub text_range: ::std::option::Option<::std::boxed::Box<Range>>,
    }
    impl DeleteParagraphBulletsRequest {
        pub fn builder() -> DeleteParagraphBulletsRequestBuilder {
            DeleteParagraphBulletsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deletes a column from a table."]
    pub struct DeleteTableColumnRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cellLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reference table cell location from which a column will be deleted. The column this cell spans will be deleted. If this is a merged cell, multiple columns will be deleted. If no columns remain in the table after this deletion, the whole table is deleted."]
        pub cell_location: ::std::option::Option<::std::boxed::Box<TableCellLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableObjectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The table to delete columns from."]
        pub table_object_id: ::std::option::Option<::std::string::String>,
    }
    impl DeleteTableColumnRequest {
        pub fn builder() -> DeleteTableColumnRequestBuilder {
            DeleteTableColumnRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deletes a row from a table."]
    pub struct DeleteTableRowRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cellLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reference table cell location from which a row will be deleted. The row this cell spans will be deleted. If this is a merged cell, multiple rows will be deleted. If no rows remain in the table after this deletion, the whole table is deleted."]
        pub cell_location: ::std::option::Option<::std::boxed::Box<TableCellLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableObjectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The table to delete rows from."]
        pub table_object_id: ::std::option::Option<::std::string::String>,
    }
    impl DeleteTableRowRequest {
        pub fn builder() -> DeleteTableRowRequestBuilder {
            DeleteTableRowRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Deletes text from a shape or a table cell."]
    pub struct DeleteTextRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cellLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The optional table cell location if the text is to be deleted from a table cell. If present, the object_id must refer to a table."]
        pub cell_location: ::std::option::Option<::std::boxed::Box<TableCellLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the shape or table from which the text will be deleted."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range of text to delete, based on TextElement indexes. There is always an implicit newline character at the end of a shape's or table cell's text that cannot be deleted. `Range.Type.ALL` will use the correct bounds, but care must be taken when specifying explicit bounds for range types `FROM_START_INDEX` and `FIXED_RANGE`. For example, if the text is \"ABC\", followed by an implicit newline, then the maximum value is 2 for `text_range.start_index` and 3 for `text_range.end_index`. Deleting text that crosses a paragraph boundary may result in changes to paragraph styles and lists as the two paragraphs are merged. Ranges that include only one code unit of a surrogate pair are expanded to include both code units."]
        pub text_range: ::std::option::Option<::std::boxed::Box<Range>>,
    }
    impl DeleteTextRequest {
        pub fn builder() -> DeleteTextRequestBuilder {
            DeleteTextRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A magnitude in a single direction in the specified units."]
    pub struct Dimension {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "magnitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The magnitude."]
        pub magnitude: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The units for magnitude."]
        pub unit: ::std::option::Option<DimensionUnitEnum>,
    }
    impl Dimension {
        pub fn builder() -> DimensionBuilder {
            DimensionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The units for magnitude."]
    pub enum DimensionUnitEnum {
        #[serde(rename = "UNIT_UNSPECIFIED")]
        #[doc = "The units are unknown."]
        UnitUnspecified,
        #[serde(rename = "EMU")]
        #[doc = "An English Metric Unit (EMU) is defined as 1/360,000 of a centimeter and thus there are 914,400 EMUs per inch, and 12,700 EMUs per point."]
        Emu,
        #[serde(rename = "PT")]
        #[doc = "A point, 1/72 of an inch."]
        Pt,
    }
    impl ::std::default::Default for DimensionUnitEnum {
        fn default() -> Self {
            Self::UnitUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Duplicates a slide or page element. When duplicating a slide, the duplicate slide will be created immediately following the specified slide. When duplicating a page element, the duplicate will be placed on the same page at the same position as the original."]
    pub struct DuplicateObjectRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the object to duplicate."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object being duplicated may contain other objects, for example when duplicating a slide or a group page element. This map defines how the IDs of duplicated objects are generated: the keys are the IDs of the original objects and its values are the IDs that will be assigned to the corresponding duplicate object. The ID of the source object's duplicate may be specified in this map as well, using the same value of the `object_id` field as a key and the newly desired ID as the value. All keys must correspond to existing IDs in the presentation. All values must be unique in the presentation and must start with an alphanumeric character or an underscore (matches regex `[a-zA-Z0-9_]`); remaining characters may include those as well as a hyphen or colon (matches regex `[a-zA-Z0-9_-:]`). The length of the new ID must not be less than 5 or greater than 50. If any IDs of source objects are omitted from the map, a new random ID will be assigned. If the map is empty or unset, all duplicate objects will receive a new random ID."]
        pub object_ids:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl DuplicateObjectRequest {
        pub fn builder() -> DuplicateObjectRequestBuilder {
            DuplicateObjectRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response of duplicating an object."]
    pub struct DuplicateObjectResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the new duplicate object."]
        pub object_id: ::std::option::Option<::std::string::String>,
    }
    impl DuplicateObjectResponse {
        pub fn builder() -> DuplicateObjectResponseBuilder {
            DuplicateObjectResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A PageElement kind representing a joined collection of PageElements."]
    pub struct Group {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "children")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The collection of elements in the group. The minimum size of a group is 2."]
        pub children: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PageElement>>>,
    }
    impl Group {
        pub fn builder() -> GroupBuilder {
            GroupBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Groups objects to create an object group. For example, groups PageElements to create a Group on the same page as all the children."]
    pub struct GroupObjectsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "childrenObjectIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object IDs of the objects to group. Only page elements can be grouped. There should be at least two page elements on the same page that are not already in another group. Some page elements, such as videos, tables and placeholder shapes cannot be grouped."]
        pub children_object_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groupObjectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A user-supplied object ID for the group to be created. If you specify an ID, it must be unique among all pages and page elements in the presentation. The ID must start with an alphanumeric character or an underscore (matches regex `[a-zA-Z0-9_]`); remaining characters may include those as well as a hyphen or colon (matches regex `[a-zA-Z0-9_-:]`). The length of the ID must not be less than 5 or greater than 50. If you don't specify an ID, a unique one is generated."]
        pub group_object_id: ::std::option::Option<::std::string::String>,
    }
    impl GroupObjectsRequest {
        pub fn builder() -> GroupObjectsRequestBuilder {
            GroupObjectsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of grouping objects."]
    pub struct GroupObjectsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the created group."]
        pub object_id: ::std::option::Option<::std::string::String>,
    }
    impl GroupObjectsResponse {
        pub fn builder() -> GroupObjectsResponseBuilder {
            GroupObjectsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A PageElement kind representing an image."]
    pub struct Image {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An URL to an image with a default lifetime of 30 minutes. This URL is tagged with the account of the requester. Anyone with the URL effectively accesses the image as the original requester. Access to the image may be lost if the presentation's sharing settings change."]
        pub content_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The properties of the image."]
        pub image_properties: ::std::option::Option<::std::boxed::Box<ImageProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source URL is the URL used to insert the image. The source URL can be empty."]
        pub source_url: ::std::option::Option<::std::string::String>,
    }
    impl Image {
        pub fn builder() -> ImageBuilder {
            ImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The properties of the Image."]
    pub struct ImageProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "brightness")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The brightness effect of the image. The value should be in the interval [-1.0, 1.0], where 0 means no effect. This property is read-only."]
        pub brightness: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contrast")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The contrast effect of the image. The value should be in the interval [-1.0, 1.0], where 0 means no effect. This property is read-only."]
        pub contrast: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cropProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The crop properties of the image. If not set, the image is not cropped. This property is read-only."]
        pub crop_properties: ::std::option::Option<::std::boxed::Box<CropProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "link")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The hyperlink destination of the image. If unset, there is no link."]
        pub link: ::std::option::Option<::std::boxed::Box<Link>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outline")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The outline of the image. If not set, the image has no outline."]
        pub outline: ::std::option::Option<::std::boxed::Box<Outline>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recolor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The recolor effect of the image. If not set, the image is not recolored. This property is read-only."]
        pub recolor: ::std::option::Option<::std::boxed::Box<Recolor>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shadow")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The shadow of the image. If not set, the image has no shadow. This property is read-only."]
        pub shadow: ::std::option::Option<::std::boxed::Box<Shadow>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transparency")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The transparency effect of the image. The value should be in the interval [0.0, 1.0], where 0 means no effect and 1 means completely transparent. This property is read-only."]
        pub transparency: ::std::option::Option<::std::primitive::f64>,
    }
    impl ImageProperties {
        pub fn builder() -> ImagePropertiesBuilder {
            ImagePropertiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Inserts columns into a table. Other columns in the table will be resized to fit the new column."]
    pub struct InsertTableColumnsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cellLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reference table cell location from which columns will be inserted. A new column will be inserted to the left (or right) of the column where the reference cell is. If the reference cell is a merged cell, a new column will be inserted to the left (or right) of the merged cell."]
        pub cell_location: ::std::option::Option<::std::boxed::Box<TableCellLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertRight")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to insert new columns to the right of the reference cell location. - `True`: insert to the right. - `False`: insert to the left."]
        pub insert_right: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "number")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of columns to be inserted. Maximum 20 per request."]
        pub number: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableObjectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The table to insert columns into."]
        pub table_object_id: ::std::option::Option<::std::string::String>,
    }
    impl InsertTableColumnsRequest {
        pub fn builder() -> InsertTableColumnsRequestBuilder {
            InsertTableColumnsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Inserts rows into a table."]
    pub struct InsertTableRowsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cellLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reference table cell location from which rows will be inserted. A new row will be inserted above (or below) the row where the reference cell is. If the reference cell is a merged cell, a new row will be inserted above (or below) the merged cell."]
        pub cell_location: ::std::option::Option<::std::boxed::Box<TableCellLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertBelow")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to insert new rows below the reference cell location. - `True`: insert below the cell. - `False`: insert above the cell."]
        pub insert_below: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "number")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of rows to be inserted. Maximum 20 per request."]
        pub number: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableObjectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The table to insert rows into."]
        pub table_object_id: ::std::option::Option<::std::string::String>,
    }
    impl InsertTableRowsRequest {
        pub fn builder() -> InsertTableRowsRequestBuilder {
            InsertTableRowsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Inserts text into a shape or a table cell."]
    pub struct InsertTextRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cellLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The optional table cell location if the text is to be inserted into a table cell. If present, the object_id must refer to a table."]
        pub cell_location: ::std::option::Option<::std::boxed::Box<TableCellLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertionIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index where the text will be inserted, in Unicode code units, based on TextElement indexes. The index is zero-based and is computed from the start of the string. The index may be adjusted to prevent insertions inside Unicode grapheme clusters. In these cases, the text will be inserted immediately after the grapheme cluster."]
        pub insertion_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the shape or table where the text will be inserted."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text to be inserted. Inserting a newline character will implicitly create a new ParagraphMarker at that index. The paragraph style of the new paragraph will be copied from the paragraph at the current insertion index, including lists and bullets. Text styles for inserted text will be determined automatically, generally preserving the styling of neighboring text. In most cases, the text will be added to the TextRun that exists at the insertion index. Some control characters (U+0000-U+0008, U+000C-U+001F) and characters from the Unicode Basic Multilingual Plane Private Use Area (U+E000-U+F8FF) will be stripped out of the inserted text."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl InsertTextRequest {
        pub fn builder() -> InsertTextRequestBuilder {
            InsertTextRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The user-specified ID mapping for a placeholder that will be created on a slide from a specified layout."]
    pub struct LayoutPlaceholderIdMapping {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layoutPlaceholder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The placeholder on a layout that will be applied to a slide. Only type and index are needed. For example, a predefined `TITLE_AND_BODY` layout may usually have a TITLE placeholder with index 0 and a BODY placeholder with index 0."]
        pub layout_placeholder: ::std::option::Option<::std::boxed::Box<Placeholder>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layoutPlaceholderObjectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the placeholder on a layout that will be applied to a slide."]
        pub layout_placeholder_object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A user-supplied object ID for the placeholder identified above that to be created onto a slide. If you specify an ID, it must be unique among all pages and page elements in the presentation. The ID must start with an alphanumeric character or an underscore (matches regex `[a-zA-Z0-9_]`); remaining characters may include those as well as a hyphen or colon (matches regex `[a-zA-Z0-9_-:]`). The length of the ID must not be less than 5 or greater than 50. If you don't specify an ID, a unique one is generated."]
        pub object_id: ::std::option::Option<::std::string::String>,
    }
    impl LayoutPlaceholderIdMapping {
        pub fn builder() -> LayoutPlaceholderIdMappingBuilder {
            LayoutPlaceholderIdMappingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The properties of Page are only relevant for pages with page_type LAYOUT."]
    pub struct LayoutProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The human-readable name of the layout."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "masterObjectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the master that this layout is based on."]
        pub master_object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the layout."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl LayoutProperties {
        pub fn builder() -> LayoutPropertiesBuilder {
            LayoutPropertiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Slide layout reference. This may reference either: - A predefined layout - One of the layouts in the presentation."]
    pub struct LayoutReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layoutId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Layout ID: the object ID of one of the layouts in the presentation."]
        pub layout_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "predefinedLayout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Predefined layout."]
        pub predefined_layout: ::std::option::Option<LayoutReferencePredefinedLayoutEnum>,
    }
    impl LayoutReference {
        pub fn builder() -> LayoutReferenceBuilder {
            LayoutReferenceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Predefined layout."]
    pub enum LayoutReferencePredefinedLayoutEnum {
        #[serde(rename = "PREDEFINED_LAYOUT_UNSPECIFIED")]
        #[doc = "Unspecified layout."]
        PredefinedLayoutUnspecified,
        #[serde(rename = "BLANK")]
        #[doc = "Blank layout, with no placeholders."]
        Blank,
        #[serde(rename = "CAPTION_ONLY")]
        #[doc = "Layout with a caption at the bottom."]
        CaptionOnly,
        #[serde(rename = "TITLE")]
        #[doc = "Layout with a title and a subtitle."]
        Title,
        #[serde(rename = "TITLE_AND_BODY")]
        #[doc = "Layout with a title and body."]
        TitleAndBody,
        #[serde(rename = "TITLE_AND_TWO_COLUMNS")]
        #[doc = "Layout with a title and two columns."]
        TitleAndTwoColumns,
        #[serde(rename = "TITLE_ONLY")]
        #[doc = "Layout with only a title."]
        TitleOnly,
        #[serde(rename = "SECTION_HEADER")]
        #[doc = "Layout with a section title."]
        SectionHeader,
        #[serde(rename = "SECTION_TITLE_AND_DESCRIPTION")]
        #[doc = "Layout with a title and subtitle on one side and description on the other."]
        SectionTitleAndDescription,
        #[serde(rename = "ONE_COLUMN_TEXT")]
        #[doc = "Layout with one title and one body, arranged in a single column."]
        OneColumnText,
        #[serde(rename = "MAIN_POINT")]
        #[doc = "Layout with a main point."]
        MainPoint,
        #[serde(rename = "BIG_NUMBER")]
        #[doc = "Layout with a big number heading."]
        BigNumber,
    }
    impl ::std::default::Default for LayoutReferencePredefinedLayoutEnum {
        fn default() -> Self {
            Self::PredefinedLayoutUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A PageElement kind representing a non-connector line, straight connector, curved connector, or bent connector."]
    pub struct Line {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineCategory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The category of the line. It matches the `category` specified in CreateLineRequest, and can be updated with UpdateLineCategoryRequest."]
        pub line_category: ::std::option::Option<LineLineCategoryEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The properties of the line."]
        pub line_properties: ::std::option::Option<::std::boxed::Box<LineProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the line."]
        pub line_type: ::std::option::Option<LineLineTypeEnum>,
    }
    impl Line {
        pub fn builder() -> LineBuilder {
            LineBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The category of the line. It matches the `category` specified in CreateLineRequest, and can be updated with UpdateLineCategoryRequest."]
    pub enum LineLineCategoryEnum {
        #[serde(rename = "LINE_CATEGORY_UNSPECIFIED")]
        #[doc = "Unspecified line category."]
        LineCategoryUnspecified,
        #[serde(rename = "STRAIGHT")]
        #[doc = "Straight connectors, including straight connector 1."]
        Straight,
        #[serde(rename = "BENT")]
        #[doc = "Bent connectors, including bent connector 2 to 5."]
        Bent,
        #[serde(rename = "CURVED")]
        #[doc = "Curved connectors, including curved connector 2 to 5."]
        Curved,
    }
    impl ::std::default::Default for LineLineCategoryEnum {
        fn default() -> Self {
            Self::LineCategoryUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the line."]
    pub enum LineLineTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "An unspecified line type."]
        TypeUnspecified,
        #[serde(rename = "STRAIGHT_CONNECTOR_1")]
        #[doc = "Straight connector 1 form. Corresponds to ECMA-376 ST_ShapeType 'straightConnector1'."]
        StraightConnector1,
        #[serde(rename = "BENT_CONNECTOR_2")]
        #[doc = "Bent connector 2 form. Corresponds to ECMA-376 ST_ShapeType 'bentConnector2'."]
        BentConnector2,
        #[serde(rename = "BENT_CONNECTOR_3")]
        #[doc = "Bent connector 3 form. Corresponds to ECMA-376 ST_ShapeType 'bentConnector3'."]
        BentConnector3,
        #[serde(rename = "BENT_CONNECTOR_4")]
        #[doc = "Bent connector 4 form. Corresponds to ECMA-376 ST_ShapeType 'bentConnector4'."]
        BentConnector4,
        #[serde(rename = "BENT_CONNECTOR_5")]
        #[doc = "Bent connector 5 form. Corresponds to ECMA-376 ST_ShapeType 'bentConnector5'."]
        BentConnector5,
        #[serde(rename = "CURVED_CONNECTOR_2")]
        #[doc = "Curved connector 2 form. Corresponds to ECMA-376 ST_ShapeType 'curvedConnector2'."]
        CurvedConnector2,
        #[serde(rename = "CURVED_CONNECTOR_3")]
        #[doc = "Curved connector 3 form. Corresponds to ECMA-376 ST_ShapeType 'curvedConnector3'."]
        CurvedConnector3,
        #[serde(rename = "CURVED_CONNECTOR_4")]
        #[doc = "Curved connector 4 form. Corresponds to ECMA-376 ST_ShapeType 'curvedConnector4'."]
        CurvedConnector4,
        #[serde(rename = "CURVED_CONNECTOR_5")]
        #[doc = "Curved connector 5 form. Corresponds to ECMA-376 ST_ShapeType 'curvedConnector5'."]
        CurvedConnector5,
        #[serde(rename = "STRAIGHT_LINE")]
        #[doc = "Straight line. Corresponds to ECMA-376 ST_ShapeType 'line'. This line type is not a connector."]
        StraightLine,
    }
    impl ::std::default::Default for LineLineTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The properties for one end of a Line connection."]
    pub struct LineConnection {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "connectedObjectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the connected page element. Some page elements, such as groups, tables, and lines do not have connection sites and therefore cannot be connected to a connector line."]
        pub connected_object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "connectionSiteIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index of the connection site on the connected page element. In most cases, it corresponds to the predefined connection site index from the ECMA-376 standard. More information on those connection sites can be found in the description of the \"cnx\" attribute in section 20.1.9.9 and Annex H. \"Predefined DrawingML Shape and Text Geometries\" of \"Office Open XML File Formats-Fundamentals and Markup Language Reference\", part 1 of [ECMA-376 5th edition] (http://www.ecma-international.org/publications/standards/Ecma-376.htm). The position of each connection site can also be viewed from Slides editor."]
        pub connection_site_index: ::std::option::Option<::std::primitive::i64>,
    }
    impl LineConnection {
        pub fn builder() -> LineConnectionBuilder {
            LineConnectionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The fill of the line."]
    pub struct LineFill {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "solidFill")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Solid color fill."]
        pub solid_fill: ::std::option::Option<::std::boxed::Box<SolidFill>>,
    }
    impl LineFill {
        pub fn builder() -> LineFillBuilder {
            LineFillBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The properties of the Line. When unset, these fields default to values that match the appearance of new lines created in the Slides editor."]
    pub struct LineProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dashStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dash style of the line."]
        pub dash_style: ::std::option::Option<LinePropertiesDashStyleEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endArrow")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The style of the arrow at the end of the line."]
        pub end_arrow: ::std::option::Option<LinePropertiesEndArrowEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endConnection")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The connection at the end of the line. If unset, there is no connection. Only lines with a Type indicating it is a \"connector\" can have an `end_connection`."]
        pub end_connection: ::std::option::Option<::std::boxed::Box<LineConnection>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineFill")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fill of the line. The default line fill matches the defaults for new lines created in the Slides editor."]
        pub line_fill: ::std::option::Option<::std::boxed::Box<LineFill>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "link")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The hyperlink destination of the line. If unset, there is no link."]
        pub link: ::std::option::Option<::std::boxed::Box<Link>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startArrow")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The style of the arrow at the beginning of the line."]
        pub start_arrow: ::std::option::Option<LinePropertiesStartArrowEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startConnection")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The connection at the beginning of the line. If unset, there is no connection. Only lines with a Type indicating it is a \"connector\" can have a `start_connection`."]
        pub start_connection: ::std::option::Option<::std::boxed::Box<LineConnection>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "weight")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The thickness of the line."]
        pub weight: ::std::option::Option<::std::boxed::Box<Dimension>>,
    }
    impl LineProperties {
        pub fn builder() -> LinePropertiesBuilder {
            LinePropertiesBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The dash style of the line."]
    pub enum LinePropertiesDashStyleEnum {
        #[serde(rename = "DASH_STYLE_UNSPECIFIED")]
        #[doc = "Unspecified dash style."]
        DashStyleUnspecified,
        #[serde(rename = "SOLID")]
        #[doc = "Solid line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'solid'. This is the default dash style."]
        Solid,
        #[serde(rename = "DOT")]
        #[doc = "Dotted line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dot'."]
        Dot,
        #[serde(rename = "DASH")]
        #[doc = "Dashed line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dash'."]
        Dash,
        #[serde(rename = "DASH_DOT")]
        #[doc = "Alternating dashes and dots. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dashDot'."]
        DashDot,
        #[serde(rename = "LONG_DASH")]
        #[doc = "Line with large dashes. Corresponds to ECMA-376 ST_PresetLineDashVal value 'lgDash'."]
        LongDash,
        #[serde(rename = "LONG_DASH_DOT")]
        #[doc = "Alternating large dashes and dots. Corresponds to ECMA-376 ST_PresetLineDashVal value 'lgDashDot'."]
        LongDashDot,
    }
    impl ::std::default::Default for LinePropertiesDashStyleEnum {
        fn default() -> Self {
            Self::DashStyleUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The style of the arrow at the end of the line."]
    pub enum LinePropertiesEndArrowEnum {
        #[serde(rename = "ARROW_STYLE_UNSPECIFIED")]
        #[doc = "An unspecified arrow style."]
        ArrowStyleUnspecified,
        #[serde(rename = "NONE")]
        #[doc = "No arrow."]
        None,
        #[serde(rename = "STEALTH_ARROW")]
        #[doc = "Arrow with notched back. Corresponds to ECMA-376 ST_LineEndType value 'stealth'."]
        StealthArrow,
        #[serde(rename = "FILL_ARROW")]
        #[doc = "Filled arrow. Corresponds to ECMA-376 ST_LineEndType value 'triangle'."]
        FillArrow,
        #[serde(rename = "FILL_CIRCLE")]
        #[doc = "Filled circle. Corresponds to ECMA-376 ST_LineEndType value 'oval'."]
        FillCircle,
        #[serde(rename = "FILL_SQUARE")]
        #[doc = "Filled square."]
        FillSquare,
        #[serde(rename = "FILL_DIAMOND")]
        #[doc = "Filled diamond. Corresponds to ECMA-376 ST_LineEndType value 'diamond'."]
        FillDiamond,
        #[serde(rename = "OPEN_ARROW")]
        #[doc = "Hollow arrow."]
        OpenArrow,
        #[serde(rename = "OPEN_CIRCLE")]
        #[doc = "Hollow circle."]
        OpenCircle,
        #[serde(rename = "OPEN_SQUARE")]
        #[doc = "Hollow square."]
        OpenSquare,
        #[serde(rename = "OPEN_DIAMOND")]
        #[doc = "Hollow diamond."]
        OpenDiamond,
    }
    impl ::std::default::Default for LinePropertiesEndArrowEnum {
        fn default() -> Self {
            Self::ArrowStyleUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The style of the arrow at the beginning of the line."]
    pub enum LinePropertiesStartArrowEnum {
        #[serde(rename = "ARROW_STYLE_UNSPECIFIED")]
        #[doc = "An unspecified arrow style."]
        ArrowStyleUnspecified,
        #[serde(rename = "NONE")]
        #[doc = "No arrow."]
        None,
        #[serde(rename = "STEALTH_ARROW")]
        #[doc = "Arrow with notched back. Corresponds to ECMA-376 ST_LineEndType value 'stealth'."]
        StealthArrow,
        #[serde(rename = "FILL_ARROW")]
        #[doc = "Filled arrow. Corresponds to ECMA-376 ST_LineEndType value 'triangle'."]
        FillArrow,
        #[serde(rename = "FILL_CIRCLE")]
        #[doc = "Filled circle. Corresponds to ECMA-376 ST_LineEndType value 'oval'."]
        FillCircle,
        #[serde(rename = "FILL_SQUARE")]
        #[doc = "Filled square."]
        FillSquare,
        #[serde(rename = "FILL_DIAMOND")]
        #[doc = "Filled diamond. Corresponds to ECMA-376 ST_LineEndType value 'diamond'."]
        FillDiamond,
        #[serde(rename = "OPEN_ARROW")]
        #[doc = "Hollow arrow."]
        OpenArrow,
        #[serde(rename = "OPEN_CIRCLE")]
        #[doc = "Hollow circle."]
        OpenCircle,
        #[serde(rename = "OPEN_SQUARE")]
        #[doc = "Hollow square."]
        OpenSquare,
        #[serde(rename = "OPEN_DIAMOND")]
        #[doc = "Hollow diamond."]
        OpenDiamond,
    }
    impl ::std::default::Default for LinePropertiesStartArrowEnum {
        fn default() -> Self {
            Self::ArrowStyleUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A hypertext link."]
    pub struct Link {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageObjectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, indicates this is a link to the specific page in this presentation with this ID. A page with this ID may not exist."]
        pub page_object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relativeLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, indicates this is a link to a slide in this presentation, addressed by its position."]
        pub relative_link: ::std::option::Option<LinkRelativeLinkEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "slideIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, indicates this is a link to the slide at this zero-based index in the presentation. There may not be a slide at this index."]
        pub slide_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, indicates this is a link to the external web page at this URL."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl Link {
        pub fn builder() -> LinkBuilder {
            LinkBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "If set, indicates this is a link to a slide in this presentation, addressed by its position."]
    pub enum LinkRelativeLinkEnum {
        #[serde(rename = "RELATIVE_SLIDE_LINK_UNSPECIFIED")]
        #[doc = "An unspecified relative slide link."]
        RelativeSlideLinkUnspecified,
        #[serde(rename = "NEXT_SLIDE")]
        #[doc = "A link to the next slide."]
        NextSlide,
        #[serde(rename = "PREVIOUS_SLIDE")]
        #[doc = "A link to the previous slide."]
        PreviousSlide,
        #[serde(rename = "FIRST_SLIDE")]
        #[doc = "A link to the first slide in the presentation."]
        FirstSlide,
        #[serde(rename = "LAST_SLIDE")]
        #[doc = "A link to the last slide in the presentation."]
        LastSlide,
    }
    impl ::std::default::Default for LinkRelativeLinkEnum {
        fn default() -> Self {
            Self::RelativeSlideLinkUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A List describes the look and feel of bullets belonging to paragraphs associated with a list. A paragraph that is part of a list has an implicit reference to that list's ID."]
    pub struct List {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "listId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the list."]
        pub list_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nestingLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A map of nesting levels to the properties of bullets at the associated level. A list has at most nine levels of nesting, so the possible values for the keys of this map are 0 through 8, inclusive."]
        pub nesting_level: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<NestingLevel>>,
        >,
    }
    impl List {
        pub fn builder() -> ListBuilder {
            ListBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The properties of Page that are only relevant for pages with page_type MASTER."]
    pub struct MasterProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The human-readable name of the master."]
        pub display_name: ::std::option::Option<::std::string::String>,
    }
    impl MasterProperties {
        pub fn builder() -> MasterPropertiesBuilder {
            MasterPropertiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Merges cells in a Table."]
    pub struct MergeTableCellsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the table."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The table range specifying which cells of the table to merge. Any text in the cells being merged will be concatenated and stored in the upper-left (\"head\") cell of the range. If the range is non-rectangular (which can occur in some cases where the range covers cells that are already merged), a 400 bad request error is returned."]
        pub table_range: ::std::option::Option<::std::boxed::Box<TableRange>>,
    }
    impl MergeTableCellsRequest {
        pub fn builder() -> MergeTableCellsRequestBuilder {
            MergeTableCellsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains properties describing the look and feel of a list bullet at a given level of nesting."]
    pub struct NestingLevel {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bulletStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The style of a bullet at this level of nesting."]
        pub bullet_style: ::std::option::Option<::std::boxed::Box<TextStyle>>,
    }
    impl NestingLevel {
        pub fn builder() -> NestingLevelBuilder {
            NestingLevelBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The properties of Page that are only relevant for pages with page_type NOTES."]
    pub struct NotesProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "speakerNotesObjectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the shape on this notes page that contains the speaker notes for the corresponding slide. The actual shape may not always exist on the notes page. Inserting text using this object ID will automatically create the shape. In this case, the actual shape may have different object ID. The `GetPresentation` or `GetPage` action will always return the latest object ID."]
        pub speaker_notes_object_id: ::std::option::Option<::std::string::String>,
    }
    impl NotesProperties {
        pub fn builder() -> NotesPropertiesBuilder {
            NotesPropertiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A themeable solid color value."]
    pub struct OpaqueColor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rgbColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An opaque RGB color."]
        pub rgb_color: ::std::option::Option<::std::boxed::Box<RgbColor>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "themeColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An opaque theme color."]
        pub theme_color: ::std::option::Option<OpaqueColorThemeColorEnum>,
    }
    impl OpaqueColor {
        pub fn builder() -> OpaqueColorBuilder {
            OpaqueColorBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "An opaque theme color."]
    pub enum OpaqueColorThemeColorEnum {
        #[serde(rename = "THEME_COLOR_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified theme color. This value should not be used."]
        ThemeColorTypeUnspecified,
        #[serde(rename = "DARK1")]
        #[doc = "Represents the first dark color."]
        Dark1,
        #[serde(rename = "LIGHT1")]
        #[doc = "Represents the first light color."]
        Light1,
        #[serde(rename = "DARK2")]
        #[doc = "Represents the second dark color."]
        Dark2,
        #[serde(rename = "LIGHT2")]
        #[doc = "Represents the second light color."]
        Light2,
        #[serde(rename = "ACCENT1")]
        #[doc = "Represents the first accent color."]
        Accent1,
        #[serde(rename = "ACCENT2")]
        #[doc = "Represents the second accent color."]
        Accent2,
        #[serde(rename = "ACCENT3")]
        #[doc = "Represents the third accent color."]
        Accent3,
        #[serde(rename = "ACCENT4")]
        #[doc = "Represents the fourth accent color."]
        Accent4,
        #[serde(rename = "ACCENT5")]
        #[doc = "Represents the fifth accent color."]
        Accent5,
        #[serde(rename = "ACCENT6")]
        #[doc = "Represents the sixth accent color."]
        Accent6,
        #[serde(rename = "HYPERLINK")]
        #[doc = "Represents the color to use for hyperlinks."]
        Hyperlink,
        #[serde(rename = "FOLLOWED_HYPERLINK")]
        #[doc = "Represents the color to use for visited hyperlinks."]
        FollowedHyperlink,
        #[serde(rename = "TEXT1")]
        #[doc = "Represents the first text color."]
        Text1,
        #[serde(rename = "BACKGROUND1")]
        #[doc = "Represents the first background color."]
        Background1,
        #[serde(rename = "TEXT2")]
        #[doc = "Represents the second text color."]
        Text2,
        #[serde(rename = "BACKGROUND2")]
        #[doc = "Represents the second background color."]
        Background2,
    }
    impl ::std::default::Default for OpaqueColorThemeColorEnum {
        fn default() -> Self {
            Self::ThemeColorTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A color that can either be fully opaque or fully transparent."]
    pub struct OptionalColor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "opaqueColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, this will be used as an opaque color. If unset, this represents a transparent color."]
        pub opaque_color: ::std::option::Option<::std::boxed::Box<OpaqueColor>>,
    }
    impl OptionalColor {
        pub fn builder() -> OptionalColorBuilder {
            OptionalColorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The outline of a PageElement. If these fields are unset, they may be inherited from a parent placeholder if it exists. If there is no parent, the fields will default to the value used for new page elements created in the Slides editor, which may depend on the page element kind."]
    pub struct Outline {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dashStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dash style of the outline."]
        pub dash_style: ::std::option::Option<OutlineDashStyleEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outlineFill")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fill of the outline."]
        pub outline_fill: ::std::option::Option<::std::boxed::Box<OutlineFill>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "propertyState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The outline property state. Updating the outline on a page element will implicitly update this field to `RENDERED`, unless another value is specified in the same request. To have no outline on a page element, set this field to `NOT_RENDERED`. In this case, any other outline fields set in the same request will be ignored."]
        pub property_state: ::std::option::Option<OutlinePropertyStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "weight")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The thickness of the outline."]
        pub weight: ::std::option::Option<::std::boxed::Box<Dimension>>,
    }
    impl Outline {
        pub fn builder() -> OutlineBuilder {
            OutlineBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The dash style of the outline."]
    pub enum OutlineDashStyleEnum {
        #[serde(rename = "DASH_STYLE_UNSPECIFIED")]
        #[doc = "Unspecified dash style."]
        DashStyleUnspecified,
        #[serde(rename = "SOLID")]
        #[doc = "Solid line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'solid'. This is the default dash style."]
        Solid,
        #[serde(rename = "DOT")]
        #[doc = "Dotted line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dot'."]
        Dot,
        #[serde(rename = "DASH")]
        #[doc = "Dashed line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dash'."]
        Dash,
        #[serde(rename = "DASH_DOT")]
        #[doc = "Alternating dashes and dots. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dashDot'."]
        DashDot,
        #[serde(rename = "LONG_DASH")]
        #[doc = "Line with large dashes. Corresponds to ECMA-376 ST_PresetLineDashVal value 'lgDash'."]
        LongDash,
        #[serde(rename = "LONG_DASH_DOT")]
        #[doc = "Alternating large dashes and dots. Corresponds to ECMA-376 ST_PresetLineDashVal value 'lgDashDot'."]
        LongDashDot,
    }
    impl ::std::default::Default for OutlineDashStyleEnum {
        fn default() -> Self {
            Self::DashStyleUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The outline property state. Updating the outline on a page element will implicitly update this field to `RENDERED`, unless another value is specified in the same request. To have no outline on a page element, set this field to `NOT_RENDERED`. In this case, any other outline fields set in the same request will be ignored."]
    pub enum OutlinePropertyStateEnum {
        #[serde(rename = "RENDERED")]
        #[doc = "If a property's state is RENDERED, then the element has the corresponding property when rendered on a page. If the element is a placeholder shape as determined by the placeholder field, and it inherits from a placeholder shape, the corresponding field may be unset, meaning that the property value is inherited from a parent placeholder. If the element does not inherit, then the field will contain the rendered value. This is the default value."]
        Rendered,
        #[serde(rename = "NOT_RENDERED")]
        #[doc = "If a property's state is NOT_RENDERED, then the element does not have the corresponding property when rendered on a page. However, the field may still be set so it can be inherited by child shapes. To remove a property from a rendered element, set its property_state to NOT_RENDERED."]
        NotRendered,
        #[serde(rename = "INHERIT")]
        #[doc = "If a property's state is INHERIT, then the property state uses the value of corresponding `property_state` field on the parent shape. Elements that do not inherit will never have an INHERIT property state."]
        Inherit,
    }
    impl ::std::default::Default for OutlinePropertyStateEnum {
        fn default() -> Self {
            Self::Rendered
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The fill of the outline."]
    pub struct OutlineFill {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "solidFill")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Solid color fill."]
        pub solid_fill: ::std::option::Option<::std::boxed::Box<SolidFill>>,
    }
    impl OutlineFill {
        pub fn builder() -> OutlineFillBuilder {
            OutlineFillBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A page in a presentation."]
    pub struct Page {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layoutProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Layout specific properties. Only set if page_type = LAYOUT."]
        pub layout_properties: ::std::option::Option<::std::boxed::Box<LayoutProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "masterProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Master specific properties. Only set if page_type = MASTER."]
        pub master_properties: ::std::option::Option<::std::boxed::Box<MasterProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notesProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Notes specific properties. Only set if page_type = NOTES."]
        pub notes_properties: ::std::option::Option<::std::boxed::Box<NotesProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID for this page. Object IDs used by Page and PageElement share the same namespace."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageElements")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The page elements rendered on the page."]
        pub page_elements: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PageElement>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The properties of the page."]
        pub page_properties: ::std::option::Option<::std::boxed::Box<PageProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the page."]
        pub page_type: ::std::option::Option<PagePageTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revisionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The revision ID of the presentation containing this page. Can be used in update requests to assert that the presentation revision hasn't changed since the last read operation. Only populated if the user has edit access to the presentation. The format of the revision ID may change over time, so it should be treated opaquely. A returned revision ID is only guaranteed to be valid for 24 hours after it has been returned and cannot be shared across users. If the revision ID is unchanged between calls, then the presentation has not changed. Conversely, a changed ID (for the same presentation and user) usually means the presentation has been updated; however, a changed ID can also be due to internal factors such as ID format changes."]
        pub revision_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "slideProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Slide specific properties. Only set if page_type = SLIDE."]
        pub slide_properties: ::std::option::Option<::std::boxed::Box<SlideProperties>>,
    }
    impl Page {
        pub fn builder() -> PageBuilder {
            PageBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the page."]
    pub enum PagePageTypeEnum {
        #[serde(rename = "SLIDE")]
        #[doc = "A slide page."]
        Slide,
        #[serde(rename = "MASTER")]
        #[doc = "A master slide page."]
        Master,
        #[serde(rename = "LAYOUT")]
        #[doc = "A layout page."]
        Layout,
        #[serde(rename = "NOTES")]
        #[doc = "A notes page."]
        Notes,
        #[serde(rename = "NOTES_MASTER")]
        #[doc = "A notes master page."]
        NotesMaster,
    }
    impl ::std::default::Default for PagePageTypeEnum {
        fn default() -> Self {
            Self::Slide
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The page background fill."]
    pub struct PageBackgroundFill {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "propertyState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background fill property state. Updating the fill on a page will implicitly update this field to `RENDERED`, unless another value is specified in the same request. To have no fill on a page, set this field to `NOT_RENDERED`. In this case, any other fill fields set in the same request will be ignored."]
        pub property_state: ::std::option::Option<PageBackgroundFillPropertyStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "solidFill")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Solid color fill."]
        pub solid_fill: ::std::option::Option<::std::boxed::Box<SolidFill>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stretchedPictureFill")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Stretched picture fill."]
        pub stretched_picture_fill: ::std::option::Option<::std::boxed::Box<StretchedPictureFill>>,
    }
    impl PageBackgroundFill {
        pub fn builder() -> PageBackgroundFillBuilder {
            PageBackgroundFillBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The background fill property state. Updating the fill on a page will implicitly update this field to `RENDERED`, unless another value is specified in the same request. To have no fill on a page, set this field to `NOT_RENDERED`. In this case, any other fill fields set in the same request will be ignored."]
    pub enum PageBackgroundFillPropertyStateEnum {
        #[serde(rename = "RENDERED")]
        #[doc = "If a property's state is RENDERED, then the element has the corresponding property when rendered on a page. If the element is a placeholder shape as determined by the placeholder field, and it inherits from a placeholder shape, the corresponding field may be unset, meaning that the property value is inherited from a parent placeholder. If the element does not inherit, then the field will contain the rendered value. This is the default value."]
        Rendered,
        #[serde(rename = "NOT_RENDERED")]
        #[doc = "If a property's state is NOT_RENDERED, then the element does not have the corresponding property when rendered on a page. However, the field may still be set so it can be inherited by child shapes. To remove a property from a rendered element, set its property_state to NOT_RENDERED."]
        NotRendered,
        #[serde(rename = "INHERIT")]
        #[doc = "If a property's state is INHERIT, then the property state uses the value of corresponding `property_state` field on the parent shape. Elements that do not inherit will never have an INHERIT property state."]
        Inherit,
    }
    impl ::std::default::Default for PageBackgroundFillPropertyStateEnum {
        fn default() -> Self {
            Self::Rendered
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A visual element rendered on a page."]
    pub struct PageElement {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The description of the page element. Combined with title to display alt text. The field is not supported for Group elements."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "elementGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A collection of page elements joined as a single unit."]
        pub element_group: ::std::option::Option<::std::boxed::Box<Group>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An image page element."]
        pub image: ::std::option::Option<::std::boxed::Box<Image>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "line")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A line page element."]
        pub line: ::std::option::Option<::std::boxed::Box<Line>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID for this page element. Object IDs used by google.apps.slides.v1.Page and google.apps.slides.v1.PageElement share the same namespace."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shape")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A generic shape."]
        pub shape: ::std::option::Option<::std::boxed::Box<Shape>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sheetsChart")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A linked chart embedded from Google Sheets. Unlinked charts are represented as images."]
        pub sheets_chart: ::std::option::Option<::std::boxed::Box<SheetsChart>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size of the page element."]
        pub size: ::std::option::Option<::std::boxed::Box<Size>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "table")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A table page element."]
        pub table: ::std::option::Option<::std::boxed::Box<Table>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the page element. Combined with description to display alt text. The field is not supported for Group elements."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transform")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The transform of the page element. The visual appearance of the page element is determined by its absolute transform. To compute the absolute transform, preconcatenate a page element's transform with the transforms of all of its parent groups. If the page element is not in a group, its absolute transform is the same as the value in this field. The initial transform for the newly created Group is always the identity transform."]
        pub transform: ::std::option::Option<::std::boxed::Box<AffineTransform>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "video")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A video page element."]
        pub video: ::std::option::Option<::std::boxed::Box<Video>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "wordArt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A word art page element."]
        pub word_art: ::std::option::Option<::std::boxed::Box<WordArt>>,
    }
    impl PageElement {
        pub fn builder() -> PageElementBuilder {
            PageElementBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Common properties for a page element. Note: When you initially create a PageElement, the API may modify the values of both `size` and `transform`, but the visual size will be unchanged."]
    pub struct PageElementProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageObjectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the page where the element is located."]
        pub page_object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size of the element."]
        pub size: ::std::option::Option<::std::boxed::Box<Size>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transform")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The transform for the element."]
        pub transform: ::std::option::Option<::std::boxed::Box<AffineTransform>>,
    }
    impl PageElementProperties {
        pub fn builder() -> PageElementPropertiesBuilder {
            PageElementPropertiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The properties of the Page. The page will inherit properties from the parent page. Depending on the page type the hierarchy is defined in either SlideProperties or LayoutProperties."]
    pub struct PageProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "colorScheme")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color scheme of the page. If unset, the color scheme is inherited from a parent page. If the page has no parent, the color scheme uses a default Slides color scheme, matching the defaults in the Slides editor. Only the concrete colors of the first 12 ThemeColorTypes are editable. In addition, only the color scheme on `Master` pages can be updated. To update the field, a color scheme containing mappings from all the first 12 ThemeColorTypes to their concrete colors must be provided. Colors for the remaining ThemeColorTypes will be ignored."]
        pub color_scheme: ::std::option::Option<::std::boxed::Box<ColorScheme>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageBackgroundFill")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background fill of the page. If unset, the background fill is inherited from a parent page if it exists. If the page has no parent, then the background fill defaults to the corresponding fill in the Slides editor."]
        pub page_background_fill: ::std::option::Option<::std::boxed::Box<PageBackgroundFill>>,
    }
    impl PageProperties {
        pub fn builder() -> PagePropertiesBuilder {
            PagePropertiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A TextElement kind that represents the beginning of a new paragraph."]
    pub struct ParagraphMarker {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bullet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bullet for this paragraph. If not present, the paragraph does not belong to a list."]
        pub bullet: ::std::option::Option<::std::boxed::Box<Bullet>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "style")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The paragraph's style"]
        pub style: ::std::option::Option<::std::boxed::Box<ParagraphStyle>>,
    }
    impl ParagraphMarker {
        pub fn builder() -> ParagraphMarkerBuilder {
            ParagraphMarkerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Styles that apply to a whole paragraph. If this text is contained in a shape with a parent placeholder, then these paragraph styles may be inherited from the parent. Which paragraph styles are inherited depend on the nesting level of lists: * A paragraph not in a list will inherit its paragraph style from the paragraph at the 0 nesting level of the list inside the parent placeholder. * A paragraph in a list will inherit its paragraph style from the paragraph at its corresponding nesting level of the list inside the parent placeholder. Inherited paragraph styles are represented as unset fields in this message."]
    pub struct ParagraphStyle {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alignment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text alignment for this paragraph."]
        pub alignment: ::std::option::Option<ParagraphStyleAlignmentEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "direction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text direction of this paragraph. If unset, the value defaults to LEFT_TO_RIGHT since text direction is not inherited."]
        pub direction: ::std::option::Option<ParagraphStyleDirectionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "indentEnd")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount indentation for the paragraph on the side that corresponds to the end of the text, based on the current text direction. If unset, the value is inherited from the parent."]
        pub indent_end: ::std::option::Option<::std::boxed::Box<Dimension>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "indentFirstLine")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of indentation for the start of the first line of the paragraph. If unset, the value is inherited from the parent."]
        pub indent_first_line: ::std::option::Option<::std::boxed::Box<Dimension>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "indentStart")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount indentation for the paragraph on the side that corresponds to the start of the text, based on the current text direction. If unset, the value is inherited from the parent."]
        pub indent_start: ::std::option::Option<::std::boxed::Box<Dimension>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineSpacing")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of space between lines, as a percentage of normal, where normal is represented as 100.0. If unset, the value is inherited from the parent."]
        pub line_spacing: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spaceAbove")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of extra space above the paragraph. If unset, the value is inherited from the parent."]
        pub space_above: ::std::option::Option<::std::boxed::Box<Dimension>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spaceBelow")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of extra space below the paragraph. If unset, the value is inherited from the parent."]
        pub space_below: ::std::option::Option<::std::boxed::Box<Dimension>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spacingMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The spacing mode for the paragraph."]
        pub spacing_mode: ::std::option::Option<ParagraphStyleSpacingModeEnum>,
    }
    impl ParagraphStyle {
        pub fn builder() -> ParagraphStyleBuilder {
            ParagraphStyleBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The text alignment for this paragraph."]
    pub enum ParagraphStyleAlignmentEnum {
        #[serde(rename = "ALIGNMENT_UNSPECIFIED")]
        #[doc = "The paragraph alignment is inherited from the parent."]
        AlignmentUnspecified,
        #[serde(rename = "START")]
        #[doc = "The paragraph is aligned to the start of the line. Left-aligned for LTR text, right-aligned otherwise."]
        Start,
        #[serde(rename = "CENTER")]
        #[doc = "The paragraph is centered."]
        Center,
        #[serde(rename = "END")]
        #[doc = "The paragraph is aligned to the end of the line. Right-aligned for LTR text, left-aligned otherwise."]
        End,
        #[serde(rename = "JUSTIFIED")]
        #[doc = "The paragraph is justified."]
        Justified,
    }
    impl ::std::default::Default for ParagraphStyleAlignmentEnum {
        fn default() -> Self {
            Self::AlignmentUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The text direction of this paragraph. If unset, the value defaults to LEFT_TO_RIGHT since text direction is not inherited."]
    pub enum ParagraphStyleDirectionEnum {
        #[serde(rename = "TEXT_DIRECTION_UNSPECIFIED")]
        #[doc = "The text direction is inherited from the parent."]
        TextDirectionUnspecified,
        #[serde(rename = "LEFT_TO_RIGHT")]
        #[doc = "The text goes from left to right."]
        LeftToRight,
        #[serde(rename = "RIGHT_TO_LEFT")]
        #[doc = "The text goes from right to left."]
        RightToLeft,
    }
    impl ::std::default::Default for ParagraphStyleDirectionEnum {
        fn default() -> Self {
            Self::TextDirectionUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The spacing mode for the paragraph."]
    pub enum ParagraphStyleSpacingModeEnum {
        #[serde(rename = "SPACING_MODE_UNSPECIFIED")]
        #[doc = "The spacing mode is inherited from the parent."]
        SpacingModeUnspecified,
        #[serde(rename = "NEVER_COLLAPSE")]
        #[doc = "Paragraph spacing is always rendered."]
        NeverCollapse,
        #[serde(rename = "COLLAPSE_LISTS")]
        #[doc = "Paragraph spacing is skipped between list elements."]
        CollapseLists,
    }
    impl ::std::default::Default for ParagraphStyleSpacingModeEnum {
        fn default() -> Self {
            Self::SpacingModeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The placeholder information that uniquely identifies a placeholder shape."]
    pub struct Placeholder {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "index")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index of the placeholder. If the same placeholder types are present in the same page, they would have different index values."]
        pub index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentObjectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of this shape's parent placeholder. If unset, the parent placeholder shape does not exist, so the shape does not inherit properties from any other shape."]
        pub parent_object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the placeholder."]
        pub _type: ::std::option::Option<PlaceholderTypeEnum>,
    }
    impl Placeholder {
        pub fn builder() -> PlaceholderBuilder {
            PlaceholderBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the placeholder."]
    pub enum PlaceholderTypeEnum {
        #[serde(rename = "NONE")]
        #[doc = "Default value, signifies it is not a placeholder."]
        None,
        #[serde(rename = "BODY")]
        #[doc = "Body text."]
        Body,
        #[serde(rename = "CHART")]
        #[doc = "Chart or graph."]
        Chart,
        #[serde(rename = "CLIP_ART")]
        #[doc = "Clip art image."]
        ClipArt,
        #[serde(rename = "CENTERED_TITLE")]
        #[doc = "Title centered."]
        CenteredTitle,
        #[serde(rename = "DIAGRAM")]
        #[doc = "Diagram."]
        Diagram,
        #[serde(rename = "DATE_AND_TIME")]
        #[doc = "Date and time."]
        DateAndTime,
        #[serde(rename = "FOOTER")]
        #[doc = "Footer text."]
        Footer,
        #[serde(rename = "HEADER")]
        #[doc = "Header text."]
        Header,
        #[serde(rename = "MEDIA")]
        #[doc = "Multimedia."]
        Media,
        #[serde(rename = "OBJECT")]
        #[doc = "Any content type."]
        Object,
        #[serde(rename = "PICTURE")]
        #[doc = "Picture."]
        Picture,
        #[serde(rename = "SLIDE_NUMBER")]
        #[doc = "Number of a slide."]
        SlideNumber,
        #[serde(rename = "SUBTITLE")]
        #[doc = "Subtitle."]
        Subtitle,
        #[serde(rename = "TABLE")]
        #[doc = "Table."]
        Table,
        #[serde(rename = "TITLE")]
        #[doc = "Slide title."]
        Title,
        #[serde(rename = "SLIDE_IMAGE")]
        #[doc = "Slide image."]
        SlideImage,
    }
    impl ::std::default::Default for PlaceholderTypeEnum {
        fn default() -> Self {
            Self::None
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Google Slides presentation."]
    pub struct Presentation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layouts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The layouts in the presentation. A layout is a template that determines how content is arranged and styled on the slides that inherit from that layout."]
        pub layouts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Page>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locale")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The locale of the presentation, as an IETF BCP 47 language tag."]
        pub locale: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "masters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The slide masters in the presentation. A slide master contains all common page elements and the common properties for a set of layouts. They serve three purposes: - Placeholder shapes on a master contain the default text styles and shape properties of all placeholder shapes on pages that use that master. - The master page properties define the common page properties inherited by its layouts. - Any other shapes on the master slide appear on all slides using that master, regardless of their layout."]
        pub masters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Page>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notesMaster")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The notes master in the presentation. It serves three purposes: - Placeholder shapes on a notes master contain the default text styles and shape properties of all placeholder shapes on notes pages. Specifically, a `SLIDE_IMAGE` placeholder shape contains the slide thumbnail, and a `BODY` placeholder shape contains the speaker notes. - The notes master page properties define the common page properties inherited by all notes pages. - Any other shapes on the notes master appear on all notes pages. The notes master is read-only."]
        pub notes_master: ::std::option::Option<::std::boxed::Box<Page>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size of pages in the presentation."]
        pub page_size: ::std::option::Option<::std::boxed::Box<Size>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "presentationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the presentation."]
        pub presentation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revisionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The revision ID of the presentation. Can be used in update requests to assert that the presentation revision hasn't changed since the last read operation. Only populated if the user has edit access to the presentation. The format of the revision ID may change over time, so it should be treated opaquely. A returned revision ID is only guaranteed to be valid for 24 hours after it has been returned and cannot be shared across users. If the revision ID is unchanged between calls, then the presentation has not changed. Conversely, a changed ID (for the same presentation and user) usually means the presentation has been updated; however, a changed ID can also be due to internal factors such as ID format changes."]
        pub revision_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "slides")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The slides in the presentation. A slide inherits properties from a slide layout."]
        pub slides: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Page>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the presentation."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl Presentation {
        pub fn builder() -> PresentationBuilder {
            PresentationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies a contiguous range of an indexed collection, such as characters in text."]
    pub struct Range {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The optional zero-based index of the end of the collection. Required for `FIXED_RANGE` ranges."]
        pub end_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The optional zero-based index of the beginning of the collection. Required for `FIXED_RANGE` and `FROM_START_INDEX` ranges."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of range."]
        pub _type: ::std::option::Option<RangeTypeEnum>,
    }
    impl Range {
        pub fn builder() -> RangeBuilder {
            RangeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of range."]
    pub enum RangeTypeEnum {
        #[serde(rename = "RANGE_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified range type. This value must not be used."]
        RangeTypeUnspecified,
        #[serde(rename = "FIXED_RANGE")]
        #[doc = "A fixed range. Both the `start_index` and `end_index` must be specified."]
        FixedRange,
        #[serde(rename = "FROM_START_INDEX")]
        #[doc = "Starts the range at `start_index` and continues until the end of the collection. The `end_index` must not be specified."]
        FromStartIndex,
        #[serde(rename = "ALL")]
        #[doc = "Sets the range to be the whole length of the collection. Both the `start_index` and the `end_index` must not be specified."]
        All,
    }
    impl ::std::default::Default for RangeTypeEnum {
        fn default() -> Self {
            Self::RangeTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A recolor effect applied on an image."]
    pub struct Recolor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the recolor effect. The name is determined from the `recolor_stops` by matching the gradient against the colors in the page's current color scheme. This property is read-only."]
        pub name: ::std::option::Option<RecolorNameEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recolorStops")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The recolor effect is represented by a gradient, which is a list of color stops. The colors in the gradient will replace the corresponding colors at the same position in the color palette and apply to the image. This property is read-only."]
        pub recolor_stops: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ColorStop>>>,
    }
    impl Recolor {
        pub fn builder() -> RecolorBuilder {
            RecolorBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The name of the recolor effect. The name is determined from the `recolor_stops` by matching the gradient against the colors in the page's current color scheme. This property is read-only."]
    pub enum RecolorNameEnum {
        #[serde(rename = "NONE")]
        #[doc = "No recolor effect. The default value."]
        None,
        #[serde(rename = "LIGHT1")]
        #[doc = "A recolor effect that lightens the image using the page's first available color from its color scheme."]
        Light1,
        #[serde(rename = "LIGHT2")]
        #[doc = "A recolor effect that lightens the image using the page's second available color from its color scheme."]
        Light2,
        #[serde(rename = "LIGHT3")]
        #[doc = "A recolor effect that lightens the image using the page's third available color from its color scheme."]
        Light3,
        #[serde(rename = "LIGHT4")]
        #[doc = "A recolor effect that lightens the image using the page's forth available color from its color scheme."]
        Light4,
        #[serde(rename = "LIGHT5")]
        #[doc = "A recolor effect that lightens the image using the page's fifth available color from its color scheme."]
        Light5,
        #[serde(rename = "LIGHT6")]
        #[doc = "A recolor effect that lightens the image using the page's sixth available color from its color scheme."]
        Light6,
        #[serde(rename = "LIGHT7")]
        #[doc = "A recolor effect that lightens the image using the page's seventh available color from its color scheme."]
        Light7,
        #[serde(rename = "LIGHT8")]
        #[doc = "A recolor effect that lightens the image using the page's eighth available color from its color scheme."]
        Light8,
        #[serde(rename = "LIGHT9")]
        #[doc = "A recolor effect that lightens the image using the page's ninth available color from its color scheme."]
        Light9,
        #[serde(rename = "LIGHT10")]
        #[doc = "A recolor effect that lightens the image using the page's tenth available color from its color scheme."]
        Light10,
        #[serde(rename = "DARK1")]
        #[doc = "A recolor effect that darkens the image using the page's first available color from its color scheme."]
        Dark1,
        #[serde(rename = "DARK2")]
        #[doc = "A recolor effect that darkens the image using the page's second available color from its color scheme."]
        Dark2,
        #[serde(rename = "DARK3")]
        #[doc = "A recolor effect that darkens the image using the page's third available color from its color scheme."]
        Dark3,
        #[serde(rename = "DARK4")]
        #[doc = "A recolor effect that darkens the image using the page's fourth available color from its color scheme."]
        Dark4,
        #[serde(rename = "DARK5")]
        #[doc = "A recolor effect that darkens the image using the page's fifth available color from its color scheme."]
        Dark5,
        #[serde(rename = "DARK6")]
        #[doc = "A recolor effect that darkens the image using the page's sixth available color from its color scheme."]
        Dark6,
        #[serde(rename = "DARK7")]
        #[doc = "A recolor effect that darkens the image using the page's seventh available color from its color scheme."]
        Dark7,
        #[serde(rename = "DARK8")]
        #[doc = "A recolor effect that darkens the image using the page's eighth available color from its color scheme."]
        Dark8,
        #[serde(rename = "DARK9")]
        #[doc = "A recolor effect that darkens the image using the page's ninth available color from its color scheme."]
        Dark9,
        #[serde(rename = "DARK10")]
        #[doc = "A recolor effect that darkens the image using the page's tenth available color from its color scheme."]
        Dark10,
        #[serde(rename = "GRAYSCALE")]
        #[doc = "A recolor effect that recolors the image to grayscale."]
        Grayscale,
        #[serde(rename = "NEGATIVE")]
        #[doc = "A recolor effect that recolors the image to negative grayscale."]
        Negative,
        #[serde(rename = "SEPIA")]
        #[doc = "A recolor effect that recolors the image using the sepia color."]
        Sepia,
        #[serde(rename = "CUSTOM")]
        #[doc = "Custom recolor effect. Refer to `recolor_stops` for the concrete gradient."]
        Custom,
    }
    impl ::std::default::Default for RecolorNameEnum {
        fn default() -> Self {
            Self::None
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Refreshes an embedded Google Sheets chart by replacing it with the latest version of the chart from Google Sheets. NOTE: Refreshing charts requires at least one of the spreadsheets.readonly, spreadsheets, drive.readonly, or drive OAuth scopes."]
    pub struct RefreshSheetsChartRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the chart to refresh."]
        pub object_id: ::std::option::Option<::std::string::String>,
    }
    impl RefreshSheetsChartRequest {
        pub fn builder() -> RefreshSheetsChartRequestBuilder {
            RefreshSheetsChartRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Replaces all shapes that match the given criteria with the provided image. The images replacing the shapes are rectangular after being inserted into the presentation and do not take on the forms of the shapes."]
    pub struct ReplaceAllShapesWithImageRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containsText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, this request will replace all of the shapes that contain the given text."]
        pub contains_text: ::std::option::Option<::std::boxed::Box<SubstringMatchCriteria>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageReplaceMethod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The image replace method. If you specify both a `replace_method` and an `image_replace_method`, the `image_replace_method` takes precedence. If you do not specify a value for `image_replace_method`, but specify a value for `replace_method`, then the specified `replace_method` value is used. If you do not specify either, then CENTER_INSIDE is used."]
        pub image_replace_method:
            ::std::option::Option<ReplaceAllShapesWithImageRequestImageReplaceMethodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The image URL. The image is fetched once at insertion time and a copy is stored for display inside the presentation. Images must be less than 50MB in size, cannot exceed 25 megapixels, and must be in one of PNG, JPEG, or GIF format. The provided URL can be at most 2 kB in length. The URL itself is saved with the image, and exposed via the Image.source_url field."]
        pub image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageObjectIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If non-empty, limits the matches to page elements only on the given pages. Returns a 400 bad request error if given the page object ID of a notes page or a notes master, or if a page with that object ID doesn't exist in the presentation."]
        pub page_object_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replaceMethod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The replace method. *Deprecated*: use `image_replace_method` instead. If you specify both a `replace_method` and an `image_replace_method`, the `image_replace_method` takes precedence."]
        pub replace_method:
            ::std::option::Option<ReplaceAllShapesWithImageRequestReplaceMethodEnum>,
    }
    impl ReplaceAllShapesWithImageRequest {
        pub fn builder() -> ReplaceAllShapesWithImageRequestBuilder {
            ReplaceAllShapesWithImageRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The image replace method. If you specify both a `replace_method` and an `image_replace_method`, the `image_replace_method` takes precedence. If you do not specify a value for `image_replace_method`, but specify a value for `replace_method`, then the specified `replace_method` value is used. If you do not specify either, then CENTER_INSIDE is used."]
    pub enum ReplaceAllShapesWithImageRequestImageReplaceMethodEnum {
        #[serde(rename = "IMAGE_REPLACE_METHOD_UNSPECIFIED")]
        #[doc = "Unspecified image replace method. This value must not be used."]
        ImageReplaceMethodUnspecified,
        #[serde(rename = "CENTER_INSIDE")]
        #[doc = "Scales and centers the image to fit within the bounds of the original shape and maintains the image's aspect ratio. The rendered size of the image may be smaller than the size of the shape. This is the default method when one is not specified."]
        CenterInside,
        #[serde(rename = "CENTER_CROP")]
        #[doc = "Scales and centers the image to fill the bounds of the original shape. The image may be cropped in order to fill the shape. The rendered size of the image will be the same as that of the original shape."]
        CenterCrop,
    }
    impl ::std::default::Default for ReplaceAllShapesWithImageRequestImageReplaceMethodEnum {
        fn default() -> Self {
            Self::ImageReplaceMethodUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The replace method. *Deprecated*: use `image_replace_method` instead. If you specify both a `replace_method` and an `image_replace_method`, the `image_replace_method` takes precedence."]
    pub enum ReplaceAllShapesWithImageRequestReplaceMethodEnum {
        #[serde(rename = "CENTER_INSIDE")]
        #[doc = "Scales and centers the image to fit within the bounds of the original shape and maintains the image's aspect ratio. The rendered size of the image may be smaller than the size of the shape. This is the default method when one is not specified."]
        CenterInside,
        #[serde(rename = "CENTER_CROP")]
        #[doc = "Scales and centers the image to fill the bounds of the original shape. The image may be cropped in order to fill the shape. The rendered size of the image will be the same as that of the original shape."]
        CenterCrop,
    }
    impl ::std::default::Default for ReplaceAllShapesWithImageRequestReplaceMethodEnum {
        fn default() -> Self {
            Self::CenterInside
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of replacing shapes with an image."]
    pub struct ReplaceAllShapesWithImageResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "occurrencesChanged")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of shapes replaced with images."]
        pub occurrences_changed: ::std::option::Option<::std::primitive::i64>,
    }
    impl ReplaceAllShapesWithImageResponse {
        pub fn builder() -> ReplaceAllShapesWithImageResponseBuilder {
            ReplaceAllShapesWithImageResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Replaces all shapes that match the given criteria with the provided Google Sheets chart. The chart will be scaled and centered to fit within the bounds of the original shape. NOTE: Replacing shapes with a chart requires at least one of the spreadsheets.readonly, spreadsheets, drive.readonly, or drive OAuth scopes."]
    pub struct ReplaceAllShapesWithSheetsChartRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "chartId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the specific chart in the Google Sheets spreadsheet."]
        pub chart_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containsText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The criteria that the shapes must match in order to be replaced. The request will replace all of the shapes that contain the given text."]
        pub contains_text: ::std::option::Option<::std::boxed::Box<SubstringMatchCriteria>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linkingMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The mode with which the chart is linked to the source spreadsheet. When not specified, the chart will be an image that is not linked."]
        pub linking_mode:
            ::std::option::Option<ReplaceAllShapesWithSheetsChartRequestLinkingModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageObjectIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If non-empty, limits the matches to page elements only on the given pages. Returns a 400 bad request error if given the page object ID of a notes page or a notes master, or if a page with that object ID doesn't exist in the presentation."]
        pub page_object_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spreadsheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the Google Sheets spreadsheet that contains the chart."]
        pub spreadsheet_id: ::std::option::Option<::std::string::String>,
    }
    impl ReplaceAllShapesWithSheetsChartRequest {
        pub fn builder() -> ReplaceAllShapesWithSheetsChartRequestBuilder {
            ReplaceAllShapesWithSheetsChartRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The mode with which the chart is linked to the source spreadsheet. When not specified, the chart will be an image that is not linked."]
    pub enum ReplaceAllShapesWithSheetsChartRequestLinkingModeEnum {
        #[serde(rename = "NOT_LINKED_IMAGE")]
        #[doc = "The chart is not associated with the source spreadsheet and cannot be updated. A chart that is not linked will be inserted as an image."]
        NotLinkedImage,
        #[serde(rename = "LINKED")]
        #[doc = "Linking the chart allows it to be updated, and other collaborators will see a link to the spreadsheet."]
        Linked,
    }
    impl ::std::default::Default for ReplaceAllShapesWithSheetsChartRequestLinkingModeEnum {
        fn default() -> Self {
            Self::NotLinkedImage
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of replacing shapes with a Google Sheets chart."]
    pub struct ReplaceAllShapesWithSheetsChartResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "occurrencesChanged")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of shapes replaced with charts."]
        pub occurrences_changed: ::std::option::Option<::std::primitive::i64>,
    }
    impl ReplaceAllShapesWithSheetsChartResponse {
        pub fn builder() -> ReplaceAllShapesWithSheetsChartResponseBuilder {
            ReplaceAllShapesWithSheetsChartResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Replaces all instances of text matching a criteria with replace text."]
    pub struct ReplaceAllTextRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containsText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Finds text in a shape matching this substring."]
        pub contains_text: ::std::option::Option<::std::boxed::Box<SubstringMatchCriteria>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageObjectIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If non-empty, limits the matches to page elements only on the given pages. Returns a 400 bad request error if given the page object ID of a notes master, or if a page with that object ID doesn't exist in the presentation."]
        pub page_object_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replaceText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text that will replace the matched text."]
        pub replace_text: ::std::option::Option<::std::string::String>,
    }
    impl ReplaceAllTextRequest {
        pub fn builder() -> ReplaceAllTextRequestBuilder {
            ReplaceAllTextRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of replacing text."]
    pub struct ReplaceAllTextResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "occurrencesChanged")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of occurrences changed by replacing all text."]
        pub occurrences_changed: ::std::option::Option<::std::primitive::i64>,
    }
    impl ReplaceAllTextResponse {
        pub fn builder() -> ReplaceAllTextResponseBuilder {
            ReplaceAllTextResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Replaces an existing image with a new image. Replacing an image removes some image effects from the existing image."]
    pub struct ReplaceImageRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageObjectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the existing image that will be replaced."]
        pub image_object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageReplaceMethod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The replacement method."]
        pub image_replace_method: ::std::option::Option<ReplaceImageRequestImageReplaceMethodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The image URL. The image is fetched once at insertion time and a copy is stored for display inside the presentation. Images must be less than 50MB in size, cannot exceed 25 megapixels, and must be in one of PNG, JPEG, or GIF format. The provided URL can be at most 2 kB in length. The URL itself is saved with the image, and exposed via the Image.source_url field."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl ReplaceImageRequest {
        pub fn builder() -> ReplaceImageRequestBuilder {
            ReplaceImageRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The replacement method."]
    pub enum ReplaceImageRequestImageReplaceMethodEnum {
        #[serde(rename = "IMAGE_REPLACE_METHOD_UNSPECIFIED")]
        #[doc = "Unspecified image replace method. This value must not be used."]
        ImageReplaceMethodUnspecified,
        #[serde(rename = "CENTER_INSIDE")]
        #[doc = "Scales and centers the image to fit within the bounds of the original shape and maintains the image's aspect ratio. The rendered size of the image may be smaller than the size of the shape. This is the default method when one is not specified."]
        CenterInside,
        #[serde(rename = "CENTER_CROP")]
        #[doc = "Scales and centers the image to fill the bounds of the original shape. The image may be cropped in order to fill the shape. The rendered size of the image will be the same as that of the original shape."]
        CenterCrop,
    }
    impl ::std::default::Default for ReplaceImageRequestImageReplaceMethodEnum {
        fn default() -> Self {
            Self::ImageReplaceMethodUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single kind of update to apply to a presentation."]
    pub struct Request {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createImage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Creates an image."]
        pub create_image: ::std::option::Option<::std::boxed::Box<CreateImageRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createLine")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Creates a line."]
        pub create_line: ::std::option::Option<::std::boxed::Box<CreateLineRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createParagraphBullets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Creates bullets for paragraphs."]
        pub create_paragraph_bullets:
            ::std::option::Option<::std::boxed::Box<CreateParagraphBulletsRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createShape")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Creates a new shape."]
        pub create_shape: ::std::option::Option<::std::boxed::Box<CreateShapeRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createSheetsChart")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Creates an embedded Google Sheets chart."]
        pub create_sheets_chart: ::std::option::Option<::std::boxed::Box<CreateSheetsChartRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createSlide")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Creates a new slide."]
        pub create_slide: ::std::option::Option<::std::boxed::Box<CreateSlideRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Creates a new table."]
        pub create_table: ::std::option::Option<::std::boxed::Box<CreateTableRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createVideo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Creates a video."]
        pub create_video: ::std::option::Option<::std::boxed::Box<CreateVideoRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteObject")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deletes a page or page element from the presentation."]
        pub delete_object: ::std::option::Option<::std::boxed::Box<DeleteObjectRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteParagraphBullets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deletes bullets from paragraphs."]
        pub delete_paragraph_bullets:
            ::std::option::Option<::std::boxed::Box<DeleteParagraphBulletsRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteTableColumn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deletes a column from a table."]
        pub delete_table_column: ::std::option::Option<::std::boxed::Box<DeleteTableColumnRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteTableRow")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deletes a row from a table."]
        pub delete_table_row: ::std::option::Option<::std::boxed::Box<DeleteTableRowRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deletes text from a shape or a table cell."]
        pub delete_text: ::std::option::Option<::std::boxed::Box<DeleteTextRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duplicateObject")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Duplicates a slide or page element."]
        pub duplicate_object: ::std::option::Option<::std::boxed::Box<DuplicateObjectRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groupObjects")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Groups objects, such as page elements."]
        pub group_objects: ::std::option::Option<::std::boxed::Box<GroupObjectsRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertTableColumns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Inserts columns into a table."]
        pub insert_table_columns:
            ::std::option::Option<::std::boxed::Box<InsertTableColumnsRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertTableRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Inserts rows into a table."]
        pub insert_table_rows: ::std::option::Option<::std::boxed::Box<InsertTableRowsRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Inserts text into a shape or table cell."]
        pub insert_text: ::std::option::Option<::std::boxed::Box<InsertTextRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mergeTableCells")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Merges cells in a Table."]
        pub merge_table_cells: ::std::option::Option<::std::boxed::Box<MergeTableCellsRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "refreshSheetsChart")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Refreshes a Google Sheets chart."]
        pub refresh_sheets_chart:
            ::std::option::Option<::std::boxed::Box<RefreshSheetsChartRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replaceAllShapesWithImage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Replaces all shapes matching some criteria with an image."]
        pub replace_all_shapes_with_image:
            ::std::option::Option<::std::boxed::Box<ReplaceAllShapesWithImageRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replaceAllShapesWithSheetsChart")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Replaces all shapes matching some criteria with a Google Sheets chart."]
        pub replace_all_shapes_with_sheets_chart:
            ::std::option::Option<::std::boxed::Box<ReplaceAllShapesWithSheetsChartRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replaceAllText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Replaces all instances of specified text."]
        pub replace_all_text: ::std::option::Option<::std::boxed::Box<ReplaceAllTextRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replaceImage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Replaces an existing image with a new image."]
        pub replace_image: ::std::option::Option<::std::boxed::Box<ReplaceImageRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rerouteLine")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reroutes a line such that it's connected at the two closest connection sites on the connected page elements."]
        pub reroute_line: ::std::option::Option<::std::boxed::Box<RerouteLineRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ungroupObjects")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ungroups objects, such as groups."]
        pub ungroup_objects: ::std::option::Option<::std::boxed::Box<UngroupObjectsRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unmergeTableCells")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unmerges cells in a Table."]
        pub unmerge_table_cells: ::std::option::Option<::std::boxed::Box<UnmergeTableCellsRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateImageProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates the properties of an Image."]
        pub update_image_properties:
            ::std::option::Option<::std::boxed::Box<UpdateImagePropertiesRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateLineCategory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates the category of a line."]
        pub update_line_category:
            ::std::option::Option<::std::boxed::Box<UpdateLineCategoryRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateLineProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates the properties of a Line."]
        pub update_line_properties:
            ::std::option::Option<::std::boxed::Box<UpdateLinePropertiesRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updatePageElementAltText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates the alt text title and/or description of a page element."]
        pub update_page_element_alt_text:
            ::std::option::Option<::std::boxed::Box<UpdatePageElementAltTextRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updatePageElementTransform")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates the transform of a page element."]
        pub update_page_element_transform:
            ::std::option::Option<::std::boxed::Box<UpdatePageElementTransformRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updatePageElementsZOrder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates the Z-order of page elements."]
        pub update_page_elements_z_order:
            ::std::option::Option<::std::boxed::Box<UpdatePageElementsZOrderRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updatePageProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates the properties of a Page."]
        pub update_page_properties:
            ::std::option::Option<::std::boxed::Box<UpdatePagePropertiesRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateParagraphStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates the styling of paragraphs within a Shape or Table."]
        pub update_paragraph_style:
            ::std::option::Option<::std::boxed::Box<UpdateParagraphStyleRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateShapeProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates the properties of a Shape."]
        pub update_shape_properties:
            ::std::option::Option<::std::boxed::Box<UpdateShapePropertiesRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateSlidesPosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates the position of a set of slides in the presentation."]
        pub update_slides_position:
            ::std::option::Option<::std::boxed::Box<UpdateSlidesPositionRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTableBorderProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates the properties of the table borders in a Table."]
        pub update_table_border_properties:
            ::std::option::Option<::std::boxed::Box<UpdateTableBorderPropertiesRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTableCellProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates the properties of a TableCell."]
        pub update_table_cell_properties:
            ::std::option::Option<::std::boxed::Box<UpdateTableCellPropertiesRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTableColumnProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates the properties of a Table column."]
        pub update_table_column_properties:
            ::std::option::Option<::std::boxed::Box<UpdateTableColumnPropertiesRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTableRowProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates the properties of a Table row."]
        pub update_table_row_properties:
            ::std::option::Option<::std::boxed::Box<UpdateTableRowPropertiesRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTextStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates the styling of text within a Shape or Table."]
        pub update_text_style: ::std::option::Option<::std::boxed::Box<UpdateTextStyleRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateVideoProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updates the properties of a Video."]
        pub update_video_properties:
            ::std::option::Option<::std::boxed::Box<UpdateVideoPropertiesRequest>>,
    }
    impl Request {
        pub fn builder() -> RequestBuilder {
            RequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Reroutes a line such that it's connected at the two closest connection sites on the connected page elements."]
    pub struct RerouteLineRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the line to reroute. Only a line with a category indicating it is a \"connector\" can be rerouted. The start and end connections of the line must be on different page elements."]
        pub object_id: ::std::option::Option<::std::string::String>,
    }
    impl RerouteLineRequest {
        pub fn builder() -> RerouteLineRequestBuilder {
            RerouteLineRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single response from an update."]
    pub struct Response {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createImage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of creating an image."]
        pub create_image: ::std::option::Option<::std::boxed::Box<CreateImageResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createLine")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of creating a line."]
        pub create_line: ::std::option::Option<::std::boxed::Box<CreateLineResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createShape")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of creating a shape."]
        pub create_shape: ::std::option::Option<::std::boxed::Box<CreateShapeResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createSheetsChart")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of creating a Google Sheets chart."]
        pub create_sheets_chart:
            ::std::option::Option<::std::boxed::Box<CreateSheetsChartResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createSlide")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of creating a slide."]
        pub create_slide: ::std::option::Option<::std::boxed::Box<CreateSlideResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of creating a table."]
        pub create_table: ::std::option::Option<::std::boxed::Box<CreateTableResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createVideo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of creating a video."]
        pub create_video: ::std::option::Option<::std::boxed::Box<CreateVideoResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duplicateObject")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of duplicating an object."]
        pub duplicate_object: ::std::option::Option<::std::boxed::Box<DuplicateObjectResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groupObjects")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of grouping objects."]
        pub group_objects: ::std::option::Option<::std::boxed::Box<GroupObjectsResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replaceAllShapesWithImage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of replacing all shapes matching some criteria with an image."]
        pub replace_all_shapes_with_image:
            ::std::option::Option<::std::boxed::Box<ReplaceAllShapesWithImageResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replaceAllShapesWithSheetsChart")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of replacing all shapes matching some criteria with a Google Sheets chart."]
        pub replace_all_shapes_with_sheets_chart:
            ::std::option::Option<::std::boxed::Box<ReplaceAllShapesWithSheetsChartResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replaceAllText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of replacing text."]
        pub replace_all_text: ::std::option::Option<::std::boxed::Box<ReplaceAllTextResponse>>,
    }
    impl Response {
        pub fn builder() -> ResponseBuilder {
            ResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An RGB color."]
    pub struct RgbColor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The blue component of the color, from 0.0 to 1.0."]
        pub blue: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "green")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The green component of the color, from 0.0 to 1.0."]
        pub green: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "red")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The red component of the color, from 0.0 to 1.0."]
        pub red: ::std::option::Option<::std::primitive::f64>,
    }
    impl RgbColor {
        pub fn builder() -> RgbColorBuilder {
            RgbColorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The shadow properties of a page element. If these fields are unset, they may be inherited from a parent placeholder if it exists. If there is no parent, the fields will default to the value used for new page elements created in the Slides editor, which may depend on the page element kind."]
    pub struct Shadow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alignment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The alignment point of the shadow, that sets the origin for translate, scale and skew of the shadow. This property is read-only."]
        pub alignment: ::std::option::Option<ShadowAlignmentEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alpha")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The alpha of the shadow's color, from 0.0 to 1.0."]
        pub alpha: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blurRadius")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The radius of the shadow blur. The larger the radius, the more diffuse the shadow becomes."]
        pub blur_radius: ::std::option::Option<::std::boxed::Box<Dimension>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "color")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The shadow color value."]
        pub color: ::std::option::Option<::std::boxed::Box<OpaqueColor>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "propertyState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The shadow property state. Updating the shadow on a page element will implicitly update this field to `RENDERED`, unless another value is specified in the same request. To have no shadow on a page element, set this field to `NOT_RENDERED`. In this case, any other shadow fields set in the same request will be ignored."]
        pub property_state: ::std::option::Option<ShadowPropertyStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rotateWithShape")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the shadow should rotate with the shape. This property is read-only."]
        pub rotate_with_shape: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transform")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Transform that encodes the translate, scale, and skew of the shadow, relative to the alignment position."]
        pub transform: ::std::option::Option<::std::boxed::Box<AffineTransform>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the shadow. This property is read-only."]
        pub _type: ::std::option::Option<ShadowTypeEnum>,
    }
    impl Shadow {
        pub fn builder() -> ShadowBuilder {
            ShadowBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The alignment point of the shadow, that sets the origin for translate, scale and skew of the shadow. This property is read-only."]
    pub enum ShadowAlignmentEnum {
        #[serde(rename = "RECTANGLE_POSITION_UNSPECIFIED")]
        #[doc = "Unspecified."]
        RectanglePositionUnspecified,
        #[serde(rename = "TOP_LEFT")]
        #[doc = "Top left."]
        TopLeft,
        #[serde(rename = "TOP_CENTER")]
        #[doc = "Top center."]
        TopCenter,
        #[serde(rename = "TOP_RIGHT")]
        #[doc = "Top right."]
        TopRight,
        #[serde(rename = "LEFT_CENTER")]
        #[doc = "Left center."]
        LeftCenter,
        #[serde(rename = "CENTER")]
        #[doc = "Center."]
        Center,
        #[serde(rename = "RIGHT_CENTER")]
        #[doc = "Right center."]
        RightCenter,
        #[serde(rename = "BOTTOM_LEFT")]
        #[doc = "Bottom left."]
        BottomLeft,
        #[serde(rename = "BOTTOM_CENTER")]
        #[doc = "Bottom center."]
        BottomCenter,
        #[serde(rename = "BOTTOM_RIGHT")]
        #[doc = "Bottom right."]
        BottomRight,
    }
    impl ::std::default::Default for ShadowAlignmentEnum {
        fn default() -> Self {
            Self::RectanglePositionUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The shadow property state. Updating the shadow on a page element will implicitly update this field to `RENDERED`, unless another value is specified in the same request. To have no shadow on a page element, set this field to `NOT_RENDERED`. In this case, any other shadow fields set in the same request will be ignored."]
    pub enum ShadowPropertyStateEnum {
        #[serde(rename = "RENDERED")]
        #[doc = "If a property's state is RENDERED, then the element has the corresponding property when rendered on a page. If the element is a placeholder shape as determined by the placeholder field, and it inherits from a placeholder shape, the corresponding field may be unset, meaning that the property value is inherited from a parent placeholder. If the element does not inherit, then the field will contain the rendered value. This is the default value."]
        Rendered,
        #[serde(rename = "NOT_RENDERED")]
        #[doc = "If a property's state is NOT_RENDERED, then the element does not have the corresponding property when rendered on a page. However, the field may still be set so it can be inherited by child shapes. To remove a property from a rendered element, set its property_state to NOT_RENDERED."]
        NotRendered,
        #[serde(rename = "INHERIT")]
        #[doc = "If a property's state is INHERIT, then the property state uses the value of corresponding `property_state` field on the parent shape. Elements that do not inherit will never have an INHERIT property state."]
        Inherit,
    }
    impl ::std::default::Default for ShadowPropertyStateEnum {
        fn default() -> Self {
            Self::Rendered
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the shadow. This property is read-only."]
    pub enum ShadowTypeEnum {
        #[serde(rename = "SHADOW_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified shadow type."]
        ShadowTypeUnspecified,
        #[serde(rename = "OUTER")]
        #[doc = "Outer shadow."]
        Outer,
    }
    impl ::std::default::Default for ShadowTypeEnum {
        fn default() -> Self {
            Self::ShadowTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A PageElement kind representing a generic shape that does not have a more specific classification."]
    pub struct Shape {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "placeholder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Placeholders are shapes that are inherit from corresponding placeholders on layouts and masters. If set, the shape is a placeholder shape and any inherited properties can be resolved by looking at the parent placeholder identified by the Placeholder.parent_object_id field."]
        pub placeholder: ::std::option::Option<::std::boxed::Box<Placeholder>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shapeProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The properties of the shape."]
        pub shape_properties: ::std::option::Option<::std::boxed::Box<ShapeProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shapeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the shape."]
        pub shape_type: ::std::option::Option<ShapeShapeTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text content of the shape."]
        pub text: ::std::option::Option<::std::boxed::Box<TextContent>>,
    }
    impl Shape {
        pub fn builder() -> ShapeBuilder {
            ShapeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the shape."]
    pub enum ShapeShapeTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "The shape type that is not predefined."]
        TypeUnspecified,
        #[serde(rename = "TEXT_BOX")]
        #[doc = "Text box shape."]
        TextBox,
        #[serde(rename = "RECTANGLE")]
        #[doc = "Rectangle shape. Corresponds to ECMA-376 ST_ShapeType 'rect'."]
        Rectangle,
        #[serde(rename = "ROUND_RECTANGLE")]
        #[doc = "Round corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType 'roundRect'"]
        RoundRectangle,
        #[serde(rename = "ELLIPSE")]
        #[doc = "Ellipse shape. Corresponds to ECMA-376 ST_ShapeType 'ellipse'"]
        Ellipse,
        #[serde(rename = "ARC")]
        #[doc = "Curved arc shape. Corresponds to ECMA-376 ST_ShapeType 'arc'"]
        Arc,
        #[serde(rename = "BENT_ARROW")]
        #[doc = "Bent arrow shape. Corresponds to ECMA-376 ST_ShapeType 'bentArrow'"]
        BentArrow,
        #[serde(rename = "BENT_UP_ARROW")]
        #[doc = "Bent up arrow shape. Corresponds to ECMA-376 ST_ShapeType 'bentUpArrow'"]
        BentUpArrow,
        #[serde(rename = "BEVEL")]
        #[doc = "Bevel shape. Corresponds to ECMA-376 ST_ShapeType 'bevel'"]
        Bevel,
        #[serde(rename = "BLOCK_ARC")]
        #[doc = "Block arc shape. Corresponds to ECMA-376 ST_ShapeType 'blockArc'"]
        BlockArc,
        #[serde(rename = "BRACE_PAIR")]
        #[doc = "Brace pair shape. Corresponds to ECMA-376 ST_ShapeType 'bracePair'"]
        BracePair,
        #[serde(rename = "BRACKET_PAIR")]
        #[doc = "Bracket pair shape. Corresponds to ECMA-376 ST_ShapeType 'bracketPair'"]
        BracketPair,
        #[serde(rename = "CAN")]
        #[doc = "Can shape. Corresponds to ECMA-376 ST_ShapeType 'can'"]
        Can,
        #[serde(rename = "CHEVRON")]
        #[doc = "Chevron shape. Corresponds to ECMA-376 ST_ShapeType 'chevron'"]
        Chevron,
        #[serde(rename = "CHORD")]
        #[doc = "Chord shape. Corresponds to ECMA-376 ST_ShapeType 'chord'"]
        Chord,
        #[serde(rename = "CLOUD")]
        #[doc = "Cloud shape. Corresponds to ECMA-376 ST_ShapeType 'cloud'"]
        Cloud,
        #[serde(rename = "CORNER")]
        #[doc = "Corner shape. Corresponds to ECMA-376 ST_ShapeType 'corner'"]
        Corner,
        #[serde(rename = "CUBE")]
        #[doc = "Cube shape. Corresponds to ECMA-376 ST_ShapeType 'cube'"]
        Cube,
        #[serde(rename = "CURVED_DOWN_ARROW")]
        #[doc = "Curved down arrow shape. Corresponds to ECMA-376 ST_ShapeType 'curvedDownArrow'"]
        CurvedDownArrow,
        #[serde(rename = "CURVED_LEFT_ARROW")]
        #[doc = "Curved left arrow shape. Corresponds to ECMA-376 ST_ShapeType 'curvedLeftArrow'"]
        CurvedLeftArrow,
        #[serde(rename = "CURVED_RIGHT_ARROW")]
        #[doc = "Curved right arrow shape. Corresponds to ECMA-376 ST_ShapeType 'curvedRightArrow'"]
        CurvedRightArrow,
        #[serde(rename = "CURVED_UP_ARROW")]
        #[doc = "Curved up arrow shape. Corresponds to ECMA-376 ST_ShapeType 'curvedUpArrow'"]
        CurvedUpArrow,
        #[serde(rename = "DECAGON")]
        #[doc = "Decagon shape. Corresponds to ECMA-376 ST_ShapeType 'decagon'"]
        Decagon,
        #[serde(rename = "DIAGONAL_STRIPE")]
        #[doc = "Diagonal stripe shape. Corresponds to ECMA-376 ST_ShapeType 'diagStripe'"]
        DiagonalStripe,
        #[serde(rename = "DIAMOND")]
        #[doc = "Diamond shape. Corresponds to ECMA-376 ST_ShapeType 'diamond'"]
        Diamond,
        #[serde(rename = "DODECAGON")]
        #[doc = "Dodecagon shape. Corresponds to ECMA-376 ST_ShapeType 'dodecagon'"]
        Dodecagon,
        #[serde(rename = "DONUT")]
        #[doc = "Donut shape. Corresponds to ECMA-376 ST_ShapeType 'donut'"]
        Donut,
        #[serde(rename = "DOUBLE_WAVE")]
        #[doc = "Double wave shape. Corresponds to ECMA-376 ST_ShapeType 'doubleWave'"]
        DoubleWave,
        #[serde(rename = "DOWN_ARROW")]
        #[doc = "Down arrow shape. Corresponds to ECMA-376 ST_ShapeType 'downArrow'"]
        DownArrow,
        #[serde(rename = "DOWN_ARROW_CALLOUT")]
        #[doc = "Callout down arrow shape. Corresponds to ECMA-376 ST_ShapeType 'downArrowCallout'"]
        DownArrowCallout,
        #[serde(rename = "FOLDED_CORNER")]
        #[doc = "Folded corner shape. Corresponds to ECMA-376 ST_ShapeType 'foldedCorner'"]
        FoldedCorner,
        #[serde(rename = "FRAME")]
        #[doc = "Frame shape. Corresponds to ECMA-376 ST_ShapeType 'frame'"]
        Frame,
        #[serde(rename = "HALF_FRAME")]
        #[doc = "Half frame shape. Corresponds to ECMA-376 ST_ShapeType 'halfFrame'"]
        HalfFrame,
        #[serde(rename = "HEART")]
        #[doc = "Heart shape. Corresponds to ECMA-376 ST_ShapeType 'heart'"]
        Heart,
        #[serde(rename = "HEPTAGON")]
        #[doc = "Heptagon shape. Corresponds to ECMA-376 ST_ShapeType 'heptagon'"]
        Heptagon,
        #[serde(rename = "HEXAGON")]
        #[doc = "Hexagon shape. Corresponds to ECMA-376 ST_ShapeType 'hexagon'"]
        Hexagon,
        #[serde(rename = "HOME_PLATE")]
        #[doc = "Home plate shape. Corresponds to ECMA-376 ST_ShapeType 'homePlate'"]
        HomePlate,
        #[serde(rename = "HORIZONTAL_SCROLL")]
        #[doc = "Horizontal scroll shape. Corresponds to ECMA-376 ST_ShapeType 'horizontalScroll'"]
        HorizontalScroll,
        #[serde(rename = "IRREGULAR_SEAL_1")]
        #[doc = "Irregular seal 1 shape. Corresponds to ECMA-376 ST_ShapeType 'irregularSeal1'"]
        IrregularSeal1,
        #[serde(rename = "IRREGULAR_SEAL_2")]
        #[doc = "Irregular seal 2 shape. Corresponds to ECMA-376 ST_ShapeType 'irregularSeal2'"]
        IrregularSeal2,
        #[serde(rename = "LEFT_ARROW")]
        #[doc = "Left arrow shape. Corresponds to ECMA-376 ST_ShapeType 'leftArrow'"]
        LeftArrow,
        #[serde(rename = "LEFT_ARROW_CALLOUT")]
        #[doc = "Callout left arrow shape. Corresponds to ECMA-376 ST_ShapeType 'leftArrowCallout'"]
        LeftArrowCallout,
        #[serde(rename = "LEFT_BRACE")]
        #[doc = "Left brace shape. Corresponds to ECMA-376 ST_ShapeType 'leftBrace'"]
        LeftBrace,
        #[serde(rename = "LEFT_BRACKET")]
        #[doc = "Left bracket shape. Corresponds to ECMA-376 ST_ShapeType 'leftBracket'"]
        LeftBracket,
        #[serde(rename = "LEFT_RIGHT_ARROW")]
        #[doc = "Left right arrow shape. Corresponds to ECMA-376 ST_ShapeType 'leftRightArrow'"]
        LeftRightArrow,
        #[serde(rename = "LEFT_RIGHT_ARROW_CALLOUT")]
        #[doc = "Callout left right arrow shape. Corresponds to ECMA-376 ST_ShapeType 'leftRightArrowCallout'"]
        LeftRightArrowCallout,
        #[serde(rename = "LEFT_RIGHT_UP_ARROW")]
        #[doc = "Left right up arrow shape. Corresponds to ECMA-376 ST_ShapeType 'leftRightUpArrow'"]
        LeftRightUpArrow,
        #[serde(rename = "LEFT_UP_ARROW")]
        #[doc = "Left up arrow shape. Corresponds to ECMA-376 ST_ShapeType 'leftUpArrow'"]
        LeftUpArrow,
        #[serde(rename = "LIGHTNING_BOLT")]
        #[doc = "Lightning bolt shape. Corresponds to ECMA-376 ST_ShapeType 'lightningBolt'"]
        LightningBolt,
        #[serde(rename = "MATH_DIVIDE")]
        #[doc = "Divide math shape. Corresponds to ECMA-376 ST_ShapeType 'mathDivide'"]
        MathDivide,
        #[serde(rename = "MATH_EQUAL")]
        #[doc = "Equal math shape. Corresponds to ECMA-376 ST_ShapeType 'mathEqual'"]
        MathEqual,
        #[serde(rename = "MATH_MINUS")]
        #[doc = "Minus math shape. Corresponds to ECMA-376 ST_ShapeType 'mathMinus'"]
        MathMinus,
        #[serde(rename = "MATH_MULTIPLY")]
        #[doc = "Multiply math shape. Corresponds to ECMA-376 ST_ShapeType 'mathMultiply'"]
        MathMultiply,
        #[serde(rename = "MATH_NOT_EQUAL")]
        #[doc = "Not equal math shape. Corresponds to ECMA-376 ST_ShapeType 'mathNotEqual'"]
        MathNotEqual,
        #[serde(rename = "MATH_PLUS")]
        #[doc = "Plus math shape. Corresponds to ECMA-376 ST_ShapeType 'mathPlus'"]
        MathPlus,
        #[serde(rename = "MOON")]
        #[doc = "Moon shape. Corresponds to ECMA-376 ST_ShapeType 'moon'"]
        Moon,
        #[serde(rename = "NO_SMOKING")]
        #[doc = "No smoking shape. Corresponds to ECMA-376 ST_ShapeType 'noSmoking'"]
        NoSmoking,
        #[serde(rename = "NOTCHED_RIGHT_ARROW")]
        #[doc = "Notched right arrow shape. Corresponds to ECMA-376 ST_ShapeType 'notchedRightArrow'"]
        NotchedRightArrow,
        #[serde(rename = "OCTAGON")]
        #[doc = "Octagon shape. Corresponds to ECMA-376 ST_ShapeType 'octagon'"]
        Octagon,
        #[serde(rename = "PARALLELOGRAM")]
        #[doc = "Parallelogram shape. Corresponds to ECMA-376 ST_ShapeType 'parallelogram'"]
        Parallelogram,
        #[serde(rename = "PENTAGON")]
        #[doc = "Pentagon shape. Corresponds to ECMA-376 ST_ShapeType 'pentagon'"]
        Pentagon,
        #[serde(rename = "PIE")]
        #[doc = "Pie shape. Corresponds to ECMA-376 ST_ShapeType 'pie'"]
        Pie,
        #[serde(rename = "PLAQUE")]
        #[doc = "Plaque shape. Corresponds to ECMA-376 ST_ShapeType 'plaque'"]
        Plaque,
        #[serde(rename = "PLUS")]
        #[doc = "Plus shape. Corresponds to ECMA-376 ST_ShapeType 'plus'"]
        Plus,
        #[serde(rename = "QUAD_ARROW")]
        #[doc = "Quad-arrow shape. Corresponds to ECMA-376 ST_ShapeType 'quadArrow'"]
        QuadArrow,
        #[serde(rename = "QUAD_ARROW_CALLOUT")]
        #[doc = "Callout quad-arrow shape. Corresponds to ECMA-376 ST_ShapeType 'quadArrowCallout'"]
        QuadArrowCallout,
        #[serde(rename = "RIBBON")]
        #[doc = "Ribbon shape. Corresponds to ECMA-376 ST_ShapeType 'ribbon'"]
        Ribbon,
        #[serde(rename = "RIBBON_2")]
        #[doc = "Ribbon 2 shape. Corresponds to ECMA-376 ST_ShapeType 'ribbon2'"]
        Ribbon2,
        #[serde(rename = "RIGHT_ARROW")]
        #[doc = "Right arrow shape. Corresponds to ECMA-376 ST_ShapeType 'rightArrow'"]
        RightArrow,
        #[serde(rename = "RIGHT_ARROW_CALLOUT")]
        #[doc = "Callout right arrow shape. Corresponds to ECMA-376 ST_ShapeType 'rightArrowCallout'"]
        RightArrowCallout,
        #[serde(rename = "RIGHT_BRACE")]
        #[doc = "Right brace shape. Corresponds to ECMA-376 ST_ShapeType 'rightBrace'"]
        RightBrace,
        #[serde(rename = "RIGHT_BRACKET")]
        #[doc = "Right bracket shape. Corresponds to ECMA-376 ST_ShapeType 'rightBracket'"]
        RightBracket,
        #[serde(rename = "ROUND_1_RECTANGLE")]
        #[doc = "One round corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType 'round1Rect'"]
        Round1Rectangle,
        #[serde(rename = "ROUND_2_DIAGONAL_RECTANGLE")]
        #[doc = "Two diagonal round corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType 'round2DiagRect'"]
        Round2DiagonalRectangle,
        #[serde(rename = "ROUND_2_SAME_RECTANGLE")]
        #[doc = "Two same-side round corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType 'round2SameRect'"]
        Round2SameRectangle,
        #[serde(rename = "RIGHT_TRIANGLE")]
        #[doc = "Right triangle shape. Corresponds to ECMA-376 ST_ShapeType 'rtTriangle'"]
        RightTriangle,
        #[serde(rename = "SMILEY_FACE")]
        #[doc = "Smiley face shape. Corresponds to ECMA-376 ST_ShapeType 'smileyFace'"]
        SmileyFace,
        #[serde(rename = "SNIP_1_RECTANGLE")]
        #[doc = "One snip corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType 'snip1Rect'"]
        Snip1Rectangle,
        #[serde(rename = "SNIP_2_DIAGONAL_RECTANGLE")]
        #[doc = "Two diagonal snip corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType 'snip2DiagRect'"]
        Snip2DiagonalRectangle,
        #[serde(rename = "SNIP_2_SAME_RECTANGLE")]
        #[doc = "Two same-side snip corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType 'snip2SameRect'"]
        Snip2SameRectangle,
        #[serde(rename = "SNIP_ROUND_RECTANGLE")]
        #[doc = "One snip one round corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType 'snipRoundRect'"]
        SnipRoundRectangle,
        #[serde(rename = "STAR_10")]
        #[doc = "Ten pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star10'"]
        Star10,
        #[serde(rename = "STAR_12")]
        #[doc = "Twelve pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star12'"]
        Star12,
        #[serde(rename = "STAR_16")]
        #[doc = "Sixteen pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star16'"]
        Star16,
        #[serde(rename = "STAR_24")]
        #[doc = "Twenty four pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star24'"]
        Star24,
        #[serde(rename = "STAR_32")]
        #[doc = "Thirty two pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star32'"]
        Star32,
        #[serde(rename = "STAR_4")]
        #[doc = "Four pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star4'"]
        Star4,
        #[serde(rename = "STAR_5")]
        #[doc = "Five pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star5'"]
        Star5,
        #[serde(rename = "STAR_6")]
        #[doc = "Six pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star6'"]
        Star6,
        #[serde(rename = "STAR_7")]
        #[doc = "Seven pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star7'"]
        Star7,
        #[serde(rename = "STAR_8")]
        #[doc = "Eight pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star8'"]
        Star8,
        #[serde(rename = "STRIPED_RIGHT_ARROW")]
        #[doc = "Striped right arrow shape. Corresponds to ECMA-376 ST_ShapeType 'stripedRightArrow'"]
        StripedRightArrow,
        #[serde(rename = "SUN")]
        #[doc = "Sun shape. Corresponds to ECMA-376 ST_ShapeType 'sun'"]
        Sun,
        #[serde(rename = "TRAPEZOID")]
        #[doc = "Trapezoid shape. Corresponds to ECMA-376 ST_ShapeType 'trapezoid'"]
        Trapezoid,
        #[serde(rename = "TRIANGLE")]
        #[doc = "Triangle shape. Corresponds to ECMA-376 ST_ShapeType 'triangle'"]
        Triangle,
        #[serde(rename = "UP_ARROW")]
        #[doc = "Up arrow shape. Corresponds to ECMA-376 ST_ShapeType 'upArrow'"]
        UpArrow,
        #[serde(rename = "UP_ARROW_CALLOUT")]
        #[doc = "Callout up arrow shape. Corresponds to ECMA-376 ST_ShapeType 'upArrowCallout'"]
        UpArrowCallout,
        #[serde(rename = "UP_DOWN_ARROW")]
        #[doc = "Up down arrow shape. Corresponds to ECMA-376 ST_ShapeType 'upDownArrow'"]
        UpDownArrow,
        #[serde(rename = "UTURN_ARROW")]
        #[doc = "U-turn arrow shape. Corresponds to ECMA-376 ST_ShapeType 'uturnArrow'"]
        UturnArrow,
        #[serde(rename = "VERTICAL_SCROLL")]
        #[doc = "Vertical scroll shape. Corresponds to ECMA-376 ST_ShapeType 'verticalScroll'"]
        VerticalScroll,
        #[serde(rename = "WAVE")]
        #[doc = "Wave shape. Corresponds to ECMA-376 ST_ShapeType 'wave'"]
        Wave,
        #[serde(rename = "WEDGE_ELLIPSE_CALLOUT")]
        #[doc = "Callout wedge ellipse shape. Corresponds to ECMA-376 ST_ShapeType 'wedgeEllipseCallout'"]
        WedgeEllipseCallout,
        #[serde(rename = "WEDGE_RECTANGLE_CALLOUT")]
        #[doc = "Callout wedge rectangle shape. Corresponds to ECMA-376 ST_ShapeType 'wedgeRectCallout'"]
        WedgeRectangleCallout,
        #[serde(rename = "WEDGE_ROUND_RECTANGLE_CALLOUT")]
        #[doc = "Callout wedge round rectangle shape. Corresponds to ECMA-376 ST_ShapeType 'wedgeRoundRectCallout'"]
        WedgeRoundRectangleCallout,
        #[serde(rename = "FLOW_CHART_ALTERNATE_PROCESS")]
        #[doc = "Alternate process flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartAlternateProcess'"]
        FlowChartAlternateProcess,
        #[serde(rename = "FLOW_CHART_COLLATE")]
        #[doc = "Collate flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartCollate'"]
        FlowChartCollate,
        #[serde(rename = "FLOW_CHART_CONNECTOR")]
        #[doc = "Connector flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartConnector'"]
        FlowChartConnector,
        #[serde(rename = "FLOW_CHART_DECISION")]
        #[doc = "Decision flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartDecision'"]
        FlowChartDecision,
        #[serde(rename = "FLOW_CHART_DELAY")]
        #[doc = "Delay flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartDelay'"]
        FlowChartDelay,
        #[serde(rename = "FLOW_CHART_DISPLAY")]
        #[doc = "Display flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartDisplay'"]
        FlowChartDisplay,
        #[serde(rename = "FLOW_CHART_DOCUMENT")]
        #[doc = "Document flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartDocument'"]
        FlowChartDocument,
        #[serde(rename = "FLOW_CHART_EXTRACT")]
        #[doc = "Extract flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartExtract'"]
        FlowChartExtract,
        #[serde(rename = "FLOW_CHART_INPUT_OUTPUT")]
        #[doc = "Input output flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartInputOutput'"]
        FlowChartInputOutput,
        #[serde(rename = "FLOW_CHART_INTERNAL_STORAGE")]
        #[doc = "Internal storage flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartInternalStorage'"]
        FlowChartInternalStorage,
        #[serde(rename = "FLOW_CHART_MAGNETIC_DISK")]
        #[doc = "Magnetic disk flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartMagneticDisk'"]
        FlowChartMagneticDisk,
        #[serde(rename = "FLOW_CHART_MAGNETIC_DRUM")]
        #[doc = "Magnetic drum flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartMagneticDrum'"]
        FlowChartMagneticDrum,
        #[serde(rename = "FLOW_CHART_MAGNETIC_TAPE")]
        #[doc = "Magnetic tape flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartMagneticTape'"]
        FlowChartMagneticTape,
        #[serde(rename = "FLOW_CHART_MANUAL_INPUT")]
        #[doc = "Manual input flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartManualInput'"]
        FlowChartManualInput,
        #[serde(rename = "FLOW_CHART_MANUAL_OPERATION")]
        #[doc = "Manual operation flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartManualOperation'"]
        FlowChartManualOperation,
        #[serde(rename = "FLOW_CHART_MERGE")]
        #[doc = "Merge flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartMerge'"]
        FlowChartMerge,
        #[serde(rename = "FLOW_CHART_MULTIDOCUMENT")]
        #[doc = "Multi-document flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartMultidocument'"]
        FlowChartMultidocument,
        #[serde(rename = "FLOW_CHART_OFFLINE_STORAGE")]
        #[doc = "Offline storage flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartOfflineStorage'"]
        FlowChartOfflineStorage,
        #[serde(rename = "FLOW_CHART_OFFPAGE_CONNECTOR")]
        #[doc = "Off-page connector flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartOffpageConnector'"]
        FlowChartOffpageConnector,
        #[serde(rename = "FLOW_CHART_ONLINE_STORAGE")]
        #[doc = "Online storage flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartOnlineStorage'"]
        FlowChartOnlineStorage,
        #[serde(rename = "FLOW_CHART_OR")]
        #[doc = "Or flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartOr'"]
        FlowChartOr,
        #[serde(rename = "FLOW_CHART_PREDEFINED_PROCESS")]
        #[doc = "Predefined process flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartPredefinedProcess'"]
        FlowChartPredefinedProcess,
        #[serde(rename = "FLOW_CHART_PREPARATION")]
        #[doc = "Preparation flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartPreparation'"]
        FlowChartPreparation,
        #[serde(rename = "FLOW_CHART_PROCESS")]
        #[doc = "Process flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartProcess'"]
        FlowChartProcess,
        #[serde(rename = "FLOW_CHART_PUNCHED_CARD")]
        #[doc = "Punched card flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartPunchedCard'"]
        FlowChartPunchedCard,
        #[serde(rename = "FLOW_CHART_PUNCHED_TAPE")]
        #[doc = "Punched tape flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartPunchedTape'"]
        FlowChartPunchedTape,
        #[serde(rename = "FLOW_CHART_SORT")]
        #[doc = "Sort flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartSort'"]
        FlowChartSort,
        #[serde(rename = "FLOW_CHART_SUMMING_JUNCTION")]
        #[doc = "Summing junction flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartSummingJunction'"]
        FlowChartSummingJunction,
        #[serde(rename = "FLOW_CHART_TERMINATOR")]
        #[doc = "Terminator flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartTerminator'"]
        FlowChartTerminator,
        #[serde(rename = "ARROW_EAST")]
        #[doc = "East arrow shape."]
        ArrowEast,
        #[serde(rename = "ARROW_NORTH_EAST")]
        #[doc = "Northeast arrow shape."]
        ArrowNorthEast,
        #[serde(rename = "ARROW_NORTH")]
        #[doc = "North arrow shape."]
        ArrowNorth,
        #[serde(rename = "SPEECH")]
        #[doc = "Speech shape."]
        Speech,
        #[serde(rename = "STARBURST")]
        #[doc = "Star burst shape."]
        Starburst,
        #[serde(rename = "TEARDROP")]
        #[doc = "Teardrop shape. Corresponds to ECMA-376 ST_ShapeType 'teardrop'"]
        Teardrop,
        #[serde(rename = "ELLIPSE_RIBBON")]
        #[doc = "Ellipse ribbon shape. Corresponds to ECMA-376 ST_ShapeType 'ellipseRibbon'"]
        EllipseRibbon,
        #[serde(rename = "ELLIPSE_RIBBON_2")]
        #[doc = "Ellipse ribbon 2 shape. Corresponds to ECMA-376 ST_ShapeType 'ellipseRibbon2'"]
        EllipseRibbon2,
        #[serde(rename = "CLOUD_CALLOUT")]
        #[doc = "Callout cloud shape. Corresponds to ECMA-376 ST_ShapeType 'cloudCallout'"]
        CloudCallout,
        #[serde(rename = "CUSTOM")]
        #[doc = "Custom shape."]
        Custom,
    }
    impl ::std::default::Default for ShapeShapeTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The shape background fill."]
    pub struct ShapeBackgroundFill {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "propertyState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background fill property state. Updating the fill on a shape will implicitly update this field to `RENDERED`, unless another value is specified in the same request. To have no fill on a shape, set this field to `NOT_RENDERED`. In this case, any other fill fields set in the same request will be ignored."]
        pub property_state: ::std::option::Option<ShapeBackgroundFillPropertyStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "solidFill")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Solid color fill."]
        pub solid_fill: ::std::option::Option<::std::boxed::Box<SolidFill>>,
    }
    impl ShapeBackgroundFill {
        pub fn builder() -> ShapeBackgroundFillBuilder {
            ShapeBackgroundFillBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The background fill property state. Updating the fill on a shape will implicitly update this field to `RENDERED`, unless another value is specified in the same request. To have no fill on a shape, set this field to `NOT_RENDERED`. In this case, any other fill fields set in the same request will be ignored."]
    pub enum ShapeBackgroundFillPropertyStateEnum {
        #[serde(rename = "RENDERED")]
        #[doc = "If a property's state is RENDERED, then the element has the corresponding property when rendered on a page. If the element is a placeholder shape as determined by the placeholder field, and it inherits from a placeholder shape, the corresponding field may be unset, meaning that the property value is inherited from a parent placeholder. If the element does not inherit, then the field will contain the rendered value. This is the default value."]
        Rendered,
        #[serde(rename = "NOT_RENDERED")]
        #[doc = "If a property's state is NOT_RENDERED, then the element does not have the corresponding property when rendered on a page. However, the field may still be set so it can be inherited by child shapes. To remove a property from a rendered element, set its property_state to NOT_RENDERED."]
        NotRendered,
        #[serde(rename = "INHERIT")]
        #[doc = "If a property's state is INHERIT, then the property state uses the value of corresponding `property_state` field on the parent shape. Elements that do not inherit will never have an INHERIT property state."]
        Inherit,
    }
    impl ::std::default::Default for ShapeBackgroundFillPropertyStateEnum {
        fn default() -> Self {
            Self::Rendered
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The properties of a Shape. If the shape is a placeholder shape as determined by the placeholder field, then these properties may be inherited from a parent placeholder shape. Determining the rendered value of the property depends on the corresponding property_state field value. Any text autofit settings on the shape are automatically deactivated by requests that can impact how text fits in the shape."]
    pub struct ShapeProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autofit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The autofit properties of the shape. This property is only set for shapes that allow text."]
        pub autofit: ::std::option::Option<::std::boxed::Box<Autofit>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentAlignment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The alignment of the content in the shape. If unspecified, the alignment is inherited from a parent placeholder if it exists. If the shape has no parent, the default alignment matches the alignment for new shapes created in the Slides editor."]
        pub content_alignment: ::std::option::Option<ShapePropertiesContentAlignmentEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "link")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The hyperlink destination of the shape. If unset, there is no link. Links are not inherited from parent placeholders."]
        pub link: ::std::option::Option<::std::boxed::Box<Link>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outline")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The outline of the shape. If unset, the outline is inherited from a parent placeholder if it exists. If the shape has no parent, then the default outline depends on the shape type, matching the defaults for new shapes created in the Slides editor."]
        pub outline: ::std::option::Option<::std::boxed::Box<Outline>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shadow")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The shadow properties of the shape. If unset, the shadow is inherited from a parent placeholder if it exists. If the shape has no parent, then the default shadow matches the defaults for new shapes created in the Slides editor. This property is read-only."]
        pub shadow: ::std::option::Option<::std::boxed::Box<Shadow>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shapeBackgroundFill")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background fill of the shape. If unset, the background fill is inherited from a parent placeholder if it exists. If the shape has no parent, then the default background fill depends on the shape type, matching the defaults for new shapes created in the Slides editor."]
        pub shape_background_fill: ::std::option::Option<::std::boxed::Box<ShapeBackgroundFill>>,
    }
    impl ShapeProperties {
        pub fn builder() -> ShapePropertiesBuilder {
            ShapePropertiesBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The alignment of the content in the shape. If unspecified, the alignment is inherited from a parent placeholder if it exists. If the shape has no parent, the default alignment matches the alignment for new shapes created in the Slides editor."]
    pub enum ShapePropertiesContentAlignmentEnum {
        #[serde(rename = "CONTENT_ALIGNMENT_UNSPECIFIED")]
        #[doc = "An unspecified content alignment. The content alignment is inherited from the parent if it exists."]
        ContentAlignmentUnspecified,
        #[serde(rename = "CONTENT_ALIGNMENT_UNSUPPORTED")]
        #[doc = "An unsupported content alignment."]
        ContentAlignmentUnsupported,
        #[serde(rename = "TOP")]
        #[doc = "An alignment that aligns the content to the top of the content holder. Corresponds to ECMA-376 ST_TextAnchoringType 't'."]
        Top,
        #[serde(rename = "MIDDLE")]
        #[doc = "An alignment that aligns the content to the middle of the content holder. Corresponds to ECMA-376 ST_TextAnchoringType 'ctr'."]
        Middle,
        #[serde(rename = "BOTTOM")]
        #[doc = "An alignment that aligns the content to the bottom of the content holder. Corresponds to ECMA-376 ST_TextAnchoringType 'b'."]
        Bottom,
    }
    impl ::std::default::Default for ShapePropertiesContentAlignmentEnum {
        fn default() -> Self {
            Self::ContentAlignmentUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A PageElement kind representing a linked chart embedded from Google Sheets."]
    pub struct SheetsChart {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "chartId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the specific chart in the Google Sheets spreadsheet that is embedded."]
        pub chart_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of an image of the embedded chart, with a default lifetime of 30 minutes. This URL is tagged with the account of the requester. Anyone with the URL effectively accesses the image as the original requester. Access to the image may be lost if the presentation's sharing settings change."]
        pub content_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sheetsChartProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The properties of the Sheets chart."]
        pub sheets_chart_properties:
            ::std::option::Option<::std::boxed::Box<SheetsChartProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spreadsheetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the Google Sheets spreadsheet that contains the source chart."]
        pub spreadsheet_id: ::std::option::Option<::std::string::String>,
    }
    impl SheetsChart {
        pub fn builder() -> SheetsChartBuilder {
            SheetsChartBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The properties of the SheetsChart."]
    pub struct SheetsChartProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "chartImageProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The properties of the embedded chart image."]
        pub chart_image_properties: ::std::option::Option<::std::boxed::Box<ImageProperties>>,
    }
    impl SheetsChartProperties {
        pub fn builder() -> SheetsChartPropertiesBuilder {
            SheetsChartPropertiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A width and height."]
    pub struct Size {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The height of the object."]
        pub height: ::std::option::Option<::std::boxed::Box<Dimension>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The width of the object."]
        pub width: ::std::option::Option<::std::boxed::Box<Dimension>>,
    }
    impl Size {
        pub fn builder() -> SizeBuilder {
            SizeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The properties of Page that are only relevant for pages with page_type SLIDE."]
    pub struct SlideProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layoutObjectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the layout that this slide is based on. This property is read-only."]
        pub layout_object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "masterObjectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the master that this slide is based on. This property is read-only."]
        pub master_object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notesPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The notes page that this slide is associated with. It defines the visual appearance of a notes page when printing or exporting slides with speaker notes. A notes page inherits properties from the notes master. The placeholder shape with type BODY on the notes page contains the speaker notes for this slide. The ID of this shape is identified by the speakerNotesObjectId field. The notes page is read-only except for the text content and styles of the speaker notes shape. This property is read-only."]
        pub notes_page: ::std::option::Option<::std::boxed::Box<Page>>,
    }
    impl SlideProperties {
        pub fn builder() -> SlidePropertiesBuilder {
            SlidePropertiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A solid color fill. The page or page element is filled entirely with the specified color value. If any field is unset, its value may be inherited from a parent placeholder if it exists."]
    pub struct SolidFill {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alpha")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fraction of this `color` that should be applied to the pixel. That is, the final pixel color is defined by the equation: pixel color = alpha * (color) + (1.0 - alpha) * (background color) This means that a value of 1.0 corresponds to a solid color, whereas a value of 0.0 corresponds to a completely transparent color."]
        pub alpha: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "color")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color value of the solid fill."]
        pub color: ::std::option::Option<::std::boxed::Box<OpaqueColor>>,
    }
    impl SolidFill {
        pub fn builder() -> SolidFillBuilder {
            SolidFillBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The stretched picture fill. The page or page element is filled entirely with the specified picture. The picture is stretched to fit its container."]
    pub struct StretchedPictureFill {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reading the content_url: An URL to a picture with a default lifetime of 30 minutes. This URL is tagged with the account of the requester. Anyone with the URL effectively accesses the picture as the original requester. Access to the picture may be lost if the presentation's sharing settings change. Writing the content_url: The picture is fetched once at insertion time and a copy is stored for display inside the presentation. Pictures must be less than 50MB in size, cannot exceed 25 megapixels, and must be in one of PNG, JPEG, or GIF format. The provided URL can be at most 2 kB in length."]
        pub content_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The original size of the picture fill. This field is read-only."]
        pub size: ::std::option::Option<::std::boxed::Box<Size>>,
    }
    impl StretchedPictureFill {
        pub fn builder() -> StretchedPictureFillBuilder {
            StretchedPictureFillBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A criteria that matches a specific string of text in a shape or table."]
    pub struct SubstringMatchCriteria {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchCase")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the search should respect case: - `True`: the search is case sensitive. - `False`: the search is case insensitive."]
        pub match_case: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text to search for in the shape or table."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl SubstringMatchCriteria {
        pub fn builder() -> SubstringMatchCriteriaBuilder {
            SubstringMatchCriteriaBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A PageElement kind representing a table."]
    pub struct Table {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of columns in the table."]
        pub columns: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "horizontalBorderRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Properties of horizontal cell borders. A table's horizontal cell borders are represented as a grid. The grid has one more row than the number of rows in the table and the same number of columns as the table. For example, if the table is 3 x 3, its horizontal borders will be represented as a grid with 4 rows and 3 columns."]
        pub horizontal_border_rows:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TableBorderRow>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of rows in the table."]
        pub rows: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableColumns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Properties of each column."]
        pub table_columns:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TableColumnProperties>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Properties and contents of each row. Cells that span multiple rows are contained in only one of these rows and have a row_span greater than 1."]
        pub table_rows: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TableRow>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verticalBorderRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Properties of vertical cell borders. A table's vertical cell borders are represented as a grid. The grid has the same number of rows as the table and one more column than the number of columns in the table. For example, if the table is 3 x 3, its vertical borders will be represented as a grid with 3 rows and 4 columns."]
        pub vertical_border_rows:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TableBorderRow>>>,
    }
    impl Table {
        pub fn builder() -> TableBuilder {
            TableBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The properties of each border cell."]
    pub struct TableBorderCell {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location of the border within the border table."]
        pub location: ::std::option::Option<::std::boxed::Box<TableCellLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableBorderProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The border properties."]
        pub table_border_properties:
            ::std::option::Option<::std::boxed::Box<TableBorderProperties>>,
    }
    impl TableBorderCell {
        pub fn builder() -> TableBorderCellBuilder {
            TableBorderCellBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The fill of the border."]
    pub struct TableBorderFill {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "solidFill")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Solid fill."]
        pub solid_fill: ::std::option::Option<::std::boxed::Box<SolidFill>>,
    }
    impl TableBorderFill {
        pub fn builder() -> TableBorderFillBuilder {
            TableBorderFillBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The border styling properties of the TableBorderCell."]
    pub struct TableBorderProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dashStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dash style of the border."]
        pub dash_style: ::std::option::Option<TableBorderPropertiesDashStyleEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableBorderFill")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fill of the table border."]
        pub table_border_fill: ::std::option::Option<::std::boxed::Box<TableBorderFill>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "weight")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The thickness of the border."]
        pub weight: ::std::option::Option<::std::boxed::Box<Dimension>>,
    }
    impl TableBorderProperties {
        pub fn builder() -> TableBorderPropertiesBuilder {
            TableBorderPropertiesBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The dash style of the border."]
    pub enum TableBorderPropertiesDashStyleEnum {
        #[serde(rename = "DASH_STYLE_UNSPECIFIED")]
        #[doc = "Unspecified dash style."]
        DashStyleUnspecified,
        #[serde(rename = "SOLID")]
        #[doc = "Solid line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'solid'. This is the default dash style."]
        Solid,
        #[serde(rename = "DOT")]
        #[doc = "Dotted line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dot'."]
        Dot,
        #[serde(rename = "DASH")]
        #[doc = "Dashed line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dash'."]
        Dash,
        #[serde(rename = "DASH_DOT")]
        #[doc = "Alternating dashes and dots. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dashDot'."]
        DashDot,
        #[serde(rename = "LONG_DASH")]
        #[doc = "Line with large dashes. Corresponds to ECMA-376 ST_PresetLineDashVal value 'lgDash'."]
        LongDash,
        #[serde(rename = "LONG_DASH_DOT")]
        #[doc = "Alternating large dashes and dots. Corresponds to ECMA-376 ST_PresetLineDashVal value 'lgDashDot'."]
        LongDashDot,
    }
    impl ::std::default::Default for TableBorderPropertiesDashStyleEnum {
        fn default() -> Self {
            Self::DashStyleUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contents of each border row in a table."]
    pub struct TableBorderRow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableBorderCells")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Properties of each border cell. When a border's adjacent table cells are merged, it is not included in the response."]
        pub table_border_cells:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TableBorderCell>>>,
    }
    impl TableBorderRow {
        pub fn builder() -> TableBorderRowBuilder {
            TableBorderRowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Properties and contents of each table cell."]
    pub struct TableCell {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnSpan")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Column span of the cell."]
        pub column_span: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location of the cell within the table."]
        pub location: ::std::option::Option<::std::boxed::Box<TableCellLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowSpan")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Row span of the cell."]
        pub row_span: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableCellProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The properties of the table cell."]
        pub table_cell_properties: ::std::option::Option<::std::boxed::Box<TableCellProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text content of the cell."]
        pub text: ::std::option::Option<::std::boxed::Box<TextContent>>,
    }
    impl TableCell {
        pub fn builder() -> TableCellBuilder {
            TableCellBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The table cell background fill."]
    pub struct TableCellBackgroundFill {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "propertyState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background fill property state. Updating the fill on a table cell will implicitly update this field to `RENDERED`, unless another value is specified in the same request. To have no fill on a table cell, set this field to `NOT_RENDERED`. In this case, any other fill fields set in the same request will be ignored."]
        pub property_state: ::std::option::Option<TableCellBackgroundFillPropertyStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "solidFill")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Solid color fill."]
        pub solid_fill: ::std::option::Option<::std::boxed::Box<SolidFill>>,
    }
    impl TableCellBackgroundFill {
        pub fn builder() -> TableCellBackgroundFillBuilder {
            TableCellBackgroundFillBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The background fill property state. Updating the fill on a table cell will implicitly update this field to `RENDERED`, unless another value is specified in the same request. To have no fill on a table cell, set this field to `NOT_RENDERED`. In this case, any other fill fields set in the same request will be ignored."]
    pub enum TableCellBackgroundFillPropertyStateEnum {
        #[serde(rename = "RENDERED")]
        #[doc = "If a property's state is RENDERED, then the element has the corresponding property when rendered on a page. If the element is a placeholder shape as determined by the placeholder field, and it inherits from a placeholder shape, the corresponding field may be unset, meaning that the property value is inherited from a parent placeholder. If the element does not inherit, then the field will contain the rendered value. This is the default value."]
        Rendered,
        #[serde(rename = "NOT_RENDERED")]
        #[doc = "If a property's state is NOT_RENDERED, then the element does not have the corresponding property when rendered on a page. However, the field may still be set so it can be inherited by child shapes. To remove a property from a rendered element, set its property_state to NOT_RENDERED."]
        NotRendered,
        #[serde(rename = "INHERIT")]
        #[doc = "If a property's state is INHERIT, then the property state uses the value of corresponding `property_state` field on the parent shape. Elements that do not inherit will never have an INHERIT property state."]
        Inherit,
    }
    impl ::std::default::Default for TableCellBackgroundFillPropertyStateEnum {
        fn default() -> Self {
            Self::Rendered
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A location of a single table cell within a table."]
    pub struct TableCellLocation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The 0-based column index."]
        pub column_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The 0-based row index."]
        pub row_index: ::std::option::Option<::std::primitive::i64>,
    }
    impl TableCellLocation {
        pub fn builder() -> TableCellLocationBuilder {
            TableCellLocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The properties of the TableCell."]
    pub struct TableCellProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentAlignment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The alignment of the content in the table cell. The default alignment matches the alignment for newly created table cells in the Slides editor."]
        pub content_alignment: ::std::option::Option<TableCellPropertiesContentAlignmentEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableCellBackgroundFill")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background fill of the table cell. The default fill matches the fill for newly created table cells in the Slides editor."]
        pub table_cell_background_fill:
            ::std::option::Option<::std::boxed::Box<TableCellBackgroundFill>>,
    }
    impl TableCellProperties {
        pub fn builder() -> TableCellPropertiesBuilder {
            TableCellPropertiesBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The alignment of the content in the table cell. The default alignment matches the alignment for newly created table cells in the Slides editor."]
    pub enum TableCellPropertiesContentAlignmentEnum {
        #[serde(rename = "CONTENT_ALIGNMENT_UNSPECIFIED")]
        #[doc = "An unspecified content alignment. The content alignment is inherited from the parent if it exists."]
        ContentAlignmentUnspecified,
        #[serde(rename = "CONTENT_ALIGNMENT_UNSUPPORTED")]
        #[doc = "An unsupported content alignment."]
        ContentAlignmentUnsupported,
        #[serde(rename = "TOP")]
        #[doc = "An alignment that aligns the content to the top of the content holder. Corresponds to ECMA-376 ST_TextAnchoringType 't'."]
        Top,
        #[serde(rename = "MIDDLE")]
        #[doc = "An alignment that aligns the content to the middle of the content holder. Corresponds to ECMA-376 ST_TextAnchoringType 'ctr'."]
        Middle,
        #[serde(rename = "BOTTOM")]
        #[doc = "An alignment that aligns the content to the bottom of the content holder. Corresponds to ECMA-376 ST_TextAnchoringType 'b'."]
        Bottom,
    }
    impl ::std::default::Default for TableCellPropertiesContentAlignmentEnum {
        fn default() -> Self {
            Self::ContentAlignmentUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Properties of each column in a table."]
    pub struct TableColumnProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnWidth")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Width of a column."]
        pub column_width: ::std::option::Option<::std::boxed::Box<Dimension>>,
    }
    impl TableColumnProperties {
        pub fn builder() -> TableColumnPropertiesBuilder {
            TableColumnPropertiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A table range represents a reference to a subset of a table. It's important to note that the cells specified by a table range do not necessarily form a rectangle. For example, let's say we have a 3 x 3 table where all the cells of the last row are merged together. The table looks like this: [ ] A table range with location = (0, 0), row span = 3 and column span = 2 specifies the following cells: x x [ x x x ]"]
    pub struct TableRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnSpan")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The column span of the table range."]
        pub column_span: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The starting location of the table range."]
        pub location: ::std::option::Option<::std::boxed::Box<TableCellLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowSpan")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The row span of the table range."]
        pub row_span: ::std::option::Option<::std::primitive::i64>,
    }
    impl TableRange {
        pub fn builder() -> TableRangeBuilder {
            TableRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Properties and contents of each row in a table."]
    pub struct TableRow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowHeight")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Height of a row."]
        pub row_height: ::std::option::Option<::std::boxed::Box<Dimension>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableCells")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Properties and contents of each cell. Cells that span multiple columns are represented only once with a column_span greater than 1. As a result, the length of this collection does not always match the number of columns of the entire table."]
        pub table_cells: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TableCell>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableRowProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Properties of the row."]
        pub table_row_properties: ::std::option::Option<::std::boxed::Box<TableRowProperties>>,
    }
    impl TableRow {
        pub fn builder() -> TableRowBuilder {
            TableRowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Properties of each row in a table."]
    pub struct TableRowProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minRowHeight")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum height of the row. The row will be rendered in the Slides editor at a height equal to or greater than this value in order to show all the text in the row's cell(s)."]
        pub min_row_height: ::std::option::Option<::std::boxed::Box<Dimension>>,
    }
    impl TableRowProperties {
        pub fn builder() -> TableRowPropertiesBuilder {
            TableRowPropertiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The general text content. The text must reside in a compatible shape (e.g. text box or rectangle) or a table cell in a page."]
    pub struct TextContent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lists")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bulleted lists contained in this text, keyed by list ID."]
        pub lists:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::boxed::Box<List>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textElements")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text contents broken down into its component parts, including styling information. This property is read-only."]
        pub text_elements: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TextElement>>>,
    }
    impl TextContent {
        pub fn builder() -> TextContentBuilder {
            TextContentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A TextElement describes the content of a range of indices in the text content of a Shape or TableCell."]
    pub struct TextElement {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A TextElement representing a spot in the text that is dynamically replaced with content that can change over time."]
        pub auto_text: ::std::option::Option<::std::boxed::Box<AutoText>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The zero-based end index of this text element, exclusive, in Unicode code units."]
        pub end_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paragraphMarker")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A marker representing the beginning of a new paragraph. The `start_index` and `end_index` of this TextElement represent the range of the paragraph. Other TextElements with an index range contained inside this paragraph's range are considered to be part of this paragraph. The range of indices of two separate paragraphs will never overlap."]
        pub paragraph_marker: ::std::option::Option<::std::boxed::Box<ParagraphMarker>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The zero-based start index of this text element, in Unicode code units."]
        pub start_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textRun")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A TextElement representing a run of text where all of the characters in the run have the same TextStyle. The `start_index` and `end_index` of TextRuns will always be fully contained in the index range of a single `paragraph_marker` TextElement. In other words, a TextRun will never span multiple paragraphs."]
        pub text_run: ::std::option::Option<::std::boxed::Box<TextRun>>,
    }
    impl TextElement {
        pub fn builder() -> TextElementBuilder {
            TextElementBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A TextElement kind that represents a run of text that all has the same styling."]
    pub struct TextRun {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text of this run."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "style")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The styling applied to this run."]
        pub style: ::std::option::Option<::std::boxed::Box<TextStyle>>,
    }
    impl TextRun {
        pub fn builder() -> TextRunBuilder {
            TextRunBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the styling that can be applied to a TextRun. If this text is contained in a shape with a parent placeholder, then these text styles may be inherited from the parent. Which text styles are inherited depend on the nesting level of lists: * A text run in a paragraph that is not in a list will inherit its text style from the the newline character in the paragraph at the 0 nesting level of the list inside the parent placeholder. * A text run in a paragraph that is in a list will inherit its text style from the newline character in the paragraph at its corresponding nesting level of the list inside the parent placeholder. Inherited text styles are represented as unset fields in this message. If text is contained in a shape without a parent placeholder, unsetting these fields will revert the style to a value matching the defaults in the Slides editor."]
    pub struct TextStyle {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backgroundColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background color of the text. If set, the color is either opaque or transparent, depending on if the `opaque_color` field in it is set."]
        pub background_color: ::std::option::Option<::std::boxed::Box<OptionalColor>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "baselineOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text's vertical offset from its normal position. Text with `SUPERSCRIPT` or `SUBSCRIPT` baseline offsets is automatically rendered in a smaller font size, computed based on the `font_size` field. The `font_size` itself is not affected by changes in this field."]
        pub baseline_offset: ::std::option::Option<TextStyleBaselineOffsetEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not the text is rendered as bold."]
        pub bold: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fontFamily")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The font family of the text. The font family can be any font from the Font menu in Slides or from [Google Fonts] (https://fonts.google.com/). If the font name is unrecognized, the text is rendered in `Arial`. Some fonts can affect the weight of the text. If an update request specifies values for both `font_family` and `bold`, the explicitly-set `bold` value is used."]
        pub font_family: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fontSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size of the text's font. When read, the `font_size` will specified in points."]
        pub font_size: ::std::option::Option<::std::boxed::Box<Dimension>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "foregroundColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The color of the text itself. If set, the color is either opaque or transparent, depending on if the `opaque_color` field in it is set."]
        pub foreground_color: ::std::option::Option<::std::boxed::Box<OptionalColor>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "italic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not the text is italicized."]
        pub italic: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "link")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The hyperlink destination of the text. If unset, there is no link. Links are not inherited from parent text. Changing the link in an update request causes some other changes to the text style of the range: * When setting a link, the text foreground color will be set to ThemeColorType.HYPERLINK and the text will be underlined. If these fields are modified in the same request, those values will be used instead of the link defaults. * Setting a link on a text range that overlaps with an existing link will also update the existing link to point to the new URL. * Links are not settable on newline characters. As a result, setting a link on a text range that crosses a paragraph boundary, such as `\"ABC\\n123\"`, will separate the newline character(s) into their own text runs. The link will be applied separately to the runs before and after the newline. * Removing a link will update the text style of the range to match the style of the preceding text (or the default text styles if the preceding text is another link) unless different styles are being set in the same request."]
        pub link: ::std::option::Option<::std::boxed::Box<Link>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "smallCaps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not the text is in small capital letters."]
        pub small_caps: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "strikethrough")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not the text is struck through."]
        pub strikethrough: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "underline")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not the text is underlined."]
        pub underline: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "weightedFontFamily")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The font family and rendered weight of the text. This field is an extension of `font_family` meant to support explicit font weights without breaking backwards compatibility. As such, when reading the style of a range of text, the value of `weighted_font_family#font_family` will always be equal to that of `font_family`. However, when writing, if both fields are included in the field mask (either explicitly or through the wildcard `\"*\"`), their values are reconciled as follows: * If `font_family` is set and `weighted_font_family` is not, the value of `font_family` is applied with weight `400` (\"normal\"). * If both fields are set, the value of `font_family` must match that of `weighted_font_family#font_family`. If so, the font family and weight of `weighted_font_family` is applied. Otherwise, a 400 bad request error is returned. * If `weighted_font_family` is set and `font_family` is not, the font family and weight of `weighted_font_family` is applied. * If neither field is set, the font family and weight of the text inherit from the parent. Note that these properties cannot inherit separately from each other. If an update request specifies values for both `weighted_font_family` and `bold`, the `weighted_font_family` is applied first, then `bold`. If `weighted_font_family#weight` is not set, it defaults to `400`. If `weighted_font_family` is set, then `weighted_font_family#font_family` must also be set with a non-empty value. Otherwise, a 400 bad request error is returned."]
        pub weighted_font_family: ::std::option::Option<::std::boxed::Box<WeightedFontFamily>>,
    }
    impl TextStyle {
        pub fn builder() -> TextStyleBuilder {
            TextStyleBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The text's vertical offset from its normal position. Text with `SUPERSCRIPT` or `SUBSCRIPT` baseline offsets is automatically rendered in a smaller font size, computed based on the `font_size` field. The `font_size` itself is not affected by changes in this field."]
    pub enum TextStyleBaselineOffsetEnum {
        #[serde(rename = "BASELINE_OFFSET_UNSPECIFIED")]
        #[doc = "The text's baseline offset is inherited from the parent."]
        BaselineOffsetUnspecified,
        #[serde(rename = "NONE")]
        #[doc = "The text is not vertically offset."]
        None,
        #[serde(rename = "SUPERSCRIPT")]
        #[doc = "The text is vertically offset upwards (superscript)."]
        Superscript,
        #[serde(rename = "SUBSCRIPT")]
        #[doc = "The text is vertically offset downwards (subscript)."]
        Subscript,
    }
    impl ::std::default::Default for TextStyleBaselineOffsetEnum {
        fn default() -> Self {
            Self::BaselineOffsetUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A pair mapping a theme color type to the concrete color it represents."]
    pub struct ThemeColorPair {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "color")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The concrete color corresponding to the theme color type above."]
        pub color: ::std::option::Option<::std::boxed::Box<RgbColor>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the theme color."]
        pub _type: ::std::option::Option<ThemeColorPairTypeEnum>,
    }
    impl ThemeColorPair {
        pub fn builder() -> ThemeColorPairBuilder {
            ThemeColorPairBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the theme color."]
    pub enum ThemeColorPairTypeEnum {
        #[serde(rename = "THEME_COLOR_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified theme color. This value should not be used."]
        ThemeColorTypeUnspecified,
        #[serde(rename = "DARK1")]
        #[doc = "Represents the first dark color."]
        Dark1,
        #[serde(rename = "LIGHT1")]
        #[doc = "Represents the first light color."]
        Light1,
        #[serde(rename = "DARK2")]
        #[doc = "Represents the second dark color."]
        Dark2,
        #[serde(rename = "LIGHT2")]
        #[doc = "Represents the second light color."]
        Light2,
        #[serde(rename = "ACCENT1")]
        #[doc = "Represents the first accent color."]
        Accent1,
        #[serde(rename = "ACCENT2")]
        #[doc = "Represents the second accent color."]
        Accent2,
        #[serde(rename = "ACCENT3")]
        #[doc = "Represents the third accent color."]
        Accent3,
        #[serde(rename = "ACCENT4")]
        #[doc = "Represents the fourth accent color."]
        Accent4,
        #[serde(rename = "ACCENT5")]
        #[doc = "Represents the fifth accent color."]
        Accent5,
        #[serde(rename = "ACCENT6")]
        #[doc = "Represents the sixth accent color."]
        Accent6,
        #[serde(rename = "HYPERLINK")]
        #[doc = "Represents the color to use for hyperlinks."]
        Hyperlink,
        #[serde(rename = "FOLLOWED_HYPERLINK")]
        #[doc = "Represents the color to use for visited hyperlinks."]
        FollowedHyperlink,
        #[serde(rename = "TEXT1")]
        #[doc = "Represents the first text color."]
        Text1,
        #[serde(rename = "BACKGROUND1")]
        #[doc = "Represents the first background color."]
        Background1,
        #[serde(rename = "TEXT2")]
        #[doc = "Represents the second text color."]
        Text2,
        #[serde(rename = "BACKGROUND2")]
        #[doc = "Represents the second background color."]
        Background2,
    }
    impl ::std::default::Default for ThemeColorPairTypeEnum {
        fn default() -> Self {
            Self::ThemeColorTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The thumbnail of a page."]
    pub struct Thumbnail {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The content URL of the thumbnail image. The URL to the image has a default lifetime of 30 minutes. This URL is tagged with the account of the requester. Anyone with the URL effectively accesses the image as the original requester. Access to the image may be lost if the presentation's sharing settings change. The mime type of the thumbnail image is the same as specified in the `GetPageThumbnailRequest`."]
        pub content_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The positive height in pixels of the thumbnail image."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The positive width in pixels of the thumbnail image."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl Thumbnail {
        pub fn builder() -> ThumbnailBuilder {
            ThumbnailBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Ungroups objects, such as groups."]
    pub struct UngroupObjectsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object IDs of the objects to ungroup. Only groups that are not inside other groups can be ungrouped. All the groups should be on the same page. The group itself is deleted. The visual sizes and positions of all the children are preserved."]
        pub object_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl UngroupObjectsRequest {
        pub fn builder() -> UngroupObjectsRequestBuilder {
            UngroupObjectsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Unmerges cells in a Table."]
    pub struct UnmergeTableCellsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the table."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The table range specifying which cells of the table to unmerge. All merged cells in this range will be unmerged, and cells that are already unmerged will not be affected. If the range has no merged cells, the request will do nothing. If there is text in any of the merged cells, the text will remain in the upper-left (\"head\") cell of the resulting block of unmerged cells."]
        pub table_range: ::std::option::Option<::std::boxed::Box<TableRange>>,
    }
    impl UnmergeTableCellsRequest {
        pub fn builder() -> UnmergeTableCellsRequestBuilder {
            UnmergeTableCellsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Update the properties of an Image."]
    pub struct UpdateImagePropertiesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields that should be updated. At least one field must be specified. The root `imageProperties` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field. For example to update the image outline color, set `fields` to `\"outline.outlineFill.solidFill.color\"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset."]
        pub fields: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The image properties to update."]
        pub image_properties: ::std::option::Option<::std::boxed::Box<ImageProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the image the updates are applied to."]
        pub object_id: ::std::option::Option<::std::string::String>,
    }
    impl UpdateImagePropertiesRequest {
        pub fn builder() -> UpdateImagePropertiesRequestBuilder {
            UpdateImagePropertiesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates the category of a line."]
    pub struct UpdateLineCategoryRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineCategory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The line category to update to. The exact line type is determined based on the category to update to and how it's routed to connect to other page elements."]
        pub line_category: ::std::option::Option<UpdateLineCategoryRequestLineCategoryEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the line the update is applied to. Only a line with a category indicating it is a \"connector\" can be updated. The line may be rerouted after updating its category."]
        pub object_id: ::std::option::Option<::std::string::String>,
    }
    impl UpdateLineCategoryRequest {
        pub fn builder() -> UpdateLineCategoryRequestBuilder {
            UpdateLineCategoryRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The line category to update to. The exact line type is determined based on the category to update to and how it's routed to connect to other page elements."]
    pub enum UpdateLineCategoryRequestLineCategoryEnum {
        #[serde(rename = "LINE_CATEGORY_UNSPECIFIED")]
        #[doc = "Unspecified line category."]
        LineCategoryUnspecified,
        #[serde(rename = "STRAIGHT")]
        #[doc = "Straight connectors, including straight connector 1."]
        Straight,
        #[serde(rename = "BENT")]
        #[doc = "Bent connectors, including bent connector 2 to 5."]
        Bent,
        #[serde(rename = "CURVED")]
        #[doc = "Curved connectors, including curved connector 2 to 5."]
        Curved,
    }
    impl ::std::default::Default for UpdateLineCategoryRequestLineCategoryEnum {
        fn default() -> Self {
            Self::LineCategoryUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates the properties of a Line."]
    pub struct UpdateLinePropertiesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields that should be updated. At least one field must be specified. The root `lineProperties` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field. For example to update the line solid fill color, set `fields` to `\"lineFill.solidFill.color\"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset."]
        pub fields: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The line properties to update."]
        pub line_properties: ::std::option::Option<::std::boxed::Box<LineProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the line the update is applied to."]
        pub object_id: ::std::option::Option<::std::string::String>,
    }
    impl UpdateLinePropertiesRequest {
        pub fn builder() -> UpdateLinePropertiesRequestBuilder {
            UpdateLinePropertiesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates the alt text title and/or description of a page element."]
    pub struct UpdatePageElementAltTextRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The updated alt text description of the page element. If unset the existing value will be maintained. The description is exposed to screen readers and other accessibility interfaces. Only use human readable values related to the content of the page element."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the page element the updates are applied to."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The updated alt text title of the page element. If unset the existing value will be maintained. The title is exposed to screen readers and other accessibility interfaces. Only use human readable values related to the content of the page element."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl UpdatePageElementAltTextRequest {
        pub fn builder() -> UpdatePageElementAltTextRequestBuilder {
            UpdatePageElementAltTextRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates the transform of a page element. Updating the transform of a group will change the absolute transform of the page elements in that group, which can change their visual appearance. See the documentation for PageElement.transform for more details."]
    pub struct UpdatePageElementTransformRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "applyMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The apply mode of the transform update."]
        pub apply_mode: ::std::option::Option<UpdatePageElementTransformRequestApplyModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the page element to update."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transform")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The input transform matrix used to update the page element."]
        pub transform: ::std::option::Option<::std::boxed::Box<AffineTransform>>,
    }
    impl UpdatePageElementTransformRequest {
        pub fn builder() -> UpdatePageElementTransformRequestBuilder {
            UpdatePageElementTransformRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The apply mode of the transform update."]
    pub enum UpdatePageElementTransformRequestApplyModeEnum {
        #[serde(rename = "APPLY_MODE_UNSPECIFIED")]
        #[doc = "Unspecified mode."]
        ApplyModeUnspecified,
        #[serde(rename = "RELATIVE")]
        #[doc = "Applies the new AffineTransform matrix to the existing one, and replaces the existing one with the resulting concatenation."]
        Relative,
        #[serde(rename = "ABSOLUTE")]
        #[doc = "Replaces the existing AffineTransform matrix with the new one."]
        Absolute,
    }
    impl ::std::default::Default for UpdatePageElementTransformRequestApplyModeEnum {
        fn default() -> Self {
            Self::ApplyModeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates the Z-order of page elements. Z-order is an ordering of the elements on the page from back to front. The page element in the front may cover the elements that are behind it."]
    pub struct UpdatePageElementsZOrderRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Z-order operation to apply on the page elements. When applying the operation on multiple page elements, the relative Z-orders within these page elements before the operation is maintained."]
        pub operation: ::std::option::Option<UpdatePageElementsZOrderRequestOperationEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageElementObjectIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object IDs of the page elements to update. All the page elements must be on the same page and must not be grouped."]
        pub page_element_object_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl UpdatePageElementsZOrderRequest {
        pub fn builder() -> UpdatePageElementsZOrderRequestBuilder {
            UpdatePageElementsZOrderRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The Z-order operation to apply on the page elements. When applying the operation on multiple page elements, the relative Z-orders within these page elements before the operation is maintained."]
    pub enum UpdatePageElementsZOrderRequestOperationEnum {
        #[serde(rename = "Z_ORDER_OPERATION_UNSPECIFIED")]
        #[doc = "Unspecified operation."]
        ZOrderOperationUnspecified,
        #[serde(rename = "BRING_TO_FRONT")]
        #[doc = "Brings the page elements to the front of the page."]
        BringToFront,
        #[serde(rename = "BRING_FORWARD")]
        #[doc = "Brings the page elements forward on the page by one element relative to the forwardmost one in the specified page elements."]
        BringForward,
        #[serde(rename = "SEND_BACKWARD")]
        #[doc = "Sends the page elements backward on the page by one element relative to the furthest behind one in the specified page elements."]
        SendBackward,
        #[serde(rename = "SEND_TO_BACK")]
        #[doc = "Sends the page elements to the back of the page."]
        SendToBack,
    }
    impl ::std::default::Default for UpdatePageElementsZOrderRequestOperationEnum {
        fn default() -> Self {
            Self::ZOrderOperationUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates the properties of a Page."]
    pub struct UpdatePagePropertiesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields that should be updated. At least one field must be specified. The root `pageProperties` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field. For example to update the page background solid fill color, set `fields` to `\"pageBackgroundFill.solidFill.color\"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset."]
        pub fields: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the page the update is applied to."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The page properties to update."]
        pub page_properties: ::std::option::Option<::std::boxed::Box<PageProperties>>,
    }
    impl UpdatePagePropertiesRequest {
        pub fn builder() -> UpdatePagePropertiesRequestBuilder {
            UpdatePagePropertiesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates the styling for all of the paragraphs within a Shape or Table that overlap with the given text index range."]
    pub struct UpdateParagraphStyleRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cellLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location of the cell in the table containing the paragraph(s) to style. If `object_id` refers to a table, `cell_location` must have a value. Otherwise, it must not."]
        pub cell_location: ::std::option::Option<::std::boxed::Box<TableCellLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields that should be updated. At least one field must be specified. The root `style` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field. For example, to update the paragraph alignment, set `fields` to `\"alignment\"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset."]
        pub fields: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the shape or table with the text to be styled."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "style")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The paragraph's style."]
        pub style: ::std::option::Option<::std::boxed::Box<ParagraphStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range of text containing the paragraph(s) to style."]
        pub text_range: ::std::option::Option<::std::boxed::Box<Range>>,
    }
    impl UpdateParagraphStyleRequest {
        pub fn builder() -> UpdateParagraphStyleRequestBuilder {
            UpdateParagraphStyleRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Update the properties of a Shape."]
    pub struct UpdateShapePropertiesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields that should be updated. At least one field must be specified. The root `shapeProperties` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field. For example to update the shape background solid fill color, set `fields` to `\"shapeBackgroundFill.solidFill.color\"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset."]
        pub fields: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the shape the updates are applied to."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shapeProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The shape properties to update."]
        pub shape_properties: ::std::option::Option<::std::boxed::Box<ShapeProperties>>,
    }
    impl UpdateShapePropertiesRequest {
        pub fn builder() -> UpdateShapePropertiesRequestBuilder {
            UpdateShapePropertiesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates the position of slides in the presentation."]
    pub struct UpdateSlidesPositionRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertionIndex")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index where the slides should be inserted, based on the slide arrangement before the move takes place. Must be between zero and the number of slides in the presentation, inclusive."]
        pub insertion_index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "slideObjectIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IDs of the slides in the presentation that should be moved. The slides in this list must be in existing presentation order, without duplicates."]
        pub slide_object_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl UpdateSlidesPositionRequest {
        pub fn builder() -> UpdateSlidesPositionRequestBuilder {
            UpdateSlidesPositionRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates the properties of the table borders in a Table."]
    pub struct UpdateTableBorderPropertiesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "borderPosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The border position in the table range the updates should apply to. If a border position is not specified, the updates will apply to all borders in the table range."]
        pub border_position:
            ::std::option::Option<UpdateTableBorderPropertiesRequestBorderPositionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields that should be updated. At least one field must be specified. The root `tableBorderProperties` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field. For example to update the table border solid fill color, set `fields` to `\"tableBorderFill.solidFill.color\"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset."]
        pub fields: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the table."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableBorderProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The table border properties to update."]
        pub table_border_properties:
            ::std::option::Option<::std::boxed::Box<TableBorderProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The table range representing the subset of the table to which the updates are applied. If a table range is not specified, the updates will apply to the entire table."]
        pub table_range: ::std::option::Option<::std::boxed::Box<TableRange>>,
    }
    impl UpdateTableBorderPropertiesRequest {
        pub fn builder() -> UpdateTableBorderPropertiesRequestBuilder {
            UpdateTableBorderPropertiesRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The border position in the table range the updates should apply to. If a border position is not specified, the updates will apply to all borders in the table range."]
    pub enum UpdateTableBorderPropertiesRequestBorderPositionEnum {
        #[serde(rename = "ALL")]
        #[doc = "All borders in the range."]
        All,
        #[serde(rename = "BOTTOM")]
        #[doc = "Borders at the bottom of the range."]
        Bottom,
        #[serde(rename = "INNER")]
        #[doc = "Borders on the inside of the range."]
        Inner,
        #[serde(rename = "INNER_HORIZONTAL")]
        #[doc = "Horizontal borders on the inside of the range."]
        InnerHorizontal,
        #[serde(rename = "INNER_VERTICAL")]
        #[doc = "Vertical borders on the inside of the range."]
        InnerVertical,
        #[serde(rename = "LEFT")]
        #[doc = "Borders at the left of the range."]
        Left,
        #[serde(rename = "OUTER")]
        #[doc = "Borders along the outside of the range."]
        Outer,
        #[serde(rename = "RIGHT")]
        #[doc = "Borders at the right of the range."]
        Right,
        #[serde(rename = "TOP")]
        #[doc = "Borders at the top of the range."]
        Top,
    }
    impl ::std::default::Default for UpdateTableBorderPropertiesRequestBorderPositionEnum {
        fn default() -> Self {
            Self::All
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Update the properties of a TableCell."]
    pub struct UpdateTableCellPropertiesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields that should be updated. At least one field must be specified. The root `tableCellProperties` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field. For example to update the table cell background solid fill color, set `fields` to `\"tableCellBackgroundFill.solidFill.color\"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset."]
        pub fields: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the table."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableCellProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The table cell properties to update."]
        pub table_cell_properties: ::std::option::Option<::std::boxed::Box<TableCellProperties>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The table range representing the subset of the table to which the updates are applied. If a table range is not specified, the updates will apply to the entire table."]
        pub table_range: ::std::option::Option<::std::boxed::Box<TableRange>>,
    }
    impl UpdateTableCellPropertiesRequest {
        pub fn builder() -> UpdateTableCellPropertiesRequestBuilder {
            UpdateTableCellPropertiesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates the properties of a Table column."]
    pub struct UpdateTableColumnPropertiesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnIndices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of zero-based indices specifying which columns to update. If no indices are provided, all columns in the table will be updated."]
        pub column_indices: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields that should be updated. At least one field must be specified. The root `tableColumnProperties` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field. For example to update the column width, set `fields` to `\"column_width\"`. If '\"column_width\"' is included in the field mask but the property is left unset, the column width will default to 406,400 EMU (32 points)."]
        pub fields: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the table."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableColumnProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The table column properties to update. If the value of `table_column_properties#column_width` in the request is less than 406,400 EMU (32 points), a 400 bad request error is returned."]
        pub table_column_properties:
            ::std::option::Option<::std::boxed::Box<TableColumnProperties>>,
    }
    impl UpdateTableColumnPropertiesRequest {
        pub fn builder() -> UpdateTableColumnPropertiesRequestBuilder {
            UpdateTableColumnPropertiesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Updates the properties of a Table row."]
    pub struct UpdateTableRowPropertiesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields that should be updated. At least one field must be specified. The root `tableRowProperties` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field. For example to update the minimum row height, set `fields` to `\"min_row_height\"`. If '\"min_row_height\"' is included in the field mask but the property is left unset, the minimum row height will default to 0."]
        pub fields: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the table."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowIndices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of zero-based indices specifying which rows to update. If no indices are provided, all rows in the table will be updated."]
        pub row_indices: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableRowProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The table row properties to update."]
        pub table_row_properties: ::std::option::Option<::std::boxed::Box<TableRowProperties>>,
    }
    impl UpdateTableRowPropertiesRequest {
        pub fn builder() -> UpdateTableRowPropertiesRequestBuilder {
            UpdateTableRowPropertiesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Update the styling of text in a Shape or Table."]
    pub struct UpdateTextStyleRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cellLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location of the cell in the table containing the text to style. If `object_id` refers to a table, `cell_location` must have a value. Otherwise, it must not."]
        pub cell_location: ::std::option::Option<::std::boxed::Box<TableCellLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields that should be updated. At least one field must be specified. The root `style` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field. For example, to update the text style to bold, set `fields` to `\"bold\"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset."]
        pub fields: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the shape or table with the text to be styled."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "style")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The style(s) to set on the text. If the value for a particular style matches that of the parent, that style will be set to inherit. Certain text style changes may cause other changes meant to mirror the behavior of the Slides editor. See the documentation of TextStyle for more information."]
        pub style: ::std::option::Option<::std::boxed::Box<TextStyle>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The range of text to style. The range may be extended to include adjacent newlines. If the range fully contains a paragraph belonging to a list, the paragraph's bullet is also updated with the matching text style."]
        pub text_range: ::std::option::Option<::std::boxed::Box<Range>>,
    }
    impl UpdateTextStyleRequest {
        pub fn builder() -> UpdateTextStyleRequestBuilder {
            UpdateTextStyleRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Update the properties of a Video."]
    pub struct UpdateVideoPropertiesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields that should be updated. At least one field must be specified. The root `videoProperties` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field. For example to update the video outline color, set `fields` to `\"outline.outlineFill.solidFill.color\"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset."]
        pub fields: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The object ID of the video the updates are applied to."]
        pub object_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video properties to update."]
        pub video_properties: ::std::option::Option<::std::boxed::Box<VideoProperties>>,
    }
    impl UpdateVideoPropertiesRequest {
        pub fn builder() -> UpdateVideoPropertiesRequestBuilder {
            UpdateVideoPropertiesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A PageElement kind representing a video."]
    pub struct Video {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video source's unique identifier for this video."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video source."]
        pub source: ::std::option::Option<VideoSourceEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An URL to a video. The URL is valid as long as the source video exists and sharing settings do not change."]
        pub url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The properties of the video."]
        pub video_properties: ::std::option::Option<::std::boxed::Box<VideoProperties>>,
    }
    impl Video {
        pub fn builder() -> VideoBuilder {
            VideoBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video source."]
    pub enum VideoSourceEnum {
        #[serde(rename = "SOURCE_UNSPECIFIED")]
        #[doc = "The video source is unspecified."]
        SourceUnspecified,
        #[serde(rename = "YOUTUBE")]
        #[doc = "The video source is YouTube."]
        Youtube,
        #[serde(rename = "DRIVE")]
        #[doc = "The video source is Google Drive."]
        Drive,
    }
    impl ::std::default::Default for VideoSourceEnum {
        fn default() -> Self {
            Self::SourceUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The properties of the Video."]
    pub struct VideoProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoPlay")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to enable video autoplay when the page is displayed in present mode. Defaults to false."]
        pub auto_play: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "end")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which to end playback, measured in seconds from the beginning of the video. If set, the end time should be after the start time. If not set or if you set this to a value that exceeds the video's length, the video will be played until its end."]
        pub end: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mute")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to mute the audio during video playback. Defaults to false."]
        pub mute: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outline")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The outline of the video. The default outline matches the defaults for new videos created in the Slides editor."]
        pub outline: ::std::option::Option<::std::boxed::Box<Outline>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "start")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which to start playback, measured in seconds from the beginning of the video. If set, the start time should be before the end time. If you set this to a value that exceeds the video's length in seconds, the video will be played from the last second. If not set, the video will be played from the beginning."]
        pub start: ::std::option::Option<::std::primitive::i64>,
    }
    impl VideoProperties {
        pub fn builder() -> VideoPropertiesBuilder {
            VideoPropertiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a font family and weight used to style a TextRun."]
    pub struct WeightedFontFamily {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fontFamily")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The font family of the text. The font family can be any font from the Font menu in Slides or from [Google Fonts] (https://fonts.google.com/). If the font name is unrecognized, the text is rendered in `Arial`."]
        pub font_family: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "weight")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The rendered weight of the text. This field can have any value that is a multiple of `100` between `100` and `900`, inclusive. This range corresponds to the numerical values described in the CSS 2.1 Specification, [section 15.6](https://www.w3.org/TR/CSS21/fonts.html#font-boldness), with non-numerical values disallowed. Weights greater than or equal to `700` are considered bold, and weights less than `700`are not bold. The default value is `400` (\"normal\")."]
        pub weight: ::std::option::Option<::std::primitive::i64>,
    }
    impl WeightedFontFamily {
        pub fn builder() -> WeightedFontFamilyBuilder {
            WeightedFontFamilyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A PageElement kind representing word art."]
    pub struct WordArt {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "renderedText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text rendered as word art."]
        pub rendered_text: ::std::option::Option<::std::string::String>,
    }
    impl WordArt {
        pub fn builder() -> WordArtBuilder {
            WordArtBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Provides control over how write requests are executed."]
    pub struct WriteControl {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requiredRevisionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The revision ID of the presentation required for the write request. If specified and the `required_revision_id` doesn't exactly match the presentation's current `revision_id`, the request will not be processed and will return a 400 bad request error."]
        pub required_revision_id: ::std::option::Option<::std::string::String>,
    }
    impl WriteControl {
        pub fn builder() -> WriteControlBuilder {
            WriteControlBuilder::default()
        }
    }
}
