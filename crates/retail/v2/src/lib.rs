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
                pub mod resources {
                    pub mod catalogs {
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
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Maximum number of Catalogs to return. If unspecified, defaults to 50. The maximum allowed value is 1000. Values above 1000 will be coerced to 1000. If this field is negative, an INVALID_ARGUMENT is returned."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A page token ListCatalogsResponse.next_page_token, received from a previous CatalogService.ListCatalogs call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to CatalogService.ListCatalogs must match the call that provided the page token. Otherwise, an INVALID_ARGUMENT error is returned."]
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
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Indicates which fields in the provided Catalog to update. If not set, will only update the Catalog.product_level_config field, which is also the only currently supported field to update. If an unsupported or unknown field is provided, an INVALID_ARGUMENT error is returned."]
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
                            pub mod branches {
                                pub mod resources {
                                    pub mod products {
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
                                                    #[serde(rename = "productId")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "Required. The ID to use for the Product, which will become the final component of the Product.name. If the caller does not have permission to create the Product, regardless of whether or not it exists, a PERMISSION_DENIED error is returned. This field must be unique among all Products with the same parent. Otherwise, an ALREADY_EXISTS error is returned. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned."]
                                                    pub product_id: ::std::option::Option<
                                                        ::std::string::String,
                                                    >,
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
                                                    #[serde(rename = "allowMissing")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "If set to true, and the Product is not found, a new Product will be created. In this situation, `update_mask` is ignored."]
                                                    pub allow_missing: ::std::option::Option<
                                                        ::std::primitive::bool,
                                                    >,
                                                    #[builder(
                                                        default = "{ ::std::default::Default::default() }",
                                                        setter(into)
                                                    )]
                                                    #[serde(rename = "updateMask")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "Indicates which fields in the provided Product to update. The immutable and output only fields are NOT supported. If not set, all supported fields (the fields that are neither immutable nor output only) are updated. If an unsupported or unknown field is provided, an INVALID_ARGUMENT error is returned."]
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
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The standard list filter."]
                                            pub filter:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The standard list page size."]
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
                                            #[doc = "The standard list page token."]
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
                            pub mod user_events {
                                pub mod methods {
                                    pub mod collect {
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
                                            #[serde(rename = "ets")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The event timestamp in milliseconds. This prevents browser caching of otherwise identical get requests. The name is abbreviated to reduce the payload bytes."]
                                            pub ets: ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "uri")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The URL including cgi-parameters but excluding the hash fragment with a length limit of 5,000 characters. This is often more useful than the referer URL, because many browsers only send the domain for 3rd party requests."]
                                            pub uri: ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "userEvent")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Required. URL encoded UserEvent proto with a length limit of 2,000,000 characters."]
                                            pub user_event:
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
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The standard list filter."]
                                    pub filter: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The standard list page size."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
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
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message that represents an arbitrary HTTP body. It should only be used for payload formats that can't be represented as JSON, such as raw binary or an HTML page. This message can be used both in streaming and non-streaming API methods in the request as well as the response. It can be used as a top-level request field, which is convenient if one wants to extract parameters from either the URL or HTTP template into the request fields and also want access to the raw HTTP body. Example: message GetResourceRequest { // A unique request id. string request_id = 1; // The raw HTTP body is bound to this field. google.api.HttpBody http_body = 2; } service ResourceService { rpc GetResource(GetResourceRequest) returns (google.api.HttpBody); rpc UpdateResource(google.api.HttpBody) returns (google.protobuf.Empty); } Example with streaming methods: service CaldavService { rpc GetCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); rpc UpdateCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); } Use of this type only changes how the request and response bodies are handled, all other features will continue to work unchanged."]
    pub struct GoogleApiHttpBody {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The HTTP Content-Type header value specifying the content type of the body."]
        pub content_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "data")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The HTTP request/response body as raw binary."]
        pub data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "extensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Application specific response metadata. Must be set in the first response for streaming APIs."]
        pub extensions: ::std::option::Option<
            ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        >,
    }
    impl GoogleApiHttpBody {
        pub fn builder() -> GoogleApiHttpBodyBuilder {
            GoogleApiHttpBodyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A description of the context in which an error occurred."]
    pub struct GoogleCloudRetailLoggingErrorContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "httpRequest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The HTTP request which was processed when the error was triggered."]
        pub http_request:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailLoggingHttpRequestContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reportLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location in the source code where the decision was made to report the error, usually the place where it was logged."]
        pub report_location:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailLoggingSourceLocation>>,
    }
    impl GoogleCloudRetailLoggingErrorContext {
        pub fn builder() -> GoogleCloudRetailLoggingErrorContextBuilder {
            GoogleCloudRetailLoggingErrorContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An error log which is reported to the Error Reporting system. This proto a superset of google.devtools.clouderrorreporting.v1beta1.ReportedErrorEvent."]
    pub struct GoogleCloudRetailLoggingErrorLog {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "context")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A description of the context in which the error occurred."]
        pub context: ::std::option::Option<::std::boxed::Box<GoogleCloudRetailLoggingErrorContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importPayload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error payload that is populated on LRO import APIs."]
        pub import_payload:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailLoggingImportErrorContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A message describing the error."]
        pub message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestPayload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The API request payload, represented as a protocol buffer. Most API request types are supported. For example: \"type.googleapis.com/google.cloud.retail.v2.ProductService.CreateProductRequest\" \"type.googleapis.com/google.cloud.retail.v2.UserEventService.WriteUserEventRequest\""]
        pub request_payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responsePayload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The API response payload, represented as a protocol buffer. This is used to log some \"soft errors\", where the response is valid but we consider there are some quality issues like unjoined events. The following API responses are supported and no PII is included: \"google.cloud.retail.v2.PredictionService.Predict\" \"google.cloud.retail.v2.UserEventService.WriteUserEvent\" \"google.cloud.retail.v2.UserEventService.CollectUserEvent\""]
        pub response_payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The service context in which this error has occurred."]
        pub service_context:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailLoggingServiceContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The RPC status associated with the error log."]
        pub status: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
    }
    impl GoogleCloudRetailLoggingErrorLog {
        pub fn builder() -> GoogleCloudRetailLoggingErrorLogBuilder {
            GoogleCloudRetailLoggingErrorLogBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "HTTP request data that is related to a reported error."]
    pub struct GoogleCloudRetailLoggingHttpRequestContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseStatusCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The HTTP response status code for the request."]
        pub response_status_code: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudRetailLoggingHttpRequestContext {
        pub fn builder() -> GoogleCloudRetailLoggingHttpRequestContextBuilder {
            GoogleCloudRetailLoggingHttpRequestContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The error payload that is populated on LRO import APIs. Including: \"google.cloud.retail.v2.ProductService.ImportProducts\" \"google.cloud.retail.v2.EventService.ImportUserEvents\""]
    pub struct GoogleCloudRetailLoggingImportErrorContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "catalogItem")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The detailed content which caused the error on importing a catalog item."]
        pub catalog_item: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsPath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GCS file path of the import source. Can be set for batch operation error."]
        pub gcs_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Line number of the content in file. Should be empty for permission or batch operation error."]
        pub line_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The operation resource name of the LRO."]
        pub operation_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "product")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The detailed content which caused the error on importing a product."]
        pub product: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userEvent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The detailed content which caused the error on importing a user event."]
        pub user_event: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailLoggingImportErrorContext {
        pub fn builder() -> GoogleCloudRetailLoggingImportErrorContextBuilder {
            GoogleCloudRetailLoggingImportErrorContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a running service that sends errors."]
    pub struct GoogleCloudRetailLoggingServiceContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "service")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An identifier of the service. For example, \"retail.googleapis.com\"."]
        pub service: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailLoggingServiceContext {
        pub fn builder() -> GoogleCloudRetailLoggingServiceContextBuilder {
            GoogleCloudRetailLoggingServiceContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Indicates a location in the source code of the service for which errors are reported."]
    pub struct GoogleCloudRetailLoggingSourceLocation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "functionName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Human-readable name of a function or method. For example, \"google.cloud.retail.v2.UserEventService.ImportUserEvents\"."]
        pub function_name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailLoggingSourceLocation {
        pub fn builder() -> GoogleCloudRetailLoggingSourceLocationBuilder {
            GoogleCloudRetailLoggingSourceLocationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "BigQuery source import data from."]
    pub struct GoogleCloudRetailV2BigQuerySource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSchema")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The schema to use when parsing the data from the source. Supported values for product imports: * `product` (default): One JSON Product per line. Each product must have a valid Product.id. * `product_merchant_center`: See [Importing catalog data from Merchant Center](https://cloud.google.com/retail/recommendations-ai/docs/upload-catalog#mc). Supported values for user events imports: * `user_event` (default): One JSON UserEvent per line. * `user_event_ga360`: Using https://support.google.com/analytics/answer/3437719?hl=en."]
        pub data_schema: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datasetId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The BigQuery data set to copy the data from with a length limit of 1,024 characters."]
        pub dataset_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsStagingDir")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Intermediate Cloud Storage directory used for the import with a length limit of 2,000 characters. Can be specified if one wants to have the BigQuery export to a specific Cloud Storage directory."]
        pub gcs_staging_dir: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The project id (can be project # or id) that the BigQuery source is in with a length limit of 128 characters. If not specified, inherits the project id from the parent request."]
        pub project_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The BigQuery table to copy the data from with a length limit of 1,024 characters."]
        pub table_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2BigQuerySource {
        pub fn builder() -> GoogleCloudRetailV2BigQuerySourceBuilder {
            GoogleCloudRetailV2BigQuerySourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The catalog configuration."]
    pub struct GoogleCloudRetailV2Catalog {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Immutable. The catalog display name. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Immutable. The fully qualified resource name of the catalog."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productLevelConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The product level configuration."]
        pub product_level_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2ProductLevelConfig>>,
    }
    impl GoogleCloudRetailV2Catalog {
        pub fn builder() -> GoogleCloudRetailV2CatalogBuilder {
            GoogleCloudRetailV2CatalogBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A custom attribute that is not explicitly modeled in Product."]
    pub struct GoogleCloudRetailV2CustomAttribute {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numbers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The numerical values of this custom attribute. For example, `[2.3, 15.4]` when the key is \"lengths_cm\". At most 400 values are allowed.Otherwise, an INVALID_ARGUMENT error is returned. Exactly one of text or numbers should be set. Otherwise, an INVALID_ARGUMENT error is returned."]
        pub numbers: ::std::option::Option<::std::vec::Vec<::std::primitive::f64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The textual values of this custom attribute. For example, `[\"yellow\", \"green\"]` when the key is \"color\". At most 400 values are allowed. Empty values are not allowed. Each value must be a UTF-8 encoded string with a length limit of 256 characters. Otherwise, an INVALID_ARGUMENT error is returned. Exactly one of text or numbers should be set. Otherwise, an INVALID_ARGUMENT error is returned."]
        pub text: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudRetailV2CustomAttribute {
        pub fn builder() -> GoogleCloudRetailV2CustomAttributeBuilder {
            GoogleCloudRetailV2CustomAttributeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Google Cloud Storage location for input content. format."]
    pub struct GoogleCloudRetailV2GcsSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataSchema")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The schema to use when parsing the data from the source. Supported values for product imports: * `product` (default): One JSON Product per line. Each product must have a valid Product.id. * `product_merchant_center`: See [Importing catalog data from Merchant Center](https://cloud.google.com/retail/recommendations-ai/docs/upload-catalog#mcc). Supported values for user events imports: * `user_event` (default): One JSON UserEvent per line. * `user_event_ga360`: Using https://support.google.com/analytics/answer/3437719?hl=en."]
        pub data_schema: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputUris")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Google Cloud Storage URIs to input files. URI can be up to 2000 characters long. URIs can match the full object path (for example, `gs://bucket/directory/object.json`) or a pattern matching one or more files, such as `gs://bucket/directory/*.json`. A request can contain at most 100 files, and each file can be up to 2 GB. See [Importing product information](https://cloud.google.com/recommendations-ai/docs/upload-catalog) for the expected file format and setup instructions."]
        pub input_uris: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudRetailV2GcsSource {
        pub fn builder() -> GoogleCloudRetailV2GcsSourceBuilder {
            GoogleCloudRetailV2GcsSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Product thumbnail/detail image."]
    pub struct GoogleCloudRetailV2Image {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Height of the image in number of pixels. This field must be nonnegative. Otherwise, an INVALID_ARGUMENT error is returned."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. URI of the image. This field must be a valid UTF-8 encoded URI with a length limit of 5,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Google Merchant Center property [image_link](https://support.google.com/merchants/answer/6324350). Schema.org property [Product.image](https://schema.org/image)."]
        pub uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Width of the image in number of pixels. This field must be nonnegative. Otherwise, an INVALID_ARGUMENT error is returned."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudRetailV2Image {
        pub fn builder() -> GoogleCloudRetailV2ImageBuilder {
            GoogleCloudRetailV2ImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration of destination for Import related errors."]
    pub struct GoogleCloudRetailV2ImportErrorsConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsPrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage path for import errors. This must be an empty, existing Cloud Storage bucket. Import errors will be written to a file in this bucket, one per line, as a JSON-encoded `google.rpc.Status` message."]
        pub gcs_prefix: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2ImportErrorsConfig {
        pub fn builder() -> GoogleCloudRetailV2ImportErrorsConfigBuilder {
            GoogleCloudRetailV2ImportErrorsConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata related to the progress of the Import operation. This will be returned by the google.longrunning.Operation.metadata field."]
    pub struct GoogleCloudRetailV2ImportMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Operation create time."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "failureCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Count of entries that encountered errors while processing."]
        pub failure_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "successCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Count of entries that were processed successfully."]
        pub success_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Operation last update time. If the operation is done, this is also the finish time."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2ImportMetadata {
        pub fn builder() -> GoogleCloudRetailV2ImportMetadataBuilder {
            GoogleCloudRetailV2ImportMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for Import methods."]
    pub struct GoogleCloudRetailV2ImportProductsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorsConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The desired location of errors incurred during the Import."]
        pub errors_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2ImportErrorsConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The desired input location of the data."]
        pub input_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2ProductInputConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateMask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates which fields in the provided imported 'products' to update. If not set, will by default update all fields."]
        pub update_mask: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2ImportProductsRequest {
        pub fn builder() -> GoogleCloudRetailV2ImportProductsRequestBuilder {
            GoogleCloudRetailV2ImportProductsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of the ImportProductsRequest. If the long running operation is done, then this message is returned by the google.longrunning.Operations.response field if the operation was successful."]
    pub struct GoogleCloudRetailV2ImportProductsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorSamples")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A sample of errors encountered while processing the request."]
        pub error_samples:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorsConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Echoes the destination for the complete errors in the request if set."]
        pub errors_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2ImportErrorsConfig>>,
    }
    impl GoogleCloudRetailV2ImportProductsResponse {
        pub fn builder() -> GoogleCloudRetailV2ImportProductsResponseBuilder {
            GoogleCloudRetailV2ImportProductsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for the ImportUserEvents request."]
    pub struct GoogleCloudRetailV2ImportUserEventsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorsConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The desired location of errors incurred during the Import. Cannot be set for inline user event imports."]
        pub errors_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2ImportErrorsConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The desired input location of the data."]
        pub input_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2UserEventInputConfig>>,
    }
    impl GoogleCloudRetailV2ImportUserEventsRequest {
        pub fn builder() -> GoogleCloudRetailV2ImportUserEventsRequestBuilder {
            GoogleCloudRetailV2ImportUserEventsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of the ImportUserEventsRequest. If the long running operation was successful, then this message is returned by the google.longrunning.Operations.response field if the operation was successful."]
    pub struct GoogleCloudRetailV2ImportUserEventsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorSamples")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A sample of errors encountered while processing the request."]
        pub error_samples:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorsConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Echoes the destination for the complete errors if this field was set in the request."]
        pub errors_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2ImportErrorsConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importSummary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Aggregated statistics of user event import status."]
        pub import_summary:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2UserEventImportSummary>>,
    }
    impl GoogleCloudRetailV2ImportUserEventsResponse {
        pub fn builder() -> GoogleCloudRetailV2ImportUserEventsResponseBuilder {
            GoogleCloudRetailV2ImportUserEventsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for CatalogService.ListCatalogs method."]
    pub struct GoogleCloudRetailV2ListCatalogsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "catalogs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All the customer's Catalogs."]
        pub catalogs:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudRetailV2Catalog>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token that can be sent as ListCatalogsRequest.page_token to retrieve the next page. If this field is omitted, there are no subsequent pages."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2ListCatalogsResponse {
        pub fn builder() -> GoogleCloudRetailV2ListCatalogsResponseBuilder {
            GoogleCloudRetailV2ListCatalogsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for Predict method."]
    pub struct GoogleCloudRetailV2PredictRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Filter for restricting prediction results with a length limit of 5,000 characters. Accepts values for tags and the `filterOutOfStockItems` flag. * Tag expressions. Restricts predictions to products that match all of the specified tags. Boolean operators `OR` and `NOT` are supported if the expression is enclosed in parentheses, and must be separated from the tag values by a space. `-\"tagA\"` is also supported and is equivalent to `NOT \"tagA\"`. Tag values must be double quoted UTF-8 encoded strings with a size limit of 1,000 characters. * filterOutOfStockItems. Restricts predictions to products that do not have a stockState value of OUT_OF_STOCK. Examples: * tag=(\"Red\" OR \"Blue\") tag=\"New-Arrival\" tag=(NOT \"promotional\") * filterOutOfStockItems tag=(-\"promotional\") * filterOutOfStockItems If your filter blocks all prediction results, nothing will be returned. If you want generic (unfiltered) popular products to be returned instead, set `strictFiltering` to false in `PredictRequest.params`."]
        pub filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The labels for the predict request. * Label keys can contain lowercase letters, digits and hyphens, must start with a letter, and must end with a letter or digit. * Non-zero label values can contain lowercase letters, digits and hyphens, must start with a letter, and must end with a letter or digit. * No more than 64 labels can be associated with a given request. See https://goo.gl/xmQnxf for more information on and examples of labels."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum number of results to return per page. Set this property to the number of prediction results needed. If zero, the service will choose a reasonable default. The maximum allowed value is 100. Values above 100 will be coerced to 100."]
        pub page_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The previous PredictResponse.next_page_token."]
        pub page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "params")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional domain specific parameters for the predictions. Allowed values: * `returnProduct`: Boolean. If set to true, the associated product object will be returned in the `results.metadata` field in the prediction response. * `returnScore`: Boolean. If set to true, the prediction 'score' corresponding to each returned product will be set in the `results.metadata` field in the prediction response. The given 'score' indicates the probability of an product being clicked/purchased given the user's context and history. * `strictFiltering`: Boolean. True by default. If set to false, the service will return generic (unfiltered) popular products instead of empty if your filter blocks all prediction results."]
        pub params:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userEvent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Context about the user, what they are looking at and what action they took to trigger the predict request. Note that this user event detail won't be ingested to userEvent logs. Thus, a separate userEvent write request is required for event logging."]
        pub user_event: ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2UserEvent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "validateOnly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Use validate only mode for this prediction query. If set to true, a dummy model will be used that returns arbitrary products. Note that the validate only mode should only be used for testing the API, or if the model is not ready."]
        pub validate_only: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudRetailV2PredictRequest {
        pub fn builder() -> GoogleCloudRetailV2PredictRequestBuilder {
            GoogleCloudRetailV2PredictRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for predict method."]
    pub struct GoogleCloudRetailV2PredictResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributionToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique attribution token. This should be included in the UserEvent logs resulting from this recommendation, which enables accurate attribution of recommendation model performance."]
        pub attribution_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "missingIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "IDs of products in the request that were missing from the inventory."]
        pub missing_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of recommended products. The order represents the ranking (from the most relevant product to the least)."]
        pub results: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudRetailV2PredictResponsePredictionResult>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "validateOnly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the validateOnly property was set in the request."]
        pub validate_only: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudRetailV2PredictResponse {
        pub fn builder() -> GoogleCloudRetailV2PredictResponseBuilder {
            GoogleCloudRetailV2PredictResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "PredictionResult represents the recommendation prediction results."]
    pub struct GoogleCloudRetailV2PredictResponsePredictionResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the recommended product"]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional product metadata / annotations. Possible values: * `product`: JSON representation of the product. Will be set if `returnProduct` is set to true in `PredictRequest.params`. * `score`: Prediction score in double value. Will be set if `returnScore` is set to true in `PredictRequest.params`."]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl GoogleCloudRetailV2PredictResponsePredictionResult {
        pub fn builder() -> GoogleCloudRetailV2PredictResponsePredictionResultBuilder {
            GoogleCloudRetailV2PredictResponsePredictionResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The price information of a Product."]
    pub struct GoogleCloudRetailV2PriceInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cost")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The costs associated with the sale of a particular product. Used for gross profit reporting. * Profit = price - cost Google Merchant Center property [cost_of_goods_sold](https://support.google.com/merchants/answer/9017895)."]
        pub cost: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currencyCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The 3-letter currency code defined in [ISO 4217](https://www.iso.org/iso-4217-currency-codes.html). If this field is an unrecognizable currency code, an INVALID_ARGUMENT error is returned."]
        pub currency_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalPrice")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Price of the product without any discount. If zero, by default set to be the price."]
        pub original_price: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "price")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Price of the product. Google Merchant Center property [price](https://support.google.com/merchants/answer/6324371). Schema.org property [Offer.priceSpecification](https://schema.org/priceSpecification)."]
        pub price: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudRetailV2PriceInfo {
        pub fn builder() -> GoogleCloudRetailV2PriceInfoBuilder {
            GoogleCloudRetailV2PriceInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Product captures all metadata information of items to be recommended or searched."]
    pub struct GoogleCloudRetailV2Product {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Highly encouraged. Extra product attributes to be included. For example, for products, this could include the store name, vendor, style, color, etc. These are very strong signals for recommendation model, thus we highly recommend providing the attributes here. Features that can take on one of a limited number of possible values. Two types of features can be set are: Textual features. some examples would be the brand/maker of a product, or country of a customer. Numerical features. Some examples would be the height/weight of a product, or age of a customer. For example: `{ \"vendor\": {\"text\": [\"vendor123\", \"vendor456\"]}, \"lengths_cm\": {\"numbers\":[2.3, 15.4]}, \"heights_cm\": {\"numbers\":[8.1, 6.4]} }`. This field needs to pass all below criteria, otherwise an INVALID_ARGUMENT error is returned: * Max entries count: 150 by default; 100 for Type.VARIANT. * Max indexable entries count: 150 by default; 40 for Type.VARIANT. * Max searchable entries count: 30. * The key must be a UTF-8 encoded string with a length limit of 128 characters."]
        pub attributes: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                ::std::boxed::Box<GoogleCloudRetailV2CustomAttribute>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "availability")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The online availability of the Product. Default to Availability.IN_STOCK. Google Merchant Center Property [availability](https://support.google.com/merchants/answer/6324448). Schema.org Property [Offer.availability](https://schema.org/availability)."]
        pub availability: ::std::option::Option<GoogleCloudRetailV2ProductAvailabilityEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "availableQuantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The available quantity of the item."]
        pub available_quantity: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "availableTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp when this Product becomes available for recommendation."]
        pub available_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "categories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Product categories. This field is repeated for supporting one product belonging to several parallel categories. Strongly recommended using the full path for better search / recommendation quality. To represent full path of category, use '>' sign to separate different hierarchies. If '>' is part of the category name, please replace it with other character(s). For example, if a shoes product belongs to both [\"Shoes & Accessories\" -> \"Shoes\"] and [\"Sports & Fitness\" -> \"Athletic Clothing\" -> \"Shoes\"], it could be represented as: \"categories\": [ \"Shoes & Accessories > Shoes\", \"Sports & Fitness > Athletic Clothing > Shoes\" ] Must be set for Type.PRIMARY Product otherwise an INVALID_ARGUMENT error is returned. At most 250 values are allowed per Product. Empty values are not allowed. Each value must be a UTF-8 encoded string with a length limit of 5,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Google Merchant Center property google_product_category. Schema.org property [Product.category] (https://schema.org/category). [mc_google_product_category]: https://support.google.com/merchants/answer/6324436"]
        pub categories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Product description. This field must be a UTF-8 encoded string with a length limit of 5,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Google Merchant Center property [description](https://support.google.com/merchants/answer/6324468). schema.org property [Product.description](https://schema.org/description)."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Immutable. Product identifier, which is the final component of name. For example, this field is \"id_1\", if name is `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/id_1`. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. Google Merchant Center property [id](https://support.google.com/merchants/answer/6324405). Schema.org Property [Product.sku](https://schema.org/sku)."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "images")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Product images for the product.Highly recommended to put the main image to the first. A maximum of 300 images are allowed. Google Merchant Center property [image_link](https://support.google.com/merchants/answer/6324350). Schema.org property [Product.image](https://schema.org/image)."]
        pub images:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudRetailV2Image>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Immutable. Full resource name of the product, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/product_id`. The branch ID must be \"default_branch\"."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "priceInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Product price and cost information. Google Merchant Center property [price](https://support.google.com/merchants/answer/6324371)."]
        pub price_info: ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2PriceInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primaryProductId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Variant group identifier. Must be an id, with the same parent branch with this product. Otherwise, an error is thrown. For Type.PRIMARY Products, this field can only be empty or set to the same value as id. For VARIANT Products, this field cannot be empty. A maximum of 2,000 products are allowed to share the same Type.PRIMARY Product. Otherwise, an INVALID_ARGUMENT error is returned. Google Merchant Center Property [item_group_id](https://support.google.com/merchants/answer/6324507). Schema.org Property [Product.inProductGroupWithID](https://schema.org/inProductGroupWithID). This field must be enabled before it can be used. [Learn more](/recommendations-ai/docs/catalog#item-group-id)."]
        pub primary_product_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom tags associated with the product. At most 250 values are allowed per Product. This value must be a UTF-8 encoded string with a length limit of 1,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. This tag can be used for filtering recommendation results by passing the tag as part of the PredictRequest.filter. Google Merchant Center property [custom_label_0–4](https://support.google.com/merchants/answer/6324473)."]
        pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Product title. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. Google Merchant Center property [title](https://support.google.com/merchants/answer/6324415). Schema.org property [Product.name](https://schema.org/name)."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Immutable. The type of the product. This field is output-only."]
        pub _type: ::std::option::Option<GoogleCloudRetailV2ProductTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Canonical URL directly linking to the product detail page. It is strongly recommended to provide a valid uri for the product, otherwise the service performance could be significantly degraded. This field must be a UTF-8 encoded string with a length limit of 5,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Google Merchant Center property [link](https://support.google.com/merchants/answer/6324416). Schema.org property [Offer.url](https://schema.org/url)."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2Product {
        pub fn builder() -> GoogleCloudRetailV2ProductBuilder {
            GoogleCloudRetailV2ProductBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The online availability of the Product. Default to Availability.IN_STOCK. Google Merchant Center Property [availability](https://support.google.com/merchants/answer/6324448). Schema.org Property [Offer.availability](https://schema.org/availability)."]
    pub enum GoogleCloudRetailV2ProductAvailabilityEnum {
        #[serde(rename = "AVAILABILITY_UNSPECIFIED")]
        #[doc = "Default product availability. Default to Availability.IN_STOCK if unset."]
        AvailabilityUnspecified,
        #[serde(rename = "IN_STOCK")]
        #[doc = "Product in stock."]
        InStock,
        #[serde(rename = "OUT_OF_STOCK")]
        #[doc = "Product out of stock."]
        OutOfStock,
        #[serde(rename = "PREORDER")]
        #[doc = "Product that is in pre-order state."]
        Preorder,
        #[serde(rename = "BACKORDER")]
        #[doc = "Product that is back-ordered (i.e. temporarily out of stock)."]
        Backorder,
    }
    impl ::std::default::Default for GoogleCloudRetailV2ProductAvailabilityEnum {
        fn default() -> Self {
            Self::AvailabilityUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Immutable. The type of the product. This field is output-only."]
    pub enum GoogleCloudRetailV2ProductTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "Default value. Default to Type.PRIMARY if unset."]
        TypeUnspecified,
        #[serde(rename = "PRIMARY")]
        #[doc = "The primary type. As the primary unit for predicting, indexing and search serving, a Type.PRIMARY Product is grouped with multiple Type.VARIANT Products."]
        Primary,
        #[serde(rename = "VARIANT")]
        #[doc = "The variant type. Type.VARIANT Products usually share some common attributes on the same Type.PRIMARY Products, but they have variant attributes like different colors, sizes and prices, etc."]
        Variant,
        #[serde(rename = "COLLECTION")]
        #[doc = "The collection type. Collection products are bundled Type.PRIMARY Products or Type.VARIANT Products that are sold together, such as a jewelry set with necklaces, earrings and rings, etc."]
        Collection,
    }
    impl ::std::default::Default for GoogleCloudRetailV2ProductTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detailed product information associated with a user event."]
    pub struct GoogleCloudRetailV2ProductDetail {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "product")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Product information. Only Product.id field is used when ingesting an event, all other product fields are ignored as we will look them up from the catalog."]
        pub product: ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2Product>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Quantity of the product associated with the user event. For example, this field will be 2 if two products are added to the shopping cart for `purchase-complete` event. Required for `add-to-cart` and `purchase-complete` event types."]
        pub quantity: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudRetailV2ProductDetail {
        pub fn builder() -> GoogleCloudRetailV2ProductDetailBuilder {
            GoogleCloudRetailV2ProductDetailBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The inline source for the input config for ImportProducts method."]
    pub struct GoogleCloudRetailV2ProductInlineSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "products")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A list of products to update/create. Each product must have a valid Product.id. Recommended max of 10k items."]
        pub products:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudRetailV2Product>>>,
    }
    impl GoogleCloudRetailV2ProductInlineSource {
        pub fn builder() -> GoogleCloudRetailV2ProductInlineSourceBuilder {
            GoogleCloudRetailV2ProductInlineSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The input config source for products."]
    pub struct GoogleCloudRetailV2ProductInputConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bigQuerySource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "BigQuery input source."]
        pub big_query_source:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2BigQuerySource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage location for the input content."]
        pub gcs_source: ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2GcsSource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productInlineSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Inline source for the input content for products."]
        pub product_inline_source:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2ProductInlineSource>>,
    }
    impl GoogleCloudRetailV2ProductInputConfig {
        pub fn builder() -> GoogleCloudRetailV2ProductInputConfigBuilder {
            GoogleCloudRetailV2ProductInputConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configures what level the product should be uploaded with regards to how users will be send events and how predictions will be made."]
    pub struct GoogleCloudRetailV2ProductLevelConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ingestionProductType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of Products allowed to be ingested into the catalog. Acceptable values are: * `primary` (default): You can only ingest Product.Type.PRIMARY Products. This means Product.primary_product_id can only be empty or set to the same value as Product.id. * `variant`: You can only ingest Product.Type.VARIANT Products. This means Product.primary_product_id cannot be empty. If this field is set to an invalid value other than these, an INVALID_ARGUMENT error is returned. If this field is `variant` and merchant_center_product_id_field is `itemGroupId`, an INVALID_ARGUMENT error is returned. See [Using catalog levels](/retail/recommendations-ai/docs/catalog#catalog-levels) for more details."]
        pub ingestion_product_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "merchantCenterProductIdField")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Which field of [Merchant Center Product](/bigquery-transfer/docs/merchant-center-products-schema) should be imported as Product.id. Acceptable values are: * `offerId` (default): Import `offerId` as the product ID. * `itemGroupId`: Import `itemGroupId` as the product ID. Notice that Retail API will choose one item from the ones with the same `itemGroupId`, and use it to represent the item group. If this field is set to an invalid value other than these, an INVALID_ARGUMENT error is returned. If this field is `itemGroupId` and ingestion_product_type is `variant`, an INVALID_ARGUMENT error is returned. See [Using catalog levels](/retail/recommendations-ai/docs/catalog#catalog-levels) for more details."]
        pub merchant_center_product_id_field: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2ProductLevelConfig {
        pub fn builder() -> GoogleCloudRetailV2ProductLevelConfigBuilder {
            GoogleCloudRetailV2ProductLevelConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A transaction represents the entire purchase transaction."]
    pub struct GoogleCloudRetailV2PurchaseTransaction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cost")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All the costs associated with the products. These can be manufacturing costs, shipping expenses not borne by the end user, or any other costs, such that: * Profit = revenue - tax - cost"]
        pub cost: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currencyCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Currency code. Use three-character ISO-4217 code."]
        pub currency_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The transaction ID with a length limit of 128 characters."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revenue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Total non-zero revenue or grand total associated with the transaction. This value include shipping, tax, or other adjustments to total revenue that you want to include as part of your revenue calculations."]
        pub revenue: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All the taxes associated with the transaction."]
        pub tax: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudRetailV2PurchaseTransaction {
        pub fn builder() -> GoogleCloudRetailV2PurchaseTransactionBuilder {
            GoogleCloudRetailV2PurchaseTransactionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata related to the progress of the Purge operation. This will be returned by the google.longrunning.Operation.metadata field."]
    pub struct GoogleCloudRetailV2PurgeMetadata {}
    impl GoogleCloudRetailV2PurgeMetadata {
        pub fn builder() -> GoogleCloudRetailV2PurgeMetadataBuilder {
            GoogleCloudRetailV2PurgeMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for PurgeUserEvents method."]
    pub struct GoogleCloudRetailV2PurgeUserEventsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The filter string to specify the events to be deleted with a length limit of 5,000 characters. Empty string filter is not allowed. The eligible fields for filtering are: * `eventType`: Double quoted UserEvent.event_type string. * `eventTime`: in ISO 8601 \"zulu\" format. * `visitorId`: Double quoted string. Specifying this will delete all events associated with a visitor. * `userId`: Double quoted string. Specifying this will delete all events associated with a user. Examples: * Deleting all events in a time range: `eventTime > \"2012-04-23T18:25:43.511Z\" eventTime < \"2012-04-23T18:30:43.511Z\"` * Deleting specific eventType in time range: `eventTime > \"2012-04-23T18:25:43.511Z\" eventType = \"detail-page-view\"` * Deleting all events for a specific visitor: `visitorId = \"visitor1024\"` The filtering fields are assumed to have an implicit AND."]
        pub filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "force")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Actually perform the purge. If `force` is set to false, the method will return the expected purge count without deleting any user events."]
        pub force: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudRetailV2PurgeUserEventsRequest {
        pub fn builder() -> GoogleCloudRetailV2PurgeUserEventsRequestBuilder {
            GoogleCloudRetailV2PurgeUserEventsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of the PurgeUserEventsRequest. If the long running operation is successfully done, then this message is returned by the google.longrunning.Operations.response field."]
    pub struct GoogleCloudRetailV2PurgeUserEventsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "purgedEventsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total count of events purged as a result of the operation."]
        pub purged_events_count: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2PurgeUserEventsResponse {
        pub fn builder() -> GoogleCloudRetailV2PurgeUserEventsResponseBuilder {
            GoogleCloudRetailV2PurgeUserEventsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for RejoinUserEvents method."]
    pub struct GoogleCloudRetailV2RejoinUserEventsMetadata {}
    impl GoogleCloudRetailV2RejoinUserEventsMetadata {
        pub fn builder() -> GoogleCloudRetailV2RejoinUserEventsMetadataBuilder {
            GoogleCloudRetailV2RejoinUserEventsMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for RejoinUserEvents method."]
    pub struct GoogleCloudRetailV2RejoinUserEventsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userEventRejoinScope")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the user event rejoin to define the scope and range of the user events to be rejoined with the latest product catalog. Defaults to USER_EVENT_REJOIN_SCOPE_UNSPECIFIED if this field is not set, or set to an invalid integer value."]
        pub user_event_rejoin_scope: ::std::option::Option<
            GoogleCloudRetailV2RejoinUserEventsRequestUserEventRejoinScopeEnum,
        >,
    }
    impl GoogleCloudRetailV2RejoinUserEventsRequest {
        pub fn builder() -> GoogleCloudRetailV2RejoinUserEventsRequestBuilder {
            GoogleCloudRetailV2RejoinUserEventsRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the user event rejoin to define the scope and range of the user events to be rejoined with the latest product catalog. Defaults to USER_EVENT_REJOIN_SCOPE_UNSPECIFIED if this field is not set, or set to an invalid integer value."]
    pub enum GoogleCloudRetailV2RejoinUserEventsRequestUserEventRejoinScopeEnum {
        #[serde(rename = "USER_EVENT_REJOIN_SCOPE_UNSPECIFIED")]
        #[doc = "Rejoin all events with the latest product catalog, including both joined events and unjoined events."]
        UserEventRejoinScopeUnspecified,
        #[serde(rename = "JOINED_EVENTS")]
        #[doc = "Only rejoin joined events with the latest product catalog."]
        JoinedEvents,
        #[serde(rename = "UNJOINED_EVENTS")]
        #[doc = "Only rejoin unjoined events with the latest product catalog."]
        UnjoinedEvents,
    }
    impl ::std::default::Default
        for GoogleCloudRetailV2RejoinUserEventsRequestUserEventRejoinScopeEnum
    {
        fn default() -> Self {
            Self::UserEventRejoinScopeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for RejoinUserEvents method."]
    pub struct GoogleCloudRetailV2RejoinUserEventsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rejoinedUserEventsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of user events that were joined with latest product catalog."]
        pub rejoined_user_events_count: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2RejoinUserEventsResponse {
        pub fn builder() -> GoogleCloudRetailV2RejoinUserEventsResponseBuilder {
            GoogleCloudRetailV2RejoinUserEventsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "UserEvent captures all metadata information Retail API needs to know about how end users interact with customers' website."]
    pub struct GoogleCloudRetailV2UserEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Extra user event features to include in the recommendation model. The key must be a UTF-8 encoded string with a length limit of 5,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. For product recommendation, an example of extra user information is traffic_channel, i.e. how user arrives at the site. Users can arrive at the site by coming to the site directly, or coming through Google search, and etc."]
        pub attributes: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                ::std::boxed::Box<GoogleCloudRetailV2CustomAttribute>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributionToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Highly recommended for user events that are the result of PredictionService.Predict. This field enables accurate attribution of recommendation model performance. The value must be a valid PredictResponse.attribution_token for user events that are the result of PredictionService.Predict. This token enables us to accurately attribute page view or purchase back to the event and the particular predict response containing this clicked/purchased product. If user clicks on product K in the recommendation results, pass PredictResponse.attribution_token as a URL parameter to product K's page. When recording events on product K's page, log the PredictResponse.attribution_token to this field."]
        pub attribution_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cartId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The id or name of the associated shopping cart. This id is used to associate multiple items added or present in the cart before purchase. This can only be set for `add-to-cart`, `purchase-complete`, or `shopping-cart-page-view` events."]
        pub cart_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Only required for UserEventService.ImportUserEvents method. Timestamp of when the user event happened."]
        pub event_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. User event type. Allowed values are: * `add-to-cart`: Products being added to cart. * `category-page-view`: Special pages such as sale or promotion pages viewed. * `detail-page-view`: Products detail page viewed. * `home-page-view`: Homepage viewed. * `promotion-offered`: Promotion is offered to a user. * `promotion-not-offered`: Promotion is not offered to a user. * `purchase-complete`: User finishing a purchase. * `search`: Product search. * `shopping-cart-page-view`: User viewing a shopping cart."]
        pub event_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "experimentIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of identifiers for the independent experiment groups this user event belongs to. This is used to distinguish between user events associated with different experiment setups (e.g. using Retail API, using different recommendation models)."]
        pub experiment_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageCategories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The categories associated with a category page. To represent full path of category, use '>' sign to separate different hierarchies. If '>' is part of the category name, please replace it with other character(s). Category pages include special pages such as sales or promotions. For instance, a special sale page may have the category hierarchy: \"pageCategories\" : [\"Sales > 2017 Black Friday Deals\"]. Required for `category-page-view` events. At least one of search_query or page_categories is required for `search` events. Other event types should not set this field. Otherwise, an INVALID_ARGUMENT error is returned."]
        pub page_categories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageViewId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique id of a web page view. This should be kept the same for all user events triggered from the same pageview. For example, an item detail page view could trigger multiple events as the user is browsing the page. The `pageViewId` property should be kept the same for all these events so that they can be grouped together properly. When using the client side event reporting with JavaScript pixel and Google Tag Manager, this value is filled in automatically."]
        pub page_view_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The main product details related to the event. This field is required for the following event types: * `add-to-cart` * `detail-page-view` * `purchase-complete` In a `search` event, this field represents the products returned to the end user on the current page (the end user may have not finished broswing the whole page yet). When a new page is returned to the end user, after pagination/filtering/ordering even for the same query, a new `search` event with different product_details is desired. The end user may have not finished broswing the whole page yet."]
        pub product_details: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudRetailV2ProductDetail>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "purchaseTransaction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A transaction represents the entire purchase transaction. Required for `purchase-complete` events. Other event types should not set this field. Otherwise, an INVALID_ARGUMENT error is returned."]
        pub purchase_transaction:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2PurchaseTransaction>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "referrerUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The referrer URL of the current page. When using the client side event reporting with JavaScript pixel and Google Tag Manager, this value is filled in automatically."]
        pub referrer_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "searchQuery")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's search query. The value must be a UTF-8 encoded string with a length limit of 5,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. At least one of search_query or page_categories is required for `search` events. Other event types should not set this field. Otherwise, an INVALID_ARGUMENT error is returned."]
        pub search_query: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Complete URL (window.location.href) of the user's current page. When using the client side event reporting with JavaScript pixel and Google Tag Manager, this value is filled in automatically. Maximum length 5,000 characters."]
        pub uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User information."]
        pub user_info: ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2UserInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A unique identifier for tracking visitors. For example, this could be implemented with an HTTP cookie, which should be able to uniquely identify a visitor on a single device. This unique identifier should not change if the visitor log in/out of the website. The field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned."]
        pub visitor_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2UserEvent {
        pub fn builder() -> GoogleCloudRetailV2UserEventBuilder {
            GoogleCloudRetailV2UserEventBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A summary of import result. The UserEventImportSummary summarizes the import status for user events."]
    pub struct GoogleCloudRetailV2UserEventImportSummary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "joinedEventsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Count of user events imported with complete existing catalog information."]
        pub joined_events_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unjoinedEventsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Count of user events imported, but with catalog information not found in the imported catalog."]
        pub unjoined_events_count: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2UserEventImportSummary {
        pub fn builder() -> GoogleCloudRetailV2UserEventImportSummaryBuilder {
            GoogleCloudRetailV2UserEventImportSummaryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The inline source for the input config for ImportUserEvents method."]
    pub struct GoogleCloudRetailV2UserEventInlineSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userEvents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A list of user events to import. Recommended max of 10k items."]
        pub user_events:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleCloudRetailV2UserEvent>>>,
    }
    impl GoogleCloudRetailV2UserEventInlineSource {
        pub fn builder() -> GoogleCloudRetailV2UserEventInlineSourceBuilder {
            GoogleCloudRetailV2UserEventInlineSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The input config source for user events."]
    pub struct GoogleCloudRetailV2UserEventInputConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bigQuerySource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. BigQuery input source."]
        pub big_query_source:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2BigQuerySource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Google Cloud Storage location for the input content."]
        pub gcs_source: ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2GcsSource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userEventInlineSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The Inline source for the input content for UserEvents."]
        pub user_event_inline_source:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2UserEventInlineSource>>,
    }
    impl GoogleCloudRetailV2UserEventInputConfig {
        pub fn builder() -> GoogleCloudRetailV2UserEventInputConfigBuilder {
            GoogleCloudRetailV2UserEventInputConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information of an end user."]
    pub struct GoogleCloudRetailV2UserInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "directUserRequest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the request is made directly from the end user, in which case the ip_address and user_agent can be populated from the HTTP request. This flag should be set only if the API request is made directly from the end user such as a mobile app (and not if a gateway or a server is processing and pushing the user events). This should not be set when using the JavaScript tag in UserEventService.CollectUserEvent."]
        pub direct_user_request: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ipAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end user's IP address. This field is used to extract location information for personalization. This field must be either an IPv4 address (e.g. \"104.133.9.80\") or an IPv6 address (e.g. \"2001:0db8:85a3:0000:0000:8a2e:0370:7334\"). Otherwise, an INVALID_ARGUMENT error is returned. This should not be set when using the JavaScript tag in UserEventService.CollectUserEvent or if direct_user_request is set."]
        pub ip_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userAgent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User agent as included in the HTTP header. The field must be a UTF-8 encoded string with a length limit of 1,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. This should not be set when using the client side event reporting with GTM or JavaScript tag in UserEventService.CollectUserEvent or if direct_user_request is set."]
        pub user_agent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Highly recommended for logged-in users. Unique identifier for logged-in user, such as a user name. The field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned."]
        pub user_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2UserInfo {
        pub fn builder() -> GoogleCloudRetailV2UserInfoBuilder {
            GoogleCloudRetailV2UserInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration of destination for Export related errors."]
    pub struct GoogleCloudRetailV2alphaExportErrorsConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsPrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage path for import errors. This must be an empty, existing Cloud Storage bucket. Export errors will be written to a file in this bucket, one per line, as a JSON-encoded `google.rpc.Status` message."]
        pub gcs_prefix: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2alphaExportErrorsConfig {
        pub fn builder() -> GoogleCloudRetailV2alphaExportErrorsConfigBuilder {
            GoogleCloudRetailV2alphaExportErrorsConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata related to the progress of the Export operation. This will be returned by the google.longrunning.Operation.metadata field."]
    pub struct GoogleCloudRetailV2alphaExportMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Operation create time."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Operation last update time. If the operation is done, this is also the finish time."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2alphaExportMetadata {
        pub fn builder() -> GoogleCloudRetailV2alphaExportMetadataBuilder {
            GoogleCloudRetailV2alphaExportMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of the ExportProductsRequest. If the long running operation is done, then this message is returned by the google.longrunning.Operations.response field if the operation was successful."]
    pub struct GoogleCloudRetailV2alphaExportProductsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorSamples")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A sample of errors encountered while processing the request."]
        pub error_samples:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorsConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Echoes the destination for the complete errors in the request if set."]
        pub errors_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2alphaExportErrorsConfig>>,
    }
    impl GoogleCloudRetailV2alphaExportProductsResponse {
        pub fn builder() -> GoogleCloudRetailV2alphaExportProductsResponseBuilder {
            GoogleCloudRetailV2alphaExportProductsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of the ExportUserEventsRequest. If the long running operation was successful, then this message is returned by the google.longrunning.Operations.response field if the operation was successful."]
    pub struct GoogleCloudRetailV2alphaExportUserEventsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorSamples")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A sample of errors encountered while processing the request."]
        pub error_samples:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorsConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Echoes the destination for the complete errors if this field was set in the request."]
        pub errors_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2alphaExportErrorsConfig>>,
    }
    impl GoogleCloudRetailV2alphaExportUserEventsResponse {
        pub fn builder() -> GoogleCloudRetailV2alphaExportUserEventsResponseBuilder {
            GoogleCloudRetailV2alphaExportUserEventsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration of destination for Import related errors."]
    pub struct GoogleCloudRetailV2alphaImportErrorsConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsPrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage path for import errors. This must be an empty, existing Cloud Storage bucket. Import errors will be written to a file in this bucket, one per line, as a JSON-encoded `google.rpc.Status` message."]
        pub gcs_prefix: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2alphaImportErrorsConfig {
        pub fn builder() -> GoogleCloudRetailV2alphaImportErrorsConfigBuilder {
            GoogleCloudRetailV2alphaImportErrorsConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata related to the progress of the Import operation. This will be returned by the google.longrunning.Operation.metadata field."]
    pub struct GoogleCloudRetailV2alphaImportMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Operation create time."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "failureCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Count of entries that encountered errors while processing."]
        pub failure_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "successCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Count of entries that were processed successfully."]
        pub success_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Operation last update time. If the operation is done, this is also the finish time."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2alphaImportMetadata {
        pub fn builder() -> GoogleCloudRetailV2alphaImportMetadataBuilder {
            GoogleCloudRetailV2alphaImportMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of the ImportProductsRequest. If the long running operation is done, then this message is returned by the google.longrunning.Operations.response field if the operation was successful."]
    pub struct GoogleCloudRetailV2alphaImportProductsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorSamples")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A sample of errors encountered while processing the request."]
        pub error_samples:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorsConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Echoes the destination for the complete errors in the request if set."]
        pub errors_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2alphaImportErrorsConfig>>,
    }
    impl GoogleCloudRetailV2alphaImportProductsResponse {
        pub fn builder() -> GoogleCloudRetailV2alphaImportProductsResponseBuilder {
            GoogleCloudRetailV2alphaImportProductsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of the ImportUserEventsRequest. If the long running operation was successful, then this message is returned by the google.longrunning.Operations.response field if the operation was successful."]
    pub struct GoogleCloudRetailV2alphaImportUserEventsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorSamples")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A sample of errors encountered while processing the request."]
        pub error_samples:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorsConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Echoes the destination for the complete errors if this field was set in the request."]
        pub errors_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2alphaImportErrorsConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importSummary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Aggregated statistics of user event import status."]
        pub import_summary: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudRetailV2alphaUserEventImportSummary>,
        >,
    }
    impl GoogleCloudRetailV2alphaImportUserEventsResponse {
        pub fn builder() -> GoogleCloudRetailV2alphaImportUserEventsResponseBuilder {
            GoogleCloudRetailV2alphaImportUserEventsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata related to the progress of the Purge operation. This will be returned by the google.longrunning.Operation.metadata field."]
    pub struct GoogleCloudRetailV2alphaPurgeMetadata {}
    impl GoogleCloudRetailV2alphaPurgeMetadata {
        pub fn builder() -> GoogleCloudRetailV2alphaPurgeMetadataBuilder {
            GoogleCloudRetailV2alphaPurgeMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of the PurgeUserEventsRequest. If the long running operation is successfully done, then this message is returned by the google.longrunning.Operations.response field."]
    pub struct GoogleCloudRetailV2alphaPurgeUserEventsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "purgedEventsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total count of events purged as a result of the operation."]
        pub purged_events_count: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2alphaPurgeUserEventsResponse {
        pub fn builder() -> GoogleCloudRetailV2alphaPurgeUserEventsResponseBuilder {
            GoogleCloudRetailV2alphaPurgeUserEventsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for RejoinUserEvents method."]
    pub struct GoogleCloudRetailV2alphaRejoinUserEventsMetadata {}
    impl GoogleCloudRetailV2alphaRejoinUserEventsMetadata {
        pub fn builder() -> GoogleCloudRetailV2alphaRejoinUserEventsMetadataBuilder {
            GoogleCloudRetailV2alphaRejoinUserEventsMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for RejoinUserEvents method."]
    pub struct GoogleCloudRetailV2alphaRejoinUserEventsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rejoinedUserEventsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of user events that were joined with latest product catalog."]
        pub rejoined_user_events_count: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2alphaRejoinUserEventsResponse {
        pub fn builder() -> GoogleCloudRetailV2alphaRejoinUserEventsResponseBuilder {
            GoogleCloudRetailV2alphaRejoinUserEventsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A summary of import result. The UserEventImportSummary summarizes the import status for user events."]
    pub struct GoogleCloudRetailV2alphaUserEventImportSummary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "joinedEventsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Count of user events imported with complete existing catalog information."]
        pub joined_events_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unjoinedEventsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Count of user events imported, but with catalog information not found in the imported catalog."]
        pub unjoined_events_count: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2alphaUserEventImportSummary {
        pub fn builder() -> GoogleCloudRetailV2alphaUserEventImportSummaryBuilder {
            GoogleCloudRetailV2alphaUserEventImportSummaryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration of destination for Export related errors."]
    pub struct GoogleCloudRetailV2betaExportErrorsConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsPrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage path for import errors. This must be an empty, existing Cloud Storage bucket. Export errors will be written to a file in this bucket, one per line, as a JSON-encoded `google.rpc.Status` message."]
        pub gcs_prefix: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2betaExportErrorsConfig {
        pub fn builder() -> GoogleCloudRetailV2betaExportErrorsConfigBuilder {
            GoogleCloudRetailV2betaExportErrorsConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata related to the progress of the Export operation. This will be returned by the google.longrunning.Operation.metadata field."]
    pub struct GoogleCloudRetailV2betaExportMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Operation create time."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Operation last update time. If the operation is done, this is also the finish time."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2betaExportMetadata {
        pub fn builder() -> GoogleCloudRetailV2betaExportMetadataBuilder {
            GoogleCloudRetailV2betaExportMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of the ExportProductsRequest. If the long running operation is done, then this message is returned by the google.longrunning.Operations.response field if the operation was successful."]
    pub struct GoogleCloudRetailV2betaExportProductsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorSamples")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A sample of errors encountered while processing the request."]
        pub error_samples:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorsConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Echoes the destination for the complete errors in the request if set."]
        pub errors_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2betaExportErrorsConfig>>,
    }
    impl GoogleCloudRetailV2betaExportProductsResponse {
        pub fn builder() -> GoogleCloudRetailV2betaExportProductsResponseBuilder {
            GoogleCloudRetailV2betaExportProductsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of the ExportUserEventsRequest. If the long running operation was successful, then this message is returned by the google.longrunning.Operations.response field if the operation was successful."]
    pub struct GoogleCloudRetailV2betaExportUserEventsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorSamples")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A sample of errors encountered while processing the request."]
        pub error_samples:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorsConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Echoes the destination for the complete errors if this field was set in the request."]
        pub errors_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2betaExportErrorsConfig>>,
    }
    impl GoogleCloudRetailV2betaExportUserEventsResponse {
        pub fn builder() -> GoogleCloudRetailV2betaExportUserEventsResponseBuilder {
            GoogleCloudRetailV2betaExportUserEventsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration of destination for Import related errors."]
    pub struct GoogleCloudRetailV2betaImportErrorsConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsPrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage path for import errors. This must be an empty, existing Cloud Storage bucket. Import errors will be written to a file in this bucket, one per line, as a JSON-encoded `google.rpc.Status` message."]
        pub gcs_prefix: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2betaImportErrorsConfig {
        pub fn builder() -> GoogleCloudRetailV2betaImportErrorsConfigBuilder {
            GoogleCloudRetailV2betaImportErrorsConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata related to the progress of the Import operation. This will be returned by the google.longrunning.Operation.metadata field."]
    pub struct GoogleCloudRetailV2betaImportMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Operation create time."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "failureCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Count of entries that encountered errors while processing."]
        pub failure_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "successCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Count of entries that were processed successfully."]
        pub success_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Operation last update time. If the operation is done, this is also the finish time."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2betaImportMetadata {
        pub fn builder() -> GoogleCloudRetailV2betaImportMetadataBuilder {
            GoogleCloudRetailV2betaImportMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of the ImportProductsRequest. If the long running operation is done, then this message is returned by the google.longrunning.Operations.response field if the operation was successful."]
    pub struct GoogleCloudRetailV2betaImportProductsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorSamples")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A sample of errors encountered while processing the request."]
        pub error_samples:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorsConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Echoes the destination for the complete errors in the request if set."]
        pub errors_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2betaImportErrorsConfig>>,
    }
    impl GoogleCloudRetailV2betaImportProductsResponse {
        pub fn builder() -> GoogleCloudRetailV2betaImportProductsResponseBuilder {
            GoogleCloudRetailV2betaImportProductsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of the ImportUserEventsRequest. If the long running operation was successful, then this message is returned by the google.longrunning.Operations.response field if the operation was successful."]
    pub struct GoogleCloudRetailV2betaImportUserEventsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorSamples")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A sample of errors encountered while processing the request."]
        pub error_samples:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorsConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Echoes the destination for the complete errors if this field was set in the request."]
        pub errors_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2betaImportErrorsConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importSummary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Aggregated statistics of user event import status."]
        pub import_summary:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2betaUserEventImportSummary>>,
    }
    impl GoogleCloudRetailV2betaImportUserEventsResponse {
        pub fn builder() -> GoogleCloudRetailV2betaImportUserEventsResponseBuilder {
            GoogleCloudRetailV2betaImportUserEventsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata related to the progress of the Purge operation. This will be returned by the google.longrunning.Operation.metadata field."]
    pub struct GoogleCloudRetailV2betaPurgeMetadata {}
    impl GoogleCloudRetailV2betaPurgeMetadata {
        pub fn builder() -> GoogleCloudRetailV2betaPurgeMetadataBuilder {
            GoogleCloudRetailV2betaPurgeMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of the PurgeUserEventsRequest. If the long running operation is successfully done, then this message is returned by the google.longrunning.Operations.response field."]
    pub struct GoogleCloudRetailV2betaPurgeUserEventsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "purgedEventsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total count of events purged as a result of the operation."]
        pub purged_events_count: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2betaPurgeUserEventsResponse {
        pub fn builder() -> GoogleCloudRetailV2betaPurgeUserEventsResponseBuilder {
            GoogleCloudRetailV2betaPurgeUserEventsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for RejoinUserEvents method."]
    pub struct GoogleCloudRetailV2betaRejoinUserEventsMetadata {}
    impl GoogleCloudRetailV2betaRejoinUserEventsMetadata {
        pub fn builder() -> GoogleCloudRetailV2betaRejoinUserEventsMetadataBuilder {
            GoogleCloudRetailV2betaRejoinUserEventsMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for RejoinUserEvents method."]
    pub struct GoogleCloudRetailV2betaRejoinUserEventsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rejoinedUserEventsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of user events that were joined with latest product catalog."]
        pub rejoined_user_events_count: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2betaRejoinUserEventsResponse {
        pub fn builder() -> GoogleCloudRetailV2betaRejoinUserEventsResponseBuilder {
            GoogleCloudRetailV2betaRejoinUserEventsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A summary of import result. The UserEventImportSummary summarizes the import status for user events."]
    pub struct GoogleCloudRetailV2betaUserEventImportSummary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "joinedEventsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Count of user events imported with complete existing catalog information."]
        pub joined_events_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unjoinedEventsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Count of user events imported, but with catalog information not found in the imported catalog."]
        pub unjoined_events_count: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2betaUserEventImportSummary {
        pub fn builder() -> GoogleCloudRetailV2betaUserEventImportSummaryBuilder {
            GoogleCloudRetailV2betaUserEventImportSummaryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration of destination for Export related errors."]
    pub struct GoogleCloudRetailV2mainExportErrorsConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsPrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage path for import errors. This must be an empty, existing Cloud Storage bucket. Export errors will be written to a file in this bucket, one per line, as a JSON-encoded `google.rpc.Status` message."]
        pub gcs_prefix: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2mainExportErrorsConfig {
        pub fn builder() -> GoogleCloudRetailV2mainExportErrorsConfigBuilder {
            GoogleCloudRetailV2mainExportErrorsConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata related to the progress of the Export operation. This will be returned by the google.longrunning.Operation.metadata field."]
    pub struct GoogleCloudRetailV2mainExportMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Operation create time."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Operation last update time. If the operation is done, this is also the finish time."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2mainExportMetadata {
        pub fn builder() -> GoogleCloudRetailV2mainExportMetadataBuilder {
            GoogleCloudRetailV2mainExportMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of the ExportProductsRequest. If the long running operation is done, then this message is returned by the google.longrunning.Operations.response field if the operation was successful."]
    pub struct GoogleCloudRetailV2mainExportProductsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorSamples")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A sample of errors encountered while processing the request."]
        pub error_samples:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorsConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Echoes the destination for the complete errors in the request if set."]
        pub errors_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2mainExportErrorsConfig>>,
    }
    impl GoogleCloudRetailV2mainExportProductsResponse {
        pub fn builder() -> GoogleCloudRetailV2mainExportProductsResponseBuilder {
            GoogleCloudRetailV2mainExportProductsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of the ExportUserEventsRequest. If the long running operation was successful, then this message is returned by the google.longrunning.Operations.response field if the operation was successful."]
    pub struct GoogleCloudRetailV2mainExportUserEventsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorSamples")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A sample of errors encountered while processing the request."]
        pub error_samples:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorsConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Echoes the destination for the complete errors if this field was set in the request."]
        pub errors_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2mainExportErrorsConfig>>,
    }
    impl GoogleCloudRetailV2mainExportUserEventsResponse {
        pub fn builder() -> GoogleCloudRetailV2mainExportUserEventsResponseBuilder {
            GoogleCloudRetailV2mainExportUserEventsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration of destination for Import related errors."]
    pub struct GoogleCloudRetailV2mainImportErrorsConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsPrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Cloud Storage path for import errors. This must be an empty, existing Cloud Storage bucket. Import errors will be written to a file in this bucket, one per line, as a JSON-encoded `google.rpc.Status` message."]
        pub gcs_prefix: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2mainImportErrorsConfig {
        pub fn builder() -> GoogleCloudRetailV2mainImportErrorsConfigBuilder {
            GoogleCloudRetailV2mainImportErrorsConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata related to the progress of the Import operation. This will be returned by the google.longrunning.Operation.metadata field."]
    pub struct GoogleCloudRetailV2mainImportMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Operation create time."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "failureCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Count of entries that encountered errors while processing."]
        pub failure_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "successCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Count of entries that were processed successfully."]
        pub success_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Operation last update time. If the operation is done, this is also the finish time."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2mainImportMetadata {
        pub fn builder() -> GoogleCloudRetailV2mainImportMetadataBuilder {
            GoogleCloudRetailV2mainImportMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of the ImportProductsRequest. If the long running operation is done, then this message is returned by the google.longrunning.Operations.response field if the operation was successful."]
    pub struct GoogleCloudRetailV2mainImportProductsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorSamples")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A sample of errors encountered while processing the request."]
        pub error_samples:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorsConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Echoes the destination for the complete errors in the request if set."]
        pub errors_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2mainImportErrorsConfig>>,
    }
    impl GoogleCloudRetailV2mainImportProductsResponse {
        pub fn builder() -> GoogleCloudRetailV2mainImportProductsResponseBuilder {
            GoogleCloudRetailV2mainImportProductsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of the ImportUserEventsRequest. If the long running operation was successful, then this message is returned by the google.longrunning.Operations.response field if the operation was successful."]
    pub struct GoogleCloudRetailV2mainImportUserEventsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorSamples")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A sample of errors encountered while processing the request."]
        pub error_samples:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorsConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Echoes the destination for the complete errors if this field was set in the request."]
        pub errors_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2mainImportErrorsConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "importSummary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Aggregated statistics of user event import status."]
        pub import_summary:
            ::std::option::Option<::std::boxed::Box<GoogleCloudRetailV2mainUserEventImportSummary>>,
    }
    impl GoogleCloudRetailV2mainImportUserEventsResponse {
        pub fn builder() -> GoogleCloudRetailV2mainImportUserEventsResponseBuilder {
            GoogleCloudRetailV2mainImportUserEventsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata related to the progress of the Purge operation. This will be returned by the google.longrunning.Operation.metadata field."]
    pub struct GoogleCloudRetailV2mainPurgeMetadata {}
    impl GoogleCloudRetailV2mainPurgeMetadata {
        pub fn builder() -> GoogleCloudRetailV2mainPurgeMetadataBuilder {
            GoogleCloudRetailV2mainPurgeMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of the PurgeUserEventsRequest. If the long running operation is successfully done, then this message is returned by the google.longrunning.Operations.response field."]
    pub struct GoogleCloudRetailV2mainPurgeUserEventsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "purgedEventsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total count of events purged as a result of the operation."]
        pub purged_events_count: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2mainPurgeUserEventsResponse {
        pub fn builder() -> GoogleCloudRetailV2mainPurgeUserEventsResponseBuilder {
            GoogleCloudRetailV2mainPurgeUserEventsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for RejoinUserEvents method."]
    pub struct GoogleCloudRetailV2mainRejoinUserEventsMetadata {}
    impl GoogleCloudRetailV2mainRejoinUserEventsMetadata {
        pub fn builder() -> GoogleCloudRetailV2mainRejoinUserEventsMetadataBuilder {
            GoogleCloudRetailV2mainRejoinUserEventsMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for RejoinUserEvents method."]
    pub struct GoogleCloudRetailV2mainRejoinUserEventsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rejoinedUserEventsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of user events that were joined with latest product catalog."]
        pub rejoined_user_events_count: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2mainRejoinUserEventsResponse {
        pub fn builder() -> GoogleCloudRetailV2mainRejoinUserEventsResponseBuilder {
            GoogleCloudRetailV2mainRejoinUserEventsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A summary of import result. The UserEventImportSummary summarizes the import status for user events."]
    pub struct GoogleCloudRetailV2mainUserEventImportSummary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "joinedEventsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Count of user events imported with complete existing catalog information."]
        pub joined_events_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unjoinedEventsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Count of user events imported, but with catalog information not found in the imported catalog."]
        pub unjoined_events_count: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudRetailV2mainUserEventImportSummary {
        pub fn builder() -> GoogleCloudRetailV2mainUserEventImportSummaryBuilder {
            GoogleCloudRetailV2mainUserEventImportSummaryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Operations.ListOperations."]
    pub struct GoogleLongrunningListOperationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The standard List next-page token."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of operations that matches the specified filter in the request."]
        pub operations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleLongrunningOperation>>>,
    }
    impl GoogleLongrunningListOperationsResponse {
        pub fn builder() -> GoogleLongrunningListOperationsResponseBuilder {
            GoogleLongrunningListOperationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This resource represents a long-running operation that is the result of a network API call."]
    pub struct GoogleLongrunningOperation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "done")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
        pub done: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error result of the operation in case of failure or cancellation."]
        pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
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
    impl GoogleLongrunningOperation {
        pub fn builder() -> GoogleLongrunningOperationBuilder {
            GoogleLongrunningOperationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
    pub struct GoogleProtobufEmpty {}
    impl GoogleProtobufEmpty {
        pub fn builder() -> GoogleProtobufEmptyBuilder {
            GoogleProtobufEmptyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
    pub struct GoogleRpcStatus {
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
    impl GoogleRpcStatus {
        pub fn builder() -> GoogleRpcStatusBuilder {
            GoogleRpcStatusBuilder::default()
        }
    }
}
