#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SiteVerificationWebResourceGettokenRequest {
    #[serde(rename = "site")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The site for which a verification token will be generated."]
    pub site: ::std::option::Option<SiteVerificationWebResourceGettokenRequestSite>,
    #[serde(rename = "verificationMethod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The verification method that will be used to verify this site. For sites, 'FILE' or 'META' methods may be used. For domains, only 'DNS' may be used."]
    pub verification_method: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The site for which a verification token will be generated."]
pub struct SiteVerificationWebResourceGettokenRequestSite {
    #[serde(rename = "identifier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The site identifier. If the type is set to SITE, the identifier is a URL. If the type is set to INET_DOMAIN, the site identifier is a domain name."]
    pub identifier: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of resource to be verified. Can be SITE or INET_DOMAIN (domain name)."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SiteVerificationWebResourceGettokenResponse {
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The verification method to use in conjunction with this token. For FILE, the token should be placed in the top-level directory of the site, stored inside a file of the same name. For META, the token should be placed in the HEAD tag of the default page that is loaded for the site. For DNS, the token should be placed in a TXT record of the domain."]
    pub method: ::std::option::Option<::std::string::String>,
    #[serde(rename = "token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The verification token. The token must be placed appropriately in order for verification to succeed."]
    pub token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SiteVerificationWebResourceListResponse {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of sites that are owned by the authenticated user."]
    pub items: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<SiteVerificationWebResourceResource>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SiteVerificationWebResourceResource {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The string used to identify this site. This value should be used in the \"id\" portion of the REST URL for the Get, Update, and Delete operations."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "owners")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email addresses of all verified owners."]
    pub owners: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "site")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The address and type of a site that is verified or will be verified."]
    pub site: ::std::option::Option<SiteVerificationWebResourceResourceSite>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The address and type of a site that is verified or will be verified."]
pub struct SiteVerificationWebResourceResourceSite {
    #[serde(rename = "identifier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The site identifier. If the type is set to SITE, the identifier is a URL. If the type is set to INET_DOMAIN, the site identifier is a domain name."]
    pub identifier: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The site type. Can be SITE or INET_DOMAIN (domain name)."]
    pub _type: ::std::option::Option<::std::string::String>,
}
