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
    pub mod agent_users {
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
                    #[serde(rename = "requestId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Request ID used for debugging."]
                    pub request_id: ::std::option::Option<::std::string::String>,
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
    #[doc = "Third-party device ID for one device."]
    pub struct AgentDeviceId {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Third-party device ID."]
        pub id: ::std::option::Option<::std::string::String>,
    }
    impl AgentDeviceId {
        pub fn builder() -> AgentDeviceIdBuilder {
            AgentDeviceIdBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Alternate third-party device ID."]
    pub struct AgentOtherDeviceId {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "agentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Project ID for your smart home Action."]
        pub agent_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique third-party device ID."]
        pub device_id: ::std::option::Option<::std::string::String>,
    }
    impl AgentOtherDeviceId {
        pub fn builder() -> AgentOtherDeviceIdBuilder {
            AgentOtherDeviceIdBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Third-party device definition. Next ID = 14"]
    pub struct Device {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Attributes for the traits supported by the device."]
        pub attributes:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom device attributes stored in Home Graph and provided to your smart home Action in each [QUERY](https://developers.google.com/assistant/smarthome/reference/intent/query) and [EXECUTE](https://developers.google.com/assistant/smarthome/reference/intent/execute) intent. Data in this object has a few constraints: No sensitive information, including but not limited to Personally Identifiable Information."]
        pub custom_data:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Device manufacturer, model, hardware version, and software version."]
        pub device_info: ::std::option::Option<::std::boxed::Box<DeviceInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Third-party device ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Names given to this device by your smart home Action."]
        pub name: ::std::option::Option<::std::boxed::Box<DeviceNames>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nonLocalTraits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "See description for \"traits\". For Smart Home Entertainment Devices (SHED) devices, some traits can only be executed on 3P cloud, e.g. \"non_local_traits\": [ { \"trait\": \"action.devices.traits.MediaInitiation\" }, { \"trait\": \"action.devices.traits.Channel\" } ] go/shed-per-trait-routing."]
        pub non_local_traits:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<NonLocalTrait>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notificationSupportedByAgent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether your smart home Action will report notifications to Google for this device via ReportStateAndNotification. If your smart home Action enables users to control device notifications, you should update this field and call RequestSyncDevices."]
        pub notification_supported_by_agent: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "otherDeviceIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Alternate IDs associated with this device. This is used to identify cloud synced devices enabled for [local fulfillment](https://developers.google.com/assistant/smarthome/concepts/local)."]
        pub other_device_ids:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AgentOtherDeviceId>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "roomHint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Suggested name for the room where this device is installed. Google attempts to use this value during user setup."]
        pub room_hint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "structureHint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Suggested name for the structure where this device is installed. Google attempts to use this value during user setup."]
        pub structure_hint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "traits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Traits supported by the device. See [device traits](https://developers.google.com/assistant/smarthome/traits)."]
        pub traits: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hardware type of the device. See [device types](https://developers.google.com/assistant/smarthome/guides)."]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "willReportState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether your smart home Action will report state of this device to Google via ReportStateAndNotification."]
        pub will_report_state: ::std::option::Option<::std::primitive::bool>,
    }
    impl Device {
        pub fn builder() -> DeviceBuilder {
            DeviceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Device information."]
    pub struct DeviceInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hwVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Device hardware version."]
        pub hw_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "manufacturer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Device manufacturer."]
        pub manufacturer: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "model")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Device model."]
        pub model: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "swVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Device software version."]
        pub sw_version: ::std::option::Option<::std::string::String>,
    }
    impl DeviceInfo {
        pub fn builder() -> DeviceInfoBuilder {
            DeviceInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Identifiers used to describe the device."]
    pub struct DeviceNames {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of names provided by the manufacturer rather than the user, such as serial numbers, SKUs, etc."]
        pub default_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Primary name of the device, generally provided by the user."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nicknames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional names provided by the user for the device."]
        pub nicknames: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl DeviceNames {
        pub fn builder() -> DeviceNamesBuilder {
            DeviceNamesBuilder::default()
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
    #[doc = "LINT.IfChange go/shed-per-trait-routing. Making it object to allow for extendible design, where we can add attributes in future."]
    pub struct NonLocalTrait {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trait")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Trait name, e.g., \"action.devices.traits.MediaInitiation\". See [device traits](https://developers.google.com/assistant/smarthome/traits)."]
        pub _trait: ::std::option::Option<::std::string::String>,
    }
    impl NonLocalTrait {
        pub fn builder() -> NonLocalTraitBuilder {
            NonLocalTraitBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request type for the [`Query`](#google.home.graph.v1.HomeGraphApiService.Query) call."]
    pub struct QueryRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "agentUserId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Third-party user ID."]
        pub agent_user_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Inputs containing third-party device IDs for which to get the device states."]
        pub inputs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<QueryRequestInput>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Request ID used for debugging."]
        pub request_id: ::std::option::Option<::std::string::String>,
    }
    impl QueryRequest {
        pub fn builder() -> QueryRequestBuilder {
            QueryRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Device ID inputs to QueryRequest."]
    pub struct QueryRequestInput {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Payload containing third-party device IDs."]
        pub payload: ::std::option::Option<::std::boxed::Box<QueryRequestPayload>>,
    }
    impl QueryRequestInput {
        pub fn builder() -> QueryRequestInputBuilder {
            QueryRequestInputBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Payload containing device IDs."]
    pub struct QueryRequestPayload {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "devices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Third-party device IDs for which to get the device states."]
        pub devices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AgentDeviceId>>>,
    }
    impl QueryRequestPayload {
        pub fn builder() -> QueryRequestPayloadBuilder {
            QueryRequestPayloadBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response type for the [`Query`](#google.home.graph.v1.HomeGraphApiService.Query) call. This should follow the same format as the Google smart home `action.devices.QUERY` [response](https://developers.google.com/assistant/smarthome/reference/intent/query). # Example ```json { \"requestId\": \"ff36a3cc-ec34-11e6-b1a0-64510650abcf\", \"payload\": { \"devices\": { \"123\": { \"on\": true, \"online\": true }, \"456\": { \"on\": true, \"online\": true, \"brightness\": 80, \"color\": { \"name\": \"cerulean\", \"spectrumRGB\": 31655 } } } } } ```"]
    pub struct QueryResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Device states for the devices given in the request."]
        pub payload: ::std::option::Option<::std::boxed::Box<QueryResponsePayload>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Request ID used for debugging. Copied from the request."]
        pub request_id: ::std::option::Option<::std::string::String>,
    }
    impl QueryResponse {
        pub fn builder() -> QueryResponseBuilder {
            QueryResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Payload containing device states information."]
    pub struct QueryResponsePayload {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
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
    impl QueryResponsePayload {
        pub fn builder() -> QueryResponsePayloadBuilder {
            QueryResponsePayloadBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The states and notifications specific to a device."]
    pub struct ReportStateAndNotificationDevice {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notifications")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Notifications metadata for devices. See the **Device NOTIFICATIONS** section of the individual trait [reference guides](https://developers.google.com/assistant/smarthome/traits)."]
        pub notifications:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "states")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "States of devices to update. See the **Device STATES** section of the individual trait [reference guides](https://developers.google.com/assistant/smarthome/traits)."]
        pub states:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ReportStateAndNotificationDevice {
        pub fn builder() -> ReportStateAndNotificationDeviceBuilder {
            ReportStateAndNotificationDeviceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request type for the [`ReportStateAndNotification`](#google.home.graph.v1.HomeGraphApiService.ReportStateAndNotification) call. It may include states, notifications, or both. States and notifications are defined per `device_id` (for example, \"123\" and \"456\" in the following example). # Example ```json { \"requestId\": \"ff36a3cc-ec34-11e6-b1a0-64510650abcf\", \"agentUserId\": \"1234\", \"payload\": { \"devices\": { \"states\": { \"123\": { \"on\": true }, \"456\": { \"on\": true, \"brightness\": 10 } }, } } } ```"]
    pub struct ReportStateAndNotificationRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "agentUserId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Third-party user ID."]
        pub agent_user_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier per event (for example, a doorbell press)."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "followUpToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to maintain state in the follow up notification response. Deprecated. See the [notifications guide](https://developers.google.com/assistant/smarthome/develop/notifications) for details on implementing follow up notifications."]
        pub follow_up_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. State of devices to update and notification metadata for devices."]
        pub payload: ::std::option::Option<::std::boxed::Box<StateAndNotificationPayload>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Request ID used for debugging."]
        pub request_id: ::std::option::Option<::std::string::String>,
    }
    impl ReportStateAndNotificationRequest {
        pub fn builder() -> ReportStateAndNotificationRequestBuilder {
            ReportStateAndNotificationRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response type for the [`ReportStateAndNotification`](#google.home.graph.v1.HomeGraphApiService.ReportStateAndNotification) call."]
    pub struct ReportStateAndNotificationResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Request ID copied from ReportStateAndNotificationRequest."]
        pub request_id: ::std::option::Option<::std::string::String>,
    }
    impl ReportStateAndNotificationResponse {
        pub fn builder() -> ReportStateAndNotificationResponseBuilder {
            ReportStateAndNotificationResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request type for the [`RequestSyncDevices`](#google.home.graph.v1.HomeGraphApiService.RequestSyncDevices) call."]
    pub struct RequestSyncDevicesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "agentUserId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Third-party user ID."]
        pub agent_user_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "async")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If set, the request will be added to a queue and a response will be returned immediately. This enables concurrent requests for the given `agent_user_id`, but the caller will not receive any error responses."]
        pub _async: ::std::option::Option<::std::primitive::bool>,
    }
    impl RequestSyncDevicesRequest {
        pub fn builder() -> RequestSyncDevicesRequestBuilder {
            RequestSyncDevicesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response type for the [`RequestSyncDevices`](#google.home.graph.v1.HomeGraphApiService.RequestSyncDevices) call. Intentionally empty upon success. An HTTP response code is returned with more details upon failure."]
    pub struct RequestSyncDevicesResponse {}
    impl RequestSyncDevicesResponse {
        pub fn builder() -> RequestSyncDevicesResponseBuilder {
            RequestSyncDevicesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Payload containing the state and notification information for devices."]
    pub struct StateAndNotificationPayload {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "devices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The devices for updating state and sending notifications."]
        pub devices: ::std::option::Option<::std::boxed::Box<ReportStateAndNotificationDevice>>,
    }
    impl StateAndNotificationPayload {
        pub fn builder() -> StateAndNotificationPayloadBuilder {
            StateAndNotificationPayloadBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request type for the [`Sync`](#google.home.graph.v1.HomeGraphApiService.Sync) call."]
    pub struct SyncRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "agentUserId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Third-party user ID."]
        pub agent_user_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Request ID used for debugging."]
        pub request_id: ::std::option::Option<::std::string::String>,
    }
    impl SyncRequest {
        pub fn builder() -> SyncRequestBuilder {
            SyncRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response type for the [`Sync`](#google.home.graph.v1.HomeGraphApiService.Sync) call. This should follow the same format as the Google smart home `action.devices.SYNC` [response](https://developers.google.com/assistant/smarthome/reference/intent/sync). # Example ```json { \"requestId\": \"ff36a3cc-ec34-11e6-b1a0-64510650abcf\", \"payload\": { \"agentUserId\": \"1836.15267389\", \"devices\": [{ \"id\": \"123\", \"type\": \"action.devices.types.OUTLET\", \"traits\": [ \"action.devices.traits.OnOff\" ], \"name\": { \"defaultNames\": [\"My Outlet 1234\"], \"name\": \"Night light\", \"nicknames\": [\"wall plug\"] }, \"willReportState\": false, \"deviceInfo\": { \"manufacturer\": \"lights-out-inc\", \"model\": \"hs1234\", \"hwVersion\": \"3.2\", \"swVersion\": \"11.4\" }, \"customData\": { \"fooValue\": 74, \"barValue\": true, \"bazValue\": \"foo\" } }] } } ```"]
    pub struct SyncResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Devices associated with the third-party user."]
        pub payload: ::std::option::Option<::std::boxed::Box<SyncResponsePayload>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Request ID used for debugging. Copied from the request."]
        pub request_id: ::std::option::Option<::std::string::String>,
    }
    impl SyncResponse {
        pub fn builder() -> SyncResponseBuilder {
            SyncResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Payload containing device information."]
    pub struct SyncResponsePayload {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "agentUserId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Third-party user ID"]
        pub agent_user_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "devices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Devices associated with the third-party user."]
        pub devices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Device>>>,
    }
    impl SyncResponsePayload {
        pub fn builder() -> SyncResponsePayloadBuilder {
            SyncResponsePayloadBuilder::default()
        }
    }
}
