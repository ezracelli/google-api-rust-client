#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GenerateAccessTokenRequest {
    #[serde(rename = "delegates")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sequence of service accounts in a delegation chain. Each service account must be granted the `roles/iam.serviceAccountTokenCreator` role on its next service account in the chain. The last service account in the chain must be granted the `roles/iam.serviceAccountTokenCreator` role on the service account that is specified in the `name` field of the request. The delegates must have the following format: `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard character is required; replacing it with a project ID is invalid."]
    pub delegates: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "lifetime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The desired lifetime duration of the access token in seconds. By default, the maximum allowed value is 1 hour. To set a lifetime of up to 12 hours, you can add the service account as an allowed value in an Organization Policy that enforces the `constraints/iam.allowServiceAccountCredentialLifetimeExtension` constraint. See detailed instructions at https://cloud.google.com/iam/help/credentials/lifetime If a value is not specified, the token's lifetime will be set to a default value of 1 hour."]
    pub lifetime: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scope")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Code to identify the scopes to be included in the OAuth 2.0 access token. See https://developers.google.com/identity/protocols/googlescopes for more information. At least one value required."]
    pub scope: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GenerateAccessTokenResponse {
    #[serde(rename = "accessToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The OAuth 2.0 access token."]
    pub access_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expireTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token expiration time. The expiration time is always set."]
    pub expire_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GenerateIdTokenRequest {
    #[serde(rename = "audience")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The audience for the token, such as the API or account that this token grants access to."]
    pub audience: ::std::option::Option<::std::string::String>,
    #[serde(rename = "delegates")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sequence of service accounts in a delegation chain. Each service account must be granted the `roles/iam.serviceAccountTokenCreator` role on its next service account in the chain. The last service account in the chain must be granted the `roles/iam.serviceAccountTokenCreator` role on the service account that is specified in the `name` field of the request. The delegates must have the following format: `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard character is required; replacing it with a project ID is invalid."]
    pub delegates: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "includeEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Include the service account email in the token. If set to `true`, the token will contain `email` and `email_verified` claims."]
    pub include_email: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GenerateIdTokenResponse {
    #[serde(rename = "token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The OpenId Connect ID token."]
    pub token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignBlobRequest {
    #[serde(rename = "delegates")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sequence of service accounts in a delegation chain. Each service account must be granted the `roles/iam.serviceAccountTokenCreator` role on its next service account in the chain. The last service account in the chain must be granted the `roles/iam.serviceAccountTokenCreator` role on the service account that is specified in the `name` field of the request. The delegates must have the following format: `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard character is required; replacing it with a project ID is invalid."]
    pub delegates: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The bytes to sign."]
    pub payload: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignBlobResponse {
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the key used to sign the blob. The key used for signing will remain valid for at least 12 hours after the blob is signed. To verify the signature, you can retrieve the public key in several formats from the following endpoints: - RSA public key wrapped in an X.509 v3 certificate: `https://www.googleapis.com/service_accounts/v1/metadata/x509/{ACCOUNT_EMAIL}` - Raw key in JSON format: `https://www.googleapis.com/service_accounts/v1/metadata/raw/{ACCOUNT_EMAIL}` - JSON Web Key (JWK): `https://www.googleapis.com/service_accounts/v1/metadata/jwk/{ACCOUNT_EMAIL}`"]
    pub key_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "signedBlob")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The signature for the blob. Does not include the original blob. After the key pair referenced by the `key_id` response field expires, Google no longer exposes the public key that can be used to verify the blob. As a result, the receiver can no longer verify the signature."]
    pub signed_blob: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignJwtRequest {
    #[serde(rename = "delegates")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sequence of service accounts in a delegation chain. Each service account must be granted the `roles/iam.serviceAccountTokenCreator` role on its next service account in the chain. The last service account in the chain must be granted the `roles/iam.serviceAccountTokenCreator` role on the service account that is specified in the `name` field of the request. The delegates must have the following format: `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard character is required; replacing it with a project ID is invalid."]
    pub delegates: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The JWT payload to sign. Must be a serialized JSON object that contains a JWT Claims Set. For example: `{\"sub\": \"user@example.com\", \"iat\": 313435}` If the JWT Claims Set contains an expiration time (`exp`) claim, it must be an integer timestamp that is not in the past and no more than 12 hours in the future."]
    pub payload: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignJwtResponse {
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the key used to sign the JWT. The key used for signing will remain valid for at least 12 hours after the JWT is signed. To verify the signature, you can retrieve the public key in several formats from the following endpoints: - RSA public key wrapped in an X.509 v3 certificate: `https://www.googleapis.com/service_accounts/v1/metadata/x509/{ACCOUNT_EMAIL}` - Raw key in JSON format: `https://www.googleapis.com/service_accounts/v1/metadata/raw/{ACCOUNT_EMAIL}` - JSON Web Key (JWK): `https://www.googleapis.com/service_accounts/v1/metadata/jwk/{ACCOUNT_EMAIL}`"]
    pub key_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "signedJwt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The signed JWT. Contains the automatically generated header; the client-supplied payload; and the signature, which is generated using the key referenced by the `kid` field in the header. After the key pair referenced by the `key_id` response field expires, Google no longer exposes the public key that can be used to verify the JWT. As a result, the receiver can no longer verify the signature."]
    pub signed_jwt: ::std::option::Option<::std::string::String>,
}
