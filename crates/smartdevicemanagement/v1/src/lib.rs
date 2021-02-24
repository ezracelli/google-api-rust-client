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
    pub mod enterprises {
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
                        pub struct QueryParameters {
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "filter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional filter to list devices. Filters can be done on: Device custom name (substring match): 'customName=wing'"]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional requested page size. Server may return fewer devices than requested. If unspecified, server will pick an appropriate default."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional token of the page to retrieve."]
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
            pub mod structures {
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
                            #[doc = "Optional filter to list structures."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Requested page size. Server may return fewer structures than requested. If unspecified, server will pick an appropriate default."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The token of the page to retrieve."]
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
                    pub mod rooms {
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
                                    #[doc = "Requested page size. Server may return fewer rooms than requested. If unspecified, server will pick an appropriate default."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The token of the page to retrieve."]
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
    #[doc = "Device resource represents an instance of enterprise managed device in the property."]
    pub struct GoogleHomeEnterpriseSdmV1Device {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The resource name of the device. For example: \"enterprises/XYZ/devices/123\"."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentRelations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Assignee details of the device."]
        pub parent_relations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleHomeEnterpriseSdmV1ParentRelation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "traits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Device traits."]
        pub traits:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Type of the device for general display purposes. For example: \"THERMOSTAT\". The device type should not be used to deduce or infer functionality of the actual device it is assigned to. Instead, use the returned traits for the device."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleHomeEnterpriseSdmV1Device {
        pub fn builder() -> GoogleHomeEnterpriseSdmV1DeviceBuilder {
            GoogleHomeEnterpriseSdmV1DeviceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for SmartDeviceManagementService.ExecuteDeviceCommand"]
    pub struct GoogleHomeEnterpriseSdmV1ExecuteDeviceCommandRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "command")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The command name to execute, represented by the fully qualified protobuf message name."]
        pub command: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "params")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The command message to execute, represented as a Struct."]
        pub params:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl GoogleHomeEnterpriseSdmV1ExecuteDeviceCommandRequest {
        pub fn builder() -> GoogleHomeEnterpriseSdmV1ExecuteDeviceCommandRequestBuilder {
            GoogleHomeEnterpriseSdmV1ExecuteDeviceCommandRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for SmartDeviceManagementService.ExecuteDeviceCommand"]
    pub struct GoogleHomeEnterpriseSdmV1ExecuteDeviceCommandResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The results of executing the command."]
        pub results:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl GoogleHomeEnterpriseSdmV1ExecuteDeviceCommandResponse {
        pub fn builder() -> GoogleHomeEnterpriseSdmV1ExecuteDeviceCommandResponseBuilder {
            GoogleHomeEnterpriseSdmV1ExecuteDeviceCommandResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for SmartDeviceManagementService.ListDevices"]
    pub struct GoogleHomeEnterpriseSdmV1ListDevicesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "devices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of devices."]
        pub devices: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleHomeEnterpriseSdmV1Device>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The pagination token to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleHomeEnterpriseSdmV1ListDevicesResponse {
        pub fn builder() -> GoogleHomeEnterpriseSdmV1ListDevicesResponseBuilder {
            GoogleHomeEnterpriseSdmV1ListDevicesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for SmartDeviceManagementService.ListRooms"]
    pub struct GoogleHomeEnterpriseSdmV1ListRoomsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The pagination token to retrieve the next page of results. If this field is omitted, there are no subsequent pages."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rooms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of rooms."]
        pub rooms: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleHomeEnterpriseSdmV1Room>>,
        >,
    }
    impl GoogleHomeEnterpriseSdmV1ListRoomsResponse {
        pub fn builder() -> GoogleHomeEnterpriseSdmV1ListRoomsResponseBuilder {
            GoogleHomeEnterpriseSdmV1ListRoomsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for SmartDeviceManagementService.ListStructures"]
    pub struct GoogleHomeEnterpriseSdmV1ListStructuresResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The pagination token to retrieve the next page of results. If this field is omitted, there are no subsequent pages."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "structures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of structures."]
        pub structures: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleHomeEnterpriseSdmV1Structure>>,
        >,
    }
    impl GoogleHomeEnterpriseSdmV1ListStructuresResponse {
        pub fn builder() -> GoogleHomeEnterpriseSdmV1ListStructuresResponseBuilder {
            GoogleHomeEnterpriseSdmV1ListStructuresResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents device relationships, for instance, structure/room to which the device is assigned to."]
    pub struct GoogleHomeEnterpriseSdmV1ParentRelation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The custom name of the relation -- e.g., structure/room where the device is assigned to."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of the relation -- e.g., structure/room where the device is assigned to. For example: \"enterprises/XYZ/structures/ABC\" or \"enterprises/XYZ/structures/ABC/rooms/123\""]
        pub parent: ::std::option::Option<::std::string::String>,
    }
    impl GoogleHomeEnterpriseSdmV1ParentRelation {
        pub fn builder() -> GoogleHomeEnterpriseSdmV1ParentRelationBuilder {
            GoogleHomeEnterpriseSdmV1ParentRelationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Room resource represents an instance of sub-space within a structure such as rooms in a hotel suite or rental apartment."]
    pub struct GoogleHomeEnterpriseSdmV1Room {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The resource name of the room. For example: \"enterprises/XYZ/structures/ABC/rooms/123\"."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "traits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Room traits."]
        pub traits:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl GoogleHomeEnterpriseSdmV1Room {
        pub fn builder() -> GoogleHomeEnterpriseSdmV1RoomBuilder {
            GoogleHomeEnterpriseSdmV1RoomBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Structure resource represents an instance of enterprise managed home or hotel room."]
    pub struct GoogleHomeEnterpriseSdmV1Structure {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The resource name of the structure. For example: \"enterprises/XYZ/structures/ABC\"."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "traits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Structure traits."]
        pub traits:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl GoogleHomeEnterpriseSdmV1Structure {
        pub fn builder() -> GoogleHomeEnterpriseSdmV1StructureBuilder {
            GoogleHomeEnterpriseSdmV1StructureBuilder::default()
        }
    }
}
