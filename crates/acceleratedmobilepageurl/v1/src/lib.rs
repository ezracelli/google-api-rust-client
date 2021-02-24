#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "AMP URL response for a requested URL."]
pub struct AmpUrl {
    #[serde(rename = "ampUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The AMP URL pointing to the publisher's web server."]
    pub amp_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cdnAmpUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The [AMP Cache URL](/amp/cache/overview#amp-cache-url-format) pointing to the cached document in the Google AMP Cache."]
    pub cdn_amp_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "originalUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The original non-AMP URL."]
    pub original_url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "AMP URL Error resource for a requested URL that couldn't be found."]
pub struct AmpUrlError {
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The error code of an API call."]
    pub error_code: ::std::option::Option<AmpUrlErrorErrorCodeEnum>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An optional descriptive error message."]
    pub error_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "originalUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The original non-AMP URL."]
    pub original_url: ::std::option::Option<::std::string::String>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "AMP URL request for a batch of URLs."]
pub struct BatchGetAmpUrlsRequest {
    #[serde(rename = "lookupStrategy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The lookup_strategy being requested."]
    pub lookup_strategy: ::std::option::Option<BatchGetAmpUrlsRequestLookupStrategyEnum>,
    #[serde(rename = "urls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of URLs to look up for the paired AMP URLs. The URLs are case-sensitive. Up to 50 URLs per lookup (see [Usage Limits](/amp/cache/reference/limits))."]
    pub urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Batch AMP URL response."]
pub struct BatchGetAmpUrlsResponse {
    #[serde(rename = "ampUrls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For each URL in BatchAmpUrlsRequest, the URL response. The response might not be in the same order as URLs in the batch request. If BatchAmpUrlsRequest contains duplicate URLs, AmpUrl is generated only once."]
    pub amp_urls: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AmpUrl>>>,
    #[serde(rename = "urlErrors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The errors for requested URLs that have no AMP URL."]
    pub url_errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AmpUrlError>>>,
}
