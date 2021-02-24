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
    pub mod photo {
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
                    #[serde(rename = "languageCode")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier. If language_code is unspecified, the user's language preference for Google services is used."]
                    pub language_code: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "view")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. Specifies if a download URL for the photo bytes should be returned in the Photo response."]
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
                #[doc = "Required. Specifies if a download URL for the photo bytes should be returned in the Photo response."]
                pub enum QueryParametersViewEnum {
                    #[serde(rename = "BASIC")]
                    #[doc = "Server reponses do not include the download URL for the photo bytes. The default value."]
                    Basic,
                    #[serde(rename = "INCLUDE_DOWNLOAD_URL")]
                    #[doc = "Server responses include the download URL for the photo bytes."]
                    IncludeDownloadUrl,
                }
                impl ::std::default::Default for QueryParametersViewEnum {
                    fn default() -> Self {
                        Self::Basic
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "updateMask")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. Mask that identifies fields on the photo metadata to update. If not present, the old Photo metadata is entirely replaced with the new Photo metadata in this request. The update fails if invalid fields are specified. Multiple fields can be specified in a comma-delimited list. The following fields are valid: * `pose.heading` * `pose.latLngPair` * `pose.pitch` * `pose.roll` * `pose.level` * `pose.altitude` * `connections` * `places` *Note:* When updateMask contains repeated fields, the entire set of repeated values get replaced with the new contents. For example, if updateMask contains `connections` and `UpdatePhotoRequest.photo.connections` is empty, all connections are removed."]
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
    pub mod photos {
        pub mod methods {
            pub mod batch_get {
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
                    #[serde(rename = "languageCode")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier. If language_code is unspecified, the user's language preference for Google services is used."]
                    pub language_code: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "photoIds")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. IDs of the Photos. For HTTP GET requests, the URL query parameter should be `photoIds=&photoIds=&...`."]
                    pub photo_ids: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "view")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. Specifies if a download URL for the photo bytes should be returned in the Photo response."]
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
                #[doc = "Required. Specifies if a download URL for the photo bytes should be returned in the Photo response."]
                pub enum QueryParametersViewEnum {
                    #[serde(rename = "BASIC")]
                    #[doc = "Server reponses do not include the download URL for the photo bytes. The default value."]
                    Basic,
                    #[serde(rename = "INCLUDE_DOWNLOAD_URL")]
                    #[doc = "Server responses include the download URL for the photo bytes."]
                    IncludeDownloadUrl,
                }
                impl ::std::default::Default for QueryParametersViewEnum {
                    fn default() -> Self {
                        Self::Basic
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
                    #[serde(rename = "filter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. The filter expression. For example: `placeId=ChIJj61dQgK6j4AR4GeTYWZsKWw`. The only filter supported at the moment is `placeId`."]
                    pub filter: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "languageCode")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier. If language_code is unspecified, the user's language preference for Google services is used."]
                    pub language_code: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of photos to return. `pageSize` must be non-negative. If `pageSize` is zero or is not provided, the default page size of 100 is used. The number of photos returned in the response may be less than `pageSize` if the number of photos that belong to the user is less than `pageSize`."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The nextPageToken value returned from a previous ListPhotos request, if any."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "view")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. Specifies if a download URL for the photos bytes should be returned in the Photos response."]
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
                #[doc = "Required. Specifies if a download URL for the photos bytes should be returned in the Photos response."]
                pub enum QueryParametersViewEnum {
                    #[serde(rename = "BASIC")]
                    #[doc = "Server reponses do not include the download URL for the photo bytes. The default value."]
                    Basic,
                    #[serde(rename = "INCLUDE_DOWNLOAD_URL")]
                    #[doc = "Server responses include the download URL for the photo bytes."]
                    IncludeDownloadUrl,
                }
                impl ::std::default::Default for QueryParametersViewEnum {
                    fn default() -> Self {
                        Self::Basic
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
    #[doc = "Request to delete multiple Photos."]
    pub struct BatchDeletePhotosRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "photoIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. IDs of the Photos. HTTP GET requests require the following syntax for the URL query parameter: `photoIds=&photoIds=&...`."]
        pub photo_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl BatchDeletePhotosRequest {
        pub fn builder() -> BatchDeletePhotosRequestBuilder {
            BatchDeletePhotosRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to batch delete of one or more Photos."]
    pub struct BatchDeletePhotosResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status for the operation to delete a single Photo in the batch request."]
        pub status: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Status>>>,
    }
    impl BatchDeletePhotosResponse {
        pub fn builder() -> BatchDeletePhotosResponseBuilder {
            BatchDeletePhotosResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to batch get of Photos."]
    pub struct BatchGetPhotosResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of results for each individual Photo requested, in the same order as the requests in BatchGetPhotos."]
        pub results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PhotoResponse>>>,
    }
    impl BatchGetPhotosResponse {
        pub fn builder() -> BatchGetPhotosResponseBuilder {
            BatchGetPhotosResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to update the metadata of photos. Updating the pixels of photos is not supported."]
    pub struct BatchUpdatePhotosRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updatePhotoRequests")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. List of UpdatePhotoRequests."]
        pub update_photo_requests:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UpdatePhotoRequest>>>,
    }
    impl BatchUpdatePhotosRequest {
        pub fn builder() -> BatchUpdatePhotosRequestBuilder {
            BatchUpdatePhotosRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to batch update of metadata of one or more Photos."]
    pub struct BatchUpdatePhotosResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of results for each individual Photo updated, in the same order as the request."]
        pub results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PhotoResponse>>>,
    }
    impl BatchUpdatePhotosResponse {
        pub fn builder() -> BatchUpdatePhotosResponseBuilder {
            BatchUpdatePhotosResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A connection is the link from a source photo to a destination photo."]
    pub struct Connection {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "target")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The destination of the connection from the containing photo to another photo."]
        pub target: ::std::option::Option<::std::boxed::Box<PhotoId>>,
    }
    impl Connection {
        pub fn builder() -> ConnectionBuilder {
            ConnectionBuilder::default()
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
    #[doc = "An object that represents a latitude/longitude pair. This is expressed as a pair of doubles to represent degrees latitude and degrees longitude. Unless specified otherwise, this must conform to the WGS84 standard. Values must be within normalized ranges."]
    pub struct LatLng {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "latitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The latitude in degrees. It must be in the range [-90.0, +90.0]."]
        pub latitude: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "longitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The longitude in degrees. It must be in the range [-180.0, +180.0]."]
        pub longitude: ::std::option::Option<::std::primitive::f64>,
    }
    impl LatLng {
        pub fn builder() -> LatLngBuilder {
            LatLngBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Level information containing level number and its corresponding name."]
    pub struct Level {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A name assigned to this Level, restricted to 3 characters. Consider how the elevator buttons would be labeled for this level if there was an elevator."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "number")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Floor number, used for ordering. 0 indicates the ground level, 1 indicates the first level above ground level, -1 indicates the first level under ground level. Non-integer values are OK."]
        pub number: ::std::option::Option<::std::primitive::f64>,
    }
    impl Level {
        pub fn builder() -> LevelBuilder {
            LevelBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to list all photos that belong to a user."]
    pub struct ListPhotosResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "photos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of photos. The pageSize field in the request determines the number of items returned."]
        pub photos: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Photo>>>,
    }
    impl ListPhotosResponse {
        pub fn builder() -> ListPhotosResponseBuilder {
            ListPhotosResponseBuilder::default()
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
    #[doc = "Photo is used to store 360 photos along with photo metadata."]
    pub struct Photo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "captureTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Absolute time when the photo was captured. When the photo has no exif timestamp, this is used to set a timestamp in the photo metadata."]
        pub capture_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "connections")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Connections to other photos. A connection represents the link from this photo to another photo."]
        pub connections: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Connection>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "downloadUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The download URL for the photo bytes. This field is set only when GetPhotoRequest.view is set to PhotoView.INCLUDE_DOWNLOAD_URL."]
        pub download_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mapsPublishStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Status in Google Maps, whether this photo was published or rejected. Not currently populated."]
        pub maps_publish_status: ::std::option::Option<PhotoMapsPublishStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "photoId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required when updating a photo. Output only when creating a photo. Identifier for the photo, which is unique among all photos in Google."]
        pub photo_id: ::std::option::Option<::std::boxed::Box<PhotoId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "places")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Places where this photo belongs."]
        pub places: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Place>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pose")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pose of the photo."]
        pub pose: ::std::option::Option<::std::boxed::Box<Pose>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shareLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The share link for the photo."]
        pub share_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnailUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The thumbnail URL for showing a preview of the given photo."]
        pub thumbnail_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transferStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Status of rights transfer on this photo."]
        pub transfer_status: ::std::option::Option<PhotoTransferStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uploadReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required when creating a photo. Input only. The resource URL where the photo bytes are uploaded to."]
        pub upload_reference: ::std::option::Option<::std::boxed::Box<UploadRef>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "viewCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. View count of the photo."]
        pub view_count: ::std::option::Option<::std::string::String>,
    }
    impl Photo {
        pub fn builder() -> PhotoBuilder {
            PhotoBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Status in Google Maps, whether this photo was published or rejected. Not currently populated."]
    pub enum PhotoMapsPublishStatusEnum {
        #[serde(rename = "UNSPECIFIED_MAPS_PUBLISH_STATUS")]
        #[doc = "The status of the photo is unknown."]
        UnspecifiedMapsPublishStatus,
        #[serde(rename = "PUBLISHED")]
        #[doc = "The photo is published to the public through Google Maps."]
        Published,
        #[serde(rename = "REJECTED_UNKNOWN")]
        #[doc = "The photo has been rejected for an unknown reason."]
        RejectedUnknown,
    }
    impl ::std::default::Default for PhotoMapsPublishStatusEnum {
        fn default() -> Self {
            Self::UnspecifiedMapsPublishStatus
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Status of rights transfer on this photo."]
    pub enum PhotoTransferStatusEnum {
        #[serde(rename = "TRANSFER_STATUS_UNKNOWN")]
        #[doc = "The status of this transfer is unspecified."]
        TransferStatusUnknown,
        #[serde(rename = "NEVER_TRANSFERRED")]
        #[doc = "This photo has never been in a transfer."]
        NeverTransferred,
        #[serde(rename = "PENDING")]
        #[doc = "This photo transfer has been initiated, but the receiver has not yet responded."]
        Pending,
        #[serde(rename = "COMPLETED")]
        #[doc = "The photo transfer has been completed, and this photo has been transferred to the recipient."]
        Completed,
        #[serde(rename = "REJECTED")]
        #[doc = "The recipient rejected this photo transfer."]
        Rejected,
        #[serde(rename = "EXPIRED")]
        #[doc = "The photo transfer expired before the recipient took any action."]
        Expired,
        #[serde(rename = "CANCELLED")]
        #[doc = "The sender cancelled this photo transfer."]
        Cancelled,
        #[serde(rename = "RECEIVED_VIA_TRANSFER")]
        #[doc = "The recipient owns this photo due to a rights transfer."]
        ReceivedViaTransfer,
    }
    impl ::std::default::Default for PhotoTransferStatusEnum {
        fn default() -> Self {
            Self::TransferStatusUnknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Identifier for a Photo."]
    pub struct PhotoId {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A unique identifier for a photo."]
        pub id: ::std::option::Option<::std::string::String>,
    }
    impl PhotoId {
        pub fn builder() -> PhotoIdBuilder {
            PhotoIdBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response payload for a single Photo in batch operations including BatchGetPhotos and BatchUpdatePhotos."]
    pub struct PhotoResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "photo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Photo resource, if the request was successful."]
        pub photo: ::std::option::Option<::std::boxed::Box<Photo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status for the operation to get or update a single photo in the batch request."]
        pub status: ::std::option::Option<::std::boxed::Box<Status>>,
    }
    impl PhotoResponse {
        pub fn builder() -> PhotoResponseBuilder {
            PhotoResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Place metadata for an entity."]
    pub struct Place {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output-only. The language_code that the name is localized with. This should be the language_code specified in the request, but may be a fallback."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output-only. The name of the place, localized to the language_code."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "placeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Place identifier, as described in https://developers.google.com/places/place-id."]
        pub place_id: ::std::option::Option<::std::string::String>,
    }
    impl Place {
        pub fn builder() -> PlaceBuilder {
            PlaceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Raw pose measurement for an entity."]
    pub struct Pose {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accuracyMeters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The estimated horizontal accuracy of this pose in meters with 68% confidence (one standard deviation). For example, on Android, this value is available from this method: https://developer.android.com/reference/android/location/Location#getAccuracy(). Other platforms have different methods of obtaining similar accuracy estimations."]
        pub accuracy_meters: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "altitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Altitude of the pose in meters above WGS84 ellipsoid. NaN indicates an unmeasured quantity."]
        pub altitude: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "heading")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Compass heading, measured at the center of the photo in degrees clockwise from North. Value must be >=0 and <360. NaN indicates an unmeasured quantity."]
        pub heading: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "latLngPair")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Latitude and longitude pair of the pose, as explained here: https://cloud.google.com/datastore/docs/reference/rest/Shared.Types/LatLng When creating a Photo, if the latitude and longitude pair are not provided, the geolocation from the exif header is used. A latitude and longitude pair not provided in the photo or exif header causes the photo process to fail."]
        pub lat_lng_pair: ::std::option::Option<::std::boxed::Box<LatLng>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "level")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Level (the floor in a building) used to configure vertical navigation."]
        pub level: ::std::option::Option<::std::boxed::Box<Level>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pitch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pitch, measured at the center of the photo in degrees. Value must be >=-90 and <= 90. A value of -90 means looking directly down, and a value of 90 means looking directly up. NaN indicates an unmeasured quantity."]
        pub pitch: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "roll")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Roll, measured in degrees. Value must be >= 0 and <360. A value of 0 means level with the horizon. NaN indicates an unmeasured quantity."]
        pub roll: ::std::option::Option<::std::primitive::f64>,
    }
    impl Pose {
        pub fn builder() -> PoseBuilder {
            PoseBuilder::default()
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
    #[doc = "Request to update the metadata of a Photo. Updating the pixels of a photo is not supported."]
    pub struct UpdatePhotoRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "photo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Photo object containing the new metadata."]
        pub photo: ::std::option::Option<::std::boxed::Box<Photo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateMask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Mask that identifies fields on the photo metadata to update. If not present, the old Photo metadata is entirely replaced with the new Photo metadata in this request. The update fails if invalid fields are specified. Multiple fields can be specified in a comma-delimited list. The following fields are valid: * `pose.heading` * `pose.latLngPair` * `pose.pitch` * `pose.roll` * `pose.level` * `pose.altitude` * `connections` * `places` *Note:* When updateMask contains repeated fields, the entire set of repeated values get replaced with the new contents. For example, if updateMask contains `connections` and `UpdatePhotoRequest.photo.connections` is empty, all connections are removed."]
        pub update_mask: ::std::option::Option<::std::string::String>,
    }
    impl UpdatePhotoRequest {
        pub fn builder() -> UpdatePhotoRequestBuilder {
            UpdatePhotoRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Upload reference for media files."]
    pub struct UploadRef {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uploadUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An upload reference should be unique for each user. It follows the form: \"https://streetviewpublish.googleapis.com/media/user/{account_id}/photo/{upload_reference}\""]
        pub upload_url: ::std::option::Option<::std::string::String>,
    }
    impl UploadRef {
        pub fn builder() -> UploadRefBuilder {
            UploadRefBuilder::default()
        }
    }
}
