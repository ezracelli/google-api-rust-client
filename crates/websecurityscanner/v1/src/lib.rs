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
    #[serde(rename = "iapCredential")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Authentication using Identity-Aware-Proxy (IAP)."]
    pub iap_credential: ::std::option::Option<::std::boxed::Box<IapCredential>>,
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
    #[doc = "Output only. The body of the request that triggered the vulnerability."]
    pub body: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The description of the vulnerability."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "finalUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The URL where the browser lands when the vulnerability is detected."]
    pub final_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "findingType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of the Finding. Detailed and up-to-date information on findings can be found here: https://cloud.google.com/security-command-center/docs/how-to-remediate-web-security-scanner-findings"]
    pub finding_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "form")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. An addon containing information reported for a vulnerability with an HTML form, if any."]
    pub form: ::std::option::Option<::std::boxed::Box<Form>>,
    #[serde(rename = "frameUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. If the vulnerability was originated from nested IFrame, the immediate parent IFrame is reported."]
    pub frame_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fuzzedUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The URL produced by the server-side fuzzer and used in the request that triggered the vulnerability."]
    pub fuzzed_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "httpMethod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The http method of the request that triggered the vulnerability, in uppercase."]
    pub http_method: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the Finding. The name follows the format of 'projects/{projectId}/scanConfigs/{scanConfigId}/scanruns/{scanRunId}/findings/{findingId}'. The finding IDs are generated by the system."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "outdatedLibrary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. An addon containing information about outdated libraries."]
    pub outdated_library: ::std::option::Option<::std::boxed::Box<OutdatedLibrary>>,
    #[serde(rename = "reproductionUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The URL containing human-readable payload that user can leverage to reproduce the vulnerability."]
    pub reproduction_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The severity level of the reported vulnerability."]
    pub severity: ::std::option::Option<FindingSeverityEnum>,
    #[serde(rename = "trackingId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The tracking ID uniquely identifies a vulnerability instance across multiple ScanRuns."]
    pub tracking_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "violatingResource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. An addon containing detailed information regarding any resource causing the vulnerability such as JavaScript sources, image, audio files, etc."]
    pub violating_resource: ::std::option::Option<::std::boxed::Box<ViolatingResource>>,
    #[serde(rename = "vulnerableHeaders")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. An addon containing information about vulnerable or missing HTTP headers."]
    pub vulnerable_headers: ::std::option::Option<::std::boxed::Box<VulnerableHeaders>>,
    #[serde(rename = "vulnerableParameters")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. An addon containing information about request parameters which were found to be vulnerable."]
    pub vulnerable_parameters: ::std::option::Option<::std::boxed::Box<VulnerableParameters>>,
    #[serde(rename = "xss")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. An addon containing information reported for an XSS, if any."]
    pub xss: ::std::option::Option<::std::boxed::Box<Xss>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The severity level of the reported vulnerability."]
pub enum FindingSeverityEnum {
    #[serde(rename = "SEVERITY_UNSPECIFIED")]
    #[doc = "No severity specified. The default value."]
    SeverityUnspecified,
    #[serde(rename = "CRITICAL")]
    #[doc = "Critical severity."]
    Critical,
    #[serde(rename = "HIGH")]
    #[doc = "High severity."]
    High,
    #[serde(rename = "MEDIUM")]
    #[doc = "Medium severity."]
    Medium,
    #[serde(rename = "LOW")]
    #[doc = "Low severity."]
    Low,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A FindingTypeStats resource represents stats regarding a specific FindingType of Findings under a given ScanRun."]
pub struct FindingTypeStats {
    #[serde(rename = "findingCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The count of findings belonging to this finding type."]
    pub finding_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "findingType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The finding type associated with the stats."]
    pub finding_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "! Information about a vulnerability with an HTML."]
pub struct Form {
    #[serde(rename = "actionUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "! The URI where to send the form when it's submitted."]
    pub action_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "! The names of form fields related to the vulnerability."]
    pub fields: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
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
#[doc = "Describes authentication configuration for Identity-Aware-Proxy (IAP)."]
pub struct IapCredential {
    #[serde(rename = "iapTestServiceAccountInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Authentication configuration when Web-Security-Scanner service account is added in Identity-Aware-Proxy (IAP) access policies."]
    pub iap_test_service_account_info:
        ::std::option::Option<::std::boxed::Box<IapTestServiceAccountInfo>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes authentication configuration when Web-Security-Scanner service account is added in Identity-Aware-Proxy (IAP) access policies."]
pub struct IapTestServiceAccountInfo {
    #[serde(rename = "targetAudienceClientId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Describes OAuth2 client id of resources protected by Identity-Aware-Proxy (IAP)."]
    pub target_audience_client_id: ::std::option::Option<::std::string::String>,
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
#[doc = "A ScanConfig resource contains the configurations to launch a scan."]
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
    #[serde(rename = "exportToSecurityCommandCenter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Controls export of scan configurations and results to Security Command Center."]
    pub export_to_security_command_center:
        ::std::option::Option<ScanConfigExportToSecurityCommandCenterEnum>,
    #[serde(rename = "managedScan")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the scan config is managed by Web Security Scanner, output only."]
    pub managed_scan: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "maxQps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum QPS during scanning. A valid value ranges from 5 to 20 inclusively. If the field is unspecified or its value is set 0, server will default to 15. Other values outside of [5, 20] range will be rejected with INVALID_ARGUMENT error."]
    pub max_qps: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resource name of the ScanConfig. The name follows the format of 'projects/{projectId}/scanConfigs/{scanConfigId}'. The ScanConfig IDs are generated by the system."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "riskLevel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The risk level selected for the scan"]
    pub risk_level: ::std::option::Option<ScanConfigRiskLevelEnum>,
    #[serde(rename = "schedule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The schedule of the ScanConfig."]
    pub schedule: ::std::option::Option<::std::boxed::Box<Schedule>>,
    #[serde(rename = "startingUrls")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The starting URLs from which the scanner finds site pages."]
    pub starting_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "staticIpScan")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the scan configuration has enabled static IP address scan feature. If enabled, the scanner will access applications from static IP addresses."]
    pub static_ip_scan: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "userAgent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user agent used during scanning."]
    pub user_agent: ::std::option::Option<ScanConfigUserAgentEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Controls export of scan configurations and results to Security Command Center."]
pub enum ScanConfigExportToSecurityCommandCenterEnum {
    #[serde(rename = "EXPORT_TO_SECURITY_COMMAND_CENTER_UNSPECIFIED")]
    #[doc = "Use default, which is ENABLED."]
    ExportToSecurityCommandCenterUnspecified,
    #[serde(rename = "ENABLED")]
    #[doc = "Export results of this scan to Security Command Center."]
    Enabled,
    #[serde(rename = "DISABLED")]
    #[doc = "Do not export results of this scan to Security Command Center."]
    Disabled,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The risk level selected for the scan"]
pub enum ScanConfigRiskLevelEnum {
    #[serde(rename = "RISK_LEVEL_UNSPECIFIED")]
    #[doc = "Use default, which is NORMAL."]
    RiskLevelUnspecified,
    #[serde(rename = "NORMAL")]
    #[doc = "Normal scanning (Recommended)"]
    Normal,
    #[serde(rename = "LOW")]
    #[doc = "Lower impact scanning"]
    Low,
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
#[doc = "Defines a custom error message used by CreateScanConfig and UpdateScanConfig APIs when scan configuration validation fails. It is also reported as part of a ScanRunErrorTrace message if scan validation fails due to a scan configuration error."]
pub struct ScanConfigError {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Indicates the reason code for a configuration failure."]
    pub code: ::std::option::Option<ScanConfigErrorCodeEnum>,
    #[serde(rename = "fieldName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Indicates the full name of the ScanConfig field that triggers this error, for example \"scan_config.max_qps\". This field is provided for troubleshooting purposes only and its actual value can change in the future."]
    pub field_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. Indicates the reason code for a configuration failure."]
pub enum ScanConfigErrorCodeEnum {
    #[serde(rename = "CODE_UNSPECIFIED")]
    #[doc = "There is no error."]
    CodeUnspecified,
    #[serde(rename = "OK")]
    #[doc = "There is no error."]
    Ok,
    #[serde(rename = "INTERNAL_ERROR")]
    #[doc = "Indicates an internal server error. Please DO NOT USE THIS ERROR CODE unless the root cause is truly unknown."]
    InternalError,
    #[serde(rename = "APPENGINE_API_BACKEND_ERROR")]
    #[doc = "One of the seed URLs is an App Engine URL but we cannot validate the scan settings due to an App Engine API backend error."]
    AppengineApiBackendError,
    #[serde(rename = "APPENGINE_API_NOT_ACCESSIBLE")]
    #[doc = "One of the seed URLs is an App Engine URL but we cannot access the App Engine API to validate scan settings."]
    AppengineApiNotAccessible,
    #[serde(rename = "APPENGINE_DEFAULT_HOST_MISSING")]
    #[doc = "One of the seed URLs is an App Engine URL but the Default Host of the App Engine is not set."]
    AppengineDefaultHostMissing,
    #[serde(rename = "CANNOT_USE_GOOGLE_COM_ACCOUNT")]
    #[doc = "Google corporate accounts can not be used for scanning."]
    CannotUseGoogleComAccount,
    #[serde(rename = "CANNOT_USE_OWNER_ACCOUNT")]
    #[doc = "The account of the scan creator can not be used for scanning."]
    CannotUseOwnerAccount,
    #[serde(rename = "COMPUTE_API_BACKEND_ERROR")]
    #[doc = "This scan targets Compute Engine, but we cannot validate scan settings due to a Compute Engine API backend error."]
    ComputeApiBackendError,
    #[serde(rename = "COMPUTE_API_NOT_ACCESSIBLE")]
    #[doc = "This scan targets Compute Engine, but we cannot access the Compute Engine API to validate the scan settings."]
    ComputeApiNotAccessible,
    #[serde(rename = "CUSTOM_LOGIN_URL_DOES_NOT_BELONG_TO_CURRENT_PROJECT")]
    #[doc = "The Custom Login URL does not belong to the current project."]
    CustomLoginUrlDoesNotBelongToCurrentProject,
    #[serde(rename = "CUSTOM_LOGIN_URL_MALFORMED")]
    #[doc = "The Custom Login URL is malformed (can not be parsed)."]
    CustomLoginUrlMalformed,
    #[serde(rename = "CUSTOM_LOGIN_URL_MAPPED_TO_NON_ROUTABLE_ADDRESS")]
    #[doc = "The Custom Login URL is mapped to a non-routable IP address in DNS."]
    CustomLoginUrlMappedToNonRoutableAddress,
    #[serde(rename = "CUSTOM_LOGIN_URL_MAPPED_TO_UNRESERVED_ADDRESS")]
    #[doc = "The Custom Login URL is mapped to an IP address which is not reserved for the current project."]
    CustomLoginUrlMappedToUnreservedAddress,
    #[serde(rename = "CUSTOM_LOGIN_URL_HAS_NON_ROUTABLE_IP_ADDRESS")]
    #[doc = "The Custom Login URL has a non-routable IP address."]
    CustomLoginUrlHasNonRoutableIpAddress,
    #[serde(rename = "CUSTOM_LOGIN_URL_HAS_UNRESERVED_IP_ADDRESS")]
    #[doc = "The Custom Login URL has an IP address which is not reserved for the current project."]
    CustomLoginUrlHasUnreservedIpAddress,
    #[serde(rename = "DUPLICATE_SCAN_NAME")]
    #[doc = "Another scan with the same name (case-sensitive) already exists."]
    DuplicateScanName,
    #[serde(rename = "INVALID_FIELD_VALUE")]
    #[doc = "A field is set to an invalid value."]
    InvalidFieldValue,
    #[serde(rename = "FAILED_TO_AUTHENTICATE_TO_TARGET")]
    #[doc = "There was an error trying to authenticate to the scan target."]
    FailedToAuthenticateToTarget,
    #[serde(rename = "FINDING_TYPE_UNSPECIFIED")]
    #[doc = "Finding type value is not specified in the list findings request."]
    FindingTypeUnspecified,
    #[serde(rename = "FORBIDDEN_TO_SCAN_COMPUTE")]
    #[doc = "Scan targets Compute Engine, yet current project was not whitelisted for Google Compute Engine Scanning Alpha access."]
    ForbiddenToScanCompute,
    #[serde(rename = "FORBIDDEN_UPDATE_TO_MANAGED_SCAN")]
    #[doc = "User tries to update managed scan"]
    ForbiddenUpdateToManagedScan,
    #[serde(rename = "MALFORMED_FILTER")]
    #[doc = "The supplied filter is malformed. For example, it can not be parsed, does not have a filter type in expression, or the same filter type appears more than once."]
    MalformedFilter,
    #[serde(rename = "MALFORMED_RESOURCE_NAME")]
    #[doc = "The supplied resource name is malformed (can not be parsed)."]
    MalformedResourceName,
    #[serde(rename = "PROJECT_INACTIVE")]
    #[doc = "The current project is not in an active state."]
    ProjectInactive,
    #[serde(rename = "REQUIRED_FIELD")]
    #[doc = "A required field is not set."]
    RequiredField,
    #[serde(rename = "RESOURCE_NAME_INCONSISTENT")]
    #[doc = "Project id, scanconfig id, scanrun id, or finding id are not consistent with each other in resource name."]
    ResourceNameInconsistent,
    #[serde(rename = "SCAN_ALREADY_RUNNING")]
    #[doc = "The scan being requested to start is already running."]
    ScanAlreadyRunning,
    #[serde(rename = "SCAN_NOT_RUNNING")]
    #[doc = "The scan that was requested to be stopped is not running."]
    ScanNotRunning,
    #[serde(rename = "SEED_URL_DOES_NOT_BELONG_TO_CURRENT_PROJECT")]
    #[doc = "One of the seed URLs does not belong to the current project."]
    SeedUrlDoesNotBelongToCurrentProject,
    #[serde(rename = "SEED_URL_MALFORMED")]
    #[doc = "One of the seed URLs is malformed (can not be parsed)."]
    SeedUrlMalformed,
    #[serde(rename = "SEED_URL_MAPPED_TO_NON_ROUTABLE_ADDRESS")]
    #[doc = "One of the seed URLs is mapped to a non-routable IP address in DNS."]
    SeedUrlMappedToNonRoutableAddress,
    #[serde(rename = "SEED_URL_MAPPED_TO_UNRESERVED_ADDRESS")]
    #[doc = "One of the seed URLs is mapped to an IP address which is not reserved for the current project."]
    SeedUrlMappedToUnreservedAddress,
    #[serde(rename = "SEED_URL_HAS_NON_ROUTABLE_IP_ADDRESS")]
    #[doc = "One of the seed URLs has on-routable IP address."]
    SeedUrlHasNonRoutableIpAddress,
    #[serde(rename = "SEED_URL_HAS_UNRESERVED_IP_ADDRESS")]
    #[doc = "One of the seed URLs has an IP address that is not reserved for the current project."]
    SeedUrlHasUnreservedIpAddress,
    #[serde(rename = "SERVICE_ACCOUNT_NOT_CONFIGURED")]
    #[doc = "The Web Security Scanner service account is not configured under the project."]
    ServiceAccountNotConfigured,
    #[serde(rename = "TOO_MANY_SCANS")]
    #[doc = "A project has reached the maximum number of scans."]
    TooManyScans,
    #[serde(rename = "UNABLE_TO_RESOLVE_PROJECT_INFO")]
    #[doc = "Resolving the details of the current project fails."]
    UnableToResolveProjectInfo,
    #[serde(rename = "UNSUPPORTED_BLACKLIST_PATTERN_FORMAT")]
    #[doc = "One or more blacklist patterns were in the wrong format."]
    UnsupportedBlacklistPatternFormat,
    #[serde(rename = "UNSUPPORTED_FILTER")]
    #[doc = "The supplied filter is not supported."]
    UnsupportedFilter,
    #[serde(rename = "UNSUPPORTED_FINDING_TYPE")]
    #[doc = "The supplied finding type is not supported. For example, we do not provide findings of the given finding type."]
    UnsupportedFindingType,
    #[serde(rename = "UNSUPPORTED_URL_SCHEME")]
    #[doc = "The URL scheme of one or more of the supplied URLs is not supported."]
    UnsupportedUrlScheme,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A ScanRun is a output-only resource representing an actual run of the scan. Next id: 12"]
pub struct ScanRun {
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time at which the ScanRun reached termination state - that the ScanRun is either finished or stopped by user."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "errorTrace")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. If result_state is an ERROR, this field provides the primary reason for scan's termination and more details, if such are available."]
    pub error_trace: ::std::option::Option<::std::boxed::Box<ScanRunErrorTrace>>,
    #[serde(rename = "executionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The execution state of the ScanRun."]
    pub execution_state: ::std::option::Option<ScanRunExecutionStateEnum>,
    #[serde(rename = "hasVulnerabilities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Whether the scan run has found any vulnerabilities."]
    pub has_vulnerabilities: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the ScanRun. The name follows the format of 'projects/{projectId}/scanConfigs/{scanConfigId}/scanRuns/{scanRunId}'. The ScanRun IDs are generated by the system."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "progressPercent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The percentage of total completion ranging from 0 to 100. If the scan is in queue, the value is 0. If the scan is running, the value ranges from 0 to 100. If the scan is finished, the value is 100."]
    pub progress_percent: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "resultState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The result state of the ScanRun. This field is only available after the execution state reaches \"FINISHED\"."]
    pub result_state: ::std::option::Option<ScanRunResultStateEnum>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time at which the ScanRun started."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "urlsCrawledCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The number of URLs crawled during this ScanRun. If the scan is in progress, the value represents the number of URLs crawled up to now."]
    pub urls_crawled_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "urlsTestedCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The number of URLs tested during this ScanRun. If the scan is in progress, the value represents the number of URLs tested up to now. The number of URLs tested is usually larger than the number URLS crawled because typically a crawled URL is tested with multiple test payloads."]
    pub urls_tested_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "warningTraces")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. A list of warnings, if such are encountered during this scan run."]
    pub warning_traces:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ScanRunWarningTrace>>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The execution state of the ScanRun."]
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
#[doc = "Output only. The result state of the ScanRun. This field is only available after the execution state reaches \"FINISHED\"."]
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
#[doc = "Output only. Defines an error trace message for a ScanRun."]
pub struct ScanRunErrorTrace {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Indicates the error reason code."]
    pub code: ::std::option::Option<ScanRunErrorTraceCodeEnum>,
    #[serde(rename = "mostCommonHttpErrorCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. If the scan encounters TOO_MANY_HTTP_ERRORS, this field indicates the most common HTTP error code, if such is available. For example, if this code is 404, the scan has encountered too many NOT_FOUND responses."]
    pub most_common_http_error_code: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "scanConfigError")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. If the scan encounters SCAN_CONFIG_ISSUE error, this field has the error message encountered during scan configuration validation that is performed before each scan run."]
    pub scan_config_error: ::std::option::Option<::std::boxed::Box<ScanConfigError>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. Indicates the error reason code."]
pub enum ScanRunErrorTraceCodeEnum {
    #[serde(rename = "CODE_UNSPECIFIED")]
    #[doc = "Default value is never used."]
    CodeUnspecified,
    #[serde(rename = "INTERNAL_ERROR")]
    #[doc = "Indicates that the scan run failed due to an internal server error."]
    InternalError,
    #[serde(rename = "SCAN_CONFIG_ISSUE")]
    #[doc = "Indicates a scan configuration error, usually due to outdated ScanConfig settings, such as starting_urls or the DNS configuration."]
    ScanConfigIssue,
    #[serde(rename = "AUTHENTICATION_CONFIG_ISSUE")]
    #[doc = "Indicates an authentication error, usually due to outdated ScanConfig authentication settings."]
    AuthenticationConfigIssue,
    #[serde(rename = "TIMED_OUT_WHILE_SCANNING")]
    #[doc = "Indicates a scan operation timeout, usually caused by a very large site."]
    TimedOutWhileScanning,
    #[serde(rename = "TOO_MANY_REDIRECTS")]
    #[doc = "Indicates that a scan encountered excessive redirects, either to authentication or some other page outside of the scan scope."]
    TooManyRedirects,
    #[serde(rename = "TOO_MANY_HTTP_ERRORS")]
    #[doc = "Indicates that a scan encountered numerous errors from the web site pages. When available, most_common_http_error_code field indicates the most common HTTP error code encountered during the scan."]
    TooManyHttpErrors,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Output only. Defines a warning trace message for ScanRun. Warning traces provide customers with useful information that helps make the scanning process more effective."]
pub struct ScanRunWarningTrace {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Indicates the warning code."]
    pub code: ::std::option::Option<ScanRunWarningTraceCodeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. Indicates the warning code."]
pub enum ScanRunWarningTraceCodeEnum {
    #[serde(rename = "CODE_UNSPECIFIED")]
    #[doc = "Default value is never used."]
    CodeUnspecified,
    #[serde(rename = "INSUFFICIENT_CRAWL_RESULTS")]
    #[doc = "Indicates that a scan discovered an unexpectedly low number of URLs. This is sometimes caused by complex navigation features or by using a single URL for numerous pages."]
    InsufficientCrawlResults,
    #[serde(rename = "TOO_MANY_CRAWL_RESULTS")]
    #[doc = "Indicates that a scan discovered too many URLs to test, or excessive redundant URLs."]
    TooManyCrawlResults,
    #[serde(rename = "TOO_MANY_FUZZ_TASKS")]
    #[doc = "Indicates that too many tests have been generated for the scan. Customer should try reducing the number of starting URLs, increasing the QPS rate, or narrowing down the scope of the scan using the excluded patterns."]
    TooManyFuzzTasks,
    #[serde(rename = "BLOCKED_BY_IAP")]
    #[doc = "Indicates that a scan is blocked by IAP."]
    BlockedByIap,
    #[serde(rename = "NO_STARTING_URL_FOUND_FOR_MANAGED_SCAN")]
    #[doc = "Indicates that no seeds is found for a scan"]
    NoStartingUrlFoundForManagedScan,
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
    #[serde(rename = "attackVector")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The attack vector of the payload triggering this XSS."]
    pub attack_vector: ::std::option::Option<XssAttackVectorEnum>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An error message generated by a javascript breakage."]
    pub error_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "stackTraces")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Stack traces leading to the point where the XSS occurred."]
    pub stack_traces: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "storedXssSeedingUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reproduction url for the seeding POST request of a Stored XSS."]
    pub stored_xss_seeding_url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The attack vector of the payload triggering this XSS."]
pub enum XssAttackVectorEnum {
    #[serde(rename = "ATTACK_VECTOR_UNSPECIFIED")]
    #[doc = "Unknown attack vector."]
    AttackVectorUnspecified,
    #[serde(rename = "LOCAL_STORAGE")]
    #[doc = "The attack comes from fuzzing the browser's localStorage."]
    LocalStorage,
    #[serde(rename = "SESSION_STORAGE")]
    #[doc = "The attack comes from fuzzing the browser's sessionStorage."]
    SessionStorage,
    #[serde(rename = "WINDOW_NAME")]
    #[doc = "The attack comes from fuzzing the window's name property."]
    WindowName,
    #[serde(rename = "REFERRER")]
    #[doc = "The attack comes from fuzzing the referrer property."]
    Referrer,
    #[serde(rename = "FORM_INPUT")]
    #[doc = "The attack comes from fuzzing an input element."]
    FormInput,
    #[serde(rename = "COOKIE")]
    #[doc = "The attack comes from fuzzing the browser's cookies."]
    Cookie,
    #[serde(rename = "POST_MESSAGE")]
    #[doc = "The attack comes from hijacking the post messaging mechanism."]
    PostMessage,
    #[serde(rename = "GET_PARAMETERS")]
    #[doc = "The attack comes from fuzzing parameters in the url."]
    GetParameters,
    #[serde(rename = "URL_FRAGMENT")]
    #[doc = "The attack comes from fuzzing the fragment in the url."]
    UrlFragment,
    #[serde(rename = "HTML_COMMENT")]
    #[doc = "The attack comes from fuzzing the HTML comments."]
    HtmlComment,
    #[serde(rename = "POST_PARAMETERS")]
    #[doc = "The attack comes from fuzzing the POST parameters."]
    PostParameters,
    #[serde(rename = "PROTOCOL")]
    #[doc = "The attack comes from fuzzing the protocol."]
    Protocol,
    #[serde(rename = "STORED_XSS")]
    #[doc = "The attack comes from the server side and is stored."]
    StoredXss,
    #[serde(rename = "SAME_ORIGIN")]
    #[doc = "The attack is a Same-Origin Method Execution attack via a GET parameter."]
    SameOrigin,
    #[serde(rename = "USER_CONTROLLABLE_URL")]
    #[doc = "The attack payload is received from a third-party host via a URL that is user-controllable"]
    UserControllableUrl,
}
