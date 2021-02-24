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
    #[serde(rename = "bearer_token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OAuth bearer token."]
    pub bearer_token: ::std::option::Option<::std::string::String>,
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
    #[builder(default = "{ query_parameters_defaults :: pp () }", setter(into))]
    #[serde(rename = "pp")]
    #[serde(default = "query_parameters_defaults :: pp")]
    #[doc = "Pretty-print response."]
    pub pp: ::std::primitive::bool,
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
    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided."]
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
    pub fn pp() -> ::std::primitive::bool {
        serde_json::from_str(&"true").unwrap()
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
    pub mod detections {
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
                    #[serde(rename = "q")]
                    #[doc = "The input text upon which to perform language detection. Repeat this\nparameter to perform language detection on multiple text inputs."]
                    pub q: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod languages {
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
                    #[serde(rename = "model")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The model type for which supported languages should be returned."]
                    pub model: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "target")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The language to use to return localized, human readable names of supported\nlanguages."]
                    pub target: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod translations {
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
                    #[serde(rename = "cid")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The customization id for translate"]
                    pub cid: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "format")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The format of the source text, in either HTML (default) or plain-text. A\nvalue of \"html\" indicates HTML and a value of \"text\" indicates plain-text."]
                    pub format: ::std::option::Option<QueryParametersFormatEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "model")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The `model` type requested for this translation. Valid values are\nlisted in public documentation."]
                    pub model: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "q")]
                    #[doc = "The input text to translate. Repeat this parameter to perform translation\noperations on multiple text inputs."]
                    pub q: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "source")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The language of the source text, set to one of the language codes listed in\nLanguage Support. If the source language is not specified, the API will\nattempt to identify the source language automatically and return it within\nthe response."]
                    pub source: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "target")]
                    #[doc = "The language to use for translation of the input text, set to one of the\nlanguage codes listed in Language Support."]
                    pub target: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "The format of the source text, in either HTML (default) or plain-text. A\nvalue of \"html\" indicates HTML and a value of \"text\" indicates plain-text."]
                pub enum QueryParametersFormatEnum {
                    #[serde(rename = "html")]
                    #[doc = "Specifies the input is in HTML"]
                    Html,
                    #[serde(rename = "text")]
                    #[doc = "Specifies the input is in plain textual format"]
                    Text,
                }
                impl ::std::default::Default for QueryParametersFormatEnum {
                    fn default() -> Self {
                        Self::Html
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
    #[doc = "The request message for language detection."]
    pub struct DetectLanguageRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "q")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The input text upon which to perform language detection. Repeat this\nparameter to perform language detection on multiple text inputs."]
        pub q: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl DetectLanguageRequest {
        pub fn builder() -> DetectLanguageRequestBuilder {
            DetectLanguageRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DetectionsListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detections")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A detections contains detection results of several text"]
        pub detections:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DetectionsResource>>>,
    }
    impl DetectionsListResponse {
        pub fn builder() -> DetectionsListResponseBuilder {
            DetectionsListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DetectionsResource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The confidence of the detection result of this language."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isReliable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A boolean to indicate is the language detection result reliable."]
        pub is_reliable: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "language")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language we detected."]
        pub language: ::std::option::Option<::std::string::String>,
    }
    impl DetectionsResource {
        pub fn builder() -> DetectionsResourceBuilder {
            DetectionsResourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for discovering supported languages."]
    pub struct GetSupportedLanguagesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "target")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language to use to return localized, human readable names of supported\nlanguages."]
        pub target: ::std::option::Option<::std::string::String>,
    }
    impl GetSupportedLanguagesRequest {
        pub fn builder() -> GetSupportedLanguagesRequestBuilder {
            GetSupportedLanguagesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LanguagesListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of source/target languages supported by the translation API. If target parameter is unspecified, the list is sorted by the ASCII code point order of the language code. If target parameter is specified, the list is sorted by the collation order of the language name in the target language."]
        pub languages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LanguagesResource>>>,
    }
    impl LanguagesListResponse {
        pub fn builder() -> LanguagesListResponseBuilder {
            LanguagesListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LanguagesResource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "language")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Supported language code, generally consisting of its ISO 639-1\nidentifier. (E.g. 'en', 'ja'). In certain cases, BCP-47 codes including\nlanguage + region identifiers are returned (e.g. 'zh-TW' and 'zh-CH')"]
        pub language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human readable name of the language localized to the target language."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl LanguagesResource {
        pub fn builder() -> LanguagesResourceBuilder {
            LanguagesResourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The main translation request message for the Cloud Translation API."]
    pub struct TranslateTextRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "format")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The format of the source text, in either HTML (default) or plain-text. A\nvalue of \"html\" indicates HTML and a value of \"text\" indicates plain-text."]
        pub format: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "model")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `model` type requested for this translation. Valid values are\nlisted in public documentation."]
        pub model: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "q")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The input text to translate. Repeat this parameter to perform translation\noperations on multiple text inputs."]
        pub q: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language of the source text, set to one of the language codes listed in\nLanguage Support. If the source language is not specified, the API will\nattempt to identify the source language automatically and return it within\nthe response."]
        pub source: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "target")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language to use for translation of the input text, set to one of the\nlanguage codes listed in Language Support."]
        pub target: ::std::option::Option<::std::string::String>,
    }
    impl TranslateTextRequest {
        pub fn builder() -> TranslateTextRequestBuilder {
            TranslateTextRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The main language translation response message."]
    pub struct TranslationsListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "translations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Translations contains list of translation results of given text"]
        pub translations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TranslationsResource>>>,
    }
    impl TranslationsListResponse {
        pub fn builder() -> TranslationsListResponseBuilder {
            TranslationsListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TranslationsResource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedSourceLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source language of the initial request, detected automatically, if\nno source language was passed within the initial request. If the\nsource language was passed, auto-detection of the language will not\noccur and this field will be empty."]
        pub detected_source_language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "model")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `model` type used for this translation. Valid values are\nlisted in public documentation. Can be different from requested `model`.\nPresent only if specific model type was explicitly requested."]
        pub model: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "translatedText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Text translated into the target language."]
        pub translated_text: ::std::option::Option<::std::string::String>,
    }
    impl TranslationsResource {
        pub fn builder() -> TranslationsResourceBuilder {
            TranslationsResourceBuilder::default()
        }
    }
}
