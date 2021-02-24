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
    pub mod encoded_full_hashes {
        pub mod methods {
            pub mod get {
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
                    #[serde(rename = "clientId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A client ID that (hopefully) uniquely identifies the client implementation of the Safe Browsing API."]
                    pub client_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientVersion")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The version of the client implementation."]
                    pub client_version: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod encoded_updates {
        pub mod methods {
            pub mod get {
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
                    #[serde(rename = "clientId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A client ID that uniquely identifies the client implementation of the Safe Browsing API."]
                    pub client_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "clientVersion")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The version of the client implementation."]
                    pub client_version: ::std::option::Option<::std::string::String>,
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
pub mod schemas {
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
    #[doc = "The expected state of a client's local database."]
    pub struct GoogleSecuritySafebrowsingV4Checksum {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sha256")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The SHA256 hash of the client state; that is, of the sorted list of all hashes present in the database."]
        pub sha256: ::std::option::Option<::std::string::String>,
    }
    impl GoogleSecuritySafebrowsingV4Checksum {
        pub fn builder() -> GoogleSecuritySafebrowsingV4ChecksumBuilder {
            GoogleSecuritySafebrowsingV4ChecksumBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The client metadata associated with Safe Browsing API requests."]
    pub struct GoogleSecuritySafebrowsingV4ClientInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A client ID that (hopefully) uniquely identifies the client implementation of the Safe Browsing API."]
        pub client_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version of the client implementation."]
        pub client_version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleSecuritySafebrowsingV4ClientInfo {
        pub fn builder() -> GoogleSecuritySafebrowsingV4ClientInfoBuilder {
            GoogleSecuritySafebrowsingV4ClientInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a Safe Browsing API update request. Clients can request updates for multiple lists in a single request. The server may not respond to all requests, if the server has no updates for that list. NOTE: Field index 2 is unused. NEXT: 5"]
    pub struct GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "client")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The client metadata."]
        pub client:
            ::std::option::Option<::std::boxed::Box<GoogleSecuritySafebrowsingV4ClientInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "listUpdateRequests")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The requested threat list updates."]
        pub list_update_requests: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<
                    GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequest,
                >,
            >,
        >,
    }
    impl GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequest {
        pub fn builder() -> GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestBuilder {
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single list update request."]
    pub struct GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequest { # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "constraints")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "The constraints associated with this request."] pub constraints : :: std :: option :: Option < :: std :: boxed :: Box < GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraints > > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "platformType")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "The type of platform at risk by entries present in the list."] pub platform_type : :: std :: option :: Option < GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "state")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "The current state of the client for the requested list (the encrypted client state that was received from the last successful list update)."] pub state : :: std :: option :: Option < :: std :: string :: String > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "threatEntryType")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "The types of entries present in the list."] pub threat_entry_type : :: std :: option :: Option < GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryTypeEnum > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "threatType")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "The type of threat posed by entries present in the list."] pub threat_type : :: std :: option :: Option < GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum > }
    impl GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequest {
        pub fn builder(
        ) -> GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestBuilder
        {
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestBuilder :: default ()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of platform at risk by entries present in the list."]
    pub enum GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum
    {
        #[serde(rename = "PLATFORM_TYPE_UNSPECIFIED")]
        #[doc = "Unknown platform."]
        PlatformTypeUnspecified,
        #[serde(rename = "WINDOWS")]
        #[doc = "Threat posed to Windows."]
        Windows,
        #[serde(rename = "LINUX")]
        #[doc = "Threat posed to Linux."]
        Linux,
        #[serde(rename = "ANDROID")]
        #[doc = "Threat posed to Android."]
        Android,
        #[serde(rename = "OSX")]
        #[doc = "Threat posed to OS X."]
        Osx,
        #[serde(rename = "IOS")]
        #[doc = "Threat posed to iOS."]
        Ios,
        #[serde(rename = "ANY_PLATFORM")]
        #[doc = "Threat posed to at least one of the defined platforms."]
        AnyPlatform,
        #[serde(rename = "ALL_PLATFORMS")]
        #[doc = "Threat posed to all defined platforms."]
        AllPlatforms,
        #[serde(rename = "CHROME")]
        #[doc = "Threat posed to Chrome."]
        Chrome,
    }
    impl :: std :: default :: Default for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestPlatformTypeEnum { fn default () -> Self { Self :: PlatformTypeUnspecified } }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The types of entries present in the list."]
    pub enum GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryTypeEnum
    {
        #[serde(rename = "THREAT_ENTRY_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified."]
        ThreatEntryTypeUnspecified,
        #[serde(rename = "URL")]
        #[doc = "A URL."]
        Url,
        #[serde(rename = "EXECUTABLE")]
        #[doc = "An executable program."]
        Executable,
        #[serde(rename = "IP_RANGE")]
        #[doc = "An IP range."]
        IpRange,
        #[serde(rename = "CHROME_EXTENSION")]
        #[doc = "Chrome extension."]
        ChromeExtension,
        #[serde(rename = "FILENAME")]
        #[doc = "Filename."]
        Filename,
        #[serde(rename = "CERT")]
        #[doc = "CERT"]
        Cert,
    }
    impl :: std :: default :: Default for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatEntryTypeEnum { fn default () -> Self { Self :: ThreatEntryTypeUnspecified } }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of threat posed by entries present in the list."]
    pub enum GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum {
        #[serde(rename = "THREAT_TYPE_UNSPECIFIED")]
        #[doc = "Unknown."]
        ThreatTypeUnspecified,
        #[serde(rename = "MALWARE")]
        #[doc = "Malware threat type."]
        Malware,
        #[serde(rename = "SOCIAL_ENGINEERING")]
        #[doc = "Social engineering threat type."]
        SocialEngineering,
        #[serde(rename = "UNWANTED_SOFTWARE")]
        #[doc = "Unwanted software threat type."]
        UnwantedSoftware,
        #[serde(rename = "POTENTIALLY_HARMFUL_APPLICATION")]
        #[doc = "Potentially harmful application threat type."]
        PotentiallyHarmfulApplication,
        #[serde(rename = "SOCIAL_ENGINEERING_INTERNAL")]
        #[doc = "Social engineering threat type for internal use."]
        SocialEngineeringInternal,
        #[serde(rename = "API_ABUSE")]
        #[doc = "API abuse threat type."]
        ApiAbuse,
        #[serde(rename = "MALICIOUS_BINARY")]
        #[doc = "Malicious binary threat type."]
        MaliciousBinary,
        #[serde(rename = "CSD_WHITELIST")]
        #[doc = "Client side detection whitelist threat type."]
        CsdWhitelist,
        #[serde(rename = "CSD_DOWNLOAD_WHITELIST")]
        #[doc = "Client side download detection whitelist threat type."]
        CsdDownloadWhitelist,
        #[serde(rename = "CLIENT_INCIDENT")]
        #[doc = "Client incident threat type."]
        ClientIncident,
        #[serde(rename = "CLIENT_INCIDENT_WHITELIST")]
        #[doc = "Whitelist used when detecting client incident threats. This enum was never launched and should be re-used for the next list."]
        ClientIncidentWhitelist,
        #[serde(rename = "APK_MALWARE_OFFLINE")]
        #[doc = "List used for offline APK checks in PAM."]
        ApkMalwareOffline,
        #[serde(rename = "SUBRESOURCE_FILTER")]
        #[doc = "Patterns to be used for activating the subresource filter. Interstitial will not be shown for patterns from this list."]
        SubresourceFilter,
        #[serde(rename = "SUSPICIOUS")]
        #[doc = "Entities that are suspected to present a threat."]
        Suspicious,
        #[serde(rename = "TRICK_TO_BILL")]
        #[doc = "Trick-to-bill threat list."]
        TrickToBill,
        #[serde(rename = "HIGH_CONFIDENCE_ALLOWLIST")]
        #[doc = "Safe list to ship hashes of known safe URL expressions."]
        HighConfidenceAllowlist,
    }
    impl ::std::default::Default
        for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestThreatTypeEnum
    {
        fn default() -> Self {
            Self::ThreatTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The constraints for this update."]
    pub struct GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraints { # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "deviceLocation")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "A client's physical location, expressed as a ISO 31166-1 alpha-2 region code."] pub device_location : :: std :: option :: Option < :: std :: string :: String > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "language")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Requests the lists for a specific language. Expects ISO 639 alpha-2 format."] pub language : :: std :: option :: Option < :: std :: string :: String > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "maxDatabaseEntries")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Sets the maximum number of entries that the client is willing to have in the local database for the specified list. This should be a power of 2 between 2**10 and 2**20. If zero, no database size limit is set."] pub max_database_entries : :: std :: option :: Option < :: std :: primitive :: i64 > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "maxUpdateEntries")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "The maximum size in number of entries. The update will not contain more entries than this value. This should be a power of 2 between 2**10 and 2**20. If zero, no update size limit is set."] pub max_update_entries : :: std :: option :: Option < :: std :: primitive :: i64 > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "region")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Requests the list for a specific geographic location. If not set the server may pick that value based on the user's IP address. Expects ISO 3166-1 alpha-2 format."] pub region : :: std :: option :: Option < :: std :: string :: String > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "supportedCompressions")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "The compression types supported by the client."] pub supported_compressions : :: std :: option :: Option < :: std :: vec :: Vec < GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintsSupportedCompressionsEnum > > }
    impl GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraints {
        pub fn builder () -> GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintsBuilder{
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintsBuilder :: default ()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintsSupportedCompressionsEnum
    {
        #[serde(rename = "COMPRESSION_TYPE_UNSPECIFIED")]
        #[doc = "Unknown."]
        CompressionTypeUnspecified,
        #[serde(rename = "RAW")]
        #[doc = "Raw, uncompressed data."]
        Raw,
        #[serde(rename = "RICE")]
        #[doc = "Rice-Golomb encoded data."]
        Rice,
    }
    impl :: std :: default :: Default for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesRequestListUpdateRequestConstraintsSupportedCompressionsEnum { fn default () -> Self { Self :: CompressionTypeUnspecified } }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "listUpdateResponses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list updates requested by the clients. The number of responses here may be less than the number of requests sent by clients. This is the case, for example, if the server has no updates for a particular list."]
        pub list_update_responses: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<
                    GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponse,
                >,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minimumWaitDuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minimum duration the client must wait before issuing any update request. If this field is not set clients may update as soon as they want."]
        pub minimum_wait_duration: ::std::option::Option<::std::string::String>,
    }
    impl GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponse {
        pub fn builder() -> GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseBuilder {
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An update to an individual list."]
    pub struct GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponse { # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "additions")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "A set of entries to add to a local threat type's list. Repeated to allow for a combination of compressed and raw data to be sent in a single response."] pub additions : :: std :: option :: Option < :: std :: vec :: Vec < :: std :: boxed :: Box < GoogleSecuritySafebrowsingV4ThreatEntrySet > > > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "checksum")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "The expected SHA256 hash of the client state; that is, of the sorted list of all hashes present in the database after applying the provided update. If the client state doesn't match the expected state, the client must disregard this update and retry later."] pub checksum : :: std :: option :: Option < :: std :: boxed :: Box < GoogleSecuritySafebrowsingV4Checksum > > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "newClientState")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "The new client state, in encrypted format. Opaque to clients."] pub new_client_state : :: std :: option :: Option < :: std :: string :: String > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "platformType")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "The platform type for which data is returned."] pub platform_type : :: std :: option :: Option < GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "removals")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "A set of entries to remove from a local threat type's list. In practice, this field is empty or contains exactly one ThreatEntrySet."] pub removals : :: std :: option :: Option < :: std :: vec :: Vec < :: std :: boxed :: Box < GoogleSecuritySafebrowsingV4ThreatEntrySet > > > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "responseType")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "The type of response. This may indicate that an action is required by the client when the response is received."] pub response_type : :: std :: option :: Option < GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseTypeEnum > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "threatEntryType")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "The format of the threats."] pub threat_entry_type : :: std :: option :: Option < GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryTypeEnum > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "threatType")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "The threat type for which data is returned."] pub threat_type : :: std :: option :: Option < GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum > }
    impl GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponse {
        pub fn builder(
        ) -> GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseBuilder
        {
            GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseBuilder :: default ()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The platform type for which data is returned."]
    pub enum GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum
    {
        #[serde(rename = "PLATFORM_TYPE_UNSPECIFIED")]
        #[doc = "Unknown platform."]
        PlatformTypeUnspecified,
        #[serde(rename = "WINDOWS")]
        #[doc = "Threat posed to Windows."]
        Windows,
        #[serde(rename = "LINUX")]
        #[doc = "Threat posed to Linux."]
        Linux,
        #[serde(rename = "ANDROID")]
        #[doc = "Threat posed to Android."]
        Android,
        #[serde(rename = "OSX")]
        #[doc = "Threat posed to OS X."]
        Osx,
        #[serde(rename = "IOS")]
        #[doc = "Threat posed to iOS."]
        Ios,
        #[serde(rename = "ANY_PLATFORM")]
        #[doc = "Threat posed to at least one of the defined platforms."]
        AnyPlatform,
        #[serde(rename = "ALL_PLATFORMS")]
        #[doc = "Threat posed to all defined platforms."]
        AllPlatforms,
        #[serde(rename = "CHROME")]
        #[doc = "Threat posed to Chrome."]
        Chrome,
    }
    impl :: std :: default :: Default for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponsePlatformTypeEnum { fn default () -> Self { Self :: PlatformTypeUnspecified } }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of response. This may indicate that an action is required by the client when the response is received."]
    pub enum GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseTypeEnum
    {
        #[serde(rename = "RESPONSE_TYPE_UNSPECIFIED")]
        #[doc = "Unknown."]
        ResponseTypeUnspecified,
        #[serde(rename = "PARTIAL_UPDATE")]
        #[doc = "Partial updates are applied to the client's existing local database."]
        PartialUpdate,
        #[serde(rename = "FULL_UPDATE")]
        #[doc = "Full updates replace the client's entire local database. This means that either the client was seriously out-of-date or the client is believed to be corrupt."]
        FullUpdate,
    }
    impl :: std :: default :: Default for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseResponseTypeEnum { fn default () -> Self { Self :: ResponseTypeUnspecified } }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The format of the threats."]
    pub enum GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryTypeEnum
    {
        #[serde(rename = "THREAT_ENTRY_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified."]
        ThreatEntryTypeUnspecified,
        #[serde(rename = "URL")]
        #[doc = "A URL."]
        Url,
        #[serde(rename = "EXECUTABLE")]
        #[doc = "An executable program."]
        Executable,
        #[serde(rename = "IP_RANGE")]
        #[doc = "An IP range."]
        IpRange,
        #[serde(rename = "CHROME_EXTENSION")]
        #[doc = "Chrome extension."]
        ChromeExtension,
        #[serde(rename = "FILENAME")]
        #[doc = "Filename."]
        Filename,
        #[serde(rename = "CERT")]
        #[doc = "CERT"]
        Cert,
    }
    impl :: std :: default :: Default for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatEntryTypeEnum { fn default () -> Self { Self :: ThreatEntryTypeUnspecified } }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The threat type for which data is returned."]
    pub enum GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum
    {
        #[serde(rename = "THREAT_TYPE_UNSPECIFIED")]
        #[doc = "Unknown."]
        ThreatTypeUnspecified,
        #[serde(rename = "MALWARE")]
        #[doc = "Malware threat type."]
        Malware,
        #[serde(rename = "SOCIAL_ENGINEERING")]
        #[doc = "Social engineering threat type."]
        SocialEngineering,
        #[serde(rename = "UNWANTED_SOFTWARE")]
        #[doc = "Unwanted software threat type."]
        UnwantedSoftware,
        #[serde(rename = "POTENTIALLY_HARMFUL_APPLICATION")]
        #[doc = "Potentially harmful application threat type."]
        PotentiallyHarmfulApplication,
        #[serde(rename = "SOCIAL_ENGINEERING_INTERNAL")]
        #[doc = "Social engineering threat type for internal use."]
        SocialEngineeringInternal,
        #[serde(rename = "API_ABUSE")]
        #[doc = "API abuse threat type."]
        ApiAbuse,
        #[serde(rename = "MALICIOUS_BINARY")]
        #[doc = "Malicious binary threat type."]
        MaliciousBinary,
        #[serde(rename = "CSD_WHITELIST")]
        #[doc = "Client side detection whitelist threat type."]
        CsdWhitelist,
        #[serde(rename = "CSD_DOWNLOAD_WHITELIST")]
        #[doc = "Client side download detection whitelist threat type."]
        CsdDownloadWhitelist,
        #[serde(rename = "CLIENT_INCIDENT")]
        #[doc = "Client incident threat type."]
        ClientIncident,
        #[serde(rename = "CLIENT_INCIDENT_WHITELIST")]
        #[doc = "Whitelist used when detecting client incident threats. This enum was never launched and should be re-used for the next list."]
        ClientIncidentWhitelist,
        #[serde(rename = "APK_MALWARE_OFFLINE")]
        #[doc = "List used for offline APK checks in PAM."]
        ApkMalwareOffline,
        #[serde(rename = "SUBRESOURCE_FILTER")]
        #[doc = "Patterns to be used for activating the subresource filter. Interstitial will not be shown for patterns from this list."]
        SubresourceFilter,
        #[serde(rename = "SUSPICIOUS")]
        #[doc = "Entities that are suspected to present a threat."]
        Suspicious,
        #[serde(rename = "TRICK_TO_BILL")]
        #[doc = "Trick-to-bill threat list."]
        TrickToBill,
        #[serde(rename = "HIGH_CONFIDENCE_ALLOWLIST")]
        #[doc = "Safe list to ship hashes of known safe URL expressions."]
        HighConfidenceAllowlist,
    }
    impl :: std :: default :: Default for GoogleSecuritySafebrowsingV4FetchThreatListUpdatesResponseListUpdateResponseThreatTypeEnum { fn default () -> Self { Self :: ThreatTypeUnspecified } }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to return full hashes matched by the provided hash prefixes."]
    pub struct GoogleSecuritySafebrowsingV4FindFullHashesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiClient")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Client metadata associated with callers of higher-level APIs built on top of the client's implementation."]
        pub api_client:
            ::std::option::Option<::std::boxed::Box<GoogleSecuritySafebrowsingV4ClientInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "client")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The client metadata."]
        pub client:
            ::std::option::Option<::std::boxed::Box<GoogleSecuritySafebrowsingV4ClientInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientStates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current client states for each of the client's local threat lists."]
        pub client_states: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threatInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The lists and hashes to be checked."]
        pub threat_info:
            ::std::option::Option<::std::boxed::Box<GoogleSecuritySafebrowsingV4ThreatInfo>>,
    }
    impl GoogleSecuritySafebrowsingV4FindFullHashesRequest {
        pub fn builder() -> GoogleSecuritySafebrowsingV4FindFullHashesRequestBuilder {
            GoogleSecuritySafebrowsingV4FindFullHashesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleSecuritySafebrowsingV4FindFullHashesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matches")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full hashes that matched the requested prefixes."]
        pub matches: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleSecuritySafebrowsingV4ThreatMatch>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minimumWaitDuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minimum duration the client must wait before issuing any find hashes request. If this field is not set, clients can issue a request as soon as they want."]
        pub minimum_wait_duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "negativeCacheDuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For requested entities that did not match the threat list, how long to cache the response."]
        pub negative_cache_duration: ::std::option::Option<::std::string::String>,
    }
    impl GoogleSecuritySafebrowsingV4FindFullHashesResponse {
        pub fn builder() -> GoogleSecuritySafebrowsingV4FindFullHashesResponseBuilder {
            GoogleSecuritySafebrowsingV4FindFullHashesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to check entries against lists."]
    pub struct GoogleSecuritySafebrowsingV4FindThreatMatchesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "client")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The client metadata."]
        pub client:
            ::std::option::Option<::std::boxed::Box<GoogleSecuritySafebrowsingV4ClientInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threatInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The lists and entries to be checked for matches."]
        pub threat_info:
            ::std::option::Option<::std::boxed::Box<GoogleSecuritySafebrowsingV4ThreatInfo>>,
    }
    impl GoogleSecuritySafebrowsingV4FindThreatMatchesRequest {
        pub fn builder() -> GoogleSecuritySafebrowsingV4FindThreatMatchesRequestBuilder {
            GoogleSecuritySafebrowsingV4FindThreatMatchesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleSecuritySafebrowsingV4FindThreatMatchesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matches")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The threat list matches."]
        pub matches: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleSecuritySafebrowsingV4ThreatMatch>>,
        >,
    }
    impl GoogleSecuritySafebrowsingV4FindThreatMatchesResponse {
        pub fn builder() -> GoogleSecuritySafebrowsingV4FindThreatMatchesResponseBuilder {
            GoogleSecuritySafebrowsingV4FindThreatMatchesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleSecuritySafebrowsingV4ListThreatListsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threatLists")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The lists available for download by the client."]
        pub threat_lists: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleSecuritySafebrowsingV4ThreatListDescriptor>>,
        >,
    }
    impl GoogleSecuritySafebrowsingV4ListThreatListsResponse {
        pub fn builder() -> GoogleSecuritySafebrowsingV4ListThreatListsResponseBuilder {
            GoogleSecuritySafebrowsingV4ListThreatListsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The uncompressed threat entries in hash format of a particular prefix length. Hashes can be anywhere from 4 to 32 bytes in size. A large majority are 4 bytes, but some hashes are lengthened if they collide with the hash of a popular URL. Used for sending ThreatEntrySet to clients that do not support compression, or when sending non-4-byte hashes to clients that do support compression."]
    pub struct GoogleSecuritySafebrowsingV4RawHashes {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "prefixSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of bytes for each prefix encoded below. This field can be anywhere from 4 (shortest prefix) to 32 (full SHA256 hash)."]
        pub prefix_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rawHashes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The hashes, in binary format, concatenated into one long string. Hashes are sorted in lexicographic order. For JSON API users, hashes are base64-encoded."]
        pub raw_hashes: ::std::option::Option<::std::string::String>,
    }
    impl GoogleSecuritySafebrowsingV4RawHashes {
        pub fn builder() -> GoogleSecuritySafebrowsingV4RawHashesBuilder {
            GoogleSecuritySafebrowsingV4RawHashesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A set of raw indices to remove from a local list."]
    pub struct GoogleSecuritySafebrowsingV4RawIndices {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "indices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The indices to remove from a lexicographically-sorted local list."]
        pub indices: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    }
    impl GoogleSecuritySafebrowsingV4RawIndices {
        pub fn builder() -> GoogleSecuritySafebrowsingV4RawIndicesBuilder {
            GoogleSecuritySafebrowsingV4RawIndicesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Rice-Golomb encoded data. Used for sending compressed 4-byte hashes or compressed removal indices."]
    pub struct GoogleSecuritySafebrowsingV4RiceDeltaEncoding {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "encodedData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The encoded deltas that are encoded using the Golomb-Rice coder."]
        pub encoded_data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firstValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The offset of the first entry in the encoded data, or, if only a single integer was encoded, that single integer's value. If the field is empty or missing, assume zero."]
        pub first_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numEntries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of entries that are delta encoded in the encoded data. If only a single integer was encoded, this will be zero and the single value will be stored in `first_value`."]
        pub num_entries: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "riceParameter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Golomb-Rice parameter, which is a number between 2 and 28. This field is missing (that is, zero) if `num_entries` is zero."]
        pub rice_parameter: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleSecuritySafebrowsingV4RiceDeltaEncoding {
        pub fn builder() -> GoogleSecuritySafebrowsingV4RiceDeltaEncodingBuilder {
            GoogleSecuritySafebrowsingV4RiceDeltaEncodingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An individual threat; for example, a malicious URL or its hash representation. Only one of these fields should be set."]
    pub struct GoogleSecuritySafebrowsingV4ThreatEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "digest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The digest of an executable in SHA256 format. The API supports both binary and hex digests. For JSON requests, digests are base64-encoded."]
        pub digest: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A hash prefix, consisting of the most significant 4-32 bytes of a SHA256 hash. This field is in binary format. For JSON requests, hashes are base64-encoded."]
        pub hash: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A URL."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl GoogleSecuritySafebrowsingV4ThreatEntry {
        pub fn builder() -> GoogleSecuritySafebrowsingV4ThreatEntryBuilder {
            GoogleSecuritySafebrowsingV4ThreatEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The metadata associated with a specific threat entry. The client is expected to know the metadata key/value pairs associated with each threat type."]
    pub struct GoogleSecuritySafebrowsingV4ThreatEntryMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metadata entries."]
        pub entries: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleSecuritySafebrowsingV4ThreatEntryMetadataMetadataEntry>,
            >,
        >,
    }
    impl GoogleSecuritySafebrowsingV4ThreatEntryMetadata {
        pub fn builder() -> GoogleSecuritySafebrowsingV4ThreatEntryMetadataBuilder {
            GoogleSecuritySafebrowsingV4ThreatEntryMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single metadata entry."]
    pub struct GoogleSecuritySafebrowsingV4ThreatEntryMetadataMetadataEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metadata entry key. For JSON requests, the key is base64-encoded."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metadata entry value. For JSON requests, the value is base64-encoded."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleSecuritySafebrowsingV4ThreatEntryMetadataMetadataEntry {
        pub fn builder() -> GoogleSecuritySafebrowsingV4ThreatEntryMetadataMetadataEntryBuilder {
            GoogleSecuritySafebrowsingV4ThreatEntryMetadataMetadataEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A set of threats that should be added or removed from a client's local database."]
    pub struct GoogleSecuritySafebrowsingV4ThreatEntrySet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "compressionType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The compression type for the entries in this set."]
        pub compression_type:
            ::std::option::Option<GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rawHashes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The raw SHA256-formatted entries."]
        pub raw_hashes:
            ::std::option::Option<::std::boxed::Box<GoogleSecuritySafebrowsingV4RawHashes>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rawIndices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The raw removal indices for a local list."]
        pub raw_indices:
            ::std::option::Option<::std::boxed::Box<GoogleSecuritySafebrowsingV4RawIndices>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "riceHashes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The encoded 4-byte prefixes of SHA256-formatted entries, using a Golomb-Rice encoding. The hashes are converted to uint32, sorted in ascending order, then delta encoded and stored as encoded_data."]
        pub rice_hashes:
            ::std::option::Option<::std::boxed::Box<GoogleSecuritySafebrowsingV4RiceDeltaEncoding>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "riceIndices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The encoded local, lexicographically-sorted list indices, using a Golomb-Rice encoding. Used for sending compressed removal indices. The removal indices (uint32) are sorted in ascending order, then delta encoded and stored as encoded_data."]
        pub rice_indices:
            ::std::option::Option<::std::boxed::Box<GoogleSecuritySafebrowsingV4RiceDeltaEncoding>>,
    }
    impl GoogleSecuritySafebrowsingV4ThreatEntrySet {
        pub fn builder() -> GoogleSecuritySafebrowsingV4ThreatEntrySetBuilder {
            GoogleSecuritySafebrowsingV4ThreatEntrySetBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The compression type for the entries in this set."]
    pub enum GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionTypeEnum {
        #[serde(rename = "COMPRESSION_TYPE_UNSPECIFIED")]
        #[doc = "Unknown."]
        CompressionTypeUnspecified,
        #[serde(rename = "RAW")]
        #[doc = "Raw, uncompressed data."]
        Raw,
        #[serde(rename = "RICE")]
        #[doc = "Rice-Golomb encoded data."]
        Rice,
    }
    impl ::std::default::Default for GoogleSecuritySafebrowsingV4ThreatEntrySetCompressionTypeEnum {
        fn default() -> Self {
            Self::CompressionTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleSecuritySafebrowsingV4ThreatHit {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Client-reported identification."]
        pub client_info:
            ::std::option::Option<::std::boxed::Box<GoogleSecuritySafebrowsingV4ClientInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entry")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The threat entry responsible for the hit. Full hash should be reported for hash-based hits."]
        pub entry:
            ::std::option::Option<::std::boxed::Box<GoogleSecuritySafebrowsingV4ThreatEntry>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "platformType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The platform type reported."]
        pub platform_type:
            ::std::option::Option<GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resources related to the threat hit."]
        pub resources: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleSecuritySafebrowsingV4ThreatHitThreatSource>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threatType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The threat type reported."]
        pub threat_type: ::std::option::Option<GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details about the user that encountered the threat."]
        pub user_info:
            ::std::option::Option<::std::boxed::Box<GoogleSecuritySafebrowsingV4ThreatHitUserInfo>>,
    }
    impl GoogleSecuritySafebrowsingV4ThreatHit {
        pub fn builder() -> GoogleSecuritySafebrowsingV4ThreatHitBuilder {
            GoogleSecuritySafebrowsingV4ThreatHitBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The platform type reported."]
    pub enum GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum {
        #[serde(rename = "PLATFORM_TYPE_UNSPECIFIED")]
        #[doc = "Unknown platform."]
        PlatformTypeUnspecified,
        #[serde(rename = "WINDOWS")]
        #[doc = "Threat posed to Windows."]
        Windows,
        #[serde(rename = "LINUX")]
        #[doc = "Threat posed to Linux."]
        Linux,
        #[serde(rename = "ANDROID")]
        #[doc = "Threat posed to Android."]
        Android,
        #[serde(rename = "OSX")]
        #[doc = "Threat posed to OS X."]
        Osx,
        #[serde(rename = "IOS")]
        #[doc = "Threat posed to iOS."]
        Ios,
        #[serde(rename = "ANY_PLATFORM")]
        #[doc = "Threat posed to at least one of the defined platforms."]
        AnyPlatform,
        #[serde(rename = "ALL_PLATFORMS")]
        #[doc = "Threat posed to all defined platforms."]
        AllPlatforms,
        #[serde(rename = "CHROME")]
        #[doc = "Threat posed to Chrome."]
        Chrome,
    }
    impl ::std::default::Default for GoogleSecuritySafebrowsingV4ThreatHitPlatformTypeEnum {
        fn default() -> Self {
            Self::PlatformTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The threat type reported."]
    pub enum GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum {
        #[serde(rename = "THREAT_TYPE_UNSPECIFIED")]
        #[doc = "Unknown."]
        ThreatTypeUnspecified,
        #[serde(rename = "MALWARE")]
        #[doc = "Malware threat type."]
        Malware,
        #[serde(rename = "SOCIAL_ENGINEERING")]
        #[doc = "Social engineering threat type."]
        SocialEngineering,
        #[serde(rename = "UNWANTED_SOFTWARE")]
        #[doc = "Unwanted software threat type."]
        UnwantedSoftware,
        #[serde(rename = "POTENTIALLY_HARMFUL_APPLICATION")]
        #[doc = "Potentially harmful application threat type."]
        PotentiallyHarmfulApplication,
        #[serde(rename = "SOCIAL_ENGINEERING_INTERNAL")]
        #[doc = "Social engineering threat type for internal use."]
        SocialEngineeringInternal,
        #[serde(rename = "API_ABUSE")]
        #[doc = "API abuse threat type."]
        ApiAbuse,
        #[serde(rename = "MALICIOUS_BINARY")]
        #[doc = "Malicious binary threat type."]
        MaliciousBinary,
        #[serde(rename = "CSD_WHITELIST")]
        #[doc = "Client side detection whitelist threat type."]
        CsdWhitelist,
        #[serde(rename = "CSD_DOWNLOAD_WHITELIST")]
        #[doc = "Client side download detection whitelist threat type."]
        CsdDownloadWhitelist,
        #[serde(rename = "CLIENT_INCIDENT")]
        #[doc = "Client incident threat type."]
        ClientIncident,
        #[serde(rename = "CLIENT_INCIDENT_WHITELIST")]
        #[doc = "Whitelist used when detecting client incident threats. This enum was never launched and should be re-used for the next list."]
        ClientIncidentWhitelist,
        #[serde(rename = "APK_MALWARE_OFFLINE")]
        #[doc = "List used for offline APK checks in PAM."]
        ApkMalwareOffline,
        #[serde(rename = "SUBRESOURCE_FILTER")]
        #[doc = "Patterns to be used for activating the subresource filter. Interstitial will not be shown for patterns from this list."]
        SubresourceFilter,
        #[serde(rename = "SUSPICIOUS")]
        #[doc = "Entities that are suspected to present a threat."]
        Suspicious,
        #[serde(rename = "TRICK_TO_BILL")]
        #[doc = "Trick-to-bill threat list."]
        TrickToBill,
        #[serde(rename = "HIGH_CONFIDENCE_ALLOWLIST")]
        #[doc = "Safe list to ship hashes of known safe URL expressions."]
        HighConfidenceAllowlist,
    }
    impl ::std::default::Default for GoogleSecuritySafebrowsingV4ThreatHitThreatTypeEnum {
        fn default() -> Self {
            Self::ThreatTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single resource related to a threat hit."]
    pub struct GoogleSecuritySafebrowsingV4ThreatHitThreatSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "referrer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Referrer of the resource. Only set if the referrer is available."]
        pub referrer: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "remoteIp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The remote IP of the resource in ASCII format. Either IPv4 or IPv6."]
        pub remote_ip: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of source reported."]
        pub _type: ::std::option::Option<GoogleSecuritySafebrowsingV4ThreatHitThreatSourceTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the resource."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl GoogleSecuritySafebrowsingV4ThreatHitThreatSource {
        pub fn builder() -> GoogleSecuritySafebrowsingV4ThreatHitThreatSourceBuilder {
            GoogleSecuritySafebrowsingV4ThreatHitThreatSourceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of source reported."]
    pub enum GoogleSecuritySafebrowsingV4ThreatHitThreatSourceTypeEnum {
        #[serde(rename = "THREAT_SOURCE_TYPE_UNSPECIFIED")]
        #[doc = "Unknown."]
        ThreatSourceTypeUnspecified,
        #[serde(rename = "MATCHING_URL")]
        #[doc = "The URL that matched the threat list (for which GetFullHash returned a valid hash)."]
        MatchingUrl,
        #[serde(rename = "TAB_URL")]
        #[doc = "The final top-level URL of the tab that the client was browsing when the match occurred."]
        TabUrl,
        #[serde(rename = "TAB_REDIRECT")]
        #[doc = "A redirect URL that was fetched before hitting the final TAB_URL."]
        TabRedirect,
        #[serde(rename = "TAB_RESOURCE")]
        #[doc = "A resource loaded within the final TAB_URL."]
        TabResource,
    }
    impl ::std::default::Default for GoogleSecuritySafebrowsingV4ThreatHitThreatSourceTypeEnum {
        fn default() -> Self {
            Self::ThreatSourceTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details about the user that encountered the threat."]
    pub struct GoogleSecuritySafebrowsingV4ThreatHitUserInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "regionCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The UN M.49 region code associated with the user's location."]
        pub region_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique user identifier defined by the client."]
        pub user_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleSecuritySafebrowsingV4ThreatHitUserInfo {
        pub fn builder() -> GoogleSecuritySafebrowsingV4ThreatHitUserInfoBuilder {
            GoogleSecuritySafebrowsingV4ThreatHitUserInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The information regarding one or more threats that a client submits when checking for matches in threat lists."]
    pub struct GoogleSecuritySafebrowsingV4ThreatInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "platformTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The platform types to be checked."]
        pub platform_types: ::std::option::Option<
            ::std::vec::Vec<GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threatEntries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The threat entries to be checked."]
        pub threat_entries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleSecuritySafebrowsingV4ThreatEntry>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threatEntryTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entry types to be checked."]
        pub threat_entry_types: ::std::option::Option<
            ::std::vec::Vec<GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesEnum>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threatTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The threat types to be checked."]
        pub threat_types: ::std::option::Option<
            ::std::vec::Vec<GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum>,
        >,
    }
    impl GoogleSecuritySafebrowsingV4ThreatInfo {
        pub fn builder() -> GoogleSecuritySafebrowsingV4ThreatInfoBuilder {
            GoogleSecuritySafebrowsingV4ThreatInfoBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum {
        #[serde(rename = "PLATFORM_TYPE_UNSPECIFIED")]
        #[doc = "Unknown platform."]
        PlatformTypeUnspecified,
        #[serde(rename = "WINDOWS")]
        #[doc = "Threat posed to Windows."]
        Windows,
        #[serde(rename = "LINUX")]
        #[doc = "Threat posed to Linux."]
        Linux,
        #[serde(rename = "ANDROID")]
        #[doc = "Threat posed to Android."]
        Android,
        #[serde(rename = "OSX")]
        #[doc = "Threat posed to OS X."]
        Osx,
        #[serde(rename = "IOS")]
        #[doc = "Threat posed to iOS."]
        Ios,
        #[serde(rename = "ANY_PLATFORM")]
        #[doc = "Threat posed to at least one of the defined platforms."]
        AnyPlatform,
        #[serde(rename = "ALL_PLATFORMS")]
        #[doc = "Threat posed to all defined platforms."]
        AllPlatforms,
        #[serde(rename = "CHROME")]
        #[doc = "Threat posed to Chrome."]
        Chrome,
    }
    impl ::std::default::Default for GoogleSecuritySafebrowsingV4ThreatInfoPlatformTypesEnum {
        fn default() -> Self {
            Self::PlatformTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesEnum {
        #[serde(rename = "THREAT_ENTRY_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified."]
        ThreatEntryTypeUnspecified,
        #[serde(rename = "URL")]
        #[doc = "A URL."]
        Url,
        #[serde(rename = "EXECUTABLE")]
        #[doc = "An executable program."]
        Executable,
        #[serde(rename = "IP_RANGE")]
        #[doc = "An IP range."]
        IpRange,
        #[serde(rename = "CHROME_EXTENSION")]
        #[doc = "Chrome extension."]
        ChromeExtension,
        #[serde(rename = "FILENAME")]
        #[doc = "Filename."]
        Filename,
        #[serde(rename = "CERT")]
        #[doc = "CERT"]
        Cert,
    }
    impl ::std::default::Default for GoogleSecuritySafebrowsingV4ThreatInfoThreatEntryTypesEnum {
        fn default() -> Self {
            Self::ThreatEntryTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum {
        #[serde(rename = "THREAT_TYPE_UNSPECIFIED")]
        #[doc = "Unknown."]
        ThreatTypeUnspecified,
        #[serde(rename = "MALWARE")]
        #[doc = "Malware threat type."]
        Malware,
        #[serde(rename = "SOCIAL_ENGINEERING")]
        #[doc = "Social engineering threat type."]
        SocialEngineering,
        #[serde(rename = "UNWANTED_SOFTWARE")]
        #[doc = "Unwanted software threat type."]
        UnwantedSoftware,
        #[serde(rename = "POTENTIALLY_HARMFUL_APPLICATION")]
        #[doc = "Potentially harmful application threat type."]
        PotentiallyHarmfulApplication,
        #[serde(rename = "SOCIAL_ENGINEERING_INTERNAL")]
        #[doc = "Social engineering threat type for internal use."]
        SocialEngineeringInternal,
        #[serde(rename = "API_ABUSE")]
        #[doc = "API abuse threat type."]
        ApiAbuse,
        #[serde(rename = "MALICIOUS_BINARY")]
        #[doc = "Malicious binary threat type."]
        MaliciousBinary,
        #[serde(rename = "CSD_WHITELIST")]
        #[doc = "Client side detection whitelist threat type."]
        CsdWhitelist,
        #[serde(rename = "CSD_DOWNLOAD_WHITELIST")]
        #[doc = "Client side download detection whitelist threat type."]
        CsdDownloadWhitelist,
        #[serde(rename = "CLIENT_INCIDENT")]
        #[doc = "Client incident threat type."]
        ClientIncident,
        #[serde(rename = "CLIENT_INCIDENT_WHITELIST")]
        #[doc = "Whitelist used when detecting client incident threats. This enum was never launched and should be re-used for the next list."]
        ClientIncidentWhitelist,
        #[serde(rename = "APK_MALWARE_OFFLINE")]
        #[doc = "List used for offline APK checks in PAM."]
        ApkMalwareOffline,
        #[serde(rename = "SUBRESOURCE_FILTER")]
        #[doc = "Patterns to be used for activating the subresource filter. Interstitial will not be shown for patterns from this list."]
        SubresourceFilter,
        #[serde(rename = "SUSPICIOUS")]
        #[doc = "Entities that are suspected to present a threat."]
        Suspicious,
        #[serde(rename = "TRICK_TO_BILL")]
        #[doc = "Trick-to-bill threat list."]
        TrickToBill,
        #[serde(rename = "HIGH_CONFIDENCE_ALLOWLIST")]
        #[doc = "Safe list to ship hashes of known safe URL expressions."]
        HighConfidenceAllowlist,
    }
    impl ::std::default::Default for GoogleSecuritySafebrowsingV4ThreatInfoThreatTypesEnum {
        fn default() -> Self {
            Self::ThreatTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes an individual threat list. A list is defined by three parameters: the type of threat posed, the type of platform targeted by the threat, and the type of entries in the list."]
    pub struct GoogleSecuritySafebrowsingV4ThreatListDescriptor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "platformType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The platform type targeted by the list's entries."]
        pub platform_type:
            ::std::option::Option<GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threatEntryType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entry types contained in the list."]
        pub threat_entry_type: ::std::option::Option<
            GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryTypeEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threatType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The threat type posed by the list's entries."]
        pub threat_type:
            ::std::option::Option<GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum>,
    }
    impl GoogleSecuritySafebrowsingV4ThreatListDescriptor {
        pub fn builder() -> GoogleSecuritySafebrowsingV4ThreatListDescriptorBuilder {
            GoogleSecuritySafebrowsingV4ThreatListDescriptorBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The platform type targeted by the list's entries."]
    pub enum GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum {
        #[serde(rename = "PLATFORM_TYPE_UNSPECIFIED")]
        #[doc = "Unknown platform."]
        PlatformTypeUnspecified,
        #[serde(rename = "WINDOWS")]
        #[doc = "Threat posed to Windows."]
        Windows,
        #[serde(rename = "LINUX")]
        #[doc = "Threat posed to Linux."]
        Linux,
        #[serde(rename = "ANDROID")]
        #[doc = "Threat posed to Android."]
        Android,
        #[serde(rename = "OSX")]
        #[doc = "Threat posed to OS X."]
        Osx,
        #[serde(rename = "IOS")]
        #[doc = "Threat posed to iOS."]
        Ios,
        #[serde(rename = "ANY_PLATFORM")]
        #[doc = "Threat posed to at least one of the defined platforms."]
        AnyPlatform,
        #[serde(rename = "ALL_PLATFORMS")]
        #[doc = "Threat posed to all defined platforms."]
        AllPlatforms,
        #[serde(rename = "CHROME")]
        #[doc = "Threat posed to Chrome."]
        Chrome,
    }
    impl ::std::default::Default for GoogleSecuritySafebrowsingV4ThreatListDescriptorPlatformTypeEnum {
        fn default() -> Self {
            Self::PlatformTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The entry types contained in the list."]
    pub enum GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryTypeEnum {
        #[serde(rename = "THREAT_ENTRY_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified."]
        ThreatEntryTypeUnspecified,
        #[serde(rename = "URL")]
        #[doc = "A URL."]
        Url,
        #[serde(rename = "EXECUTABLE")]
        #[doc = "An executable program."]
        Executable,
        #[serde(rename = "IP_RANGE")]
        #[doc = "An IP range."]
        IpRange,
        #[serde(rename = "CHROME_EXTENSION")]
        #[doc = "Chrome extension."]
        ChromeExtension,
        #[serde(rename = "FILENAME")]
        #[doc = "Filename."]
        Filename,
        #[serde(rename = "CERT")]
        #[doc = "CERT"]
        Cert,
    }
    impl ::std::default::Default
        for GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatEntryTypeEnum
    {
        fn default() -> Self {
            Self::ThreatEntryTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The threat type posed by the list's entries."]
    pub enum GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum {
        #[serde(rename = "THREAT_TYPE_UNSPECIFIED")]
        #[doc = "Unknown."]
        ThreatTypeUnspecified,
        #[serde(rename = "MALWARE")]
        #[doc = "Malware threat type."]
        Malware,
        #[serde(rename = "SOCIAL_ENGINEERING")]
        #[doc = "Social engineering threat type."]
        SocialEngineering,
        #[serde(rename = "UNWANTED_SOFTWARE")]
        #[doc = "Unwanted software threat type."]
        UnwantedSoftware,
        #[serde(rename = "POTENTIALLY_HARMFUL_APPLICATION")]
        #[doc = "Potentially harmful application threat type."]
        PotentiallyHarmfulApplication,
        #[serde(rename = "SOCIAL_ENGINEERING_INTERNAL")]
        #[doc = "Social engineering threat type for internal use."]
        SocialEngineeringInternal,
        #[serde(rename = "API_ABUSE")]
        #[doc = "API abuse threat type."]
        ApiAbuse,
        #[serde(rename = "MALICIOUS_BINARY")]
        #[doc = "Malicious binary threat type."]
        MaliciousBinary,
        #[serde(rename = "CSD_WHITELIST")]
        #[doc = "Client side detection whitelist threat type."]
        CsdWhitelist,
        #[serde(rename = "CSD_DOWNLOAD_WHITELIST")]
        #[doc = "Client side download detection whitelist threat type."]
        CsdDownloadWhitelist,
        #[serde(rename = "CLIENT_INCIDENT")]
        #[doc = "Client incident threat type."]
        ClientIncident,
        #[serde(rename = "CLIENT_INCIDENT_WHITELIST")]
        #[doc = "Whitelist used when detecting client incident threats. This enum was never launched and should be re-used for the next list."]
        ClientIncidentWhitelist,
        #[serde(rename = "APK_MALWARE_OFFLINE")]
        #[doc = "List used for offline APK checks in PAM."]
        ApkMalwareOffline,
        #[serde(rename = "SUBRESOURCE_FILTER")]
        #[doc = "Patterns to be used for activating the subresource filter. Interstitial will not be shown for patterns from this list."]
        SubresourceFilter,
        #[serde(rename = "SUSPICIOUS")]
        #[doc = "Entities that are suspected to present a threat."]
        Suspicious,
        #[serde(rename = "TRICK_TO_BILL")]
        #[doc = "Trick-to-bill threat list."]
        TrickToBill,
        #[serde(rename = "HIGH_CONFIDENCE_ALLOWLIST")]
        #[doc = "Safe list to ship hashes of known safe URL expressions."]
        HighConfidenceAllowlist,
    }
    impl ::std::default::Default for GoogleSecuritySafebrowsingV4ThreatListDescriptorThreatTypeEnum {
        fn default() -> Self {
            Self::ThreatTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A match when checking a threat entry in the Safe Browsing threat lists."]
    pub struct GoogleSecuritySafebrowsingV4ThreatMatch {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cacheDuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cache lifetime for the returned match. Clients must not cache this response for more than this duration to avoid false positives."]
        pub cache_duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "platformType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The platform type matching this threat."]
        pub platform_type:
            ::std::option::Option<GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The threat matching this threat."]
        pub threat:
            ::std::option::Option<::std::boxed::Box<GoogleSecuritySafebrowsingV4ThreatEntry>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threatEntryMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional metadata associated with this threat."]
        pub threat_entry_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleSecuritySafebrowsingV4ThreatEntryMetadata>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threatEntryType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The threat entry type matching this threat."]
        pub threat_entry_type:
            ::std::option::Option<GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threatType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The threat type matching this threat."]
        pub threat_type:
            ::std::option::Option<GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum>,
    }
    impl GoogleSecuritySafebrowsingV4ThreatMatch {
        pub fn builder() -> GoogleSecuritySafebrowsingV4ThreatMatchBuilder {
            GoogleSecuritySafebrowsingV4ThreatMatchBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The platform type matching this threat."]
    pub enum GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum {
        #[serde(rename = "PLATFORM_TYPE_UNSPECIFIED")]
        #[doc = "Unknown platform."]
        PlatformTypeUnspecified,
        #[serde(rename = "WINDOWS")]
        #[doc = "Threat posed to Windows."]
        Windows,
        #[serde(rename = "LINUX")]
        #[doc = "Threat posed to Linux."]
        Linux,
        #[serde(rename = "ANDROID")]
        #[doc = "Threat posed to Android."]
        Android,
        #[serde(rename = "OSX")]
        #[doc = "Threat posed to OS X."]
        Osx,
        #[serde(rename = "IOS")]
        #[doc = "Threat posed to iOS."]
        Ios,
        #[serde(rename = "ANY_PLATFORM")]
        #[doc = "Threat posed to at least one of the defined platforms."]
        AnyPlatform,
        #[serde(rename = "ALL_PLATFORMS")]
        #[doc = "Threat posed to all defined platforms."]
        AllPlatforms,
        #[serde(rename = "CHROME")]
        #[doc = "Threat posed to Chrome."]
        Chrome,
    }
    impl ::std::default::Default for GoogleSecuritySafebrowsingV4ThreatMatchPlatformTypeEnum {
        fn default() -> Self {
            Self::PlatformTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The threat entry type matching this threat."]
    pub enum GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryTypeEnum {
        #[serde(rename = "THREAT_ENTRY_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified."]
        ThreatEntryTypeUnspecified,
        #[serde(rename = "URL")]
        #[doc = "A URL."]
        Url,
        #[serde(rename = "EXECUTABLE")]
        #[doc = "An executable program."]
        Executable,
        #[serde(rename = "IP_RANGE")]
        #[doc = "An IP range."]
        IpRange,
        #[serde(rename = "CHROME_EXTENSION")]
        #[doc = "Chrome extension."]
        ChromeExtension,
        #[serde(rename = "FILENAME")]
        #[doc = "Filename."]
        Filename,
        #[serde(rename = "CERT")]
        #[doc = "CERT"]
        Cert,
    }
    impl ::std::default::Default for GoogleSecuritySafebrowsingV4ThreatMatchThreatEntryTypeEnum {
        fn default() -> Self {
            Self::ThreatEntryTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The threat type matching this threat."]
    pub enum GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum {
        #[serde(rename = "THREAT_TYPE_UNSPECIFIED")]
        #[doc = "Unknown."]
        ThreatTypeUnspecified,
        #[serde(rename = "MALWARE")]
        #[doc = "Malware threat type."]
        Malware,
        #[serde(rename = "SOCIAL_ENGINEERING")]
        #[doc = "Social engineering threat type."]
        SocialEngineering,
        #[serde(rename = "UNWANTED_SOFTWARE")]
        #[doc = "Unwanted software threat type."]
        UnwantedSoftware,
        #[serde(rename = "POTENTIALLY_HARMFUL_APPLICATION")]
        #[doc = "Potentially harmful application threat type."]
        PotentiallyHarmfulApplication,
        #[serde(rename = "SOCIAL_ENGINEERING_INTERNAL")]
        #[doc = "Social engineering threat type for internal use."]
        SocialEngineeringInternal,
        #[serde(rename = "API_ABUSE")]
        #[doc = "API abuse threat type."]
        ApiAbuse,
        #[serde(rename = "MALICIOUS_BINARY")]
        #[doc = "Malicious binary threat type."]
        MaliciousBinary,
        #[serde(rename = "CSD_WHITELIST")]
        #[doc = "Client side detection whitelist threat type."]
        CsdWhitelist,
        #[serde(rename = "CSD_DOWNLOAD_WHITELIST")]
        #[doc = "Client side download detection whitelist threat type."]
        CsdDownloadWhitelist,
        #[serde(rename = "CLIENT_INCIDENT")]
        #[doc = "Client incident threat type."]
        ClientIncident,
        #[serde(rename = "CLIENT_INCIDENT_WHITELIST")]
        #[doc = "Whitelist used when detecting client incident threats. This enum was never launched and should be re-used for the next list."]
        ClientIncidentWhitelist,
        #[serde(rename = "APK_MALWARE_OFFLINE")]
        #[doc = "List used for offline APK checks in PAM."]
        ApkMalwareOffline,
        #[serde(rename = "SUBRESOURCE_FILTER")]
        #[doc = "Patterns to be used for activating the subresource filter. Interstitial will not be shown for patterns from this list."]
        SubresourceFilter,
        #[serde(rename = "SUSPICIOUS")]
        #[doc = "Entities that are suspected to present a threat."]
        Suspicious,
        #[serde(rename = "TRICK_TO_BILL")]
        #[doc = "Trick-to-bill threat list."]
        TrickToBill,
        #[serde(rename = "HIGH_CONFIDENCE_ALLOWLIST")]
        #[doc = "Safe list to ship hashes of known safe URL expressions."]
        HighConfidenceAllowlist,
    }
    impl ::std::default::Default for GoogleSecuritySafebrowsingV4ThreatMatchThreatTypeEnum {
        fn default() -> Self {
            Self::ThreatTypeUnspecified
        }
    }
}
