#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Device resource represents an instance of enterprise managed device in the property."]
pub struct GoogleHomeEnterpriseSdmV1Device {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The resource name of the device. For example: \"enterprises/XYZ/devices/123\"."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parentRelations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Assignee details of the device."]
    pub parent_relations: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleHomeEnterpriseSdmV1ParentRelation>>,
    >,
    #[serde(rename = "traits")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Device traits."]
    pub traits: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Type of the device for general display purposes. For example: \"THERMOSTAT\". The device type should not be used to deduce or infer functionality of the actual device it is assigned to. Instead, use the returned traits for the device."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for SmartDeviceManagementService.ExecuteDeviceCommand"]
pub struct GoogleHomeEnterpriseSdmV1ExecuteDeviceCommandRequest {
    #[serde(rename = "command")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The command name to execute, represented by the fully qualified protobuf message name."]
    pub command: ::std::option::Option<::std::string::String>,
    #[serde(rename = "params")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The command message to execute, represented as a Struct."]
    pub params: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for SmartDeviceManagementService.ExecuteDeviceCommand"]
pub struct GoogleHomeEnterpriseSdmV1ExecuteDeviceCommandResponse {
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The results of executing the command."]
    pub results: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for SmartDeviceManagementService.ListDevices"]
pub struct GoogleHomeEnterpriseSdmV1ListDevicesResponse {
    #[serde(rename = "devices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of devices."]
    pub devices:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleHomeEnterpriseSdmV1Device>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pagination token to retrieve the next page of results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for SmartDeviceManagementService.ListRooms"]
pub struct GoogleHomeEnterpriseSdmV1ListRoomsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pagination token to retrieve the next page of results. If this field is omitted, there are no subsequent pages."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rooms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of rooms."]
    pub rooms:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleHomeEnterpriseSdmV1Room>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for SmartDeviceManagementService.ListStructures"]
pub struct GoogleHomeEnterpriseSdmV1ListStructuresResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pagination token to retrieve the next page of results. If this field is omitted, there are no subsequent pages."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "structures")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of structures."]
    pub structures: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleHomeEnterpriseSdmV1Structure>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents device relationships, for instance, structure/room to which the device is assigned to."]
pub struct GoogleHomeEnterpriseSdmV1ParentRelation {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The custom name of the relation -- e.g., structure/room where the device is assigned to."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The name of the relation -- e.g., structure/room where the device is assigned to. For example: \"enterprises/XYZ/structures/ABC\" or \"enterprises/XYZ/structures/ABC/rooms/123\""]
    pub parent: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Room resource represents an instance of sub-space within a structure such as rooms in a hotel suite or rental apartment."]
pub struct GoogleHomeEnterpriseSdmV1Room {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the room. For example: \"enterprises/XYZ/structures/ABC/rooms/123\"."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "traits")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Room traits."]
    pub traits: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Structure resource represents an instance of enterprise managed home or hotel room."]
pub struct GoogleHomeEnterpriseSdmV1Structure {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource name of the structure. For example: \"enterprises/XYZ/structures/ABC\"."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "traits")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Structure traits."]
    pub traits: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
}
