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
    pub mod projects {
        pub mod resources {
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
                pub mod resources {
                    pub mod key_rings {
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
                                    #[serde(rename = "keyRingId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Required. It must be unique within a location and match the regular expression `[a-zA-Z0-9_-]{1,63}`"]
                                    pub key_ring_id: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                            pub mod get_iam_policy {
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
                                    #[serde(rename = "options.requestedPolicyVersion")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional. The policy format version to be returned. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional bindings must specify version 3. Policies without any conditional bindings may specify any valid value or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
                                    pub options_requested_policy_version:
                                        ::std::option::Option<::std::primitive::i64>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
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
                                    #[serde(rename = "filter")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional. Only include resources that match the filter in the response. For more information, see [Sorting and filtering list results](https://cloud.google.com/kms/docs/sorting-and-filtering)."]
                                    pub filter: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "orderBy")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional. Specify how the results should be sorted. If not specified, the results will be sorted in the default order. For more information, see [Sorting and filtering list results](https://cloud.google.com/kms/docs/sorting-and-filtering)."]
                                    pub order_by: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional. Optional limit on the number of KeyRings to include in the response. Further KeyRings can subsequently be obtained by including the ListKeyRingsResponse.next_page_token in a subsequent request. If unspecified, the server will pick an appropriate default."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional. Optional pagination token, returned earlier via ListKeyRingsResponse.next_page_token."]
                                    pub page_token: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                        }
                        pub mod resources {
                            pub mod crypto_keys {
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
                                            #[serde(rename = "cryptoKeyId")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Required. It must be unique within a KeyRing and match the regular expression `[a-zA-Z0-9_-]{1,63}`"]
                                            pub crypto_key_id:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "skipInitialVersionCreation")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "If set to true, the request will create a CryptoKey without any CryptoKeyVersions. You must manually call CreateCryptoKeyVersion or ImportCryptoKeyVersion before you can use this CryptoKey."]
                                            pub skip_initial_version_creation:
                                                ::std::option::Option<::std::primitive::bool>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                    pub mod get_iam_policy {
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
                                            #[serde(rename = "options.requestedPolicyVersion")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Optional. The policy format version to be returned. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional bindings must specify version 3. Policies without any conditional bindings may specify any valid value or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
                                            pub options_requested_policy_version:
                                                ::std::option::Option<::std::primitive::i64>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
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
                                            #[serde(rename = "filter")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Optional. Only include resources that match the filter in the response. For more information, see [Sorting and filtering list results](https://cloud.google.com/kms/docs/sorting-and-filtering)."]
                                            pub filter:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "orderBy")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Optional. Specify how the results should be sorted. If not specified, the results will be sorted in the default order. For more information, see [Sorting and filtering list results](https://cloud.google.com/kms/docs/sorting-and-filtering)."]
                                            pub order_by:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Optional. Optional limit on the number of CryptoKeys to include in the response. Further CryptoKeys can subsequently be obtained by including the ListCryptoKeysResponse.next_page_token in a subsequent request. If unspecified, the server will pick an appropriate default."]
                                            pub page_size:
                                                ::std::option::Option<::std::primitive::i64>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Optional. Optional pagination token, returned earlier via ListCryptoKeysResponse.next_page_token."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "versionView")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The fields of the primary version to include in the response."]
                                            pub version_view: ::std::option::Option<
                                                QueryParametersVersionViewEnum,
                                            >,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                        #[derive(
                                            Debug,
                                            PartialEq,
                                            Copy,
                                            Clone,
                                            serde :: Serialize,
                                            serde :: Deserialize,
                                        )]
                                        #[doc = "The fields of the primary version to include in the response."]
                                        pub enum QueryParametersVersionViewEnum {
                                            #[serde(
                                                rename = "CRYPTO_KEY_VERSION_VIEW_UNSPECIFIED"
                                            )]
                                            #[doc = "Default view for each CryptoKeyVersion. Does not include the attestation field."]
                                            CryptoKeyVersionViewUnspecified,
                                            #[serde(rename = "FULL")]
                                            #[doc = "Provides all fields in each CryptoKeyVersion, including the attestation."]
                                            Full,
                                        }
                                        impl ::std::default::Default for QueryParametersVersionViewEnum {
                                            fn default() -> Self {
                                                Self::CryptoKeyVersionViewUnspecified
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
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Required. List of fields to be updated in this request."]
                                            pub update_mask:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                }
                                pub mod resources {
                                    pub mod crypto_key_versions {
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
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "Optional. Only include resources that match the filter in the response. For more information, see [Sorting and filtering list results](https://cloud.google.com/kms/docs/sorting-and-filtering)."]
                                                    pub filter: ::std::option::Option<
                                                        ::std::string::String,
                                                    >,
                                                    #[builder(
                                                        default = "{ ::std::default::Default::default() }",
                                                        setter(into)
                                                    )]
                                                    #[serde(rename = "orderBy")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "Optional. Specify how the results should be sorted. If not specified, the results will be sorted in the default order. For more information, see [Sorting and filtering list results](https://cloud.google.com/kms/docs/sorting-and-filtering)."]
                                                    pub order_by: ::std::option::Option<
                                                        ::std::string::String,
                                                    >,
                                                    #[builder(
                                                        default = "{ ::std::default::Default::default() }",
                                                        setter(into)
                                                    )]
                                                    #[serde(rename = "pageSize")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "Optional. Optional limit on the number of CryptoKeyVersions to include in the response. Further CryptoKeyVersions can subsequently be obtained by including the ListCryptoKeyVersionsResponse.next_page_token in a subsequent request. If unspecified, the server will pick an appropriate default."]
                                                    pub page_size: ::std::option::Option<
                                                        ::std::primitive::i64,
                                                    >,
                                                    #[builder(
                                                        default = "{ ::std::default::Default::default() }",
                                                        setter(into)
                                                    )]
                                                    #[serde(rename = "pageToken")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "Optional. Optional pagination token, returned earlier via ListCryptoKeyVersionsResponse.next_page_token."]
                                                    pub page_token: ::std::option::Option<
                                                        ::std::string::String,
                                                    >,
                                                    #[builder(
                                                        default = "{ ::std::default::Default::default() }",
                                                        setter(into)
                                                    )]
                                                    #[serde(rename = "view")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "The fields to include in the response."]
                                                    pub view: ::std::option::Option<
                                                        QueryParametersViewEnum,
                                                    >,
                                                }
                                                impl QueryParameters {
                                                    pub fn builder() -> QueryParametersBuilder {
                                                        QueryParametersBuilder::default()
                                                    }
                                                }
                                                #[derive(
                                                    Debug,
                                                    PartialEq,
                                                    Copy,
                                                    Clone,
                                                    serde :: Serialize,
                                                    serde :: Deserialize,
                                                )]
                                                #[doc = "The fields to include in the response."]
                                                pub enum QueryParametersViewEnum {
                                                    #[serde(
                                                        rename = "CRYPTO_KEY_VERSION_VIEW_UNSPECIFIED"
                                                    )]
                                                    #[doc = "Default view for each CryptoKeyVersion. Does not include the attestation field."]
                                                    CryptoKeyVersionViewUnspecified,
                                                    #[serde(rename = "FULL")]
                                                    #[doc = "Provides all fields in each CryptoKeyVersion, including the attestation."]
                                                    Full,
                                                }
                                                impl ::std::default::Default for QueryParametersViewEnum {
                                                    fn default() -> Self {
                                                        Self::CryptoKeyVersionViewUnspecified
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
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "Required. List of fields to be updated in this request."]
                                                    pub update_mask: ::std::option::Option<
                                                        ::std::string::String,
                                                    >,
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
                            pub mod import_jobs {
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
                                            #[serde(rename = "importJobId")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Required. It must be unique within a KeyRing and match the regular expression `[a-zA-Z0-9_-]{1,63}`"]
                                            pub import_job_id:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                    pub mod get_iam_policy {
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
                                            #[serde(rename = "options.requestedPolicyVersion")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Optional. The policy format version to be returned. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional bindings must specify version 3. Policies without any conditional bindings may specify any valid value or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
                                            pub options_requested_policy_version:
                                                ::std::option::Option<::std::primitive::i64>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
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
                                            #[serde(rename = "filter")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Optional. Only include resources that match the filter in the response. For more information, see [Sorting and filtering list results](https://cloud.google.com/kms/docs/sorting-and-filtering)."]
                                            pub filter:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "orderBy")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Optional. Specify how the results should be sorted. If not specified, the results will be sorted in the default order. For more information, see [Sorting and filtering list results](https://cloud.google.com/kms/docs/sorting-and-filtering)."]
                                            pub order_by:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Optional. Optional limit on the number of ImportJobs to include in the response. Further ImportJobs can subsequently be obtained by including the ListImportJobsResponse.next_page_token in a subsequent request. If unspecified, the server will pick an appropriate default."]
                                            pub page_size:
                                                ::std::option::Option<::std::primitive::i64>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Optional. Optional pagination token, returned earlier via ListImportJobsResponse.next_page_token."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
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
            }
        }
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for KeyManagementService.AsymmetricDecrypt."]
    pub struct AsymmetricDecryptRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ciphertext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The data encrypted with the named CryptoKeyVersion's public key using OAEP."]
        pub ciphertext: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ciphertextCrc32c")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. An optional CRC32C checksum of the AsymmetricDecryptRequest.ciphertext. If specified, KeyManagementService will verify the integrity of the received AsymmetricDecryptRequest.ciphertext using this checksum. KeyManagementService will report an error if the checksum verification fails. If you receive a checksum error, your client should verify that CRC32C(AsymmetricDecryptRequest.ciphertext) is equal to AsymmetricDecryptRequest.ciphertext_crc32c, and if so, perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type. NOTE: This field is in Beta."]
        pub ciphertext_crc32c: ::std::option::Option<::std::string::String>,
    }
    impl AsymmetricDecryptRequest {
        pub fn builder() -> AsymmetricDecryptRequestBuilder {
            AsymmetricDecryptRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for KeyManagementService.AsymmetricDecrypt."]
    pub struct AsymmetricDecryptResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "plaintext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The decrypted data originally encrypted with the matching public key."]
        pub plaintext: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "plaintextCrc32c")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Integrity verification field. A CRC32C checksum of the returned AsymmetricDecryptResponse.plaintext. An integrity check of AsymmetricDecryptResponse.plaintext can be performed by computing the CRC32C checksum of AsymmetricDecryptResponse.plaintext and comparing your results to this field. Discard the response in case of non-matching checksum values, and perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type. NOTE: This field is in Beta."]
        pub plaintext_crc32c: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "protectionLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ProtectionLevel of the CryptoKeyVersion used in decryption."]
        pub protection_level: ::std::option::Option<AsymmetricDecryptResponseProtectionLevelEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verifiedCiphertextCrc32c")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Integrity verification field. A flag indicating whether AsymmetricDecryptRequest.ciphertext_crc32c was received by KeyManagementService and used for the integrity verification of the ciphertext. A false value of this field indicates either that AsymmetricDecryptRequest.ciphertext_crc32c was left unset or that it was not delivered to KeyManagementService. If you've set AsymmetricDecryptRequest.ciphertext_crc32c but this field is still false, discard the response and perform a limited number of retries. NOTE: This field is in Beta."]
        pub verified_ciphertext_crc32c: ::std::option::Option<::std::primitive::bool>,
    }
    impl AsymmetricDecryptResponse {
        pub fn builder() -> AsymmetricDecryptResponseBuilder {
            AsymmetricDecryptResponseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The ProtectionLevel of the CryptoKeyVersion used in decryption."]
    pub enum AsymmetricDecryptResponseProtectionLevelEnum {
        #[serde(rename = "PROTECTION_LEVEL_UNSPECIFIED")]
        #[doc = "Not specified."]
        ProtectionLevelUnspecified,
        #[serde(rename = "SOFTWARE")]
        #[doc = "Crypto operations are performed in software."]
        Software,
        #[serde(rename = "HSM")]
        #[doc = "Crypto operations are performed in a Hardware Security Module."]
        Hsm,
        #[serde(rename = "EXTERNAL")]
        #[doc = "Crypto operations are performed by an external key manager."]
        External,
    }
    impl ::std::default::Default for AsymmetricDecryptResponseProtectionLevelEnum {
        fn default() -> Self {
            Self::ProtectionLevelUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for KeyManagementService.AsymmetricSign."]
    pub struct AsymmetricSignRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "digest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The digest of the data to sign. The digest must be produced with the same digest algorithm as specified by the key version's algorithm."]
        pub digest: ::std::option::Option<::std::boxed::Box<Digest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "digestCrc32c")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. An optional CRC32C checksum of the AsymmetricSignRequest.digest. If specified, KeyManagementService will verify the integrity of the received AsymmetricSignRequest.digest using this checksum. KeyManagementService will report an error if the checksum verification fails. If you receive a checksum error, your client should verify that CRC32C(AsymmetricSignRequest.digest) is equal to AsymmetricSignRequest.digest_crc32c, and if so, perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type. NOTE: This field is in Beta."]
        pub digest_crc32c: ::std::option::Option<::std::string::String>,
    }
    impl AsymmetricSignRequest {
        pub fn builder() -> AsymmetricSignRequestBuilder {
            AsymmetricSignRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for KeyManagementService.AsymmetricSign."]
    pub struct AsymmetricSignResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the CryptoKeyVersion used for signing. Check this field to verify that the intended resource was used for signing. NOTE: This field is in Beta."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "protectionLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ProtectionLevel of the CryptoKeyVersion used for signing."]
        pub protection_level: ::std::option::Option<AsymmetricSignResponseProtectionLevelEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "signature")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The created signature."]
        pub signature: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "signatureCrc32c")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Integrity verification field. A CRC32C checksum of the returned AsymmetricSignResponse.signature. An integrity check of AsymmetricSignResponse.signature can be performed by computing the CRC32C checksum of AsymmetricSignResponse.signature and comparing your results to this field. Discard the response in case of non-matching checksum values, and perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type. NOTE: This field is in Beta."]
        pub signature_crc32c: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verifiedDigestCrc32c")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Integrity verification field. A flag indicating whether AsymmetricSignRequest.digest_crc32c was received by KeyManagementService and used for the integrity verification of the digest. A false value of this field indicates either that AsymmetricSignRequest.digest_crc32c was left unset or that it was not delivered to KeyManagementService. If you've set AsymmetricSignRequest.digest_crc32c but this field is still false, discard the response and perform a limited number of retries. NOTE: This field is in Beta."]
        pub verified_digest_crc32c: ::std::option::Option<::std::primitive::bool>,
    }
    impl AsymmetricSignResponse {
        pub fn builder() -> AsymmetricSignResponseBuilder {
            AsymmetricSignResponseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The ProtectionLevel of the CryptoKeyVersion used for signing."]
    pub enum AsymmetricSignResponseProtectionLevelEnum {
        #[serde(rename = "PROTECTION_LEVEL_UNSPECIFIED")]
        #[doc = "Not specified."]
        ProtectionLevelUnspecified,
        #[serde(rename = "SOFTWARE")]
        #[doc = "Crypto operations are performed in software."]
        Software,
        #[serde(rename = "HSM")]
        #[doc = "Crypto operations are performed in a Hardware Security Module."]
        Hsm,
        #[serde(rename = "EXTERNAL")]
        #[doc = "Crypto operations are performed by an external key manager."]
        External,
    }
    impl ::std::default::Default for AsymmetricSignResponseProtectionLevelEnum {
        fn default() -> Self {
            Self::ProtectionLevelUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { \"audit_configs\": [ { \"service\": \"allServices\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" }, { \"log_type\": \"ADMIN_READ\" } ] }, { \"service\": \"sampleservice.googleapis.com\", \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\" }, { \"log_type\": \"DATA_WRITE\", \"exempted_members\": [ \"user:aliya@example.com\" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts jose@example.com from DATA_READ logging, and aliya@example.com from DATA_WRITE logging."]
    pub struct AuditConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "auditLogConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The configuration for logging of each type of permission."]
        pub audit_log_configs:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuditLogConfig>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "service")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services."]
        pub service: ::std::option::Option<::std::string::String>,
    }
    impl AuditConfig {
        pub fn builder() -> AuditConfigBuilder {
            AuditConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Provides the configuration for logging a type of permissions. Example: { \"audit_log_configs\": [ { \"log_type\": \"DATA_READ\", \"exempted_members\": [ \"user:jose@example.com\" ] }, { \"log_type\": \"DATA_WRITE\" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging."]
    pub struct AuditLogConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exemptedMembers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members."]
        pub exempted_members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The log type that this config enables."]
        pub log_type: ::std::option::Option<AuditLogConfigLogTypeEnum>,
    }
    impl AuditLogConfig {
        pub fn builder() -> AuditLogConfigBuilder {
            AuditLogConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The log type that this config enables."]
    pub enum AuditLogConfigLogTypeEnum {
        #[serde(rename = "LOG_TYPE_UNSPECIFIED")]
        #[doc = "Default case. Should never be this."]
        LogTypeUnspecified,
        #[serde(rename = "ADMIN_READ")]
        #[doc = "Admin reads. Example: CloudIAM getIamPolicy"]
        AdminRead,
        #[serde(rename = "DATA_WRITE")]
        #[doc = "Data writes. Example: CloudSQL Users create"]
        DataWrite,
        #[serde(rename = "DATA_READ")]
        #[doc = "Data reads. Example: CloudSQL Users list"]
        DataRead,
    }
    impl ::std::default::Default for AuditLogConfigLogTypeEnum {
        fn default() -> Self {
            Self::LogTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Associates `members` with a `role`."]
    pub struct Binding {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "condition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the members in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        pub condition: ::std::option::Option<::std::boxed::Box<Expr>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "members")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the identities requesting access for a Cloud Platform resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. "]
        pub members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Role that is assigned to `members`. For example, `roles/viewer`, `roles/editor`, or `roles/owner`."]
        pub role: ::std::option::Option<::std::string::String>,
    }
    impl Binding {
        pub fn builder() -> BindingBuilder {
            BindingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Certificate chains needed to verify the attestation. Certificates in chains are PEM-encoded and are ordered based on https://tools.ietf.org/html/rfc5246#section-7.4.2."]
    pub struct CertificateChains {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "caviumCerts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cavium certificate chain corresponding to the attestation."]
        pub cavium_certs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "googleCardCerts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google card certificate chain corresponding to the attestation."]
        pub google_card_certs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "googlePartitionCerts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google partition certificate chain corresponding to the attestation."]
        pub google_partition_certs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl CertificateChains {
        pub fn builder() -> CertificateChainsBuilder {
            CertificateChainsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A CryptoKey represents a logical key that can be used for cryptographic operations. A CryptoKey is made up of zero or more versions, which represent the actual key material used in cryptographic operations."]
    pub struct CryptoKey {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time at which this CryptoKey was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Labels with user-defined metadata. For more information, see [Labeling Keys](https://cloud.google.com/kms/docs/labeling-keys)."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The resource name for this CryptoKey in the format `projects/*/locations/*/keyRings/*/cryptoKeys/*`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextRotationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "At next_rotation_time, the Key Management Service will automatically: 1. Create a new version of this CryptoKey. 2. Mark the new version as primary. Key rotations performed manually via CreateCryptoKeyVersion and UpdateCryptoKeyPrimaryVersion do not affect next_rotation_time. Keys with purpose ENCRYPT_DECRYPT support automatic rotation. For other keys, this field must be omitted."]
        pub next_rotation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A copy of the \"primary\" CryptoKeyVersion that will be used by Encrypt when this CryptoKey is given in EncryptRequest.name. The CryptoKey's primary version can be updated via UpdateCryptoKeyPrimaryVersion. Keys with purpose ENCRYPT_DECRYPT may have a primary. For other keys, this field will be omitted."]
        pub primary: ::std::option::Option<::std::boxed::Box<CryptoKeyVersion>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "purpose")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Immutable. The immutable purpose of this CryptoKey."]
        pub purpose: ::std::option::Option<CryptoKeyPurposeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rotationPeriod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "next_rotation_time will be advanced by this period when the service automatically rotates a key. Must be at least 24 hours and at most 876,000 hours. If rotation_period is set, next_rotation_time must also be set. Keys with purpose ENCRYPT_DECRYPT support automatic rotation. For other keys, this field must be omitted."]
        pub rotation_period: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionTemplate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A template describing settings for new CryptoKeyVersion instances. The properties of new CryptoKeyVersion instances created by either CreateCryptoKeyVersion or auto-rotation are controlled by this template."]
        pub version_template: ::std::option::Option<::std::boxed::Box<CryptoKeyVersionTemplate>>,
    }
    impl CryptoKey {
        pub fn builder() -> CryptoKeyBuilder {
            CryptoKeyBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Immutable. The immutable purpose of this CryptoKey."]
    pub enum CryptoKeyPurposeEnum {
        #[serde(rename = "CRYPTO_KEY_PURPOSE_UNSPECIFIED")]
        #[doc = "Not specified."]
        CryptoKeyPurposeUnspecified,
        #[serde(rename = "ENCRYPT_DECRYPT")]
        #[doc = "CryptoKeys with this purpose may be used with Encrypt and Decrypt."]
        EncryptDecrypt,
        #[serde(rename = "ASYMMETRIC_SIGN")]
        #[doc = "CryptoKeys with this purpose may be used with AsymmetricSign and GetPublicKey."]
        AsymmetricSign,
        #[serde(rename = "ASYMMETRIC_DECRYPT")]
        #[doc = "CryptoKeys with this purpose may be used with AsymmetricDecrypt and GetPublicKey."]
        AsymmetricDecrypt,
    }
    impl ::std::default::Default for CryptoKeyPurposeEnum {
        fn default() -> Self {
            Self::CryptoKeyPurposeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A CryptoKeyVersion represents an individual cryptographic key, and the associated key material. An ENABLED version can be used for cryptographic operations. For security reasons, the raw cryptographic key material represented by a CryptoKeyVersion can never be viewed or exported. It can only be used to encrypt, decrypt, or sign data when an authorized user or application invokes Cloud KMS."]
    pub struct CryptoKeyVersion {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "algorithm")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The CryptoKeyVersionAlgorithm that this CryptoKeyVersion supports."]
        pub algorithm: ::std::option::Option<CryptoKeyVersionAlgorithmEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attestation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Statement that was generated and signed by the HSM at key creation time. Use this statement to verify attributes of the key as stored on the HSM, independently of Google. Only provided for key versions with protection_level HSM."]
        pub attestation: ::std::option::Option<::std::boxed::Box<KeyOperationAttestation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time at which this CryptoKeyVersion was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destroyEventTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time this CryptoKeyVersion's key material was destroyed. Only present if state is DESTROYED."]
        pub destroy_event_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destroyTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time this CryptoKeyVersion's key material is scheduled for destruction. Only present if state is DESTROY_SCHEDULED."]
        pub destroy_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "externalProtectionLevelOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ExternalProtectionLevelOptions stores a group of additional fields for configuring a CryptoKeyVersion that are specific to the EXTERNAL protection level."]
        pub external_protection_level_options:
            ::std::option::Option<::std::boxed::Box<ExternalProtectionLevelOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "generateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time this CryptoKeyVersion's key material was generated."]
        pub generate_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importFailureReason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The root cause of an import failure. Only present if state is IMPORT_FAILED."]
        pub import_failure_reason: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importJob")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of the ImportJob used to import this CryptoKeyVersion. Only present if the underlying key material was imported."]
        pub import_job: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time at which this CryptoKeyVersion's key material was imported."]
        pub import_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The resource name for this CryptoKeyVersion in the format `projects/*/locations/*/keyRings/*/cryptoKeys/*/cryptoKeyVersions/*`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "protectionLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The ProtectionLevel describing how crypto operations are performed with this CryptoKeyVersion."]
        pub protection_level: ::std::option::Option<CryptoKeyVersionProtectionLevelEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current state of the CryptoKeyVersion."]
        pub state: ::std::option::Option<CryptoKeyVersionStateEnum>,
    }
    impl CryptoKeyVersion {
        pub fn builder() -> CryptoKeyVersionBuilder {
            CryptoKeyVersionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The CryptoKeyVersionAlgorithm that this CryptoKeyVersion supports."]
    pub enum CryptoKeyVersionAlgorithmEnum {
        #[serde(rename = "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED")]
        #[doc = "Not specified."]
        CryptoKeyVersionAlgorithmUnspecified,
        #[serde(rename = "GOOGLE_SYMMETRIC_ENCRYPTION")]
        #[doc = "Creates symmetric encryption keys."]
        GoogleSymmetricEncryption,
        #[serde(rename = "RSA_SIGN_PSS_2048_SHA256")]
        #[doc = "RSASSA-PSS 2048 bit key with a SHA256 digest."]
        RsaSignPss2048Sha256,
        #[serde(rename = "RSA_SIGN_PSS_3072_SHA256")]
        #[doc = "RSASSA-PSS 3072 bit key with a SHA256 digest."]
        RsaSignPss3072Sha256,
        #[serde(rename = "RSA_SIGN_PSS_4096_SHA256")]
        #[doc = "RSASSA-PSS 4096 bit key with a SHA256 digest."]
        RsaSignPss4096Sha256,
        #[serde(rename = "RSA_SIGN_PSS_4096_SHA512")]
        #[doc = "RSASSA-PSS 4096 bit key with a SHA512 digest."]
        RsaSignPss4096Sha512,
        #[serde(rename = "RSA_SIGN_PKCS1_2048_SHA256")]
        #[doc = "RSASSA-PKCS1-v1_5 with a 2048 bit key and a SHA256 digest."]
        RsaSignPkcs12048Sha256,
        #[serde(rename = "RSA_SIGN_PKCS1_3072_SHA256")]
        #[doc = "RSASSA-PKCS1-v1_5 with a 3072 bit key and a SHA256 digest."]
        RsaSignPkcs13072Sha256,
        #[serde(rename = "RSA_SIGN_PKCS1_4096_SHA256")]
        #[doc = "RSASSA-PKCS1-v1_5 with a 4096 bit key and a SHA256 digest."]
        RsaSignPkcs14096Sha256,
        #[serde(rename = "RSA_SIGN_PKCS1_4096_SHA512")]
        #[doc = "RSASSA-PKCS1-v1_5 with a 4096 bit key and a SHA512 digest."]
        RsaSignPkcs14096Sha512,
        #[serde(rename = "RSA_DECRYPT_OAEP_2048_SHA256")]
        #[doc = "RSAES-OAEP 2048 bit key with a SHA256 digest."]
        RsaDecryptOaep2048Sha256,
        #[serde(rename = "RSA_DECRYPT_OAEP_3072_SHA256")]
        #[doc = "RSAES-OAEP 3072 bit key with a SHA256 digest."]
        RsaDecryptOaep3072Sha256,
        #[serde(rename = "RSA_DECRYPT_OAEP_4096_SHA256")]
        #[doc = "RSAES-OAEP 4096 bit key with a SHA256 digest."]
        RsaDecryptOaep4096Sha256,
        #[serde(rename = "RSA_DECRYPT_OAEP_4096_SHA512")]
        #[doc = "RSAES-OAEP 4096 bit key with a SHA512 digest."]
        RsaDecryptOaep4096Sha512,
        #[serde(rename = "EC_SIGN_P256_SHA256")]
        #[doc = "ECDSA on the NIST P-256 curve with a SHA256 digest."]
        EcSignP256Sha256,
        #[serde(rename = "EC_SIGN_P384_SHA384")]
        #[doc = "ECDSA on the NIST P-384 curve with a SHA384 digest."]
        EcSignP384Sha384,
        #[serde(rename = "EXTERNAL_SYMMETRIC_ENCRYPTION")]
        #[doc = "Algorithm representing symmetric encryption by an external key manager."]
        ExternalSymmetricEncryption,
    }
    impl ::std::default::Default for CryptoKeyVersionAlgorithmEnum {
        fn default() -> Self {
            Self::CryptoKeyVersionAlgorithmUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The ProtectionLevel describing how crypto operations are performed with this CryptoKeyVersion."]
    pub enum CryptoKeyVersionProtectionLevelEnum {
        #[serde(rename = "PROTECTION_LEVEL_UNSPECIFIED")]
        #[doc = "Not specified."]
        ProtectionLevelUnspecified,
        #[serde(rename = "SOFTWARE")]
        #[doc = "Crypto operations are performed in software."]
        Software,
        #[serde(rename = "HSM")]
        #[doc = "Crypto operations are performed in a Hardware Security Module."]
        Hsm,
        #[serde(rename = "EXTERNAL")]
        #[doc = "Crypto operations are performed by an external key manager."]
        External,
    }
    impl ::std::default::Default for CryptoKeyVersionProtectionLevelEnum {
        fn default() -> Self {
            Self::ProtectionLevelUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The current state of the CryptoKeyVersion."]
    pub enum CryptoKeyVersionStateEnum {
        #[serde(rename = "CRYPTO_KEY_VERSION_STATE_UNSPECIFIED")]
        #[doc = "Not specified."]
        CryptoKeyVersionStateUnspecified,
        #[serde(rename = "PENDING_GENERATION")]
        #[doc = "This version is still being generated. It may not be used, enabled, disabled, or destroyed yet. Cloud KMS will automatically mark this version ENABLED as soon as the version is ready."]
        PendingGeneration,
        #[serde(rename = "ENABLED")]
        #[doc = "This version may be used for cryptographic operations."]
        Enabled,
        #[serde(rename = "DISABLED")]
        #[doc = "This version may not be used, but the key material is still available, and the version can be placed back into the ENABLED state."]
        Disabled,
        #[serde(rename = "DESTROYED")]
        #[doc = "This version is destroyed, and the key material is no longer stored. A version may not leave this state once entered."]
        Destroyed,
        #[serde(rename = "DESTROY_SCHEDULED")]
        #[doc = "This version is scheduled for destruction, and will be destroyed soon. Call RestoreCryptoKeyVersion to put it back into the DISABLED state."]
        DestroyScheduled,
        #[serde(rename = "PENDING_IMPORT")]
        #[doc = "This version is still being imported. It may not be used, enabled, disabled, or destroyed yet. Cloud KMS will automatically mark this version ENABLED as soon as the version is ready."]
        PendingImport,
        #[serde(rename = "IMPORT_FAILED")]
        #[doc = "This version was not imported successfully. It may not be used, enabled, disabled, or destroyed. The submitted key material has been discarded. Additional details can be found in CryptoKeyVersion.import_failure_reason."]
        ImportFailed,
    }
    impl ::std::default::Default for CryptoKeyVersionStateEnum {
        fn default() -> Self {
            Self::CryptoKeyVersionStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A CryptoKeyVersionTemplate specifies the properties to use when creating a new CryptoKeyVersion, either manually with CreateCryptoKeyVersion or automatically as a result of auto-rotation."]
    pub struct CryptoKeyVersionTemplate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "algorithm")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Algorithm to use when creating a CryptoKeyVersion based on this template. For backwards compatibility, GOOGLE_SYMMETRIC_ENCRYPTION is implied if both this field is omitted and CryptoKey.purpose is ENCRYPT_DECRYPT."]
        pub algorithm: ::std::option::Option<CryptoKeyVersionTemplateAlgorithmEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "protectionLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ProtectionLevel to use when creating a CryptoKeyVersion based on this template. Immutable. Defaults to SOFTWARE."]
        pub protection_level: ::std::option::Option<CryptoKeyVersionTemplateProtectionLevelEnum>,
    }
    impl CryptoKeyVersionTemplate {
        pub fn builder() -> CryptoKeyVersionTemplateBuilder {
            CryptoKeyVersionTemplateBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Algorithm to use when creating a CryptoKeyVersion based on this template. For backwards compatibility, GOOGLE_SYMMETRIC_ENCRYPTION is implied if both this field is omitted and CryptoKey.purpose is ENCRYPT_DECRYPT."]
    pub enum CryptoKeyVersionTemplateAlgorithmEnum {
        #[serde(rename = "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED")]
        #[doc = "Not specified."]
        CryptoKeyVersionAlgorithmUnspecified,
        #[serde(rename = "GOOGLE_SYMMETRIC_ENCRYPTION")]
        #[doc = "Creates symmetric encryption keys."]
        GoogleSymmetricEncryption,
        #[serde(rename = "RSA_SIGN_PSS_2048_SHA256")]
        #[doc = "RSASSA-PSS 2048 bit key with a SHA256 digest."]
        RsaSignPss2048Sha256,
        #[serde(rename = "RSA_SIGN_PSS_3072_SHA256")]
        #[doc = "RSASSA-PSS 3072 bit key with a SHA256 digest."]
        RsaSignPss3072Sha256,
        #[serde(rename = "RSA_SIGN_PSS_4096_SHA256")]
        #[doc = "RSASSA-PSS 4096 bit key with a SHA256 digest."]
        RsaSignPss4096Sha256,
        #[serde(rename = "RSA_SIGN_PSS_4096_SHA512")]
        #[doc = "RSASSA-PSS 4096 bit key with a SHA512 digest."]
        RsaSignPss4096Sha512,
        #[serde(rename = "RSA_SIGN_PKCS1_2048_SHA256")]
        #[doc = "RSASSA-PKCS1-v1_5 with a 2048 bit key and a SHA256 digest."]
        RsaSignPkcs12048Sha256,
        #[serde(rename = "RSA_SIGN_PKCS1_3072_SHA256")]
        #[doc = "RSASSA-PKCS1-v1_5 with a 3072 bit key and a SHA256 digest."]
        RsaSignPkcs13072Sha256,
        #[serde(rename = "RSA_SIGN_PKCS1_4096_SHA256")]
        #[doc = "RSASSA-PKCS1-v1_5 with a 4096 bit key and a SHA256 digest."]
        RsaSignPkcs14096Sha256,
        #[serde(rename = "RSA_SIGN_PKCS1_4096_SHA512")]
        #[doc = "RSASSA-PKCS1-v1_5 with a 4096 bit key and a SHA512 digest."]
        RsaSignPkcs14096Sha512,
        #[serde(rename = "RSA_DECRYPT_OAEP_2048_SHA256")]
        #[doc = "RSAES-OAEP 2048 bit key with a SHA256 digest."]
        RsaDecryptOaep2048Sha256,
        #[serde(rename = "RSA_DECRYPT_OAEP_3072_SHA256")]
        #[doc = "RSAES-OAEP 3072 bit key with a SHA256 digest."]
        RsaDecryptOaep3072Sha256,
        #[serde(rename = "RSA_DECRYPT_OAEP_4096_SHA256")]
        #[doc = "RSAES-OAEP 4096 bit key with a SHA256 digest."]
        RsaDecryptOaep4096Sha256,
        #[serde(rename = "RSA_DECRYPT_OAEP_4096_SHA512")]
        #[doc = "RSAES-OAEP 4096 bit key with a SHA512 digest."]
        RsaDecryptOaep4096Sha512,
        #[serde(rename = "EC_SIGN_P256_SHA256")]
        #[doc = "ECDSA on the NIST P-256 curve with a SHA256 digest."]
        EcSignP256Sha256,
        #[serde(rename = "EC_SIGN_P384_SHA384")]
        #[doc = "ECDSA on the NIST P-384 curve with a SHA384 digest."]
        EcSignP384Sha384,
        #[serde(rename = "EXTERNAL_SYMMETRIC_ENCRYPTION")]
        #[doc = "Algorithm representing symmetric encryption by an external key manager."]
        ExternalSymmetricEncryption,
    }
    impl ::std::default::Default for CryptoKeyVersionTemplateAlgorithmEnum {
        fn default() -> Self {
            Self::CryptoKeyVersionAlgorithmUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "ProtectionLevel to use when creating a CryptoKeyVersion based on this template. Immutable. Defaults to SOFTWARE."]
    pub enum CryptoKeyVersionTemplateProtectionLevelEnum {
        #[serde(rename = "PROTECTION_LEVEL_UNSPECIFIED")]
        #[doc = "Not specified."]
        ProtectionLevelUnspecified,
        #[serde(rename = "SOFTWARE")]
        #[doc = "Crypto operations are performed in software."]
        Software,
        #[serde(rename = "HSM")]
        #[doc = "Crypto operations are performed in a Hardware Security Module."]
        Hsm,
        #[serde(rename = "EXTERNAL")]
        #[doc = "Crypto operations are performed by an external key manager."]
        External,
    }
    impl ::std::default::Default for CryptoKeyVersionTemplateProtectionLevelEnum {
        fn default() -> Self {
            Self::ProtectionLevelUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for KeyManagementService.Decrypt."]
    pub struct DecryptRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalAuthenticatedData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Optional data that must match the data originally supplied in EncryptRequest.additional_authenticated_data."]
        pub additional_authenticated_data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalAuthenticatedDataCrc32c")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. An optional CRC32C checksum of the DecryptRequest.additional_authenticated_data. If specified, KeyManagementService will verify the integrity of the received DecryptRequest.additional_authenticated_data using this checksum. KeyManagementService will report an error if the checksum verification fails. If you receive a checksum error, your client should verify that CRC32C(DecryptRequest.additional_authenticated_data) is equal to DecryptRequest.additional_authenticated_data_crc32c, and if so, perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type. NOTE: This field is in Beta."]
        pub additional_authenticated_data_crc32c: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ciphertext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The encrypted data originally returned in EncryptResponse.ciphertext."]
        pub ciphertext: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ciphertextCrc32c")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. An optional CRC32C checksum of the DecryptRequest.ciphertext. If specified, KeyManagementService will verify the integrity of the received DecryptRequest.ciphertext using this checksum. KeyManagementService will report an error if the checksum verification fails. If you receive a checksum error, your client should verify that CRC32C(DecryptRequest.ciphertext) is equal to DecryptRequest.ciphertext_crc32c, and if so, perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type. NOTE: This field is in Beta."]
        pub ciphertext_crc32c: ::std::option::Option<::std::string::String>,
    }
    impl DecryptRequest {
        pub fn builder() -> DecryptRequestBuilder {
            DecryptRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for KeyManagementService.Decrypt."]
    pub struct DecryptResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "plaintext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The decrypted data originally supplied in EncryptRequest.plaintext."]
        pub plaintext: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "plaintextCrc32c")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Integrity verification field. A CRC32C checksum of the returned DecryptResponse.plaintext. An integrity check of DecryptResponse.plaintext can be performed by computing the CRC32C checksum of DecryptResponse.plaintext and comparing your results to this field. Discard the response in case of non-matching checksum values, and perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: receiving this response message indicates that KeyManagementService is able to successfully decrypt the ciphertext. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type. NOTE: This field is in Beta."]
        pub plaintext_crc32c: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "protectionLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ProtectionLevel of the CryptoKeyVersion used in decryption."]
        pub protection_level: ::std::option::Option<DecryptResponseProtectionLevelEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "usedPrimary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the Decryption was performed using the primary key version."]
        pub used_primary: ::std::option::Option<::std::primitive::bool>,
    }
    impl DecryptResponse {
        pub fn builder() -> DecryptResponseBuilder {
            DecryptResponseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The ProtectionLevel of the CryptoKeyVersion used in decryption."]
    pub enum DecryptResponseProtectionLevelEnum {
        #[serde(rename = "PROTECTION_LEVEL_UNSPECIFIED")]
        #[doc = "Not specified."]
        ProtectionLevelUnspecified,
        #[serde(rename = "SOFTWARE")]
        #[doc = "Crypto operations are performed in software."]
        Software,
        #[serde(rename = "HSM")]
        #[doc = "Crypto operations are performed in a Hardware Security Module."]
        Hsm,
        #[serde(rename = "EXTERNAL")]
        #[doc = "Crypto operations are performed by an external key manager."]
        External,
    }
    impl ::std::default::Default for DecryptResponseProtectionLevelEnum {
        fn default() -> Self {
            Self::ProtectionLevelUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for KeyManagementService.DestroyCryptoKeyVersion."]
    pub struct DestroyCryptoKeyVersionRequest {}
    impl DestroyCryptoKeyVersionRequest {
        pub fn builder() -> DestroyCryptoKeyVersionRequestBuilder {
            DestroyCryptoKeyVersionRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Digest holds a cryptographic message digest."]
    pub struct Digest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sha256")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A message digest produced with the SHA-256 algorithm."]
        pub sha256: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sha384")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A message digest produced with the SHA-384 algorithm."]
        pub sha384: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sha512")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A message digest produced with the SHA-512 algorithm."]
        pub sha512: ::std::option::Option<::std::string::String>,
    }
    impl Digest {
        pub fn builder() -> DigestBuilder {
            DigestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for KeyManagementService.Encrypt."]
    pub struct EncryptRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalAuthenticatedData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Optional data that, if specified, must also be provided during decryption through DecryptRequest.additional_authenticated_data. The maximum size depends on the key version's protection_level. For SOFTWARE keys, the AAD must be no larger than 64KiB. For HSM keys, the combined length of the plaintext and additional_authenticated_data fields must be no larger than 8KiB."]
        pub additional_authenticated_data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalAuthenticatedDataCrc32c")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. An optional CRC32C checksum of the EncryptRequest.additional_authenticated_data. If specified, KeyManagementService will verify the integrity of the received EncryptRequest.additional_authenticated_data using this checksum. KeyManagementService will report an error if the checksum verification fails. If you receive a checksum error, your client should verify that CRC32C(EncryptRequest.additional_authenticated_data) is equal to EncryptRequest.additional_authenticated_data_crc32c, and if so, perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type. NOTE: This field is in Beta."]
        pub additional_authenticated_data_crc32c: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "plaintext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The data to encrypt. Must be no larger than 64KiB. The maximum size depends on the key version's protection_level. For SOFTWARE keys, the plaintext must be no larger than 64KiB. For HSM keys, the combined length of the plaintext and additional_authenticated_data fields must be no larger than 8KiB."]
        pub plaintext: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "plaintextCrc32c")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. An optional CRC32C checksum of the EncryptRequest.plaintext. If specified, KeyManagementService will verify the integrity of the received EncryptRequest.plaintext using this checksum. KeyManagementService will report an error if the checksum verification fails. If you receive a checksum error, your client should verify that CRC32C(EncryptRequest.plaintext) is equal to EncryptRequest.plaintext_crc32c, and if so, perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type. NOTE: This field is in Beta."]
        pub plaintext_crc32c: ::std::option::Option<::std::string::String>,
    }
    impl EncryptRequest {
        pub fn builder() -> EncryptRequestBuilder {
            EncryptRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for KeyManagementService.Encrypt."]
    pub struct EncryptResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ciphertext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The encrypted data."]
        pub ciphertext: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ciphertextCrc32c")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Integrity verification field. A CRC32C checksum of the returned EncryptResponse.ciphertext. An integrity check of EncryptResponse.ciphertext can be performed by computing the CRC32C checksum of EncryptResponse.ciphertext and comparing your results to this field. Discard the response in case of non-matching checksum values, and perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type. NOTE: This field is in Beta."]
        pub ciphertext_crc32c: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the CryptoKeyVersion used in encryption. Check this field to verify that the intended resource was used for encryption."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "protectionLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ProtectionLevel of the CryptoKeyVersion used in encryption."]
        pub protection_level: ::std::option::Option<EncryptResponseProtectionLevelEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verifiedAdditionalAuthenticatedDataCrc32c")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Integrity verification field. A flag indicating whether EncryptRequest.additional_authenticated_data_crc32c was received by KeyManagementService and used for the integrity verification of the AAD. A false value of this field indicates either that EncryptRequest.additional_authenticated_data_crc32c was left unset or that it was not delivered to KeyManagementService. If you've set EncryptRequest.additional_authenticated_data_crc32c but this field is still false, discard the response and perform a limited number of retries. NOTE: This field is in Beta."]
        pub verified_additional_authenticated_data_crc32c:
            ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verifiedPlaintextCrc32c")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Integrity verification field. A flag indicating whether EncryptRequest.plaintext_crc32c was received by KeyManagementService and used for the integrity verification of the plaintext. A false value of this field indicates either that EncryptRequest.plaintext_crc32c was left unset or that it was not delivered to KeyManagementService. If you've set EncryptRequest.plaintext_crc32c but this field is still false, discard the response and perform a limited number of retries. NOTE: This field is in Beta."]
        pub verified_plaintext_crc32c: ::std::option::Option<::std::primitive::bool>,
    }
    impl EncryptResponse {
        pub fn builder() -> EncryptResponseBuilder {
            EncryptResponseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The ProtectionLevel of the CryptoKeyVersion used in encryption."]
    pub enum EncryptResponseProtectionLevelEnum {
        #[serde(rename = "PROTECTION_LEVEL_UNSPECIFIED")]
        #[doc = "Not specified."]
        ProtectionLevelUnspecified,
        #[serde(rename = "SOFTWARE")]
        #[doc = "Crypto operations are performed in software."]
        Software,
        #[serde(rename = "HSM")]
        #[doc = "Crypto operations are performed in a Hardware Security Module."]
        Hsm,
        #[serde(rename = "EXTERNAL")]
        #[doc = "Crypto operations are performed by an external key manager."]
        External,
    }
    impl ::std::default::Default for EncryptResponseProtectionLevelEnum {
        fn default() -> Self {
            Self::ProtectionLevelUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: \"Summary size limit\" description: \"Determines if a summary is less than 100 chars\" expression: \"document.summary.size() < 100\" Example (Equality): title: \"Requestor is owner\" description: \"Determines if requestor is the document owner\" expression: \"document.owner == request.auth.claims.email\" Example (Logic): title: \"Public documents\" description: \"Determine whether the document should be publicly visible\" expression: \"document.type != 'private' && document.type != 'internal'\" Example (Data Manipulation): title: \"Notification string\" description: \"Create a notification string with a timestamp.\" expression: \"'New message received at ' + string(document.create_time)\" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information."]
    pub struct Expr {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expression")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Textual representation of an expression in Common Expression Language syntax."]
        pub expression: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl Expr {
        pub fn builder() -> ExprBuilder {
            ExprBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "ExternalProtectionLevelOptions stores a group of additional fields for configuring a CryptoKeyVersion that are specific to the EXTERNAL protection level."]
    pub struct ExternalProtectionLevelOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "externalKeyUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI for an external resource that this CryptoKeyVersion represents."]
        pub external_key_uri: ::std::option::Option<::std::string::String>,
    }
    impl ExternalProtectionLevelOptions {
        pub fn builder() -> ExternalProtectionLevelOptionsBuilder {
            ExternalProtectionLevelOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for KeyManagementService.ImportCryptoKeyVersion."]
    pub struct ImportCryptoKeyVersionRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "algorithm")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The algorithm of the key being imported. This does not need to match the version_template of the CryptoKey this version imports into."]
        pub algorithm: ::std::option::Option<ImportCryptoKeyVersionRequestAlgorithmEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importJob")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name of the ImportJob that was used to wrap this key material."]
        pub import_job: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rsaAesWrappedKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Wrapped key material produced with RSA_OAEP_3072_SHA1_AES_256 or RSA_OAEP_4096_SHA1_AES_256. This field contains the concatenation of two wrapped keys: 1. An ephemeral AES-256 wrapping key wrapped with the public_key using RSAES-OAEP with SHA-1, MGF1 with SHA-1, and an empty label. 2. The key to be imported, wrapped with the ephemeral AES-256 key using AES-KWP (RFC 5649). If importing symmetric key material, it is expected that the unwrapped key contains plain bytes. If importing asymmetric key material, it is expected that the unwrapped key is in PKCS#8-encoded DER format (the PrivateKeyInfo structure from RFC 5208). This format is the same as the format produced by PKCS#11 mechanism CKM_RSA_AES_KEY_WRAP."]
        pub rsa_aes_wrapped_key: ::std::option::Option<::std::string::String>,
    }
    impl ImportCryptoKeyVersionRequest {
        pub fn builder() -> ImportCryptoKeyVersionRequestBuilder {
            ImportCryptoKeyVersionRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The algorithm of the key being imported. This does not need to match the version_template of the CryptoKey this version imports into."]
    pub enum ImportCryptoKeyVersionRequestAlgorithmEnum {
        #[serde(rename = "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED")]
        #[doc = "Not specified."]
        CryptoKeyVersionAlgorithmUnspecified,
        #[serde(rename = "GOOGLE_SYMMETRIC_ENCRYPTION")]
        #[doc = "Creates symmetric encryption keys."]
        GoogleSymmetricEncryption,
        #[serde(rename = "RSA_SIGN_PSS_2048_SHA256")]
        #[doc = "RSASSA-PSS 2048 bit key with a SHA256 digest."]
        RsaSignPss2048Sha256,
        #[serde(rename = "RSA_SIGN_PSS_3072_SHA256")]
        #[doc = "RSASSA-PSS 3072 bit key with a SHA256 digest."]
        RsaSignPss3072Sha256,
        #[serde(rename = "RSA_SIGN_PSS_4096_SHA256")]
        #[doc = "RSASSA-PSS 4096 bit key with a SHA256 digest."]
        RsaSignPss4096Sha256,
        #[serde(rename = "RSA_SIGN_PSS_4096_SHA512")]
        #[doc = "RSASSA-PSS 4096 bit key with a SHA512 digest."]
        RsaSignPss4096Sha512,
        #[serde(rename = "RSA_SIGN_PKCS1_2048_SHA256")]
        #[doc = "RSASSA-PKCS1-v1_5 with a 2048 bit key and a SHA256 digest."]
        RsaSignPkcs12048Sha256,
        #[serde(rename = "RSA_SIGN_PKCS1_3072_SHA256")]
        #[doc = "RSASSA-PKCS1-v1_5 with a 3072 bit key and a SHA256 digest."]
        RsaSignPkcs13072Sha256,
        #[serde(rename = "RSA_SIGN_PKCS1_4096_SHA256")]
        #[doc = "RSASSA-PKCS1-v1_5 with a 4096 bit key and a SHA256 digest."]
        RsaSignPkcs14096Sha256,
        #[serde(rename = "RSA_SIGN_PKCS1_4096_SHA512")]
        #[doc = "RSASSA-PKCS1-v1_5 with a 4096 bit key and a SHA512 digest."]
        RsaSignPkcs14096Sha512,
        #[serde(rename = "RSA_DECRYPT_OAEP_2048_SHA256")]
        #[doc = "RSAES-OAEP 2048 bit key with a SHA256 digest."]
        RsaDecryptOaep2048Sha256,
        #[serde(rename = "RSA_DECRYPT_OAEP_3072_SHA256")]
        #[doc = "RSAES-OAEP 3072 bit key with a SHA256 digest."]
        RsaDecryptOaep3072Sha256,
        #[serde(rename = "RSA_DECRYPT_OAEP_4096_SHA256")]
        #[doc = "RSAES-OAEP 4096 bit key with a SHA256 digest."]
        RsaDecryptOaep4096Sha256,
        #[serde(rename = "RSA_DECRYPT_OAEP_4096_SHA512")]
        #[doc = "RSAES-OAEP 4096 bit key with a SHA512 digest."]
        RsaDecryptOaep4096Sha512,
        #[serde(rename = "EC_SIGN_P256_SHA256")]
        #[doc = "ECDSA on the NIST P-256 curve with a SHA256 digest."]
        EcSignP256Sha256,
        #[serde(rename = "EC_SIGN_P384_SHA384")]
        #[doc = "ECDSA on the NIST P-384 curve with a SHA384 digest."]
        EcSignP384Sha384,
        #[serde(rename = "EXTERNAL_SYMMETRIC_ENCRYPTION")]
        #[doc = "Algorithm representing symmetric encryption by an external key manager."]
        ExternalSymmetricEncryption,
    }
    impl ::std::default::Default for ImportCryptoKeyVersionRequestAlgorithmEnum {
        fn default() -> Self {
            Self::CryptoKeyVersionAlgorithmUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An ImportJob can be used to create CryptoKeys and CryptoKeyVersions using pre-existing key material, generated outside of Cloud KMS. When an ImportJob is created, Cloud KMS will generate a \"wrapping key\", which is a public/private key pair. You use the wrapping key to encrypt (also known as wrap) the pre-existing key material to protect it during the import process. The nature of the wrapping key depends on the choice of import_method. When the wrapping key generation is complete, the state will be set to ACTIVE and the public_key can be fetched. The fetched public key can then be used to wrap your pre-existing key material. Once the key material is wrapped, it can be imported into a new CryptoKeyVersion in an existing CryptoKey by calling ImportCryptoKeyVersion. Multiple CryptoKeyVersions can be imported with a single ImportJob. Cloud KMS uses the private key portion of the wrapping key to unwrap the key material. Only Cloud KMS has access to the private key. An ImportJob expires 3 days after it is created. Once expired, Cloud KMS will no longer be able to import or unwrap any key material that was wrapped with the ImportJob's public key. For more information, see [Importing a key](https://cloud.google.com/kms/docs/importing-a-key)."]
    pub struct ImportJob {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attestation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Statement that was generated and signed by the key creator (for example, an HSM) at key creation time. Use this statement to verify attributes of the key as stored on the HSM, independently of Google. Only present if the chosen ImportMethod is one with a protection level of HSM."]
        pub attestation: ::std::option::Option<::std::boxed::Box<KeyOperationAttestation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time at which this ImportJob was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expireEventTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time this ImportJob expired. Only present if state is EXPIRED."]
        pub expire_event_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expireTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time at which this ImportJob is scheduled for expiration and can no longer be used to import key material."]
        pub expire_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "generateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time this ImportJob's key material was generated."]
        pub generate_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importMethod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Immutable. The wrapping method to be used for incoming key material."]
        pub import_method: ::std::option::Option<ImportJobImportMethodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The resource name for this ImportJob in the format `projects/*/locations/*/keyRings/*/importJobs/*`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "protectionLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Immutable. The protection level of the ImportJob. This must match the protection_level of the version_template on the CryptoKey you attempt to import into."]
        pub protection_level: ::std::option::Option<ImportJobProtectionLevelEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publicKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The public key with which to wrap key material prior to import. Only returned if state is ACTIVE."]
        pub public_key: ::std::option::Option<::std::boxed::Box<WrappingPublicKey>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The current state of the ImportJob, indicating if it can be used."]
        pub state: ::std::option::Option<ImportJobStateEnum>,
    }
    impl ImportJob {
        pub fn builder() -> ImportJobBuilder {
            ImportJobBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Immutable. The wrapping method to be used for incoming key material."]
    pub enum ImportJobImportMethodEnum {
        #[serde(rename = "IMPORT_METHOD_UNSPECIFIED")]
        #[doc = "Not specified."]
        ImportMethodUnspecified,
        #[serde(rename = "RSA_OAEP_3072_SHA1_AES_256")]
        #[doc = "This ImportMethod represents the CKM_RSA_AES_KEY_WRAP key wrapping scheme defined in the PKCS #11 standard. In summary, this involves wrapping the raw key with an ephemeral AES key, and wrapping the ephemeral AES key with a 3072 bit RSA key. For more details, see [RSA AES key wrap mechanism](http://docs.oasis-open.org/pkcs11/pkcs11-curr/v2.40/cos01/pkcs11-curr-v2.40-cos01.html#_Toc408226908)."]
        RsaOaep3072Sha1Aes256,
        #[serde(rename = "RSA_OAEP_4096_SHA1_AES_256")]
        #[doc = "This ImportMethod represents the CKM_RSA_AES_KEY_WRAP key wrapping scheme defined in the PKCS #11 standard. In summary, this involves wrapping the raw key with an ephemeral AES key, and wrapping the ephemeral AES key with a 4096 bit RSA key. For more details, see [RSA AES key wrap mechanism](http://docs.oasis-open.org/pkcs11/pkcs11-curr/v2.40/cos01/pkcs11-curr-v2.40-cos01.html#_Toc408226908)."]
        RsaOaep4096Sha1Aes256,
    }
    impl ::std::default::Default for ImportJobImportMethodEnum {
        fn default() -> Self {
            Self::ImportMethodUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Immutable. The protection level of the ImportJob. This must match the protection_level of the version_template on the CryptoKey you attempt to import into."]
    pub enum ImportJobProtectionLevelEnum {
        #[serde(rename = "PROTECTION_LEVEL_UNSPECIFIED")]
        #[doc = "Not specified."]
        ProtectionLevelUnspecified,
        #[serde(rename = "SOFTWARE")]
        #[doc = "Crypto operations are performed in software."]
        Software,
        #[serde(rename = "HSM")]
        #[doc = "Crypto operations are performed in a Hardware Security Module."]
        Hsm,
        #[serde(rename = "EXTERNAL")]
        #[doc = "Crypto operations are performed by an external key manager."]
        External,
    }
    impl ::std::default::Default for ImportJobProtectionLevelEnum {
        fn default() -> Self {
            Self::ProtectionLevelUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The current state of the ImportJob, indicating if it can be used."]
    pub enum ImportJobStateEnum {
        #[serde(rename = "IMPORT_JOB_STATE_UNSPECIFIED")]
        #[doc = "Not specified."]
        ImportJobStateUnspecified,
        #[serde(rename = "PENDING_GENERATION")]
        #[doc = "The wrapping key for this job is still being generated. It may not be used. Cloud KMS will automatically mark this job as ACTIVE as soon as the wrapping key is generated."]
        PendingGeneration,
        #[serde(rename = "ACTIVE")]
        #[doc = "This job may be used in CreateCryptoKey and CreateCryptoKeyVersion requests."]
        Active,
        #[serde(rename = "EXPIRED")]
        #[doc = "This job can no longer be used and may not leave this state once entered."]
        Expired,
    }
    impl ::std::default::Default for ImportJobStateEnum {
        fn default() -> Self {
            Self::ImportJobStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains an HSM-generated attestation about a key operation. For more information, see [Verifying attestations] (https://cloud.google.com/kms/docs/attest-key)."]
    pub struct KeyOperationAttestation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "certChains")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The certificate chains needed to validate the attestation"]
        pub cert_chains: ::std::option::Option<::std::boxed::Box<CertificateChains>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The attestation data provided by the HSM when the key operation was performed."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "format")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The format of the attestation data."]
        pub format: ::std::option::Option<KeyOperationAttestationFormatEnum>,
    }
    impl KeyOperationAttestation {
        pub fn builder() -> KeyOperationAttestationBuilder {
            KeyOperationAttestationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The format of the attestation data."]
    pub enum KeyOperationAttestationFormatEnum {
        #[serde(rename = "ATTESTATION_FORMAT_UNSPECIFIED")]
        #[doc = "Not specified."]
        AttestationFormatUnspecified,
        #[serde(rename = "CAVIUM_V1_COMPRESSED")]
        #[doc = "Cavium HSM attestation compressed with gzip. Note that this format is defined by Cavium and subject to change at any time."]
        CaviumV1Compressed,
        #[serde(rename = "CAVIUM_V2_COMPRESSED")]
        #[doc = "Cavium HSM attestation V2 compressed with gzip. This is a new format introduced in Cavium's version 3.2-08."]
        CaviumV2Compressed,
    }
    impl ::std::default::Default for KeyOperationAttestationFormatEnum {
        fn default() -> Self {
            Self::AttestationFormatUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A KeyRing is a toplevel logical grouping of CryptoKeys."]
    pub struct KeyRing {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time at which this KeyRing was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The resource name for the KeyRing in the format `projects/*/locations/*/keyRings/*`."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl KeyRing {
        pub fn builder() -> KeyRingBuilder {
            KeyRingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for KeyManagementService.ListCryptoKeyVersions."]
    pub struct ListCryptoKeyVersionsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cryptoKeyVersions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of CryptoKeyVersions."]
        pub crypto_key_versions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CryptoKeyVersion>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve next page of results. Pass this value in ListCryptoKeyVersionsRequest.page_token to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of CryptoKeyVersions that matched the query."]
        pub total_size: ::std::option::Option<::std::primitive::i64>,
    }
    impl ListCryptoKeyVersionsResponse {
        pub fn builder() -> ListCryptoKeyVersionsResponseBuilder {
            ListCryptoKeyVersionsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for KeyManagementService.ListCryptoKeys."]
    pub struct ListCryptoKeysResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cryptoKeys")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of CryptoKeys."]
        pub crypto_keys: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CryptoKey>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve next page of results. Pass this value in ListCryptoKeysRequest.page_token to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of CryptoKeys that matched the query."]
        pub total_size: ::std::option::Option<::std::primitive::i64>,
    }
    impl ListCryptoKeysResponse {
        pub fn builder() -> ListCryptoKeysResponseBuilder {
            ListCryptoKeysResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for KeyManagementService.ListImportJobs."]
    pub struct ListImportJobsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importJobs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of ImportJobs."]
        pub import_jobs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ImportJob>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve next page of results. Pass this value in ListImportJobsRequest.page_token to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of ImportJobs that matched the query."]
        pub total_size: ::std::option::Option<::std::primitive::i64>,
    }
    impl ListImportJobsResponse {
        pub fn builder() -> ListImportJobsResponseBuilder {
            ListImportJobsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for KeyManagementService.ListKeyRings."]
    pub struct ListKeyRingsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyRings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of KeyRings."]
        pub key_rings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<KeyRing>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve next page of results. Pass this value in ListKeyRingsRequest.page_token to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of KeyRings that matched the query."]
        pub total_size: ::std::option::Option<::std::primitive::i64>,
    }
    impl ListKeyRingsResponse {
        pub fn builder() -> ListKeyRingsResponseBuilder {
            ListKeyRingsResponseBuilder::default()
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
        #[doc = "Cross-service attributes for the location. For example {\"cloud.googleapis.com/region\": \"us-east1\"}"]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The canonical id for this location. For example: `\"us-east1\"`."]
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
        #[doc = "Resource name for the location, which may vary between implementations. For example: `\"projects/example-project/locations/us-east1\"`"]
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
    #[doc = "Cloud KMS metadata for the given google.cloud.location.Location."]
    pub struct LocationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ekmAvailable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether CryptoKeys with protection_level EXTERNAL can be created in this location."]
        pub ekm_available: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hsmAvailable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether CryptoKeys with protection_level HSM can be created in this location."]
        pub hsm_available: ::std::option::Option<::std::primitive::bool>,
    }
    impl LocationMetadata {
        pub fn builder() -> LocationMetadataBuilder {
            LocationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members` to a single `role`. Members can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { \"bindings\": [ { \"role\": \"roles/resourcemanager.organizationAdmin\", \"members\": [ \"user:mike@example.com\", \"group:admins@example.com\", \"domain:google.com\", \"serviceAccount:my-project-id@appspot.gserviceaccount.com\" ] }, { \"role\": \"roles/resourcemanager.organizationViewer\", \"members\": [ \"user:eve@example.com\" ], \"condition\": { \"title\": \"expirable access\", \"description\": \"Does not grant access after Sep 2020\", \"expression\": \"request.time < timestamp('2020-10-01T00:00:00.000Z')\", } } ], \"etag\": \"BwWWja0YfJA=\", \"version\": 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') - etag: BwWWja0YfJA= - version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/)."]
    pub struct Policy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "auditConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies cloud audit logging configuration for this policy."]
        pub audit_configs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AuditConfig>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bindings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Associates a list of `members` to a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one member."]
        pub bindings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Binding>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "`etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        pub version: ::std::option::Option<::std::primitive::i64>,
    }
    impl Policy {
        pub fn builder() -> PolicyBuilder {
            PolicyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The public key for a given CryptoKeyVersion. Obtained via GetPublicKey."]
    pub struct PublicKey {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "algorithm")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Algorithm associated with this key."]
        pub algorithm: ::std::option::Option<PublicKeyAlgorithmEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the CryptoKeyVersion public key. Provided here for verification. NOTE: This field is in Beta."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pem")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The public key, encoded in PEM format. For more information, see the [RFC 7468](https://tools.ietf.org/html/rfc7468) sections for [General Considerations](https://tools.ietf.org/html/rfc7468#section-2) and [Textual Encoding of Subject Public Key Info] (https://tools.ietf.org/html/rfc7468#section-13)."]
        pub pem: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pemCrc32c")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Integrity verification field. A CRC32C checksum of the returned PublicKey.pem. An integrity check of PublicKey.pem can be performed by computing the CRC32C checksum of PublicKey.pem and comparing your results to this field. Discard the response in case of non-matching checksum values, and perform a limited number of retries. A persistent mismatch may indicate an issue in your computation of the CRC32C checksum. Note: This field is defined as int64 for reasons of compatibility across different languages. However, it is a non-negative integer, which will never exceed 2^32-1, and can be safely downconverted to uint32 in languages that support this type. NOTE: This field is in Beta."]
        pub pem_crc32c: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "protectionLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ProtectionLevel of the CryptoKeyVersion public key."]
        pub protection_level: ::std::option::Option<PublicKeyProtectionLevelEnum>,
    }
    impl PublicKey {
        pub fn builder() -> PublicKeyBuilder {
            PublicKeyBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The Algorithm associated with this key."]
    pub enum PublicKeyAlgorithmEnum {
        #[serde(rename = "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED")]
        #[doc = "Not specified."]
        CryptoKeyVersionAlgorithmUnspecified,
        #[serde(rename = "GOOGLE_SYMMETRIC_ENCRYPTION")]
        #[doc = "Creates symmetric encryption keys."]
        GoogleSymmetricEncryption,
        #[serde(rename = "RSA_SIGN_PSS_2048_SHA256")]
        #[doc = "RSASSA-PSS 2048 bit key with a SHA256 digest."]
        RsaSignPss2048Sha256,
        #[serde(rename = "RSA_SIGN_PSS_3072_SHA256")]
        #[doc = "RSASSA-PSS 3072 bit key with a SHA256 digest."]
        RsaSignPss3072Sha256,
        #[serde(rename = "RSA_SIGN_PSS_4096_SHA256")]
        #[doc = "RSASSA-PSS 4096 bit key with a SHA256 digest."]
        RsaSignPss4096Sha256,
        #[serde(rename = "RSA_SIGN_PSS_4096_SHA512")]
        #[doc = "RSASSA-PSS 4096 bit key with a SHA512 digest."]
        RsaSignPss4096Sha512,
        #[serde(rename = "RSA_SIGN_PKCS1_2048_SHA256")]
        #[doc = "RSASSA-PKCS1-v1_5 with a 2048 bit key and a SHA256 digest."]
        RsaSignPkcs12048Sha256,
        #[serde(rename = "RSA_SIGN_PKCS1_3072_SHA256")]
        #[doc = "RSASSA-PKCS1-v1_5 with a 3072 bit key and a SHA256 digest."]
        RsaSignPkcs13072Sha256,
        #[serde(rename = "RSA_SIGN_PKCS1_4096_SHA256")]
        #[doc = "RSASSA-PKCS1-v1_5 with a 4096 bit key and a SHA256 digest."]
        RsaSignPkcs14096Sha256,
        #[serde(rename = "RSA_SIGN_PKCS1_4096_SHA512")]
        #[doc = "RSASSA-PKCS1-v1_5 with a 4096 bit key and a SHA512 digest."]
        RsaSignPkcs14096Sha512,
        #[serde(rename = "RSA_DECRYPT_OAEP_2048_SHA256")]
        #[doc = "RSAES-OAEP 2048 bit key with a SHA256 digest."]
        RsaDecryptOaep2048Sha256,
        #[serde(rename = "RSA_DECRYPT_OAEP_3072_SHA256")]
        #[doc = "RSAES-OAEP 3072 bit key with a SHA256 digest."]
        RsaDecryptOaep3072Sha256,
        #[serde(rename = "RSA_DECRYPT_OAEP_4096_SHA256")]
        #[doc = "RSAES-OAEP 4096 bit key with a SHA256 digest."]
        RsaDecryptOaep4096Sha256,
        #[serde(rename = "RSA_DECRYPT_OAEP_4096_SHA512")]
        #[doc = "RSAES-OAEP 4096 bit key with a SHA512 digest."]
        RsaDecryptOaep4096Sha512,
        #[serde(rename = "EC_SIGN_P256_SHA256")]
        #[doc = "ECDSA on the NIST P-256 curve with a SHA256 digest."]
        EcSignP256Sha256,
        #[serde(rename = "EC_SIGN_P384_SHA384")]
        #[doc = "ECDSA on the NIST P-384 curve with a SHA384 digest."]
        EcSignP384Sha384,
        #[serde(rename = "EXTERNAL_SYMMETRIC_ENCRYPTION")]
        #[doc = "Algorithm representing symmetric encryption by an external key manager."]
        ExternalSymmetricEncryption,
    }
    impl ::std::default::Default for PublicKeyAlgorithmEnum {
        fn default() -> Self {
            Self::CryptoKeyVersionAlgorithmUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The ProtectionLevel of the CryptoKeyVersion public key."]
    pub enum PublicKeyProtectionLevelEnum {
        #[serde(rename = "PROTECTION_LEVEL_UNSPECIFIED")]
        #[doc = "Not specified."]
        ProtectionLevelUnspecified,
        #[serde(rename = "SOFTWARE")]
        #[doc = "Crypto operations are performed in software."]
        Software,
        #[serde(rename = "HSM")]
        #[doc = "Crypto operations are performed in a Hardware Security Module."]
        Hsm,
        #[serde(rename = "EXTERNAL")]
        #[doc = "Crypto operations are performed by an external key manager."]
        External,
    }
    impl ::std::default::Default for PublicKeyProtectionLevelEnum {
        fn default() -> Self {
            Self::ProtectionLevelUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for KeyManagementService.RestoreCryptoKeyVersion."]
    pub struct RestoreCryptoKeyVersionRequest {}
    impl RestoreCryptoKeyVersionRequest {
        pub fn builder() -> RestoreCryptoKeyVersionRequestBuilder {
            RestoreCryptoKeyVersionRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for `SetIamPolicy` method."]
    pub struct SetIamPolicyRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Cloud Platform services (such as Projects) might reject them."]
        pub policy: ::std::option::Option<::std::boxed::Box<Policy>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateMask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: \"bindings, etag\"`"]
        pub update_mask: ::std::option::Option<::std::string::String>,
    }
    impl SetIamPolicyRequest {
        pub fn builder() -> SetIamPolicyRequestBuilder {
            SetIamPolicyRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for `TestIamPermissions` method."]
    pub struct TestIamPermissionsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of permissions to check for the `resource`. Permissions with wildcards (such as '*' or 'storage.*') are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions)."]
        pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl TestIamPermissionsRequest {
        pub fn builder() -> TestIamPermissionsRequestBuilder {
            TestIamPermissionsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for `TestIamPermissions` method."]
    pub struct TestIamPermissionsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A subset of `TestPermissionsRequest.permissions` that the caller is allowed."]
        pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl TestIamPermissionsResponse {
        pub fn builder() -> TestIamPermissionsResponseBuilder {
            TestIamPermissionsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for KeyManagementService.UpdateCryptoKeyPrimaryVersion."]
    pub struct UpdateCryptoKeyPrimaryVersionRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cryptoKeyVersionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The id of the child CryptoKeyVersion to use as primary."]
        pub crypto_key_version_id: ::std::option::Option<::std::string::String>,
    }
    impl UpdateCryptoKeyPrimaryVersionRequest {
        pub fn builder() -> UpdateCryptoKeyPrimaryVersionRequestBuilder {
            UpdateCryptoKeyPrimaryVersionRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The public key component of the wrapping key. For details of the type of key this public key corresponds to, see the ImportMethod."]
    pub struct WrappingPublicKey {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pem")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The public key, encoded in PEM format. For more information, see the [RFC 7468](https://tools.ietf.org/html/rfc7468) sections for [General Considerations](https://tools.ietf.org/html/rfc7468#section-2) and [Textual Encoding of Subject Public Key Info] (https://tools.ietf.org/html/rfc7468#section-13)."]
        pub pem: ::std::option::Option<::std::string::String>,
    }
    impl WrappingPublicKey {
        pub fn builder() -> WrappingPublicKeyBuilder {
            WrappingPublicKeyBuilder::default()
        }
    }
}
