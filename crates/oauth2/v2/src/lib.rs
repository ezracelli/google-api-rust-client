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
pub mod methods {
    pub mod tokeninfo {
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
            #[serde(rename = "access_token")]
            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
            pub access_token: ::std::option::Option<::std::string::String>,
            #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
            #[serde(rename = "id_token")]
            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
            pub id_token: ::std::option::Option<::std::string::String>,
        }
        impl QueryParameters {
            pub fn builder() -> QueryParametersBuilder {
                QueryParametersBuilder::default()
            }
        }
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Tokeninfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audience")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Who is the intended audience for this token. In general the same as issued_to."]
        pub audience: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address of the user. Present only if the email scope is present in the request."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expires_in")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The expiry time of the token, as number of seconds left until expiry."]
        pub expires_in: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "issued_to")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "To whom was the token issued to. In general the same as audience."]
        pub issued_to: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scope")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The space separated list of scopes granted to this token."]
        pub scope: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "user_id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The obfuscated user id."]
        pub user_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verified_email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Boolean flag which is true if the email address is verified. Present only if the email scope is present in the request."]
        pub verified_email: ::std::option::Option<::std::primitive::bool>,
    }
    impl Tokeninfo {
        pub fn builder() -> TokeninfoBuilder {
            TokeninfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Userinfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's email address."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "family_name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's last name."]
        pub family_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gender")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's gender."]
        pub gender: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "given_name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's first name."]
        pub given_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hd")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The hosted domain e.g. example.com if the user is Google apps user."]
        pub hd: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The obfuscated ID of the user."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "link")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL of the profile page."]
        pub link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locale")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's preferred locale."]
        pub locale: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's full name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "picture")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL of the user's picture image."]
        pub picture: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ userinfo_defaults :: verified_email () }", setter(into))]
        #[serde(rename = "verified_email")]
        #[serde(default = "userinfo_defaults :: verified_email")]
        #[doc = "Boolean flag which is true if the email address is verified. Always verified because we only return the user's primary email address."]
        pub verified_email: ::std::primitive::bool,
    }
    impl Userinfo {
        pub fn builder() -> UserinfoBuilder {
            UserinfoBuilder::default()
        }
    }
    mod userinfo_defaults {
        pub fn verified_email() -> ::std::primitive::bool {
            serde_json::from_str(&"true").unwrap()
        }
    }
}
