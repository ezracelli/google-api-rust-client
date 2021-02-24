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
    pub mod hashes {
        pub mod methods {
            pub mod search {
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
                    #[serde(rename = "hashPrefix")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A hash prefix, consisting of the most significant 4-32 bytes of a SHA256 hash. For JSON requests, this field is base64-encoded. Note that if this parameter is provided by a URI, it must be encoded using the web safe base64 variant (RFC 4648)."]
                    pub hash_prefix: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "threatTypes")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. The ThreatLists to search in. Multiple ThreatLists may be specified."]
                    pub threat_types: ::std::option::Option<QueryParametersThreatTypesEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Required. The ThreatLists to search in. Multiple ThreatLists may be specified."]
                pub enum QueryParametersThreatTypesEnum {
                    #[serde(rename = "THREAT_TYPE_UNSPECIFIED")]
                    #[doc = "No entries should match this threat type. This threat type is unused."]
                    ThreatTypeUnspecified,
                    #[serde(rename = "MALWARE")]
                    #[doc = "Malware targeting any platform."]
                    Malware,
                    #[serde(rename = "SOCIAL_ENGINEERING")]
                    #[doc = "Social engineering targeting any platform."]
                    SocialEngineering,
                    #[serde(rename = "UNWANTED_SOFTWARE")]
                    #[doc = "Unwanted software targeting any platform."]
                    UnwantedSoftware,
                }
                impl ::std::default::Default for QueryParametersThreatTypesEnum {
                    fn default() -> Self {
                        Self::ThreatTypeUnspecified
                    }
                }
            }
        }
    }
    pub mod operations {
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
                    #[serde(rename = "filter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The standard list filter."]
                    pub filter: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The standard list page size."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The standard list page token."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod threat_lists {
        pub mod methods {
            pub mod compute_diff {
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
                    #[serde(rename = "constraints.maxDatabaseEntries")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Sets the maximum number of entries that the client is willing to have in the local database. This should be a power of 2 between 2**10 and 2**20. If zero, no database size limit is set."]
                    pub constraints_max_database_entries:
                        ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "constraints.maxDiffEntries")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum size in number of entries. The diff will not contain more entries than this value. This should be a power of 2 between 2**10 and 2**20. If zero, no diff size limit is set."]
                    pub constraints_max_diff_entries: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "constraints.supportedCompressions")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The compression types supported by the client."]
                    pub constraints_supported_compressions:
                        ::std::option::Option<QueryParametersConstraintsSupportedCompressionsEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "threatType")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. The threat list to update. Only a single ThreatType should be specified per request. If you want to handle multiple ThreatTypes, you must make one request per ThreatType."]
                    pub threat_type: ::std::option::Option<QueryParametersThreatTypeEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "versionToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The current version token of the client for the requested list (the client version that was received from the last successful diff). If the client does not have a version token (this is the first time calling ComputeThreatListDiff), this may be left empty and a full database snapshot will be returned."]
                    pub version_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "The compression types supported by the client."]
                pub enum QueryParametersConstraintsSupportedCompressionsEnum {
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
                impl ::std::default::Default for QueryParametersConstraintsSupportedCompressionsEnum {
                    fn default() -> Self {
                        Self::CompressionTypeUnspecified
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Required. The threat list to update. Only a single ThreatType should be specified per request. If you want to handle multiple ThreatTypes, you must make one request per ThreatType."]
                pub enum QueryParametersThreatTypeEnum {
                    #[serde(rename = "THREAT_TYPE_UNSPECIFIED")]
                    #[doc = "No entries should match this threat type. This threat type is unused."]
                    ThreatTypeUnspecified,
                    #[serde(rename = "MALWARE")]
                    #[doc = "Malware targeting any platform."]
                    Malware,
                    #[serde(rename = "SOCIAL_ENGINEERING")]
                    #[doc = "Social engineering targeting any platform."]
                    SocialEngineering,
                    #[serde(rename = "UNWANTED_SOFTWARE")]
                    #[doc = "Unwanted software targeting any platform."]
                    UnwantedSoftware,
                }
                impl ::std::default::Default for QueryParametersThreatTypeEnum {
                    fn default() -> Self {
                        Self::ThreatTypeUnspecified
                    }
                }
            }
        }
    }
    pub mod uris {
        pub mod methods {
            pub mod search {
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
                    #[serde(rename = "threatTypes")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. The ThreatLists to search in. Multiple ThreatLists may be specified."]
                    pub threat_types: ::std::option::Option<QueryParametersThreatTypesEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "uri")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. The URI to be checked for matches."]
                    pub uri: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Required. The ThreatLists to search in. Multiple ThreatLists may be specified."]
                pub enum QueryParametersThreatTypesEnum {
                    #[serde(rename = "THREAT_TYPE_UNSPECIFIED")]
                    #[doc = "No entries should match this threat type. This threat type is unused."]
                    ThreatTypeUnspecified,
                    #[serde(rename = "MALWARE")]
                    #[doc = "Malware targeting any platform."]
                    Malware,
                    #[serde(rename = "SOCIAL_ENGINEERING")]
                    #[doc = "Social engineering targeting any platform."]
                    SocialEngineering,
                    #[serde(rename = "UNWANTED_SOFTWARE")]
                    #[doc = "Unwanted software targeting any platform."]
                    UnwantedSoftware,
                }
                impl ::std::default::Default for QueryParametersThreatTypesEnum {
                    fn default() -> Self {
                        Self::ThreatTypeUnspecified
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
    pub struct GoogleCloudWebriskV1ComputeThreatListDiffResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A set of entries to add to a local threat type's list."]
        pub additions:
            ::std::option::Option<::std::boxed::Box<GoogleCloudWebriskV1ThreatEntryAdditions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "checksum")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The expected SHA256 hash of the client state; that is, of the sorted list of all hashes present in the database after applying the provided diff. If the client state doesn't match the expected state, the client must discard this diff and retry later."]
        pub checksum: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudWebriskV1ComputeThreatListDiffResponseChecksum>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newVersionToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The new opaque client version token. This should be retained by the client and passed into the next call of ComputeThreatListDiff as 'version_token'. A separate version token should be stored and used for each threatList."]
        pub new_version_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recommendedNextDiff")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The soonest the client should wait before issuing any diff request. Querying sooner is unlikely to produce a meaningful diff. Waiting longer is acceptable considering the use case. If this field is not set clients may update as soon as they want."]
        pub recommended_next_diff: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "removals")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A set of entries to remove from a local threat type's list. This field may be empty."]
        pub removals:
            ::std::option::Option<::std::boxed::Box<GoogleCloudWebriskV1ThreatEntryRemovals>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of response. This may indicate that an action must be taken by the client when the response is received."]
        pub response_type: ::std::option::Option<
            GoogleCloudWebriskV1ComputeThreatListDiffResponseResponseTypeEnum,
        >,
    }
    impl GoogleCloudWebriskV1ComputeThreatListDiffResponse {
        pub fn builder() -> GoogleCloudWebriskV1ComputeThreatListDiffResponseBuilder {
            GoogleCloudWebriskV1ComputeThreatListDiffResponseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of response. This may indicate that an action must be taken by the client when the response is received."]
    pub enum GoogleCloudWebriskV1ComputeThreatListDiffResponseResponseTypeEnum {
        #[serde(rename = "RESPONSE_TYPE_UNSPECIFIED")]
        #[doc = "Unknown."]
        ResponseTypeUnspecified,
        #[serde(rename = "DIFF")]
        #[doc = "Partial updates are applied to the client's existing local database."]
        Diff,
        #[serde(rename = "RESET")]
        #[doc = "Full updates resets the client's entire local database. This means that either the client had no state, was seriously out-of-date, or the client is believed to be corrupt."]
        Reset,
    }
    impl ::std::default::Default for GoogleCloudWebriskV1ComputeThreatListDiffResponseResponseTypeEnum {
        fn default() -> Self {
            Self::ResponseTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The expected state of a client's local database."]
    pub struct GoogleCloudWebriskV1ComputeThreatListDiffResponseChecksum {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sha256")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The SHA256 hash of the client state; that is, of the sorted list of all hashes present in the database."]
        pub sha256: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudWebriskV1ComputeThreatListDiffResponseChecksum {
        pub fn builder() -> GoogleCloudWebriskV1ComputeThreatListDiffResponseChecksumBuilder {
            GoogleCloudWebriskV1ComputeThreatListDiffResponseChecksumBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The uncompressed threat entries in hash format. Hashes can be anywhere from 4 to 32 bytes in size. A large majority are 4 bytes, but some hashes are lengthened if they collide with the hash of a popular URI. Used for sending ThreatEntryAdditons to clients that do not support compression, or when sending non-4-byte hashes to clients that do support compression."]
    pub struct GoogleCloudWebriskV1RawHashes {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "prefixSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of bytes for each prefix encoded below. This field can be anywhere from 4 (shortest prefix) to 32 (full SHA256 hash). In practice this is almost always 4, except in exceptional circumstances."]
        pub prefix_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rawHashes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The hashes, in binary format, concatenated into one long string. Hashes are sorted in lexicographic order. For JSON API users, hashes are base64-encoded."]
        pub raw_hashes: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudWebriskV1RawHashes {
        pub fn builder() -> GoogleCloudWebriskV1RawHashesBuilder {
            GoogleCloudWebriskV1RawHashesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A set of raw indices to remove from a local list."]
    pub struct GoogleCloudWebriskV1RawIndices {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "indices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The indices to remove from a lexicographically-sorted local list."]
        pub indices: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    }
    impl GoogleCloudWebriskV1RawIndices {
        pub fn builder() -> GoogleCloudWebriskV1RawIndicesBuilder {
            GoogleCloudWebriskV1RawIndicesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Rice-Golomb encoded data. Used for sending compressed 4-byte hashes or compressed removal indices."]
    pub struct GoogleCloudWebriskV1RiceDeltaEncoding {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "encodedData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The encoded deltas that are encoded using the Golomb-Rice coder."]
        pub encoded_data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entryCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of entries that are delta encoded in the encoded data. If only a single integer was encoded, this will be zero and the single value will be stored in `first_value`."]
        pub entry_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firstValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The offset of the first entry in the encoded data, or, if only a single integer was encoded, that single integer's value. If the field is empty or missing, assume zero."]
        pub first_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "riceParameter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Golomb-Rice parameter, which is a number between 2 and 28. This field is missing (that is, zero) if `num_entries` is zero."]
        pub rice_parameter: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudWebriskV1RiceDeltaEncoding {
        pub fn builder() -> GoogleCloudWebriskV1RiceDeltaEncodingBuilder {
            GoogleCloudWebriskV1RiceDeltaEncodingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudWebriskV1SearchHashesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "negativeExpireTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For requested entities that did not match the threat list, how long to cache the response until."]
        pub negative_expire_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full hashes that matched the requested prefixes. The hash will be populated in the key."]
        pub threats: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudWebriskV1SearchHashesResponseThreatHash>>,
        >,
    }
    impl GoogleCloudWebriskV1SearchHashesResponse {
        pub fn builder() -> GoogleCloudWebriskV1SearchHashesResponseBuilder {
            GoogleCloudWebriskV1SearchHashesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains threat information on a matching hash."]
    pub struct GoogleCloudWebriskV1SearchHashesResponseThreatHash {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expireTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cache lifetime for the returned match. Clients must not cache this response past this timestamp to avoid false positives."]
        pub expire_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A 32 byte SHA256 hash. This field is in binary format. For JSON requests, hashes are base64-encoded."]
        pub hash: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threatTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ThreatList this threat belongs to. This must contain at least one entry."]
        pub threat_types: ::std::option::Option<
            ::std::vec::Vec<GoogleCloudWebriskV1SearchHashesResponseThreatHashThreatTypesEnum>,
        >,
    }
    impl GoogleCloudWebriskV1SearchHashesResponseThreatHash {
        pub fn builder() -> GoogleCloudWebriskV1SearchHashesResponseThreatHashBuilder {
            GoogleCloudWebriskV1SearchHashesResponseThreatHashBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GoogleCloudWebriskV1SearchHashesResponseThreatHashThreatTypesEnum {
        #[serde(rename = "THREAT_TYPE_UNSPECIFIED")]
        #[doc = "No entries should match this threat type. This threat type is unused."]
        ThreatTypeUnspecified,
        #[serde(rename = "MALWARE")]
        #[doc = "Malware targeting any platform."]
        Malware,
        #[serde(rename = "SOCIAL_ENGINEERING")]
        #[doc = "Social engineering targeting any platform."]
        SocialEngineering,
        #[serde(rename = "UNWANTED_SOFTWARE")]
        #[doc = "Unwanted software targeting any platform."]
        UnwantedSoftware,
    }
    impl ::std::default::Default for GoogleCloudWebriskV1SearchHashesResponseThreatHashThreatTypesEnum {
        fn default() -> Self {
            Self::ThreatTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudWebriskV1SearchUrisResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The threat list matches. This may be empty if the URI is on no list."]
        pub threat: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudWebriskV1SearchUrisResponseThreatUri>,
        >,
    }
    impl GoogleCloudWebriskV1SearchUrisResponse {
        pub fn builder() -> GoogleCloudWebriskV1SearchUrisResponseBuilder {
            GoogleCloudWebriskV1SearchUrisResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains threat information on a matching uri."]
    pub struct GoogleCloudWebriskV1SearchUrisResponseThreatUri {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expireTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cache lifetime for the returned match. Clients must not cache this response past this timestamp to avoid false positives."]
        pub expire_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threatTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ThreatList this threat belongs to."]
        pub threat_types: ::std::option::Option<
            ::std::vec::Vec<GoogleCloudWebriskV1SearchUrisResponseThreatUriThreatTypesEnum>,
        >,
    }
    impl GoogleCloudWebriskV1SearchUrisResponseThreatUri {
        pub fn builder() -> GoogleCloudWebriskV1SearchUrisResponseThreatUriBuilder {
            GoogleCloudWebriskV1SearchUrisResponseThreatUriBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GoogleCloudWebriskV1SearchUrisResponseThreatUriThreatTypesEnum {
        #[serde(rename = "THREAT_TYPE_UNSPECIFIED")]
        #[doc = "No entries should match this threat type. This threat type is unused."]
        ThreatTypeUnspecified,
        #[serde(rename = "MALWARE")]
        #[doc = "Malware targeting any platform."]
        Malware,
        #[serde(rename = "SOCIAL_ENGINEERING")]
        #[doc = "Social engineering targeting any platform."]
        SocialEngineering,
        #[serde(rename = "UNWANTED_SOFTWARE")]
        #[doc = "Unwanted software targeting any platform."]
        UnwantedSoftware,
    }
    impl ::std::default::Default for GoogleCloudWebriskV1SearchUrisResponseThreatUriThreatTypesEnum {
        fn default() -> Self {
            Self::ThreatTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Wraps a URI that might be displaying malicious content."]
    pub struct GoogleCloudWebriskV1Submission {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "threatTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ThreatTypes found to be associated with the submitted URI after reviewing it. This may be empty if the URI was not added to any list."]
        pub threat_types:
            ::std::option::Option<::std::vec::Vec<GoogleCloudWebriskV1SubmissionThreatTypesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The URI that is being reported for malicious content to be analyzed."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudWebriskV1Submission {
        pub fn builder() -> GoogleCloudWebriskV1SubmissionBuilder {
            GoogleCloudWebriskV1SubmissionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GoogleCloudWebriskV1SubmissionThreatTypesEnum {
        #[serde(rename = "THREAT_TYPE_UNSPECIFIED")]
        #[doc = "No entries should match this threat type. This threat type is unused."]
        ThreatTypeUnspecified,
        #[serde(rename = "MALWARE")]
        #[doc = "Malware targeting any platform."]
        Malware,
        #[serde(rename = "SOCIAL_ENGINEERING")]
        #[doc = "Social engineering targeting any platform."]
        SocialEngineering,
        #[serde(rename = "UNWANTED_SOFTWARE")]
        #[doc = "Unwanted software targeting any platform."]
        UnwantedSoftware,
    }
    impl ::std::default::Default for GoogleCloudWebriskV1SubmissionThreatTypesEnum {
        fn default() -> Self {
            Self::ThreatTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to send a potentially malicious URI to WebRisk."]
    pub struct GoogleCloudWebriskV1SubmitUriRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "submission")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The submission that contains the URI to be scanned."]
        pub submission: ::std::option::Option<::std::boxed::Box<GoogleCloudWebriskV1Submission>>,
    }
    impl GoogleCloudWebriskV1SubmitUriRequest {
        pub fn builder() -> GoogleCloudWebriskV1SubmitUriRequestBuilder {
            GoogleCloudWebriskV1SubmitUriRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains the set of entries to add to a local database. May contain a combination of compressed and raw data in a single response."]
    pub struct GoogleCloudWebriskV1ThreatEntryAdditions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rawHashes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The raw SHA256-formatted entries. Repeated to allow returning sets of hashes with different prefix sizes."]
        pub raw_hashes: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudWebriskV1RawHashes>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "riceHashes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The encoded 4-byte prefixes of SHA256-formatted entries, using a Golomb-Rice encoding. The hashes are converted to uint32, sorted in ascending order, then delta encoded and stored as encoded_data."]
        pub rice_hashes:
            ::std::option::Option<::std::boxed::Box<GoogleCloudWebriskV1RiceDeltaEncoding>>,
    }
    impl GoogleCloudWebriskV1ThreatEntryAdditions {
        pub fn builder() -> GoogleCloudWebriskV1ThreatEntryAdditionsBuilder {
            GoogleCloudWebriskV1ThreatEntryAdditionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains the set of entries to remove from a local database."]
    pub struct GoogleCloudWebriskV1ThreatEntryRemovals {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rawIndices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The raw removal indices for a local list."]
        pub raw_indices: ::std::option::Option<::std::boxed::Box<GoogleCloudWebriskV1RawIndices>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "riceIndices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The encoded local, lexicographically-sorted list indices, using a Golomb-Rice encoding. Used for sending compressed removal indices. The removal indices (uint32) are sorted in ascending order, then delta encoded and stored as encoded_data."]
        pub rice_indices:
            ::std::option::Option<::std::boxed::Box<GoogleCloudWebriskV1RiceDeltaEncoding>>,
    }
    impl GoogleCloudWebriskV1ThreatEntryRemovals {
        pub fn builder() -> GoogleCloudWebriskV1ThreatEntryRemovalsBuilder {
            GoogleCloudWebriskV1ThreatEntryRemovalsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for Operations.CancelOperation."]
    pub struct GoogleLongrunningCancelOperationRequest {}
    impl GoogleLongrunningCancelOperationRequest {
        pub fn builder() -> GoogleLongrunningCancelOperationRequestBuilder {
            GoogleLongrunningCancelOperationRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Operations.ListOperations."]
    pub struct GoogleLongrunningListOperationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The standard List next-page token."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of operations that matches the specified filter in the request."]
        pub operations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleLongrunningOperation>>>,
    }
    impl GoogleLongrunningListOperationsResponse {
        pub fn builder() -> GoogleLongrunningListOperationsResponseBuilder {
            GoogleLongrunningListOperationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This resource represents a long-running operation that is the result of a network API call."]
    pub struct GoogleLongrunningOperation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "done")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
        pub done: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error result of the operation in case of failure or cancellation."]
        pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
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
    impl GoogleLongrunningOperation {
        pub fn builder() -> GoogleLongrunningOperationBuilder {
            GoogleLongrunningOperationBuilder::default()
        }
    }
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
    #[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
    pub struct GoogleRpcStatus {
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
    impl GoogleRpcStatus {
        pub fn builder() -> GoogleRpcStatusBuilder {
            GoogleRpcStatusBuilder::default()
        }
    }
}
