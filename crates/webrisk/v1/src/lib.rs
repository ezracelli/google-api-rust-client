#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudWebriskV1ComputeThreatListDiffResponse {
    #[serde(rename = "additions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A set of entries to add to a local threat type's list."]
    pub additions:
        ::std::option::Option<::std::boxed::Box<GoogleCloudWebriskV1ThreatEntryAdditions>>,
    #[serde(rename = "checksum")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The expected SHA256 hash of the client state; that is, of the sorted list of all hashes present in the database after applying the provided diff. If the client state doesn't match the expected state, the client must discard this diff and retry later."]
    pub checksum: ::std::option::Option<
        ::std::boxed::Box<GoogleCloudWebriskV1ComputeThreatListDiffResponseChecksum>,
    >,
    #[serde(rename = "newVersionToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The new opaque client version token. This should be retained by the client and passed into the next call of ComputeThreatListDiff as 'version_token'. A separate version token should be stored and used for each threatList."]
    pub new_version_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "recommendedNextDiff")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The soonest the client should wait before issuing any diff request. Querying sooner is unlikely to produce a meaningful diff. Waiting longer is acceptable considering the use case. If this field is not set clients may update as soon as they want."]
    pub recommended_next_diff: ::std::option::Option<::std::string::String>,
    #[serde(rename = "removals")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A set of entries to remove from a local threat type's list. This field may be empty."]
    pub removals: ::std::option::Option<::std::boxed::Box<GoogleCloudWebriskV1ThreatEntryRemovals>>,
    #[serde(rename = "responseType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of response. This may indicate that an action must be taken by the client when the response is received."]
    pub response_type:
        ::std::option::Option<GoogleCloudWebriskV1ComputeThreatListDiffResponseResponseTypeEnum>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The expected state of a client's local database."]
pub struct GoogleCloudWebriskV1ComputeThreatListDiffResponseChecksum {
    #[serde(rename = "sha256")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The SHA256 hash of the client state; that is, of the sorted list of all hashes present in the database."]
    pub sha256: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The uncompressed threat entries in hash format. Hashes can be anywhere from 4 to 32 bytes in size. A large majority are 4 bytes, but some hashes are lengthened if they collide with the hash of a popular URI. Used for sending ThreatEntryAdditons to clients that do not support compression, or when sending non-4-byte hashes to clients that do support compression."]
pub struct GoogleCloudWebriskV1RawHashes {
    #[serde(rename = "prefixSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of bytes for each prefix encoded below. This field can be anywhere from 4 (shortest prefix) to 32 (full SHA256 hash). In practice this is almost always 4, except in exceptional circumstances."]
    pub prefix_size: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "rawHashes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The hashes, in binary format, concatenated into one long string. Hashes are sorted in lexicographic order. For JSON API users, hashes are base64-encoded."]
    pub raw_hashes: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A set of raw indices to remove from a local list."]
pub struct GoogleCloudWebriskV1RawIndices {
    #[serde(rename = "indices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The indices to remove from a lexicographically-sorted local list."]
    pub indices: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The Rice-Golomb encoded data. Used for sending compressed 4-byte hashes or compressed removal indices."]
pub struct GoogleCloudWebriskV1RiceDeltaEncoding {
    #[serde(rename = "encodedData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The encoded deltas that are encoded using the Golomb-Rice coder."]
    pub encoded_data: ::std::option::Option<::std::string::String>,
    #[serde(rename = "entryCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of entries that are delta encoded in the encoded data. If only a single integer was encoded, this will be zero and the single value will be stored in `first_value`."]
    pub entry_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "firstValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The offset of the first entry in the encoded data, or, if only a single integer was encoded, that single integer's value. If the field is empty or missing, assume zero."]
    pub first_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "riceParameter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Golomb-Rice parameter, which is a number between 2 and 28. This field is missing (that is, zero) if `num_entries` is zero."]
    pub rice_parameter: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudWebriskV1SearchHashesResponse {
    #[serde(rename = "negativeExpireTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For requested entities that did not match the threat list, how long to cache the response until."]
    pub negative_expire_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "threats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full hashes that matched the requested prefixes. The hash will be populated in the key."]
    pub threats: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleCloudWebriskV1SearchHashesResponseThreatHash>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains threat information on a matching hash."]
pub struct GoogleCloudWebriskV1SearchHashesResponseThreatHash {
    #[serde(rename = "expireTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The cache lifetime for the returned match. Clients must not cache this response past this timestamp to avoid false positives."]
    pub expire_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hash")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A 32 byte SHA256 hash. This field is in binary format. For JSON requests, hashes are base64-encoded."]
    pub hash: ::std::option::Option<::std::string::String>,
    #[serde(rename = "threatTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ThreatList this threat belongs to. This must contain at least one entry."]
    pub threat_types: ::std::option::Option<
        ::std::vec::Vec<GoogleCloudWebriskV1SearchHashesResponseThreatHashThreatTypesEnum>,
    >,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoogleCloudWebriskV1SearchUrisResponse {
    #[serde(rename = "threat")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The threat list matches. This may be empty if the URI is on no list."]
    pub threat:
        ::std::option::Option<::std::boxed::Box<GoogleCloudWebriskV1SearchUrisResponseThreatUri>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains threat information on a matching uri."]
pub struct GoogleCloudWebriskV1SearchUrisResponseThreatUri {
    #[serde(rename = "expireTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The cache lifetime for the returned match. Clients must not cache this response past this timestamp to avoid false positives."]
    pub expire_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "threatTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ThreatList this threat belongs to."]
    pub threat_types: ::std::option::Option<
        ::std::vec::Vec<GoogleCloudWebriskV1SearchUrisResponseThreatUriThreatTypesEnum>,
    >,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Wraps a URI that might be displaying malicious content."]
pub struct GoogleCloudWebriskV1Submission {
    #[serde(rename = "threatTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ThreatTypes found to be associated with the submitted URI after reviewing it. This may be empty if the URI was not added to any list."]
    pub threat_types:
        ::std::option::Option<::std::vec::Vec<GoogleCloudWebriskV1SubmissionThreatTypesEnum>>,
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The URI that is being reported for malicious content to be analyzed."]
    pub uri: ::std::option::Option<::std::string::String>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request to send a potentially malicious URI to WebRisk."]
pub struct GoogleCloudWebriskV1SubmitUriRequest {
    #[serde(rename = "submission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The submission that contains the URI to be scanned."]
    pub submission: ::std::option::Option<::std::boxed::Box<GoogleCloudWebriskV1Submission>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains the set of entries to add to a local database. May contain a combination of compressed and raw data in a single response."]
pub struct GoogleCloudWebriskV1ThreatEntryAdditions {
    #[serde(rename = "rawHashes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The raw SHA256-formatted entries. Repeated to allow returning sets of hashes with different prefix sizes."]
    pub raw_hashes:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudWebriskV1RawHashes>>>,
    #[serde(rename = "riceHashes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The encoded 4-byte prefixes of SHA256-formatted entries, using a Golomb-Rice encoding. The hashes are converted to uint32, sorted in ascending order, then delta encoded and stored as encoded_data."]
    pub rice_hashes:
        ::std::option::Option<::std::boxed::Box<GoogleCloudWebriskV1RiceDeltaEncoding>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Contains the set of entries to remove from a local database."]
pub struct GoogleCloudWebriskV1ThreatEntryRemovals {
    #[serde(rename = "rawIndices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The raw removal indices for a local list."]
    pub raw_indices: ::std::option::Option<::std::boxed::Box<GoogleCloudWebriskV1RawIndices>>,
    #[serde(rename = "riceIndices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The encoded local, lexicographically-sorted list indices, using a Golomb-Rice encoding. Used for sending compressed removal indices. The removal indices (uint32) are sorted in ascending order, then delta encoded and stored as encoded_data."]
    pub rice_indices:
        ::std::option::Option<::std::boxed::Box<GoogleCloudWebriskV1RiceDeltaEncoding>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for Operations.CancelOperation."]
pub struct GoogleLongrunningCancelOperationRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for Operations.ListOperations."]
pub struct GoogleLongrunningListOperationsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The standard List next-page token."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of operations that matches the specified filter in the request."]
    pub operations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleLongrunningOperation>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This resource represents a long-running operation that is the result of a network API call."]
pub struct GoogleLongrunningOperation {
    #[serde(rename = "done")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
    pub done: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The error result of the operation in case of failure or cancellation."]
    pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any."]
    pub metadata: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "response")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`."]
    pub response: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct GoogleProtobufEmpty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
pub struct GoogleRpcStatus {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status code, which should be an enum value of google.rpc.Code."]
    pub code: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of messages that carry the error details. There is a common set of message types for APIs to use."]
    pub details: ::std::option::Option<
        ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    >,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client."]
    pub message: ::std::option::Option<::std::string::String>,
}
