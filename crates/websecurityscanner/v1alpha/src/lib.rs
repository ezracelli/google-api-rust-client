#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Scan authentication configuration."]
pub struct Authentication {
    #[serde(rename = "customAccount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Authentication using a custom account."]
    pub custom_account: ::std::option::Option<::std::boxed::Box<CustomAccount>>,
    #[serde(rename = "googleAccount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Authentication using a Google account."]
    pub google_account: ::std::option::Option<::std::boxed::Box<GoogleAccount>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A CrawledUrl resource represents a URL that was crawled during a ScanRun. Web Security Scanner Service crawls the web applications, following all links within the scope of sites, to find the URLs to test against."]
pub struct CrawledUrl {
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The body of the request that was used to visit the URL."]
    pub body: ::std::option::Option<::std::string::String>,
    #[serde(rename = "httpMethod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The http method of the request that was used to visit the URL, in uppercase."]
    pub http_method: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The URL that was crawled."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes authentication configuration that uses a custom account."]
pub struct CustomAccount {
    #[serde(rename = "loginUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The login form URL of the website."]
    pub login_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Input only. The password of the custom account. The credential is stored encrypted and not returned in any response nor included in audit logs."]
    pub password: ::std::option::Option<::std::string::String>,
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The user name of the custom account."]
    pub username: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Finding resource represents a vulnerability instance identified during a ScanRun."]
pub struct Finding {
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The body of the request that triggered the vulnerability."]
    pub body: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The description of the vulnerability."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "finalUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL where the browser lands when the vulnerability is detected."]
    pub final_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "findingType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the Finding."]
    pub finding_type: ::std::option::Option<FindingFindingTypeEnum>,
    #[serde(rename = "frameUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the vulnerability was originated from nested IFrame, the immediate parent IFrame is reported."]
    pub frame_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fuzzedUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL produced by the server-side fuzzer and used in the request that triggered the vulnerability."]
    pub fuzzed_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "httpMethod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The http method of the request that triggered the vulnerability, in uppercase."]
    pub http_method: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the Finding. The name follows the format of 'projects/{projectId}/scanConfigs/{scanConfigId}/scanruns/{scanRunId}/findings/{findingId}'. The finding IDs are generated by the system."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "outdatedLibrary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An addon containing information about outdated libraries."]
    pub outdated_library: ::std::option::Option<::std::boxed::Box<OutdatedLibrary>>,
    #[serde(rename = "reproductionUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL containing human-readable payload that user can leverage to reproduce the vulnerability."]
    pub reproduction_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "trackingId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The tracking ID uniquely identifies a vulnerability instance across multiple ScanRuns."]
    pub tracking_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "violatingResource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An addon containing detailed information regarding any resource causing the vulnerability such as JavaScript sources, image, audio files, etc."]
    pub violating_resource: ::std::option::Option<::std::boxed::Box<ViolatingResource>>,
    #[serde(rename = "vulnerableHeaders")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An addon containing information about vulnerable or missing HTTP headers."]
    pub vulnerable_headers: ::std::option::Option<::std::boxed::Box<VulnerableHeaders>>,
    #[serde(rename = "vulnerableParameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An addon containing information about request parameters which were found to be vulnerable."]
    pub vulnerable_parameters: ::std::option::Option<::std::boxed::Box<VulnerableParameters>>,
    #[serde(rename = "xss")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An addon containing information reported for an XSS, if any."]
    pub xss: ::std::option::Option<::std::boxed::Box<Xss>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The type of the Finding."]
pub enum FindingFindingTypeEnum {
    #[serde(rename = "FINDING_TYPE_UNSPECIFIED")]
    #[doc = "The invalid finding type."]
    FindingTypeUnspecified,
    #[serde(rename = "MIXED_CONTENT")]
    #[doc = "A page that was served over HTTPS also resources over HTTP. A man-in-the-middle attacker could tamper with the HTTP resource and gain full access to the website that loads the resource or to monitor the actions taken by the user."]
    MixedContent,
    #[serde(rename = "OUTDATED_LIBRARY")]
    #[doc = "The version of an included library is known to contain a security issue. The scanner checks the version of library in use against a known list of vulnerable libraries. False positives are possible if the version detection fails or if the library has been manually patched."]
    OutdatedLibrary,
    #[serde(rename = "ROSETTA_FLASH")]
    #[doc = "This type of vulnerability occurs when the value of a request parameter is reflected at the beginning of the response, for example, in requests using JSONP. Under certain circumstances, an attacker may be able to supply an alphanumeric-only Flash file in the vulnerable parameter causing the browser to execute the Flash file as if it originated on the vulnerable server."]
    RosettaFlash,
    #[serde(rename = "XSS_CALLBACK")]
    #[doc = "A cross-site scripting (XSS) bug is found via JavaScript callback. For detailed explanations on XSS, see https://www.google.com/about/appsecurity/learning/xss/."]
    XssCallback,
    #[serde(rename = "XSS_ERROR")]
    #[doc = "A potential cross-site scripting (XSS) bug due to JavaScript breakage. In some circumstances, the application under test might modify the test string before it is parsed by the browser. When the browser attempts to runs this modified test string, it will likely break and throw a JavaScript execution error, thus an injection issue is occurring. However, it may not be exploitable. Manual verification is needed to see if the test string modifications can be evaded and confirm that the issue is in fact an XSS vulnerability. For detailed explanations on XSS, see https://www.google.com/about/appsecurity/learning/xss/."]
    XssError,
    #[serde(rename = "CLEAR_TEXT_PASSWORD")]
    #[doc = "An application appears to be transmitting a password field in clear text. An attacker can eavesdrop network traffic and sniff the password field."]
    ClearTextPassword,
    #[serde(rename = "INVALID_CONTENT_TYPE")]
    #[doc = "An application returns sensitive content with an invalid content type, or without an 'X-Content-Type-Options: nosniff' header."]
    InvalidContentType,
    #[serde(rename = "XSS_ANGULAR_CALLBACK")]
    #[doc = "A cross-site scripting (XSS) vulnerability in AngularJS module that occurs when a user-provided string is interpolated by Angular."]
    XssAngularCallback,
    #[serde(rename = "INVALID_HEADER")]
    #[doc = "A malformed or invalid valued header."]
    InvalidHeader,
    #[serde(rename = "MISSPELLED_SECURITY_HEADER_NAME")]
    #[doc = "Misspelled security header name."]
    MisspelledSecurityHeaderName,
    #[serde(rename = "MISMATCHING_SECURITY_HEADER_VALUES")]
    #[doc = "Mismatching values in a duplicate security header."]
    MismatchingSecurityHeaderValues,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A FindingTypeStats resource represents stats regarding a specific FindingType of Findings under a given ScanRun."]
pub struct FindingTypeStats {
    #[serde(rename = "findingCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The count of findings belonging to this finding type."]
    pub finding_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "findingType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The finding type associated with the stats."]
    pub finding_type: ::std::option::Option<FindingTypeStatsFindingTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The finding type associated with the stats."]
pub enum FindingTypeStatsFindingTypeEnum {
    #[serde(rename = "FINDING_TYPE_UNSPECIFIED")]
    #[doc = "The invalid finding type."]
    FindingTypeUnspecified,
    #[serde(rename = "MIXED_CONTENT")]
    #[doc = "A page that was served over HTTPS also resources over HTTP. A man-in-the-middle attacker could tamper with the HTTP resource and gain full access to the website that loads the resource or to monitor the actions taken by the user."]
    MixedContent,
    #[serde(rename = "OUTDATED_LIBRARY")]
    #[doc = "The version of an included library is known to contain a security issue. The scanner checks the version of library in use against a known list of vulnerable libraries. False positives are possible if the version detection fails or if the library has been manually patched."]
    OutdatedLibrary,
    #[serde(rename = "ROSETTA_FLASH")]
    #[doc = "This type of vulnerability occurs when the value of a request parameter is reflected at the beginning of the response, for example, in requests using JSONP. Under certain circumstances, an attacker may be able to supply an alphanumeric-only Flash file in the vulnerable parameter causing the browser to execute the Flash file as if it originated on the vulnerable server."]
    RosettaFlash,
    #[serde(rename = "XSS_CALLBACK")]
    #[doc = "A cross-site scripting (XSS) bug is found via JavaScript callback. For detailed explanations on XSS, see https://www.google.com/about/appsecurity/learning/xss/."]
    XssCallback,
    #[serde(rename = "XSS_ERROR")]
    #[doc = "A potential cross-site scripting (XSS) bug due to JavaScript breakage. In some circumstances, the application under test might modify the test string before it is parsed by the browser. When the browser attempts to runs this modified test string, it will likely break and throw a JavaScript execution error, thus an injection issue is occurring. However, it may not be exploitable. Manual verification is needed to see if the test string modifications can be evaded and confirm that the issue is in fact an XSS vulnerability. For detailed explanations on XSS, see https://www.google.com/about/appsecurity/learning/xss/."]
    XssError,
    #[serde(rename = "CLEAR_TEXT_PASSWORD")]
    #[doc = "An application appears to be transmitting a password field in clear text. An attacker can eavesdrop network traffic and sniff the password field."]
    ClearTextPassword,
    #[serde(rename = "INVALID_CONTENT_TYPE")]
    #[doc = "An application returns sensitive content with an invalid content type, or without an 'X-Content-Type-Options: nosniff' header."]
    InvalidContentType,
    #[serde(rename = "XSS_ANGULAR_CALLBACK")]
    #[doc = "A cross-site scripting (XSS) vulnerability in AngularJS module that occurs when a user-provided string is interpolated by Angular."]
    XssAngularCallback,
    #[serde(rename = "INVALID_HEADER")]
    #[doc = "A malformed or invalid valued header."]
    InvalidHeader,
    #[serde(rename = "MISSPELLED_SECURITY_HEADER_NAME")]
    #[doc = "Misspelled security header name."]
    MisspelledSecurityHeaderName,
    #[serde(rename = "MISMATCHING_SECURITY_HEADER_VALUES")]
    #[doc = "Mismatching values in a duplicate security header."]
    MismatchingSecurityHeaderValues,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes authentication configuration that uses a Google account."]
pub struct GoogleAccount {
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Input only. The password of the Google account. The credential is stored encrypted and not returned in any response nor included in audit logs."]
    pub password: ::std::option::Option<::std::string::String>,
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The user name of the Google account."]
    pub username: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes a HTTP Header."]
pub struct Header {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Header name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Header value."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the `ListCrawledUrls` method."]
pub struct ListCrawledUrlsResponse {
    #[serde(rename = "crawledUrls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of CrawledUrls returned."]
    pub crawled_urls: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CrawledUrl>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the `ListFindingTypeStats` method."]
pub struct ListFindingTypeStatsResponse {
    #[serde(rename = "findingTypeStats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of FindingTypeStats returned."]
    pub finding_type_stats:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FindingTypeStats>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the `ListFindings` method."]
pub struct ListFindingsResponse {
    #[serde(rename = "findings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of Findings returned."]
    pub findings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Finding>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the `ListScanConfigs` method."]
pub struct ListScanConfigsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scanConfigs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of ScanConfigs returned."]
    pub scan_configs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ScanConfig>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for the `ListScanRuns` method."]
pub struct ListScanRunsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scanRuns")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of ScanRuns returned."]
    pub scan_runs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ScanRun>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information reported for an outdated library."]
pub struct OutdatedLibrary {
    #[serde(rename = "learnMoreUrls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URLs to learn more information about the vulnerabilities in the library."]
    pub learn_more_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "libraryName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the outdated library."]
    pub library_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version number."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A ScanConfig resource contains the configurations to launch a scan. next id: 12"]
pub struct ScanConfig {
    #[serde(rename = "authentication")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The authentication configuration. If specified, service will use the authentication configuration during scanning."]
    pub authentication: ::std::option::Option<::std::boxed::Box<Authentication>>,
    #[serde(rename = "blacklistPatterns")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The excluded URL patterns as described in https://cloud.google.com/security-command-center/docs/how-to-use-web-security-scanner#excluding_urls"]
    pub blacklist_patterns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The user provided display name of the ScanConfig."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "latestRun")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Latest ScanRun if available."]
    pub latest_run: ::std::option::Option<::std::boxed::Box<ScanRun>>,
    #[serde(rename = "maxQps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum QPS during scanning. A valid value ranges from 5 to 20 inclusively. If the field is unspecified or its value is set 0, server will default to 15. Other values outside of [5, 20] range will be rejected with INVALID_ARGUMENT error."]
    pub max_qps: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the ScanConfig. The name follows the format of 'projects/{projectId}/scanConfigs/{scanConfigId}'. The ScanConfig IDs are generated by the system."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "schedule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The schedule of the ScanConfig."]
    pub schedule: ::std::option::Option<::std::boxed::Box<Schedule>>,
    #[serde(rename = "startingUrls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The starting URLs from which the scanner finds site pages."]
    pub starting_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "targetPlatforms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set of Google Cloud platforms targeted by the scan. If empty, APP_ENGINE will be used as a default."]
    pub target_platforms: ::std::option::Option<::std::vec::Vec<ScanConfigTargetPlatformsEnum>>,
    #[serde(rename = "userAgent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user agent used during scanning."]
    pub user_agent: ::std::option::Option<ScanConfigUserAgentEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum ScanConfigTargetPlatformsEnum {
    #[serde(rename = "TARGET_PLATFORM_UNSPECIFIED")]
    #[doc = "The target platform is unknown. Requests with this enum value will be rejected with INVALID_ARGUMENT error."]
    TargetPlatformUnspecified,
    #[serde(rename = "APP_ENGINE")]
    #[doc = "Google App Engine service."]
    AppEngine,
    #[serde(rename = "COMPUTE")]
    #[doc = "Google Compute Engine service."]
    Compute,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The user agent used during scanning."]
pub enum ScanConfigUserAgentEnum {
    #[serde(rename = "USER_AGENT_UNSPECIFIED")]
    #[doc = "The user agent is unknown. Service will default to CHROME_LINUX."]
    UserAgentUnspecified,
    #[serde(rename = "CHROME_LINUX")]
    #[doc = "Chrome on Linux. This is the service default if unspecified."]
    ChromeLinux,
    #[serde(rename = "CHROME_ANDROID")]
    #[doc = "Chrome on Android."]
    ChromeAndroid,
    #[serde(rename = "SAFARI_IPHONE")]
    #[doc = "Safari on IPhone."]
    SafariIphone,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A ScanRun is a output-only resource representing an actual run of the scan."]
pub struct ScanRun {
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the ScanRun reached termination state - that the ScanRun is either finished or stopped by user."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "executionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The execution state of the ScanRun."]
    pub execution_state: ::std::option::Option<ScanRunExecutionStateEnum>,
    #[serde(rename = "hasVulnerabilities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the scan run has found any vulnerabilities."]
    pub has_vulnerabilities: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the ScanRun. The name follows the format of 'projects/{projectId}/scanConfigs/{scanConfigId}/scanRuns/{scanRunId}'. The ScanRun IDs are generated by the system."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progressPercent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The percentage of total completion ranging from 0 to 100. If the scan is in queue, the value is 0. If the scan is running, the value ranges from 0 to 100. If the scan is finished, the value is 100."]
    pub progress_percent: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "resultState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result state of the ScanRun. This field is only available after the execution state reaches \"FINISHED\"."]
    pub result_state: ::std::option::Option<ScanRunResultStateEnum>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the ScanRun started."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "urlsCrawledCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of URLs crawled during this ScanRun. If the scan is in progress, the value represents the number of URLs crawled up to now."]
    pub urls_crawled_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "urlsTestedCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of URLs tested during this ScanRun. If the scan is in progress, the value represents the number of URLs tested up to now. The number of URLs tested is usually larger than the number URLS crawled because typically a crawled URL is tested with multiple test payloads."]
    pub urls_tested_count: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The execution state of the ScanRun."]
pub enum ScanRunExecutionStateEnum {
    #[serde(rename = "EXECUTION_STATE_UNSPECIFIED")]
    #[doc = "Represents an invalid state caused by internal server error. This value should never be returned."]
    ExecutionStateUnspecified,
    #[serde(rename = "QUEUED")]
    #[doc = "The scan is waiting in the queue."]
    Queued,
    #[serde(rename = "SCANNING")]
    #[doc = "The scan is in progress."]
    Scanning,
    #[serde(rename = "FINISHED")]
    #[doc = "The scan is either finished or stopped by user."]
    Finished,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The result state of the ScanRun. This field is only available after the execution state reaches \"FINISHED\"."]
pub enum ScanRunResultStateEnum {
    #[serde(rename = "RESULT_STATE_UNSPECIFIED")]
    #[doc = "Default value. This value is returned when the ScanRun is not yet finished."]
    ResultStateUnspecified,
    #[serde(rename = "SUCCESS")]
    #[doc = "The scan finished without errors."]
    Success,
    #[serde(rename = "ERROR")]
    #[doc = "The scan finished with errors."]
    Error,
    #[serde(rename = "KILLED")]
    #[doc = "The scan was terminated by user."]
    Killed,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Scan schedule configuration."]
pub struct Schedule {
    #[serde(rename = "intervalDurationDays")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The duration of time between executions in days."]
    pub interval_duration_days: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "scheduleTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A timestamp indicates when the next run will be scheduled. The value is refreshed by the server after each run. If unspecified, it will default to current server time, which means the scan will be scheduled to start immediately."]
    pub schedule_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the `StartScanRun` method."]
pub struct StartScanRunRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for the `StopScanRun` method."]
pub struct StopScanRunRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information regarding any resource causing the vulnerability such as JavaScript sources, image, audio files, etc."]
pub struct ViolatingResource {
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The MIME type of this resource."]
    pub content_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL of this violating resource."]
    pub resource_url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about vulnerable or missing HTTP Headers."]
pub struct VulnerableHeaders {
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of vulnerable headers."]
    pub headers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Header>>>,
    #[serde(rename = "missingHeaders")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of missing headers."]
    pub missing_headers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Header>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about vulnerable request parameters."]
pub struct VulnerableParameters {
    #[serde(rename = "parameterNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The vulnerable parameter names."]
    pub parameter_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information reported for an XSS."]
pub struct Xss {
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An error message generated by a javascript breakage."]
    pub error_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "stackTraces")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Stack traces leading to the point where the XSS occurred."]
    pub stack_traces: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
