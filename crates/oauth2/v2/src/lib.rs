#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tokeninfo {
    #[serde(rename = "audience")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Who is the intended audience for this token. In general the same as issued_to."]
    pub audience: ::std::option::Option<::std::string::String>,
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email address of the user. Present only if the email scope is present in the request."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expires_in")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The expiry time of the token, as number of seconds left until expiry."]
    pub expires_in: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "issued_to")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "To whom was the token issued to. In general the same as audience."]
    pub issued_to: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scope")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The space separated list of scopes granted to this token."]
    pub scope: ::std::option::Option<::std::string::String>,
    #[serde(rename = "user_id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The obfuscated user id."]
    pub user_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "verified_email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Boolean flag which is true if the email address is verified. Present only if the email scope is present in the request."]
    pub verified_email: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Userinfo {
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user's email address."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "family_name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user's last name."]
    pub family_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gender")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user's gender."]
    pub gender: ::std::option::Option<::std::string::String>,
    #[serde(rename = "given_name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user's first name."]
    pub given_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hd")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The hosted domain e.g. example.com if the user is Google apps user."]
    pub hd: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The obfuscated ID of the user."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "link")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL of the profile page."]
    pub link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user's preferred locale."]
    pub locale: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The user's full name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "picture")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL of the user's picture image."]
    pub picture: ::std::option::Option<::std::string::String>,
    #[serde(rename = "verified_email")]
    #[serde(default = "userinfo_defaults :: verified_email")]
    #[doc = "Boolean flag which is true if the email address is verified. Always verified because we only return the user's primary email address."]
    pub verified_email: ::std::primitive::bool,
}
mod userinfo_defaults {
    pub fn verified_email() -> ::std::primitive::bool {
        serde_json::from_str(&"true").unwrap()
    }
}
