#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Third-party device ID for one device."]
pub struct AgentDeviceId {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Third-party device ID."]
    pub id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Alternate third-party device ID."]
pub struct AgentOtherDeviceId {
    #[serde(rename = "agentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Project ID for your smart home Action."]
    pub agent_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique third-party device ID."]
    pub device_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Third-party device definition. Next ID = 14"]
pub struct Device {
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Attributes for the traits supported by the device."]
    pub attributes:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom device attributes stored in Home Graph and provided to your smart home Action in each [QUERY](https://developers.google.com/assistant/smarthome/reference/intent/query) and [EXECUTE](https://developers.google.com/assistant/smarthome/reference/intent/execute) intent. Data in this object has a few constraints: No sensitive information, including but not limited to Personally Identifiable Information."]
    pub custom_data:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "deviceInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Device manufacturer, model, hardware version, and software version."]
    pub device_info: ::std::option::Option<::std::boxed::Box<DeviceInfo>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Third-party device ID."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Names given to this device by your smart home Action."]
    pub name: ::std::option::Option<::std::boxed::Box<DeviceNames>>,
    #[serde(rename = "nonLocalTraits")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "See description for \"traits\". For Smart Home Entertainment Devices (SHED) devices, some traits can only be executed on 3P cloud, e.g. \"non_local_traits\": [ { \"trait\": \"action.devices.traits.MediaInitiation\" }, { \"trait\": \"action.devices.traits.Channel\" } ] go/shed-per-trait-routing."]
    pub non_local_traits: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NonLocalTrait>>>,
    #[serde(rename = "notificationSupportedByAgent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether your smart home Action will report notifications to Google for this device via ReportStateAndNotification. If your smart home Action enables users to control device notifications, you should update this field and call RequestSyncDevices."]
    pub notification_supported_by_agent: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "otherDeviceIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Alternate IDs associated with this device. This is used to identify cloud synced devices enabled for [local fulfillment](https://developers.google.com/assistant/smarthome/concepts/local)."]
    pub other_device_ids:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AgentOtherDeviceId>>>,
    #[serde(rename = "roomHint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Suggested name for the room where this device is installed. Google attempts to use this value during user setup."]
    pub room_hint: ::std::option::Option<::std::string::String>,
    #[serde(rename = "structureHint")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Suggested name for the structure where this device is installed. Google attempts to use this value during user setup."]
    pub structure_hint: ::std::option::Option<::std::string::String>,
    #[serde(rename = "traits")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Traits supported by the device. See [device traits](https://developers.google.com/assistant/smarthome/traits)."]
    pub traits: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Hardware type of the device. See [device types](https://developers.google.com/assistant/smarthome/guides)."]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "willReportState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether your smart home Action will report state of this device to Google via ReportStateAndNotification."]
    pub will_report_state: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Device information."]
pub struct DeviceInfo {
    #[serde(rename = "hwVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Device hardware version."]
    pub hw_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "manufacturer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Device manufacturer."]
    pub manufacturer: ::std::option::Option<::std::string::String>,
    #[serde(rename = "model")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Device model."]
    pub model: ::std::option::Option<::std::string::String>,
    #[serde(rename = "swVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Device software version."]
    pub sw_version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Identifiers used to describe the device."]
pub struct DeviceNames {
    #[serde(rename = "defaultNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of names provided by the manufacturer rather than the user, such as serial numbers, SKUs, etc."]
    pub default_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Primary name of the device, generally provided by the user."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nicknames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional names provided by the user for the device."]
    pub nicknames: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct Empty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "LINT.IfChange go/shed-per-trait-routing. Making it object to allow for extendible design, where we can add attributes in future."]
pub struct NonLocalTrait {
    #[serde(rename = "trait")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Trait name, e.g., \"action.devices.traits.MediaInitiation\". See [device traits](https://developers.google.com/assistant/smarthome/traits)."]
    pub _trait: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request type for the [`Query`](#google.home.graph.v1.HomeGraphApiService.Query) call."]
pub struct QueryRequest {
    #[serde(rename = "agentUserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Third-party user ID."]
    pub agent_user_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inputs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Inputs containing third-party device IDs for which to get the device states."]
    pub inputs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<QueryRequestInput>>>,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Request ID used for debugging."]
    pub request_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Device ID inputs to QueryRequest."]
pub struct QueryRequestInput {
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Payload containing third-party device IDs."]
    pub payload: ::std::option::Option<::std::boxed::Box<QueryRequestPayload>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Payload containing device IDs."]
pub struct QueryRequestPayload {
    #[serde(rename = "devices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Third-party device IDs for which to get the device states."]
    pub devices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AgentDeviceId>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response type for the [`Query`](#google.home.graph.v1.HomeGraphApiService.Query) call. This should follow the same format as the Google smart home `action.devices.QUERY` [response](https://developers.google.com/assistant/smarthome/reference/intent/query). # Example ```json { \"requestId\": \"ff36a3cc-ec34-11e6-b1a0-64510650abcf\", \"payload\": { \"devices\": { \"123\": { \"on\": true, \"online\": true }, \"456\": { \"on\": true, \"online\": true, \"brightness\": 80, \"color\": { \"name\": \"cerulean\", \"spectrumRGB\": 31655 } } } } } ```"]
pub struct QueryResponse {
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Device states for the devices given in the request."]
    pub payload: ::std::option::Option<::std::boxed::Box<QueryResponsePayload>>,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Request ID used for debugging. Copied from the request."]
    pub request_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Payload containing device states information."]
pub struct QueryResponsePayload {
    #[serde(rename = "devices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "States of the devices. Map of third-party device ID to struct of device states."]
    pub devices: ::std::option::Option<
        ::std::collections::BTreeMap<
            String,
            ::std::collections::BTreeMap<String, ::serde_json::Value>,
        >,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The states and notifications specific to a device."]
pub struct ReportStateAndNotificationDevice {
    #[serde(rename = "notifications")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Notifications metadata for devices. See the **Device NOTIFICATIONS** section of the individual trait [reference guides](https://developers.google.com/assistant/smarthome/traits)."]
    pub notifications:
        ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "states")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "States of devices to update. See the **Device STATES** section of the individual trait [reference guides](https://developers.google.com/assistant/smarthome/traits)."]
    pub states: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request type for the [`ReportStateAndNotification`](#google.home.graph.v1.HomeGraphApiService.ReportStateAndNotification) call. It may include states, notifications, or both. States and notifications are defined per `device_id` (for example, \"123\" and \"456\" in the following example). # Example ```json { \"requestId\": \"ff36a3cc-ec34-11e6-b1a0-64510650abcf\", \"agentUserId\": \"1234\", \"payload\": { \"devices\": { \"states\": { \"123\": { \"on\": true }, \"456\": { \"on\": true, \"brightness\": 10 } }, } } } ```"]
pub struct ReportStateAndNotificationRequest {
    #[serde(rename = "agentUserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Third-party user ID."]
    pub agent_user_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "eventId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier per event (for example, a doorbell press)."]
    pub event_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "followUpToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to maintain state in the follow up notification response. Deprecated. See the [notifications guide](https://developers.google.com/assistant/smarthome/develop/notifications) for details on implementing follow up notifications."]
    pub follow_up_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. State of devices to update and notification metadata for devices."]
    pub payload: ::std::option::Option<::std::boxed::Box<StateAndNotificationPayload>>,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Request ID used for debugging."]
    pub request_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response type for the [`ReportStateAndNotification`](#google.home.graph.v1.HomeGraphApiService.ReportStateAndNotification) call."]
pub struct ReportStateAndNotificationResponse {
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Request ID copied from ReportStateAndNotificationRequest."]
    pub request_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request type for the [`RequestSyncDevices`](#google.home.graph.v1.HomeGraphApiService.RequestSyncDevices) call."]
pub struct RequestSyncDevicesRequest {
    #[serde(rename = "agentUserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Third-party user ID."]
    pub agent_user_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "async")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. If set, the request will be added to a queue and a response will be returned immediately. This enables concurrent requests for the given `agent_user_id`, but the caller will not receive any error responses."]
    pub _async: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response type for the [`RequestSyncDevices`](#google.home.graph.v1.HomeGraphApiService.RequestSyncDevices) call. Intentionally empty upon success. An HTTP response code is returned with more details upon failure."]
pub struct RequestSyncDevicesResponse {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Payload containing the state and notification information for devices."]
pub struct StateAndNotificationPayload {
    #[serde(rename = "devices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The devices for updating state and sending notifications."]
    pub devices: ::std::option::Option<::std::boxed::Box<ReportStateAndNotificationDevice>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request type for the [`Sync`](#google.home.graph.v1.HomeGraphApiService.Sync) call."]
pub struct SyncRequest {
    #[serde(rename = "agentUserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Third-party user ID."]
    pub agent_user_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Request ID used for debugging."]
    pub request_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response type for the [`Sync`](#google.home.graph.v1.HomeGraphApiService.Sync) call. This should follow the same format as the Google smart home `action.devices.SYNC` [response](https://developers.google.com/assistant/smarthome/reference/intent/sync). # Example ```json { \"requestId\": \"ff36a3cc-ec34-11e6-b1a0-64510650abcf\", \"payload\": { \"agentUserId\": \"1836.15267389\", \"devices\": [{ \"id\": \"123\", \"type\": \"action.devices.types.OUTLET\", \"traits\": [ \"action.devices.traits.OnOff\" ], \"name\": { \"defaultNames\": [\"My Outlet 1234\"], \"name\": \"Night light\", \"nicknames\": [\"wall plug\"] }, \"willReportState\": false, \"deviceInfo\": { \"manufacturer\": \"lights-out-inc\", \"model\": \"hs1234\", \"hwVersion\": \"3.2\", \"swVersion\": \"11.4\" }, \"customData\": { \"fooValue\": 74, \"barValue\": true, \"bazValue\": \"foo\" } }] } } ```"]
pub struct SyncResponse {
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Devices associated with the third-party user."]
    pub payload: ::std::option::Option<::std::boxed::Box<SyncResponsePayload>>,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Request ID used for debugging. Copied from the request."]
    pub request_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Payload containing device information."]
pub struct SyncResponsePayload {
    #[serde(rename = "agentUserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Third-party user ID"]
    pub agent_user_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "devices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Devices associated with the third-party user."]
    pub devices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Device>>>,
}
