#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes an android app asset."]
pub struct AndroidAppAsset {
    #[serde(rename = "certificate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Because there is no global enforcement of package name uniqueness, we also require a signing certificate, which in combination with the package name uniquely identifies an app. Some apps' signing keys are rotated, so they may be signed by different keys over time. We treat these as distinct assets, since we use (package name, cert) as the unique ID. This should not normally pose any problems as both versions of the app will make the same or similar statements. Other assets making statements about the app will have to be updated when a key is rotated, however. (Note that the syntaxes for publishing and querying for statements contain syntactic sugar to easily let you specify apps that are known by multiple certificates.) REQUIRED"]
    pub certificate: ::std::option::Option<::std::boxed::Box<CertificateInfo>>,
    #[serde(rename = "packageName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Android App assets are naturally identified by their Java package name. For example, the Google Maps app uses the package name `com.google.android.apps.maps`. REQUIRED"]
    pub package_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Uniquely identifies an asset. A digital asset is an identifiable and addressable online entity that typically provides some service or content. Examples of assets are websites, Android apps, Twitter feeds, and Plus Pages."]
pub struct Asset {
    #[serde(rename = "androidApp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set if this is an Android App asset."]
    pub android_app: ::std::option::Option<::std::boxed::Box<AndroidAppAsset>>,
    #[serde(rename = "web")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set if this is a web asset."]
    pub web: ::std::option::Option<::std::boxed::Box<WebAsset>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes an X509 certificate."]
pub struct CertificateInfo {
    #[serde(rename = "sha256Fingerprint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The uppercase SHA-265 fingerprint of the certificate. From the PEM certificate, it can be acquired like this: $ keytool -printcert -file $CERTFILE | grep SHA256: SHA256: 14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83: \\ 42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5 or like this: $ openssl x509 -in $CERTFILE -noout -fingerprint -sha256 SHA256 Fingerprint=14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64: \\ 16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5 In this example, the contents of this field would be `14:6D:E9:83:C5:73: 06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF: 44:E5`. If these tools are not available to you, you can convert the PEM certificate into the DER format, compute the SHA-256 hash of that string and represent the result as a hexstring (that is, uppercase hexadecimal representations of each octet, separated by colons)."]
    pub sha256_fingerprint: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for the CheckAssetLinks call."]
pub struct CheckResponse {
    #[serde(rename = "debugString")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Human-readable message containing information intended to help end users understand, reproduce and debug the result. The message will be in English and we are currently not planning to offer any translations. Please note that no guarantees are made about the contents or format of this string. Any aspect of it may be subject to change without notice. You should not attempt to programmatically parse this data. For programmatic access, use the error_code field below."]
    pub debug_string: ::std::option::Option<::std::string::String>,
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Error codes that describe the result of the Check operation."]
    pub error_code: ::std::option::Option<::std::vec::Vec<CheckResponseErrorCodeEnum>>,
    #[serde(rename = "linked")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Set to true if the assets specified in the request are linked by the relation specified in the request."]
    pub linked: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "maxAge")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "From serving time, how much longer the response should be considered valid barring further updates. REQUIRED"]
    pub max_age: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum CheckResponseErrorCodeEnum {
    #[serde(rename = "ERROR_CODE_UNSPECIFIED")]
    #[doc = ""]
    ErrorCodeUnspecified,
    #[serde(rename = "ERROR_CODE_INVALID_QUERY")]
    #[doc = "Unable to parse query."]
    ErrorCodeInvalidQuery,
    #[serde(rename = "ERROR_CODE_FETCH_ERROR")]
    #[doc = "Unable to fetch the asset links data."]
    ErrorCodeFetchError,
    #[serde(rename = "ERROR_CODE_FAILED_SSL_VALIDATION")]
    #[doc = "Invalid HTTPS certificate ."]
    ErrorCodeFailedSslValidation,
    #[serde(rename = "ERROR_CODE_REDIRECT")]
    #[doc = "HTTP redirects (e.g, 301) are not allowed."]
    ErrorCodeRedirect,
    #[serde(rename = "ERROR_CODE_TOO_LARGE")]
    #[doc = "Asset links data exceeds maximum size."]
    ErrorCodeTooLarge,
    #[serde(rename = "ERROR_CODE_MALFORMED_HTTP_RESPONSE")]
    #[doc = "Can't parse HTTP response."]
    ErrorCodeMalformedHttpResponse,
    #[serde(rename = "ERROR_CODE_WRONG_CONTENT_TYPE")]
    #[doc = "HTTP Content-type should be application/json."]
    ErrorCodeWrongContentType,
    #[serde(rename = "ERROR_CODE_MALFORMED_CONTENT")]
    #[doc = "JSON content is malformed."]
    ErrorCodeMalformedContent,
    #[serde(rename = "ERROR_CODE_SECURE_ASSET_INCLUDES_INSECURE")]
    #[doc = "A secure asset includes an insecure asset (security downgrade)."]
    ErrorCodeSecureAssetIncludesInsecure,
    #[serde(rename = "ERROR_CODE_FETCH_BUDGET_EXHAUSTED")]
    #[doc = "Too many includes (maybe a loop)."]
    ErrorCodeFetchBudgetExhausted,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for the List call."]
pub struct ListResponse {
    #[serde(rename = "debugString")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Human-readable message containing information intended to help end users understand, reproduce and debug the result. The message will be in English and we are currently not planning to offer any translations. Please note that no guarantees are made about the contents or format of this string. Any aspect of it may be subject to change without notice. You should not attempt to programmatically parse this data. For programmatic access, use the error_code field below."]
    pub debug_string: ::std::option::Option<::std::string::String>,
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Error codes that describe the result of the List operation."]
    pub error_code: ::std::option::Option<::std::vec::Vec<ListResponseErrorCodeEnum>>,
    #[serde(rename = "maxAge")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "From serving time, how much longer the response should be considered valid barring further updates. REQUIRED"]
    pub max_age: ::std::option::Option<::std::string::String>,
    #[serde(rename = "statements")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of all the matching statements that have been found."]
    pub statements: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Statement>>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum ListResponseErrorCodeEnum {
    #[serde(rename = "ERROR_CODE_UNSPECIFIED")]
    #[doc = ""]
    ErrorCodeUnspecified,
    #[serde(rename = "ERROR_CODE_INVALID_QUERY")]
    #[doc = "Unable to parse query."]
    ErrorCodeInvalidQuery,
    #[serde(rename = "ERROR_CODE_FETCH_ERROR")]
    #[doc = "Unable to fetch the asset links data."]
    ErrorCodeFetchError,
    #[serde(rename = "ERROR_CODE_FAILED_SSL_VALIDATION")]
    #[doc = "Invalid HTTPS certificate ."]
    ErrorCodeFailedSslValidation,
    #[serde(rename = "ERROR_CODE_REDIRECT")]
    #[doc = "HTTP redirects (e.g, 301) are not allowed."]
    ErrorCodeRedirect,
    #[serde(rename = "ERROR_CODE_TOO_LARGE")]
    #[doc = "Asset links data exceeds maximum size."]
    ErrorCodeTooLarge,
    #[serde(rename = "ERROR_CODE_MALFORMED_HTTP_RESPONSE")]
    #[doc = "Can't parse HTTP response."]
    ErrorCodeMalformedHttpResponse,
    #[serde(rename = "ERROR_CODE_WRONG_CONTENT_TYPE")]
    #[doc = "HTTP Content-type should be application/json."]
    ErrorCodeWrongContentType,
    #[serde(rename = "ERROR_CODE_MALFORMED_CONTENT")]
    #[doc = "JSON content is malformed."]
    ErrorCodeMalformedContent,
    #[serde(rename = "ERROR_CODE_SECURE_ASSET_INCLUDES_INSECURE")]
    #[doc = "A secure asset includes an insecure asset (security downgrade)."]
    ErrorCodeSecureAssetIncludesInsecure,
    #[serde(rename = "ERROR_CODE_FETCH_BUDGET_EXHAUSTED")]
    #[doc = "Too many includes (maybe a loop)."]
    ErrorCodeFetchBudgetExhausted,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes a reliable statement that has been made about the relationship between a source asset and a target asset. Statements are always made by the source asset, either directly or by delegating to a statement list that is stored elsewhere. For more detailed definitions of statements and assets, please refer to our [API documentation landing page](/digital-asset-links/v1/getting-started)."]
pub struct Statement {
    #[serde(rename = "relation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The relation identifies the use of the statement as intended by the source asset's owner (that is, the person or entity who issued the statement). Every complete statement has a relation. We identify relations with strings of the format `/`, where `` must be one of a set of pre-defined purpose categories, and `` is a free-form lowercase alphanumeric string that describes the specific use case of the statement. Refer to [our API documentation](/digital-asset-links/v1/relation-strings) for the current list of supported relations. Example: `delegate_permission/common.handle_all_urls` REQUIRED"]
    pub relation: ::std::option::Option<::std::string::String>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Every statement has a source asset. REQUIRED"]
    pub source: ::std::option::Option<::std::boxed::Box<Asset>>,
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Every statement has a target asset. REQUIRED"]
    pub target: ::std::option::Option<::std::boxed::Box<Asset>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes a web asset."]
pub struct WebAsset {
    #[serde(rename = "site")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Web assets are identified by a URL that contains only the scheme, hostname and port parts. The format is http[s]://[:] Hostnames must be fully qualified: they must end in a single period (\"`.`\"). Only the schemes \"http\" and \"https\" are currently allowed. Port numbers are given as a decimal number, and they must be omitted if the standard port numbers are used: 80 for http and 443 for https. We call this limited URL the \"site\". All URLs that share the same scheme, hostname and port are considered to be a part of the site and thus belong to the web asset. Example: the asset with the site `https://www.google.com` contains all these URLs: * `https://www.google.com/` * `https://www.google.com:443/` * `https://www.google.com/foo` * `https://www.google.com/foo?bar` * `https://www.google.com/foo#bar` * `https://user@password:www.google.com/` But it does not contain these URLs: * `http://www.google.com/` (wrong scheme) * `https://google.com/` (hostname does not match) * `https://www.google.com:444/` (port does not match) REQUIRED"]
    pub site: ::std::option::Option<::std::string::String>,
}
