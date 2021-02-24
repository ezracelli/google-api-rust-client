#[derive(
    Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
)]
pub struct QueryParameters {
    #[builder(default = "{ query_parameters_defaults :: alt () }", setter(into))]
    #[serde(rename = "alt")]
    #[serde(default = "query_parameters_defaults :: alt")]
    #[doc = "Data format for the response."]
    pub alt: QueryParametersAltEnum,
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
    #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
    pub quota_user: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "userIp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Please use quotaUser instead."]
    pub user_ip: ::std::option::Option<::std::string::String>,
}
impl QueryParameters {
    pub fn builder() -> QueryParametersBuilder {
        QueryParametersBuilder::default()
    }
}
mod query_parameters_defaults {
    pub fn alt() -> super::QueryParametersAltEnum {
        serde_json::from_str(&"json").unwrap()
    }
    pub fn pretty_print() -> ::std::primitive::bool {
        serde_json::from_str(&"true").unwrap()
    }
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Data format for the response."]
pub enum QueryParametersAltEnum {
    #[serde(rename = "json")]
    #[doc = "Responses with Content-Type of application/json"]
    Json,
}
impl ::std::default::Default for QueryParametersAltEnum {
    fn default() -> Self {
        Self::Json
    }
}
pub mod resources {
    pub mod relyingparty {
        pub mod methods {
            pub mod get_project_config {
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
                    #[serde(rename = "delegatedProjectNumber")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Delegated GCP project number of the request."]
                    pub delegated_project_number: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "projectNumber")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "GCP project number of the request."]
                    pub project_number: ::std::option::Option<::std::string::String>,
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
    #[doc = "Response of creating the IDP authentication URL."]
    pub struct CreateAuthUriResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allProviders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "all providers the user has once used to do federated login"]
        pub all_providers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI used by the IDP to authenticate the user."]
        pub auth_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "captchaRequired")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if captcha is required."]
        pub captcha_required: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "forExistingProvider")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the authUri is for user's existing provider."]
        pub for_existing_provider: ::std::option::Option<::std::primitive::bool>,
        #[builder(
            default = "{ create_auth_uri_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "create_auth_uri_response_defaults :: kind")]
        #[doc = "The fixed string identitytoolkit#CreateAuthUriResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "providerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The provider ID of the auth URI."]
        pub provider_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "registered")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the user is registered if the identifier is an email."]
        pub registered: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sessionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Session ID which should be passed in the following verifyAssertion request."]
        pub session_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "signinMethods")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All sign-in methods this user has used."]
        pub signin_methods: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl CreateAuthUriResponse {
        pub fn builder() -> CreateAuthUriResponseBuilder {
            CreateAuthUriResponseBuilder::default()
        }
    }
    mod create_auth_uri_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("identitytoolkit#CreateAuthUriResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Respone of deleting account."]
    pub struct DeleteAccountResponse {
        #[builder(
            default = "{ delete_account_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "delete_account_response_defaults :: kind")]
        #[doc = "The fixed string \"identitytoolkit#DeleteAccountResponse\"."]
        pub kind: ::std::string::String,
    }
    impl DeleteAccountResponse {
        pub fn builder() -> DeleteAccountResponseBuilder {
            DeleteAccountResponseBuilder::default()
        }
    }
    mod delete_account_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("identitytoolkit#DeleteAccountResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of downloading accounts in batch."]
    pub struct DownloadAccountResponse {
        #[builder(
            default = "{ download_account_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "download_account_response_defaults :: kind")]
        #[doc = "The fixed string \"identitytoolkit#DownloadAccountResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The next page token. To be used in a subsequent request to return the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "users")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user accounts data."]
        pub users: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UserInfo>>>,
    }
    impl DownloadAccountResponse {
        pub fn builder() -> DownloadAccountResponseBuilder {
            DownloadAccountResponseBuilder::default()
        }
    }
    mod download_account_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("identitytoolkit#DownloadAccountResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of email signIn."]
    pub struct EmailLinkSigninResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's email."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expiresIn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Expiration time of STS id token in seconds."]
        pub expires_in: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "idToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The STS id token to login the newly signed in user."]
        pub id_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isNewUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the user is new."]
        pub is_new_user: ::std::option::Option<::std::primitive::bool>,
        #[builder(
            default = "{ email_link_signin_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "email_link_signin_response_defaults :: kind")]
        #[doc = "The fixed string \"identitytoolkit#EmailLinkSigninResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The RP local ID of the user."]
        pub local_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "refreshToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The refresh token for the signed in user."]
        pub refresh_token: ::std::option::Option<::std::string::String>,
    }
    impl EmailLinkSigninResponse {
        pub fn builder() -> EmailLinkSigninResponseBuilder {
            EmailLinkSigninResponseBuilder::default()
        }
    }
    mod email_link_signin_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("identitytoolkit#EmailLinkSigninResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Template for an email template."]
    pub struct EmailTemplate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "body")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email body."]
        pub body: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "format")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email body format."]
        pub format: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "from")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "From address of the email."]
        pub from: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fromDisplayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "From display name."]
        pub from_display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replyTo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reply-to address."]
        pub reply_to: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subject")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Subject of the email."]
        pub subject: ::std::option::Option<::std::string::String>,
    }
    impl EmailTemplate {
        pub fn builder() -> EmailTemplateBuilder {
            EmailTemplateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of getting account information."]
    pub struct GetAccountInfoResponse {
        #[builder(
            default = "{ get_account_info_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "get_account_info_response_defaults :: kind")]
        #[doc = "The fixed string \"identitytoolkit#GetAccountInfoResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "users")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The info of the users."]
        pub users: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UserInfo>>>,
    }
    impl GetAccountInfoResponse {
        pub fn builder() -> GetAccountInfoResponseBuilder {
            GetAccountInfoResponseBuilder::default()
        }
    }
    mod get_account_info_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("identitytoolkit#GetAccountInfoResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of getting a code for user confirmation (reset password, change email etc.)."]
    pub struct GetOobConfirmationCodeResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address that the email is sent to."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ get_oob_confirmation_code_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "get_oob_confirmation_code_response_defaults :: kind")]
        #[doc = "The fixed string \"identitytoolkit#GetOobConfirmationCodeResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oobCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The code to be send to the user."]
        pub oob_code: ::std::option::Option<::std::string::String>,
    }
    impl GetOobConfirmationCodeResponse {
        pub fn builder() -> GetOobConfirmationCodeResponseBuilder {
            GetOobConfirmationCodeResponseBuilder::default()
        }
    }
    mod get_oob_confirmation_code_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("identitytoolkit#GetOobConfirmationCodeResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of getting recaptcha param."]
    pub struct GetRecaptchaParamResponse {
        #[builder(
            default = "{ get_recaptcha_param_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "get_recaptcha_param_response_defaults :: kind")]
        #[doc = "The fixed string \"identitytoolkit#GetRecaptchaParamResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recaptchaSiteKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Site key registered at recaptcha."]
        pub recaptcha_site_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recaptchaStoken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The stoken field for the recaptcha widget, used to request captcha challenge."]
        pub recaptcha_stoken: ::std::option::Option<::std::string::String>,
    }
    impl GetRecaptchaParamResponse {
        pub fn builder() -> GetRecaptchaParamResponseBuilder {
            GetRecaptchaParamResponseBuilder::default()
        }
    }
    mod get_recaptcha_param_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("identitytoolkit#GetRecaptchaParamResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to get the IDP authentication URL."]
    pub struct IdentitytoolkitRelyingpartyCreateAuthUriRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The app ID of the mobile app, base64(CERT_SHA1):PACKAGE_NAME for Android, BUNDLE_ID for iOS."]
        pub app_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authFlowType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Explicitly specify the auth flow type. Currently only support \"CODE_FLOW\" type. The field is only used for Google provider."]
        pub auth_flow_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The relying party OAuth client ID."]
        pub client_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "context")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The opaque value used by the client to maintain context info between the authentication request and the IDP callback."]
        pub context: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "continueUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI to which the IDP redirects the user after the federated login flow."]
        pub continue_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customParameter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The query parameter that client can customize by themselves in auth url. The following parameters are reserved for server so that they cannot be customized by clients: client_id, response_type, scope, redirect_uri, state, oauth_token."]
        pub custom_parameter:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hostedDomain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The hosted domain to restrict sign-in to accounts at that domain for Google Apps hosted accounts."]
        pub hosted_domain: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "identifier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email or federated ID of the user."]
        pub identifier: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oauthConsumerKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The developer's consumer key for OpenId OAuth Extension"]
        pub oauth_consumer_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oauthScope")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional oauth scopes, beyond the basid user profile, that the user would be prompted to grant"]
        pub oauth_scope: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "openidRealm")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional realm for OpenID protocol. The sub string \"scheme://domain:port\" of the param \"continueUri\" is used if this is not set."]
        pub openid_realm: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "otaApp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The native app package for OTA installation."]
        pub ota_app: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "providerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IdP ID. For white listed IdPs it's a short domain name e.g. google.com, aol.com, live.net and yahoo.com. For other OpenID IdPs it's the OP identifier."]
        pub provider_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sessionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The session_id passed by client."]
        pub session_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tenantId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For multi-tenant use cases, in order to construct sign-in URL with the correct IDP parameters, Firebear needs to know which Tenant to retrieve IDP configs from."]
        pub tenant_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tenantProjectNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tenant project number to be used for idp discovery."]
        pub tenant_project_number: ::std::option::Option<::std::string::String>,
    }
    impl IdentitytoolkitRelyingpartyCreateAuthUriRequest {
        pub fn builder() -> IdentitytoolkitRelyingpartyCreateAuthUriRequestBuilder {
            IdentitytoolkitRelyingpartyCreateAuthUriRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to delete account."]
    pub struct IdentitytoolkitRelyingpartyDeleteAccountRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "delegatedProjectNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        pub delegated_project_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "idToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The GITKit token or STS id token of the authenticated user."]
        pub id_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The local ID of the user."]
        pub local_id: ::std::option::Option<::std::string::String>,
    }
    impl IdentitytoolkitRelyingpartyDeleteAccountRequest {
        pub fn builder() -> IdentitytoolkitRelyingpartyDeleteAccountRequestBuilder {
            IdentitytoolkitRelyingpartyDeleteAccountRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to download user account in batch."]
    pub struct IdentitytoolkitRelyingpartyDownloadAccountRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "delegatedProjectNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        pub delegated_project_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The max number of results to return in the response."]
        pub max_results: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token for the next page. This should be taken from the previous response."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetProjectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specify which project (field value is actually project id) to operate. Only used when provided credential."]
        pub target_project_id: ::std::option::Option<::std::string::String>,
    }
    impl IdentitytoolkitRelyingpartyDownloadAccountRequest {
        pub fn builder() -> IdentitytoolkitRelyingpartyDownloadAccountRequestBuilder {
            IdentitytoolkitRelyingpartyDownloadAccountRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to sign in with email."]
    pub struct IdentitytoolkitRelyingpartyEmailLinkSigninRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address of the user."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "idToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token for linking flow."]
        pub id_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oobCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The confirmation code."]
        pub oob_code: ::std::option::Option<::std::string::String>,
    }
    impl IdentitytoolkitRelyingpartyEmailLinkSigninRequest {
        pub fn builder() -> IdentitytoolkitRelyingpartyEmailLinkSigninRequestBuilder {
            IdentitytoolkitRelyingpartyEmailLinkSigninRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to get the account information."]
    pub struct IdentitytoolkitRelyingpartyGetAccountInfoRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "delegatedProjectNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        pub delegated_project_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of emails of the users to inquiry."]
        pub email: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "idToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The GITKit token of the authenticated user."]
        pub id_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of local ID's of the users to inquiry."]
        pub local_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phoneNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Privileged caller can query users by specified phone number."]
        pub phone_number: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl IdentitytoolkitRelyingpartyGetAccountInfoRequest {
        pub fn builder() -> IdentitytoolkitRelyingpartyGetAccountInfoRequestBuilder {
            IdentitytoolkitRelyingpartyGetAccountInfoRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of getting the project configuration."]
    pub struct IdentitytoolkitRelyingpartyGetProjectConfigResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowPasswordUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to allow password user sign in or sign up."]
        pub allow_password_user: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Browser API key, needed when making http request to Apiary."]
        pub api_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authorizedDomains")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Authorized domains."]
        pub authorized_domains: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "changeEmailTemplate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Change email template."]
        pub change_email_template: ::std::option::Option<::std::boxed::Box<EmailTemplate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dynamicLinksDomain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub dynamic_links_domain: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableAnonymousUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether anonymous user is enabled."]
        pub enable_anonymous_user: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "idpConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OAuth2 provider configuration."]
        pub idp_config: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<IdpConfig>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "legacyResetPasswordTemplate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Legacy reset password email template."]
        pub legacy_reset_password_template: ::std::option::Option<::std::boxed::Box<EmailTemplate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Project ID of the relying party."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resetPasswordTemplate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reset password email template."]
        pub reset_password_template: ::std::option::Option<::std::boxed::Box<EmailTemplate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "useEmailSending")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to use email sending provided by Firebear."]
        pub use_email_sending: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verifyEmailTemplate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Verify email template."]
        pub verify_email_template: ::std::option::Option<::std::boxed::Box<EmailTemplate>>,
    }
    impl IdentitytoolkitRelyingpartyGetProjectConfigResponse {
        pub fn builder() -> IdentitytoolkitRelyingpartyGetProjectConfigResponseBuilder {
            IdentitytoolkitRelyingpartyGetProjectConfigResponseBuilder::default()
        }
    }
    #[doc = "Respone of getting public keys."]
    pub type IdentitytoolkitRelyingpartyGetPublicKeysResponse =
        ::std::collections::BTreeMap<String, ::std::string::String>;
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to reset the password."]
    pub struct IdentitytoolkitRelyingpartyResetPasswordRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address of the user."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newPassword")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The new password inputted by the user."]
        pub new_password: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oldPassword")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The old password inputted by the user."]
        pub old_password: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oobCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The confirmation code."]
        pub oob_code: ::std::option::Option<::std::string::String>,
    }
    impl IdentitytoolkitRelyingpartyResetPasswordRequest {
        pub fn builder() -> IdentitytoolkitRelyingpartyResetPasswordRequestBuilder {
            IdentitytoolkitRelyingpartyResetPasswordRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for Identitytoolkit-SendVerificationCode"]
    pub struct IdentitytoolkitRelyingpartySendVerificationCodeRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iosReceipt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Receipt of successful app token validation with APNS."]
        pub ios_receipt: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iosSecret")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Secret delivered to iOS app via APNS."]
        pub ios_secret: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phoneNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The phone number to send the verification code to in E.164 format."]
        pub phone_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recaptchaToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Recaptcha solution."]
        pub recaptcha_token: ::std::option::Option<::std::string::String>,
    }
    impl IdentitytoolkitRelyingpartySendVerificationCodeRequest {
        pub fn builder() -> IdentitytoolkitRelyingpartySendVerificationCodeRequestBuilder {
            IdentitytoolkitRelyingpartySendVerificationCodeRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for Identitytoolkit-SendVerificationCode"]
    pub struct IdentitytoolkitRelyingpartySendVerificationCodeResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sessionInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Encrypted session information"]
        pub session_info: ::std::option::Option<::std::string::String>,
    }
    impl IdentitytoolkitRelyingpartySendVerificationCodeResponse {
        pub fn builder() -> IdentitytoolkitRelyingpartySendVerificationCodeResponseBuilder {
            IdentitytoolkitRelyingpartySendVerificationCodeResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to set the account information."]
    pub struct IdentitytoolkitRelyingpartySetAccountInfoRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "captchaChallenge")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The captcha challenge."]
        pub captcha_challenge: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "captchaResponse")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Response to the captcha."]
        pub captcha_response: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp when the account is created."]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customAttributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The custom attributes to be set in the user's id token."]
        pub custom_attributes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "delegatedProjectNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        pub delegated_project_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteAttribute")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The attributes users request to delete."]
        pub delete_attribute: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleteProvider")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IDPs the user request to delete."]
        pub delete_provider: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disableUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to disable the user."]
        pub disable_user: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the user."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email of the user."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emailVerified")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mark the email as verified or not."]
        pub email_verified: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "idToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The GITKit token of the authenticated user."]
        pub id_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instanceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Instance id token of the app."]
        pub instance_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastLoginAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Last login timestamp."]
        pub last_login_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The local ID of the user."]
        pub local_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oobCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The out-of-band code of the change email request."]
        pub oob_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "password")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The new password of the user."]
        pub password: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phoneNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Privileged caller can update user with specified phone number."]
        pub phone_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "photoUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The photo url of the user."]
        pub photo_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provider")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The associated IDPs of the user."]
        pub provider: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "returnSecureToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether return sts id token and refresh token instead of gitkit token."]
        pub return_secure_token: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "upgradeToFederatedLogin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mark the user to upgrade to federated login."]
        pub upgrade_to_federated_login: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "validSince")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp in seconds for valid login token."]
        pub valid_since: ::std::option::Option<::std::string::String>,
    }
    impl IdentitytoolkitRelyingpartySetAccountInfoRequest {
        pub fn builder() -> IdentitytoolkitRelyingpartySetAccountInfoRequestBuilder {
            IdentitytoolkitRelyingpartySetAccountInfoRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to set the project configuration."]
    pub struct IdentitytoolkitRelyingpartySetProjectConfigRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowPasswordUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to allow password user sign in or sign up."]
        pub allow_password_user: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Browser API key, needed when making http request to Apiary."]
        pub api_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authorizedDomains")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Authorized domains for widget redirect."]
        pub authorized_domains: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "changeEmailTemplate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Change email template."]
        pub change_email_template: ::std::option::Option<::std::boxed::Box<EmailTemplate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "delegatedProjectNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        pub delegated_project_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableAnonymousUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to enable anonymous user."]
        pub enable_anonymous_user: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "idpConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Oauth2 provider configuration."]
        pub idp_config: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<IdpConfig>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "legacyResetPasswordTemplate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Legacy reset password email template."]
        pub legacy_reset_password_template: ::std::option::Option<::std::boxed::Box<EmailTemplate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resetPasswordTemplate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reset password email template."]
        pub reset_password_template: ::std::option::Option<::std::boxed::Box<EmailTemplate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "useEmailSending")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to use email sending provided by Firebear."]
        pub use_email_sending: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verifyEmailTemplate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Verify email template."]
        pub verify_email_template: ::std::option::Option<::std::boxed::Box<EmailTemplate>>,
    }
    impl IdentitytoolkitRelyingpartySetProjectConfigRequest {
        pub fn builder() -> IdentitytoolkitRelyingpartySetProjectConfigRequestBuilder {
            IdentitytoolkitRelyingpartySetProjectConfigRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of setting the project configuration."]
    pub struct IdentitytoolkitRelyingpartySetProjectConfigResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Project ID of the relying party."]
        pub project_id: ::std::option::Option<::std::string::String>,
    }
    impl IdentitytoolkitRelyingpartySetProjectConfigResponse {
        pub fn builder() -> IdentitytoolkitRelyingpartySetProjectConfigResponseBuilder {
            IdentitytoolkitRelyingpartySetProjectConfigResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to sign out user."]
    pub struct IdentitytoolkitRelyingpartySignOutUserRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instanceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Instance id token of the app."]
        pub instance_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The local ID of the user."]
        pub local_id: ::std::option::Option<::std::string::String>,
    }
    impl IdentitytoolkitRelyingpartySignOutUserRequest {
        pub fn builder() -> IdentitytoolkitRelyingpartySignOutUserRequestBuilder {
            IdentitytoolkitRelyingpartySignOutUserRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of signing out user."]
    pub struct IdentitytoolkitRelyingpartySignOutUserResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The local ID of the user."]
        pub local_id: ::std::option::Option<::std::string::String>,
    }
    impl IdentitytoolkitRelyingpartySignOutUserResponse {
        pub fn builder() -> IdentitytoolkitRelyingpartySignOutUserResponseBuilder {
            IdentitytoolkitRelyingpartySignOutUserResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to signup new user, create anonymous user or anonymous user reauth."]
    pub struct IdentitytoolkitRelyingpartySignupNewUserRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "captchaChallenge")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The captcha challenge."]
        pub captcha_challenge: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "captchaResponse")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Response to the captcha."]
        pub captcha_response: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to disable the user. Only can be used by service account."]
        pub disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the user."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email of the user."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emailVerified")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mark the email as verified or not. Only can be used by service account."]
        pub email_verified: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "idToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The GITKit token of the authenticated user."]
        pub id_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instanceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Instance id token of the app."]
        pub instance_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Privileged caller can create user with specified user id."]
        pub local_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "password")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The new password of the user."]
        pub password: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phoneNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Privileged caller can create user with specified phone number."]
        pub phone_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "photoUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The photo url of the user."]
        pub photo_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tenantId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For multi-tenant use cases, in order to construct sign-in URL with the correct IDP parameters, Firebear needs to know which Tenant to retrieve IDP configs from."]
        pub tenant_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tenantProjectNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tenant project number to be used for idp discovery."]
        pub tenant_project_number: ::std::option::Option<::std::string::String>,
    }
    impl IdentitytoolkitRelyingpartySignupNewUserRequest {
        pub fn builder() -> IdentitytoolkitRelyingpartySignupNewUserRequestBuilder {
            IdentitytoolkitRelyingpartySignupNewUserRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to upload user account in batch."]
    pub struct IdentitytoolkitRelyingpartyUploadAccountRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowOverwrite")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether allow overwrite existing account when user local_id exists."]
        pub allow_overwrite: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blockSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub block_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpuMemCost")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The following 4 fields are for standard scrypt algorithm."]
        pub cpu_mem_cost: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "delegatedProjectNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        pub delegated_project_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dkLen")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub dk_len: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hashAlgorithm")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The password hash algorithm."]
        pub hash_algorithm: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "memoryCost")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Memory cost for hash calculation. Used by scrypt similar algorithms."]
        pub memory_cost: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parallelization")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub parallelization: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rounds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rounds for hash calculation. Used by scrypt and similar algorithms."]
        pub rounds: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "saltSeparator")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The salt separator."]
        pub salt_separator: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sanityCheck")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, backend will do sanity check(including duplicate email and federated id) when uploading account."]
        pub sanity_check: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "signerKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The key for to hash the password."]
        pub signer_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetProjectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specify which project (field value is actually project id) to operate. Only used when provided credential."]
        pub target_project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "users")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The account info to be stored."]
        pub users: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UserInfo>>>,
    }
    impl IdentitytoolkitRelyingpartyUploadAccountRequest {
        pub fn builder() -> IdentitytoolkitRelyingpartyUploadAccountRequestBuilder {
            IdentitytoolkitRelyingpartyUploadAccountRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to verify the IDP assertion."]
    pub struct IdentitytoolkitRelyingpartyVerifyAssertionRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoCreate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When it's true, automatically creates a new account if the user doesn't exist. When it's false, allows existing user to sign in normally and throws exception if the user doesn't exist."]
        pub auto_create: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "delegatedProjectNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        pub delegated_project_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "idToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The GITKit token of the authenticated user."]
        pub id_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instanceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Instance id token of the app."]
        pub instance_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pendingIdToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The GITKit token for the non-trusted IDP pending to be confirmed by the user."]
        pub pending_id_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postBody")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The post body if the request is a HTTP POST."]
        pub post_body: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI to which the IDP redirects the user back. It may contain federated login result params added by the IDP."]
        pub request_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "returnIdpCredential")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether return 200 and IDP credential rather than throw exception when federated id is already linked."]
        pub return_idp_credential: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "returnRefreshToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to return refresh tokens."]
        pub return_refresh_token: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "returnSecureToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether return sts id token and refresh token instead of gitkit token."]
        pub return_secure_token: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sessionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Session ID, which should match the one in previous createAuthUri request."]
        pub session_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tenantId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For multi-tenant use cases, in order to construct sign-in URL with the correct IDP parameters, Firebear needs to know which Tenant to retrieve IDP configs from."]
        pub tenant_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tenantProjectNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tenant project number to be used for idp discovery."]
        pub tenant_project_number: ::std::option::Option<::std::string::String>,
    }
    impl IdentitytoolkitRelyingpartyVerifyAssertionRequest {
        pub fn builder() -> IdentitytoolkitRelyingpartyVerifyAssertionRequestBuilder {
            IdentitytoolkitRelyingpartyVerifyAssertionRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to verify a custom token"]
    pub struct IdentitytoolkitRelyingpartyVerifyCustomTokenRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "delegatedProjectNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        pub delegated_project_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instanceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Instance id token of the app."]
        pub instance_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "returnSecureToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether return sts id token and refresh token instead of gitkit token."]
        pub return_secure_token: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "token")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The custom token to verify"]
        pub token: ::std::option::Option<::std::string::String>,
    }
    impl IdentitytoolkitRelyingpartyVerifyCustomTokenRequest {
        pub fn builder() -> IdentitytoolkitRelyingpartyVerifyCustomTokenRequestBuilder {
            IdentitytoolkitRelyingpartyVerifyCustomTokenRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to verify the password."]
    pub struct IdentitytoolkitRelyingpartyVerifyPasswordRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "captchaChallenge")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The captcha challenge."]
        pub captcha_challenge: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "captchaResponse")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Response to the captcha."]
        pub captcha_response: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "delegatedProjectNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        pub delegated_project_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email of the user."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "idToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The GITKit token of the authenticated user."]
        pub id_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instanceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Instance id token of the app."]
        pub instance_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "password")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The password inputed by the user."]
        pub password: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pendingIdToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The GITKit token for the non-trusted IDP, which is to be confirmed by the user."]
        pub pending_id_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "returnSecureToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether return sts id token and refresh token instead of gitkit token."]
        pub return_secure_token: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tenantId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For multi-tenant use cases, in order to construct sign-in URL with the correct IDP parameters, Firebear needs to know which Tenant to retrieve IDP configs from."]
        pub tenant_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tenantProjectNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tenant project number to be used for idp discovery."]
        pub tenant_project_number: ::std::option::Option<::std::string::String>,
    }
    impl IdentitytoolkitRelyingpartyVerifyPasswordRequest {
        pub fn builder() -> IdentitytoolkitRelyingpartyVerifyPasswordRequestBuilder {
            IdentitytoolkitRelyingpartyVerifyPasswordRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for Identitytoolkit-VerifyPhoneNumber"]
    pub struct IdentitytoolkitRelyingpartyVerifyPhoneNumberRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "idToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub id_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub operation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phoneNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub phone_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sessionInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The session info previously returned by IdentityToolkit-SendVerificationCode."]
        pub session_info: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "temporaryProof")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub temporary_proof: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verificationProof")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub verification_proof: ::std::option::Option<::std::string::String>,
    }
    impl IdentitytoolkitRelyingpartyVerifyPhoneNumberRequest {
        pub fn builder() -> IdentitytoolkitRelyingpartyVerifyPhoneNumberRequestBuilder {
            IdentitytoolkitRelyingpartyVerifyPhoneNumberRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for Identitytoolkit-VerifyPhoneNumber"]
    pub struct IdentitytoolkitRelyingpartyVerifyPhoneNumberResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expiresIn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub expires_in: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "idToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub id_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isNewUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub is_new_user: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub local_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phoneNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub phone_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "refreshToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub refresh_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "temporaryProof")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub temporary_proof: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "temporaryProofExpiresIn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub temporary_proof_expires_in: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verificationProof")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub verification_proof: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verificationProofExpiresIn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub verification_proof_expires_in: ::std::option::Option<::std::string::String>,
    }
    impl IdentitytoolkitRelyingpartyVerifyPhoneNumberResponse {
        pub fn builder() -> IdentitytoolkitRelyingpartyVerifyPhoneNumberResponseBuilder {
            IdentitytoolkitRelyingpartyVerifyPhoneNumberResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Template for a single idp configuration."]
    pub struct IdpConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OAuth2 client ID."]
        pub client_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this IDP is enabled."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "experimentPercent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Percent of users who will be prompted/redirected federated login for this IDP."]
        pub experiment_percent: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provider")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OAuth2 provider."]
        pub provider: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secret")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OAuth2 client secret."]
        pub secret: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whitelistedAudiences")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whitelisted client IDs for audience check."]
        pub whitelisted_audiences: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl IdpConfig {
        pub fn builder() -> IdpConfigBuilder {
            IdpConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request of getting a code for user confirmation (reset password, change email etc.)"]
    pub struct Relyingparty {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "androidInstallApp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "whether or not to install the android app on the device where the link is opened"]
        pub android_install_app: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "androidMinimumVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "minimum version of the app. if the version on the device is lower than this version then the user is taken to the play store to upgrade the app"]
        pub android_minimum_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "androidPackageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "android package name of the android app to handle the action code"]
        pub android_package_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canHandleCodeInApp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "whether or not the app can handle the oob code without first going to web"]
        pub can_handle_code_in_app: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "captchaResp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The recaptcha response from the user."]
        pub captcha_resp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "challenge")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The recaptcha challenge presented to the user."]
        pub challenge: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "continueUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The url to continue to the Gitkit app"]
        pub continue_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email of the user."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iOSAppStoreId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "iOS app store id to download the app if it's not already installed"]
        pub i_os_app_store_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iOSBundleId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "the iOS bundle id of iOS app to handle the action code"]
        pub i_os_bundle_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "idToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's Gitkit login token for email change."]
        pub id_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ relyingparty_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "relyingparty_defaults :: kind")]
        #[doc = "The fixed string \"identitytoolkit#relyingparty\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The new email if the code is for email change."]
        pub new_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request type."]
        pub request_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userIp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IP address of the user."]
        pub user_ip: ::std::option::Option<::std::string::String>,
    }
    impl Relyingparty {
        pub fn builder() -> RelyingpartyBuilder {
            RelyingpartyBuilder::default()
        }
    }
    mod relyingparty_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("identitytoolkit#relyingparty")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of resetting the password."]
    pub struct ResetPasswordResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's email. If the out-of-band code is for email recovery, the user's original email."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ reset_password_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "reset_password_response_defaults :: kind")]
        #[doc = "The fixed string \"identitytoolkit#ResetPasswordResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the out-of-band code is for email recovery, the user's new email."]
        pub new_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request type."]
        pub request_type: ::std::option::Option<::std::string::String>,
    }
    impl ResetPasswordResponse {
        pub fn builder() -> ResetPasswordResponseBuilder {
            ResetPasswordResponseBuilder::default()
        }
    }
    mod reset_password_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("identitytoolkit#ResetPasswordResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Respone of setting the account information."]
    pub struct SetAccountInfoResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the user."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email of the user."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emailVerified")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If email has been verified."]
        pub email_verified: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expiresIn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If idToken is STS id token, then this field will be expiration time of STS id token in seconds."]
        pub expires_in: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "idToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Gitkit id token to login the newly sign up user."]
        pub id_token: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ set_account_info_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "set_account_info_response_defaults :: kind")]
        #[doc = "The fixed string \"identitytoolkit#SetAccountInfoResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The local ID of the user."]
        pub local_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The new email the user attempts to change to."]
        pub new_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "passwordHash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's hashed password."]
        pub password_hash: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "photoUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The photo url of the user."]
        pub photo_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "providerUserInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's profiles at the associated IdPs."]
        pub provider_user_info:
            ::std::option::Option<::std::vec::Vec<SetAccountInfoResponseProviderUserInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "refreshToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If idToken is STS id token, then this field will be refresh token."]
        pub refresh_token: ::std::option::Option<::std::string::String>,
    }
    impl SetAccountInfoResponse {
        pub fn builder() -> SetAccountInfoResponseBuilder {
            SetAccountInfoResponseBuilder::default()
        }
    }
    mod set_account_info_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("identitytoolkit#SetAccountInfoResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct SetAccountInfoResponseProviderUserInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's display name at the IDP."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "federatedId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User's identifier at IDP."]
        pub federated_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "photoUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's photo url at the IDP."]
        pub photo_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "providerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IdP ID. For whitelisted IdPs it's a short domain name, e.g., google.com, aol.com, live.net and yahoo.com. For other OpenID IdPs it's the OP identifier."]
        pub provider_id: ::std::option::Option<::std::string::String>,
    }
    impl SetAccountInfoResponseProviderUserInfo {
        pub fn builder() -> SetAccountInfoResponseProviderUserInfoBuilder {
            SetAccountInfoResponseProviderUserInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of signing up new user, creating anonymous user or anonymous user reauth."]
    pub struct SignupNewUserResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the user."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email of the user."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expiresIn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If idToken is STS id token, then this field will be expiration time of STS id token in seconds."]
        pub expires_in: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "idToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Gitkit id token to login the newly sign up user."]
        pub id_token: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ signup_new_user_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "signup_new_user_response_defaults :: kind")]
        #[doc = "The fixed string \"identitytoolkit#SignupNewUserResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The RP local ID of the user."]
        pub local_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "refreshToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If idToken is STS id token, then this field will be refresh token."]
        pub refresh_token: ::std::option::Option<::std::string::String>,
    }
    impl SignupNewUserResponse {
        pub fn builder() -> SignupNewUserResponseBuilder {
            SignupNewUserResponseBuilder::default()
        }
    }
    mod signup_new_user_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("identitytoolkit#SignupNewUserResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Respone of uploading accounts in batch."]
    pub struct UploadAccountResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error encountered while processing the account info."]
        pub error: ::std::option::Option<::std::vec::Vec<UploadAccountResponseError>>,
        #[builder(
            default = "{ upload_account_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "upload_account_response_defaults :: kind")]
        #[doc = "The fixed string \"identitytoolkit#UploadAccountResponse\"."]
        pub kind: ::std::string::String,
    }
    impl UploadAccountResponse {
        pub fn builder() -> UploadAccountResponseBuilder {
            UploadAccountResponseBuilder::default()
        }
    }
    mod upload_account_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("identitytoolkit#UploadAccountResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct UploadAccountResponseError {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "index")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index of the malformed account, starting from 0."]
        pub index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Detailed error message for the account info."]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl UploadAccountResponseError {
        pub fn builder() -> UploadAccountResponseErrorBuilder {
            UploadAccountResponseErrorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Template for an individual account info."]
    pub struct UserInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User creation timestamp."]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customAttributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The custom attributes to be set in the user's id token."]
        pub custom_attributes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customAuth")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the user is authenticated by the developer."]
        pub custom_auth: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the user is disabled."]
        pub disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the user."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email of the user."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emailVerified")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the email has been verified."]
        pub email_verified: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastLoginAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "last login timestamp."]
        pub last_login_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The local ID of the user."]
        pub local_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "passwordHash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's hashed password."]
        pub password_hash: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "passwordUpdatedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp when the password was last updated."]
        pub password_updated_at: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phoneNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User's phone number."]
        pub phone_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "photoUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the user profile photo."]
        pub photo_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "providerUserInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IDP of the user."]
        pub provider_user_info: ::std::option::Option<::std::vec::Vec<UserInfoProviderUserInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rawPassword")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's plain text password."]
        pub raw_password: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "salt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's password salt."]
        pub salt: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "screenName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User's screen name at Twitter or login name at Github."]
        pub screen_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "validSince")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp in seconds for valid login token."]
        pub valid_since: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Version of the user's password."]
        pub version: ::std::option::Option<::std::primitive::i64>,
    }
    impl UserInfo {
        pub fn builder() -> UserInfoBuilder {
            UserInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct UserInfoProviderUserInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's display name at the IDP."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User's email at IDP."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "federatedId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User's identifier at IDP."]
        pub federated_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phoneNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User's phone number."]
        pub phone_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "photoUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's photo url at the IDP."]
        pub photo_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "providerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IdP ID. For white listed IdPs it's a short domain name, e.g., google.com, aol.com, live.net and yahoo.com. For other OpenID IdPs it's the OP identifier."]
        pub provider_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rawId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User's raw identifier directly returned from IDP."]
        pub raw_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "screenName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User's screen name at Twitter or login name at Github."]
        pub screen_name: ::std::option::Option<::std::string::String>,
    }
    impl UserInfoProviderUserInfo {
        pub fn builder() -> UserInfoProviderUserInfoBuilder {
            UserInfoProviderUserInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of verifying the IDP assertion."]
    pub struct VerifyAssertionResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "action")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The action code."]
        pub action: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appInstallationUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL for OTA app installation."]
        pub app_installation_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appScheme")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The custom scheme used by mobile app."]
        pub app_scheme: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "context")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The opaque value used by the client to maintain context info between the authentication request and the IDP callback."]
        pub context: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dateOfBirth")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The birth date of the IdP account."]
        pub date_of_birth: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The display name of the user."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email returned by the IdP. NOTE: The federated login user may not own the email."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emailRecycled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "It's true if the email is recycled."]
        pub email_recycled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emailVerified")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value is true if the IDP is also the email provider. It means the user owns the email."]
        pub email_verified: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Client error code."]
        pub error_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expiresIn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If idToken is STS id token, then this field will be expiration time of STS id token in seconds."]
        pub expires_in: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "federatedId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique ID identifies the IdP account."]
        pub federated_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firstName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The first name of the user."]
        pub first_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full name of the user."]
        pub full_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "idToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID token."]
        pub id_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "It's the identifier param in the createAuthUri request if the identifier is an email. It can be used to check whether the user input email is different from the asserted email."]
        pub input_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isNewUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if it's a new user sign-in, false if it's a returning user."]
        pub is_new_user: ::std::option::Option<::std::primitive::bool>,
        #[builder(
            default = "{ verify_assertion_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "verify_assertion_response_defaults :: kind")]
        #[doc = "The fixed string \"identitytoolkit#VerifyAssertionResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "language")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language preference of the user."]
        pub language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last name of the user."]
        pub last_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The RP local ID if it's already been mapped to the IdP account identified by the federated ID."]
        pub local_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "needConfirmation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the assertion is from a non-trusted IDP and need account linking confirmation."]
        pub need_confirmation: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "needEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether need client to supply email to complete the federated login flow."]
        pub need_email: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nickName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The nick name of the user."]
        pub nick_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oauthAccessToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The OAuth2 access token."]
        pub oauth_access_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oauthAuthorizationCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The OAuth2 authorization code."]
        pub oauth_authorization_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oauthExpireIn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The lifetime in seconds of the OAuth2 access token."]
        pub oauth_expire_in: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oauthIdToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The OIDC id token."]
        pub oauth_id_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oauthRequestToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user approved request token for the OpenID OAuth extension."]
        pub oauth_request_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oauthScope")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The scope for the OpenID OAuth extension."]
        pub oauth_scope: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oauthTokenSecret")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The OAuth1 access token secret."]
        pub oauth_token_secret: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The original email stored in the mapping storage. It's returned when the federated ID is associated to a different email."]
        pub original_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "photoUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI of the public accessible profiel picture."]
        pub photo_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "providerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The IdP ID. For white listed IdPs it's a short domain name e.g. google.com, aol.com, live.net and yahoo.com. If the \"providerId\" param is set to OpenID OP identifer other than the whilte listed IdPs the OP identifier is returned. If the \"identifier\" param is federated ID in the createAuthUri request. The domain part of the federated ID is returned."]
        pub provider_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rawUserInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Raw IDP-returned user info."]
        pub raw_user_info: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "refreshToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If idToken is STS id token, then this field will be refresh token."]
        pub refresh_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "screenName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The screen_name of a Twitter user or the login name at Github."]
        pub screen_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timezone of the user."]
        pub time_zone: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verifiedProvider")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When action is 'map', contains the idps which can be used for confirmation."]
        pub verified_provider: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl VerifyAssertionResponse {
        pub fn builder() -> VerifyAssertionResponseBuilder {
            VerifyAssertionResponseBuilder::default()
        }
    }
    mod verify_assertion_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("identitytoolkit#VerifyAssertionResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response from verifying a custom token"]
    pub struct VerifyCustomTokenResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expiresIn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If idToken is STS id token, then this field will be expiration time of STS id token in seconds."]
        pub expires_in: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "idToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The GITKit token for authenticated user."]
        pub id_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isNewUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if it's a new user sign-in, false if it's a returning user."]
        pub is_new_user: ::std::option::Option<::std::primitive::bool>,
        #[builder(
            default = "{ verify_custom_token_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "verify_custom_token_response_defaults :: kind")]
        #[doc = "The fixed string \"identitytoolkit#VerifyCustomTokenResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "refreshToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If idToken is STS id token, then this field will be refresh token."]
        pub refresh_token: ::std::option::Option<::std::string::String>,
    }
    impl VerifyCustomTokenResponse {
        pub fn builder() -> VerifyCustomTokenResponseBuilder {
            VerifyCustomTokenResponseBuilder::default()
        }
    }
    mod verify_custom_token_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("identitytoolkit#VerifyCustomTokenResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request of verifying the password."]
    pub struct VerifyPasswordResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the user."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email returned by the IdP. NOTE: The federated login user may not own the email."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expiresIn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If idToken is STS id token, then this field will be expiration time of STS id token in seconds."]
        pub expires_in: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "idToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The GITKit token for authenticated user."]
        pub id_token: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ verify_password_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "verify_password_response_defaults :: kind")]
        #[doc = "The fixed string \"identitytoolkit#VerifyPasswordResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The RP local ID if it's already been mapped to the IdP account identified by the federated ID."]
        pub local_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oauthAccessToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The OAuth2 access token."]
        pub oauth_access_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oauthAuthorizationCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The OAuth2 authorization code."]
        pub oauth_authorization_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oauthExpireIn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The lifetime in seconds of the OAuth2 access token."]
        pub oauth_expire_in: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "photoUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI of the user's photo at IdP"]
        pub photo_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "refreshToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If idToken is STS id token, then this field will be refresh token."]
        pub refresh_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "registered")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the email is registered."]
        pub registered: ::std::option::Option<::std::primitive::bool>,
    }
    impl VerifyPasswordResponse {
        pub fn builder() -> VerifyPasswordResponseBuilder {
            VerifyPasswordResponseBuilder::default()
        }
    }
    mod verify_password_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("identitytoolkit#VerifyPasswordResponse")
        }
    }
}
