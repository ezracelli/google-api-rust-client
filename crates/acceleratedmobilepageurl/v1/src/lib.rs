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
    #[doc = "AMP URL response for a requested URL."]
    pub struct AmpUrl {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ampUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The AMP URL pointing to the publisher's web server."]
        pub amp_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cdnAmpUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The [AMP Cache URL](/amp/cache/overview#amp-cache-url-format) pointing to the cached document in the Google AMP Cache."]
        pub cdn_amp_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The original non-AMP URL."]
        pub original_url: ::std::option::Option<::std::string::String>,
    }
    impl AmpUrl {
        pub fn builder() -> AmpUrlBuilder {
            AmpUrlBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "AMP URL Error resource for a requested URL that couldn't be found."]
    pub struct AmpUrlError {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error code of an API call."]
        pub error_code: ::std::option::Option<AmpUrlErrorErrorCodeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional descriptive error message."]
        pub error_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The original non-AMP URL."]
        pub original_url: ::std::option::Option<::std::string::String>,
    }
    impl AmpUrlError {
        pub fn builder() -> AmpUrlErrorBuilder {
            AmpUrlErrorBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The error code of an API call."]
    pub enum AmpUrlErrorErrorCodeEnum {
        #[serde(rename = "ERROR_CODE_UNSPECIFIED")]
        #[doc = "Not specified error."]
        ErrorCodeUnspecified,
        #[serde(rename = "INPUT_URL_NOT_FOUND")]
        #[doc = "Indicates the requested URL is not found in the index, possibly because it's unable to be found, not able to be accessed by Googlebot, or some other error."]
        InputUrlNotFound,
        #[serde(rename = "NO_AMP_URL")]
        #[doc = "Indicates no AMP URL has been found that corresponds to the requested URL."]
        NoAmpUrl,
        #[serde(rename = "APPLICATION_ERROR")]
        #[doc = "Indicates some kind of application error occurred at the server. Client advised to retry."]
        ApplicationError,
        #[serde(rename = "URL_IS_VALID_AMP")]
        #[doc = "DEPRECATED: Indicates the requested URL is a valid AMP URL. This is a non-error state, should not be relied upon as a sign of success or failure. It will be removed in future versions of the API."]
        UrlIsValidAmp,
        #[serde(rename = "URL_IS_INVALID_AMP")]
        #[doc = "Indicates that an AMP URL has been found that corresponds to the request URL, but it is not valid AMP HTML."]
        UrlIsInvalidAmp,
    }
    impl ::std::default::Default for AmpUrlErrorErrorCodeEnum {
        fn default() -> Self {
            Self::ErrorCodeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "AMP URL request for a batch of URLs."]
    pub struct BatchGetAmpUrlsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lookupStrategy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The lookup_strategy being requested."]
        pub lookup_strategy: ::std::option::Option<BatchGetAmpUrlsRequestLookupStrategyEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "urls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of URLs to look up for the paired AMP URLs. The URLs are case-sensitive. Up to 50 URLs per lookup (see [Usage Limits](/amp/cache/reference/limits))."]
        pub urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl BatchGetAmpUrlsRequest {
        pub fn builder() -> BatchGetAmpUrlsRequestBuilder {
            BatchGetAmpUrlsRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The lookup_strategy being requested."]
    pub enum BatchGetAmpUrlsRequestLookupStrategyEnum {
        #[serde(rename = "FETCH_LIVE_DOC")]
        #[doc = "FETCH_LIVE_DOC strategy involves live document fetch of URLs not found in the index. Any request URL not found in the index is crawled in realtime to validate if there is a corresponding AMP URL. This strategy has higher coverage but with extra latency introduced by realtime crawling. This is the default strategy. Applications using this strategy should set higher HTTP timeouts of the API calls."]
        FetchLiveDoc,
        #[serde(rename = "IN_INDEX_DOC")]
        #[doc = "IN_INDEX_DOC strategy skips fetching live documents of URL(s) not found in index. For applications which need low latency use of IN_INDEX_DOC strategy is recommended."]
        InIndexDoc,
    }
    impl ::std::default::Default for BatchGetAmpUrlsRequestLookupStrategyEnum {
        fn default() -> Self {
            Self::FetchLiveDoc
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Batch AMP URL response."]
    pub struct BatchGetAmpUrlsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ampUrls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For each URL in BatchAmpUrlsRequest, the URL response. The response might not be in the same order as URLs in the batch request. If BatchAmpUrlsRequest contains duplicate URLs, AmpUrl is generated only once."]
        pub amp_urls: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AmpUrl>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "urlErrors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The errors for requested URLs that have no AMP URL."]
        pub url_errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AmpUrlError>>>,
    }
    impl BatchGetAmpUrlsResponse {
        pub fn builder() -> BatchGetAmpUrlsResponseBuilder {
            BatchGetAmpUrlsResponseBuilder::default()
        }
    }
}
