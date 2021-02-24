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
    pub mod applications {
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
                    #[serde(rename = "customerId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Immutable ID of the Google Workspace account."]
                    pub customer_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Maximum number of results to return. Default is 100."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token to specify next page in the list."]
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
    pub mod transfers {
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
                    #[serde(rename = "customerId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Immutable ID of the Google Workspace account."]
                    pub customer_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Maximum number of results to return. Default is 100."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "newOwnerUserId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Destination user's profile ID."]
                    pub new_owner_user_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "oldOwnerUserId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Source user's profile ID."]
                    pub old_owner_user_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token to specify the next page in the list."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "status")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Status of the transfer."]
                    pub status: ::std::option::Option<::std::string::String>,
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
    #[doc = "Applications resources represent applications installed on the domain that support transferring ownership of user data."]
    pub struct Application {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The application's ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ application_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "application_defaults :: kind")]
        #[doc = "Identifies the resource as a DataTransfer Application Resource."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The application's name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transferParams")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of all possible transfer parameters for this application. These parameters can be used to select the data of the user in this application to be transferred."]
        pub transfer_params:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ApplicationTransferParam>>>,
    }
    impl Application {
        pub fn builder() -> ApplicationBuilder {
            ApplicationBuilder::default()
        }
    }
    mod application_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("admin#datatransfer#ApplicationResource")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Template to map fields of ApplicationDataTransfer resource."]
    pub struct ApplicationDataTransfer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "applicationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The application's ID."]
        pub application_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "applicationTransferParams")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The transfer parameters for the application. These parameters are used to select the data which will get transferred in context of this application."]
        pub application_transfer_params:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ApplicationTransferParam>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "applicationTransferStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Current status of transfer for this application. (Read-only)"]
        pub application_transfer_status: ::std::option::Option<::std::string::String>,
    }
    impl ApplicationDataTransfer {
        pub fn builder() -> ApplicationDataTransferBuilder {
            ApplicationDataTransferBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Template for application transfer parameters."]
    pub struct ApplicationTransferParam {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the transfer parameter. eg: 'PRIVACY_LEVEL'"]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value of the corresponding transfer parameter. eg: 'PRIVATE' or 'SHARED'"]
        pub value: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ApplicationTransferParam {
        pub fn builder() -> ApplicationTransferParamBuilder {
            ApplicationTransferParamBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Template for a collection of Applications."]
    pub struct ApplicationsListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "applications")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of applications that support data transfer and are also installed for the customer."]
        pub applications: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Application>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ applications_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "applications_list_response_defaults :: kind")]
        #[doc = "Identifies the resource as a collection of Applications."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token which will be used to specify next page in list API."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ApplicationsListResponse {
        pub fn builder() -> ApplicationsListResponseBuilder {
            ApplicationsListResponseBuilder::default()
        }
    }
    mod applications_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("admin#datatransfer#applicationsList")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Transfer resource represents the transfer of the ownership of user data between users."]
    pub struct DataTransfer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "applicationDataTransfers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of per application data transfer resources. It contains data transfer details of the applications associated with this transfer resource. Note that this list is also used to specify the applications for which data transfer has to be done at the time of the transfer resource creation."]
        pub application_data_transfers:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ApplicationDataTransfer>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The transfer's ID (Read-only)."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ data_transfer_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "data_transfer_defaults :: kind")]
        #[doc = "Identifies the resource as a DataTransfer request."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newOwnerUserId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the user to whom the data is being transferred."]
        pub new_owner_user_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oldOwnerUserId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the user whose data is being transferred."]
        pub old_owner_user_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "overallTransferStatusCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Overall transfer status (Read-only)."]
        pub overall_transfer_status_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the data transfer was requested (Read-only)."]
        pub request_time: ::std::option::Option<::std::string::String>,
    }
    impl DataTransfer {
        pub fn builder() -> DataTransferBuilder {
            DataTransferBuilder::default()
        }
    }
    mod data_transfer_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("admin#datatransfer#DataTransfer")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Template for a collection of DataTransfer resources."]
    pub struct DataTransfersListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataTransfers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of data transfer requests."]
        pub data_transfers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DataTransfer>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ETag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ data_transfers_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "data_transfers_list_response_defaults :: kind")]
        #[doc = "Identifies the resource as a collection of data transfer requests."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token which will be used to specify next page in list API."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl DataTransfersListResponse {
        pub fn builder() -> DataTransfersListResponseBuilder {
            DataTransfersListResponseBuilder::default()
        }
    }
    mod data_transfers_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("admin#datatransfer#dataTransfersList")
        }
    }
}
