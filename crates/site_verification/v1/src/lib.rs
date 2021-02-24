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
        serde_json::from_str(&"false").unwrap()
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
    pub mod web_resource {
        pub mod methods {
            pub mod insert {
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
                    #[serde(rename = "verificationMethod")]
                    #[doc = "The method to use for verifying a site or domain."]
                    pub verification_method: ::std::string::String,
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
    pub struct SiteVerificationWebResourceGettokenRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "site")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The site for which a verification token will be generated."]
        pub site: ::std::option::Option<SiteVerificationWebResourceGettokenRequestSite>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verificationMethod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The verification method that will be used to verify this site. For sites, 'FILE' or 'META' methods may be used. For domains, only 'DNS' may be used."]
        pub verification_method: ::std::option::Option<::std::string::String>,
    }
    impl SiteVerificationWebResourceGettokenRequest {
        pub fn builder() -> SiteVerificationWebResourceGettokenRequestBuilder {
            SiteVerificationWebResourceGettokenRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The site for which a verification token will be generated."]
    pub struct SiteVerificationWebResourceGettokenRequestSite {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "identifier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The site identifier. If the type is set to SITE, the identifier is a URL. If the type is set to INET_DOMAIN, the site identifier is a domain name."]
        pub identifier: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of resource to be verified. Can be SITE or INET_DOMAIN (domain name)."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl SiteVerificationWebResourceGettokenRequestSite {
        pub fn builder() -> SiteVerificationWebResourceGettokenRequestSiteBuilder {
            SiteVerificationWebResourceGettokenRequestSiteBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct SiteVerificationWebResourceGettokenResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The verification method to use in conjunction with this token. For FILE, the token should be placed in the top-level directory of the site, stored inside a file of the same name. For META, the token should be placed in the HEAD tag of the default page that is loaded for the site. For DNS, the token should be placed in a TXT record of the domain."]
        pub method: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "token")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The verification token. The token must be placed appropriately in order for verification to succeed."]
        pub token: ::std::option::Option<::std::string::String>,
    }
    impl SiteVerificationWebResourceGettokenResponse {
        pub fn builder() -> SiteVerificationWebResourceGettokenResponseBuilder {
            SiteVerificationWebResourceGettokenResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct SiteVerificationWebResourceListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of sites that are owned by the authenticated user."]
        pub items: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<SiteVerificationWebResourceResource>>,
        >,
    }
    impl SiteVerificationWebResourceListResponse {
        pub fn builder() -> SiteVerificationWebResourceListResponseBuilder {
            SiteVerificationWebResourceListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct SiteVerificationWebResourceResource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The string used to identify this site. This value should be used in the \"id\" portion of the REST URL for the Get, Update, and Delete operations."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "owners")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email addresses of all verified owners."]
        pub owners: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "site")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The address and type of a site that is verified or will be verified."]
        pub site: ::std::option::Option<SiteVerificationWebResourceResourceSite>,
    }
    impl SiteVerificationWebResourceResource {
        pub fn builder() -> SiteVerificationWebResourceResourceBuilder {
            SiteVerificationWebResourceResourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The address and type of a site that is verified or will be verified."]
    pub struct SiteVerificationWebResourceResourceSite {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "identifier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The site identifier. If the type is set to SITE, the identifier is a URL. If the type is set to INET_DOMAIN, the site identifier is a domain name."]
        pub identifier: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The site type. Can be SITE or INET_DOMAIN (domain name)."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl SiteVerificationWebResourceResourceSite {
        pub fn builder() -> SiteVerificationWebResourceResourceSiteBuilder {
            SiteVerificationWebResourceResourceSiteBuilder::default()
        }
    }
}
