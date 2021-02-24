#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Result message for VerifiedAccess.CreateChallenge."]
pub struct Challenge {
    #[serde(rename = "alternativeChallenge")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Challenge generated with the old signing key (this will only be present during key rotation)"]
    pub alternative_challenge: ::std::option::Option<::std::boxed::Box<SignedData>>,
    #[serde(rename = "challenge")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Generated challenge"]
    pub challenge: ::std::option::Option<::std::boxed::Box<SignedData>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The wrapper message of any data and its signature."]
pub struct SignedData {
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The data to be signed."]
    pub data: ::std::option::Option<::std::string::String>,
    #[serde(rename = "signature")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The signature of the data field."]
    pub signature: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "signed ChallengeResponse"]
pub struct VerifyChallengeResponseRequest {
    #[serde(rename = "challengeResponse")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The generated response to the challenge"]
    pub challenge_response: ::std::option::Option<::std::boxed::Box<SignedData>>,
    #[serde(rename = "expectedIdentity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Service can optionally provide identity information about the device or user associated with the key. For an EMK, this value is the enrolled domain. For an EUK, this value is the user's email address. If present, this value will be checked against contents of the response, and verification will fail if there is no match."]
    pub expected_identity: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Result message for VerifiedAccess.VerifyChallengeResponse."]
pub struct VerifyChallengeResponseResult {
    #[serde(rename = "deviceEnrollmentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Device enrollment id is returned in this field (for the machine response only)."]
    pub device_enrollment_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "devicePermanentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Device permanent id is returned in this field (for the machine response only)."]
    pub device_permanent_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "signedPublicKeyAndChallenge")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Certificate Signing Request (in the SPKAC format, base64 encoded) is returned in this field. This field will be set only if device has included CSR in its challenge response. (the option to include CSR is now available for both user and machine responses)"]
    pub signed_public_key_and_challenge: ::std::option::Option<::std::string::String>,
    #[serde(rename = "verificationOutput")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For EMCert check, device permanent id is returned here. For EUCert check, signed_public_key_and_challenge [base64 encoded] is returned if present, otherwise empty string is returned. This field is deprecated, please use device_permanent_id or signed_public_key_and_challenge fields."]
    pub verification_output: ::std::option::Option<::std::string::String>,
}
