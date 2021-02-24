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
        serde_json::from_str(&"json").unwrap()
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
            pub mod notes {
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
                            #[serde(rename = "name")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The name of the project. Should be of the form \"providers/{provider_id}\". @Deprecated"]
                            pub name: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "noteId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The ID to use for this note."]
                            pub note_id: ::std::option::Option<::std::string::String>,
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
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The filter expression."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "name")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The name field will contain the project Id for example: \"providers/{provider_id} @Deprecated"]
                            pub name: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Number of notes to return in the list."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Token to provide to skip to a particular spot in the list."]
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
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The fields to update."]
                            pub update_mask: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
                pub mod resources {
                    pub mod occurrences {
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
                                    #[doc = "The filter expression."]
                                    pub filter: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Number of notes to return in the list."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Token to provide to skip to a particular spot in the list."]
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
            pub mod occurrences {
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
                            #[serde(rename = "name")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The name of the project. Should be of the form \"projects/{project_id}\". @Deprecated"]
                            pub name: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                    pub mod get_vulnerability_summary {
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
                            #[doc = "The filter expression."]
                            pub filter: ::std::option::Option<::std::string::String>,
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
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The filter expression."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "kind")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The kind of occurrences to filter on."]
                            pub kind: ::std::option::Option<QueryParametersKindEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "name")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The name field contains the project Id. For example: \"projects/{project_id} @Deprecated"]
                            pub name: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Number of occurrences to return in the list."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Token to provide to skip to a particular spot in the list."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "The kind of occurrences to filter on."]
                        pub enum QueryParametersKindEnum {
                            #[serde(rename = "KIND_UNSPECIFIED")]
                            #[doc = "Unknown"]
                            KindUnspecified,
                            #[serde(rename = "PACKAGE_VULNERABILITY")]
                            #[doc = "The note and occurrence represent a package vulnerability."]
                            PackageVulnerability,
                            #[serde(rename = "BUILD_DETAILS")]
                            #[doc = "The note and occurrence assert build provenance."]
                            BuildDetails,
                            #[serde(rename = "IMAGE_BASIS")]
                            #[doc = "This represents an image basis relationship."]
                            ImageBasis,
                            #[serde(rename = "PACKAGE_MANAGER")]
                            #[doc = "This represents a package installed via a package manager."]
                            PackageManager,
                            #[serde(rename = "DEPLOYABLE")]
                            #[doc = "The note and occurrence track deployment events."]
                            Deployable,
                            #[serde(rename = "DISCOVERY")]
                            #[doc = "The note and occurrence track the initial discovery status of a resource."]
                            Discovery,
                            #[serde(rename = "ATTESTATION_AUTHORITY")]
                            #[doc = "This represents a logical \"role\" that can attest to artifacts."]
                            AttestationAuthority,
                            #[serde(rename = "UPGRADE")]
                            #[doc = "This represents an available software upgrade."]
                            Upgrade,
                        }
                        impl ::std::default::Default for QueryParametersKindEnum {
                            fn default() -> Self {
                                Self::KindUnspecified
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
                            #[doc = "The fields to update."]
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
            pub mod scan_configs {
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
                            #[doc = "The filter expression."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The number of items to return."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The page token to use for the next request."]
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
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The fields to update."]
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
        }
    }
    pub mod providers {
        pub mod resources {
            pub mod notes {
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
                            #[serde(rename = "noteId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The ID to use for this note."]
                            pub note_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "parent")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "This field contains the project Id for example: \"projects/{project_id}"]
                            pub parent: ::std::option::Option<::std::string::String>,
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
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The filter expression."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Number of notes to return in the list."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Token to provide to skip to a particular spot in the list."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "parent")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "This field contains the project Id for example: \"projects/{PROJECT_ID}\"."]
                            pub parent: ::std::option::Option<::std::string::String>,
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
                            #[serde(rename = "updateMask")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The fields to update."]
                            pub update_mask: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
                pub mod resources {
                    pub mod occurrences {
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
                                    #[doc = "The filter expression."]
                                    pub filter: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Number of notes to return in the list."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Token to provide to skip to a particular spot in the list."]
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
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Artifact describes a build product."]
    pub struct Artifact {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "checksum")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hash or checksum value of a binary, or Docker Registry 2.0 digest of a container."]
        pub checksum: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Artifact ID, if any; for container images, this will be a URL by digest like gcr.io/projectID/imagename@sha256:123456"]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the artifact. This may be the path to a binary or jar file, or in the case of a container build, the name used to push the container image to Google Container Registry, as presented to `docker push`. This field is deprecated in favor of the plural `names` field; it continues to exist here to allow existing BuildProvenance serialized to json in google.devtools.containeranalysis.v1alpha1.BuildDetails.provenance_bytes to deserialize back into proto."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "names")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Related artifact names. This may be the path to a binary or jar file, or in the case of a container build, the name used to push the container image to Google Container Registry, as presented to `docker push`. Note that a single Artifact ID can have multiple names, for example if two tags are applied to one image."]
        pub names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl Artifact {
        pub fn builder() -> ArtifactBuilder {
            ArtifactBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Occurrence that represents a single \"attestation\". The authenticity of an Attestation can be verified using the attached signature. If the verifier trusts the public key of the signer, then verifying the signature is sufficient to establish trust. In this circumstance, the AttestationAuthority to which this Attestation is attached is primarily useful for look-up (how to find this Attestation if you already know the Authority and artifact to be verified) and intent (which authority was this attestation intended to sign for)."]
    pub struct Attestation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pgpSignedAttestation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub pgp_signed_attestation: ::std::option::Option<::std::boxed::Box<PgpSignedAttestation>>,
    }
    impl Attestation {
        pub fn builder() -> AttestationBuilder {
            AttestationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Note kind that represents a logical attestation \"role\" or \"authority\". For example, an organization might have one `AttestationAuthority` for \"QA\" and one for \"build\". This Note is intended to act strictly as a grouping mechanism for the attached Occurrences (Attestations). This grouping mechanism also provides a security boundary, since IAM ACLs gate the ability for a principle to attach an Occurrence to a given Note. It also provides a single point of lookup to find all attached Attestation Occurrences, even if they don't all live in the same project."]
    pub struct AttestationAuthority {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub hint: ::std::option::Option<::std::boxed::Box<AttestationAuthorityHint>>,
    }
    impl AttestationAuthority {
        pub fn builder() -> AttestationAuthorityBuilder {
            AttestationAuthorityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This submessage provides human-readable hints about the purpose of the AttestationAuthority. Because the name of a Note acts as its resource reference, it is important to disambiguate the canonical name of the Note (which might be a UUID for security purposes) from \"readable\" names more suitable for debug output. Note that these hints should NOT be used to look up AttestationAuthorities in security sensitive contexts, such as when looking up Attestations to verify."]
    pub struct AttestationAuthorityHint {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "humanReadableName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The human readable name of this Attestation Authority, for example \"qa\"."]
        pub human_readable_name: ::std::option::Option<::std::string::String>,
    }
    impl AttestationAuthorityHint {
        pub fn builder() -> AttestationAuthorityHintBuilder {
            AttestationAuthorityHintBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Basis describes the base image portion (Note) of the DockerImage relationship. Linked occurrences are derived from this or an equivalent image via: FROM Or an equivalent reference, e.g. a tag of the resource_url."]
    pub struct Basis {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fingerprint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fingerprint of the base image."]
        pub fingerprint: ::std::option::Option<::std::boxed::Box<Fingerprint>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource_url for the resource representing the basis of associated occurrence images."]
        pub resource_url: ::std::option::Option<::std::string::String>,
    }
    impl Basis {
        pub fn builder() -> BasisBuilder {
            BasisBuilder::default()
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
    #[doc = "Message encapsulating build provenance details."]
    pub struct BuildDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provenance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The actual provenance"]
        pub provenance: ::std::option::Option<::std::boxed::Box<BuildProvenance>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "provenanceBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serialized JSON representation of the provenance, used in generating the `BuildSignature` in the corresponding Result. After verifying the signature, `provenance_bytes` can be unmarshalled and compared to the provenance to confirm that it is unchanged. A base64-encoded string representation of the provenance bytes is used for the signature in order to interoperate with openssl which expects this format for signature verification. The serialized form is captured both to avoid ambiguity in how the provenance is marshalled to json as well to prevent incompatibilities with future changes."]
        pub provenance_bytes: ::std::option::Option<::std::string::String>,
    }
    impl BuildDetails {
        pub fn builder() -> BuildDetailsBuilder {
            BuildDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Provenance of a build. Contains all information needed to verify the full details about the build from source to completion."]
    pub struct BuildProvenance {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buildOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Special options applied to this build. This is a catch-all field where build providers can enter any desired additional details."]
        pub build_options:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "builderVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Version string of the builder at the time this build was executed."]
        pub builder_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "builtArtifacts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output of the build."]
        pub built_artifacts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Artifact>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commands")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Commands requested by the build."]
        pub commands: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Command>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time at which the build was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creator")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "E-mail address of the user who initiated this build. Note that this was the user's e-mail address at the time the build was initiated; this address may not represent the same end-user for all time."]
        pub creator: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "finishTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time at which execution of the build was finished."]
        pub finish_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier of the build."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logsBucket")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage bucket where logs were written."]
        pub logs_bucket: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the project."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceProvenance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of the Source input to the build."]
        pub source_provenance: ::std::option::Option<::std::boxed::Box<Source>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time at which execution of the build was started."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "triggerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Trigger identifier if the build was triggered automatically; empty if not."]
        pub trigger_id: ::std::option::Option<::std::string::String>,
    }
    impl BuildProvenance {
        pub fn builder() -> BuildProvenanceBuilder {
            BuildProvenanceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message encapsulating the signature of the verified build."]
    pub struct BuildSignature {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An Id for the key used to sign. This could be either an Id for the key stored in `public_key` (such as the Id or fingerprint for a PGP key, or the CN for a cert), or a reference to an external key (such as a reference to a key in Cloud Key Management Service)."]
        pub key_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the key, either stored in `public_key` or referenced in `key_id`"]
        pub key_type: ::std::option::Option<BuildSignatureKeyTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publicKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Public key of the builder which can be used to verify that the related findings are valid and unchanged. If `key_type` is empty, this defaults to PEM encoded public keys. This field may be empty if `key_id` references an external key. For Cloud Build based signatures, this is a PEM encoded public key. To verify the Cloud Build signature, place the contents of this field into a file (public.pem). The signature field is base64-decoded into its binary representation in signature.bin, and the provenance bytes from `BuildDetails` are base64-decoded into a binary representation in signed.bin. OpenSSL can then verify the signature: `openssl sha256 -verify public.pem -signature signature.bin signed.bin`"]
        pub public_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "signature")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Signature of the related `BuildProvenance`, encoded in a base64 string."]
        pub signature: ::std::option::Option<::std::string::String>,
    }
    impl BuildSignature {
        pub fn builder() -> BuildSignatureBuilder {
            BuildSignatureBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the key, either stored in `public_key` or referenced in `key_id`"]
    pub enum BuildSignatureKeyTypeEnum {
        #[serde(rename = "KEY_TYPE_UNSPECIFIED")]
        #[doc = "`KeyType` is not set."]
        KeyTypeUnspecified,
        #[serde(rename = "PGP_ASCII_ARMORED")]
        #[doc = "`PGP ASCII Armored` public key."]
        PgpAsciiArmored,
        #[serde(rename = "PKIX_PEM")]
        #[doc = "`PKIX PEM` public key."]
        PkixPem,
    }
    impl ::std::default::Default for BuildSignatureKeyTypeEnum {
        fn default() -> Self {
            Self::KeyTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Note holding the version of the provider's builder and the signature of the provenance message in linked BuildDetails."]
    pub struct BuildType {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "builderVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Version of the builder which produced this Note."]
        pub builder_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "signature")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Signature of the build in Occurrences pointing to the Note containing this `BuilderDetails`."]
        pub signature: ::std::option::Option<::std::boxed::Box<BuildSignature>>,
    }
    impl BuildType {
        pub fn builder() -> BuildTypeBuilder {
            BuildTypeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Command describes a step performed as part of the build pipeline."]
    pub struct Command {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "args")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Command-line arguments used when executing this Command."]
        pub args: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dir")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Working directory (relative to project source root) used when running this Command."]
        pub dir: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "env")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Environment variables set before running this Command."]
        pub env: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional unique identifier for this Command, used in wait_for to reference this Command as a dependency."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the command, as presented on the command line, or if the command is packaged as a Docker container, as presented to `docker pull`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "waitFor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID(s) of the Command(s) that this Command depends on."]
        pub wait_for: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl Command {
        pub fn builder() -> CommandBuilder {
            CommandBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for creating an operation"]
    pub struct CreateOperationRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The operation to create."]
        pub operation: ::std::option::Option<::std::boxed::Box<Operation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID to use for this operation."]
        pub operation_id: ::std::option::Option<::std::string::String>,
    }
    impl CreateOperationRequest {
        pub fn builder() -> CreateOperationRequestBuilder {
            CreateOperationRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An artifact that can be deployed in some runtime."]
    pub struct Deployable {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource URI for the artifact being deployed."]
        pub resource_uri: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl Deployable {
        pub fn builder() -> DeployableBuilder {
            DeployableBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The period during which some deployable was active in a runtime."]
    pub struct Deployment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "address")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Address of the runtime element hosting this deployment."]
        pub address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "config")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration used to create this deployment."]
        pub config: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deployTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Beginning of the lifetime of this deployment."]
        pub deploy_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "platform")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Platform hosting this deployment."]
        pub platform: ::std::option::Option<DeploymentPlatformEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Resource URI for the artifact being deployed taken from the deployable field with the same name."]
        pub resource_uri: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "undeployTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "End of the lifetime of this deployment."]
        pub undeploy_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identity of the user that triggered this deployment."]
        pub user_email: ::std::option::Option<::std::string::String>,
    }
    impl Deployment {
        pub fn builder() -> DeploymentBuilder {
            DeploymentBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Platform hosting this deployment."]
    pub enum DeploymentPlatformEnum {
        #[serde(rename = "PLATFORM_UNSPECIFIED")]
        #[doc = "Unknown"]
        PlatformUnspecified,
        #[serde(rename = "GKE")]
        #[doc = "Google Container Engine"]
        Gke,
        #[serde(rename = "FLEX")]
        #[doc = "Google App Engine: Flexible Environment"]
        Flex,
        #[serde(rename = "CUSTOM")]
        #[doc = "Custom user-defined platform"]
        Custom,
    }
    impl ::std::default::Default for DeploymentPlatformEnum {
        fn default() -> Self {
            Self::PlatformUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Derived describes the derived image portion (Occurrence) of the DockerImage relationship. This image would be produced from a Dockerfile with FROM ."]
    pub struct Derived {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "baseResourceUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. This contains the base image URL for the derived image occurrence."]
        pub base_resource_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "distance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The number of layers by which this image differs from the associated image basis."]
        pub distance: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fingerprint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fingerprint of the derived image."]
        pub fingerprint: ::std::option::Option<::std::boxed::Box<Fingerprint>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layerInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This contains layer-specific metadata, if populated it has length \"distance\" and is ordered with [distance] being the layer immediately following the base image and [1] being the final layer."]
        pub layer_info: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Layer>>>,
    }
    impl Derived {
        pub fn builder() -> DerivedBuilder {
            DerivedBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Identifies all occurrences of this vulnerability in the package for a specific distro/location For example: glibc in cpe:/o:debian:debian_linux:8 for versions 2.1 - 2.2"]
    pub struct Detail {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpeUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cpe_uri in [cpe format] (https://cpe.mitre.org/specification/) in which the vulnerability manifests. Examples include distro or storage location for vulnerable jar. This field can be used as a filter in list requests."]
        pub cpe_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A vendor-specific description of this note."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fixedLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fix for this specific package version."]
        pub fixed_location: ::std::option::Option<::std::boxed::Box<VulnerabilityLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isObsolete")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this Detail is obsolete. Occurrences are expected not to point to obsolete details."]
        pub is_obsolete: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxAffectedVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The max version of the package in which the vulnerability exists."]
        pub max_affected_version: ::std::option::Option<::std::boxed::Box<Version>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minAffectedVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The min version of the package in which the vulnerability exists."]
        pub min_affected_version: ::std::option::Option<::std::boxed::Box<Version>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "package")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the package where the vulnerability was found. This field can be used as a filter in list requests."]
        pub package: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packageType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of package; whether native or non native(ruby gems, node.js packages etc)"]
        pub package_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "severityName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The severity (eg: distro assigned severity) for this vulnerability."]
        pub severity_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source from which the information in this Detail was obtained."]
        pub source: ::std::option::Option<::std::string::String>,
    }
    impl Detail {
        pub fn builder() -> DetailBuilder {
            DetailBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Provides information about the scan status of a discovered resource."]
    pub struct Discovered {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "analysisStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of discovery for the resource."]
        pub analysis_status: ::std::option::Option<DiscoveredAnalysisStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "analysisStatusError")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When an error is encountered this will contain a LocalizedMessage under details to show to the user. The LocalizedMessage output only and populated by the API."]
        pub analysis_status_error: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "continuousAnalysis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the resource is continuously analyzed."]
        pub continuous_analysis: ::std::option::Option<DiscoveredContinuousAnalysisEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpe")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The CPE of the resource being scanned."]
        pub cpe: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. An operation that indicates the status of the current scan. This field is deprecated, do not use."]
        pub operation: ::std::option::Option<::std::boxed::Box<Operation>>,
    }
    impl Discovered {
        pub fn builder() -> DiscoveredBuilder {
            DiscoveredBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The status of discovery for the resource."]
    pub enum DiscoveredAnalysisStatusEnum {
        #[serde(rename = "ANALYSIS_STATUS_UNSPECIFIED")]
        #[doc = "Unknown"]
        AnalysisStatusUnspecified,
        #[serde(rename = "PENDING")]
        #[doc = "Resource is known but no action has been taken yet."]
        Pending,
        #[serde(rename = "SCANNING")]
        #[doc = "Resource is being analyzed."]
        Scanning,
        #[serde(rename = "FINISHED_SUCCESS")]
        #[doc = "Analysis has finished successfully."]
        FinishedSuccess,
        #[serde(rename = "FINISHED_FAILED")]
        #[doc = "Analysis has finished unsuccessfully, the analysis itself is in a bad state."]
        FinishedFailed,
        #[serde(rename = "FINISHED_UNSUPPORTED")]
        #[doc = "The resource is known not to be supported."]
        FinishedUnsupported,
    }
    impl ::std::default::Default for DiscoveredAnalysisStatusEnum {
        fn default() -> Self {
            Self::AnalysisStatusUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Whether the resource is continuously analyzed."]
    pub enum DiscoveredContinuousAnalysisEnum {
        #[serde(rename = "CONTINUOUS_ANALYSIS_UNSPECIFIED")]
        #[doc = "Unknown"]
        ContinuousAnalysisUnspecified,
        #[serde(rename = "ACTIVE")]
        #[doc = "The resource is continuously analyzed."]
        Active,
        #[serde(rename = "INACTIVE")]
        #[doc = "The resource is ignored for continuous analysis."]
        Inactive,
    }
    impl ::std::default::Default for DiscoveredContinuousAnalysisEnum {
        fn default() -> Self {
            Self::ContinuousAnalysisUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A note that indicates a type of analysis a provider would perform. This note exists in a provider's project. A `Discovery` occurrence is created in a consumer's project at the start of analysis. The occurrence's operation will indicate the status of the analysis. Absence of an occurrence linked to this note for a resource indicates that analysis hasn't started."]
    pub struct Discovery {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "analysisKind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The kind of analysis that is handled by this discovery."]
        pub analysis_kind: ::std::option::Option<DiscoveryAnalysisKindEnum>,
    }
    impl Discovery {
        pub fn builder() -> DiscoveryBuilder {
            DiscoveryBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The kind of analysis that is handled by this discovery."]
    pub enum DiscoveryAnalysisKindEnum {
        #[serde(rename = "KIND_UNSPECIFIED")]
        #[doc = "Unknown"]
        KindUnspecified,
        #[serde(rename = "PACKAGE_VULNERABILITY")]
        #[doc = "The note and occurrence represent a package vulnerability."]
        PackageVulnerability,
        #[serde(rename = "BUILD_DETAILS")]
        #[doc = "The note and occurrence assert build provenance."]
        BuildDetails,
        #[serde(rename = "IMAGE_BASIS")]
        #[doc = "This represents an image basis relationship."]
        ImageBasis,
        #[serde(rename = "PACKAGE_MANAGER")]
        #[doc = "This represents a package installed via a package manager."]
        PackageManager,
        #[serde(rename = "DEPLOYABLE")]
        #[doc = "The note and occurrence track deployment events."]
        Deployable,
        #[serde(rename = "DISCOVERY")]
        #[doc = "The note and occurrence track the initial discovery status of a resource."]
        Discovery,
        #[serde(rename = "ATTESTATION_AUTHORITY")]
        #[doc = "This represents a logical \"role\" that can attest to artifacts."]
        AttestationAuthority,
        #[serde(rename = "UPGRADE")]
        #[doc = "This represents an available software upgrade."]
        Upgrade,
    }
    impl ::std::default::Default for DiscoveryAnalysisKindEnum {
        fn default() -> Self {
            Self::KindUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This represents a particular channel of distribution for a given package. e.g. Debian's jessie-backports dpkg mirror"]
    pub struct Distribution {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "architecture")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The CPU architecture for which packages in this distribution channel were built"]
        pub architecture: ::std::option::Option<DistributionArchitectureEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpeUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cpe_uri in [cpe format](https://cpe.mitre.org/specification/) denoting the package manager version distributing a package."]
        pub cpe_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The distribution channel-specific description of this package."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "latestVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The latest available version of this package in this distribution channel."]
        pub latest_version: ::std::option::Option<::std::boxed::Box<Version>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maintainer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A freeform string denoting the maintainer of this package."]
        pub maintainer: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The distribution channel-specific homepage for this package."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl Distribution {
        pub fn builder() -> DistributionBuilder {
            DistributionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The CPU architecture for which packages in this distribution channel were built"]
    pub enum DistributionArchitectureEnum {
        #[serde(rename = "ARCHITECTURE_UNSPECIFIED")]
        #[doc = "Unknown architecture"]
        ArchitectureUnspecified,
        #[serde(rename = "X86")]
        #[doc = "X86 architecture"]
        X86,
        #[serde(rename = "X64")]
        #[doc = "X64 architecture"]
        X64,
    }
    impl ::std::default::Default for DistributionArchitectureEnum {
        fn default() -> Self {
            Self::ArchitectureUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
    pub struct Empty {}
    impl Empty {
        pub fn builder() -> EmptyBuilder {
            EmptyBuilder::default()
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
    #[doc = "Container message for hashes of byte content of files, used in Source messages to verify integrity of source input to the build."]
    pub struct FileHashes {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileHash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Collection of file hashes."]
        pub file_hash: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Hash>>>,
    }
    impl FileHashes {
        pub fn builder() -> FileHashesBuilder {
            FileHashesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A set of properties that uniquely identify a given Docker image."]
    pub struct Fingerprint {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "v1Name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The layer-id of the final layer in the Docker image's v1 representation. This field can be used as a filter in list requests."]
        pub v1_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "v2Blob")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ordered list of v2 blobs that represent a given image."]
        pub v2_blob: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "v2Name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of the image's v2 blobs computed via: [bottom] := v2_blobbottom := sha256(v2_blob[N] + \" \" + v2_name[N+1]) Only the name of the final blob is kept. This field can be used as a filter in list requests."]
        pub v2_name: ::std::option::Option<::std::string::String>,
    }
    impl Fingerprint {
        pub fn builder() -> FingerprintBuilder {
            FingerprintBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for `GetIamPolicy` method."]
    pub struct GetIamPolicyRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "options")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OPTIONAL: A `GetPolicyOptions` object for specifying options to `GetIamPolicy`."]
        pub options: ::std::option::Option<::std::boxed::Box<GetPolicyOptions>>,
    }
    impl GetIamPolicyRequest {
        pub fn builder() -> GetIamPolicyRequestBuilder {
            GetIamPolicyRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Encapsulates settings provided to GetIamPolicy."]
    pub struct GetPolicyOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestedPolicyVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The policy format version to be returned. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional bindings must specify version 3. Policies without any conditional bindings may specify any valid value or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        pub requested_policy_version: ::std::option::Option<::std::primitive::i64>,
    }
    impl GetPolicyOptions {
        pub fn builder() -> GetPolicyOptionsBuilder {
            GetPolicyOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A summary of how many vulnz occurrences there are per severity type. counts by groups, or if we should have different summary messages like this."]
    pub struct GetVulnzOccurrencesSummaryResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "counts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A map of how many occurrences were found for each severity."]
        pub counts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SeverityCount>>>,
    }
    impl GetVulnzOccurrencesSummaryResponse {
        pub fn builder() -> GetVulnzOccurrencesSummaryResponseBuilder {
            GetVulnzOccurrencesSummaryResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An alias to a repo revision."]
    pub struct GoogleDevtoolsContaineranalysisV1alpha1AliasContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The alias kind."]
        pub kind:
            ::std::option::Option<GoogleDevtoolsContaineranalysisV1alpha1AliasContextKindEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The alias name."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsContaineranalysisV1alpha1AliasContext {
        pub fn builder() -> GoogleDevtoolsContaineranalysisV1alpha1AliasContextBuilder {
            GoogleDevtoolsContaineranalysisV1alpha1AliasContextBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The alias kind."]
    pub enum GoogleDevtoolsContaineranalysisV1alpha1AliasContextKindEnum {
        #[serde(rename = "KIND_UNSPECIFIED")]
        #[doc = "Unknown."]
        KindUnspecified,
        #[serde(rename = "FIXED")]
        #[doc = "Git tag."]
        Fixed,
        #[serde(rename = "MOVABLE")]
        #[doc = "Git branch."]
        Movable,
        #[serde(rename = "OTHER")]
        #[doc = "Used to specify non-standard aliases. For example, if a Git repo has a ref named \"refs/foo/bar\"."]
        Other,
    }
    impl ::std::default::Default for GoogleDevtoolsContaineranalysisV1alpha1AliasContextKindEnum {
        fn default() -> Self {
            Self::KindUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A CloudRepoSourceContext denotes a particular revision in a Google Cloud Source Repo."]
    pub struct GoogleDevtoolsContaineranalysisV1alpha1CloudRepoSourceContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aliasContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An alias, which may be a branch or tag."]
        pub alias_context: ::std::option::Option<
            ::std::boxed::Box<GoogleDevtoolsContaineranalysisV1alpha1AliasContext>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "repoId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the repo."]
        pub repo_id:
            ::std::option::Option<::std::boxed::Box<GoogleDevtoolsContaineranalysisV1alpha1RepoId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revisionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A revision ID."]
        pub revision_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsContaineranalysisV1alpha1CloudRepoSourceContext {
        pub fn builder() -> GoogleDevtoolsContaineranalysisV1alpha1CloudRepoSourceContextBuilder {
            GoogleDevtoolsContaineranalysisV1alpha1CloudRepoSourceContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A SourceContext referring to a Gerrit project."]
    pub struct GoogleDevtoolsContaineranalysisV1alpha1GerritSourceContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aliasContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An alias, which may be a branch or tag."]
        pub alias_context: ::std::option::Option<
            ::std::boxed::Box<GoogleDevtoolsContaineranalysisV1alpha1AliasContext>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gerritProject")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full project name within the host. Projects may be nested, so \"project/subproject\" is a valid project name. The \"repo name\" is the hostURI/project."]
        pub gerrit_project: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hostUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI of a running Gerrit instance."]
        pub host_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revisionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A revision (commit) ID."]
        pub revision_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsContaineranalysisV1alpha1GerritSourceContext {
        pub fn builder() -> GoogleDevtoolsContaineranalysisV1alpha1GerritSourceContextBuilder {
            GoogleDevtoolsContaineranalysisV1alpha1GerritSourceContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A GitSourceContext denotes a particular revision in a third party Git repository (e.g., GitHub)."]
    pub struct GoogleDevtoolsContaineranalysisV1alpha1GitSourceContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revisionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Git commit hash."]
        pub revision_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Git repository URL."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsContaineranalysisV1alpha1GitSourceContext {
        pub fn builder() -> GoogleDevtoolsContaineranalysisV1alpha1GitSourceContextBuilder {
            GoogleDevtoolsContaineranalysisV1alpha1GitSourceContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for all operations used and required for all operations that created by Container Analysis Providers"]
    pub struct GoogleDevtoolsContaineranalysisV1alpha1OperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time this operation was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time that this operation was marked completed or failed."]
        pub end_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsContaineranalysisV1alpha1OperationMetadata {
        pub fn builder() -> GoogleDevtoolsContaineranalysisV1alpha1OperationMetadataBuilder {
            GoogleDevtoolsContaineranalysisV1alpha1OperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Selects a repo using a Google Cloud Platform project ID (e.g., winged-cargo-31) and a repo name within that project."]
    pub struct GoogleDevtoolsContaineranalysisV1alpha1ProjectRepoId {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the project."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "repoName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the repo. Leave empty for the default repo."]
        pub repo_name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsContaineranalysisV1alpha1ProjectRepoId {
        pub fn builder() -> GoogleDevtoolsContaineranalysisV1alpha1ProjectRepoIdBuilder {
            GoogleDevtoolsContaineranalysisV1alpha1ProjectRepoIdBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A unique identifier for a Cloud Repo."]
    pub struct GoogleDevtoolsContaineranalysisV1alpha1RepoId {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectRepoId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A combination of a project ID and a repo name."]
        pub project_repo_id: ::std::option::Option<
            ::std::boxed::Box<GoogleDevtoolsContaineranalysisV1alpha1ProjectRepoId>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A server-assigned, globally unique identifier."]
        pub uid: ::std::option::Option<::std::string::String>,
    }
    impl GoogleDevtoolsContaineranalysisV1alpha1RepoId {
        pub fn builder() -> GoogleDevtoolsContaineranalysisV1alpha1RepoIdBuilder {
            GoogleDevtoolsContaineranalysisV1alpha1RepoIdBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A SourceContext is a reference to a tree of files. A SourceContext together with a path point to a unique revision of a single file or directory."]
    pub struct GoogleDevtoolsContaineranalysisV1alpha1SourceContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloudRepo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A SourceContext referring to a revision in a Google Cloud Source Repo."]
        pub cloud_repo: ::std::option::Option<
            ::std::boxed::Box<GoogleDevtoolsContaineranalysisV1alpha1CloudRepoSourceContext>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gerrit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A SourceContext referring to a Gerrit project."]
        pub gerrit: ::std::option::Option<
            ::std::boxed::Box<GoogleDevtoolsContaineranalysisV1alpha1GerritSourceContext>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "git")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A SourceContext referring to any third party Git repo (e.g., GitHub)."]
        pub git: ::std::option::Option<
            ::std::boxed::Box<GoogleDevtoolsContaineranalysisV1alpha1GitSourceContext>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Labels with user defined metadata."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl GoogleDevtoolsContaineranalysisV1alpha1SourceContext {
        pub fn builder() -> GoogleDevtoolsContaineranalysisV1alpha1SourceContextBuilder {
            GoogleDevtoolsContaineranalysisV1alpha1SourceContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Container message for hash values."]
    pub struct Hash {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of hash that was performed."]
        pub _type: ::std::option::Option<HashTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The hash value."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl Hash {
        pub fn builder() -> HashBuilder {
            HashBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of hash that was performed."]
    pub enum HashTypeEnum {
        #[serde(rename = "NONE")]
        #[doc = "No hash requested."]
        None,
        #[serde(rename = "SHA256")]
        #[doc = "A sha256 hash."]
        Sha256,
    }
    impl ::std::default::Default for HashTypeEnum {
        fn default() -> Self {
            Self::None
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This represents how a particular software package may be installed on a system."]
    pub struct Installation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All of the places within the filesystem versions of this package have been found."]
        pub location: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Location>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of the installed package."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl Installation {
        pub fn builder() -> InstallationBuilder {
            InstallationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Layer holds metadata specific to a layer of a Docker image."]
    pub struct Layer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "arguments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The recovered arguments to the Dockerfile directive."]
        pub arguments: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "directive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The recovered Dockerfile directive used to construct this layer."]
        pub directive: ::std::option::Option<LayerDirectiveEnum>,
    }
    impl Layer {
        pub fn builder() -> LayerBuilder {
            LayerBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The recovered Dockerfile directive used to construct this layer."]
    pub enum LayerDirectiveEnum {
        #[serde(rename = "DIRECTIVE_UNSPECIFIED")]
        #[doc = "Default value for unsupported/missing directive"]
        DirectiveUnspecified,
        #[serde(rename = "MAINTAINER")]
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Maintainer,
        #[serde(rename = "RUN")]
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Run,
        #[serde(rename = "CMD")]
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Cmd,
        #[serde(rename = "LABEL")]
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Label,
        #[serde(rename = "EXPOSE")]
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Expose,
        #[serde(rename = "ENV")]
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Env,
        #[serde(rename = "ADD")]
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Add,
        #[serde(rename = "COPY")]
        #[doc = "https://docs.docker.com/reference/builder/#copy"]
        Copy,
        #[serde(rename = "ENTRYPOINT")]
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Entrypoint,
        #[serde(rename = "VOLUME")]
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Volume,
        #[serde(rename = "USER")]
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        User,
        #[serde(rename = "WORKDIR")]
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Workdir,
        #[serde(rename = "ARG")]
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Arg,
        #[serde(rename = "ONBUILD")]
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Onbuild,
        #[serde(rename = "STOPSIGNAL")]
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Stopsignal,
        #[serde(rename = "HEALTHCHECK")]
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Healthcheck,
        #[serde(rename = "SHELL")]
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Shell,
    }
    impl ::std::default::Default for LayerDirectiveEnum {
        fn default() -> Self {
            Self::DirectiveUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response including listed occurrences for a note."]
    pub struct ListNoteOccurrencesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to receive the next page of notes."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "occurrences")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The occurrences attached to the specified note."]
        pub occurrences: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Occurrence>>>,
    }
    impl ListNoteOccurrencesResponse {
        pub fn builder() -> ListNoteOccurrencesResponseBuilder {
            ListNoteOccurrencesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response including listed notes."]
    pub struct ListNotesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The next pagination token in the list response. It should be used as page_token for the following request. An empty value means no more result."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The occurrences requested"]
        pub notes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Note>>>,
    }
    impl ListNotesResponse {
        pub fn builder() -> ListNotesResponseBuilder {
            ListNotesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response including listed active occurrences."]
    pub struct ListOccurrencesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The next pagination token in the list response. It should be used as `page_token` for the following request. An empty value means no more results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "occurrences")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The occurrences requested."]
        pub occurrences: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Occurrence>>>,
    }
    impl ListOccurrencesResponse {
        pub fn builder() -> ListOccurrencesResponseBuilder {
            ListOccurrencesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of scan configs for the project."]
    pub struct ListScanConfigsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A page token to pass in order to get more scan configs."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scanConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of scan configs."]
        pub scan_configs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ScanConfig>>>,
    }
    impl ListScanConfigsResponse {
        pub fn builder() -> ListScanConfigsResponseBuilder {
            ListScanConfigsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An occurrence of a particular package installation found within a system's filesystem. e.g. glibc was found in /var/lib/dpkg/status"]
    pub struct Location {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpeUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cpe_uri in [cpe format](https://cpe.mitre.org/specification/) denoting the package manager version distributing a package."]
        pub cpe_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The path from which we gathered that this package/version is installed."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version installed at this location."]
        pub version: ::std::option::Option<::std::boxed::Box<Version>>,
    }
    impl Location {
        pub fn builder() -> LocationBuilder {
            LocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Provides a detailed description of a `Note`."]
    pub struct Note {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attestationAuthority")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A note describing an attestation role."]
        pub attestation_authority: ::std::option::Option<::std::boxed::Box<AttestationAuthority>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "baseImage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A note describing a base image."]
        pub base_image: ::std::option::Option<::std::boxed::Box<Basis>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buildType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Build provenance type for a verifiable build."]
        pub build_type: ::std::option::Option<::std::boxed::Box<BuildType>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time this note was created. This field can be used as a filter in list requests."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deployable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A note describing something that can be deployed."]
        pub deployable: ::std::option::Option<::std::boxed::Box<Deployable>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "discovery")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A note describing a provider/analysis type."]
        pub discovery: ::std::option::Option<::std::boxed::Box<Discovery>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expirationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time of expiration for this note, null if note does not expire."]
        pub expiration_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. This explicitly denotes which kind of note is specified. This field can be used as a filter in list requests."]
        pub kind: ::std::option::Option<NoteKindEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "longDescription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A detailed description of this `Note`."]
        pub long_description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the note in the form \"projects/{provider_project_id}/notes/{NOTE_ID}\""]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "package")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A note describing a package hosted by various package managers."]
        pub package: ::std::option::Option<::std::boxed::Box<Package>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relatedUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URLs associated with this note"]
        pub related_url: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RelatedUrl>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shortDescription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A one sentence description of this `Note`."]
        pub short_description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time this note was last updated. This field can be used as a filter in list requests."]
        pub update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "upgrade")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A note describing an upgrade."]
        pub upgrade: ::std::option::Option<::std::boxed::Box<UpgradeNote>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vulnerabilityType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A package vulnerability type of note."]
        pub vulnerability_type: ::std::option::Option<::std::boxed::Box<VulnerabilityType>>,
    }
    impl Note {
        pub fn builder() -> NoteBuilder {
            NoteBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. This explicitly denotes which kind of note is specified. This field can be used as a filter in list requests."]
    pub enum NoteKindEnum {
        #[serde(rename = "KIND_UNSPECIFIED")]
        #[doc = "Unknown"]
        KindUnspecified,
        #[serde(rename = "PACKAGE_VULNERABILITY")]
        #[doc = "The note and occurrence represent a package vulnerability."]
        PackageVulnerability,
        #[serde(rename = "BUILD_DETAILS")]
        #[doc = "The note and occurrence assert build provenance."]
        BuildDetails,
        #[serde(rename = "IMAGE_BASIS")]
        #[doc = "This represents an image basis relationship."]
        ImageBasis,
        #[serde(rename = "PACKAGE_MANAGER")]
        #[doc = "This represents a package installed via a package manager."]
        PackageManager,
        #[serde(rename = "DEPLOYABLE")]
        #[doc = "The note and occurrence track deployment events."]
        Deployable,
        #[serde(rename = "DISCOVERY")]
        #[doc = "The note and occurrence track the initial discovery status of a resource."]
        Discovery,
        #[serde(rename = "ATTESTATION_AUTHORITY")]
        #[doc = "This represents a logical \"role\" that can attest to artifacts."]
        AttestationAuthority,
        #[serde(rename = "UPGRADE")]
        #[doc = "This represents an available software upgrade."]
        Upgrade,
    }
    impl ::std::default::Default for NoteKindEnum {
        fn default() -> Self {
            Self::KindUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "`Occurrence` includes information about analysis occurrences for an image."]
    pub struct Occurrence {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attestation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes an attestation of an artifact."]
        pub attestation: ::std::option::Option<::std::boxed::Box<Attestation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buildDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Build details for a verifiable build."]
        pub build_details: ::std::option::Option<::std::boxed::Box<BuildDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time this `Occurrence` was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deployment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes the deployment of an artifact on a runtime."]
        pub deployment: ::std::option::Option<::std::boxed::Box<Deployment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "derivedImage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes how this resource derives from the basis in the associated note."]
        pub derived_image: ::std::option::Option<::std::boxed::Box<Derived>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "discovered")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes the initial scan status for this resource."]
        pub discovered: ::std::option::Option<::std::boxed::Box<Discovered>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "installation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes the installation of a package on the linked resource."]
        pub installation: ::std::option::Option<::std::boxed::Box<Installation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. This explicitly denotes which of the `Occurrence` details are specified. This field can be used as a filter in list requests."]
        pub kind: ::std::option::Option<OccurrenceKindEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of the `Occurrence` in the form \"projects/{project_id}/occurrences/{OCCURRENCE_ID}\""]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "noteName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An analysis note associated with this image, in the form \"providers/{provider_id}/notes/{NOTE_ID}\" This field can be used as a filter in list requests."]
        pub note_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "remediation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A description of actions that can be taken to remedy the `Note`"]
        pub remediation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = " The resource for which the `Occurrence` applies."]
        pub resource: ::std::option::Option<::std::boxed::Box<Resource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique URL of the image or the container for which the `Occurrence` applies. For example, https://gcr.io/project/image@sha256:foo This field can be used as a filter in list requests."]
        pub resource_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time this `Occurrence` was last updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "upgrade")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes an upgrade."]
        pub upgrade: ::std::option::Option<::std::boxed::Box<UpgradeOccurrence>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vulnerabilityDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of a security vulnerability note."]
        pub vulnerability_details: ::std::option::Option<::std::boxed::Box<VulnerabilityDetails>>,
    }
    impl Occurrence {
        pub fn builder() -> OccurrenceBuilder {
            OccurrenceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. This explicitly denotes which of the `Occurrence` details are specified. This field can be used as a filter in list requests."]
    pub enum OccurrenceKindEnum {
        #[serde(rename = "KIND_UNSPECIFIED")]
        #[doc = "Unknown"]
        KindUnspecified,
        #[serde(rename = "PACKAGE_VULNERABILITY")]
        #[doc = "The note and occurrence represent a package vulnerability."]
        PackageVulnerability,
        #[serde(rename = "BUILD_DETAILS")]
        #[doc = "The note and occurrence assert build provenance."]
        BuildDetails,
        #[serde(rename = "IMAGE_BASIS")]
        #[doc = "This represents an image basis relationship."]
        ImageBasis,
        #[serde(rename = "PACKAGE_MANAGER")]
        #[doc = "This represents a package installed via a package manager."]
        PackageManager,
        #[serde(rename = "DEPLOYABLE")]
        #[doc = "The note and occurrence track deployment events."]
        Deployable,
        #[serde(rename = "DISCOVERY")]
        #[doc = "The note and occurrence track the initial discovery status of a resource."]
        Discovery,
        #[serde(rename = "ATTESTATION_AUTHORITY")]
        #[doc = "This represents a logical \"role\" that can attest to artifacts."]
        AttestationAuthority,
        #[serde(rename = "UPGRADE")]
        #[doc = "This represents an available software upgrade."]
        Upgrade,
    }
    impl ::std::default::Default for OccurrenceKindEnum {
        fn default() -> Self {
            Self::KindUnspecified
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
        #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
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
        #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "response")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`."]
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
    #[doc = "This represents a particular package that is distributed over various channels. e.g. glibc (aka libc6) is distributed by many, at various versions."]
    pub struct Package {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "distribution")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The various channels by which a package is distributed."]
        pub distribution: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Distribution>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the package."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl Package {
        pub fn builder() -> PackageBuilder {
            PackageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This message wraps a location affected by a vulnerability and its associated fix (if one is available)."]
    pub struct PackageIssue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "affectedLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location of the vulnerability."]
        pub affected_location: ::std::option::Option<::std::boxed::Box<VulnerabilityLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fixedLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location of the available fix for vulnerability."]
        pub fixed_location: ::std::option::Option<::std::boxed::Box<VulnerabilityLocation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "severityName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub severity_name: ::std::option::Option<::std::string::String>,
    }
    impl PackageIssue {
        pub fn builder() -> PackageIssueBuilder {
            PackageIssueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An attestation wrapper with a PGP-compatible signature. This message only supports `ATTACHED` signatures, where the payload that is signed is included alongside the signature itself in the same file."]
    pub struct PgpSignedAttestation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type (for example schema) of the attestation payload that was signed. The verifier must ensure that the provided type is one that the verifier supports, and that the attestation payload is a valid instantiation of that type (for example by validating a JSON schema)."]
        pub content_type: ::std::option::Option<PgpSignedAttestationContentTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pgpKeyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cryptographic fingerprint of the key used to generate the signature, as output by, e.g. `gpg --list-keys`. This should be the version 4, full 160-bit fingerprint, expressed as a 40 character hexadecimal string. See https://tools.ietf.org/html/rfc4880#section-12.2 for details. Implementations may choose to acknowledge \"LONG\", \"SHORT\", or other abbreviated key IDs, but only the full fingerprint is guaranteed to work. In gpg, the full fingerprint can be retrieved from the `fpr` field returned when calling --list-keys with --with-colons. For example: ``` gpg --with-colons --with-fingerprint --force-v4-certs \\ --list-keys attester@example.com tru::1:1513631572:0:3:1:5 pub:...... fpr:::::::::24FF6481B76AC91E66A00AC657A93A81EF3AE6FB: ``` Above, the fingerprint is `24FF6481B76AC91E66A00AC657A93A81EF3AE6FB`."]
        pub pgp_key_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "signature")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The raw content of the signature, as output by GNU Privacy Guard (GPG) or equivalent. Since this message only supports attached signatures, the payload that was signed must be attached. While the signature format supported is dependent on the verification implementation, currently only ASCII-armored (`--armor` to gpg), non-clearsigned (`--sign` rather than `--clearsign` to gpg) are supported. Concretely, `gpg --sign --armor --output=signature.gpg payload.json` will create the signature content expected in this field in `signature.gpg` for the `payload.json` attestation payload."]
        pub signature: ::std::option::Option<::std::string::String>,
    }
    impl PgpSignedAttestation {
        pub fn builder() -> PgpSignedAttestationBuilder {
            PgpSignedAttestationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Type (for example schema) of the attestation payload that was signed. The verifier must ensure that the provided type is one that the verifier supports, and that the attestation payload is a valid instantiation of that type (for example by validating a JSON schema)."]
    pub enum PgpSignedAttestationContentTypeEnum {
        #[serde(rename = "CONTENT_TYPE_UNSPECIFIED")]
        #[doc = "`ContentType` is not set."]
        ContentTypeUnspecified,
        #[serde(rename = "SIMPLE_SIGNING_JSON")]
        #[doc = "Atomic format attestation signature. See https://github.com/containers/image/blob/8a5d2f82a6e3263290c8e0276c3e0f64e77723e7/docs/atomic-signature.md The payload extracted from `signature` is a JSON blob conforming to the linked schema."]
        SimpleSigningJson,
    }
    impl ::std::default::Default for PgpSignedAttestationContentTypeEnum {
        fn default() -> Self {
            Self::ContentTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members` to a single `role`. Members can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { \"bindings\": [ { \"role\": \"roles/resourcemanager.organizationAdmin\", \"members\": [ \"user:mike@example.com\", \"group:admins@example.com\", \"domain:google.com\", \"serviceAccount:my-project-id@appspot.gserviceaccount.com\" ] }, { \"role\": \"roles/resourcemanager.organizationViewer\", \"members\": [ \"user:eve@example.com\" ], \"condition\": { \"title\": \"expirable access\", \"description\": \"Does not grant access after Sep 2020\", \"expression\": \"request.time < timestamp('2020-10-01T00:00:00.000Z')\", } } ], \"etag\": \"BwWWja0YfJA=\", \"version\": 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') - etag: BwWWja0YfJA= - version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/)."]
    pub struct Policy {
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
    #[doc = "Metadata for any related URL information"]
    pub struct RelatedUrl {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "label")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label to describe usage of the URL"]
        pub label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specific URL to associate with the note"]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl RelatedUrl {
        pub fn builder() -> RelatedUrlBuilder {
            RelatedUrlBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "RepoSource describes the location of the source in a Google Cloud Source Repository."]
    pub struct RepoSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "branchName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the branch to build."]
        pub branch_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commitSha")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Explicit commit SHA to build."]
        pub commit_sha: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the project that owns the repo."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "repoName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the repo."]
        pub repo_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tagName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the tag to build."]
        pub tag_name: ::std::option::Option<::std::string::String>,
    }
    impl RepoSource {
        pub fn builder() -> RepoSourceBuilder {
            RepoSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = " Resource is an entity that can have metadata. E.g., a Docker image."]
    pub struct Resource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentHash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The hash of the resource content. E.g., the Docker digest."]
        pub content_hash: ::std::option::Option<::std::boxed::Box<Hash>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the resource. E.g., the name of a Docker image - \"Debian\"."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique URI of the resource. E.g., \"https://gcr.io/project/image@sha256:foo\" for a Docker image."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl Resource {
        pub fn builder() -> ResourceBuilder {
            ResourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Indicates various scans and whether they are turned on or off."]
    pub struct ScanConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time this scan config was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A human-readable description of what the `ScanConfig` does."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the Scan is enabled."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of the ScanConfig in the form “projects/{project_id}/scanConfigs/{scan_config_id}\"."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time this scan config was last updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl ScanConfig {
        pub fn builder() -> ScanConfigBuilder {
            ScanConfigBuilder::default()
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
    }
    impl SetIamPolicyRequest {
        pub fn builder() -> SetIamPolicyRequestBuilder {
            SetIamPolicyRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The number of occurrences created for a specific severity."]
    pub struct SeverityCount {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of occurrences with the severity."]
        pub count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "severity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The severity of the occurrences."]
        pub severity: ::std::option::Option<SeverityCountSeverityEnum>,
    }
    impl SeverityCount {
        pub fn builder() -> SeverityCountBuilder {
            SeverityCountBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The severity of the occurrences."]
    pub enum SeverityCountSeverityEnum {
        #[serde(rename = "SEVERITY_UNSPECIFIED")]
        #[doc = "Unknown Impact"]
        SeverityUnspecified,
        #[serde(rename = "MINIMAL")]
        #[doc = "Minimal Impact"]
        Minimal,
        #[serde(rename = "LOW")]
        #[doc = "Low Impact"]
        Low,
        #[serde(rename = "MEDIUM")]
        #[doc = "Medium Impact"]
        Medium,
        #[serde(rename = "HIGH")]
        #[doc = "High Impact"]
        High,
        #[serde(rename = "CRITICAL")]
        #[doc = "Critical Impact"]
        Critical,
    }
    impl ::std::default::Default for SeverityCountSeverityEnum {
        fn default() -> Self {
            Self::SeverityUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Source describes the location of the source used for the build."]
    pub struct Source {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalContexts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If provided, some of the source code used for the build may be found in these locations, in the case where the source repository had multiple remotes or submodules. This list will not include the context specified in the context field."]
        pub additional_contexts: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleDevtoolsContaineranalysisV1alpha1SourceContext>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "artifactStorageSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If provided, the input binary artifacts for the build came from this location."]
        pub artifact_storage_source: ::std::option::Option<::std::boxed::Box<StorageSource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "context")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If provided, the source code used for the build came from this location."]
        pub context: ::std::option::Option<
            ::std::boxed::Box<GoogleDevtoolsContaineranalysisV1alpha1SourceContext>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileHashes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hash(es) of the build source, which can be used to verify that the original source integrity was maintained in the build. The keys to this map are file paths used as build source and the values contain the hash values for those files. If the build source came in a single package such as a gzipped tarfile (.tar.gz), the FileHash will be for the single path to that file."]
        pub file_hashes: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<FileHashes>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "repoSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If provided, get source from this location in a Cloud Repo."]
        pub repo_source: ::std::option::Option<::std::boxed::Box<RepoSource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storageSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If provided, get the source from this location in in Google Cloud Storage."]
        pub storage_source: ::std::option::Option<::std::boxed::Box<StorageSource>>,
    }
    impl Source {
        pub fn builder() -> SourceBuilder {
            SourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
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
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "StorageSource describes the location of the source in an archive file in Google Cloud Storage."]
    pub struct StorageSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucket")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage bucket containing source (see [Bucket Name Requirements] (https://cloud.google.com/storage/docs/bucket-naming#requirements))."]
        pub bucket: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "generation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage generation for the object."]
        pub generation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "object")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage object containing source."]
        pub object: ::std::option::Option<::std::string::String>,
    }
    impl StorageSource {
        pub fn builder() -> StorageSourceBuilder {
            StorageSourceBuilder::default()
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
    #[doc = "Request for updating an existing operation"]
    pub struct UpdateOperationRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The operation to create."]
        pub operation: ::std::option::Option<::std::boxed::Box<Operation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateMask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fields to update."]
        pub update_mask: ::std::option::Option<::std::string::String>,
    }
    impl UpdateOperationRequest {
        pub fn builder() -> UpdateOperationRequestBuilder {
            UpdateOperationRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Upgrade Distribution represents metadata about the Upgrade for each operating system (CPE). Some distributions have additional metadata around updates, classifying them into various categories and severities."]
    pub struct UpgradeDistribution {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "classification")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The operating system classification of this Upgrade, as specified by the upstream operating system upgrade feed."]
        pub classification: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpeUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required - The specific operating system this metadata applies to. See https://cpe.mitre.org/specification/."]
        pub cpe_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cve")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cve that would be resolved by this upgrade."]
        pub cve: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "severity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The severity as specified by the upstream operating system."]
        pub severity: ::std::option::Option<::std::string::String>,
    }
    impl UpgradeDistribution {
        pub fn builder() -> UpgradeDistributionBuilder {
            UpgradeDistributionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An Upgrade Note represents a potential upgrade of a package to a given version. For each package version combination (i.e. bash 4.0, bash 4.1, bash 4.1.2), there will be a Upgrade Note."]
    pub struct UpgradeNote {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "distributions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata about the upgrade for each specific operating system."]
        pub distributions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UpgradeDistribution>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "package")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required - The package this Upgrade is for."]
        pub package: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required - The version of the package in machine + human readable form."]
        pub version: ::std::option::Option<::std::boxed::Box<Version>>,
    }
    impl UpgradeNote {
        pub fn builder() -> UpgradeNoteBuilder {
            UpgradeNoteBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An Upgrade Occurrence represents that a specific resource_url could install a specific upgrade. This presence is supplied via local sources (i.e. it is present in the mirror and the running system has noticed its availability)."]
    pub struct UpgradeOccurrence {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "distribution")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata about the upgrade for available for the specific operating system for the resource_url. This allows efficient filtering, as well as making it easier to use the occurrence."]
        pub distribution: ::std::option::Option<::std::boxed::Box<UpgradeDistribution>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "package")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required - The package this Upgrade is for."]
        pub package: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parsedVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required - The version of the package in a machine + human readable form."]
        pub parsed_version: ::std::option::Option<::std::boxed::Box<Version>>,
    }
    impl UpgradeOccurrence {
        pub fn builder() -> UpgradeOccurrenceBuilder {
            UpgradeOccurrenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Version contains structured information about the version of the package. For a discussion of this in Debian/Ubuntu: http://serverfault.com/questions/604541/debian-packages-version-convention For a discussion of this in Redhat/Fedora/Centos: http://blog.jasonantman.com/2014/07/how-yum-and-rpm-compare-versions/"]
    pub struct Version {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "epoch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Used to correct mistakes in the version numbering scheme."]
        pub epoch: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inclusive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this version is vulnerable, when defining the version bounds. For example, if the minimum version is 2.0, inclusive=true would say 2.0 is vulnerable, while inclusive=false would say it's not"]
        pub inclusive: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Distinguish between sentinel MIN/MAX versions and normal versions. If kind is not NORMAL, then the other fields are ignored."]
        pub kind: ::std::option::Option<VersionKindEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The main part of the version name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The iteration of the package build from the above version."]
        pub revision: ::std::option::Option<::std::string::String>,
    }
    impl Version {
        pub fn builder() -> VersionBuilder {
            VersionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Distinguish between sentinel MIN/MAX versions and normal versions. If kind is not NORMAL, then the other fields are ignored."]
    pub enum VersionKindEnum {
        #[serde(rename = "NORMAL")]
        #[doc = "A standard package version, defined by the other fields."]
        Normal,
        #[serde(rename = "MINIMUM")]
        #[doc = "A special version representing negative infinity, other fields are ignored."]
        Minimum,
        #[serde(rename = "MAXIMUM")]
        #[doc = "A special version representing positive infinity, other fields are ignored."]
        Maximum,
    }
    impl ::std::default::Default for VersionKindEnum {
        fn default() -> Self {
            Self::Normal
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Used by Occurrence to point to where the vulnerability exists and how to fix it."]
    pub struct VulnerabilityDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cvssScore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The CVSS score of this vulnerability. CVSS score is on a scale of 0-10 where 0 indicates low severity and 10 indicates high severity."]
        pub cvss_score: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "effectiveSeverity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The distro assigned severity for this vulnerability when that is available and note provider assigned severity when distro has not yet assigned a severity for this vulnerability."]
        pub effective_severity: ::std::option::Option<VulnerabilityDetailsEffectiveSeverityEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packageIssue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of affected locations and their fixes (if available) within the associated resource."]
        pub package_issue: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PackageIssue>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "severity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The note provider assigned Severity of the vulnerability."]
        pub severity: ::std::option::Option<VulnerabilityDetailsSeverityEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of package; whether native or non native(ruby gems, node.js packages etc)"]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl VulnerabilityDetails {
        pub fn builder() -> VulnerabilityDetailsBuilder {
            VulnerabilityDetailsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The distro assigned severity for this vulnerability when that is available and note provider assigned severity when distro has not yet assigned a severity for this vulnerability."]
    pub enum VulnerabilityDetailsEffectiveSeverityEnum {
        #[serde(rename = "SEVERITY_UNSPECIFIED")]
        #[doc = "Unknown Impact"]
        SeverityUnspecified,
        #[serde(rename = "MINIMAL")]
        #[doc = "Minimal Impact"]
        Minimal,
        #[serde(rename = "LOW")]
        #[doc = "Low Impact"]
        Low,
        #[serde(rename = "MEDIUM")]
        #[doc = "Medium Impact"]
        Medium,
        #[serde(rename = "HIGH")]
        #[doc = "High Impact"]
        High,
        #[serde(rename = "CRITICAL")]
        #[doc = "Critical Impact"]
        Critical,
    }
    impl ::std::default::Default for VulnerabilityDetailsEffectiveSeverityEnum {
        fn default() -> Self {
            Self::SeverityUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The note provider assigned Severity of the vulnerability."]
    pub enum VulnerabilityDetailsSeverityEnum {
        #[serde(rename = "SEVERITY_UNSPECIFIED")]
        #[doc = "Unknown Impact"]
        SeverityUnspecified,
        #[serde(rename = "MINIMAL")]
        #[doc = "Minimal Impact"]
        Minimal,
        #[serde(rename = "LOW")]
        #[doc = "Low Impact"]
        Low,
        #[serde(rename = "MEDIUM")]
        #[doc = "Medium Impact"]
        Medium,
        #[serde(rename = "HIGH")]
        #[doc = "High Impact"]
        High,
        #[serde(rename = "CRITICAL")]
        #[doc = "Critical Impact"]
        Critical,
    }
    impl ::std::default::Default for VulnerabilityDetailsSeverityEnum {
        fn default() -> Self {
            Self::SeverityUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The location of the vulnerability"]
    pub struct VulnerabilityLocation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cpeUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cpe_uri in [cpe format] (https://cpe.mitre.org/specification/) format. Examples include distro or storage location for vulnerable jar. This field can be used as a filter in list requests."]
        pub cpe_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "package")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The package being described."]
        pub package: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version of the package being described. This field can be used as a filter in list requests."]
        pub version: ::std::option::Option<::std::boxed::Box<Version>>,
    }
    impl VulnerabilityLocation {
        pub fn builder() -> VulnerabilityLocationBuilder {
            VulnerabilityLocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "VulnerabilityType provides metadata about a security vulnerability."]
    pub struct VulnerabilityType {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cvssScore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The CVSS score for this Vulnerability."]
        pub cvss_score: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "details")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All information about the package to specifically identify this vulnerability. One entry per (version range and cpe_uri) the package vulnerability has manifested in."]
        pub details: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Detail>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "severity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Note provider assigned impact of the vulnerability"]
        pub severity: ::std::option::Option<VulnerabilityTypeSeverityEnum>,
    }
    impl VulnerabilityType {
        pub fn builder() -> VulnerabilityTypeBuilder {
            VulnerabilityTypeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Note provider assigned impact of the vulnerability"]
    pub enum VulnerabilityTypeSeverityEnum {
        #[serde(rename = "SEVERITY_UNSPECIFIED")]
        #[doc = "Unknown Impact"]
        SeverityUnspecified,
        #[serde(rename = "MINIMAL")]
        #[doc = "Minimal Impact"]
        Minimal,
        #[serde(rename = "LOW")]
        #[doc = "Low Impact"]
        Low,
        #[serde(rename = "MEDIUM")]
        #[doc = "Medium Impact"]
        Medium,
        #[serde(rename = "HIGH")]
        #[doc = "High Impact"]
        High,
        #[serde(rename = "CRITICAL")]
        #[doc = "Critical Impact"]
        Critical,
    }
    impl ::std::default::Default for VulnerabilityTypeSeverityEnum {
        fn default() -> Self {
            Self::SeverityUnspecified
        }
    }
}
