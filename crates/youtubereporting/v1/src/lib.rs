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
    pub mod jobs {
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The content owner's external ID on which behalf the user is acting on. If not set, the user is acting for himself (his own channel)."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The content owner's external ID on which behalf the user is acting on. If not set, the user is acting for himself (his own channel)."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The content owner's external ID on which behalf the user is acting on. If not set, the user is acting for himself (his own channel)."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "includeSystemManaged")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If set to true, also system-managed jobs will be returned; otherwise only user-created jobs will be returned. System-managed jobs can neither be modified nor deleted."]
                    pub include_system_managed: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The content owner's external ID on which behalf the user is acting on. If not set, the user is acting for himself (his own channel)."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Requested page size. Server may return fewer jobs than requested. If unspecified, server will pick an appropriate default."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListReportTypesResponse.next_page_token returned in response to the previous call to the `ListJobs` method."]
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
            pub mod reports {
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
                            #[serde(rename = "onBehalfOfContentOwner")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The content owner's external ID on which behalf the user is acting on. If not set, the user is acting for himself (his own channel)."]
                            pub on_behalf_of_content_owner:
                                ::std::option::Option<::std::string::String>,
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
                            #[serde(rename = "createdAfter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If set, only reports created after the specified date/time are returned."]
                            pub created_after: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "onBehalfOfContentOwner")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The content owner's external ID on which behalf the user is acting on. If not set, the user is acting for himself (his own channel)."]
                            pub on_behalf_of_content_owner:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Requested page size. Server may return fewer report types than requested. If unspecified, server will pick an appropriate default."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListReportsResponse.next_page_token returned in response to the previous call to the `ListReports` method."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "startTimeAtOrAfter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If set, only reports whose start time is greater than or equal the specified date/time are returned."]
                            pub start_time_at_or_after:
                                ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "startTimeBefore")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "If set, only reports whose start time is smaller than the specified date/time are returned."]
                            pub start_time_before: ::std::option::Option<::std::string::String>,
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
    pub mod report_types {
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
                    #[serde(rename = "includeSystemManaged")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If set to true, also system-managed report types will be returned; otherwise only the report types that can be used to create new reporting jobs will be returned."]
                    pub include_system_managed: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The content owner's external ID on which behalf the user is acting on. If not set, the user is acting for himself (his own channel)."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Requested page size. Server may return fewer report types than requested. If unspecified, server will pick an appropriate default."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListReportTypesResponse.next_page_token returned in response to the previous call to the `ListReportTypes` method."]
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
pub mod schemas {
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
    #[doc = "gdata"]
    pub struct GdataBlobstore2Info {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blobGeneration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub blob_generation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blobId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub blob_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "downloadReadHandle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub download_read_handle: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "readToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub read_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uploadMetadataContainer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub upload_metadata_container: ::std::option::Option<::std::string::String>,
    }
    impl GdataBlobstore2Info {
        pub fn builder() -> GdataBlobstore2InfoBuilder {
            GdataBlobstore2InfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "gdata"]
    pub struct GdataCompositeMedia {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blobRef")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub blob_ref: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blobstore2Info")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub blobstore2_info: ::std::option::Option<::std::boxed::Box<GdataBlobstore2Info>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cosmoBinaryReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub cosmo_binary_reference: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "crc32cHash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub crc32c_hash: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inline")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub inline: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "length")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub length: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "md5Hash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub md5_hash: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub object_id: ::std::option::Option<::std::boxed::Box<GdataObjectId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "referenceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub reference_type: ::std::option::Option<GdataCompositeMediaReferenceTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sha1Hash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub sha1_hash: ::std::option::Option<::std::string::String>,
    }
    impl GdataCompositeMedia {
        pub fn builder() -> GdataCompositeMediaBuilder {
            GdataCompositeMediaBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "gdata"]
    pub enum GdataCompositeMediaReferenceTypeEnum {
        #[serde(rename = "PATH")]
        #[doc = "gdata"]
        Path,
        #[serde(rename = "BLOB_REF")]
        #[doc = "gdata"]
        BlobRef,
        #[serde(rename = "INLINE")]
        #[doc = "gdata"]
        Inline,
        #[serde(rename = "BIGSTORE_REF")]
        #[doc = "gdata"]
        BigstoreRef,
        #[serde(rename = "COSMO_BINARY_REFERENCE")]
        #[doc = "gdata"]
        CosmoBinaryReference,
    }
    impl ::std::default::Default for GdataCompositeMediaReferenceTypeEnum {
        fn default() -> Self {
            Self::Path
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "gdata"]
    pub struct GdataContentTypeInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bestGuess")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub best_guess: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fromBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub from_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fromFileName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub from_file_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fromHeader")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub from_header: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fromUrlPath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub from_url_path: ::std::option::Option<::std::string::String>,
    }
    impl GdataContentTypeInfo {
        pub fn builder() -> GdataContentTypeInfoBuilder {
            GdataContentTypeInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "gdata"]
    pub struct GdataDiffChecksumsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "checksumsLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub checksums_location: ::std::option::Option<::std::boxed::Box<GdataCompositeMedia>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "chunkSizeBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub chunk_size_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub object_location: ::std::option::Option<::std::boxed::Box<GdataCompositeMedia>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectSizeBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub object_size_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub object_version: ::std::option::Option<::std::string::String>,
    }
    impl GdataDiffChecksumsResponse {
        pub fn builder() -> GdataDiffChecksumsResponseBuilder {
            GdataDiffChecksumsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "gdata"]
    pub struct GdataDiffDownloadResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub object_location: ::std::option::Option<::std::boxed::Box<GdataCompositeMedia>>,
    }
    impl GdataDiffDownloadResponse {
        pub fn builder() -> GdataDiffDownloadResponseBuilder {
            GdataDiffDownloadResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "gdata"]
    pub struct GdataDiffUploadRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "checksumsInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub checksums_info: ::std::option::Option<::std::boxed::Box<GdataCompositeMedia>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub object_info: ::std::option::Option<::std::boxed::Box<GdataCompositeMedia>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub object_version: ::std::option::Option<::std::string::String>,
    }
    impl GdataDiffUploadRequest {
        pub fn builder() -> GdataDiffUploadRequestBuilder {
            GdataDiffUploadRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "gdata"]
    pub struct GdataDiffUploadResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub object_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalObject")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub original_object: ::std::option::Option<::std::boxed::Box<GdataCompositeMedia>>,
    }
    impl GdataDiffUploadResponse {
        pub fn builder() -> GdataDiffUploadResponseBuilder {
            GdataDiffUploadResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "gdata"]
    pub struct GdataDiffVersionResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectSizeBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub object_size_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub object_version: ::std::option::Option<::std::string::String>,
    }
    impl GdataDiffVersionResponse {
        pub fn builder() -> GdataDiffVersionResponseBuilder {
            GdataDiffVersionResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "gdata"]
    pub struct GdataDownloadParameters {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowGzipCompression")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub allow_gzip_compression: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ignoreRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub ignore_range: ::std::option::Option<::std::primitive::bool>,
    }
    impl GdataDownloadParameters {
        pub fn builder() -> GdataDownloadParametersBuilder {
            GdataDownloadParametersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "gdata"]
    pub struct GdataMedia {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "algorithm")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub algorithm: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bigstoreObjectRef")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub bigstore_object_ref: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blobRef")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub blob_ref: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blobstore2Info")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub blobstore2_info: ::std::option::Option<::std::boxed::Box<GdataBlobstore2Info>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "compositeMedia")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub composite_media:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GdataCompositeMedia>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub content_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentTypeInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub content_type_info: ::std::option::Option<::std::boxed::Box<GdataContentTypeInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cosmoBinaryReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub cosmo_binary_reference: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "crc32cHash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub crc32c_hash: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diffChecksumsResponse")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub diff_checksums_response:
            ::std::option::Option<::std::boxed::Box<GdataDiffChecksumsResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diffDownloadResponse")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub diff_download_response:
            ::std::option::Option<::std::boxed::Box<GdataDiffDownloadResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diffUploadRequest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub diff_upload_request: ::std::option::Option<::std::boxed::Box<GdataDiffUploadRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diffUploadResponse")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub diff_upload_response: ::std::option::Option<::std::boxed::Box<GdataDiffUploadResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diffVersionResponse")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub diff_version_response:
            ::std::option::Option<::std::boxed::Box<GdataDiffVersionResponse>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "downloadParameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub download_parameters: ::std::option::Option<::std::boxed::Box<GdataDownloadParameters>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filename")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub filename: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub hash: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hashVerified")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub hash_verified: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inline")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub inline: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isPotentialRetry")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub is_potential_retry: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "length")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub length: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "md5Hash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub md5_hash: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mediaId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub media_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub object_id: ::std::option::Option<::std::boxed::Box<GdataObjectId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "referenceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub reference_type: ::std::option::Option<GdataMediaReferenceTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sha1Hash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub sha1_hash: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sha256Hash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub sha256_hash: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "token")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub token: ::std::option::Option<::std::string::String>,
    }
    impl GdataMedia {
        pub fn builder() -> GdataMediaBuilder {
            GdataMediaBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "gdata"]
    pub enum GdataMediaReferenceTypeEnum {
        #[serde(rename = "PATH")]
        #[doc = "gdata"]
        Path,
        #[serde(rename = "BLOB_REF")]
        #[doc = "gdata"]
        BlobRef,
        #[serde(rename = "INLINE")]
        #[doc = "gdata"]
        Inline,
        #[serde(rename = "GET_MEDIA")]
        #[doc = "gdata"]
        GetMedia,
        #[serde(rename = "COMPOSITE_MEDIA")]
        #[doc = "gdata"]
        CompositeMedia,
        #[serde(rename = "BIGSTORE_REF")]
        #[doc = "gdata"]
        BigstoreRef,
        #[serde(rename = "DIFF_VERSION_RESPONSE")]
        #[doc = "gdata"]
        DiffVersionResponse,
        #[serde(rename = "DIFF_CHECKSUMS_RESPONSE")]
        #[doc = "gdata"]
        DiffChecksumsResponse,
        #[serde(rename = "DIFF_DOWNLOAD_RESPONSE")]
        #[doc = "gdata"]
        DiffDownloadResponse,
        #[serde(rename = "DIFF_UPLOAD_REQUEST")]
        #[doc = "gdata"]
        DiffUploadRequest,
        #[serde(rename = "DIFF_UPLOAD_RESPONSE")]
        #[doc = "gdata"]
        DiffUploadResponse,
        #[serde(rename = "COSMO_BINARY_REFERENCE")]
        #[doc = "gdata"]
        CosmoBinaryReference,
        #[serde(rename = "ARBITRARY_BYTES")]
        #[doc = "gdata"]
        ArbitraryBytes,
    }
    impl ::std::default::Default for GdataMediaReferenceTypeEnum {
        fn default() -> Self {
            Self::Path
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "gdata"]
    pub struct GdataObjectId {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub bucket_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "generation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub generation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "gdata"]
        pub object_name: ::std::option::Option<::std::string::String>,
    }
    impl GdataObjectId {
        pub fn builder() -> GdataObjectIdBuilder {
            GdataObjectIdBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A job creating reports of a specific type."]
    pub struct Job {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creation date/time of the job."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expireTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date/time when this job will expire/expired. After a job expired, no new reports are generated."]
        pub expire_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The server-generated ID of the job (max. 40 characters)."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the job (max. 100 characters)."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reportTypeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of reports this job creates. Corresponds to the ID of a ReportType."]
        pub report_type_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "systemManaged")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if this a system-managed job that cannot be modified by the user; otherwise false."]
        pub system_managed: ::std::option::Option<::std::primitive::bool>,
    }
    impl Job {
        pub fn builder() -> JobBuilder {
            JobBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ReportingService.ListJobs."]
    pub struct ListJobsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of jobs."]
        pub jobs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Job>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve next page of results. Pass this value in the ListJobsRequest.page_token field in the subsequent call to `ListJobs` method to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListJobsResponse {
        pub fn builder() -> ListJobsResponseBuilder {
            ListJobsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ReportingService.ListReportTypes."]
    pub struct ListReportTypesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve next page of results. Pass this value in the ListReportTypesRequest.page_token field in the subsequent call to `ListReportTypes` method to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reportTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of report types."]
        pub report_types: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ReportType>>>,
    }
    impl ListReportTypesResponse {
        pub fn builder() -> ListReportTypesResponseBuilder {
            ListReportTypesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ReportingService.ListReports."]
    pub struct ListReportsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve next page of results. Pass this value in the ListReportsRequest.page_token field in the subsequent call to `ListReports` method to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reports")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of report types."]
        pub reports: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Report>>>,
    }
    impl ListReportsResponse {
        pub fn builder() -> ListReportsResponseBuilder {
            ListReportsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A report's metadata including the URL from which the report itself can be downloaded."]
    pub struct Report {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date/time when this report was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "downloadUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL from which the report can be downloaded (max. 1000 characters)."]
        pub download_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end of the time period that the report instance covers. The value is exclusive."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The server-generated ID of the report."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobExpireTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date/time when the job this report belongs to will expire/expired."]
        pub job_expire_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "jobId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the job that created this report."]
        pub job_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start of the time period that the report instance covers. The value is inclusive."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl Report {
        pub fn builder() -> ReportBuilder {
            ReportBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A report type."]
    pub struct ReportType {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deprecateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date/time when this report type was/will be deprecated."]
        pub deprecate_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the report type (max. 100 characters)."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the report type (max. 100 characters)."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "systemManaged")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if this a system-managed report type; otherwise false. Reporting jobs for system-managed report types are created automatically and can thus not be used in the `CreateJob` method."]
        pub system_managed: ::std::option::Option<::std::primitive::bool>,
    }
    impl ReportType {
        pub fn builder() -> ReportTypeBuilder {
            ReportTypeBuilder::default()
        }
    }
}
