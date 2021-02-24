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
    pub mod assetlinks {
        pub mod methods {
            pub mod check {
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
                    #[serde(rename = "relation")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Query string for the relation. We identify relations with strings of the format `/`, where `` must be one of a set of pre-defined purpose categories, and `` is a free-form lowercase alphanumeric string that describes the specific use case of the statement. Refer to [our API documentation](/digital-asset-links/v1/relation-strings) for the current list of supported relations. For a query to match an asset link, both the query's and the asset link's relation strings must match exactly. Example: A query with relation `delegate_permission/common.handle_all_urls` matches an asset link with relation `delegate_permission/common.handle_all_urls`."]
                    pub relation: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "source.androidApp.certificate.sha256Fingerprint")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The uppercase SHA-265 fingerprint of the certificate. From the PEM certificate, it can be acquired like this: $ keytool -printcert -file $CERTFILE | grep SHA256: SHA256: 14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83: \\ 42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5 or like this: $ openssl x509 -in $CERTFILE -noout -fingerprint -sha256 SHA256 Fingerprint=14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64: \\ 16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5 In this example, the contents of this field would be `14:6D:E9:83:C5:73: 06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF: 44:E5`. If these tools are not available to you, you can convert the PEM certificate into the DER format, compute the SHA-256 hash of that string and represent the result as a hexstring (that is, uppercase hexadecimal representations of each octet, separated by colons)."]
                    pub source_android_app_certificate_sha256_fingerprint:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "source.androidApp.packageName")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Android App assets are naturally identified by their Java package name. For example, the Google Maps app uses the package name `com.google.android.apps.maps`. REQUIRED"]
                    pub source_android_app_package_name:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "source.web.site")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Web assets are identified by a URL that contains only the scheme, hostname and port parts. The format is http[s]://[:] Hostnames must be fully qualified: they must end in a single period (\"`.`\"). Only the schemes \"http\" and \"https\" are currently allowed. Port numbers are given as a decimal number, and they must be omitted if the standard port numbers are used: 80 for http and 443 for https. We call this limited URL the \"site\". All URLs that share the same scheme, hostname and port are considered to be a part of the site and thus belong to the web asset. Example: the asset with the site `https://www.google.com` contains all these URLs: * `https://www.google.com/` * `https://www.google.com:443/` * `https://www.google.com/foo` * `https://www.google.com/foo?bar` * `https://www.google.com/foo#bar` * `https://user@password:www.google.com/` But it does not contain these URLs: * `http://www.google.com/` (wrong scheme) * `https://google.com/` (hostname does not match) * `https://www.google.com:444/` (port does not match) REQUIRED"]
                    pub source_web_site: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "target.androidApp.certificate.sha256Fingerprint")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The uppercase SHA-265 fingerprint of the certificate. From the PEM certificate, it can be acquired like this: $ keytool -printcert -file $CERTFILE | grep SHA256: SHA256: 14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83: \\ 42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5 or like this: $ openssl x509 -in $CERTFILE -noout -fingerprint -sha256 SHA256 Fingerprint=14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64: \\ 16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5 In this example, the contents of this field would be `14:6D:E9:83:C5:73: 06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF: 44:E5`. If these tools are not available to you, you can convert the PEM certificate into the DER format, compute the SHA-256 hash of that string and represent the result as a hexstring (that is, uppercase hexadecimal representations of each octet, separated by colons)."]
                    pub target_android_app_certificate_sha256_fingerprint:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "target.androidApp.packageName")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Android App assets are naturally identified by their Java package name. For example, the Google Maps app uses the package name `com.google.android.apps.maps`. REQUIRED"]
                    pub target_android_app_package_name:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "target.web.site")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Web assets are identified by a URL that contains only the scheme, hostname and port parts. The format is http[s]://[:] Hostnames must be fully qualified: they must end in a single period (\"`.`\"). Only the schemes \"http\" and \"https\" are currently allowed. Port numbers are given as a decimal number, and they must be omitted if the standard port numbers are used: 80 for http and 443 for https. We call this limited URL the \"site\". All URLs that share the same scheme, hostname and port are considered to be a part of the site and thus belong to the web asset. Example: the asset with the site `https://www.google.com` contains all these URLs: * `https://www.google.com/` * `https://www.google.com:443/` * `https://www.google.com/foo` * `https://www.google.com/foo?bar` * `https://www.google.com/foo#bar` * `https://user@password:www.google.com/` But it does not contain these URLs: * `http://www.google.com/` (wrong scheme) * `https://google.com/` (hostname does not match) * `https://www.google.com:444/` (port does not match) REQUIRED"]
                    pub target_web_site: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod statements {
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
                    #[serde(rename = "relation")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Use only associations that match the specified relation. See the [`Statement`](#Statement) message for a detailed definition of relation strings. For a query to match a statement, one of the following must be true: * both the query's and the statement's relation strings match exactly, or * the query's relation string is empty or missing. Example: A query with relation `delegate_permission/common.handle_all_urls` matches an asset link with relation `delegate_permission/common.handle_all_urls`."]
                    pub relation: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "source.androidApp.certificate.sha256Fingerprint")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The uppercase SHA-265 fingerprint of the certificate. From the PEM certificate, it can be acquired like this: $ keytool -printcert -file $CERTFILE | grep SHA256: SHA256: 14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83: \\ 42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5 or like this: $ openssl x509 -in $CERTFILE -noout -fingerprint -sha256 SHA256 Fingerprint=14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64: \\ 16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5 In this example, the contents of this field would be `14:6D:E9:83:C5:73: 06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF: 44:E5`. If these tools are not available to you, you can convert the PEM certificate into the DER format, compute the SHA-256 hash of that string and represent the result as a hexstring (that is, uppercase hexadecimal representations of each octet, separated by colons)."]
                    pub source_android_app_certificate_sha256_fingerprint:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "source.androidApp.packageName")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Android App assets are naturally identified by their Java package name. For example, the Google Maps app uses the package name `com.google.android.apps.maps`. REQUIRED"]
                    pub source_android_app_package_name:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "source.web.site")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Web assets are identified by a URL that contains only the scheme, hostname and port parts. The format is http[s]://[:] Hostnames must be fully qualified: they must end in a single period (\"`.`\"). Only the schemes \"http\" and \"https\" are currently allowed. Port numbers are given as a decimal number, and they must be omitted if the standard port numbers are used: 80 for http and 443 for https. We call this limited URL the \"site\". All URLs that share the same scheme, hostname and port are considered to be a part of the site and thus belong to the web asset. Example: the asset with the site `https://www.google.com` contains all these URLs: * `https://www.google.com/` * `https://www.google.com:443/` * `https://www.google.com/foo` * `https://www.google.com/foo?bar` * `https://www.google.com/foo#bar` * `https://user@password:www.google.com/` But it does not contain these URLs: * `http://www.google.com/` (wrong scheme) * `https://google.com/` (hostname does not match) * `https://www.google.com:444/` (port does not match) REQUIRED"]
                    pub source_web_site: ::std::option::Option<::std::string::String>,
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
    #[doc = "Describes an android app asset."]
    pub struct AndroidAppAsset {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "certificate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Because there is no global enforcement of package name uniqueness, we also require a signing certificate, which in combination with the package name uniquely identifies an app. Some apps' signing keys are rotated, so they may be signed by different keys over time. We treat these as distinct assets, since we use (package name, cert) as the unique ID. This should not normally pose any problems as both versions of the app will make the same or similar statements. Other assets making statements about the app will have to be updated when a key is rotated, however. (Note that the syntaxes for publishing and querying for statements contain syntactic sugar to easily let you specify apps that are known by multiple certificates.) REQUIRED"]
        pub certificate: ::std::option::Option<::std::boxed::Box<CertificateInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Android App assets are naturally identified by their Java package name. For example, the Google Maps app uses the package name `com.google.android.apps.maps`. REQUIRED"]
        pub package_name: ::std::option::Option<::std::string::String>,
    }
    impl AndroidAppAsset {
        pub fn builder() -> AndroidAppAssetBuilder {
            AndroidAppAssetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Uniquely identifies an asset. A digital asset is an identifiable and addressable online entity that typically provides some service or content. Examples of assets are websites, Android apps, Twitter feeds, and Plus Pages."]
    pub struct Asset {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "androidApp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set if this is an Android App asset."]
        pub android_app: ::std::option::Option<::std::boxed::Box<AndroidAppAsset>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "web")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set if this is a web asset."]
        pub web: ::std::option::Option<::std::boxed::Box<WebAsset>>,
    }
    impl Asset {
        pub fn builder() -> AssetBuilder {
            AssetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes an X509 certificate."]
    pub struct CertificateInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sha256Fingerprint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The uppercase SHA-265 fingerprint of the certificate. From the PEM certificate, it can be acquired like this: $ keytool -printcert -file $CERTFILE | grep SHA256: SHA256: 14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83: \\ 42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5 or like this: $ openssl x509 -in $CERTFILE -noout -fingerprint -sha256 SHA256 Fingerprint=14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64: \\ 16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5 In this example, the contents of this field would be `14:6D:E9:83:C5:73: 06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF: 44:E5`. If these tools are not available to you, you can convert the PEM certificate into the DER format, compute the SHA-256 hash of that string and represent the result as a hexstring (that is, uppercase hexadecimal representations of each octet, separated by colons)."]
        pub sha256_fingerprint: ::std::option::Option<::std::string::String>,
    }
    impl CertificateInfo {
        pub fn builder() -> CertificateInfoBuilder {
            CertificateInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for the CheckAssetLinks call."]
    pub struct CheckResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "debugString")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human-readable message containing information intended to help end users understand, reproduce and debug the result. The message will be in English and we are currently not planning to offer any translations. Please note that no guarantees are made about the contents or format of this string. Any aspect of it may be subject to change without notice. You should not attempt to programmatically parse this data. For programmatic access, use the error_code field below."]
        pub debug_string: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Error codes that describe the result of the Check operation."]
        pub error_code: ::std::option::Option<::std::vec::Vec<CheckResponseErrorCodeEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linked")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set to true if the assets specified in the request are linked by the relation specified in the request."]
        pub linked: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxAge")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "From serving time, how much longer the response should be considered valid barring further updates. REQUIRED"]
        pub max_age: ::std::option::Option<::std::string::String>,
    }
    impl CheckResponse {
        pub fn builder() -> CheckResponseBuilder {
            CheckResponseBuilder::default()
        }
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
    impl ::std::default::Default for CheckResponseErrorCodeEnum {
        fn default() -> Self {
            Self::ErrorCodeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for the List call."]
    pub struct ListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "debugString")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human-readable message containing information intended to help end users understand, reproduce and debug the result. The message will be in English and we are currently not planning to offer any translations. Please note that no guarantees are made about the contents or format of this string. Any aspect of it may be subject to change without notice. You should not attempt to programmatically parse this data. For programmatic access, use the error_code field below."]
        pub debug_string: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Error codes that describe the result of the List operation."]
        pub error_code: ::std::option::Option<::std::vec::Vec<ListResponseErrorCodeEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxAge")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "From serving time, how much longer the response should be considered valid barring further updates. REQUIRED"]
        pub max_age: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statements")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of all the matching statements that have been found."]
        pub statements: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Statement>>>,
    }
    impl ListResponse {
        pub fn builder() -> ListResponseBuilder {
            ListResponseBuilder::default()
        }
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
    impl ::std::default::Default for ListResponseErrorCodeEnum {
        fn default() -> Self {
            Self::ErrorCodeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a reliable statement that has been made about the relationship between a source asset and a target asset. Statements are always made by the source asset, either directly or by delegating to a statement list that is stored elsewhere. For more detailed definitions of statements and assets, please refer to our [API documentation landing page](/digital-asset-links/v1/getting-started)."]
    pub struct Statement {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The relation identifies the use of the statement as intended by the source asset's owner (that is, the person or entity who issued the statement). Every complete statement has a relation. We identify relations with strings of the format `/`, where `` must be one of a set of pre-defined purpose categories, and `` is a free-form lowercase alphanumeric string that describes the specific use case of the statement. Refer to [our API documentation](/digital-asset-links/v1/relation-strings) for the current list of supported relations. Example: `delegate_permission/common.handle_all_urls` REQUIRED"]
        pub relation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Every statement has a source asset. REQUIRED"]
        pub source: ::std::option::Option<::std::boxed::Box<Asset>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "target")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Every statement has a target asset. REQUIRED"]
        pub target: ::std::option::Option<::std::boxed::Box<Asset>>,
    }
    impl Statement {
        pub fn builder() -> StatementBuilder {
            StatementBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a web asset."]
    pub struct WebAsset {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "site")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Web assets are identified by a URL that contains only the scheme, hostname and port parts. The format is http[s]://[:] Hostnames must be fully qualified: they must end in a single period (\"`.`\"). Only the schemes \"http\" and \"https\" are currently allowed. Port numbers are given as a decimal number, and they must be omitted if the standard port numbers are used: 80 for http and 443 for https. We call this limited URL the \"site\". All URLs that share the same scheme, hostname and port are considered to be a part of the site and thus belong to the web asset. Example: the asset with the site `https://www.google.com` contains all these URLs: * `https://www.google.com/` * `https://www.google.com:443/` * `https://www.google.com/foo` * `https://www.google.com/foo?bar` * `https://www.google.com/foo#bar` * `https://user@password:www.google.com/` But it does not contain these URLs: * `http://www.google.com/` (wrong scheme) * `https://google.com/` (hostname does not match) * `https://www.google.com:444/` (port does not match) REQUIRED"]
        pub site: ::std::option::Option<::std::string::String>,
    }
    impl WebAsset {
        pub fn builder() -> WebAssetBuilder {
            WebAssetBuilder::default()
        }
    }
}
