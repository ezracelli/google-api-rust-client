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
                    pub mod registries {
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
                                    #[doc = "The maximum number of registries to return in the response. If this value is zero, the service will select a default size. A call may return fewer objects than requested. A non-empty `next_page_token` in the response indicates that more data is available."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The value returned by the last `ListDeviceRegistriesResponse`; indicates that this is a continuation of a prior `ListDeviceRegistries` call and the system should return the next page of data."]
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
                                    #[doc = "Required. Only updates the `device_registry` fields indicated by this mask. The field mask must not be empty, and it must not contain fields that are immutable or only set by the server. Mutable top-level fields: `event_notification_config`, `http_config`, `mqtt_config`, and `state_notification_config`."]
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
                            pub mod devices {
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
                                            #[serde(rename = "fieldMask")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The fields of the `Device` resource to be returned in the response. If the field mask is unset or empty, all fields are returned. Fields have to be provided in snake_case format, for example: `last_heartbeat_time`."]
                                            pub field_mask:
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
                                        pub struct QueryParameters { # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "deviceIds")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "A list of device string IDs. For example, `['device0', 'device12']`. If empty, this field is ignored. Maximum IDs: 10,000"] pub device_ids : :: std :: option :: Option < :: std :: string :: String > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "deviceNumIds")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "A list of device numeric IDs. If empty, this field is ignored. Maximum IDs: 10,000."] pub device_num_ids : :: std :: option :: Option < :: std :: string :: String > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "fieldMask")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "The fields of the `Device` resource to be returned in the response. The fields `id` and `num_id` are always returned, along with any other fields specified in snake_case format, for example: `last_heartbeat_time`."] pub field_mask : :: std :: option :: Option < :: std :: string :: String > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "gatewayListOptions.associationsDeviceId")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "If set, returns only the gateways with which the specified device is associated. The device ID can be numeric (`num_id`) or the user-defined string (`id`). For example, if `456` is specified, returns only the gateways to which the device with `num_id` 456 is bound."] pub gateway_list_options_associations_device_id : :: std :: option :: Option < :: std :: string :: String > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "gatewayListOptions.associationsGatewayId")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "If set, only devices associated with the specified gateway are returned. The gateway ID can be numeric (`num_id`) or the user-defined string (`id`). For example, if `123` is specified, only devices bound to the gateway with `num_id` 123 are returned."] pub gateway_list_options_associations_gateway_id : :: std :: option :: Option < :: std :: string :: String > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "gatewayListOptions.gatewayType")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "If `GATEWAY` is specified, only gateways are returned. If `NON_GATEWAY` is specified, only non-gateway devices are returned. If `GATEWAY_TYPE_UNSPECIFIED` is specified, all devices are returned."] pub gateway_list_options_gateway_type : :: std :: option :: Option < QueryParametersGatewayListOptionsGatewayTypeEnum > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "pageSize")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "The maximum number of devices to return in the response. If this value is zero, the service will select a default size. A call may return fewer objects than requested. A non-empty `next_page_token` in the response indicates that more data is available."] pub page_size : :: std :: option :: Option < :: std :: primitive :: i64 > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "pageToken")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "The value returned by the last `ListDevicesResponse`; indicates that this is a continuation of a prior `ListDevices` call and the system should return the next page of data."] pub page_token : :: std :: option :: Option < :: std :: string :: String > }
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
                                        #[doc = "If `GATEWAY` is specified, only gateways are returned. If `NON_GATEWAY` is specified, only non-gateway devices are returned. If `GATEWAY_TYPE_UNSPECIFIED` is specified, all devices are returned."]
                                        pub enum QueryParametersGatewayListOptionsGatewayTypeEnum {
                                            #[serde(rename = "GATEWAY_TYPE_UNSPECIFIED")]
                                            #[doc = "If unspecified, the device is considered a non-gateway device."]
                                            GatewayTypeUnspecified,
                                            #[serde(rename = "GATEWAY")]
                                            #[doc = "The device is a gateway."]
                                            Gateway,
                                            #[serde(rename = "NON_GATEWAY")]
                                            #[doc = "The device is not a gateway."]
                                            NonGateway,
                                        }
                                        impl ::std::default::Default for QueryParametersGatewayListOptionsGatewayTypeEnum {
                                            fn default() -> Self {
                                                Self::GatewayTypeUnspecified
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
                                            #[doc = "Required. Only updates the `device` fields indicated by this mask. The field mask must not be empty, and it must not contain fields that are immutable or only set by the server. Mutable top-level fields: `credentials`, `blocked`, and `metadata`"]
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
                                    pub mod config_versions {
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
                                                    #[serde(rename = "numVersions")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "The number of versions to list. Versions are listed in decreasing order of the version number. The maximum number of versions retained is 10. If this value is zero, it will return all the versions available."]
                                                    pub num_versions: ::std::option::Option<
                                                        ::std::primitive::i64,
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
                                    pub mod states {
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
                                                    #[serde(rename = "numStates")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "The number of states to list. States are listed in descending order of update time. The maximum number of states retained is 10. If this value is zero, it will return all the states available."]
                                                    pub num_states: ::std::option::Option<
                                                        ::std::primitive::i64,
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
                            pub mod groups {
                                pub mod resources {
                                    pub mod devices {
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
                                                pub struct QueryParameters { # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "deviceIds")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "A list of device string IDs. For example, `['device0', 'device12']`. If empty, this field is ignored. Maximum IDs: 10,000"] pub device_ids : :: std :: option :: Option < :: std :: string :: String > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "deviceNumIds")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "A list of device numeric IDs. If empty, this field is ignored. Maximum IDs: 10,000."] pub device_num_ids : :: std :: option :: Option < :: std :: string :: String > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "fieldMask")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "The fields of the `Device` resource to be returned in the response. The fields `id` and `num_id` are always returned, along with any other fields specified in snake_case format, for example: `last_heartbeat_time`."] pub field_mask : :: std :: option :: Option < :: std :: string :: String > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "gatewayListOptions.associationsDeviceId")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "If set, returns only the gateways with which the specified device is associated. The device ID can be numeric (`num_id`) or the user-defined string (`id`). For example, if `456` is specified, returns only the gateways to which the device with `num_id` 456 is bound."] pub gateway_list_options_associations_device_id : :: std :: option :: Option < :: std :: string :: String > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "gatewayListOptions.associationsGatewayId")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "If set, only devices associated with the specified gateway are returned. The gateway ID can be numeric (`num_id`) or the user-defined string (`id`). For example, if `123` is specified, only devices bound to the gateway with `num_id` 123 are returned."] pub gateway_list_options_associations_gateway_id : :: std :: option :: Option < :: std :: string :: String > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "gatewayListOptions.gatewayType")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "If `GATEWAY` is specified, only gateways are returned. If `NON_GATEWAY` is specified, only non-gateway devices are returned. If `GATEWAY_TYPE_UNSPECIFIED` is specified, all devices are returned."] pub gateway_list_options_gateway_type : :: std :: option :: Option < QueryParametersGatewayListOptionsGatewayTypeEnum > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "pageSize")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "The maximum number of devices to return in the response. If this value is zero, the service will select a default size. A call may return fewer objects than requested. A non-empty `next_page_token` in the response indicates that more data is available."] pub page_size : :: std :: option :: Option < :: std :: primitive :: i64 > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "pageToken")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "The value returned by the last `ListDevicesResponse`; indicates that this is a continuation of a prior `ListDevices` call and the system should return the next page of data."] pub page_token : :: std :: option :: Option < :: std :: string :: String > }
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
                                                #[doc = "If `GATEWAY` is specified, only gateways are returned. If `NON_GATEWAY` is specified, only non-gateway devices are returned. If `GATEWAY_TYPE_UNSPECIFIED` is specified, all devices are returned."]
                                                pub enum QueryParametersGatewayListOptionsGatewayTypeEnum {
                                                    #[serde(rename = "GATEWAY_TYPE_UNSPECIFIED")]
                                                    #[doc = "If unspecified, the device is considered a non-gateway device."]
                                                    GatewayTypeUnspecified,
                                                    #[serde(rename = "GATEWAY")]
                                                    #[doc = "The device is a gateway."]
                                                    Gateway,
                                                    #[serde(rename = "NON_GATEWAY")]
                                                    #[doc = "The device is not a gateway."]
                                                    NonGateway,
                                                }
                                                impl ::std::default::Default for QueryParametersGatewayListOptionsGatewayTypeEnum {
                                                    fn default() -> Self {
                                                        Self::GatewayTypeUnspecified
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
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for `BindDeviceToGateway`."]
    pub struct BindDeviceToGatewayRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The device to associate with the specified gateway. The value of `device_id` can be either the device numeric ID or the user-defined device identifier."]
        pub device_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gatewayId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The value of `gateway_id` can be either the device numeric ID or the user-defined device identifier."]
        pub gateway_id: ::std::option::Option<::std::string::String>,
    }
    impl BindDeviceToGatewayRequest {
        pub fn builder() -> BindDeviceToGatewayRequestBuilder {
            BindDeviceToGatewayRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for `BindDeviceToGateway`."]
    pub struct BindDeviceToGatewayResponse {}
    impl BindDeviceToGatewayResponse {
        pub fn builder() -> BindDeviceToGatewayResponseBuilder {
            BindDeviceToGatewayResponseBuilder::default()
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
    #[doc = "The device resource."]
    pub struct Device {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blocked")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If a device is blocked, connections or requests from this device will fail. Can be used to temporarily prevent the device from connecting if, for example, the sensor is generating bad data and needs maintenance."]
        pub blocked: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "config")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The most recent device configuration, which is eventually sent from Cloud IoT Core to the device. If not present on creation, the configuration will be initialized with an empty payload and version value of `1`. To update this field after creation, use the `DeviceManager.ModifyCloudToDeviceConfig` method."]
        pub config: ::std::option::Option<::std::boxed::Box<DeviceConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "credentials")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The credentials used to authenticate this device. To allow credential rotation without interruption, multiple device credentials can be bound to this device. No more than 3 credentials can be bound to a single device at a time. When new credentials are added to a device, they are verified against the registry credentials. For details, see the description of the `DeviceRegistry.credentials` field."]
        pub credentials:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DeviceCredential>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gatewayConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Gateway-related configuration and state."]
        pub gateway_config: ::std::option::Option<::std::boxed::Box<GatewayConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user-defined device identifier. The device ID must be unique within a device registry."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastConfigAckTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The last time a cloud-to-device config version acknowledgment was received from the device. This field is only for configurations sent through MQTT."]
        pub last_config_ack_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastConfigSendTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The last time a cloud-to-device config version was sent to the device."]
        pub last_config_send_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastErrorStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The error message of the most recent error, such as a failure to publish to Cloud Pub/Sub. 'last_error_time' is the timestamp of this field. If no errors have occurred, this field has an empty message and the status code 0 == OK. Otherwise, this field is expected to have a status code other than OK."]
        pub last_error_status: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastErrorTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The time the most recent error occurred, such as a failure to publish to Cloud Pub/Sub. This field is the timestamp of 'last_error_status'."]
        pub last_error_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastEventTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The last time a telemetry event was received. Timestamps are periodically collected and written to storage; they may be stale by a few minutes."]
        pub last_event_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastHeartbeatTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The last time an MQTT `PINGREQ` was received. This field applies only to devices connecting through MQTT. MQTT clients usually only send `PINGREQ` messages if the connection is idle, and no other messages have been sent. Timestamps are periodically collected and written to storage; they may be stale by a few minutes."]
        pub last_heartbeat_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastStateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The last time a state event was received. Timestamps are periodically collected and written to storage; they may be stale by a few minutes."]
        pub last_state_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "**Beta Feature** The logging verbosity for device activity. If unspecified, DeviceRegistry.log_level will be used."]
        pub log_level: ::std::option::Option<DeviceLogLevelEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metadata key-value pairs assigned to the device. This metadata is not interpreted or indexed by Cloud IoT Core. It can be used to add contextual information for the device. Keys must conform to the regular expression a-zA-Z+ and be less than 128 bytes in length. Values are free-form strings. Each value must be less than or equal to 32 KB in size. The total size of all keys and values must be less than 256 KB, and the maximum number of key-value pairs is 500."]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource path name. For example, `projects/p1/locations/us-central1/registries/registry0/devices/dev0` or `projects/p1/locations/us-central1/registries/registry0/devices/{num_id}`. When `name` is populated as a response from the service, it always ends in the device numeric ID."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] A server-defined unique numeric ID for the device. This is a more compact way to identify devices, and it is globally unique."]
        pub num_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The state most recently received from the device. If no state has been reported, this field is not present."]
        pub state: ::std::option::Option<::std::boxed::Box<DeviceState>>,
    }
    impl Device {
        pub fn builder() -> DeviceBuilder {
            DeviceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "**Beta Feature** The logging verbosity for device activity. If unspecified, DeviceRegistry.log_level will be used."]
    pub enum DeviceLogLevelEnum {
        #[serde(rename = "LOG_LEVEL_UNSPECIFIED")]
        #[doc = "No logging specified. If not specified, logging will be disabled."]
        LogLevelUnspecified,
        #[serde(rename = "NONE")]
        #[doc = "Disables logging."]
        None,
        #[serde(rename = "ERROR")]
        #[doc = "Error events will be logged."]
        Error,
        #[serde(rename = "INFO")]
        #[doc = "Informational events will be logged, such as connections and disconnections."]
        Info,
        #[serde(rename = "DEBUG")]
        #[doc = "All events will be logged."]
        Debug,
    }
    impl ::std::default::Default for DeviceLogLevelEnum {
        fn default() -> Self {
            Self::LogLevelUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The device configuration. Eventually delivered to devices."]
    pub struct DeviceConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "binaryData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device configuration data."]
        pub binary_data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloudUpdateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The time at which this configuration version was updated in Cloud IoT Core. This timestamp is set by the server."]
        pub cloud_update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceAckTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The time at which Cloud IoT Core received the acknowledgment from the device, indicating that the device has received this configuration version. If this field is not present, the device has not yet acknowledged that it received this version. Note that when the config was sent to the device, many config versions may have been available in Cloud IoT Core while the device was disconnected, and on connection, only the latest version is sent to the device. Some versions may never be sent to the device, and therefore are never acknowledged. This timestamp is set by Cloud IoT Core."]
        pub device_ack_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The version of this update. The version number is assigned by the server, and is always greater than 0 after device creation. The version must be 0 on the `CreateDevice` request if a `config` is specified; the response of `CreateDevice` will always have a value of 1."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl DeviceConfig {
        pub fn builder() -> DeviceConfigBuilder {
            DeviceConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A server-stored device credential used for authentication."]
    pub struct DeviceCredential {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expirationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Optional] The time at which this credential becomes invalid. This credential will be ignored for new client authentication requests after this timestamp; however, it will not be automatically deleted."]
        pub expiration_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publicKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A public key used to verify the signature of JSON Web Tokens (JWTs). When adding a new device credential, either via device creation or via modifications, this public key credential may be required to be signed by one of the registry level certificates. More specifically, if the registry contains at least one certificate, any new device credential must be signed by one of the registry certificates. As a result, when the registry contains certificates, only X.509 certificates are accepted as device credentials. However, if the registry does not contain a certificate, self-signed certificates and public keys will be accepted. New device credentials must be different from every registry-level certificate."]
        pub public_key: ::std::option::Option<::std::boxed::Box<PublicKeyCredential>>,
    }
    impl DeviceCredential {
        pub fn builder() -> DeviceCredentialBuilder {
            DeviceCredentialBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A container for a group of devices."]
    pub struct DeviceRegistry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "credentials")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The credentials used to verify the device credentials. No more than 10 credentials can be bound to a single registry at a time. The verification process occurs at the time of device creation or update. If this field is empty, no verification is performed. Otherwise, the credentials of a newly created device or added credentials of an updated device should be signed with one of these registry credentials. Note, however, that existing devices will never be affected by modifications to this list of credentials: after a device has been successfully created in a registry, it should be able to connect even if its registry credentials are revoked, deleted, or modified."]
        pub credentials:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RegistryCredential>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventNotificationConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The configuration for notification of telemetry events received from the device. All telemetry events that were successfully published by the device and acknowledged by Cloud IoT Core are guaranteed to be delivered to Cloud Pub/Sub. If multiple configurations match a message, only the first matching configuration is used. If you try to publish a device telemetry event using MQTT without specifying a Cloud Pub/Sub topic for the device's registry, the connection closes automatically. If you try to do so using an HTTP connection, an error is returned. Up to 10 configurations may be provided."]
        pub event_notification_configs:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventNotificationConfig>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "httpConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The DeviceService (HTTP) configuration for this device registry."]
        pub http_config: ::std::option::Option<::std::boxed::Box<HttpConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier of this device registry. For example, `myRegistry`."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "**Beta Feature** The default logging verbosity for activity from devices in this registry. The verbosity level can be overridden by Device.log_level."]
        pub log_level: ::std::option::Option<DeviceRegistryLogLevelEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mqttConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The MQTT configuration for this device registry."]
        pub mqtt_config: ::std::option::Option<::std::boxed::Box<MqttConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource path name. For example, `projects/example-project/locations/us-central1/registries/my-registry`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stateNotificationConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The configuration for notification of new states received from the device. State updates are guaranteed to be stored in the state history, but notifications to Cloud Pub/Sub are not guaranteed. For example, if permissions are misconfigured or the specified topic doesn't exist, no notification will be published but the state will still be stored in Cloud IoT Core."]
        pub state_notification_config:
            ::std::option::Option<::std::boxed::Box<StateNotificationConfig>>,
    }
    impl DeviceRegistry {
        pub fn builder() -> DeviceRegistryBuilder {
            DeviceRegistryBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "**Beta Feature** The default logging verbosity for activity from devices in this registry. The verbosity level can be overridden by Device.log_level."]
    pub enum DeviceRegistryLogLevelEnum {
        #[serde(rename = "LOG_LEVEL_UNSPECIFIED")]
        #[doc = "No logging specified. If not specified, logging will be disabled."]
        LogLevelUnspecified,
        #[serde(rename = "NONE")]
        #[doc = "Disables logging."]
        None,
        #[serde(rename = "ERROR")]
        #[doc = "Error events will be logged."]
        Error,
        #[serde(rename = "INFO")]
        #[doc = "Informational events will be logged, such as connections and disconnections."]
        Info,
        #[serde(rename = "DEBUG")]
        #[doc = "All events will be logged."]
        Debug,
    }
    impl ::std::default::Default for DeviceRegistryLogLevelEnum {
        fn default() -> Self {
            Self::LogLevelUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The device state, as reported by the device."]
    pub struct DeviceState {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "binaryData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device state data."]
        pub binary_data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The time at which this state version was updated in Cloud IoT Core."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl DeviceState {
        pub fn builder() -> DeviceStateBuilder {
            DeviceStateBuilder::default()
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
    #[doc = "The configuration for forwarding telemetry events."]
    pub struct EventNotificationConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pubsubTopicName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A Cloud Pub/Sub topic name. For example, `projects/myProject/topics/deviceEvents`."]
        pub pubsub_topic_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subfolderMatches")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the subfolder name matches this string exactly, this configuration will be used. The string must not include the leading '/' character. If empty, all strings are matched. This field is used only for telemetry events; subfolders are not supported for state changes."]
        pub subfolder_matches: ::std::option::Option<::std::string::String>,
    }
    impl EventNotificationConfig {
        pub fn builder() -> EventNotificationConfigBuilder {
            EventNotificationConfigBuilder::default()
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
    #[doc = "Gateway-related configuration and state."]
    pub struct GatewayConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gatewayAuthMethod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates how to authorize and/or authenticate devices to access the gateway."]
        pub gateway_auth_method: ::std::option::Option<GatewayConfigGatewayAuthMethodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gatewayType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the device is a gateway."]
        pub gateway_type: ::std::option::Option<GatewayConfigGatewayTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastAccessedGatewayId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The ID of the gateway the device accessed most recently."]
        pub last_accessed_gateway_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastAccessedGatewayTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The most recent time at which the device accessed the gateway specified in `last_accessed_gateway`."]
        pub last_accessed_gateway_time: ::std::option::Option<::std::string::String>,
    }
    impl GatewayConfig {
        pub fn builder() -> GatewayConfigBuilder {
            GatewayConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Indicates how to authorize and/or authenticate devices to access the gateway."]
    pub enum GatewayConfigGatewayAuthMethodEnum {
        #[serde(rename = "GATEWAY_AUTH_METHOD_UNSPECIFIED")]
        #[doc = "No authentication/authorization method specified. No devices are allowed to access the gateway."]
        GatewayAuthMethodUnspecified,
        #[serde(rename = "ASSOCIATION_ONLY")]
        #[doc = "The device is authenticated through the gateway association only. Device credentials are ignored even if provided."]
        AssociationOnly,
        #[serde(rename = "DEVICE_AUTH_TOKEN_ONLY")]
        #[doc = "The device is authenticated through its own credentials. Gateway association is not checked."]
        DeviceAuthTokenOnly,
        #[serde(rename = "ASSOCIATION_AND_DEVICE_AUTH_TOKEN")]
        #[doc = "The device is authenticated through both device credentials and gateway association. The device must be bound to the gateway and must provide its own credentials."]
        AssociationAndDeviceAuthToken,
    }
    impl ::std::default::Default for GatewayConfigGatewayAuthMethodEnum {
        fn default() -> Self {
            Self::GatewayAuthMethodUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Indicates whether the device is a gateway."]
    pub enum GatewayConfigGatewayTypeEnum {
        #[serde(rename = "GATEWAY_TYPE_UNSPECIFIED")]
        #[doc = "If unspecified, the device is considered a non-gateway device."]
        GatewayTypeUnspecified,
        #[serde(rename = "GATEWAY")]
        #[doc = "The device is a gateway."]
        Gateway,
        #[serde(rename = "NON_GATEWAY")]
        #[doc = "The device is not a gateway."]
        NonGateway,
    }
    impl ::std::default::Default for GatewayConfigGatewayTypeEnum {
        fn default() -> Self {
            Self::GatewayTypeUnspecified
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
    #[doc = "The configuration of the HTTP bridge for a device registry."]
    pub struct HttpConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "httpEnabledState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If enabled, allows devices to use DeviceService via the HTTP protocol. Otherwise, any requests to DeviceService will fail for this registry."]
        pub http_enabled_state: ::std::option::Option<HttpConfigHttpEnabledStateEnum>,
    }
    impl HttpConfig {
        pub fn builder() -> HttpConfigBuilder {
            HttpConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "If enabled, allows devices to use DeviceService via the HTTP protocol. Otherwise, any requests to DeviceService will fail for this registry."]
    pub enum HttpConfigHttpEnabledStateEnum {
        #[serde(rename = "HTTP_STATE_UNSPECIFIED")]
        #[doc = "No HTTP state specified. If not specified, DeviceService will be enabled by default."]
        HttpStateUnspecified,
        #[serde(rename = "HTTP_ENABLED")]
        #[doc = "Enables DeviceService (HTTP) service for the registry."]
        HttpEnabled,
        #[serde(rename = "HTTP_DISABLED")]
        #[doc = "Disables DeviceService (HTTP) service for the registry."]
        HttpDisabled,
    }
    impl ::std::default::Default for HttpConfigHttpEnabledStateEnum {
        fn default() -> Self {
            Self::HttpStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for `ListDeviceConfigVersions`."]
    pub struct ListDeviceConfigVersionsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device configuration for the last few versions. Versions are listed in decreasing order, starting from the most recent one."]
        pub device_configs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DeviceConfig>>>,
    }
    impl ListDeviceConfigVersionsResponse {
        pub fn builder() -> ListDeviceConfigVersionsResponseBuilder {
            ListDeviceConfigVersionsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for `ListDeviceRegistries`."]
    pub struct ListDeviceRegistriesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceRegistries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The registries that matched the query."]
        pub device_registries:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DeviceRegistry>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If not empty, indicates that there may be more registries that match the request; this value should be passed in a new `ListDeviceRegistriesRequest`."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListDeviceRegistriesResponse {
        pub fn builder() -> ListDeviceRegistriesResponseBuilder {
            ListDeviceRegistriesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for `ListDeviceStates`."]
    pub struct ListDeviceStatesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceStates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last few device states. States are listed in descending order of server update time, starting from the most recent one."]
        pub device_states: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DeviceState>>>,
    }
    impl ListDeviceStatesResponse {
        pub fn builder() -> ListDeviceStatesResponseBuilder {
            ListDeviceStatesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for `ListDevices`."]
    pub struct ListDevicesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "devices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The devices that match the request."]
        pub devices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Device>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If not empty, indicates that there may be more devices that match the request; this value should be passed in a new `ListDevicesRequest`."]
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
    #[doc = "Request for `ModifyCloudToDeviceConfig`."]
    pub struct ModifyCloudToDeviceConfigRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "binaryData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The configuration data for the device."]
        pub binary_data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionToUpdate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version number to update. If this value is zero, it will not check the version number of the server and will always update the current version; otherwise, this update will fail if the version number found on the server does not match this version number. This is used to support multiple simultaneous updates without losing data."]
        pub version_to_update: ::std::option::Option<::std::string::String>,
    }
    impl ModifyCloudToDeviceConfigRequest {
        pub fn builder() -> ModifyCloudToDeviceConfigRequestBuilder {
            ModifyCloudToDeviceConfigRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The configuration of MQTT for a device registry."]
    pub struct MqttConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mqttEnabledState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If enabled, allows connections using the MQTT protocol. Otherwise, MQTT connections to this registry will fail."]
        pub mqtt_enabled_state: ::std::option::Option<MqttConfigMqttEnabledStateEnum>,
    }
    impl MqttConfig {
        pub fn builder() -> MqttConfigBuilder {
            MqttConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "If enabled, allows connections using the MQTT protocol. Otherwise, MQTT connections to this registry will fail."]
    pub enum MqttConfigMqttEnabledStateEnum {
        #[serde(rename = "MQTT_STATE_UNSPECIFIED")]
        #[doc = "No MQTT state specified. If not specified, MQTT will be enabled by default."]
        MqttStateUnspecified,
        #[serde(rename = "MQTT_ENABLED")]
        #[doc = "Enables a MQTT connection."]
        MqttEnabled,
        #[serde(rename = "MQTT_DISABLED")]
        #[doc = "Disables a MQTT connection."]
        MqttDisabled,
    }
    impl ::std::default::Default for MqttConfigMqttEnabledStateEnum {
        fn default() -> Self {
            Self::MqttStateUnspecified
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
    #[doc = "A public key certificate format and data."]
    pub struct PublicKeyCertificate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "certificate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The certificate data."]
        pub certificate: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "format")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The certificate format."]
        pub format: ::std::option::Option<PublicKeyCertificateFormatEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "x509Details")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[Output only] The certificate details. Used only for X.509 certificates."]
        pub x509_details: ::std::option::Option<::std::boxed::Box<X509CertificateDetails>>,
    }
    impl PublicKeyCertificate {
        pub fn builder() -> PublicKeyCertificateBuilder {
            PublicKeyCertificateBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The certificate format."]
    pub enum PublicKeyCertificateFormatEnum {
        #[serde(rename = "UNSPECIFIED_PUBLIC_KEY_CERTIFICATE_FORMAT")]
        #[doc = "The format has not been specified. This is an invalid default value and must not be used."]
        UnspecifiedPublicKeyCertificateFormat,
        #[serde(rename = "X509_CERTIFICATE_PEM")]
        #[doc = "An X.509v3 certificate ([RFC5280](https://www.ietf.org/rfc/rfc5280.txt)), encoded in base64, and wrapped by `-----BEGIN CERTIFICATE-----` and `-----END CERTIFICATE-----`."]
        X509CertificatePem,
    }
    impl ::std::default::Default for PublicKeyCertificateFormatEnum {
        fn default() -> Self {
            Self::UnspecifiedPublicKeyCertificateFormat
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A public key format and data."]
    pub struct PublicKeyCredential {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "format")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The format of the key."]
        pub format: ::std::option::Option<PublicKeyCredentialFormatEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The key data."]
        pub key: ::std::option::Option<::std::string::String>,
    }
    impl PublicKeyCredential {
        pub fn builder() -> PublicKeyCredentialBuilder {
            PublicKeyCredentialBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The format of the key."]
    pub enum PublicKeyCredentialFormatEnum {
        #[serde(rename = "UNSPECIFIED_PUBLIC_KEY_FORMAT")]
        #[doc = "The format has not been specified. This is an invalid default value and must not be used."]
        UnspecifiedPublicKeyFormat,
        #[serde(rename = "RSA_PEM")]
        #[doc = "An RSA public key encoded in base64, and wrapped by `-----BEGIN PUBLIC KEY-----` and `-----END PUBLIC KEY-----`. This can be used to verify `RS256` signatures in JWT tokens ([RFC7518]( https://www.ietf.org/rfc/rfc7518.txt))."]
        RsaPem,
        #[serde(rename = "RSA_X509_PEM")]
        #[doc = "As RSA_PEM, but wrapped in an X.509v3 certificate ([RFC5280]( https://www.ietf.org/rfc/rfc5280.txt)), encoded in base64, and wrapped by `-----BEGIN CERTIFICATE-----` and `-----END CERTIFICATE-----`."]
        RsaX509Pem,
        #[serde(rename = "ES256_PEM")]
        #[doc = "Public key for the ECDSA algorithm using P-256 and SHA-256, encoded in base64, and wrapped by `-----BEGIN PUBLIC KEY-----` and `-----END PUBLIC KEY-----`. This can be used to verify JWT tokens with the `ES256` algorithm ([RFC7518](https://www.ietf.org/rfc/rfc7518.txt)). This curve is defined in [OpenSSL](https://www.openssl.org/) as the `prime256v1` curve."]
        Es256Pem,
        #[serde(rename = "ES256_X509_PEM")]
        #[doc = "As ES256_PEM, but wrapped in an X.509v3 certificate ([RFC5280]( https://www.ietf.org/rfc/rfc5280.txt)), encoded in base64, and wrapped by `-----BEGIN CERTIFICATE-----` and `-----END CERTIFICATE-----`."]
        Es256X509Pem,
    }
    impl ::std::default::Default for PublicKeyCredentialFormatEnum {
        fn default() -> Self {
            Self::UnspecifiedPublicKeyFormat
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A server-stored registry credential used to validate device credentials."]
    pub struct RegistryCredential {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publicKeyCertificate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A public key certificate used to verify the device credentials."]
        pub public_key_certificate: ::std::option::Option<::std::boxed::Box<PublicKeyCertificate>>,
    }
    impl RegistryCredential {
        pub fn builder() -> RegistryCredentialBuilder {
            RegistryCredentialBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for `SendCommandToDevice`."]
    pub struct SendCommandToDeviceRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "binaryData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The command data to send to the device."]
        pub binary_data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subfolder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional subfolder for the command. If empty, the command will be delivered to the /devices/{device-id}/commands topic, otherwise it will be delivered to the /devices/{device-id}/commands/{subfolder} topic. Multi-level subfolders are allowed. This field must not have more than 256 characters, and must not contain any MQTT wildcards (\"+\" or \"#\") or null characters."]
        pub subfolder: ::std::option::Option<::std::string::String>,
    }
    impl SendCommandToDeviceRequest {
        pub fn builder() -> SendCommandToDeviceRequestBuilder {
            SendCommandToDeviceRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for `SendCommandToDevice`."]
    pub struct SendCommandToDeviceResponse {}
    impl SendCommandToDeviceResponse {
        pub fn builder() -> SendCommandToDeviceResponseBuilder {
            SendCommandToDeviceResponseBuilder::default()
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
    #[doc = "The configuration for notification of new states received from the device."]
    pub struct StateNotificationConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pubsubTopicName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A Cloud Pub/Sub topic name. For example, `projects/myProject/topics/deviceEvents`."]
        pub pubsub_topic_name: ::std::option::Option<::std::string::String>,
    }
    impl StateNotificationConfig {
        pub fn builder() -> StateNotificationConfigBuilder {
            StateNotificationConfigBuilder::default()
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
    #[doc = "Request for `UnbindDeviceFromGateway`."]
    pub struct UnbindDeviceFromGatewayRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The device to disassociate from the specified gateway. The value of `device_id` can be either the device numeric ID or the user-defined device identifier."]
        pub device_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gatewayId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The value of `gateway_id` can be either the device numeric ID or the user-defined device identifier."]
        pub gateway_id: ::std::option::Option<::std::string::String>,
    }
    impl UnbindDeviceFromGatewayRequest {
        pub fn builder() -> UnbindDeviceFromGatewayRequestBuilder {
            UnbindDeviceFromGatewayRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for `UnbindDeviceFromGateway`."]
    pub struct UnbindDeviceFromGatewayResponse {}
    impl UnbindDeviceFromGatewayResponse {
        pub fn builder() -> UnbindDeviceFromGatewayResponseBuilder {
            UnbindDeviceFromGatewayResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details of an X.509 certificate. For informational purposes only."]
    pub struct X509CertificateDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expiryTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time the certificate becomes invalid."]
        pub expiry_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "issuer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entity that signed the certificate."]
        pub issuer: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publicKeyType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of public key in the certificate."]
        pub public_key_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "signatureAlgorithm")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The algorithm used to sign the certificate."]
        pub signature_algorithm: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time the certificate becomes valid."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subject")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entity the certificate and public key belong to."]
        pub subject: ::std::option::Option<::std::string::String>,
    }
    impl X509CertificateDetails {
        pub fn builder() -> X509CertificateDetailsBuilder {
            X509CertificateDetailsBuilder::default()
        }
    }
}
