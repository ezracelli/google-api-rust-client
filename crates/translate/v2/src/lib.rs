#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for language detection."]
pub struct DetectLanguageRequest {
    #[serde(rename = "q")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The input text upon which to perform language detection. Repeat this\nparameter to perform language detection on multiple text inputs."]
    pub q: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DetectionsListResponse {
    #[serde(rename = "detections")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A detections contains detection results of several text"]
    pub detections: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DetectionsResource>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DetectionsResource {
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The confidence of the detection result of this language."]
    pub confidence: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "isReliable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A boolean to indicate is the language detection result reliable."]
    pub is_reliable: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The language we detected."]
    pub language: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for discovering supported languages."]
pub struct GetSupportedLanguagesRequest {
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The language to use to return localized, human readable names of supported\nlanguages."]
    pub target: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LanguagesListResponse {
    #[serde(rename = "languages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of source/target languages supported by the translation API. If target parameter is unspecified, the list is sorted by the ASCII code point order of the language code. If target parameter is specified, the list is sorted by the collation order of the language name in the target language."]
    pub languages: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LanguagesResource>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LanguagesResource {
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Supported language code, generally consisting of its ISO 639-1\nidentifier. (E.g. 'en', 'ja'). In certain cases, BCP-47 codes including\nlanguage + region identifiers are returned (e.g. 'zh-TW' and 'zh-CH')"]
    pub language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Human readable name of the language localized to the target language."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The main translation request message for the Cloud Translation API."]
pub struct TranslateTextRequest {
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The format of the source text, in either HTML (default) or plain-text. A\nvalue of \"html\" indicates HTML and a value of \"text\" indicates plain-text."]
    pub format: ::std::option::Option<::std::string::String>,
    #[serde(rename = "model")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `model` type requested for this translation. Valid values are\nlisted in public documentation."]
    pub model: ::std::option::Option<::std::string::String>,
    #[serde(rename = "q")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The input text to translate. Repeat this parameter to perform translation\noperations on multiple text inputs."]
    pub q: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The language of the source text, set to one of the language codes listed in\nLanguage Support. If the source language is not specified, the API will\nattempt to identify the source language automatically and return it within\nthe response."]
    pub source: ::std::option::Option<::std::string::String>,
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The language to use for translation of the input text, set to one of the\nlanguage codes listed in Language Support."]
    pub target: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The main language translation response message."]
pub struct TranslationsListResponse {
    #[serde(rename = "translations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Translations contains list of translation results of given text"]
    pub translations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TranslationsResource>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TranslationsResource {
    #[serde(rename = "detectedSourceLanguage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source language of the initial request, detected automatically, if\nno source language was passed within the initial request. If the\nsource language was passed, auto-detection of the language will not\noccur and this field will be empty."]
    pub detected_source_language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "model")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `model` type used for this translation. Valid values are\nlisted in public documentation. Can be different from requested `model`.\nPresent only if specific model type was explicitly requested."]
    pub model: ::std::option::Option<::std::string::String>,
    #[serde(rename = "translatedText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Text translated into the target language."]
    pub translated_text: ::std::option::Option<::std::string::String>,
}
