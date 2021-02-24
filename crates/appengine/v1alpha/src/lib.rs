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
    pub mod apps {
        pub mod resources {
            pub mod authorized_certificates {
                pub mod methods {
                    pub mod get {
                        #[derive(
                            Clone,
                            Debug,
                            PartialEq,
                            derive_builder :: Builder,
                            serde :: Serialize,
                            serde :: Deserialize,
                        )]
                        pub struct QueryParameters {
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "view")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Controls the set of fields returned in the GET response."]
                            pub view: ::std::option::Option<QueryParametersViewEnum>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Controls the set of fields returned in the GET response."]
                        pub enum QueryParametersViewEnum {
                            #[serde(rename = "BASIC_CERTIFICATE")]
                            #[doc = "Basic certificate information, including applicable domains and expiration date."]
                            BasicCertificate,
                            #[serde(rename = "FULL_CERTIFICATE")]
                            #[doc = "The information from BASIC_CERTIFICATE, plus detailed information on the domain mappings that have this certificate mapped."]
                            FullCertificate,
                        }
                        impl ::std::default::Default for QueryParametersViewEnum {
                            fn default() -> Self {
                                Self::BasicCertificate
                            }
                        }
                    }
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum results to return per page."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Continuation token for fetching the next page of results."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "view")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Controls the set of fields returned in the LIST response."]
                            pub view: ::std::option::Option<QueryParametersViewEnum>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Controls the set of fields returned in the LIST response."]
                        pub enum QueryParametersViewEnum {
                            #[serde(rename = "BASIC_CERTIFICATE")]
                            #[doc = "Basic certificate information, including applicable domains and expiration date."]
                            BasicCertificate,
                            #[serde(rename = "FULL_CERTIFICATE")]
                            #[doc = "The information from BASIC_CERTIFICATE, plus detailed information on the domain mappings that have this certificate mapped."]
                            FullCertificate,
                        }
                        impl ::std::default::Default for QueryParametersViewEnum {
                            fn default() -> Self {
                                Self::BasicCertificate
                            }
                        }
                    }
                    pub mod patch {
                        #[derive(
                            Clone,
                            Debug,
                            PartialEq,
                            derive_builder :: Builder,
                            serde :: Serialize,
                            serde :: Deserialize,
                        )]
                        pub struct QueryParameters {
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Standard field mask for the set of fields to be updated. Updates are only supported on the certificate_raw_data and display_name fields."]
                            pub update_mask: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod authorized_domains {
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum results to return per page."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Continuation token for fetching the next page of results."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod domain_mappings {
                pub mod methods {
                    pub mod create {
                        #[derive(
                            Clone,
                            Debug,
                            PartialEq,
                            derive_builder :: Builder,
                            serde :: Serialize,
                            serde :: Deserialize,
                        )]
                        pub struct QueryParameters {
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "noManagedCertificate")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Whether a managed certificate should be provided by App Engine. If true, a certificate ID must be manaually set in the DomainMapping resource to configure SSL for this domain. If false, a managed certificate will be provisioned and a certificate ID will be automatically populated."]
                            pub no_managed_certificate:
                                ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "overrideStrategy")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Whether the domain creation should override any existing mappings for this domain. By default, overrides are rejected."]
                            pub override_strategy:
                                ::std::option::Option<QueryParametersOverrideStrategyEnum>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Whether the domain creation should override any existing mappings for this domain. By default, overrides are rejected."]
                        pub enum QueryParametersOverrideStrategyEnum {
                            #[serde(rename = "UNSPECIFIED_DOMAIN_OVERRIDE_STRATEGY")]
                            #[doc = "Strategy unspecified. Defaults to STRICT."]
                            UnspecifiedDomainOverrideStrategy,
                            #[serde(rename = "STRICT")]
                            #[doc = "Overrides not allowed. If a mapping already exists for the specified domain, the request will return an ALREADY_EXISTS (409)."]
                            Strict,
                            #[serde(rename = "OVERRIDE")]
                            #[doc = "Overrides allowed. If a mapping already exists for the specified domain, the request will overwrite it. Note that this might stop another Google product from serving. For example, if the domain is mapped to another App Engine application, that app will no longer serve from that domain."]
                            Override,
                        }
                        impl ::std::default::Default for QueryParametersOverrideStrategyEnum {
                            fn default() -> Self {
                                Self::UnspecifiedDomainOverrideStrategy
                            }
                        }
                    }
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum results to return per page."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Continuation token for fetching the next page of results."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                    pub mod patch {
                        #[derive(
                            Clone,
                            Debug,
                            PartialEq,
                            derive_builder :: Builder,
                            serde :: Serialize,
                            serde :: Deserialize,
                        )]
                        pub struct QueryParameters {
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "noManagedCertificate")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Whether a managed certificate should be provided by App Engine. If true, a certificate ID must be manually set in the DomainMapping resource to configure SSL for this domain. If false, a managed certificate will be provisioned and a certificate ID will be automatically populated. Only applicable if ssl_settings.certificate_id is specified in the update mask."]
                            pub no_managed_certificate:
                                ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Standard field mask for the set of fields to be updated."]
                            pub update_mask: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod locations {
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "filter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The standard list filter."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The standard list page size."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The standard list page token."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod operations {
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "filter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The standard list filter."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The standard list page size."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The standard list page token."]
                            pub page_token: ::std::option::Option<::std::string::String>,
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
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An SSL certificate that a user has been authorized to administer. A user is authorized to administer any certificate that applies to one of their authorized domains."]
    pub struct AuthorizedCertificate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "certificateRawData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The SSL certificate serving the AuthorizedCertificate resource. This must be obtained independently from a certificate authority."]
        pub certificate_raw_data: ::std::option::Option<::std::boxed::Box<CertificateRawData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user-specified display name of the certificate. This is not guaranteed to be unique. Example: My Certificate."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domainMappingsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Aggregate count of the domain mappings with this certificate mapped. This count includes domain mappings on applications for which the user does not have VIEWER permissions.Only returned by GET or LIST requests when specifically requested by the view=FULL_CERTIFICATE option.@OutputOnly"]
        pub domain_mappings_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domainNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Topmost applicable domains of this certificate. This certificate applies to these domains and their subdomains. Example: example.com.@OutputOnly"]
        pub domain_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expireTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when this certificate expires. To update the renewal time on this certificate, upload an SSL certificate with a different expiration time using AuthorizedCertificates.UpdateAuthorizedCertificate.@OutputOnly"]
        pub expire_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Relative name of the certificate. This is a unique value autogenerated on AuthorizedCertificate resource creation. Example: 12345.@OutputOnly"]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "managedCertificate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Only applicable if this certificate is managed by App Engine. Managed certificates are tied to the lifecycle of a DomainMapping and cannot be updated or deleted via the AuthorizedCertificates API. If this certificate is manually administered by the user, this field will be empty.@OutputOnly"]
        pub managed_certificate: ::std::option::Option<::std::boxed::Box<ManagedCertificate>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Full path to the AuthorizedCertificate resource in the API. Example: apps/myapp/authorizedCertificates/12345.@OutputOnly"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visibleDomainMappings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full paths to user visible Domain Mapping resources that have this certificate mapped. Example: apps/myapp/domainMappings/example.com.This may not represent the full list of mapped domain mappings if the user does not have VIEWER permissions on all of the applications that have this certificate mapped. See domain_mappings_count for a complete count.Only returned by GET or LIST requests when specifically requested by the view=FULL_CERTIFICATE option.@OutputOnly"]
        pub visible_domain_mappings: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl AuthorizedCertificate {
        pub fn builder() -> AuthorizedCertificateBuilder {
            AuthorizedCertificateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A domain that a user has been authorized to administer. To authorize use of a domain, verify ownership via Webmaster Central (https://www.google.com/webmasters/verification/home)."]
    pub struct AuthorizedDomain {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fully qualified domain name of the domain authorized for use. Example: example.com."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Full path to the AuthorizedDomain resource in the API. Example: apps/myapp/authorizedDomains/example.com.@OutputOnly"]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl AuthorizedDomain {
        pub fn builder() -> AuthorizedDomainBuilder {
            AuthorizedDomainBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An SSL certificate obtained from a certificate authority."]
    pub struct CertificateRawData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "privateKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unencrypted PEM encoded RSA private key. This field is set once on certificate creation and then encrypted. The key size must be 2048 bits or fewer. Must include the header and footer. Example: -----BEGIN RSA PRIVATE KEY----- -----END RSA PRIVATE KEY----- @InputOnly"]
        pub private_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publicCertificate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "PEM encoded x.509 public key certificate. This field is set once on certificate creation. Must include the header and footer. Example: -----BEGIN CERTIFICATE----- -----END CERTIFICATE----- "]
        pub public_certificate: ::std::option::Option<::std::string::String>,
    }
    impl CertificateRawData {
        pub fn builder() -> CertificateRawDataBuilder {
            CertificateRawDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for the given google.longrunning.Operation during a google.appengine.v1.CreateVersionRequest."]
    pub struct CreateVersionMetadataV1 {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloudBuildId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Cloud Build ID if one was created as part of the version create. @OutputOnly"]
        pub cloud_build_id: ::std::option::Option<::std::string::String>,
    }
    impl CreateVersionMetadataV1 {
        pub fn builder() -> CreateVersionMetadataV1Builder {
            CreateVersionMetadataV1Builder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for the given google.longrunning.Operation during a google.appengine.v1alpha.CreateVersionRequest."]
    pub struct CreateVersionMetadataV1Alpha {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloudBuildId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Cloud Build ID if one was created as part of the version create. @OutputOnly"]
        pub cloud_build_id: ::std::option::Option<::std::string::String>,
    }
    impl CreateVersionMetadataV1Alpha {
        pub fn builder() -> CreateVersionMetadataV1AlphaBuilder {
            CreateVersionMetadataV1AlphaBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for the given google.longrunning.Operation during a google.appengine.v1beta.CreateVersionRequest."]
    pub struct CreateVersionMetadataV1Beta {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloudBuildId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Cloud Build ID if one was created as part of the version create. @OutputOnly"]
        pub cloud_build_id: ::std::option::Option<::std::string::String>,
    }
    impl CreateVersionMetadataV1Beta {
        pub fn builder() -> CreateVersionMetadataV1BetaBuilder {
            CreateVersionMetadataV1BetaBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A domain serving an App Engine application."]
    pub struct DomainMapping {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Relative name of the domain serving the application. Example: example.com."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Full path to the DomainMapping resource in the API. Example: apps/myapp/domainMapping/example.com.@OutputOnly"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceRecords")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource records required to configure this domain mapping. These records must be added to the domain's DNS configuration in order to serve the application via this domain mapping.@OutputOnly"]
        pub resource_records:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ResourceRecord>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sslSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "SSL configuration for this domain. If unconfigured, this domain will not serve with SSL."]
        pub ssl_settings: ::std::option::Option<::std::boxed::Box<SslSettings>>,
    }
    impl DomainMapping {
        pub fn builder() -> DomainMappingBuilder {
            DomainMappingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for Empty is empty JSON object {}."]
    pub struct Empty {}
    impl Empty {
        pub fn builder() -> EmptyBuilder {
            EmptyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for AuthorizedCertificates.ListAuthorizedCertificates."]
    pub struct ListAuthorizedCertificatesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "certificates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The SSL certificates the user is authorized to administer."]
        pub certificates:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuthorizedCertificate>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token for fetching the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListAuthorizedCertificatesResponse {
        pub fn builder() -> ListAuthorizedCertificatesResponseBuilder {
            ListAuthorizedCertificatesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for AuthorizedDomains.ListAuthorizedDomains."]
    pub struct ListAuthorizedDomainsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domains")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The authorized domains belonging to the user."]
        pub domains: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuthorizedDomain>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token for fetching the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListAuthorizedDomainsResponse {
        pub fn builder() -> ListAuthorizedDomainsResponseBuilder {
            ListAuthorizedDomainsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for DomainMappings.ListDomainMappings."]
    pub struct ListDomainMappingsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domainMappings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The domain mappings for the application."]
        pub domain_mappings:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DomainMapping>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token for fetching the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListDomainMappingsResponse {
        pub fn builder() -> ListDomainMappingsResponseBuilder {
            ListDomainMappingsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Locations.ListLocations."]
    pub struct ListLocationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of locations that matches the specified filter in the request."]
        pub locations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Location>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The standard List next-page token."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListLocationsResponse {
        pub fn builder() -> ListLocationsResponseBuilder {
            ListLocationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Operations.ListOperations."]
    pub struct ListOperationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The standard List next-page token."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of operations that matches the specified filter in the request."]
        pub operations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Operation>>>,
    }
    impl ListOperationsResponse {
        pub fn builder() -> ListOperationsResponseBuilder {
            ListOperationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A resource that represents Google Cloud Platform location."]
    pub struct Location {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The friendly name for this location, typically a nearby city name. For example, \"Tokyo\"."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cross-service attributes for the location. For example {\"cloud.googleapis.com/region\": \"us-east1\"} "]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The canonical id for this location. For example: \"us-east1\"."]
        pub location_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Service-specific metadata. For example the available capacity at the given location."]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name for the location, which may vary between implementations. For example: \"projects/example-project/locations/us-east1\""]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl Location {
        pub fn builder() -> LocationBuilder {
            LocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for the given google.cloud.location.Location."]
    pub struct LocationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "flexibleEnvironmentAvailable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "App Engine flexible environment is available in the given location.@OutputOnly"]
        pub flexible_environment_available: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "searchApiAvailable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Search API (https://cloud.google.com/appengine/docs/standard/python/search) is available in the given location."]
        pub search_api_available: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "standardEnvironmentAvailable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "App Engine standard environment is available in the given location.@OutputOnly"]
        pub standard_environment_available: ::std::option::Option<::std::primitive::bool>,
    }
    impl LocationMetadata {
        pub fn builder() -> LocationMetadataBuilder {
            LocationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A certificate managed by App Engine."]
    pub struct ManagedCertificate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastRenewalTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time at which the certificate was last renewed. The renewal process is fully managed. Certificate renewal will automatically occur before the certificate expires. Renewal errors can be tracked via ManagementStatus.@OutputOnly"]
        pub last_renewal_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of certificate management. Refers to the most recent certificate acquisition or renewal attempt.@OutputOnly"]
        pub status: ::std::option::Option<ManagedCertificateStatusEnum>,
    }
    impl ManagedCertificate {
        pub fn builder() -> ManagedCertificateBuilder {
            ManagedCertificateBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Status of certificate management. Refers to the most recent certificate acquisition or renewal attempt.@OutputOnly"]
    pub enum ManagedCertificateStatusEnum {
        #[serde(rename = "UNSPECIFIED_STATUS")]
        #[doc = ""]
        UnspecifiedStatus,
        #[serde(rename = "OK")]
        #[doc = "Certificate was successfully obtained and inserted into the serving system."]
        Ok,
        #[serde(rename = "PENDING")]
        #[doc = "Certificate is under active attempts to acquire or renew."]
        Pending,
        #[serde(rename = "FAILED_RETRYING_INTERNAL")]
        #[doc = "Most recent renewal failed due to a system failure and will be retried. System failure is likely transient, and subsequent renewal attempts may succeed. The last successfully provisioned certificate may still be serving."]
        FailedRetryingInternal,
        #[serde(rename = "FAILED_RETRYING_NOT_VISIBLE")]
        #[doc = "Most recent renewal failed due to an invalid DNS setup and will be retried. Renewal attempts will continue to fail until the certificate domain's DNS configuration is fixed. The last successfully provisioned certificate may still be serving."]
        FailedRetryingNotVisible,
        #[serde(rename = "FAILED_PERMANENTLY_NOT_VISIBLE")]
        #[doc = "All renewal attempts have been exhausted. Most recent renewal failed due to an invalid DNS setup and will not be retried. The last successfully provisioned certificate may still be serving."]
        FailedPermanentlyNotVisible,
        #[serde(rename = "FAILED_RETRYING_CAA_FORBIDDEN")]
        #[doc = "Most recent renewal failed due to an explicit CAA record that does not include one of the in-use CAs (Google CA and Let's Encrypt). Renewals will continue to fail until the CAA is reconfigured. The last successfully provisioned certificate may still be serving."]
        FailedRetryingCaaForbidden,
        #[serde(rename = "FAILED_RETRYING_CAA_CHECKING")]
        #[doc = "Most recent renewal failed due to a CAA retrieval failure. This means that the domain's DNS provider does not properly handle CAA records, failing requests for CAA records when no CAA records are defined. Renewals will continue to fail until the DNS provider is changed or a CAA record is added for the given domain. The last successfully provisioned certificate may still be serving."]
        FailedRetryingCaaChecking,
    }
    impl ::std::default::Default for ManagedCertificateStatusEnum {
        fn default() -> Self {
            Self::UnspecifiedStatus
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This resource represents a long-running operation that is the result of a network API call."]
    pub struct Operation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "done")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available."]
        pub done: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error result of the operation in case of failure or cancellation."]
        pub error: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any."]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the name should be a resource name ending with operations/{unique_id}."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "response")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The normal response of the operation in case of success. If the original method returns no data on success, such as Delete, the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type XxxResponse, where Xxx is the original method name. For example, if the original method name is TakeSnapshot(), the inferred response type is TakeSnapshotResponse."]
        pub response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl Operation {
        pub fn builder() -> OperationBuilder {
            OperationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for the given google.longrunning.Operation."]
    pub struct OperationMetadataV1 {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createVersionMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub create_version_metadata:
            ::std::option::Option<::std::boxed::Box<CreateVersionMetadataV1>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time that this operation completed.@OutputOnly"]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ephemeralMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ephemeral message that may change every time the operation is polled. @OutputOnly"]
        pub ephemeral_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time that this operation was created.@OutputOnly"]
        pub insert_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "API method that initiated this operation. Example: google.appengine.v1.Versions.CreateVersion.@OutputOnly"]
        pub method: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "target")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the resource that this operation is acting on. Example: apps/myapp/services/default.@OutputOnly"]
        pub target: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "user")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User who requested this operation.@OutputOnly"]
        pub user: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warning")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Durable messages that persist on every operation poll. @OutputOnly"]
        pub warning: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl OperationMetadataV1 {
        pub fn builder() -> OperationMetadataV1Builder {
            OperationMetadataV1Builder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for the given google.longrunning.Operation."]
    pub struct OperationMetadataV1Alpha {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createVersionMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub create_version_metadata:
            ::std::option::Option<::std::boxed::Box<CreateVersionMetadataV1Alpha>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time that this operation completed.@OutputOnly"]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ephemeralMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ephemeral message that may change every time the operation is polled. @OutputOnly"]
        pub ephemeral_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time that this operation was created.@OutputOnly"]
        pub insert_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "API method that initiated this operation. Example: google.appengine.v1alpha.Versions.CreateVersion.@OutputOnly"]
        pub method: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "target")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the resource that this operation is acting on. Example: apps/myapp/services/default.@OutputOnly"]
        pub target: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "user")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User who requested this operation.@OutputOnly"]
        pub user: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warning")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Durable messages that persist on every operation poll. @OutputOnly"]
        pub warning: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl OperationMetadataV1Alpha {
        pub fn builder() -> OperationMetadataV1AlphaBuilder {
            OperationMetadataV1AlphaBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for the given google.longrunning.Operation."]
    pub struct OperationMetadataV1Beta {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createVersionMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub create_version_metadata:
            ::std::option::Option<::std::boxed::Box<CreateVersionMetadataV1Beta>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time that this operation completed.@OutputOnly"]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ephemeralMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ephemeral message that may change every time the operation is polled. @OutputOnly"]
        pub ephemeral_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "insertTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time that this operation was created.@OutputOnly"]
        pub insert_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "API method that initiated this operation. Example: google.appengine.v1beta.Versions.CreateVersion.@OutputOnly"]
        pub method: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "target")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the resource that this operation is acting on. Example: apps/myapp/services/default.@OutputOnly"]
        pub target: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "user")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User who requested this operation.@OutputOnly"]
        pub user: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warning")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Durable messages that persist on every operation poll. @OutputOnly"]
        pub warning: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl OperationMetadataV1Beta {
        pub fn builder() -> OperationMetadataV1BetaBuilder {
            OperationMetadataV1BetaBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A DNS resource record."]
    pub struct ResourceRecord {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Relative name of the object affected by this record. Only applicable for CNAME records. Example: 'www'."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rrdata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data for this record. Values vary by record type, as defined in RFC 1035 (section 5) and RFC 1034 (section 3.6.1)."]
        pub rrdata: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource record type. Example: AAAA."]
        pub _type: ::std::option::Option<ResourceRecordTypeEnum>,
    }
    impl ResourceRecord {
        pub fn builder() -> ResourceRecordBuilder {
            ResourceRecordBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Resource record type. Example: AAAA."]
    pub enum ResourceRecordTypeEnum {
        #[serde(rename = "A")]
        #[doc = "An A resource record. Data is an IPv4 address."]
        A,
        #[serde(rename = "AAAA")]
        #[doc = "An AAAA resource record. Data is an IPv6 address."]
        Aaaa,
        #[serde(rename = "CNAME")]
        #[doc = "A CNAME resource record. Data is a domain name to be aliased."]
        Cname,
    }
    impl ::std::default::Default for ResourceRecordTypeEnum {
        fn default() -> Self {
            Self::A
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "SSL configuration for a DomainMapping resource."]
    pub struct SslSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "certificateId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the AuthorizedCertificate resource configuring SSL for the application. Clearing this field will remove SSL support.By default, a managed certificate is automatically created for every domain mapping. To omit SSL support or to configure SSL manually, specify no_managed_certificate on a CREATE or UPDATE request. You must be authorized to administer the AuthorizedCertificate resource to manually map it to a DomainMapping resource. Example: 12345."]
        pub certificate_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isManagedCertificate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the mapped certificate is an App Engine managed certificate. Managed certificates are created by default with a domain mapping. To opt out, specify no_managed_certificate on a CREATE or UPDATE request.@OutputOnly"]
        pub is_managed_certificate: ::std::option::Option<::std::primitive::bool>,
    }
    impl SslSettings {
        pub fn builder() -> SslSettingsBuilder {
            SslSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Status type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by gRPC (https://github.com/grpc). Each Status message contains three pieces of data: error code, error message, and error details.You can find out more about this error model and how to work with it in the API Design Guide (https://cloud.google.com/apis/design/errors)."]
    pub struct Status {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        pub code: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "details")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of messages that carry the error details. There is a common set of message types for APIs to use."]
        pub details: ::std::option::Option<
            ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client."]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl Status {
        pub fn builder() -> StatusBuilder {
            StatusBuilder::default()
        }
    }
}
