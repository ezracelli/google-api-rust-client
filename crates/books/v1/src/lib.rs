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
    pub mod bookshelves {
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "source")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "String to identify the originator of this request."]
                    pub source: ::std::option::Option<::std::string::String>,
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "source")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "String to identify the originator of this request."]
                    pub source: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
        pub mod resources {
            pub mod volumes {
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
                            #[serde(rename = "maxResults")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum number of results to return"]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "showPreorders")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Set to true to show pre-ordered books. Defaults to false."]
                            pub show_preorders: ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "source")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "String to identify the originator of this request."]
                            pub source: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "startIndex")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Index of the first element to return (starts at 0)"]
                            pub start_index: ::std::option::Option<::std::primitive::i64>,
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
    pub mod cloudloading {
        pub mod methods {
            pub mod add_book {
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
                    #[serde(rename = "drive_document_id")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A drive document id. The upload_client_token must not be set."]
                    pub drive_document_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "mime_type")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The document MIME type. It can be set only if the drive_document_id is set."]
                    pub mime_type: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "name")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The document name. It can be set only if the drive_document_id is set."]
                    pub name: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "upload_client_token")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Scotty upload token."]
                    pub upload_client_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod delete_book {
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
                    #[serde(rename = "volumeId")]
                    #[doc = "The id of the book to be removed."]
                    pub volume_id: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod dictionary {
        pub mod methods {
            pub mod list_offline_metadata {
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
                    #[serde(rename = "cpksver")]
                    #[doc = "The device/version ID from which to request the data."]
                    pub cpksver: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod familysharing {
        pub mod methods {
            pub mod get_family_info {
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
                    #[serde(rename = "source")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "String to identify the originator of this request."]
                    pub source: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod share {
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
                    #[serde(rename = "docId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The docid to share."]
                    pub doc_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "source")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "String to identify the originator of this request."]
                    pub source: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "volumeId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The volume to share."]
                    pub volume_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod unshare {
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
                    #[serde(rename = "docId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The docid to unshare."]
                    pub doc_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "source")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "String to identify the originator of this request."]
                    pub source: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "volumeId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The volume to unshare."]
                    pub volume_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod layers {
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "contentVersion")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The content version for the requested volume."]
                    pub content_version: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "source")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "String to identify the originator of this request."]
                    pub source: ::std::option::Option<::std::string::String>,
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "contentVersion")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The content version for the requested volume."]
                    pub content_version: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Maximum number of results to return"]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The value of the nextToken from the previous page."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "source")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "String to identify the originator of this request."]
                    pub source: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
        pub mod resources {
            pub mod annotation_data {
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
                            #[serde(rename = "allowWebDefinitions")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "For the dictionary layer. Whether or not to allow web definitions."]
                            pub allow_web_definitions:
                                ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "contentVersion")]
                            #[doc = "The content version for the volume you are trying to retrieve."]
                            pub content_version: ::std::string::String,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "h")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The requested pixel height for any images. If height is provided width must also be provided."]
                            pub h: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "locale")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The locale information for the data. ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'."]
                            pub locale: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "scale")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The requested scale for the image."]
                            pub scale: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "source")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "String to identify the originator of this request."]
                            pub source: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "w")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The requested pixel width for any images. If width is provided height must also be provided."]
                            pub w: ::std::option::Option<::std::primitive::i64>,
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
                            #[serde(rename = "annotationDataId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The list of Annotation Data Ids to retrieve. Pagination is ignored if this is set."]
                            pub annotation_data_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "contentVersion")]
                            #[doc = "The content version for the requested volume."]
                            pub content_version: ::std::string::String,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "h")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The requested pixel height for any images. If height is provided width must also be provided."]
                            pub h: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "locale")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The locale information for the data. ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'."]
                            pub locale: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "maxResults")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum number of results to return"]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The value of the nextToken from the previous page."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "scale")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The requested scale for the image."]
                            pub scale: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "source")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "String to identify the originator of this request."]
                            pub source: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "updatedMax")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "RFC 3339 timestamp to restrict to items updated prior to this timestamp (exclusive)."]
                            pub updated_max: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "updatedMin")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "RFC 3339 timestamp to restrict to items updated since this timestamp (inclusive)."]
                            pub updated_min: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "w")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The requested pixel width for any images. If width is provided height must also be provided."]
                            pub w: ::std::option::Option<::std::primitive::i64>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod volume_annotations {
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
                            #[serde(rename = "locale")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The locale information for the data. ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'."]
                            pub locale: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "source")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "String to identify the originator of this request."]
                            pub source: ::std::option::Option<::std::string::String>,
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
                            #[serde(rename = "contentVersion")]
                            #[doc = "The content version for the requested volume."]
                            pub content_version: ::std::string::String,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "endOffset")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The end offset to end retrieving data from."]
                            pub end_offset: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "endPosition")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The end position to end retrieving data from."]
                            pub end_position: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "locale")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The locale information for the data. ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'."]
                            pub locale: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "maxResults")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum number of results to return"]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The value of the nextToken from the previous page."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "showDeleted")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Set to true to return deleted annotations. updatedMin must be in the request to use this. Defaults to false."]
                            pub show_deleted: ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "source")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "String to identify the originator of this request."]
                            pub source: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "startOffset")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The start offset to start retrieving data from."]
                            pub start_offset: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "startPosition")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The start position to start retrieving data from."]
                            pub start_position: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "updatedMax")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "RFC 3339 timestamp to restrict to items updated prior to this timestamp (exclusive)."]
                            pub updated_max: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "updatedMin")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "RFC 3339 timestamp to restrict to items updated since this timestamp (inclusive)."]
                            pub updated_min: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "volumeAnnotationsVersion")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The version of the volume annotations that you are requesting."]
                            pub volume_annotations_version:
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
    pub mod myconfig {
        pub mod methods {
            pub mod get_user_settings {
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
                    #[serde(rename = "country")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Unused. Added only to workaround TEX mandatory request template requirement"]
                    pub country: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod release_download_access {
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
                    #[serde(rename = "cpksver")]
                    #[doc = "The device/version ID from which to release the restriction."]
                    pub cpksver: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "locale")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "ISO-639-1, ISO-3166-1 codes for message localization, i.e. en_US."]
                    pub locale: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "source")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "String to identify the originator of this request."]
                    pub source: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "volumeIds")]
                    #[doc = "The volume(s) to release restrictions for."]
                    pub volume_ids: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod request_access {
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
                    #[serde(rename = "cpksver")]
                    #[doc = "The device/version ID from which to request the restrictions."]
                    pub cpksver: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "licenseTypes")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The type of access license to request. If not specified, the default is BOTH."]
                    pub license_types: ::std::option::Option<QueryParametersLicenseTypesEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "locale")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "ISO-639-1, ISO-3166-1 codes for message localization, i.e. en_US."]
                    pub locale: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "nonce")]
                    #[doc = "The client nonce value."]
                    pub nonce: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "source")]
                    #[doc = "String to identify the originator of this request."]
                    pub source: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "volumeId")]
                    #[doc = "The volume to request concurrent/download restrictions for."]
                    pub volume_id: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "The type of access license to request. If not specified, the default is BOTH."]
                pub enum QueryParametersLicenseTypesEnum {
                    #[serde(rename = "LICENSE_TYPES_UNDEFINED")]
                    #[doc = ""]
                    LicenseTypesUndefined,
                    #[serde(rename = "BOTH")]
                    #[doc = "Both concurrent and download licenses."]
                    Both,
                    #[serde(rename = "CONCURRENT")]
                    #[doc = "Concurrent access license."]
                    Concurrent,
                    #[serde(rename = "DOWNLOAD")]
                    #[doc = "Offline download access license."]
                    Download,
                }
                impl ::std::default::Default for QueryParametersLicenseTypesEnum {
                    fn default() -> Self {
                        Self::LicenseTypesUndefined
                    }
                }
            }
            pub mod sync_volume_licenses {
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
                    #[serde(rename = "cpksver")]
                    #[doc = "The device/version ID from which to release the restriction."]
                    pub cpksver: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "features")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "List of features supported by the client, i.e., 'RENTALS'"]
                    pub features: ::std::option::Option<QueryParametersFeaturesEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "includeNonComicsSeries")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Set to true to include non-comics series. Defaults to false."]
                    pub include_non_comics_series: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "locale")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "ISO-639-1, ISO-3166-1 codes for message localization, i.e. en_US."]
                    pub locale: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "nonce")]
                    #[doc = "The client nonce value."]
                    pub nonce: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "showPreorders")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Set to true to show pre-ordered books. Defaults to false."]
                    pub show_preorders: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "source")]
                    #[doc = "String to identify the originator of this request."]
                    pub source: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "volumeIds")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The volume(s) to request download restrictions for."]
                    pub volume_ids: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "List of features supported by the client, i.e., 'RENTALS'"]
                pub enum QueryParametersFeaturesEnum {
                    #[serde(rename = "FEATURES_UNDEFINED")]
                    #[doc = ""]
                    FeaturesUndefined,
                    #[serde(rename = "RENTALS")]
                    #[doc = "Client supports rentals."]
                    Rentals,
                }
                impl ::std::default::Default for QueryParametersFeaturesEnum {
                    fn default() -> Self {
                        Self::FeaturesUndefined
                    }
                }
            }
        }
    }
    pub mod mylibrary {
        pub mod resources {
            pub mod annotations {
                pub mod methods {
                    pub mod delete {
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
                            #[serde(rename = "source")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "String to identify the originator of this request."]
                            pub source: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "annotationId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The ID for the annotation to insert."]
                            pub annotation_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "country")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "ISO-3166-1 code to override the IP-based location."]
                            pub country: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "showOnlySummaryInResponse")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Requests that only the summary of the specified layer be provided in the response."]
                            pub show_only_summary_in_response:
                                ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "source")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "String to identify the originator of this request."]
                            pub source: ::std::option::Option<::std::string::String>,
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
                            #[serde(rename = "contentVersion")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The content version for the requested volume."]
                            pub content_version: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "layerId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The layer ID to limit annotation by."]
                            pub layer_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "layerIds")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The layer ID(s) to limit annotation by."]
                            pub layer_ids: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "maxResults")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum number of results to return"]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The value of the nextToken from the previous page."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "showDeleted")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Set to true to return deleted annotations. updatedMin must be in the request to use this. Defaults to false."]
                            pub show_deleted: ::std::option::Option<::std::primitive::bool>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "source")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "String to identify the originator of this request."]
                            pub source: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "updatedMax")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "RFC 3339 timestamp to restrict to items updated prior to this timestamp (exclusive)."]
                            pub updated_max: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "updatedMin")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "RFC 3339 timestamp to restrict to items updated since this timestamp (inclusive)."]
                            pub updated_min: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "volumeId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The volume to restrict annotations to."]
                            pub volume_id: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                    pub mod summary {
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
                            #[serde(rename = "layerIds")]
                            #[doc = "Array of layer IDs to get the summary for."]
                            pub layer_ids: ::std::string::String,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "volumeId")]
                            #[doc = "Volume id to get the summary for."]
                            pub volume_id: ::std::string::String,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                    pub mod update {
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
                            #[serde(rename = "source")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "String to identify the originator of this request."]
                            pub source: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
            }
            pub mod bookshelves {
                pub mod methods {
                    pub mod add_volume {
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
                            #[serde(rename = "reason")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The reason for which the book is added to the library."]
                            pub reason: ::std::option::Option<QueryParametersReasonEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "source")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "String to identify the originator of this request."]
                            pub source: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "volumeId")]
                            #[doc = "ID of volume to add."]
                            pub volume_id: ::std::string::String,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "The reason for which the book is added to the library."]
                        pub enum QueryParametersReasonEnum {
                            #[serde(rename = "REASON_UNDEFINED")]
                            #[doc = ""]
                            ReasonUndefined,
                            #[serde(rename = "IOS_PREX")]
                            #[doc = "Volumes added from the PREX flow on iOS."]
                            IosPrex,
                            #[serde(rename = "IOS_SEARCH")]
                            #[doc = "Volumes added from the Search flow on iOS."]
                            IosSearch,
                            #[serde(rename = "ONBOARDING")]
                            #[doc = "Volumes added from the Onboarding flow."]
                            Onboarding,
                        }
                        impl ::std::default::Default for QueryParametersReasonEnum {
                            fn default() -> Self {
                                Self::ReasonUndefined
                            }
                        }
                    }
                    pub mod clear_volumes {
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
                            #[serde(rename = "source")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "String to identify the originator of this request."]
                            pub source: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
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
                            #[serde(rename = "source")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "String to identify the originator of this request."]
                            pub source: ::std::option::Option<::std::string::String>,
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
                            #[serde(rename = "source")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "String to identify the originator of this request."]
                            pub source: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                    pub mod move_volume {
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
                            #[serde(rename = "source")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "String to identify the originator of this request."]
                            pub source: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "volumeId")]
                            #[doc = "ID of volume to move."]
                            pub volume_id: ::std::string::String,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "volumePosition")]
                            #[doc = "Position on shelf to move the item (0 puts the item before the current first item, 1 puts it between the first and the second and so on.)"]
                            pub volume_position: ::std::primitive::i64,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                    pub mod remove_volume {
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
                            #[serde(rename = "reason")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The reason for which the book is removed from the library."]
                            pub reason: ::std::option::Option<QueryParametersReasonEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "source")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "String to identify the originator of this request."]
                            pub source: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "volumeId")]
                            #[doc = "ID of volume to remove."]
                            pub volume_id: ::std::string::String,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "The reason for which the book is removed from the library."]
                        pub enum QueryParametersReasonEnum {
                            #[serde(rename = "REASON_UNDEFINED")]
                            #[doc = ""]
                            ReasonUndefined,
                            #[serde(rename = "ONBOARDING")]
                            #[doc = "Samples removed from the Onboarding flow."]
                            Onboarding,
                        }
                        impl ::std::default::Default for QueryParametersReasonEnum {
                            fn default() -> Self {
                                Self::ReasonUndefined
                            }
                        }
                    }
                }
                pub mod resources {
                    pub mod volumes {
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
                                    #[serde(rename = "country")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "ISO-3166-1 code to override the IP-based location."]
                                    pub country: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "maxResults")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Maximum number of results to return"]
                                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "projection")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Restrict information returned to a set of selected fields."]
                                    pub projection:
                                        ::std::option::Option<QueryParametersProjectionEnum>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "q")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Full-text search query string in this bookshelf."]
                                    pub q: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "showPreorders")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Set to true to show pre-ordered books. Defaults to false."]
                                    pub show_preorders:
                                        ::std::option::Option<::std::primitive::bool>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "source")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "String to identify the originator of this request."]
                                    pub source: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "startIndex")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Index of the first element to return (starts at 0)"]
                                    pub start_index: ::std::option::Option<::std::primitive::i64>,
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
                                #[doc = "Restrict information returned to a set of selected fields."]
                                pub enum QueryParametersProjectionEnum {
                                    #[serde(rename = "PROJECTION_UNDEFINED")]
                                    #[doc = ""]
                                    ProjectionUndefined,
                                    #[serde(rename = "FULL")]
                                    #[doc = "Includes all volume data."]
                                    Full,
                                    #[serde(rename = "LITE")]
                                    #[doc = "Includes a subset of fields in volumeInfo and accessInfo."]
                                    Lite,
                                }
                                impl ::std::default::Default for QueryParametersProjectionEnum {
                                    fn default() -> Self {
                                        Self::ProjectionUndefined
                                    }
                                }
                            }
                        }
                    }
                }
            }
            pub mod readingpositions {
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
                            #[serde(rename = "contentVersion")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Volume content version for which this reading position is requested."]
                            pub content_version: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "source")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "String to identify the originator of this request."]
                            pub source: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                    pub mod set_position {
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
                            #[serde(rename = "action")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Action that caused this reading position to be set."]
                            pub action: ::std::option::Option<QueryParametersActionEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "contentVersion")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Volume content version for which this reading position applies."]
                            pub content_version: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "deviceCookie")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Random persistent device cookie optional on set position."]
                            pub device_cookie: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "position")]
                            #[doc = "Position string for the new volume reading position."]
                            pub position: ::std::string::String,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "source")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "String to identify the originator of this request."]
                            pub source: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "timestamp")]
                            #[doc = "RFC 3339 UTC format timestamp associated with this reading position."]
                            pub timestamp: ::std::string::String,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Action that caused this reading position to be set."]
                        pub enum QueryParametersActionEnum {
                            #[serde(rename = "ACTION_UNDEFINED")]
                            #[doc = ""]
                            ActionUndefined,
                            #[serde(rename = "bookmark")]
                            #[doc = "User chose bookmark within volume."]
                            Bookmark,
                            #[serde(rename = "chapter")]
                            #[doc = "User selected chapter from list."]
                            Chapter,
                            #[serde(rename = "next-page")]
                            #[doc = "Next page event."]
                            NextPage,
                            #[serde(rename = "prev-page")]
                            #[doc = "Previous page event."]
                            PrevPage,
                            #[serde(rename = "scroll")]
                            #[doc = "User navigated to page."]
                            Scroll,
                            #[serde(rename = "search")]
                            #[doc = "User chose search results within volume."]
                            Search,
                        }
                        impl ::std::default::Default for QueryParametersActionEnum {
                            fn default() -> Self {
                                Self::ActionUndefined
                            }
                        }
                    }
                }
            }
        }
    }
    pub mod notification {
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "locale")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating notification title and body."]
                    pub locale: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "notification_id")]
                    #[doc = "String to identify the notification."]
                    pub notification_id: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "source")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "String to identify the originator of this request."]
                    pub source: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod onboarding {
        pub mod methods {
            pub mod list_categories {
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
                    #[serde(rename = "locale")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "ISO-639-1 language and ISO-3166-1 country code. Default is en-US if unset."]
                    pub locale: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod list_category_volumes {
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
                    #[serde(rename = "categoryId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "List of category ids requested."]
                    pub category_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "locale")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "ISO-639-1 language and ISO-3166-1 country code. Default is en-US if unset."]
                    pub locale: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxAllowedMaturityRating")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum allowed maturity rating of returned volumes. Books with a higher maturity rating are filtered out."]
                    pub max_allowed_maturity_rating:
                        ::std::option::Option<QueryParametersMaxAllowedMaturityRatingEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Number of maximum results per page to be included in the response."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The value of the nextToken from the previous page."]
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
                #[doc = "The maximum allowed maturity rating of returned volumes. Books with a higher maturity rating are filtered out."]
                pub enum QueryParametersMaxAllowedMaturityRatingEnum {
                    #[serde(rename = "MAX_ALLOWED_MATURITY_RATING_UNDEFINED")]
                    #[doc = ""]
                    MaxAllowedMaturityRatingUndefined,
                    #[serde(rename = "MATURE")]
                    #[doc = "Show books which are rated mature or lower."]
                    Mature,
                    #[serde(rename = "not-mature")]
                    #[doc = "Show books which are rated not mature."]
                    NotMature,
                }
                impl ::std::default::Default for QueryParametersMaxAllowedMaturityRatingEnum {
                    fn default() -> Self {
                        Self::MaxAllowedMaturityRatingUndefined
                    }
                }
            }
        }
    }
    pub mod personalizedstream {
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "locale")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating recommendations."]
                    pub locale: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxAllowedMaturityRating")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum allowed maturity rating of returned recommendations. Books with a higher maturity rating are filtered out."]
                    pub max_allowed_maturity_rating:
                        ::std::option::Option<QueryParametersMaxAllowedMaturityRatingEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "source")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "String to identify the originator of this request."]
                    pub source: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "The maximum allowed maturity rating of returned recommendations. Books with a higher maturity rating are filtered out."]
                pub enum QueryParametersMaxAllowedMaturityRatingEnum {
                    #[serde(rename = "MAX_ALLOWED_MATURITY_RATING_UNDEFINED")]
                    #[doc = ""]
                    MaxAllowedMaturityRatingUndefined,
                    #[serde(rename = "MATURE")]
                    #[doc = "Show books which are rated mature or lower."]
                    Mature,
                    #[serde(rename = "not-mature")]
                    #[doc = "Show books which are rated not mature."]
                    NotMature,
                }
                impl ::std::default::Default for QueryParametersMaxAllowedMaturityRatingEnum {
                    fn default() -> Self {
                        Self::MaxAllowedMaturityRatingUndefined
                    }
                }
            }
        }
    }
    pub mod promooffer {
        pub mod methods {
            pub mod accept {
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
                    #[serde(rename = "androidId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "device android_id"]
                    pub android_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "device")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "device device"]
                    pub device: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "manufacturer")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "device manufacturer"]
                    pub manufacturer: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "model")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "device model"]
                    pub model: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "offerId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub offer_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "product")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "device product"]
                    pub product: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "serial")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "device serial"]
                    pub serial: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "volumeId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Volume id to exercise the offer"]
                    pub volume_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod dismiss {
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
                    #[serde(rename = "androidId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "device android_id"]
                    pub android_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "device")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "device device"]
                    pub device: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "manufacturer")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "device manufacturer"]
                    pub manufacturer: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "model")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "device model"]
                    pub model: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "offerId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Offer to dimiss"]
                    pub offer_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "product")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "device product"]
                    pub product: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "serial")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "device serial"]
                    pub serial: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "androidId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "device android_id"]
                    pub android_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "device")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "device device"]
                    pub device: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "manufacturer")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "device manufacturer"]
                    pub manufacturer: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "model")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "device model"]
                    pub model: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "product")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "device product"]
                    pub product: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "serial")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "device serial"]
                    pub serial: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod series {
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "series_id")]
                    #[doc = "String that identifies the series"]
                    pub series_id: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
        pub mod resources {
            pub mod membership {
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
                            #[serde(rename = "page_size")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Number of maximum results per page to be included in the response."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "page_token")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The value of the nextToken from the previous page."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "series_id")]
                            #[doc = "String that identifies the series"]
                            pub series_id: ::std::string::String,
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
    pub mod volumes {
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "country")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "ISO-3166-1 code to override the IP-based location."]
                    pub country: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "includeNonComicsSeries")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Set to true to include non-comics series. Defaults to false."]
                    pub include_non_comics_series: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "partner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Brand results for partner ID."]
                    pub partner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "projection")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restrict information returned to a set of selected fields."]
                    pub projection: ::std::option::Option<QueryParametersProjectionEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "source")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "string to identify the originator of this request."]
                    pub source: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "user_library_consistent_read")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub user_library_consistent_read: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Restrict information returned to a set of selected fields."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "PROJECTION_UNDEFINED")]
                    #[doc = ""]
                    ProjectionUndefined,
                    #[serde(rename = "FULL")]
                    #[doc = "Includes all volume data."]
                    Full,
                    #[serde(rename = "LITE")]
                    #[doc = "Includes a subset of fields in volumeInfo and accessInfo."]
                    Lite,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::ProjectionUndefined
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "download")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restrict to volumes by download availability."]
                    pub download: ::std::option::Option<QueryParametersDownloadEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "filter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filter search results."]
                    pub filter: ::std::option::Option<QueryParametersFilterEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "langRestrict")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restrict results to books with this language code."]
                    pub lang_restrict: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "libraryRestrict")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restrict search to this user's library."]
                    pub library_restrict: ::std::option::Option<QueryParametersLibraryRestrictEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxAllowedMaturityRating")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum allowed maturity rating of returned recommendations. Books with a higher maturity rating are filtered out."]
                    pub max_allowed_maturity_rating:
                        ::std::option::Option<QueryParametersMaxAllowedMaturityRatingEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Maximum number of results to return."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "orderBy")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Sort search results."]
                    pub order_by: ::std::option::Option<QueryParametersOrderByEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "partner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restrict and brand results for partner ID."]
                    pub partner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "printType")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restrict to books or magazines."]
                    pub print_type: ::std::option::Option<QueryParametersPrintTypeEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "projection")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restrict information returned to a set of selected fields."]
                    pub projection: ::std::option::Option<QueryParametersProjectionEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "q")]
                    #[doc = "Full-text search query string."]
                    pub q: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "showPreorders")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Set to true to show books available for preorder. Defaults to false."]
                    pub show_preorders: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "source")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "String to identify the originator of this request."]
                    pub source: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "startIndex")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Index of the first result to return (starts at 0)"]
                    pub start_index: ::std::option::Option<::std::primitive::i64>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Restrict to volumes by download availability."]
                pub enum QueryParametersDownloadEnum {
                    #[serde(rename = "DOWNLOAD_UNDEFINED")]
                    #[doc = ""]
                    DownloadUndefined,
                    #[serde(rename = "EPUB")]
                    #[doc = "All volumes with epub."]
                    Epub,
                }
                impl ::std::default::Default for QueryParametersDownloadEnum {
                    fn default() -> Self {
                        Self::DownloadUndefined
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Filter search results."]
                pub enum QueryParametersFilterEnum {
                    #[serde(rename = "FILTER_UNDEFINED")]
                    #[doc = ""]
                    FilterUndefined,
                    #[serde(rename = "ebooks")]
                    #[doc = "All Google eBooks."]
                    Ebooks,
                    #[serde(rename = "free-ebooks")]
                    #[doc = "Google eBook with full volume text viewability."]
                    FreeEbooks,
                    #[serde(rename = "full")]
                    #[doc = "Public can view entire volume text."]
                    Full,
                    #[serde(rename = "paid-ebooks")]
                    #[doc = "Google eBook with a price."]
                    PaidEbooks,
                    #[serde(rename = "partial")]
                    #[doc = "Public able to see parts of text."]
                    Partial,
                }
                impl ::std::default::Default for QueryParametersFilterEnum {
                    fn default() -> Self {
                        Self::FilterUndefined
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Restrict search to this user's library."]
                pub enum QueryParametersLibraryRestrictEnum {
                    #[serde(rename = "LIBRARY_RESTRICT_UNDEFINED")]
                    #[doc = ""]
                    LibraryRestrictUndefined,
                    #[serde(rename = "my-library")]
                    #[doc = "Restrict to the user's library, any shelf."]
                    MyLibrary,
                    #[serde(rename = "no-restrict")]
                    #[doc = "Do not restrict based on user's library."]
                    NoRestrict,
                }
                impl ::std::default::Default for QueryParametersLibraryRestrictEnum {
                    fn default() -> Self {
                        Self::LibraryRestrictUndefined
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "The maximum allowed maturity rating of returned recommendations. Books with a higher maturity rating are filtered out."]
                pub enum QueryParametersMaxAllowedMaturityRatingEnum {
                    #[serde(rename = "MAX_ALLOWED_MATURITY_RATING_UNDEFINED")]
                    #[doc = ""]
                    MaxAllowedMaturityRatingUndefined,
                    #[serde(rename = "MATURE")]
                    #[doc = "Show books which are rated mature or lower."]
                    Mature,
                    #[serde(rename = "not-mature")]
                    #[doc = "Show books which are rated not mature."]
                    NotMature,
                }
                impl ::std::default::Default for QueryParametersMaxAllowedMaturityRatingEnum {
                    fn default() -> Self {
                        Self::MaxAllowedMaturityRatingUndefined
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Sort search results."]
                pub enum QueryParametersOrderByEnum {
                    #[serde(rename = "ORDER_BY_UNDEFINED")]
                    #[doc = ""]
                    OrderByUndefined,
                    #[serde(rename = "newest")]
                    #[doc = "Most recently published."]
                    Newest,
                    #[serde(rename = "relevance")]
                    #[doc = "Relevance to search terms."]
                    Relevance,
                }
                impl ::std::default::Default for QueryParametersOrderByEnum {
                    fn default() -> Self {
                        Self::OrderByUndefined
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Restrict to books or magazines."]
                pub enum QueryParametersPrintTypeEnum {
                    #[serde(rename = "PRINT_TYPE_UNDEFINED")]
                    #[doc = ""]
                    PrintTypeUndefined,
                    #[serde(rename = "ALL")]
                    #[doc = "All volume content types."]
                    All,
                    #[serde(rename = "BOOKS")]
                    #[doc = "Just books."]
                    Books,
                    #[serde(rename = "MAGAZINES")]
                    #[doc = "Just magazines."]
                    Magazines,
                }
                impl ::std::default::Default for QueryParametersPrintTypeEnum {
                    fn default() -> Self {
                        Self::PrintTypeUndefined
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Restrict information returned to a set of selected fields."]
                pub enum QueryParametersProjectionEnum {
                    #[serde(rename = "PROJECTION_UNDEFINED")]
                    #[doc = ""]
                    ProjectionUndefined,
                    #[serde(rename = "FULL")]
                    #[doc = "Includes all volume data."]
                    Full,
                    #[serde(rename = "LITE")]
                    #[doc = "Includes a subset of fields in volumeInfo and accessInfo."]
                    Lite,
                }
                impl ::std::default::Default for QueryParametersProjectionEnum {
                    fn default() -> Self {
                        Self::ProjectionUndefined
                    }
                }
            }
        }
        pub mod resources {
            pub mod associated {
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
                            #[serde(rename = "association")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Association type."]
                            pub association: ::std::option::Option<QueryParametersAssociationEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "locale")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating recommendations."]
                            pub locale: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "maxAllowedMaturityRating")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum allowed maturity rating of returned recommendations. Books with a higher maturity rating are filtered out."]
                            pub max_allowed_maturity_rating:
                                ::std::option::Option<QueryParametersMaxAllowedMaturityRatingEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "source")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "String to identify the originator of this request."]
                            pub source: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Association type."]
                        pub enum QueryParametersAssociationEnum {
                            #[serde(rename = "ASSOCIATION_UNDEFINED")]
                            #[doc = ""]
                            AssociationUndefined,
                            #[serde(rename = "end-of-sample")]
                            #[doc = "Recommendations for display end-of-sample."]
                            EndOfSample,
                            #[serde(rename = "end-of-volume")]
                            #[doc = "Recommendations for display end-of-volume."]
                            EndOfVolume,
                            #[serde(rename = "related-for-play")]
                            #[doc = "Related volumes for Play Store."]
                            RelatedForPlay,
                        }
                        impl ::std::default::Default for QueryParametersAssociationEnum {
                            fn default() -> Self {
                                Self::AssociationUndefined
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "The maximum allowed maturity rating of returned recommendations. Books with a higher maturity rating are filtered out."]
                        pub enum QueryParametersMaxAllowedMaturityRatingEnum {
                            #[serde(rename = "MAX_ALLOWED_MATURITY_RATING_UNDEFINED")]
                            #[doc = ""]
                            MaxAllowedMaturityRatingUndefined,
                            #[serde(rename = "MATURE")]
                            #[doc = "Show books which are rated mature or lower."]
                            Mature,
                            #[serde(rename = "not-mature")]
                            #[doc = "Show books which are rated not mature."]
                            NotMature,
                        }
                        impl ::std::default::Default for QueryParametersMaxAllowedMaturityRatingEnum {
                            fn default() -> Self {
                                Self::MaxAllowedMaturityRatingUndefined
                            }
                        }
                    }
                }
            }
            pub mod mybooks {
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
                            #[serde(rename = "acquireMethod")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "How the book was acquired"]
                            pub acquire_method:
                                ::std::option::Option<QueryParametersAcquireMethodEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "country")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "ISO-3166-1 code to override the IP-based location."]
                            pub country: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "locale")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "ISO-639-1 language and ISO-3166-1 country code. Ex:'en_US'. Used for generating recommendations."]
                            pub locale: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "maxResults")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum number of results to return."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "processingState")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The processing state of the user uploaded volumes to be returned. Applicable only if the UPLOADED is specified in the acquireMethod."]
                            pub processing_state:
                                ::std::option::Option<QueryParametersProcessingStateEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "source")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "String to identify the originator of this request."]
                            pub source: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "startIndex")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Index of the first result to return (starts at 0)"]
                            pub start_index: ::std::option::Option<::std::primitive::i64>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "How the book was acquired"]
                        pub enum QueryParametersAcquireMethodEnum {
                            #[serde(rename = "ACQUIRE_METHOD_UNDEFINED")]
                            #[doc = ""]
                            AcquireMethodUndefined,
                            #[serde(rename = "FAMILY_SHARED")]
                            #[doc = "Books acquired via Family Sharing"]
                            FamilyShared,
                            #[serde(rename = "PREORDERED")]
                            #[doc = "Preordered books (not yet available)"]
                            Preordered,
                            #[serde(rename = "PREVIOUSLY_RENTED")]
                            #[doc = "User-rented books past their expiration time"]
                            PreviouslyRented,
                            #[serde(rename = "PUBLIC_DOMAIN")]
                            #[doc = "Public domain books"]
                            PublicDomain,
                            #[serde(rename = "PURCHASED")]
                            #[doc = "Purchased books"]
                            Purchased,
                            #[serde(rename = "RENTED")]
                            #[doc = "User-rented books"]
                            Rented,
                            #[serde(rename = "SAMPLE")]
                            #[doc = "Sample books"]
                            Sample,
                            #[serde(rename = "UPLOADED")]
                            #[doc = "User uploaded books"]
                            Uploaded,
                        }
                        impl ::std::default::Default for QueryParametersAcquireMethodEnum {
                            fn default() -> Self {
                                Self::AcquireMethodUndefined
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "The processing state of the user uploaded volumes to be returned. Applicable only if the UPLOADED is specified in the acquireMethod."]
                        pub enum QueryParametersProcessingStateEnum {
                            #[serde(rename = "PROCESSING_STATE_UNDEFINED")]
                            #[doc = ""]
                            ProcessingStateUndefined,
                            #[serde(rename = "COMPLETED_FAILED")]
                            #[doc = "The volume processing hase failed."]
                            CompletedFailed,
                            #[serde(rename = "COMPLETED_SUCCESS")]
                            #[doc = "The volume processing was completed."]
                            CompletedSuccess,
                            #[serde(rename = "RUNNING")]
                            #[doc = "The volume processing is not completed."]
                            Running,
                        }
                        impl ::std::default::Default for QueryParametersProcessingStateEnum {
                            fn default() -> Self {
                                Self::ProcessingStateUndefined
                            }
                        }
                    }
                }
            }
            pub mod recommended {
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
                            #[serde(rename = "locale")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating recommendations."]
                            pub locale: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "maxAllowedMaturityRating")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum allowed maturity rating of returned recommendations. Books with a higher maturity rating are filtered out."]
                            pub max_allowed_maturity_rating:
                                ::std::option::Option<QueryParametersMaxAllowedMaturityRatingEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "source")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "String to identify the originator of this request."]
                            pub source: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "The maximum allowed maturity rating of returned recommendations. Books with a higher maturity rating are filtered out."]
                        pub enum QueryParametersMaxAllowedMaturityRatingEnum {
                            #[serde(rename = "MAX_ALLOWED_MATURITY_RATING_UNDEFINED")]
                            #[doc = ""]
                            MaxAllowedMaturityRatingUndefined,
                            #[serde(rename = "MATURE")]
                            #[doc = "Show books which are rated mature or lower."]
                            Mature,
                            #[serde(rename = "not-mature")]
                            #[doc = "Show books which are rated not mature."]
                            NotMature,
                        }
                        impl ::std::default::Default for QueryParametersMaxAllowedMaturityRatingEnum {
                            fn default() -> Self {
                                Self::MaxAllowedMaturityRatingUndefined
                            }
                        }
                    }
                    pub mod rate {
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
                            #[serde(rename = "locale")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating recommendations."]
                            pub locale: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "rating")]
                            #[doc = "Rating to be given to the volume."]
                            pub rating: QueryParametersRatingEnum,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "source")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "String to identify the originator of this request."]
                            pub source: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "volumeId")]
                            #[doc = "ID of the source volume."]
                            pub volume_id: ::std::string::String,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Rating to be given to the volume."]
                        pub enum QueryParametersRatingEnum {
                            #[serde(rename = "RATING_UNDEFINED")]
                            #[doc = ""]
                            RatingUndefined,
                            #[serde(rename = "HAVE_IT")]
                            #[doc = "Rating indicating a dismissal due to ownership."]
                            HaveIt,
                            #[serde(rename = "NOT_INTERESTED")]
                            #[doc = "Rating indicating a negative dismissal of a volume."]
                            NotInterested,
                        }
                        impl ::std::default::Default for QueryParametersRatingEnum {
                            fn default() -> Self {
                                Self::RatingUndefined
                            }
                        }
                    }
                }
            }
            pub mod useruploaded {
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
                            #[serde(rename = "locale")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating recommendations."]
                            pub locale: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "maxResults")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum number of results to return."]
                            pub max_results: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "processingState")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The processing state of the user uploaded volumes to be returned."]
                            pub processing_state:
                                ::std::option::Option<QueryParametersProcessingStateEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "source")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "String to identify the originator of this request."]
                            pub source: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "startIndex")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Index of the first result to return (starts at 0)"]
                            pub start_index: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "volumeId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The ids of the volumes to be returned. If not specified all that match the processingState are returned."]
                            pub volume_id: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "The processing state of the user uploaded volumes to be returned."]
                        pub enum QueryParametersProcessingStateEnum {
                            #[serde(rename = "PROCESSING_STATE_UNDEFINED")]
                            #[doc = ""]
                            ProcessingStateUndefined,
                            #[serde(rename = "COMPLETED_FAILED")]
                            #[doc = "The volume processing hase failed."]
                            CompletedFailed,
                            #[serde(rename = "COMPLETED_SUCCESS")]
                            #[doc = "The volume processing was completed."]
                            CompletedSuccess,
                            #[serde(rename = "RUNNING")]
                            #[doc = "The volume processing is not completed."]
                            Running,
                        }
                        impl ::std::default::Default for QueryParametersProcessingStateEnum {
                            fn default() -> Self {
                                Self::ProcessingStateUndefined
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
    pub struct Annotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "afterSelectedText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Anchor text after excerpt. For requests, if the user bookmarked a screen that has no flowing text on it, then this field should be empty."]
        pub after_selected_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "beforeSelectedText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Anchor text before excerpt. For requests, if the user bookmarked a screen that has no flowing text on it, then this field should be empty."]
        pub before_selected_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientVersionRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Selection ranges sent from the client."]
        pub client_version_ranges: ::std::option::Option<AnnotationClientVersionRanges>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "created")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp for the created time of this annotation."]
        pub created: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentVersionRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Selection ranges for the most recent content version."]
        pub current_version_ranges: ::std::option::Option<AnnotationCurrentVersionRanges>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User-created data for this annotation."]
        pub data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates that this annotation is deleted."]
        pub deleted: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "highlightStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The highlight style for this annotation."]
        pub highlight_style: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Id of this annotation, in the form of a GUID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource type."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The layer this annotation is for."]
        pub layer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layerSummary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub layer_summary: ::std::option::Option<AnnotationLayerSummary>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pages that this annotation spans."]
        pub page_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selectedText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Excerpt from the volume."]
        pub selected_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL to this resource."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp for the last time this annotation was modified."]
        pub updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The volume that this annotation belongs to."]
        pub volume_id: ::std::option::Option<::std::string::String>,
    }
    impl Annotation {
        pub fn builder() -> AnnotationBuilder {
            AnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Selection ranges sent from the client."]
    pub struct AnnotationClientVersionRanges {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cfiRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Range in CFI format for this annotation sent by client."]
        pub cfi_range: ::std::option::Option<::std::boxed::Box<BooksAnnotationsRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Content version the client sent in."]
        pub content_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gbImageRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Range in GB image format for this annotation sent by client."]
        pub gb_image_range: ::std::option::Option<::std::boxed::Box<BooksAnnotationsRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gbTextRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Range in GB text format for this annotation sent by client."]
        pub gb_text_range: ::std::option::Option<::std::boxed::Box<BooksAnnotationsRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageCfiRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Range in image CFI format for this annotation sent by client."]
        pub image_cfi_range: ::std::option::Option<::std::boxed::Box<BooksAnnotationsRange>>,
    }
    impl AnnotationClientVersionRanges {
        pub fn builder() -> AnnotationClientVersionRangesBuilder {
            AnnotationClientVersionRangesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Selection ranges for the most recent content version."]
    pub struct AnnotationCurrentVersionRanges {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cfiRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Range in CFI format for this annotation for version above."]
        pub cfi_range: ::std::option::Option<::std::boxed::Box<BooksAnnotationsRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Content version applicable to ranges below."]
        pub content_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gbImageRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Range in GB image format for this annotation for version above."]
        pub gb_image_range: ::std::option::Option<::std::boxed::Box<BooksAnnotationsRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gbTextRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Range in GB text format for this annotation for version above."]
        pub gb_text_range: ::std::option::Option<::std::boxed::Box<BooksAnnotationsRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageCfiRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Range in image CFI format for this annotation for version above."]
        pub image_cfi_range: ::std::option::Option<::std::boxed::Box<BooksAnnotationsRange>>,
    }
    impl AnnotationCurrentVersionRanges {
        pub fn builder() -> AnnotationCurrentVersionRangesBuilder {
            AnnotationCurrentVersionRangesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AnnotationLayerSummary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowedCharacterCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum allowed characters on this layer, especially for the \"copy\" layer."]
        pub allowed_character_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "limitType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of limitation on this layer. \"limited\" or \"unlimited\" for the \"copy\" layer."]
        pub limit_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "remainingCharacterCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Remaining allowed characters on this layer, especially for the \"copy\" layer."]
        pub remaining_character_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl AnnotationLayerSummary {
        pub fn builder() -> AnnotationLayerSummaryBuilder {
            AnnotationLayerSummaryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Annotations {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of annotations."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Annotation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource type."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to pass in for pagination for the next page. This will not be present if this request does not have more results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalItems")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of annotations found. This may be greater than the number of notes returned in this response if results have been paginated."]
        pub total_items: ::std::option::Option<::std::primitive::i64>,
    }
    impl Annotations {
        pub fn builder() -> AnnotationsBuilder {
            AnnotationsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AnnotationsSummary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub layers: ::std::option::Option<::std::vec::Vec<AnnotationsSummaryLayers>>,
    }
    impl AnnotationsSummary {
        pub fn builder() -> AnnotationsSummaryBuilder {
            AnnotationsSummaryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AnnotationsSummaryLayers {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowedCharacterCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub allowed_character_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub layer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "limitType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub limit_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "remainingCharacterCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub remaining_character_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub updated: ::std::option::Option<::std::string::String>,
    }
    impl AnnotationsSummaryLayers {
        pub fn builder() -> AnnotationsSummaryLayersBuilder {
            AnnotationsSummaryLayersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Annotationsdata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of Annotation Data."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GeoAnnotationdata>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource type"]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to pass in for pagination for the next page. This will not be present if this request does not have more results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalItems")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of volume annotations found."]
        pub total_items: ::std::option::Option<::std::primitive::i64>,
    }
    impl Annotationsdata {
        pub fn builder() -> AnnotationsdataBuilder {
            AnnotationsdataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct BooksAnnotationsRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The offset from the ending position."]
        pub end_offset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endPosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ending position for the range."]
        pub end_position: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startOffset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The offset from the starting position."]
        pub start_offset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startPosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The starting position for the range."]
        pub start_position: ::std::option::Option<::std::string::String>,
    }
    impl BooksAnnotationsRange {
        pub fn builder() -> BooksAnnotationsRangeBuilder {
            BooksAnnotationsRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct BooksCloudloadingResource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "author")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub author: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "processingState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub processing_state: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub volume_id: ::std::option::Option<::std::string::String>,
    }
    impl BooksCloudloadingResource {
        pub fn builder() -> BooksCloudloadingResourceBuilder {
            BooksCloudloadingResourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct BooksVolumesRecommendedRateResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "consistency_token")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub consistency_token: ::std::option::Option<::std::string::String>,
    }
    impl BooksVolumesRecommendedRateResponse {
        pub fn builder() -> BooksVolumesRecommendedRateResponseBuilder {
            BooksVolumesRecommendedRateResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Bookshelf {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "access")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this bookshelf is PUBLIC or PRIVATE."]
        pub access: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "created")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Created time for this bookshelf (formatted UTC timestamp with millisecond resolution)."]
        pub created: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of this bookshelf."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Id of this bookshelf, only unique by user."]
        pub id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource type for bookshelf metadata."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL to this resource."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title of this bookshelf."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Last modified time of this bookshelf (formatted UTC timestamp with millisecond resolution)."]
        pub updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumeCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of volumes in this bookshelf."]
        pub volume_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumesLastUpdated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Last time a volume was added or removed from this bookshelf (formatted UTC timestamp with millisecond resolution)."]
        pub volumes_last_updated: ::std::option::Option<::std::string::String>,
    }
    impl Bookshelf {
        pub fn builder() -> BookshelfBuilder {
            BookshelfBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Bookshelves {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of bookshelves."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Bookshelf>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource type."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl Bookshelves {
        pub fn builder() -> BookshelvesBuilder {
            BookshelvesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Category {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of onboarding categories."]
        pub items: ::std::option::Option<::std::vec::Vec<CategoryItems>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource type."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl Category {
        pub fn builder() -> CategoryBuilder {
            CategoryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct CategoryItems {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "badgeUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub badge_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "categoryId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub category_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl CategoryItems {
        pub fn builder() -> CategoryItemsBuilder {
            CategoryItemsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ConcurrentAccessRestriction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceAllowed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether access is granted for this (user, device, volume)."]
        pub device_allowed: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource type."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxConcurrentDevices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of concurrent access licenses for this volume."]
        pub max_concurrent_devices: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Error/warning message."]
        pub message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nonce")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Client nonce for verification. Download access and client-validation only."]
        pub nonce: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reasonCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Error/warning reason code."]
        pub reason_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "restricted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this volume has any concurrent access restrictions."]
        pub restricted: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "signature")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Response signature."]
        pub signature: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Client app identifier for verification. Download access and client-validation only."]
        pub source: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeWindowSeconds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time in seconds for license auto-expiration."]
        pub time_window_seconds: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies the volume for which this entry applies."]
        pub volume_id: ::std::option::Option<::std::string::String>,
    }
    impl ConcurrentAccessRestriction {
        pub fn builder() -> ConcurrentAccessRestrictionBuilder {
            ConcurrentAccessRestrictionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DictionaryAnnotationdata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of annotation this data is for."]
        pub annotation_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "JSON encoded data for this dictionary annotation data. Emitted with name 'data' in JSON output. Either this or geo_data will be populated."]
        pub data: ::std::option::Option<::std::boxed::Box<Dictlayerdata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "encodedData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Base64 encoded data for this annotation data."]
        pub encoded_data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique id for this annotation data."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource Type"]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Layer id for this data. *"]
        pub layer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL for this resource. *"]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp for the last time this data was updated. (RFC 3339 UTC date-time format)."]
        pub updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The volume id for this data. *"]
        pub volume_id: ::std::option::Option<::std::string::String>,
    }
    impl DictionaryAnnotationdata {
        pub fn builder() -> DictionaryAnnotationdataBuilder {
            DictionaryAnnotationdataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Dictlayerdata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "common")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub common: ::std::option::Option<DictlayerdataCommon>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dict")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub dict: ::std::option::Option<DictlayerdataDict>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl Dictlayerdata {
        pub fn builder() -> DictlayerdataBuilder {
            DictlayerdataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DictlayerdataCommon {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The display title and localized canonical name to use when searching for this entity on Google search."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl DictlayerdataCommon {
        pub fn builder() -> DictlayerdataCommonBuilder {
            DictlayerdataCommonBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DictlayerdataDict {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source, url and attribution for this dictionary data."]
        pub source: ::std::option::Option<DictlayerdataDictSource>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "words")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub words: ::std::option::Option<::std::vec::Vec<DictlayerdataDictWords>>,
    }
    impl DictlayerdataDict {
        pub fn builder() -> DictlayerdataDictBuilder {
            DictlayerdataDictBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The source, url and attribution for this dictionary data."]
    pub struct DictlayerdataDictSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attribution")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub attribution: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl DictlayerdataDictSource {
        pub fn builder() -> DictlayerdataDictSourceBuilder {
            DictlayerdataDictSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DictlayerdataDictWords {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "derivatives")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub derivatives: ::std::option::Option<::std::vec::Vec<DictlayerdataDictWordsDerivatives>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "examples")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub examples: ::std::option::Option<::std::vec::Vec<DictlayerdataDictWordsExamples>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "senses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub senses: ::std::option::Option<::std::vec::Vec<DictlayerdataDictWordsSenses>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The words with different meanings but not related words, e.g. \"go\" (game) and \"go\" (verb)."]
        pub source: ::std::option::Option<DictlayerdataDictWordsSource>,
    }
    impl DictlayerdataDictWords {
        pub fn builder() -> DictlayerdataDictWordsBuilder {
            DictlayerdataDictWordsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DictlayerdataDictWordsDerivatives {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub source: ::std::option::Option<DictlayerdataDictWordsDerivativesSource>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl DictlayerdataDictWordsDerivatives {
        pub fn builder() -> DictlayerdataDictWordsDerivativesBuilder {
            DictlayerdataDictWordsDerivativesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DictlayerdataDictWordsDerivativesSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attribution")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub attribution: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl DictlayerdataDictWordsDerivativesSource {
        pub fn builder() -> DictlayerdataDictWordsDerivativesSourceBuilder {
            DictlayerdataDictWordsDerivativesSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DictlayerdataDictWordsExamples {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub source: ::std::option::Option<DictlayerdataDictWordsExamplesSource>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl DictlayerdataDictWordsExamples {
        pub fn builder() -> DictlayerdataDictWordsExamplesBuilder {
            DictlayerdataDictWordsExamplesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DictlayerdataDictWordsExamplesSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attribution")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub attribution: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl DictlayerdataDictWordsExamplesSource {
        pub fn builder() -> DictlayerdataDictWordsExamplesSourceBuilder {
            DictlayerdataDictWordsExamplesSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DictlayerdataDictWordsSenses {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conjugations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub conjugations:
            ::std::option::Option<::std::vec::Vec<DictlayerdataDictWordsSensesConjugations>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "definitions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub definitions:
            ::std::option::Option<::std::vec::Vec<DictlayerdataDictWordsSensesDefinitions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partOfSpeech")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub part_of_speech: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pronunciation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub pronunciation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pronunciationUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub pronunciation_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub source: ::std::option::Option<DictlayerdataDictWordsSensesSource>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "syllabification")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub syllabification: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "synonyms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub synonyms: ::std::option::Option<::std::vec::Vec<DictlayerdataDictWordsSensesSynonyms>>,
    }
    impl DictlayerdataDictWordsSenses {
        pub fn builder() -> DictlayerdataDictWordsSensesBuilder {
            DictlayerdataDictWordsSensesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DictlayerdataDictWordsSensesConjugations {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl DictlayerdataDictWordsSensesConjugations {
        pub fn builder() -> DictlayerdataDictWordsSensesConjugationsBuilder {
            DictlayerdataDictWordsSensesConjugationsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DictlayerdataDictWordsSensesDefinitions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "definition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub definition: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "examples")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub examples:
            ::std::option::Option<::std::vec::Vec<DictlayerdataDictWordsSensesDefinitionsExamples>>,
    }
    impl DictlayerdataDictWordsSensesDefinitions {
        pub fn builder() -> DictlayerdataDictWordsSensesDefinitionsBuilder {
            DictlayerdataDictWordsSensesDefinitionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DictlayerdataDictWordsSensesDefinitionsExamples {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub source: ::std::option::Option<DictlayerdataDictWordsSensesDefinitionsExamplesSource>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl DictlayerdataDictWordsSensesDefinitionsExamples {
        pub fn builder() -> DictlayerdataDictWordsSensesDefinitionsExamplesBuilder {
            DictlayerdataDictWordsSensesDefinitionsExamplesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DictlayerdataDictWordsSensesDefinitionsExamplesSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attribution")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub attribution: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl DictlayerdataDictWordsSensesDefinitionsExamplesSource {
        pub fn builder() -> DictlayerdataDictWordsSensesDefinitionsExamplesSourceBuilder {
            DictlayerdataDictWordsSensesDefinitionsExamplesSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DictlayerdataDictWordsSensesSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attribution")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub attribution: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl DictlayerdataDictWordsSensesSource {
        pub fn builder() -> DictlayerdataDictWordsSensesSourceBuilder {
            DictlayerdataDictWordsSensesSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DictlayerdataDictWordsSensesSynonyms {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub source: ::std::option::Option<DictlayerdataDictWordsSensesSynonymsSource>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl DictlayerdataDictWordsSensesSynonyms {
        pub fn builder() -> DictlayerdataDictWordsSensesSynonymsBuilder {
            DictlayerdataDictWordsSensesSynonymsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DictlayerdataDictWordsSensesSynonymsSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attribution")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub attribution: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl DictlayerdataDictWordsSensesSynonymsSource {
        pub fn builder() -> DictlayerdataDictWordsSensesSynonymsSourceBuilder {
            DictlayerdataDictWordsSensesSynonymsSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The words with different meanings but not related words, e.g. \"go\" (game) and \"go\" (verb)."]
    pub struct DictlayerdataDictWordsSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attribution")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub attribution: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl DictlayerdataDictWordsSource {
        pub fn builder() -> DictlayerdataDictWordsSourceBuilder {
            DictlayerdataDictWordsSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Discoveryclusters {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clusters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub clusters: ::std::option::Option<::std::vec::Vec<DiscoveryclustersClusters>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resorce type."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalClusters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub total_clusters: ::std::option::Option<::std::primitive::i64>,
    }
    impl Discoveryclusters {
        pub fn builder() -> DiscoveryclustersBuilder {
            DiscoveryclustersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DiscoveryclustersClusters {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "banner_with_content_container")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub banner_with_content_container:
            ::std::option::Option<DiscoveryclustersClustersBannerWithContentContainer>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub sub_title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalVolumes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub total_volumes: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub uid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub volumes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Volume>>>,
    }
    impl DiscoveryclustersClusters {
        pub fn builder() -> DiscoveryclustersClustersBuilder {
            DiscoveryclustersClustersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DiscoveryclustersClustersBannerWithContentContainer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fillColorArgb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub fill_color_argb: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maskColorArgb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub mask_color_argb: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "moreButtonText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub more_button_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "moreButtonUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub more_button_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textColorArgb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub text_color_argb: ::std::option::Option<::std::string::String>,
    }
    impl DiscoveryclustersClustersBannerWithContentContainer {
        pub fn builder() -> DiscoveryclustersClustersBannerWithContentContainerBuilder {
            DiscoveryclustersClustersBannerWithContentContainerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DownloadAccessRestriction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceAllowed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If restricted, whether access is granted for this (user, device, volume)."]
        pub device_allowed: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "downloadsAcquired")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If restricted, the number of content download licenses already acquired (including the requesting client, if licensed)."]
        pub downloads_acquired: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "justAcquired")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If deviceAllowed, whether access was just acquired with this request."]
        pub just_acquired: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource type."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxDownloadDevices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If restricted, the maximum number of content download licenses for this volume."]
        pub max_download_devices: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Error/warning message."]
        pub message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nonce")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Client nonce for verification. Download access and client-validation only."]
        pub nonce: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reasonCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Error/warning reason code. Additional codes may be added in the future. 0 OK 100 ACCESS_DENIED_PUBLISHER_LIMIT 101 ACCESS_DENIED_LIMIT 200 WARNING_USED_LAST_ACCESS"]
        pub reason_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "restricted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this volume has any download access restrictions."]
        pub restricted: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "signature")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Response signature."]
        pub signature: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Client app identifier for verification. Download access and client-validation only."]
        pub source: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies the volume for which this entry applies."]
        pub volume_id: ::std::option::Option<::std::string::String>,
    }
    impl DownloadAccessRestriction {
        pub fn builder() -> DownloadAccessRestrictionBuilder {
            DownloadAccessRestrictionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DownloadAccesses {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "downloadAccessList")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of download access responses."]
        pub download_access_list:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DownloadAccessRestriction>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource type."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl DownloadAccesses {
        pub fn builder() -> DownloadAccessesBuilder {
            DownloadAccessesBuilder::default()
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
    pub struct FamilyInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource type."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "membership")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Family membership info of the user that made the request."]
        pub membership: ::std::option::Option<FamilyInfoMembership>,
    }
    impl FamilyInfo {
        pub fn builder() -> FamilyInfoBuilder {
            FamilyInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Family membership info of the user that made the request."]
    pub struct FamilyInfoMembership {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "acquirePermission")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restrictions on user buying and acquiring content."]
        pub acquire_permission: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ageGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The age group of the user."]
        pub age_group: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowedMaturityRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum allowed maturity rating for the user."]
        pub allowed_maturity_rating: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isInFamily")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub is_in_family: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The role of the user in the family."]
        pub role: ::std::option::Option<::std::string::String>,
    }
    impl FamilyInfoMembership {
        pub fn builder() -> FamilyInfoMembershipBuilder {
            FamilyInfoMembershipBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GeoAnnotationdata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of annotation this data is for."]
        pub annotation_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "JSON encoded data for this geo annotation data. Emitted with name 'data' in JSON output. Either this or dict_data will be populated."]
        pub data: ::std::option::Option<::std::boxed::Box<Geolayerdata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "encodedData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Base64 encoded data for this annotation data."]
        pub encoded_data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique id for this annotation data."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource Type"]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Layer id for this data. *"]
        pub layer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL for this resource. *"]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp for the last time this data was updated. (RFC 3339 UTC date-time format)."]
        pub updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The volume id for this data. *"]
        pub volume_id: ::std::option::Option<::std::string::String>,
    }
    impl GeoAnnotationdata {
        pub fn builder() -> GeoAnnotationdataBuilder {
            GeoAnnotationdataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Geolayerdata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "common")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub common: ::std::option::Option<GeolayerdataCommon>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "geo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub geo: ::std::option::Option<GeolayerdataGeo>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl Geolayerdata {
        pub fn builder() -> GeolayerdataBuilder {
            GeolayerdataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GeolayerdataCommon {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lang")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language of the information url and description."]
        pub lang: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previewImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL for the preview image information."]
        pub preview_image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The description for this location."]
        pub snippet: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippetUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL for information for this location. Ex: wikipedia link."]
        pub snippet_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The display title and localized canonical name to use when searching for this entity on Google search."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl GeolayerdataCommon {
        pub fn builder() -> GeolayerdataCommonBuilder {
            GeolayerdataCommonBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GeolayerdataGeo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The boundary of the location as a set of loops containing pairs of latitude, longitude coordinates."]
        pub boundary: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cachePolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cache policy active for this data. EX: UNRESTRICTED, RESTRICTED, NEVER"]
        pub cache_policy: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "countryCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The country code of the location."]
        pub country_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "latitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The latitude of the location."]
        pub latitude: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "longitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The longitude of the location."]
        pub longitude: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mapType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of map that should be used for this location. EX: HYBRID, ROADMAP, SATELLITE, TERRAIN"]
        pub map_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "viewport")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The viewport for showing this location. This is a latitude, longitude rectangle."]
        pub viewport: ::std::option::Option<GeolayerdataGeoViewport>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zoom")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Zoom level to use for the map. Zoom levels between 0 (the lowest zoom level, in which the entire world can be seen on one map) to 21+ (down to individual buildings). See: https: //developers.google.com/maps/documentation/staticmaps/#Zoomlevels"]
        pub zoom: ::std::option::Option<::std::primitive::i64>,
    }
    impl GeolayerdataGeo {
        pub fn builder() -> GeolayerdataGeoBuilder {
            GeolayerdataGeoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The viewport for showing this location. This is a latitude, longitude rectangle."]
    pub struct GeolayerdataGeoViewport {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hi")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub hi: ::std::option::Option<GeolayerdataGeoViewportHi>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub lo: ::std::option::Option<GeolayerdataGeoViewportLo>,
    }
    impl GeolayerdataGeoViewport {
        pub fn builder() -> GeolayerdataGeoViewportBuilder {
            GeolayerdataGeoViewportBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GeolayerdataGeoViewportHi {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "latitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub latitude: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "longitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub longitude: ::std::option::Option<::std::primitive::f64>,
    }
    impl GeolayerdataGeoViewportHi {
        pub fn builder() -> GeolayerdataGeoViewportHiBuilder {
            GeolayerdataGeoViewportHiBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GeolayerdataGeoViewportLo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "latitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub latitude: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "longitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub longitude: ::std::option::Option<::std::primitive::f64>,
    }
    impl GeolayerdataGeoViewportLo {
        pub fn builder() -> GeolayerdataGeoViewportLoBuilder {
            GeolayerdataGeoViewportLoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Layersummaries {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of layer summary items."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Layersummary>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource type."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalItems")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of layer summaries found."]
        pub total_items: ::std::option::Option<::std::primitive::i64>,
    }
    impl Layersummaries {
        pub fn builder() -> LayersummariesBuilder {
            LayersummariesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Layersummary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of annotations for this layer."]
        pub annotation_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of annotation types contained for this layer."]
        pub annotation_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationsDataLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to get data for this annotation."]
        pub annotations_data_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationsLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The link to get the annotations for this layer."]
        pub annotations_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The content version this resource is for."]
        pub content_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of data items for this layer."]
        pub data_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique id of this layer summary."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource Type"]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The layer id for this summary."]
        pub layer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL to this resource."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp for the last time an item in this layer was updated. (RFC 3339 UTC date-time format)."]
        pub updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumeAnnotationsVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current version of this layer's volume annotations. Note that this version applies only to the data in the books.layers.volumeAnnotations.* responses. The actual annotation data is versioned separately."]
        pub volume_annotations_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The volume id this resource is for."]
        pub volume_id: ::std::option::Option<::std::string::String>,
    }
    impl Layersummary {
        pub fn builder() -> LayersummaryBuilder {
            LayersummaryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Metadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of offline dictionary metadata."]
        pub items: ::std::option::Option<::std::vec::Vec<MetadataItems>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource type."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl Metadata {
        pub fn builder() -> MetadataBuilder {
            MetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct MetadataItems {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "download_url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub download_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "encrypted_key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub encrypted_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "language")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl MetadataItems {
        pub fn builder() -> MetadataItemsBuilder {
            MetadataItemsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Notification {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "body")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub body: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "crmExperimentIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of crm experiment ids."]
        pub crm_experiment_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "doc_id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub doc_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "doc_type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub doc_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dont_show_notification")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub dont_show_notification: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iconUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub icon_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "is_document_mature")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub is_document_mature: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource type."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notificationGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub notification_group: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notification_type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub notification_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pcampaign_id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub pcampaign_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub reason: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "show_notification_settings_action")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub show_notification_settings_action: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub target_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeToExpireMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub time_to_expire_ms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl Notification {
        pub fn builder() -> NotificationBuilder {
            NotificationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Offers {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of offers."]
        pub items: ::std::option::Option<::std::vec::Vec<OffersItems>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource type."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl Offers {
        pub fn builder() -> OffersBuilder {
            OffersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OffersItems {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "artUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub art_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gservicesKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub gservices_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub items: ::std::option::Option<::std::vec::Vec<OffersItemsItems>>,
    }
    impl OffersItems {
        pub fn builder() -> OffersItemsBuilder {
            OffersItemsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OffersItemsItems {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "author")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub author: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canonicalVolumeLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub canonical_volume_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "coverUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub cover_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub volume_id: ::std::option::Option<::std::string::String>,
    }
    impl OffersItemsItems {
        pub fn builder() -> OffersItemsItemsBuilder {
            OffersItemsItemsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ReadingPosition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "epubCfiPosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Position in an EPUB as a CFI."]
        pub epub_cfi_position: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gbImagePosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Position in a volume for image-based content."]
        pub gb_image_position: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gbTextPosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Position in a volume for text-based content."]
        pub gb_text_position: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource type for a reading position."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pdfPosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Position in a PDF file."]
        pub pdf_position: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp when this reading position was last updated (formatted UTC timestamp with millisecond resolution)."]
        pub updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Volume id associated with this reading position."]
        pub volume_id: ::std::option::Option<::std::string::String>,
    }
    impl ReadingPosition {
        pub fn builder() -> ReadingPositionBuilder {
            ReadingPositionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct RequestAccessData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "concurrentAccess")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A concurrent access response."]
        pub concurrent_access:
            ::std::option::Option<::std::boxed::Box<ConcurrentAccessRestriction>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "downloadAccess")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A download access response."]
        pub download_access: ::std::option::Option<::std::boxed::Box<DownloadAccessRestriction>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource type."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl RequestAccessData {
        pub fn builder() -> RequestAccessDataBuilder {
            RequestAccessDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Review {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "author")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Author of this review."]
        pub author: ::std::option::Option<ReviewAuthor>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Review text."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "date")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date of this review."]
        pub date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullTextUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL for the full review text, for reviews gathered from the web."]
        pub full_text_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource type for a review."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Star rating for this review. Possible values are ONE, TWO, THREE, FOUR, FIVE or NOT_RATED."]
        pub rating: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information regarding the source of this review, when the review is not from a Google Books user."]
        pub source: ::std::option::Option<ReviewSource>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title for this review."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Source type for this review. Possible values are EDITORIAL, WEB_USER or GOOGLE_USER."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Volume that this review is for."]
        pub volume_id: ::std::option::Option<::std::string::String>,
    }
    impl Review {
        pub fn builder() -> ReviewBuilder {
            ReviewBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Author of this review."]
    pub struct ReviewAuthor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of this person."]
        pub display_name: ::std::option::Option<::std::string::String>,
    }
    impl ReviewAuthor {
        pub fn builder() -> ReviewAuthorBuilder {
            ReviewAuthorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information regarding the source of this review, when the review is not from a Google Books user."]
    pub struct ReviewSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the source."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "extraDescription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Extra text about the source of the review."]
        pub extra_description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL of the source of the review."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl ReviewSource {
        pub fn builder() -> ReviewSourceBuilder {
            ReviewSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Series {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource type."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "series")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub series: ::std::option::Option<::std::vec::Vec<SeriesSeries>>,
    }
    impl Series {
        pub fn builder() -> SeriesBuilder {
            SeriesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct SeriesSeries {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bannerImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub banner_image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eligibleForSubscription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub eligible_for_subscription: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isComplete")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub is_complete: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "seriesFormatType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub series_format_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "seriesId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub series_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "seriesSubscriptionReleaseInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub series_subscription_release_info:
            ::std::option::Option<SeriesSeriesSeriesSubscriptionReleaseInfo>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "seriesType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub series_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subscriptionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub subscription_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl SeriesSeries {
        pub fn builder() -> SeriesSeriesBuilder {
            SeriesSeriesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct SeriesSeriesSeriesSubscriptionReleaseInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cancelTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub cancel_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentReleaseInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub current_release_info:
            ::std::option::Option<SeriesSeriesSeriesSubscriptionReleaseInfoCurrentReleaseInfo>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextReleaseInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub next_release_info:
            ::std::option::Option<SeriesSeriesSeriesSubscriptionReleaseInfoNextReleaseInfo>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "seriesSubscriptionType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub series_subscription_type: ::std::option::Option<::std::string::String>,
    }
    impl SeriesSeriesSeriesSubscriptionReleaseInfo {
        pub fn builder() -> SeriesSeriesSeriesSubscriptionReleaseInfoBuilder {
            SeriesSeriesSeriesSubscriptionReleaseInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct SeriesSeriesSeriesSubscriptionReleaseInfoCurrentReleaseInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountInMicros")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub amount_in_micros: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currencyCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub currency_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "releaseNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub release_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "releaseTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub release_time: ::std::option::Option<::std::string::String>,
    }
    impl SeriesSeriesSeriesSubscriptionReleaseInfoCurrentReleaseInfo {
        pub fn builder() -> SeriesSeriesSeriesSubscriptionReleaseInfoCurrentReleaseInfoBuilder {
            SeriesSeriesSeriesSubscriptionReleaseInfoCurrentReleaseInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct SeriesSeriesSeriesSubscriptionReleaseInfoNextReleaseInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountInMicros")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub amount_in_micros: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currencyCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub currency_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "releaseNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub release_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "releaseTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub release_time: ::std::option::Option<::std::string::String>,
    }
    impl SeriesSeriesSeriesSubscriptionReleaseInfoNextReleaseInfo {
        pub fn builder() -> SeriesSeriesSeriesSubscriptionReleaseInfoNextReleaseInfoBuilder {
            SeriesSeriesSeriesSubscriptionReleaseInfoNextReleaseInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Seriesmembership {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resorce type."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "member")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub member: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Volume>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl Seriesmembership {
        pub fn builder() -> SeriesmembershipBuilder {
            SeriesmembershipBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Usersettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource type."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notesExport")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User settings in sub-objects, each for different purposes."]
        pub notes_export: ::std::option::Option<UsersettingsNotesExport>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notification")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub notification: ::std::option::Option<UsersettingsNotification>,
    }
    impl Usersettings {
        pub fn builder() -> UsersettingsBuilder {
            UsersettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "User settings in sub-objects, each for different purposes."]
    pub struct UsersettingsNotesExport {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "folderName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub folder_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub is_enabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl UsersettingsNotesExport {
        pub fn builder() -> UsersettingsNotesExportBuilder {
            UsersettingsNotesExportBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct UsersettingsNotification {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchMyInterests")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub match_my_interests: ::std::option::Option<UsersettingsNotificationMatchMyInterests>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "moreFromAuthors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub more_from_authors: ::std::option::Option<UsersettingsNotificationMoreFromAuthors>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "moreFromSeries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub more_from_series: ::std::option::Option<UsersettingsNotificationMoreFromSeries>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "priceDrop")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub price_drop: ::std::option::Option<UsersettingsNotificationPriceDrop>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rewardExpirations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub reward_expirations: ::std::option::Option<UsersettingsNotificationRewardExpirations>,
    }
    impl UsersettingsNotification {
        pub fn builder() -> UsersettingsNotificationBuilder {
            UsersettingsNotificationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct UsersettingsNotificationMatchMyInterests {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "opted_state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub opted_state: ::std::option::Option<::std::string::String>,
    }
    impl UsersettingsNotificationMatchMyInterests {
        pub fn builder() -> UsersettingsNotificationMatchMyInterestsBuilder {
            UsersettingsNotificationMatchMyInterestsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct UsersettingsNotificationMoreFromAuthors {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "opted_state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub opted_state: ::std::option::Option<::std::string::String>,
    }
    impl UsersettingsNotificationMoreFromAuthors {
        pub fn builder() -> UsersettingsNotificationMoreFromAuthorsBuilder {
            UsersettingsNotificationMoreFromAuthorsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct UsersettingsNotificationMoreFromSeries {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "opted_state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub opted_state: ::std::option::Option<::std::string::String>,
    }
    impl UsersettingsNotificationMoreFromSeries {
        pub fn builder() -> UsersettingsNotificationMoreFromSeriesBuilder {
            UsersettingsNotificationMoreFromSeriesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct UsersettingsNotificationPriceDrop {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "opted_state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub opted_state: ::std::option::Option<::std::string::String>,
    }
    impl UsersettingsNotificationPriceDrop {
        pub fn builder() -> UsersettingsNotificationPriceDropBuilder {
            UsersettingsNotificationPriceDropBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct UsersettingsNotificationRewardExpirations {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "opted_state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub opted_state: ::std::option::Option<::std::string::String>,
    }
    impl UsersettingsNotificationRewardExpirations {
        pub fn builder() -> UsersettingsNotificationRewardExpirationsBuilder {
            UsersettingsNotificationRewardExpirationsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Volume {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accessInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Any information about a volume related to reading or obtaining that volume text. This information can depend on country (books may be public domain in one country but not in another, e.g.)."]
        pub access_info: ::std::option::Option<VolumeAccessInfo>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Opaque identifier for a specific version of a volume resource. (In LITE projection)"]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier for a volume. (In LITE projection.)"]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource type for a volume. (In LITE projection.)"]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layerInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "What layers exist in this volume and high level information about them."]
        pub layer_info: ::std::option::Option<VolumeLayerInfo>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recommendedInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Recommendation related information for this volume."]
        pub recommended_info: ::std::option::Option<VolumeRecommendedInfo>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "saleInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Any information about a volume related to the eBookstore and/or purchaseability. This information can depend on the country where the request originates from (i.e. books may not be for sale in certain countries)."]
        pub sale_info: ::std::option::Option<VolumeSaleInfo>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "searchInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Search result information related to this volume."]
        pub search_info: ::std::option::Option<VolumeSearchInfo>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL to this resource. (In LITE projection.)"]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User specific information related to this volume. (e.g. page this user last read or whether they purchased this book)"]
        pub user_info: ::std::option::Option<VolumeUserInfo>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumeInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "General volume information."]
        pub volume_info: ::std::option::Option<VolumeVolumeInfo>,
    }
    impl Volume {
        pub fn builder() -> VolumeBuilder {
            VolumeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Any information about a volume related to reading or obtaining that volume text. This information can depend on country (books may be public domain in one country but not in another, e.g.)."]
    pub struct VolumeAccessInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accessViewStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Combines the access and viewability of this volume into a single status field for this user. Values can be FULL_PURCHASED, FULL_PUBLIC_DOMAIN, SAMPLE or NONE. (In LITE projection.)"]
        pub access_view_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "country")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The two-letter ISO_3166-1 country code for which this access information is valid. (In LITE projection.)"]
        pub country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "downloadAccess")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about a volume's download license access restrictions."]
        pub download_access: ::std::option::Option<::std::boxed::Box<DownloadAccessRestriction>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "driveImportedContentLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL to the Google Drive viewer if this volume is uploaded by the user by selecting the file from Google Drive."]
        pub drive_imported_content_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "embeddable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this volume can be embedded in a viewport using the Embedded Viewer API."]
        pub embeddable: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "epub")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about epub content. (In LITE projection.)"]
        pub epub: ::std::option::Option<VolumeAccessInfoEpub>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "explicitOfflineLicenseManagement")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this volume requires that the client explicitly request offline download license rather than have it done automatically when loading the content, if the client supports it."]
        pub explicit_offline_license_management: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pdf")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about pdf content. (In LITE projection.)"]
        pub pdf: ::std::option::Option<VolumeAccessInfoPdf>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publicDomain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not this book is public domain in the country listed above."]
        pub public_domain: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quoteSharingAllowed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether quote sharing is allowed for this volume."]
        pub quote_sharing_allowed: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textToSpeechPermission")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether text-to-speech is permitted for this volume. Values can be ALLOWED, ALLOWED_FOR_ACCESSIBILITY, or NOT_ALLOWED."]
        pub text_to_speech_permission: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "viewOrderUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For ordered but not yet processed orders, we give a URL that can be used to go to the appropriate Google Wallet page."]
        pub view_order_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "viewability")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The read access of a volume. Possible values are PARTIAL, ALL_PAGES, NO_PAGES or UNKNOWN. This value depends on the country listed above. A value of PARTIAL means that the publisher has allowed some portion of the volume to be viewed publicly, without purchase. This can apply to eBooks as well as non-eBooks. Public domain books will always have a value of ALL_PAGES."]
        pub viewability: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webReaderLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL to read this volume on the Google Books site. Link will not allow users to read non-viewable volumes."]
        pub web_reader_link: ::std::option::Option<::std::string::String>,
    }
    impl VolumeAccessInfo {
        pub fn builder() -> VolumeAccessInfoBuilder {
            VolumeAccessInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about epub content. (In LITE projection.)"]
    pub struct VolumeAccessInfoEpub {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "acsTokenLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL to retrieve ACS token for epub download. (In LITE projection.)"]
        pub acs_token_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "downloadLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL to download epub. (In LITE projection.)"]
        pub download_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isAvailable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Is a flowing text epub available either as public domain or for purchase. (In LITE projection.)"]
        pub is_available: ::std::option::Option<::std::primitive::bool>,
    }
    impl VolumeAccessInfoEpub {
        pub fn builder() -> VolumeAccessInfoEpubBuilder {
            VolumeAccessInfoEpubBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about pdf content. (In LITE projection.)"]
    pub struct VolumeAccessInfoPdf {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "acsTokenLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL to retrieve ACS token for pdf download. (In LITE projection.)"]
        pub acs_token_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "downloadLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL to download pdf. (In LITE projection.)"]
        pub download_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isAvailable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Is a scanned image pdf available either as public domain or for purchase. (In LITE projection.)"]
        pub is_available: ::std::option::Option<::std::primitive::bool>,
    }
    impl VolumeAccessInfoPdf {
        pub fn builder() -> VolumeAccessInfoPdfBuilder {
            VolumeAccessInfoPdfBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "What layers exist in this volume and high level information about them."]
    pub struct VolumeLayerInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A layer should appear here if and only if the layer exists for this book."]
        pub layers: ::std::option::Option<::std::vec::Vec<VolumeLayerInfoLayers>>,
    }
    impl VolumeLayerInfo {
        pub fn builder() -> VolumeLayerInfoBuilder {
            VolumeLayerInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct VolumeLayerInfoLayers {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The layer id of this layer (e.g. \"geo\")."]
        pub layer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumeAnnotationsVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current version of this layer's volume annotations. Note that this version applies only to the data in the books.layers.volumeAnnotations.* responses. The actual annotation data is versioned separately."]
        pub volume_annotations_version: ::std::option::Option<::std::string::String>,
    }
    impl VolumeLayerInfoLayers {
        pub fn builder() -> VolumeLayerInfoLayersBuilder {
            VolumeLayerInfoLayersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Recommendation related information for this volume."]
    pub struct VolumeRecommendedInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "explanation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A text explaining why this volume is recommended."]
        pub explanation: ::std::option::Option<::std::string::String>,
    }
    impl VolumeRecommendedInfo {
        pub fn builder() -> VolumeRecommendedInfoBuilder {
            VolumeRecommendedInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Any information about a volume related to the eBookstore and/or purchaseability. This information can depend on the country where the request originates from (i.e. books may not be for sale in certain countries)."]
    pub struct VolumeSaleInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buyLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL to purchase this volume on the Google Books site. (In LITE projection)"]
        pub buy_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "country")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The two-letter ISO_3166-1 country code for which this sale information is valid. (In LITE projection.)"]
        pub country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isEbook")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not this volume is an eBook (can be added to the My eBooks shelf)."]
        pub is_ebook: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "listPrice")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Suggested retail price. (In LITE projection.)"]
        pub list_price: ::std::option::Option<VolumeSaleInfoListPrice>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Offers available for this volume (sales and rentals)."]
        pub offers: ::std::option::Option<::std::vec::Vec<VolumeSaleInfoOffers>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "onSaleDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date on which this book is available for sale."]
        pub on_sale_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "retailPrice")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The actual selling price of the book. This is the same as the suggested retail or list price unless there are offers or discounts on this volume. (In LITE projection.)"]
        pub retail_price: ::std::option::Option<VolumeSaleInfoRetailPrice>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "saleability")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not this book is available for sale or offered for free in the Google eBookstore for the country listed above. Possible values are FOR_SALE, FOR_RENTAL_ONLY, FOR_SALE_AND_RENTAL, FREE, NOT_FOR_SALE, or FOR_PREORDER."]
        pub saleability: ::std::option::Option<::std::string::String>,
    }
    impl VolumeSaleInfo {
        pub fn builder() -> VolumeSaleInfoBuilder {
            VolumeSaleInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Suggested retail price. (In LITE projection.)"]
    pub struct VolumeSaleInfoListPrice {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Amount in the currency listed below. (In LITE projection.)"]
        pub amount: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currencyCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An ISO 4217, three-letter currency code. (In LITE projection.)"]
        pub currency_code: ::std::option::Option<::std::string::String>,
    }
    impl VolumeSaleInfoListPrice {
        pub fn builder() -> VolumeSaleInfoListPriceBuilder {
            VolumeSaleInfoListPriceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct VolumeSaleInfoOffers {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "finskyOfferType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The finsky offer type (e.g., PURCHASE=0 RENTAL=3)"]
        pub finsky_offer_type: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "giftable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the offer is giftable."]
        pub giftable: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "listPrice")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Offer list (=undiscounted) price in Micros."]
        pub list_price: ::std::option::Option<VolumeSaleInfoOffersListPrice>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rentalDuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The rental duration (for rental offers only)."]
        pub rental_duration: ::std::option::Option<VolumeSaleInfoOffersRentalDuration>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "retailPrice")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Offer retail (=discounted) price in Micros"]
        pub retail_price: ::std::option::Option<VolumeSaleInfoOffersRetailPrice>,
    }
    impl VolumeSaleInfoOffers {
        pub fn builder() -> VolumeSaleInfoOffersBuilder {
            VolumeSaleInfoOffersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Offer list (=undiscounted) price in Micros."]
    pub struct VolumeSaleInfoOffersListPrice {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountInMicros")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub amount_in_micros: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currencyCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub currency_code: ::std::option::Option<::std::string::String>,
    }
    impl VolumeSaleInfoOffersListPrice {
        pub fn builder() -> VolumeSaleInfoOffersListPriceBuilder {
            VolumeSaleInfoOffersListPriceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The rental duration (for rental offers only)."]
    pub struct VolumeSaleInfoOffersRentalDuration {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub count: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub unit: ::std::option::Option<::std::string::String>,
    }
    impl VolumeSaleInfoOffersRentalDuration {
        pub fn builder() -> VolumeSaleInfoOffersRentalDurationBuilder {
            VolumeSaleInfoOffersRentalDurationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Offer retail (=discounted) price in Micros"]
    pub struct VolumeSaleInfoOffersRetailPrice {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountInMicros")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub amount_in_micros: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currencyCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub currency_code: ::std::option::Option<::std::string::String>,
    }
    impl VolumeSaleInfoOffersRetailPrice {
        pub fn builder() -> VolumeSaleInfoOffersRetailPriceBuilder {
            VolumeSaleInfoOffersRetailPriceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The actual selling price of the book. This is the same as the suggested retail or list price unless there are offers or discounts on this volume. (In LITE projection.)"]
    pub struct VolumeSaleInfoRetailPrice {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Amount in the currency listed below. (In LITE projection.)"]
        pub amount: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currencyCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An ISO 4217, three-letter currency code. (In LITE projection.)"]
        pub currency_code: ::std::option::Option<::std::string::String>,
    }
    impl VolumeSaleInfoRetailPrice {
        pub fn builder() -> VolumeSaleInfoRetailPriceBuilder {
            VolumeSaleInfoRetailPriceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Search result information related to this volume."]
    pub struct VolumeSearchInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textSnippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A text snippet containing the search query."]
        pub text_snippet: ::std::option::Option<::std::string::String>,
    }
    impl VolumeSearchInfo {
        pub fn builder() -> VolumeSearchInfoBuilder {
            VolumeSearchInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "User specific information related to this volume. (e.g. page this user last read or whether they purchased this book)"]
    pub struct VolumeUserInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "acquiredTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp when this volume was acquired by the user. (RFC 3339 UTC date-time format) Acquiring includes purchase, user upload, receiving family sharing, etc."]
        pub acquired_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "acquisitionType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How this volume was acquired."]
        pub acquisition_type: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "copy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Copy/Paste accounting information."]
        pub copy: ::std::option::Option<VolumeUserInfoCopy>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entitlementType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this volume is purchased, sample, pd download etc."]
        pub entitlement_type: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "familySharing")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information on the ability to share with the family."]
        pub family_sharing: ::std::option::Option<VolumeUserInfoFamilySharing>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isFamilySharedFromUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not the user shared this volume with the family."]
        pub is_family_shared_from_user: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isFamilySharedToUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not the user received this volume through family sharing."]
        pub is_family_shared_to_user: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isFamilySharingAllowed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated: Replaced by familySharing."]
        pub is_family_sharing_allowed: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isFamilySharingDisabledByFop")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated: Replaced by familySharing."]
        pub is_family_sharing_disabled_by_fop: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isInMyBooks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not this volume is currently in \"my books.\""]
        pub is_in_my_books: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isPreordered")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not this volume was pre-ordered by the authenticated user making the request. (In LITE projection.)"]
        pub is_preordered: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isPurchased")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not this volume was purchased by the authenticated user making the request. (In LITE projection.)"]
        pub is_purchased: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isUploaded")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not this volume was user uploaded."]
        pub is_uploaded: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "readingPosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's current reading position in the volume, if one is available. (In LITE projection.)"]
        pub reading_position: ::std::option::Option<::std::boxed::Box<ReadingPosition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rentalPeriod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Period during this book is/was a valid rental."]
        pub rental_period: ::std::option::Option<VolumeUserInfoRentalPeriod>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rentalState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this book is an active or an expired rental."]
        pub rental_state: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "review")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This user's review of this volume, if one exists."]
        pub review: ::std::option::Option<::std::boxed::Box<Review>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp when this volume was last modified by a user action, such as a reading position update, volume purchase or writing a review. (RFC 3339 UTC date-time format)."]
        pub updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userUploadedVolumeInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub user_uploaded_volume_info: ::std::option::Option<VolumeUserInfoUserUploadedVolumeInfo>,
    }
    impl VolumeUserInfo {
        pub fn builder() -> VolumeUserInfoBuilder {
            VolumeUserInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Copy/Paste accounting information."]
    pub struct VolumeUserInfoCopy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowedCharacterCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub allowed_character_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "limitType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub limit_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "remainingCharacterCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub remaining_character_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub updated: ::std::option::Option<::std::string::String>,
    }
    impl VolumeUserInfoCopy {
        pub fn builder() -> VolumeUserInfoCopyBuilder {
            VolumeUserInfoCopyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information on the ability to share with the family."]
    pub struct VolumeUserInfoFamilySharing {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "familyRole")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The role of the user in the family."]
        pub family_role: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isSharingAllowed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not this volume can be shared with the family by the user. This includes sharing eligibility of both the volume and the user. If the value is true, the user can initiate a family sharing action."]
        pub is_sharing_allowed: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isSharingDisabledByFop")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not sharing this volume is temporarily disabled due to issues with the Family Wallet."]
        pub is_sharing_disabled_by_fop: ::std::option::Option<::std::primitive::bool>,
    }
    impl VolumeUserInfoFamilySharing {
        pub fn builder() -> VolumeUserInfoFamilySharingBuilder {
            VolumeUserInfoFamilySharingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Period during this book is/was a valid rental."]
    pub struct VolumeUserInfoRentalPeriod {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endUtcSec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub end_utc_sec: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startUtcSec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub start_utc_sec: ::std::option::Option<::std::string::String>,
    }
    impl VolumeUserInfoRentalPeriod {
        pub fn builder() -> VolumeUserInfoRentalPeriodBuilder {
            VolumeUserInfoRentalPeriodBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct VolumeUserInfoUserUploadedVolumeInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "processingState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub processing_state: ::std::option::Option<::std::string::String>,
    }
    impl VolumeUserInfoUserUploadedVolumeInfo {
        pub fn builder() -> VolumeUserInfoUserUploadedVolumeInfoBuilder {
            VolumeUserInfoUserUploadedVolumeInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "General volume information."]
    pub struct VolumeVolumeInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowAnonLogging")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether anonymous logging should be allowed."]
        pub allow_anon_logging: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The names of the authors and/or editors for this volume. (In LITE projection)"]
        pub authors: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "averageRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The mean review rating for this volume. (min = 1.0, max = 5.0)"]
        pub average_rating: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canonicalVolumeLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Canonical URL for a volume. (In LITE projection.)"]
        pub canonical_volume_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "categories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of subject categories, such as \"Fiction\", \"Suspense\", etc."]
        pub categories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "comicsContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the volume has comics content."]
        pub comics_content: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An identifier for the version of the volume content (text & images). (In LITE projection)"]
        pub content_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A synopsis of the volume. The text of the description is formatted in HTML and includes simple formatting elements, such as b, i, and br tags. (In LITE projection.)"]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Physical dimensions of this volume."]
        pub dimensions: ::std::option::Option<VolumeVolumeInfoDimensions>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageLinks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of image links for all the sizes that are available. (In LITE projection.)"]
        pub image_links: ::std::option::Option<VolumeVolumeInfoImageLinks>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "industryIdentifiers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Industry standard identifiers for this volume."]
        pub industry_identifiers:
            ::std::option::Option<::std::vec::Vec<VolumeVolumeInfoIndustryIdentifiers>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "infoLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL to view information about this volume on the Google Books site. (In LITE projection)"]
        pub info_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "language")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Best language for this volume (based on content). It is the two-letter ISO 639-1 code such as 'fr', 'en', etc."]
        pub language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mainCategory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The main category to which this volume belongs. It will be the category from the categories list returned below that has the highest weight."]
        pub main_category: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maturityRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub maturity_rating: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of pages as per publisher metadata."]
        pub page_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "panelizationSummary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A top-level summary of the panelization info in this volume."]
        pub panelization_summary: ::std::option::Option<VolumeVolumeInfoPanelizationSummary>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previewLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL to preview this volume on the Google Books site."]
        pub preview_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "printType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of publication of this volume. Possible values are BOOK or MAGAZINE."]
        pub print_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "printedPageCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of printed pages in generated pdf representation."]
        pub printed_page_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishedDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date of publication. (In LITE projection.)"]
        pub published_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publisher")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Publisher of this volume. (In LITE projection.)"]
        pub publisher: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ratingsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of review ratings for this volume."]
        pub ratings_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "readingModes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reading modes available for this volume."]
        pub reading_modes: ::std::option::Option<VolumeVolumeInfoReadingModes>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "samplePageCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of sample pages as per publisher metadata."]
        pub sample_page_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "seriesInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub series_info: ::std::option::Option<::std::boxed::Box<Volumeseriesinfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subtitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Volume subtitle. (In LITE projection.)"]
        pub subtitle: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Volume title. (In LITE projection.)"]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl VolumeVolumeInfo {
        pub fn builder() -> VolumeVolumeInfoBuilder {
            VolumeVolumeInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Physical dimensions of this volume."]
    pub struct VolumeVolumeInfoDimensions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Height or length of this volume (in cm)."]
        pub height: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thickness")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Thickness of this volume (in cm)."]
        pub thickness: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Width of this volume (in cm)."]
        pub width: ::std::option::Option<::std::string::String>,
    }
    impl VolumeVolumeInfoDimensions {
        pub fn builder() -> VolumeVolumeInfoDimensionsBuilder {
            VolumeVolumeInfoDimensionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of image links for all the sizes that are available. (In LITE projection.)"]
    pub struct VolumeVolumeInfoImageLinks {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "extraLarge")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image link for extra large size (width of ~1280 pixels). (In LITE projection)"]
        pub extra_large: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "large")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image link for large size (width of ~800 pixels). (In LITE projection)"]
        pub large: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "medium")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image link for medium size (width of ~575 pixels). (In LITE projection)"]
        pub medium: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "small")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image link for small size (width of ~300 pixels). (In LITE projection)"]
        pub small: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "smallThumbnail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image link for small thumbnail size (width of ~80 pixels). (In LITE projection)"]
        pub small_thumbnail: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image link for thumbnail size (width of ~128 pixels). (In LITE projection)"]
        pub thumbnail: ::std::option::Option<::std::string::String>,
    }
    impl VolumeVolumeInfoImageLinks {
        pub fn builder() -> VolumeVolumeInfoImageLinksBuilder {
            VolumeVolumeInfoImageLinksBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct VolumeVolumeInfoIndustryIdentifiers {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "identifier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Industry specific volume identifier."]
        pub identifier: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier type. Possible values are ISBN_10, ISBN_13, ISSN and OTHER."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl VolumeVolumeInfoIndustryIdentifiers {
        pub fn builder() -> VolumeVolumeInfoIndustryIdentifiersBuilder {
            VolumeVolumeInfoIndustryIdentifiersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A top-level summary of the panelization info in this volume."]
    pub struct VolumeVolumeInfoPanelizationSummary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containsEpubBubbles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub contains_epub_bubbles: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containsImageBubbles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub contains_image_bubbles: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "epubBubbleVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub epub_bubble_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageBubbleVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub image_bubble_version: ::std::option::Option<::std::string::String>,
    }
    impl VolumeVolumeInfoPanelizationSummary {
        pub fn builder() -> VolumeVolumeInfoPanelizationSummaryBuilder {
            VolumeVolumeInfoPanelizationSummaryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The reading modes available for this volume."]
    pub struct VolumeVolumeInfoReadingModes {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub image: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub text: ::std::option::Option<::std::primitive::bool>,
    }
    impl VolumeVolumeInfoReadingModes {
        pub fn builder() -> VolumeVolumeInfoReadingModesBuilder {
            VolumeVolumeInfoReadingModesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Volume2 {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of volumes."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Volume>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource type."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl Volume2 {
        pub fn builder() -> Volume2Builder {
            Volume2Builder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Volumeannotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationDataId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The annotation data id for this volume annotation."]
        pub annotation_data_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationDataLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to get data for this annotation."]
        pub annotation_data_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of annotation this is."]
        pub annotation_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The content ranges to identify the selected text."]
        pub content_ranges: ::std::option::Option<VolumeannotationContentRanges>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data for this annotation."]
        pub data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates that this annotation is deleted."]
        pub deleted: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique id of this volume annotation."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource Type"]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "layerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Layer this annotation is for."]
        pub layer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pages the annotation spans."]
        pub page_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selectedText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Excerpt from the volume."]
        pub selected_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL to this resource."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp for the last time this anntoation was updated. (RFC 3339 UTC date-time format)."]
        pub updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Volume this annotation is for."]
        pub volume_id: ::std::option::Option<::std::string::String>,
    }
    impl Volumeannotation {
        pub fn builder() -> VolumeannotationBuilder {
            VolumeannotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The content ranges to identify the selected text."]
    pub struct VolumeannotationContentRanges {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cfiRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Range in CFI format for this annotation for version above."]
        pub cfi_range: ::std::option::Option<::std::boxed::Box<BooksAnnotationsRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Content version applicable to ranges below."]
        pub content_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gbImageRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Range in GB image format for this annotation for version above."]
        pub gb_image_range: ::std::option::Option<::std::boxed::Box<BooksAnnotationsRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gbTextRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Range in GB text format for this annotation for version above."]
        pub gb_text_range: ::std::option::Option<::std::boxed::Box<BooksAnnotationsRange>>,
    }
    impl VolumeannotationContentRanges {
        pub fn builder() -> VolumeannotationContentRangesBuilder {
            VolumeannotationContentRangesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Volumeannotations {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of volume annotations."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Volumeannotation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource type"]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to pass in for pagination for the next page. This will not be present if this request does not have more results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalItems")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of volume annotations found."]
        pub total_items: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version string for all of the volume annotations in this layer (not just the ones in this response). Note: the version string doesn't apply to the annotation data, just the information in this response (e.g. the location of annotations in the book)."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl Volumeannotations {
        pub fn builder() -> VolumeannotationsBuilder {
            VolumeannotationsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Volumes {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of volumes."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Volume>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource type."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalItems")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of volumes found. This might be greater than the number of volumes returned in this response if results have been paginated."]
        pub total_items: ::std::option::Option<::std::primitive::i64>,
    }
    impl Volumes {
        pub fn builder() -> VolumesBuilder {
            VolumesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Volumeseriesinfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bookDisplayNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The display number string. This should be used only for display purposes and the actual sequence should be inferred from the below orderNumber."]
        pub book_display_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource type."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shortSeriesBookTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Short book title in the context of the series."]
        pub short_series_book_title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumeSeries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub volume_series: ::std::option::Option<::std::vec::Vec<VolumeseriesinfoVolumeSeries>>,
    }
    impl Volumeseriesinfo {
        pub fn builder() -> VolumeseriesinfoBuilder {
            VolumeseriesinfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct VolumeseriesinfoVolumeSeries {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "issue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of issues. Applicable only for Collection Edition and Omnibus."]
        pub issue: ::std::option::Option<::std::vec::Vec<VolumeseriesinfoVolumeSeriesIssue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orderNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The book order number in the series."]
        pub order_number: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "seriesBookType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The book type in the context of series. Examples - Single Issue, Collection Edition, etc."]
        pub series_book_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "seriesId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The series id."]
        pub series_id: ::std::option::Option<::std::string::String>,
    }
    impl VolumeseriesinfoVolumeSeries {
        pub fn builder() -> VolumeseriesinfoVolumeSeriesBuilder {
            VolumeseriesinfoVolumeSeriesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct VolumeseriesinfoVolumeSeriesIssue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "issueDisplayNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub issue_display_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "issueOrderNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub issue_order_number: ::std::option::Option<::std::primitive::i64>,
    }
    impl VolumeseriesinfoVolumeSeriesIssue {
        pub fn builder() -> VolumeseriesinfoVolumeSeriesIssueBuilder {
            VolumeseriesinfoVolumeSeriesIssueBuilder::default()
        }
    }
}
