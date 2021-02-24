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
    pub mod devices {
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "customer")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer_id}`, where customer_id is the customer to whom the device belongs."]
                    pub customer: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "customer")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Customer in format: `customers/{customer_id}`, where customer_id is the customer to whom the device belongs."]
                    pub customer: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "customer")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer."]
                    pub customer: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "filter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. Additional restrictions when fetching list of devices. For a list of search fields, refer to [Mobile device search fields](https://developers.google.com/admin-sdk/directory/v1/search-operators). Multiple search fields are separated by the space character."]
                    pub filter: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "orderBy")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. Order specification for devices in the response. Only one of the following field names may be used to specify the order: `create_time`, `last_sync_time`, `model`, `os_version`, `device_type` and `serial_number`. `desc` may be specified optionally to specify results to be sorted in descending order. Default order is ascending."]
                    pub order_by: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. The maximum number of Devices to return. If unspecified, at most 20 Devices will be returned. The maximum value is 100; values above 100 will be coerced to 100."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. A page token, received from a previous `ListDevices` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListDevices` must match the call that provided the page token."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "view")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Optional. The view to use for the List request."]
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
                #[doc = "Optional. The view to use for the List request."]
                pub enum QueryParametersViewEnum {
                    #[serde(rename = "VIEW_UNSPECIFIED")]
                    #[doc = "Default value. The value is unused."]
                    ViewUnspecified,
                    #[serde(rename = "COMPANY_INVENTORY")]
                    #[doc = "This view contains all devices imported by the company admin. Each device in the response contains all information specified by the company admin when importing the device (i.e. asset tags)."]
                    CompanyInventory,
                    #[serde(rename = "USER_ASSIGNED_DEVICES")]
                    #[doc = "This view contains all devices with at least one user registered on the device. Each device in the response contains all device information, except for asset tags."]
                    UserAssignedDevices,
                }
                impl ::std::default::Default for QueryParametersViewEnum {
                    fn default() -> Self {
                        Self::ViewUnspecified
                    }
                }
            }
        }
        pub mod resources {
            pub mod device_users {
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
                            #[serde(rename = "customer")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer_id}`, where customer_id is the customer to whom the device belongs."]
                            pub customer: ::std::option::Option<::std::string::String>,
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
                            #[serde(rename = "customer")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer_id}`, where customer_id is the customer to whom the device belongs."]
                            pub customer: ::std::option::Option<::std::string::String>,
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
                            #[serde(rename = "customer")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer_id}`, where customer_id is the customer to whom the device belongs."]
                            pub customer: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "filter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. Additional restrictions when fetching list of devices. For a list of search fields, refer to [Mobile device search fields](https://developers.google.com/admin-sdk/directory/v1/search-operators). Multiple search fields are separated by the space character."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "orderBy")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. Order specification for devices in the response."]
                            pub order_by: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. The maximum number of DeviceUsers to return. If unspecified, at most 5 DeviceUsers will be returned. The maximum value is 20; values above 20 will be coerced to 20."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional. A page token, received from a previous `ListDeviceUsers` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListBooks` must match the call that provided the page token."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                    pub mod lookup {
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
                            #[serde(rename = "androidId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Android Id returned by [Settings.Secure#ANDROID_ID](https://developer.android.com/reference/android/provider/Settings.Secure.html#ANDROID_ID)."]
                            pub android_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of DeviceUsers to return. If unspecified, at most 20 DeviceUsers will be returned. The maximum value is 20; values above 20 will be coerced to 20."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A page token, received from a previous `LookupDeviceUsers` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `LookupDeviceUsers` must match the call that provided the page token."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "rawResourceId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Raw Resource Id used by Google Endpoint Verification. If the user is enrolled into Google Endpoint Verification, this id will be saved as the 'device_resource_id' field in the following platform dependent files. Mac: ~/.secureConnect/context_aware_config.json Windows: C:\\Users\\%USERPROFILE%\\.secureConnect\\context_aware_config.json Linux: ~/.secureConnect/context_aware_config.json"]
                            pub raw_resource_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "userId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The user whose DeviceUser's resource name will be fetched. Must be set to 'me' to fetch the DeviceUser's resource name for the calling user."]
                            pub user_id: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
                pub mod resources {
                    pub mod client_states {
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
                                    #[serde(rename = "customer")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer_id}`, where customer_id is the customer to whom the device belongs."]
                                    pub customer: ::std::option::Option<::std::string::String>,
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
                                    #[serde(rename = "customer")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer_id}`, where customer_id is the customer to whom the device belongs."]
                                    pub customer: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "updateMask")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional. Comma-separated list of fully qualified names of fields to be updated. If not specified, all updatable fields in ClientState are updated."]
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
        }
    }
    pub mod groups {
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
                    #[serde(rename = "initialGroupConfig")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. The initial configuration option for the `Group`."]
                    pub initial_group_config:
                        ::std::option::Option<QueryParametersInitialGroupConfigEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Required. The initial configuration option for the `Group`."]
                pub enum QueryParametersInitialGroupConfigEnum {
                    #[serde(rename = "INITIAL_GROUP_CONFIG_UNSPECIFIED")]
                    #[doc = "Default. Should not be used."]
                    InitialGroupConfigUnspecified,
                    #[serde(rename = "WITH_INITIAL_OWNER")]
                    #[doc = "The end user making the request will be added as the initial owner of the `Group`."]
                    WithInitialOwner,
                    #[serde(rename = "EMPTY")]
                    #[doc = "An empty group is created without any initial owners. This can only be used by admins of the domain."]
                    Empty,
                }
                impl ::std::default::Default for QueryParametersInitialGroupConfigEnum {
                    fn default() -> Self {
                        Self::InitialGroupConfigUnspecified
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
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of results to return. Note that the number of results returned may be less than this value even if there are more available results. To fetch all results, clients must continue calling this method repeatedly until the response no longer contains a `next_page_token`. If unspecified, defaults to 200 for `View.BASIC` and to 50 for `View.FULL`. Must not be greater than 1000 for `View.BASIC` or 500 for `View.FULL`."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The `next_page_token` value returned from a previous list request, if any."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "parent")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. The parent resource under which to list all `Group`s. Must be of the form `identitysources/{identity_source_id}` for external- identity-mapped groups or `customers/{customer_id}` for Google Groups."]
                    pub parent: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "view")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The level of detail to be returned. If unspecified, defaults to `View.BASIC`."]
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
                #[doc = "The level of detail to be returned. If unspecified, defaults to `View.BASIC`."]
                pub enum QueryParametersViewEnum {
                    #[serde(rename = "VIEW_UNSPECIFIED")]
                    #[doc = "Default. Should not be used."]
                    ViewUnspecified,
                    #[serde(rename = "BASIC")]
                    #[doc = "Only basic resource information is returned."]
                    Basic,
                    #[serde(rename = "FULL")]
                    #[doc = "All resource information is returned."]
                    Full,
                }
                impl ::std::default::Default for QueryParametersViewEnum {
                    fn default() -> Self {
                        Self::ViewUnspecified
                    }
                }
            }
            pub mod lookup {
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
                    #[serde(rename = "groupKey.id")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The ID of the entity. For Google-managed entities, the `id` must be the email address of an existing group or user. For external-identity-mapped entities, the `id` must be a string conforming to the Identity Source's requirements. Must be unique within a `namespace`."]
                    pub group_key_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "groupKey.namespace")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The namespace in which the entity exists. If not specified, the `EntityKey` represents a Google-managed entity such as a Google user or a Google Group. If specified, the `EntityKey` represents an external-identity-mapped group. The namespace must correspond to an identity source created in Admin Console and must be in the form of `identitysources/{identity_source_id}."]
                    pub group_key_namespace: ::std::option::Option<::std::string::String>,
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "updateMask")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. The fully-qualified names of fields to update. May only contain the following fields: `display_name`, `description`."]
                    pub update_mask: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod search {
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
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of results to return. Note that the number of results returned may be less than this value even if there are more available results. To fetch all results, clients must continue calling this method repeatedly until the response no longer contains a `next_page_token`. If unspecified, defaults to 200 for `GroupView.BASIC` and to 50 for `GroupView.FULL`. Must not be greater than 1000 for `GroupView.BASIC` or 500 for `GroupView.FULL`."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The `next_page_token` value returned from a previous search request, if any."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "query")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Required. The search query. Must be specified in [Common Expression Language](https://opensource.google/projects/cel). May only contain equality operators on the parent and inclusion operators on labels (e.g., `parent == 'customers/{customer_id}' && 'cloudidentity.googleapis.com/groups.discussion_forum' in labels`)."]
                    pub query: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "view")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The level of detail to be returned. If unspecified, defaults to `View.BASIC`."]
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
                #[doc = "The level of detail to be returned. If unspecified, defaults to `View.BASIC`."]
                pub enum QueryParametersViewEnum {
                    #[serde(rename = "BASIC")]
                    #[doc = "Default. Only basic resource information is returned."]
                    Basic,
                    #[serde(rename = "FULL")]
                    #[doc = "All resource information is returned."]
                    Full,
                }
                impl ::std::default::Default for QueryParametersViewEnum {
                    fn default() -> Self {
                        Self::Basic
                    }
                }
            }
        }
        pub mod resources {
            pub mod memberships {
                pub mod methods {
                    pub mod check_transitive_membership {
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
                            #[serde(rename = "query")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. A CEL expression that MUST include member specification. This is a `required` field. Certain groups are uniquely identified by both a 'member_key_id' and a 'member_key_namespace', which requires an additional query input: 'member_key_namespace'. Example query: `member_key_id == 'member_key_id_value'`"]
                            pub query: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                    pub mod get_membership_graph {
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
                            #[serde(rename = "query")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. A CEL expression that MUST include member specification AND label(s). Certain groups are uniquely identified by both a 'member_key_id' and a 'member_key_namespace', which requires an additional query input: 'member_key_namespace'. Example query: `member_key_id == 'member_key_id_value' && in labels`"]
                            pub query: ::std::option::Option<::std::string::String>,
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
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of results to return. Note that the number of results returned may be less than this value even if there are more available results. To fetch all results, clients must continue calling this method repeatedly until the response no longer contains a `next_page_token`. If unspecified, defaults to 200 for `GroupView.BASIC` and to 50 for `GroupView.FULL`. Must not be greater than 1000 for `GroupView.BASIC` or 500 for `GroupView.FULL`."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The `next_page_token` value returned from a previous search request, if any."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "view")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The level of detail to be returned. If unspecified, defaults to `MembershipView.BASIC`."]
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
                        #[doc = "The level of detail to be returned. If unspecified, defaults to `MembershipView.BASIC`."]
                        pub enum QueryParametersViewEnum {
                            #[serde(rename = "BASIC")]
                            #[doc = "Default. Only basic resource information is returned."]
                            Basic,
                            #[serde(rename = "FULL")]
                            #[doc = "All resource information is returned."]
                            Full,
                        }
                        impl ::std::default::Default for QueryParametersViewEnum {
                            fn default() -> Self {
                                Self::Basic
                            }
                        }
                    }
                    pub mod lookup {
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
                            #[serde(rename = "memberKey.id")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The ID of the entity. For Google-managed entities, the `id` must be the email address of an existing group or user. For external-identity-mapped entities, the `id` must be a string conforming to the Identity Source's requirements. Must be unique within a `namespace`."]
                            pub member_key_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "memberKey.namespace")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The namespace in which the entity exists. If not specified, the `EntityKey` represents a Google-managed entity such as a Google user or a Google Group. If specified, the `EntityKey` represents an external-identity-mapped group. The namespace must correspond to an identity source created in Admin Console and must be in the form of `identitysources/{identity_source_id}."]
                            pub member_key_namespace: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                    pub mod search_transitive_groups {
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
                            #[doc = "The default page size is 200 (max 1000)."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The next_page_token value returned from a previous list request, if any."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "query")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Required. A CEL expression that MUST include member specification AND label(s). This is a `required` field. Users can search on label attributes of groups. CONTAINS match ('in') is supported on labels. Certain groups are uniquely identified by both a 'member_key_id' and a 'member_key_namespace', which requires an additional query input: 'member_key_namespace'. Example query: `member_key_id == 'member_key_id_value' && in labels`"]
                            pub query: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                    pub mod search_transitive_memberships {
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
                            #[doc = "The default page size is 200 (max 1000)."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The next_page_token value returned from a previous list request, if any."]
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
    #[doc = "Resource representing the Android specific attributes of a Device."]
    pub struct AndroidAttributes {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabledUnknownSources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether applications from unknown sources can be installed on device."]
        pub enabled_unknown_sources: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ownerProfileAccount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this account is on an owner/primary profile. For phones, only true for owner profiles. Android 4+ devices can have secondary or restricted user profiles."]
        pub owner_profile_account: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ownershipPrivilege")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ownership privileges on device."]
        pub ownership_privilege: ::std::option::Option<AndroidAttributesOwnershipPrivilegeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "supportsWorkProfile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether device supports Android work profiles. If false, this service will not block access to corp data even if an administrator turns on the \"Enforce Work Profile\" policy."]
        pub supports_work_profile: ::std::option::Option<::std::primitive::bool>,
    }
    impl AndroidAttributes {
        pub fn builder() -> AndroidAttributesBuilder {
            AndroidAttributesBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Ownership privileges on device."]
    pub enum AndroidAttributesOwnershipPrivilegeEnum {
        #[serde(rename = "OWNERSHIP_PRIVILEGE_UNSPECIFIED")]
        #[doc = "Ownership privilege is not set."]
        OwnershipPrivilegeUnspecified,
        #[serde(rename = "DEVICE_ADMINISTRATOR")]
        #[doc = "Active device administrator privileges on the device."]
        DeviceAdministrator,
        #[serde(rename = "PROFILE_OWNER")]
        #[doc = "Profile Owner privileges. The account is in a managed corporate profile."]
        ProfileOwner,
        #[serde(rename = "DEVICE_OWNER")]
        #[doc = "Device Owner privileges on the device."]
        DeviceOwner,
    }
    impl ::std::default::Default for AndroidAttributesOwnershipPrivilegeEnum {
        fn default() -> Self {
            Self::OwnershipPrivilegeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for approving the device to access user data."]
    pub struct ApproveDeviceUserRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer_id}`, where customer_id is the customer to whom the device belongs."]
        pub customer: ::std::option::Option<::std::string::String>,
    }
    impl ApproveDeviceUserRequest {
        pub fn builder() -> ApproveDeviceUserRequestBuilder {
            ApproveDeviceUserRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for approving the device to access user data."]
    pub struct ApproveDeviceUserResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resultant DeviceUser object for the action."]
        pub device_user: ::std::option::Option<::std::boxed::Box<DeviceUser>>,
    }
    impl ApproveDeviceUserResponse {
        pub fn builder() -> ApproveDeviceUserResponseBuilder {
            ApproveDeviceUserResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for blocking account on device."]
    pub struct BlockDeviceUserRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer_id}`, where customer_id is the customer to whom the device belongs."]
        pub customer: ::std::option::Option<::std::string::String>,
    }
    impl BlockDeviceUserRequest {
        pub fn builder() -> BlockDeviceUserRequestBuilder {
            BlockDeviceUserRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for blocking the device from accessing user data."]
    pub struct BlockDeviceUserResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resultant DeviceUser object for the action."]
        pub device_user: ::std::option::Option<::std::boxed::Box<DeviceUser>>,
    }
    impl BlockDeviceUserResponse {
        pub fn builder() -> BlockDeviceUserResponseBuilder {
            BlockDeviceUserResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for cancelling an unfinished device wipe."]
    pub struct CancelWipeDeviceRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer_id}`, where customer_id is the customer to whom the device belongs."]
        pub customer: ::std::option::Option<::std::string::String>,
    }
    impl CancelWipeDeviceRequest {
        pub fn builder() -> CancelWipeDeviceRequestBuilder {
            CancelWipeDeviceRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for cancelling an unfinished device wipe."]
    pub struct CancelWipeDeviceResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "device")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resultant Device object for the action. Note that asset tags will not be returned in the device object."]
        pub device: ::std::option::Option<::std::boxed::Box<Device>>,
    }
    impl CancelWipeDeviceResponse {
        pub fn builder() -> CancelWipeDeviceResponseBuilder {
            CancelWipeDeviceResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for cancelling an unfinished user account wipe."]
    pub struct CancelWipeDeviceUserRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer_id}`, where customer_id is the customer to whom the device belongs."]
        pub customer: ::std::option::Option<::std::string::String>,
    }
    impl CancelWipeDeviceUserRequest {
        pub fn builder() -> CancelWipeDeviceUserRequestBuilder {
            CancelWipeDeviceUserRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for cancelling an unfinished user account wipe."]
    pub struct CancelWipeDeviceUserResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resultant DeviceUser object for the action."]
        pub device_user: ::std::option::Option<::std::boxed::Box<DeviceUser>>,
    }
    impl CancelWipeDeviceUserResponse {
        pub fn builder() -> CancelWipeDeviceUserResponseBuilder {
            CancelWipeDeviceUserResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for MembershipsService.CheckTransitiveMembership."]
    pub struct CheckTransitiveMembershipResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hasMembership")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Response does not include the possible roles of a member since the behavior of this rpc is not all-or-nothing unlike the other rpcs. So, it may not be possible to list all the roles definitively, due to possible lack of authorization in some of the paths."]
        pub has_membership: ::std::option::Option<::std::primitive::bool>,
    }
    impl CheckTransitiveMembershipResponse {
        pub fn builder() -> CheckTransitiveMembershipResponseBuilder {
            CheckTransitiveMembershipResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the state associated with an API client calling the Devices API. Resource representing ClientState and supports updates from API users"]
    pub struct ClientState {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "assetTags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The caller can specify asset tags for this resource"]
        pub asset_tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "complianceState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The compliance state of the resource as specified by the API client."]
        pub compliance_state: ::std::option::Option<ClientStateComplianceStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time the client state data was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field may be used to store a unique identifier for the API resource within which these CustomAttributes are a field."]
        pub custom_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that needs to be passed back for concurrency control in updates. Token needs to be passed back in UpdateRequest"]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "healthScore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Health score of the resource"]
        pub health_score: ::std::option::Option<ClientStateHealthScoreEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyValuePairs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The map of key-value attributes stored by callers specific to a device. The total serialized length of this map may not exceed 10KB. No limit is placed on the number of attributes in a map."]
        pub key_value_pairs: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<CustomAttributeValue>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastUpdateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time the client state data was last updated."]
        pub last_update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "managed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The management state of the resource as specified by the API client."]
        pub managed: ::std::option::Option<ClientStateManagedEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the ClientState in format: `devices/{device_id}/deviceUsers/{device_user_id}/clientState/{partner_id}`, where partner_id corresponds to the partner storing the data."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ownerType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The owner of the ClientState"]
        pub owner_type: ::std::option::Option<ClientStateOwnerTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scoreReason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A descriptive cause of the health score."]
        pub score_reason: ::std::option::Option<::std::string::String>,
    }
    impl ClientState {
        pub fn builder() -> ClientStateBuilder {
            ClientStateBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The compliance state of the resource as specified by the API client."]
    pub enum ClientStateComplianceStateEnum {
        #[serde(rename = "COMPLIANCE_STATE_UNSPECIFIED")]
        #[doc = "The compliance state of the resource is unknown or unspecified."]
        ComplianceStateUnspecified,
        #[serde(rename = "COMPLIANT")]
        #[doc = "Device is compliant with third party policies"]
        Compliant,
        #[serde(rename = "NON_COMPLIANT")]
        #[doc = "Device is not compliant with third party policies"]
        NonCompliant,
    }
    impl ::std::default::Default for ClientStateComplianceStateEnum {
        fn default() -> Self {
            Self::ComplianceStateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The Health score of the resource"]
    pub enum ClientStateHealthScoreEnum {
        #[serde(rename = "HEALTH_SCORE_UNSPECIFIED")]
        #[doc = "Default value"]
        HealthScoreUnspecified,
        #[serde(rename = "VERY_POOR")]
        #[doc = "The object is in very poor health as defined by the caller."]
        VeryPoor,
        #[serde(rename = "POOR")]
        #[doc = "The object is in poor health as defined by the caller."]
        Poor,
        #[serde(rename = "NEUTRAL")]
        #[doc = "The object health is neither good nor poor, as defined by the caller."]
        Neutral,
        #[serde(rename = "GOOD")]
        #[doc = "The object is in good health as defined by the caller."]
        Good,
        #[serde(rename = "VERY_GOOD")]
        #[doc = "The object is in very good health as defined by the caller."]
        VeryGood,
    }
    impl ::std::default::Default for ClientStateHealthScoreEnum {
        fn default() -> Self {
            Self::HealthScoreUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The management state of the resource as specified by the API client."]
    pub enum ClientStateManagedEnum {
        #[serde(rename = "MANAGED_STATE_UNSPECIFIED")]
        #[doc = "The management state of the resource is unknown or unspecified."]
        ManagedStateUnspecified,
        #[serde(rename = "MANAGED")]
        #[doc = "The resource is managed."]
        Managed,
        #[serde(rename = "UNMANAGED")]
        #[doc = "The resource is not managed."]
        Unmanaged,
    }
    impl ::std::default::Default for ClientStateManagedEnum {
        fn default() -> Self {
            Self::ManagedStateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The owner of the ClientState"]
    pub enum ClientStateOwnerTypeEnum {
        #[serde(rename = "OWNER_TYPE_UNSPECIFIED")]
        #[doc = "Unknown owner type"]
        OwnerTypeUnspecified,
        #[serde(rename = "OWNER_TYPE_CUSTOMER")]
        #[doc = "Customer is the owner"]
        OwnerTypeCustomer,
        #[serde(rename = "OWNER_TYPE_PARTNER")]
        #[doc = "Partner is the owner"]
        OwnerTypePartner,
    }
    impl ::std::default::Default for ClientStateOwnerTypeEnum {
        fn default() -> Self {
            Self::OwnerTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for creating a Company Owned device."]
    pub struct CreateDeviceRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer_id}`, where customer_id is the customer to whom the device belongs."]
        pub customer: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "device")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The device to be created. The name field within this device is ignored in the create method. A new name is created by the method, and returned within the response. Only the fields `device_type`, `serial_number` and `asset_tag` (if present) are used to create the device.`device_type` and `serial_number` are required."]
        pub device: ::std::option::Option<::std::boxed::Box<Device>>,
    }
    impl CreateDeviceRequest {
        pub fn builder() -> CreateDeviceRequestBuilder {
            CreateDeviceRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional custom attribute values may be one of these types"]
    pub struct CustomAttributeValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boolValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a boolean value."]
        pub bool_value: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numberValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a double value."]
        pub number_value: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stringValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a string value."]
        pub string_value: ::std::option::Option<::std::string::String>,
    }
    impl CustomAttributeValue {
        pub fn builder() -> CustomAttributeValueBuilder {
            CustomAttributeValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Device within the Cloud Identity Devices API. Represents a Device known to Google Cloud, independent of the device ownership, type, and whether it is assigned or in use by a user."]
    pub struct Device {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "androidSpecificAttributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Attributes specific to Android devices."]
        pub android_specific_attributes:
            ::std::option::Option<::std::boxed::Box<AndroidAttributes>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "assetTag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Asset tag of the device."]
        pub asset_tag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basebandVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Baseband version of the device."]
        pub baseband_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bootloaderVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Device bootloader version. Example: 0.6.7."]
        pub bootloader_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "brand")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Device brand. Example: Samsung."]
        pub brand: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buildNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Build number of the device."]
        pub build_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "compromisedState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Represents whether the Device is compromised."]
        pub compromised_state: ::std::option::Option<DeviceCompromisedStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. When the Company-Owned device was imported. This field is empty for BYOD devices."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Type of device."]
        pub device_type: ::std::option::Option<DeviceDeviceTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabledDeveloperOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Whether developer options is enabled on device."]
        pub enabled_developer_options: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabledUsbDebugging")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Whether USB debugging is enabled on device."]
        pub enabled_usb_debugging: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "encryptionState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Device encryption state."]
        pub encryption_state: ::std::option::Option<DeviceEncryptionStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imei")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. IMEI number of device if GSM device; empty otherwise."]
        pub imei: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kernelVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Kernel version of the device."]
        pub kernel_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastSyncTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Most recent time when device synced with this service."]
        pub last_sync_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "managementState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Management state of the device"]
        pub management_state: ::std::option::Option<DeviceManagementStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "manufacturer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Device manufacturer. Example: Motorola."]
        pub manufacturer: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "meid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. MEID number of device if CDMA device; empty otherwise."]
        pub meid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "model")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Model name of device. Example: Pixel 3."]
        pub model: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Device in format: `devices/{device_id}`, where device_id is the unique id assigned to the Device."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "networkOperator")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Mobile or network operator of device, if available."]
        pub network_operator: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "osVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. OS version of the device. Example: Android 8.1.0."]
        pub os_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "otherAccounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Domain name for Google accounts on device. Type for other accounts on device. On Android, will only be populated if |ownership_privilege| is |PROFILE_OWNER| or |DEVICE_OWNER|. Does not include the account signed in to the device policy app if that account's domain has only one account. Examples: \"com.example\", \"xyz.com\"."]
        pub other_accounts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ownerType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Whether the device is owned by the company or an individual"]
        pub owner_type: ::std::option::Option<DeviceOwnerTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "releaseVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. OS release version. Example: 6.0."]
        pub release_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "securityPatchTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. OS security patch update time on device."]
        pub security_patch_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serialNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serial Number of device. Example: HT82V1A01076."]
        pub serial_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "wifiMacAddresses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "WiFi MAC addresses of device."]
        pub wifi_mac_addresses: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl Device {
        pub fn builder() -> DeviceBuilder {
            DeviceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Represents whether the Device is compromised."]
    pub enum DeviceCompromisedStateEnum {
        #[serde(rename = "COMPROMISED_STATE_UNSPECIFIED")]
        #[doc = "Default value."]
        CompromisedStateUnspecified,
        #[serde(rename = "COMPROMISED")]
        #[doc = "The device is compromised (currently, this means Android device is rooted)."]
        Compromised,
        #[serde(rename = "UNCOMPROMISED")]
        #[doc = "The device is safe (currently, this means Android device is unrooted)."]
        Uncompromised,
    }
    impl ::std::default::Default for DeviceCompromisedStateEnum {
        fn default() -> Self {
            Self::CompromisedStateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Type of device."]
    pub enum DeviceDeviceTypeEnum {
        #[serde(rename = "DEVICE_TYPE_UNSPECIFIED")]
        #[doc = "Unknown device type"]
        DeviceTypeUnspecified,
        #[serde(rename = "ANDROID")]
        #[doc = "Device is an Android device"]
        Android,
        #[serde(rename = "IOS")]
        #[doc = "Device is an iOS device"]
        Ios,
        #[serde(rename = "GOOGLE_SYNC")]
        #[doc = "Device is a Google Sync device."]
        GoogleSync,
        #[serde(rename = "WINDOWS")]
        #[doc = "Device is a Windows device."]
        Windows,
        #[serde(rename = "MAC_OS")]
        #[doc = "Device is a MacOS device."]
        MacOs,
        #[serde(rename = "LINUX")]
        #[doc = "Device is a Linux device."]
        Linux,
        #[serde(rename = "CHROME_OS")]
        #[doc = "Device is a ChromeOS device."]
        ChromeOs,
    }
    impl ::std::default::Default for DeviceDeviceTypeEnum {
        fn default() -> Self {
            Self::DeviceTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Device encryption state."]
    pub enum DeviceEncryptionStateEnum {
        #[serde(rename = "ENCRYPTION_STATE_UNSPECIFIED")]
        #[doc = "Encryption Status is not set."]
        EncryptionStateUnspecified,
        #[serde(rename = "UNSUPPORTED_BY_DEVICE")]
        #[doc = "Device doesn't support encryption."]
        UnsupportedByDevice,
        #[serde(rename = "ENCRYPTED")]
        #[doc = "Device is encrypted."]
        Encrypted,
        #[serde(rename = "NOT_ENCRYPTED")]
        #[doc = "Device is not encrypted."]
        NotEncrypted,
    }
    impl ::std::default::Default for DeviceEncryptionStateEnum {
        fn default() -> Self {
            Self::EncryptionStateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Management state of the device"]
    pub enum DeviceManagementStateEnum {
        #[serde(rename = "MANAGEMENT_STATE_UNSPECIFIED")]
        #[doc = "Default value. This value is unused."]
        ManagementStateUnspecified,
        #[serde(rename = "APPROVED")]
        #[doc = "Device is approved."]
        Approved,
        #[serde(rename = "BLOCKED")]
        #[doc = "Device is blocked."]
        Blocked,
        #[serde(rename = "PENDING")]
        #[doc = "Device is pending approval."]
        Pending,
        #[serde(rename = "UNPROVISIONED")]
        #[doc = "The device is not provisioned. Device will start from this state until some action is taken (i.e. a user starts using the device)."]
        Unprovisioned,
        #[serde(rename = "WIPING")]
        #[doc = "Data and settings on the device are being removed."]
        Wiping,
        #[serde(rename = "WIPED")]
        #[doc = "All data and settings on the device are removed."]
        Wiped,
    }
    impl ::std::default::Default for DeviceManagementStateEnum {
        fn default() -> Self {
            Self::ManagementStateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Whether the device is owned by the company or an individual"]
    pub enum DeviceOwnerTypeEnum {
        #[serde(rename = "DEVICE_OWNERSHIP_UNSPECIFIED")]
        #[doc = "Default value. The value is unused."]
        DeviceOwnershipUnspecified,
        #[serde(rename = "COMPANY")]
        #[doc = "Company owns the device."]
        Company,
        #[serde(rename = "BYOD")]
        #[doc = "Bring Your Own Device (i.e. individual owns the device)"]
        Byod,
    }
    impl ::std::default::Default for DeviceOwnerTypeEnum {
        fn default() -> Self {
            Self::DeviceOwnershipUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a user's use of a Device in the Cloud Identity Devices API. A DeviceUser is a resource representing a user's use of a Device"]
    pub struct DeviceUser {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "compromisedState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Compromised State of the DeviceUser object"]
        pub compromised_state: ::std::option::Option<DeviceUserCompromisedStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the user first signed in to the device"]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firstSyncTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Most recent time when user registered with this service."]
        pub first_sync_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Default locale used on device, in IETF BCP-47 format."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastSyncTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Last time when user synced with policies."]
        pub last_sync_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "managementState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Management state of the user on the device."]
        pub management_state: ::std::option::Option<DeviceUserManagementStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the DeviceUser in format: `devices/{device_id}/deviceUsers/{user_id}`, where user_id is the ID of the user associated with the user session."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "passwordState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Password state of the DeviceUser object"]
        pub password_state: ::std::option::Option<DeviceUserPasswordStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userAgent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. User agent on the device for this specific user"]
        pub user_agent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email address of the user registered on the device."]
        pub user_email: ::std::option::Option<::std::string::String>,
    }
    impl DeviceUser {
        pub fn builder() -> DeviceUserBuilder {
            DeviceUserBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Compromised State of the DeviceUser object"]
    pub enum DeviceUserCompromisedStateEnum {
        #[serde(rename = "COMPROMISED_STATE_UNSPECIFIED")]
        #[doc = "Compromised state of Device User account is unknown or unspecified."]
        CompromisedStateUnspecified,
        #[serde(rename = "COMPROMISED")]
        #[doc = "Device User Account is compromised."]
        Compromised,
        #[serde(rename = "NOT_COMPROMISED")]
        #[doc = "Device User Account is not compromised."]
        NotCompromised,
    }
    impl ::std::default::Default for DeviceUserCompromisedStateEnum {
        fn default() -> Self {
            Self::CompromisedStateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Management state of the user on the device."]
    pub enum DeviceUserManagementStateEnum {
        #[serde(rename = "MANAGEMENT_STATE_UNSPECIFIED")]
        #[doc = "Default value. This value is unused."]
        ManagementStateUnspecified,
        #[serde(rename = "WIPING")]
        #[doc = "This user's data and profile is being removed from the device."]
        Wiping,
        #[serde(rename = "WIPED")]
        #[doc = "This user's data and profile is removed from the device."]
        Wiped,
        #[serde(rename = "APPROVED")]
        #[doc = "User is approved to access data on the device."]
        Approved,
        #[serde(rename = "BLOCKED")]
        #[doc = "User is blocked from accessing data on the device."]
        Blocked,
        #[serde(rename = "PENDING_APPROVAL")]
        #[doc = "User is awaiting approval."]
        PendingApproval,
        #[serde(rename = "UNENROLLED")]
        #[doc = "User is unenrolled from Advanced Windows Management, but the Windows account is still intact."]
        Unenrolled,
    }
    impl ::std::default::Default for DeviceUserManagementStateEnum {
        fn default() -> Self {
            Self::ManagementStateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Password state of the DeviceUser object"]
    pub enum DeviceUserPasswordStateEnum {
        #[serde(rename = "PASSWORD_STATE_UNSPECIFIED")]
        #[doc = "Password state not set."]
        PasswordStateUnspecified,
        #[serde(rename = "PASSWORD_SET")]
        #[doc = "Password set in object."]
        PasswordSet,
        #[serde(rename = "PASSWORD_NOT_SET")]
        #[doc = "Password not set in object."]
        PasswordNotSet,
    }
    impl ::std::default::Default for DeviceUserPasswordStateEnum {
        fn default() -> Self {
            Self::PasswordStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Dynamic group metadata like queries and status."]
    pub struct DynamicGroupMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Memberships will be the union of all queries. Only one entry with USER resource is currently supported. Customers can create up to 100 dynamic groups."]
        pub queries: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DynamicGroupQuery>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Status of the dynamic group."]
        pub status: ::std::option::Option<::std::boxed::Box<DynamicGroupStatus>>,
    }
    impl DynamicGroupMetadata {
        pub fn builder() -> DynamicGroupMetadataBuilder {
            DynamicGroupMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines a query on a resource."]
    pub struct DynamicGroupQuery {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "query")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Query that determines the memberships of the dynamic group. Examples: All users with at least one `organizations.department` of engineering. `user.organizations.exists(org, org.department=='engineering')` All users with at least one location that has `area` of `foo` and `building_id` of `bar`. `user.locations.exists(loc, loc.area=='foo' && loc.building_id=='bar')`"]
        pub query: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub resource_type: ::std::option::Option<DynamicGroupQueryResourceTypeEnum>,
    }
    impl DynamicGroupQuery {
        pub fn builder() -> DynamicGroupQueryBuilder {
            DynamicGroupQueryBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum DynamicGroupQueryResourceTypeEnum {
        #[serde(rename = "RESOURCE_TYPE_UNSPECIFIED")]
        #[doc = "Default value (not valid)"]
        ResourceTypeUnspecified,
        #[serde(rename = "USER")]
        #[doc = "For queries on User"]
        User,
    }
    impl ::std::default::Default for DynamicGroupQueryResourceTypeEnum {
        fn default() -> Self {
            Self::ResourceTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The current status of a dynamic group along with timestamp."]
    pub struct DynamicGroupStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of the dynamic group."]
        pub status: ::std::option::Option<DynamicGroupStatusStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statusTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The latest time at which the dynamic group is guaranteed to be in the given status. If status is `UP_TO_DATE`, the latest time at which the dynamic group was confirmed to be up-to-date. If status is `UPDATING_MEMBERSHIPS`, the time at which dynamic group was created."]
        pub status_time: ::std::option::Option<::std::string::String>,
    }
    impl DynamicGroupStatus {
        pub fn builder() -> DynamicGroupStatusBuilder {
            DynamicGroupStatusBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Status of the dynamic group."]
    pub enum DynamicGroupStatusStatusEnum {
        #[serde(rename = "STATUS_UNSPECIFIED")]
        #[doc = "Default."]
        StatusUnspecified,
        #[serde(rename = "UP_TO_DATE")]
        #[doc = "The dynamic group is up-to-date."]
        UpToDate,
        #[serde(rename = "UPDATING_MEMBERSHIPS")]
        #[doc = "The dynamic group has just been created and memberships are being updated."]
        UpdatingMemberships,
    }
    impl ::std::default::Default for DynamicGroupStatusStatusEnum {
        fn default() -> Self {
            Self::StatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A unique identifier for an entity in the Cloud Identity Groups API. An entity can represent either a group with an optional `namespace` or a user without a `namespace`. The combination of `id` and `namespace` must be unique; however, the same `id` can be used with different `namespace`s."]
    pub struct EntityKey {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the entity. For Google-managed entities, the `id` must be the email address of an existing group or user. For external-identity-mapped entities, the `id` must be a string conforming to the Identity Source's requirements. Must be unique within a `namespace`."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "namespace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The namespace in which the entity exists. If not specified, the `EntityKey` represents a Google-managed entity such as a Google user or a Google Group. If specified, the `EntityKey` represents an external-identity-mapped group. The namespace must correspond to an identity source created in Admin Console and must be in the form of `identitysources/{identity_source_id}."]
        pub namespace: ::std::option::Option<::std::string::String>,
    }
    impl EntityKey {
        pub fn builder() -> EntityKeyBuilder {
            EntityKeyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The `MembershipRole` expiry details."]
    pub struct ExpiryDetail {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expireTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the `MembershipRole` will expire."]
        pub expire_time: ::std::option::Option<::std::string::String>,
    }
    impl ExpiryDetail {
        pub fn builder() -> ExpiryDetailBuilder {
            ExpiryDetailBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for MembershipsService.GetMembershipGraph."]
    pub struct GetMembershipGraphResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adjacencyList")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The membership graph's path information represented as an adjacency list."]
        pub adjacency_list:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MembershipAdjacencyList>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groups")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resources representing each group in the adjacency list. Each group in this list can be correlated to a 'group' of the MembershipAdjacencyList using the 'name' of the Group resource."]
        pub groups: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Group>>>,
    }
    impl GetMembershipGraphResponse {
        pub fn builder() -> GetMembershipGraphResponseBuilder {
            GetMembershipGraphResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Resource representing the Android specific attributes of a Device."]
    pub struct GoogleAppsCloudidentityDevicesV1AndroidAttributes {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabledUnknownSources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether applications from unknown sources can be installed on device."]
        pub enabled_unknown_sources: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ownerProfileAccount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this account is on an owner/primary profile. For phones, only true for owner profiles. Android 4+ devices can have secondary or restricted user profiles."]
        pub owner_profile_account: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ownershipPrivilege")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ownership privileges on device."]
        pub ownership_privilege: ::std::option::Option<
            GoogleAppsCloudidentityDevicesV1AndroidAttributesOwnershipPrivilegeEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "supportsWorkProfile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether device supports Android work profiles. If false, this service will not block access to corp data even if an administrator turns on the \"Enforce Work Profile\" policy."]
        pub supports_work_profile: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleAppsCloudidentityDevicesV1AndroidAttributes {
        pub fn builder() -> GoogleAppsCloudidentityDevicesV1AndroidAttributesBuilder {
            GoogleAppsCloudidentityDevicesV1AndroidAttributesBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Ownership privileges on device."]
    pub enum GoogleAppsCloudidentityDevicesV1AndroidAttributesOwnershipPrivilegeEnum {
        #[serde(rename = "OWNERSHIP_PRIVILEGE_UNSPECIFIED")]
        #[doc = "Ownership privilege is not set."]
        OwnershipPrivilegeUnspecified,
        #[serde(rename = "DEVICE_ADMINISTRATOR")]
        #[doc = "Active device administrator privileges on the device."]
        DeviceAdministrator,
        #[serde(rename = "PROFILE_OWNER")]
        #[doc = "Profile Owner privileges. The account is in a managed corporate profile."]
        ProfileOwner,
        #[serde(rename = "DEVICE_OWNER")]
        #[doc = "Device Owner privileges on the device."]
        DeviceOwner,
    }
    impl ::std::default::Default
        for GoogleAppsCloudidentityDevicesV1AndroidAttributesOwnershipPrivilegeEnum
    {
        fn default() -> Self {
            Self::OwnershipPrivilegeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for approving the device to access user data."]
    pub struct GoogleAppsCloudidentityDevicesV1ApproveDeviceUserResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resultant DeviceUser object for the action."]
        pub device_user:
            ::std::option::Option<::std::boxed::Box<GoogleAppsCloudidentityDevicesV1DeviceUser>>,
    }
    impl GoogleAppsCloudidentityDevicesV1ApproveDeviceUserResponse {
        pub fn builder() -> GoogleAppsCloudidentityDevicesV1ApproveDeviceUserResponseBuilder {
            GoogleAppsCloudidentityDevicesV1ApproveDeviceUserResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for blocking the device from accessing user data."]
    pub struct GoogleAppsCloudidentityDevicesV1BlockDeviceUserResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resultant DeviceUser object for the action."]
        pub device_user:
            ::std::option::Option<::std::boxed::Box<GoogleAppsCloudidentityDevicesV1DeviceUser>>,
    }
    impl GoogleAppsCloudidentityDevicesV1BlockDeviceUserResponse {
        pub fn builder() -> GoogleAppsCloudidentityDevicesV1BlockDeviceUserResponseBuilder {
            GoogleAppsCloudidentityDevicesV1BlockDeviceUserResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for cancelling an unfinished device wipe."]
    pub struct GoogleAppsCloudidentityDevicesV1CancelWipeDeviceResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "device")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resultant Device object for the action. Note that asset tags will not be returned in the device object."]
        pub device:
            ::std::option::Option<::std::boxed::Box<GoogleAppsCloudidentityDevicesV1Device>>,
    }
    impl GoogleAppsCloudidentityDevicesV1CancelWipeDeviceResponse {
        pub fn builder() -> GoogleAppsCloudidentityDevicesV1CancelWipeDeviceResponseBuilder {
            GoogleAppsCloudidentityDevicesV1CancelWipeDeviceResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for cancelling an unfinished user account wipe."]
    pub struct GoogleAppsCloudidentityDevicesV1CancelWipeDeviceUserResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resultant DeviceUser object for the action."]
        pub device_user:
            ::std::option::Option<::std::boxed::Box<GoogleAppsCloudidentityDevicesV1DeviceUser>>,
    }
    impl GoogleAppsCloudidentityDevicesV1CancelWipeDeviceUserResponse {
        pub fn builder() -> GoogleAppsCloudidentityDevicesV1CancelWipeDeviceUserResponseBuilder {
            GoogleAppsCloudidentityDevicesV1CancelWipeDeviceUserResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the state associated with an API client calling the Devices API. Resource representing ClientState and supports updates from API users"]
    pub struct GoogleAppsCloudidentityDevicesV1ClientState {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "assetTags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The caller can specify asset tags for this resource"]
        pub asset_tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "complianceState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The compliance state of the resource as specified by the API client."]
        pub compliance_state:
            ::std::option::Option<GoogleAppsCloudidentityDevicesV1ClientStateComplianceStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time the client state data was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field may be used to store a unique identifier for the API resource within which these CustomAttributes are a field."]
        pub custom_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that needs to be passed back for concurrency control in updates. Token needs to be passed back in UpdateRequest"]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "healthScore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Health score of the resource. The Health score is the callers specification of the condition of the device from a usability point of view. For example, a third-party device management provider may specify a health score based on its compliance with organizational policies."]
        pub health_score:
            ::std::option::Option<GoogleAppsCloudidentityDevicesV1ClientStateHealthScoreEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keyValuePairs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The map of key-value attributes stored by callers specific to a device. The total serialized length of this map may not exceed 10KB. No limit is placed on the number of attributes in a map."]
        pub key_value_pairs: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                ::std::boxed::Box<GoogleAppsCloudidentityDevicesV1CustomAttributeValue>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastUpdateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time the client state data was last updated."]
        pub last_update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "managed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The management state of the resource as specified by the API client."]
        pub managed: ::std::option::Option<GoogleAppsCloudidentityDevicesV1ClientStateManagedEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the ClientState in format: `devices/{device_id}/deviceUsers/{device_user_id}/clientState/{partner_id}`, where partner_id corresponds to the partner storing the data. For partners belonging to the \"BeyondCorp Alliance\", this is the partner ID specified to you by Google. For all other callers, this is a string of the form: `{customer_id}-suffix`, where `customer_id` is your customer ID. The *suffix* is any string the caller specifies. This string will be displayed verbatim in the administration console. This suffix is used in setting up Custom Access Levels in Context-Aware Access. Your organization's customer ID can be obtained from the URL: `GET https://www.googleapis.com/admin/directory/v1/customers/my_customer` The `id` field in the response contains the customer ID starting with the letter 'C'. The customer ID to be used in this API is the string after the letter 'C' (not including 'C')"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ownerType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The owner of the ClientState"]
        pub owner_type:
            ::std::option::Option<GoogleAppsCloudidentityDevicesV1ClientStateOwnerTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scoreReason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A descriptive cause of the health score."]
        pub score_reason: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAppsCloudidentityDevicesV1ClientState {
        pub fn builder() -> GoogleAppsCloudidentityDevicesV1ClientStateBuilder {
            GoogleAppsCloudidentityDevicesV1ClientStateBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The compliance state of the resource as specified by the API client."]
    pub enum GoogleAppsCloudidentityDevicesV1ClientStateComplianceStateEnum {
        #[serde(rename = "COMPLIANCE_STATE_UNSPECIFIED")]
        #[doc = "The compliance state of the resource is unknown or unspecified."]
        ComplianceStateUnspecified,
        #[serde(rename = "COMPLIANT")]
        #[doc = "Device is compliant with third party policies"]
        Compliant,
        #[serde(rename = "NON_COMPLIANT")]
        #[doc = "Device is not compliant with third party policies"]
        NonCompliant,
    }
    impl ::std::default::Default for GoogleAppsCloudidentityDevicesV1ClientStateComplianceStateEnum {
        fn default() -> Self {
            Self::ComplianceStateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The Health score of the resource. The Health score is the callers specification of the condition of the device from a usability point of view. For example, a third-party device management provider may specify a health score based on its compliance with organizational policies."]
    pub enum GoogleAppsCloudidentityDevicesV1ClientStateHealthScoreEnum {
        #[serde(rename = "HEALTH_SCORE_UNSPECIFIED")]
        #[doc = "Default value"]
        HealthScoreUnspecified,
        #[serde(rename = "VERY_POOR")]
        #[doc = "The object is in very poor health as defined by the caller."]
        VeryPoor,
        #[serde(rename = "POOR")]
        #[doc = "The object is in poor health as defined by the caller."]
        Poor,
        #[serde(rename = "NEUTRAL")]
        #[doc = "The object health is neither good nor poor, as defined by the caller."]
        Neutral,
        #[serde(rename = "GOOD")]
        #[doc = "The object is in good health as defined by the caller."]
        Good,
        #[serde(rename = "VERY_GOOD")]
        #[doc = "The object is in very good health as defined by the caller."]
        VeryGood,
    }
    impl ::std::default::Default for GoogleAppsCloudidentityDevicesV1ClientStateHealthScoreEnum {
        fn default() -> Self {
            Self::HealthScoreUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The management state of the resource as specified by the API client."]
    pub enum GoogleAppsCloudidentityDevicesV1ClientStateManagedEnum {
        #[serde(rename = "MANAGED_STATE_UNSPECIFIED")]
        #[doc = "The management state of the resource is unknown or unspecified."]
        ManagedStateUnspecified,
        #[serde(rename = "MANAGED")]
        #[doc = "The resource is managed."]
        Managed,
        #[serde(rename = "UNMANAGED")]
        #[doc = "The resource is not managed."]
        Unmanaged,
    }
    impl ::std::default::Default for GoogleAppsCloudidentityDevicesV1ClientStateManagedEnum {
        fn default() -> Self {
            Self::ManagedStateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The owner of the ClientState"]
    pub enum GoogleAppsCloudidentityDevicesV1ClientStateOwnerTypeEnum {
        #[serde(rename = "OWNER_TYPE_UNSPECIFIED")]
        #[doc = "Unknown owner type"]
        OwnerTypeUnspecified,
        #[serde(rename = "OWNER_TYPE_CUSTOMER")]
        #[doc = "Customer is the owner"]
        OwnerTypeCustomer,
        #[serde(rename = "OWNER_TYPE_PARTNER")]
        #[doc = "Partner is the owner"]
        OwnerTypePartner,
    }
    impl ::std::default::Default for GoogleAppsCloudidentityDevicesV1ClientStateOwnerTypeEnum {
        fn default() -> Self {
            Self::OwnerTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional custom attribute values may be one of these types"]
    pub struct GoogleAppsCloudidentityDevicesV1CustomAttributeValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boolValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a boolean value."]
        pub bool_value: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numberValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a double value."]
        pub number_value: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stringValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a string value."]
        pub string_value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAppsCloudidentityDevicesV1CustomAttributeValue {
        pub fn builder() -> GoogleAppsCloudidentityDevicesV1CustomAttributeValueBuilder {
            GoogleAppsCloudidentityDevicesV1CustomAttributeValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = " A Device within the Cloud Identity Devices API. Represents a Device known to Google Cloud, independent of the device ownership, type, and whether it is assigned or in use by a user."]
    pub struct GoogleAppsCloudidentityDevicesV1Device {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "androidSpecificAttributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Attributes specific to Android devices."]
        pub android_specific_attributes: ::std::option::Option<
            ::std::boxed::Box<GoogleAppsCloudidentityDevicesV1AndroidAttributes>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "assetTag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Asset tag of the device."]
        pub asset_tag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basebandVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Baseband version of the device."]
        pub baseband_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bootloaderVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Device bootloader version. Example: 0.6.7."]
        pub bootloader_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "brand")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Device brand. Example: Samsung."]
        pub brand: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buildNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Build number of the device."]
        pub build_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "compromisedState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Represents whether the Device is compromised."]
        pub compromised_state:
            ::std::option::Option<GoogleAppsCloudidentityDevicesV1DeviceCompromisedStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. When the Company-Owned device was imported. This field is empty for BYOD devices."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Type of device."]
        pub device_type:
            ::std::option::Option<GoogleAppsCloudidentityDevicesV1DeviceDeviceTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabledDeveloperOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Whether developer options is enabled on device."]
        pub enabled_developer_options: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabledUsbDebugging")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Whether USB debugging is enabled on device."]
        pub enabled_usb_debugging: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "encryptionState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Device encryption state."]
        pub encryption_state:
            ::std::option::Option<GoogleAppsCloudidentityDevicesV1DeviceEncryptionStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imei")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. IMEI number of device if GSM device; empty otherwise."]
        pub imei: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kernelVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Kernel version of the device."]
        pub kernel_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastSyncTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Most recent time when device synced with this service."]
        pub last_sync_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "managementState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Management state of the device"]
        pub management_state:
            ::std::option::Option<GoogleAppsCloudidentityDevicesV1DeviceManagementStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "manufacturer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Device manufacturer. Example: Motorola."]
        pub manufacturer: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "meid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. MEID number of device if CDMA device; empty otherwise."]
        pub meid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "model")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Model name of device. Example: Pixel 3."]
        pub model: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Device in format: `devices/{device_id}`, where device_id is the unique id assigned to the Device."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "networkOperator")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Mobile or network operator of device, if available."]
        pub network_operator: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "osVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. OS version of the device. Example: Android 8.1.0."]
        pub os_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "otherAccounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Domain name for Google accounts on device. Type for other accounts on device. On Android, will only be populated if |ownership_privilege| is |PROFILE_OWNER| or |DEVICE_OWNER|. Does not include the account signed in to the device policy app if that account's domain has only one account. Examples: \"com.example\", \"xyz.com\"."]
        pub other_accounts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ownerType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Whether the device is owned by the company or an individual"]
        pub owner_type: ::std::option::Option<GoogleAppsCloudidentityDevicesV1DeviceOwnerTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "releaseVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. OS release version. Example: 6.0."]
        pub release_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "securityPatchTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. OS security patch update time on device."]
        pub security_patch_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serialNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serial Number of device. Example: HT82V1A01076."]
        pub serial_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "wifiMacAddresses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "WiFi MAC addresses of device."]
        pub wifi_mac_addresses: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleAppsCloudidentityDevicesV1Device {
        pub fn builder() -> GoogleAppsCloudidentityDevicesV1DeviceBuilder {
            GoogleAppsCloudidentityDevicesV1DeviceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Represents whether the Device is compromised."]
    pub enum GoogleAppsCloudidentityDevicesV1DeviceCompromisedStateEnum {
        #[serde(rename = "COMPROMISED_STATE_UNSPECIFIED")]
        #[doc = "Default value."]
        CompromisedStateUnspecified,
        #[serde(rename = "COMPROMISED")]
        #[doc = "The device is compromised (currently, this means Android device is rooted)."]
        Compromised,
        #[serde(rename = "UNCOMPROMISED")]
        #[doc = "The device is safe (currently, this means Android device is unrooted)."]
        Uncompromised,
    }
    impl ::std::default::Default for GoogleAppsCloudidentityDevicesV1DeviceCompromisedStateEnum {
        fn default() -> Self {
            Self::CompromisedStateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Type of device."]
    pub enum GoogleAppsCloudidentityDevicesV1DeviceDeviceTypeEnum {
        #[serde(rename = "DEVICE_TYPE_UNSPECIFIED")]
        #[doc = "Unknown device type"]
        DeviceTypeUnspecified,
        #[serde(rename = "ANDROID")]
        #[doc = "Device is an Android device"]
        Android,
        #[serde(rename = "IOS")]
        #[doc = "Device is an iOS device"]
        Ios,
        #[serde(rename = "GOOGLE_SYNC")]
        #[doc = "Device is a Google Sync device."]
        GoogleSync,
        #[serde(rename = "WINDOWS")]
        #[doc = "Device is a Windows device."]
        Windows,
        #[serde(rename = "MAC_OS")]
        #[doc = "Device is a MacOS device."]
        MacOs,
        #[serde(rename = "LINUX")]
        #[doc = "Device is a Linux device."]
        Linux,
        #[serde(rename = "CHROME_OS")]
        #[doc = "Device is a ChromeOS device."]
        ChromeOs,
    }
    impl ::std::default::Default for GoogleAppsCloudidentityDevicesV1DeviceDeviceTypeEnum {
        fn default() -> Self {
            Self::DeviceTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Device encryption state."]
    pub enum GoogleAppsCloudidentityDevicesV1DeviceEncryptionStateEnum {
        #[serde(rename = "ENCRYPTION_STATE_UNSPECIFIED")]
        #[doc = "Encryption Status is not set."]
        EncryptionStateUnspecified,
        #[serde(rename = "UNSUPPORTED_BY_DEVICE")]
        #[doc = "Device doesn't support encryption."]
        UnsupportedByDevice,
        #[serde(rename = "ENCRYPTED")]
        #[doc = "Device is encrypted."]
        Encrypted,
        #[serde(rename = "NOT_ENCRYPTED")]
        #[doc = "Device is not encrypted."]
        NotEncrypted,
    }
    impl ::std::default::Default for GoogleAppsCloudidentityDevicesV1DeviceEncryptionStateEnum {
        fn default() -> Self {
            Self::EncryptionStateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Management state of the device"]
    pub enum GoogleAppsCloudidentityDevicesV1DeviceManagementStateEnum {
        #[serde(rename = "MANAGEMENT_STATE_UNSPECIFIED")]
        #[doc = "Default value. This value is unused."]
        ManagementStateUnspecified,
        #[serde(rename = "APPROVED")]
        #[doc = "Device is approved."]
        Approved,
        #[serde(rename = "BLOCKED")]
        #[doc = "Device is blocked."]
        Blocked,
        #[serde(rename = "PENDING")]
        #[doc = "Device is pending approval."]
        Pending,
        #[serde(rename = "UNPROVISIONED")]
        #[doc = "The device is not provisioned. Device will start from this state until some action is taken (i.e. a user starts using the device)."]
        Unprovisioned,
        #[serde(rename = "WIPING")]
        #[doc = "Data and settings on the device are being removed."]
        Wiping,
        #[serde(rename = "WIPED")]
        #[doc = "All data and settings on the device are removed."]
        Wiped,
    }
    impl ::std::default::Default for GoogleAppsCloudidentityDevicesV1DeviceManagementStateEnum {
        fn default() -> Self {
            Self::ManagementStateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Whether the device is owned by the company or an individual"]
    pub enum GoogleAppsCloudidentityDevicesV1DeviceOwnerTypeEnum {
        #[serde(rename = "DEVICE_OWNERSHIP_UNSPECIFIED")]
        #[doc = "Default value. The value is unused."]
        DeviceOwnershipUnspecified,
        #[serde(rename = "COMPANY")]
        #[doc = "Company owns the device."]
        Company,
        #[serde(rename = "BYOD")]
        #[doc = "Bring Your Own Device (i.e. individual owns the device)"]
        Byod,
    }
    impl ::std::default::Default for GoogleAppsCloudidentityDevicesV1DeviceOwnerTypeEnum {
        fn default() -> Self {
            Self::DeviceOwnershipUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a user's use of a Device in the Cloud Identity Devices API. A DeviceUser is a resource representing a user's use of a Device"]
    pub struct GoogleAppsCloudidentityDevicesV1DeviceUser {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "compromisedState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Compromised State of the DeviceUser object"]
        pub compromised_state:
            ::std::option::Option<GoogleAppsCloudidentityDevicesV1DeviceUserCompromisedStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the user first signed in to the device"]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firstSyncTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Most recent time when user registered with this service."]
        pub first_sync_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Default locale used on device, in IETF BCP-47 format."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastSyncTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Last time when user synced with policies."]
        pub last_sync_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "managementState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Management state of the user on the device."]
        pub management_state:
            ::std::option::Option<GoogleAppsCloudidentityDevicesV1DeviceUserManagementStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the DeviceUser in format: `devices/{device_id}/deviceUsers/{user_id}`, where user_id is the ID of the user associated with the user session."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "passwordState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Password state of the DeviceUser object"]
        pub password_state:
            ::std::option::Option<GoogleAppsCloudidentityDevicesV1DeviceUserPasswordStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userAgent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. User agent on the device for this specific user"]
        pub user_agent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email address of the user registered on the device."]
        pub user_email: ::std::option::Option<::std::string::String>,
    }
    impl GoogleAppsCloudidentityDevicesV1DeviceUser {
        pub fn builder() -> GoogleAppsCloudidentityDevicesV1DeviceUserBuilder {
            GoogleAppsCloudidentityDevicesV1DeviceUserBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Compromised State of the DeviceUser object"]
    pub enum GoogleAppsCloudidentityDevicesV1DeviceUserCompromisedStateEnum {
        #[serde(rename = "COMPROMISED_STATE_UNSPECIFIED")]
        #[doc = "Compromised state of Device User account is unknown or unspecified."]
        CompromisedStateUnspecified,
        #[serde(rename = "COMPROMISED")]
        #[doc = "Device User Account is compromised."]
        Compromised,
        #[serde(rename = "NOT_COMPROMISED")]
        #[doc = "Device User Account is not compromised."]
        NotCompromised,
    }
    impl ::std::default::Default for GoogleAppsCloudidentityDevicesV1DeviceUserCompromisedStateEnum {
        fn default() -> Self {
            Self::CompromisedStateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Management state of the user on the device."]
    pub enum GoogleAppsCloudidentityDevicesV1DeviceUserManagementStateEnum {
        #[serde(rename = "MANAGEMENT_STATE_UNSPECIFIED")]
        #[doc = "Default value. This value is unused."]
        ManagementStateUnspecified,
        #[serde(rename = "WIPING")]
        #[doc = "This user's data and profile is being removed from the device."]
        Wiping,
        #[serde(rename = "WIPED")]
        #[doc = "This user's data and profile is removed from the device."]
        Wiped,
        #[serde(rename = "APPROVED")]
        #[doc = "User is approved to access data on the device."]
        Approved,
        #[serde(rename = "BLOCKED")]
        #[doc = "User is blocked from accessing data on the device."]
        Blocked,
        #[serde(rename = "PENDING_APPROVAL")]
        #[doc = "User is awaiting approval."]
        PendingApproval,
        #[serde(rename = "UNENROLLED")]
        #[doc = "User is unenrolled from Advanced Windows Management, but the Windows account is still intact."]
        Unenrolled,
    }
    impl ::std::default::Default for GoogleAppsCloudidentityDevicesV1DeviceUserManagementStateEnum {
        fn default() -> Self {
            Self::ManagementStateUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Password state of the DeviceUser object"]
    pub enum GoogleAppsCloudidentityDevicesV1DeviceUserPasswordStateEnum {
        #[serde(rename = "PASSWORD_STATE_UNSPECIFIED")]
        #[doc = "Password state not set."]
        PasswordStateUnspecified,
        #[serde(rename = "PASSWORD_SET")]
        #[doc = "Password set in object."]
        PasswordSet,
        #[serde(rename = "PASSWORD_NOT_SET")]
        #[doc = "Password not set in object."]
        PasswordNotSet,
    }
    impl ::std::default::Default for GoogleAppsCloudidentityDevicesV1DeviceUserPasswordStateEnum {
        fn default() -> Self {
            Self::PasswordStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for wiping all data on the device."]
    pub struct GoogleAppsCloudidentityDevicesV1WipeDeviceResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "device")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resultant Device object for the action. Note that asset tags will not be returned in the device object."]
        pub device:
            ::std::option::Option<::std::boxed::Box<GoogleAppsCloudidentityDevicesV1Device>>,
    }
    impl GoogleAppsCloudidentityDevicesV1WipeDeviceResponse {
        pub fn builder() -> GoogleAppsCloudidentityDevicesV1WipeDeviceResponseBuilder {
            GoogleAppsCloudidentityDevicesV1WipeDeviceResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for wiping the user's account from the device."]
    pub struct GoogleAppsCloudidentityDevicesV1WipeDeviceUserResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resultant DeviceUser object for the action."]
        pub device_user:
            ::std::option::Option<::std::boxed::Box<GoogleAppsCloudidentityDevicesV1DeviceUser>>,
    }
    impl GoogleAppsCloudidentityDevicesV1WipeDeviceUserResponse {
        pub fn builder() -> GoogleAppsCloudidentityDevicesV1WipeDeviceUserResponseBuilder {
            GoogleAppsCloudidentityDevicesV1WipeDeviceUserResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A group within the Cloud Identity Groups API. A `Group` is a collection of entities, where each entity is either a user, another group, or a service account."]
    pub struct Group {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalGroupKeys")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional entity key aliases for a Group."]
        pub additional_group_keys:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EntityKey>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time when the `Group` was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An extended description to help users determine the purpose of a `Group`. Must not be longer than 4,096 characters."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The display name of the `Group`."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dynamicGroupMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Dynamic group metadata like queries and status."]
        pub dynamic_group_metadata: ::std::option::Option<::std::boxed::Box<DynamicGroupMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groupKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Immutable. The `EntityKey` of the `Group`."]
        pub group_key: ::std::option::Option<::std::boxed::Box<EntityKey>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. One or more label entries that apply to the Group. Currently supported labels contain a key with an empty value. Google Groups are the default type of group and have a label with a key of `cloudidentity.googleapis.com/groups.discussion_forum` and an empty value. Existing Google Groups can have an additional label with a key of `cloudidentity.googleapis.com/groups.security` and an empty value added to them. **This is an immutable change and the security label cannot be removed once added.** Dynamic groups have a label with a key of `cloudidentity.googleapis.com/groups.dynamic`. Identity-mapped groups for Cloud Search have a label with a key of `system/groups/external` and an empty value. Examples: {\"cloudidentity.googleapis.com/groups.discussion_forum\": \"\"} or {\"system/groups/external\": \"\"}."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The [resource name](https://cloud.google.com/apis/design/resource_names) of the `Group`. Shall be of the form `groups/{group_id}`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Immutable. The resource name of the entity under which this `Group` resides in the Cloud Identity resource hierarchy. Must be of the form `identitysources/{identity_source_id}` for external- identity-mapped groups or `customers/{customer_id}` for Google Groups."]
        pub parent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time when the `Group` was last updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl Group {
        pub fn builder() -> GroupBuilder {
            GroupBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message representing a transitive group of a user or a group."]
    pub struct GroupRelation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display name for this group."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "group")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name for this group."]
        pub group: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groupKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entity key has an id and a namespace. In case of discussion forums, the id will be an email address without a namespace."]
        pub group_key: ::std::option::Option<::std::boxed::Box<EntityKey>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Labels for Group resource."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The relation between the member and the transitive group."]
        pub relation_type: ::std::option::Option<GroupRelationRelationTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "roles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Membership roles of the member for the group."]
        pub roles:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TransitiveMembershipRole>>>,
    }
    impl GroupRelation {
        pub fn builder() -> GroupRelationBuilder {
            GroupRelationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The relation between the member and the transitive group."]
    pub enum GroupRelationRelationTypeEnum {
        #[serde(rename = "RELATION_TYPE_UNSPECIFIED")]
        #[doc = "The relation type is undefined or undetermined."]
        RelationTypeUnspecified,
        #[serde(rename = "DIRECT")]
        #[doc = "The two entities have only a direct membership with each other."]
        Direct,
        #[serde(rename = "INDIRECT")]
        #[doc = "The two entities have only an indirect membership with each other."]
        Indirect,
        #[serde(rename = "DIRECT_AND_INDIRECT")]
        #[doc = "The two entities have both a direct and an indirect membership with each other."]
        DirectAndIndirect,
    }
    impl ::std::default::Default for GroupRelationRelationTypeEnum {
        fn default() -> Self {
            Self::RelationTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message that is returned in LRO result of ListClientStates Operation."]
    pub struct ListClientStatesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientStates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Client states meeting the list restrictions."]
        pub client_states: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ClientState>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results. Empty if there are no more results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListClientStatesResponse {
        pub fn builder() -> ListClientStatesResponseBuilder {
            ListClientStatesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message that is returned from the ListDeviceUsers method."]
    pub struct ListDeviceUsersResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceUsers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Devices meeting the list restrictions."]
        pub device_users: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DeviceUser>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results. Empty if there are no more results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListDeviceUsersResponse {
        pub fn builder() -> ListDeviceUsersResponseBuilder {
            ListDeviceUsersResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message that is returned from the ListDevices method."]
    pub struct ListDevicesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "devices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Devices meeting the list restrictions."]
        pub devices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Device>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results. Empty if there are no more results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListDevicesResponse {
        pub fn builder() -> ListDevicesResponseBuilder {
            ListDevicesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for GroupsService.ListGroups."]
    pub struct ListGroupsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groups")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `Group`s under the specified `parent`."]
        pub groups: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Group>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A continuation token to retrieve the next page of results, or empty if there are no more results available."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListGroupsResponse {
        pub fn builder() -> ListGroupsResponseBuilder {
            ListGroupsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for MembershipsService.ListMemberships."]
    pub struct ListMembershipsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "memberships")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `Membership`s under the specified `parent`."]
        pub memberships: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Membership>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A continuation token to retrieve the next page of results, or empty if there are no more results available."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListMembershipsResponse {
        pub fn builder() -> ListMembershipsResponseBuilder {
            ListMembershipsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for GroupsService.LookupGroupName."]
    pub struct LookupGroupNameResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The [resource name](https://cloud.google.com/apis/design/resource_names) of the looked-up `Group`."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl LookupGroupNameResponse {
        pub fn builder() -> LookupGroupNameResponseBuilder {
            LookupGroupNameResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for MembershipsService.LookupMembershipName."]
    pub struct LookupMembershipNameResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The [resource name](https://cloud.google.com/apis/design/resource_names) of the looked-up `Membership`. Must be of the form `groups/{group_id}/memberships/{membership_id}`."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl LookupMembershipNameResponse {
        pub fn builder() -> LookupMembershipNameResponseBuilder {
            LookupMembershipNameResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response containing resource names of the DeviceUsers associated with the caller's credentials."]
    pub struct LookupSelfDeviceUsersResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The obfuscated customer Id that may be passed back to other Devices API methods such as List, Get, etc."]
        pub customer: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "names")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Resource names](https://cloud.google.com/apis/design/resource_names) of the DeviceUsers in the format: `devices/{device_id}/deviceUsers/{user_resource_id}`, where device_id is the unique ID assigned to a Device and user_resource_id is the unique user ID"]
        pub names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results. Empty if there are no more results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl LookupSelfDeviceUsersResponse {
        pub fn builder() -> LookupSelfDeviceUsersResponseBuilder {
            LookupSelfDeviceUsersResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message representing a transitive membership of a group."]
    pub struct MemberRelation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "member")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name for this member if member is a GROUP, otherwise it is empty."]
        pub member: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "preferredMemberKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entity key has an id and a namespace. In case of discussion forums, the id will be an email address without a namespace."]
        pub preferred_member_key:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EntityKey>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relationType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The relation between the group and the transitive member."]
        pub relation_type: ::std::option::Option<MemberRelationRelationTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "roles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The membership role details (i.e name of role and expiry time)."]
        pub roles:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TransitiveMembershipRole>>>,
    }
    impl MemberRelation {
        pub fn builder() -> MemberRelationBuilder {
            MemberRelationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The relation between the group and the transitive member."]
    pub enum MemberRelationRelationTypeEnum {
        #[serde(rename = "RELATION_TYPE_UNSPECIFIED")]
        #[doc = "The relation type is undefined or undetermined."]
        RelationTypeUnspecified,
        #[serde(rename = "DIRECT")]
        #[doc = "The two entities have only a direct membership with each other."]
        Direct,
        #[serde(rename = "INDIRECT")]
        #[doc = "The two entities have only an indirect membership with each other."]
        Indirect,
        #[serde(rename = "DIRECT_AND_INDIRECT")]
        #[doc = "The two entities have both a direct and an indirect membership with each other."]
        DirectAndIndirect,
    }
    impl ::std::default::Default for MemberRelationRelationTypeEnum {
        fn default() -> Self {
            Self::RelationTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A membership within the Cloud Identity Groups API. A `Membership` defines a relationship between a `Group` and an entity belonging to that `Group`, referred to as a \"member\"."]
    pub struct Membership {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time when the `Membership` was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "memberKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Immutable. The `EntityKey` of the member. Either `member_key` or `preferred_member_key` must be set when calling MembershipsService.CreateMembership but not both; both shall be set when returned."]
        pub member_key: ::std::option::Option<::std::boxed::Box<EntityKey>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The [resource name](https://cloud.google.com/apis/design/resource_names) of the `Membership`. Shall be of the form `groups/{group_id}/memberships/{membership_id}`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "preferredMemberKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Immutable. The `EntityKey` of the member. Either `member_key` or `preferred_member_key` must be set when calling MembershipsService.CreateMembership but not both; both shall be set when returned."]
        pub preferred_member_key: ::std::option::Option<::std::boxed::Box<EntityKey>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "roles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `MembershipRole`s that apply to the `Membership`. If unspecified, defaults to a single `MembershipRole` with `name` `MEMBER`. Must not contain duplicate `MembershipRole`s with the same `name`."]
        pub roles: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MembershipRole>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The type of the membership."]
        pub _type: ::std::option::Option<MembershipTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time when the `Membership` was last updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl Membership {
        pub fn builder() -> MembershipBuilder {
            MembershipBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The type of the membership."]
    pub enum MembershipTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "Default. Should not be used."]
        TypeUnspecified,
        #[serde(rename = "USER")]
        #[doc = "Represents user type."]
        User,
        #[serde(rename = "SERVICE_ACCOUNT")]
        #[doc = "Represents service account type."]
        ServiceAccount,
        #[serde(rename = "GROUP")]
        #[doc = "Represents group type."]
        Group,
        #[serde(rename = "OTHER")]
        #[doc = "Represents other type."]
        Other,
    }
    impl ::std::default::Default for MembershipTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Membership graph's path information as an adjacency list."]
    pub struct MembershipAdjacencyList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "edges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Each edge contains information about the member that belongs to this group. Note: Fields returned here will help identify the specific Membership resource (e.g name, preferred_member_key and role), but may not be a comprehensive list of all fields."]
        pub edges: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Membership>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "group")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name of the group that the members belong to."]
        pub group: ::std::option::Option<::std::string::String>,
    }
    impl MembershipAdjacencyList {
        pub fn builder() -> MembershipAdjacencyListBuilder {
            MembershipAdjacencyListBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A membership role within the Cloud Identity Groups API. A `MembershipRole` defines the privileges granted to a `Membership`."]
    pub struct MembershipRole {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expiryDetail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The expiry details of the `MembershipRole`. Expiry details are only supported for `MEMBER` `MembershipRoles`. May be set if `name` is `MEMBER`. Must not be set if `name` is any other value."]
        pub expiry_detail: ::std::option::Option<::std::boxed::Box<ExpiryDetail>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the `MembershipRole`. Must be one of `OWNER`, `MANAGER`, `MEMBER`."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl MembershipRole {
        pub fn builder() -> MembershipRoleBuilder {
            MembershipRoleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for MembershipsService.ModifyMembershipRoles."]
    pub struct ModifyMembershipRolesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addRoles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `MembershipRole`s to be added. Adding or removing roles in the same request as updating roles is not supported. Must not be set if `update_roles_params` is set."]
        pub add_roles: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MembershipRole>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "removeRoles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `name`s of the `MembershipRole`s to be removed. Adding or removing roles in the same request as updating roles is not supported. It is not possible to remove the `MEMBER` `MembershipRole`. If you wish to delete a `Membership`, call MembershipsService.DeleteMembership instead. Must not contain `MEMBER`. Must not be set if `update_roles_params` is set."]
        pub remove_roles: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateRolesParams")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `MembershipRole`s to be updated. Updating roles in the same request as adding or removing roles is not supported. Must not be set if either `add_roles` or `remove_roles` is set."]
        pub update_roles_params:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UpdateMembershipRolesParams>>>,
    }
    impl ModifyMembershipRolesRequest {
        pub fn builder() -> ModifyMembershipRolesRequestBuilder {
            ModifyMembershipRolesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for MembershipsService.ModifyMembershipRoles."]
    pub struct ModifyMembershipRolesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "membership")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `Membership` resource after modifying its `MembershipRole`s."]
        pub membership: ::std::option::Option<::std::boxed::Box<Membership>>,
    }
    impl ModifyMembershipRolesResponse {
        pub fn builder() -> ModifyMembershipRolesResponseBuilder {
            ModifyMembershipRolesResponseBuilder::default()
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
    #[doc = "The response message for GroupsService.SearchGroups."]
    pub struct SearchGroupsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groups")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `Group`s that match the search query."]
        pub groups: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Group>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A continuation token to retrieve the next page of results, or empty if there are no more results available."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl SearchGroupsResponse {
        pub fn builder() -> SearchGroupsResponseBuilder {
            SearchGroupsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for MembershipsService.SearchTransitiveGroups."]
    pub struct SearchTransitiveGroupsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "memberships")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of transitive groups satisfying the query."]
        pub memberships: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GroupRelation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results available for listing."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl SearchTransitiveGroupsResponse {
        pub fn builder() -> SearchTransitiveGroupsResponseBuilder {
            SearchTransitiveGroupsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for MembershipsService.SearchTransitiveMemberships."]
    pub struct SearchTransitiveMembershipsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "memberships")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of transitive members satisfying the query."]
        pub memberships: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MemberRelation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl SearchTransitiveMembershipsResponse {
        pub fn builder() -> SearchTransitiveMembershipsResponseBuilder {
            SearchTransitiveMembershipsResponseBuilder::default()
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
    #[doc = "Message representing the role of a TransitiveMembership."]
    pub struct TransitiveMembershipRole {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "TransitiveMembershipRole in string format. Currently supported TransitiveMembershipRoles: `\"MEMBER\"`, `\"OWNER\"`, and `\"MANAGER\"`."]
        pub role: ::std::option::Option<::std::string::String>,
    }
    impl TransitiveMembershipRole {
        pub fn builder() -> TransitiveMembershipRoleBuilder {
            TransitiveMembershipRoleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The details of an update to a `MembershipRole`."]
    pub struct UpdateMembershipRolesParams {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fieldMask")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fully-qualified names of fields to update. May only contain the field `expiry_detail`."]
        pub field_mask: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "membershipRole")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `MembershipRole`s to be updated. Only `MEMBER` `MembershipRoles` can currently be updated. May only contain a `MembershipRole` with `name` `MEMBER`."]
        pub membership_role: ::std::option::Option<::std::boxed::Box<MembershipRole>>,
    }
    impl UpdateMembershipRolesParams {
        pub fn builder() -> UpdateMembershipRolesParamsBuilder {
            UpdateMembershipRolesParamsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The `UserInvitation` resource represents an email sent to an unmanaged user account (an email address that shares the domain of the Google Workspace customer but is not managed by it yet), inviting them to join the customer’s domain. If the user accepts the `UserInvitation`, the account will become a managed account."]
    pub struct UserInvitation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mailsSentCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of invitation emails sent to the user."]
        pub mails_sent_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Shall be of the form `customers/{customer}/userinvitations/{user_email_address}`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "State of the `UserInvitation`."]
        pub state: ::std::option::Option<UserInvitationStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time when the `UserInvitation` was last updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl UserInvitation {
        pub fn builder() -> UserInvitationBuilder {
            UserInvitationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "State of the `UserInvitation`."]
    pub enum UserInvitationStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "The default value. This value is used if the state is omitted."]
        StateUnspecified,
        #[serde(rename = "NOT_YET_SENT")]
        #[doc = "The `UserInvitation` has been created and is ready for sending as an email."]
        NotYetSent,
        #[serde(rename = "INVITED")]
        #[doc = "The user has been invited by email."]
        Invited,
        #[serde(rename = "ACCEPTED")]
        #[doc = "The user has accepted the invitation and is part of the organization."]
        Accepted,
        #[serde(rename = "DECLINED")]
        #[doc = "The user declined the invitation."]
        Declined,
    }
    impl ::std::default::Default for UserInvitationStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for wiping all data on the device."]
    pub struct WipeDeviceRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer_id}`, where customer_id is the customer to whom the device belongs."]
        pub customer: ::std::option::Option<::std::string::String>,
    }
    impl WipeDeviceRequest {
        pub fn builder() -> WipeDeviceRequestBuilder {
            WipeDeviceRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for wiping all data on the device."]
    pub struct WipeDeviceResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "device")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resultant Device object for the action. Note that asset tags will not be returned in the device object."]
        pub device: ::std::option::Option<::std::boxed::Box<Device>>,
    }
    impl WipeDeviceResponse {
        pub fn builder() -> WipeDeviceResponseBuilder {
            WipeDeviceResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for starting an account wipe on device."]
    pub struct WipeDeviceUserRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer_id}`, where customer_id is the customer to whom the device belongs."]
        pub customer: ::std::option::Option<::std::string::String>,
    }
    impl WipeDeviceUserRequest {
        pub fn builder() -> WipeDeviceUserRequestBuilder {
            WipeDeviceUserRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for wiping the user's account from the device."]
    pub struct WipeDeviceUserResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceUser")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resultant DeviceUser object for the action."]
        pub device_user: ::std::option::Option<::std::boxed::Box<DeviceUser>>,
    }
    impl WipeDeviceUserResponse {
        pub fn builder() -> WipeDeviceUserResponseBuilder {
            WipeDeviceUserResponseBuilder::default()
        }
    }
}
