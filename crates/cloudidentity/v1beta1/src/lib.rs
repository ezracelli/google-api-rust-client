#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Resource representing the Android specific attributes of a Device."]
pub struct AndroidAttributes {
    #[serde(rename = "enabledUnknownSources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether applications from unknown sources can be installed on device."]
    pub enabled_unknown_sources: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "ownerProfileAccount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this account is on an owner/primary profile. For phones, only true for owner profiles. Android 4+ devices can have secondary or restricted user profiles."]
    pub owner_profile_account: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "ownershipPrivilege")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ownership privileges on device."]
    pub ownership_privilege: ::std::option::Option<AndroidAttributesOwnershipPrivilegeEnum>,
    #[serde(rename = "supportsWorkProfile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether device supports Android work profiles. If false, this service will not block access to corp data even if an administrator turns on the \"Enforce Work Profile\" policy."]
    pub supports_work_profile: ::std::option::Option<::std::primitive::bool>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for approving the device to access user data."]
pub struct ApproveDeviceUserRequest {
    #[serde(rename = "customer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer_id}`, where customer_id is the customer to whom the device belongs."]
    pub customer: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for approving the device to access user data."]
pub struct ApproveDeviceUserResponse {
    #[serde(rename = "deviceUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resultant DeviceUser object for the action."]
    pub device_user: ::std::option::Option<::std::boxed::Box<DeviceUser>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for blocking account on device."]
pub struct BlockDeviceUserRequest {
    #[serde(rename = "customer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer_id}`, where customer_id is the customer to whom the device belongs."]
    pub customer: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for blocking the device from accessing user data."]
pub struct BlockDeviceUserResponse {
    #[serde(rename = "deviceUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resultant DeviceUser object for the action."]
    pub device_user: ::std::option::Option<::std::boxed::Box<DeviceUser>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for cancelling an unfinished device wipe."]
pub struct CancelWipeDeviceRequest {
    #[serde(rename = "customer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer_id}`, where customer_id is the customer to whom the device belongs."]
    pub customer: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for cancelling an unfinished device wipe."]
pub struct CancelWipeDeviceResponse {
    #[serde(rename = "device")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resultant Device object for the action. Note that asset tags will not be returned in the device object."]
    pub device: ::std::option::Option<::std::boxed::Box<Device>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for cancelling an unfinished user account wipe."]
pub struct CancelWipeDeviceUserRequest {
    #[serde(rename = "customer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer_id}`, where customer_id is the customer to whom the device belongs."]
    pub customer: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for cancelling an unfinished user account wipe."]
pub struct CancelWipeDeviceUserResponse {
    #[serde(rename = "deviceUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resultant DeviceUser object for the action."]
    pub device_user: ::std::option::Option<::std::boxed::Box<DeviceUser>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for MembershipsService.CheckTransitiveMembership."]
pub struct CheckTransitiveMembershipResponse {
    #[serde(rename = "hasMembership")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Response does not include the possible roles of a member since the behavior of this rpc is not all-or-nothing unlike the other rpcs. So, it may not be possible to list all the roles definitively, due to possible lack of authorization in some of the paths."]
    pub has_membership: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the state associated with an API client calling the Devices API. Resource representing ClientState and supports updates from API users"]
pub struct ClientState {
    #[serde(rename = "assetTags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The caller can specify asset tags for this resource"]
    pub asset_tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "complianceState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The compliance state of the resource as specified by the API client."]
    pub compliance_state: ::std::option::Option<ClientStateComplianceStateEnum>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time the client state data was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field may be used to store a unique identifier for the API resource within which these CustomAttributes are a field."]
    pub custom_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token that needs to be passed back for concurrency control in updates. Token needs to be passed back in UpdateRequest"]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "healthScore")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Health score of the resource"]
    pub health_score: ::std::option::Option<ClientStateHealthScoreEnum>,
    #[serde(rename = "keyValuePairs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The map of key-value attributes stored by callers specific to a device. The total serialized length of this map may not exceed 10KB. No limit is placed on the number of attributes in a map."]
    pub key_value_pairs: ::std::option::Option<
        ::std::collections::BTreeMap<String, ::std::boxed::Box<CustomAttributeValue>>,
    >,
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time the client state data was last updated."]
    pub last_update_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "managed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The management state of the resource as specified by the API client."]
    pub managed: ::std::option::Option<ClientStateManagedEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the ClientState in format: `devices/{device_id}/deviceUsers/{device_user_id}/clientState/{partner_id}`, where partner_id corresponds to the partner storing the data."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ownerType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The owner of the ClientState"]
    pub owner_type: ::std::option::Option<ClientStateOwnerTypeEnum>,
    #[serde(rename = "scoreReason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A descriptive cause of the health score."]
    pub score_reason: ::std::option::Option<::std::string::String>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for creating a Company Owned device."]
pub struct CreateDeviceRequest {
    #[serde(rename = "customer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer_id}`, where customer_id is the customer to whom the device belongs."]
    pub customer: ::std::option::Option<::std::string::String>,
    #[serde(rename = "device")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The device to be created. The name field within this device is ignored in the create method. A new name is created by the method, and returned within the response. Only the fields `device_type`, `serial_number` and `asset_tag` (if present) are used to create the device.`device_type` and `serial_number` are required."]
    pub device: ::std::option::Option<::std::boxed::Box<Device>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Additional custom attribute values may be one of these types"]
pub struct CustomAttributeValue {
    #[serde(rename = "boolValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents a boolean value."]
    pub bool_value: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "numberValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents a double value."]
    pub number_value: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "stringValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents a string value."]
    pub string_value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A Device within the Cloud Identity Devices API. Represents a Device known to Google Cloud, independent of the device ownership, type, and whether it is assigned or in use by a user."]
pub struct Device {
    #[serde(rename = "androidSpecificAttributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Attributes specific to Android devices."]
    pub android_specific_attributes: ::std::option::Option<::std::boxed::Box<AndroidAttributes>>,
    #[serde(rename = "assetTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Asset tag of the device."]
    pub asset_tag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "basebandVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Baseband version of the device."]
    pub baseband_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bootloaderVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Device bootloader version. Example: 0.6.7."]
    pub bootloader_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "brand")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Device brand. Example: Samsung."]
    pub brand: ::std::option::Option<::std::string::String>,
    #[serde(rename = "buildNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Build number of the device."]
    pub build_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "compromisedState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Represents whether the Device is compromised."]
    pub compromised_state: ::std::option::Option<DeviceCompromisedStateEnum>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. When the Company-Owned device was imported. This field is empty for BYOD devices."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deviceType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Type of device."]
    pub device_type: ::std::option::Option<DeviceDeviceTypeEnum>,
    #[serde(rename = "enabledDeveloperOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Whether developer options is enabled on device."]
    pub enabled_developer_options: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "enabledUsbDebugging")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Whether USB debugging is enabled on device."]
    pub enabled_usb_debugging: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "encryptionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Device encryption state."]
    pub encryption_state: ::std::option::Option<DeviceEncryptionStateEnum>,
    #[serde(rename = "imei")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. IMEI number of device if GSM device; empty otherwise."]
    pub imei: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kernelVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Kernel version of the device."]
    pub kernel_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastSyncTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Most recent time when device synced with this service."]
    pub last_sync_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "managementState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Management state of the device"]
    pub management_state: ::std::option::Option<DeviceManagementStateEnum>,
    #[serde(rename = "manufacturer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Device manufacturer. Example: Motorola."]
    pub manufacturer: ::std::option::Option<::std::string::String>,
    #[serde(rename = "meid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. MEID number of device if CDMA device; empty otherwise."]
    pub meid: ::std::option::Option<::std::string::String>,
    #[serde(rename = "model")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Model name of device. Example: Pixel 3."]
    pub model: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Device in format: `devices/{device_id}`, where device_id is the unique id assigned to the Device."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "networkOperator")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Mobile or network operator of device, if available."]
    pub network_operator: ::std::option::Option<::std::string::String>,
    #[serde(rename = "osVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. OS version of the device. Example: Android 8.1.0."]
    pub os_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "otherAccounts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Domain name for Google accounts on device. Type for other accounts on device. On Android, will only be populated if |ownership_privilege| is |PROFILE_OWNER| or |DEVICE_OWNER|. Does not include the account signed in to the device policy app if that account's domain has only one account. Examples: \"com.example\", \"xyz.com\"."]
    pub other_accounts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "ownerType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Whether the device is owned by the company or an individual"]
    pub owner_type: ::std::option::Option<DeviceOwnerTypeEnum>,
    #[serde(rename = "releaseVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. OS release version. Example: 6.0."]
    pub release_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "securityPatchTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. OS security patch update time on device."]
    pub security_patch_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serialNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Serial Number of device. Example: HT82V1A01076."]
    pub serial_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "wifiMacAddresses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "WiFi MAC addresses of device."]
    pub wifi_mac_addresses: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a user's use of a Device in the Cloud Identity Devices API. A DeviceUser is a resource representing a user's use of a Device"]
pub struct DeviceUser {
    #[serde(rename = "compromisedState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Compromised State of the DeviceUser object"]
    pub compromised_state: ::std::option::Option<DeviceUserCompromisedStateEnum>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When the user first signed in to the device"]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "firstSyncTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Most recent time when user registered with this service."]
    pub first_sync_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Default locale used on device, in IETF BCP-47 format."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastSyncTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Last time when user synced with policies."]
    pub last_sync_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "managementState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Management state of the user on the device."]
    pub management_state: ::std::option::Option<DeviceUserManagementStateEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the DeviceUser in format: `devices/{device_id}/deviceUsers/{user_id}`, where user_id is the ID of the user associated with the user session."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "passwordState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Password state of the DeviceUser object"]
    pub password_state: ::std::option::Option<DeviceUserPasswordStateEnum>,
    #[serde(rename = "userAgent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. User agent on the device for this specific user"]
    pub user_agent: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Email address of the user registered on the device."]
    pub user_email: ::std::option::Option<::std::string::String>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Dynamic group metadata like queries and status."]
pub struct DynamicGroupMetadata {
    #[serde(rename = "queries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Memberships will be the union of all queries. Only one entry with USER resource is currently supported. Customers can create up to 100 dynamic groups."]
    pub queries: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DynamicGroupQuery>>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Status of the dynamic group."]
    pub status: ::std::option::Option<::std::boxed::Box<DynamicGroupStatus>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Defines a query on a resource."]
pub struct DynamicGroupQuery {
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Query that determines the memberships of the dynamic group. Examples: All users with at least one `organizations.department` of engineering. `user.organizations.exists(org, org.department=='engineering')` All users with at least one location that has `area` of `foo` and `building_id` of `bar`. `user.locations.exists(loc, loc.area=='foo' && loc.building_id=='bar')`"]
    pub query: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resource_type: ::std::option::Option<DynamicGroupQueryResourceTypeEnum>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The current status of a dynamic group along with timestamp."]
pub struct DynamicGroupStatus {
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the dynamic group."]
    pub status: ::std::option::Option<DynamicGroupStatusStatusEnum>,
    #[serde(rename = "statusTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The latest time at which the dynamic group is guaranteed to be in the given status. If status is `UP_TO_DATE`, the latest time at which the dynamic group was confirmed to be up-to-date. If status is `UPDATING_MEMBERSHIPS`, the time at which dynamic group was created."]
    pub status_time: ::std::option::Option<::std::string::String>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A unique identifier for an entity in the Cloud Identity Groups API. An entity can represent either a group with an optional `namespace` or a user without a `namespace`. The combination of `id` and `namespace` must be unique; however, the same `id` can be used with different `namespace`s."]
pub struct EntityKey {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the entity. For Google-managed entities, the `id` must be the email address of an existing group or user. For external-identity-mapped entities, the `id` must be a string conforming to the Identity Source's requirements. Must be unique within a `namespace`."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The namespace in which the entity exists. If not specified, the `EntityKey` represents a Google-managed entity such as a Google user or a Google Group. If specified, the `EntityKey` represents an external-identity-mapped group. The namespace must correspond to an identity source created in Admin Console and must be in the form of `identitysources/{identity_source_id}."]
    pub namespace: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The `MembershipRole` expiry details."]
pub struct ExpiryDetail {
    #[serde(rename = "expireTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the `MembershipRole` will expire."]
    pub expire_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for MembershipsService.GetMembershipGraph."]
pub struct GetMembershipGraphResponse {
    #[serde(rename = "adjacencyList")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The membership graph's path information represented as an adjacency list."]
    pub adjacency_list:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MembershipAdjacencyList>>>,
    #[serde(rename = "groups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The resources representing each group in the adjacency list. Each group in this list can be correlated to a 'group' of the MembershipAdjacencyList using the 'name' of the Group resource."]
    pub groups: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Group>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Resource representing the Android specific attributes of a Device."]
pub struct GoogleAppsCloudidentityDevicesV1AndroidAttributes {
    #[serde(rename = "enabledUnknownSources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether applications from unknown sources can be installed on device."]
    pub enabled_unknown_sources: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "ownerProfileAccount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this account is on an owner/primary profile. For phones, only true for owner profiles. Android 4+ devices can have secondary or restricted user profiles."]
    pub owner_profile_account: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "ownershipPrivilege")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ownership privileges on device."]
    pub ownership_privilege: ::std::option::Option<
        GoogleAppsCloudidentityDevicesV1AndroidAttributesOwnershipPrivilegeEnum,
    >,
    #[serde(rename = "supportsWorkProfile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether device supports Android work profiles. If false, this service will not block access to corp data even if an administrator turns on the \"Enforce Work Profile\" policy."]
    pub supports_work_profile: ::std::option::Option<::std::primitive::bool>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for approving the device to access user data."]
pub struct GoogleAppsCloudidentityDevicesV1ApproveDeviceUserResponse {
    #[serde(rename = "deviceUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resultant DeviceUser object for the action."]
    pub device_user:
        ::std::option::Option<::std::boxed::Box<GoogleAppsCloudidentityDevicesV1DeviceUser>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for blocking the device from accessing user data."]
pub struct GoogleAppsCloudidentityDevicesV1BlockDeviceUserResponse {
    #[serde(rename = "deviceUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resultant DeviceUser object for the action."]
    pub device_user:
        ::std::option::Option<::std::boxed::Box<GoogleAppsCloudidentityDevicesV1DeviceUser>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for cancelling an unfinished device wipe."]
pub struct GoogleAppsCloudidentityDevicesV1CancelWipeDeviceResponse {
    #[serde(rename = "device")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resultant Device object for the action. Note that asset tags will not be returned in the device object."]
    pub device: ::std::option::Option<::std::boxed::Box<GoogleAppsCloudidentityDevicesV1Device>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for cancelling an unfinished user account wipe."]
pub struct GoogleAppsCloudidentityDevicesV1CancelWipeDeviceUserResponse {
    #[serde(rename = "deviceUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resultant DeviceUser object for the action."]
    pub device_user:
        ::std::option::Option<::std::boxed::Box<GoogleAppsCloudidentityDevicesV1DeviceUser>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents the state associated with an API client calling the Devices API. Resource representing ClientState and supports updates from API users"]
pub struct GoogleAppsCloudidentityDevicesV1ClientState {
    #[serde(rename = "assetTags")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The caller can specify asset tags for this resource"]
    pub asset_tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "complianceState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The compliance state of the resource as specified by the API client."]
    pub compliance_state:
        ::std::option::Option<GoogleAppsCloudidentityDevicesV1ClientStateComplianceStateEnum>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time the client state data was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This field may be used to store a unique identifier for the API resource within which these CustomAttributes are a field."]
    pub custom_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token that needs to be passed back for concurrency control in updates. Token needs to be passed back in UpdateRequest"]
    pub etag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "healthScore")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Health score of the resource. The Health score is the callers specification of the condition of the device from a usability point of view. For example, a third-party device management provider may specify a health score based on its compliance with organizational policies."]
    pub health_score:
        ::std::option::Option<GoogleAppsCloudidentityDevicesV1ClientStateHealthScoreEnum>,
    #[serde(rename = "keyValuePairs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The map of key-value attributes stored by callers specific to a device. The total serialized length of this map may not exceed 10KB. No limit is placed on the number of attributes in a map."]
    pub key_value_pairs: ::std::option::Option<
        ::std::collections::BTreeMap<
            String,
            ::std::boxed::Box<GoogleAppsCloudidentityDevicesV1CustomAttributeValue>,
        >,
    >,
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time the client state data was last updated."]
    pub last_update_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "managed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The management state of the resource as specified by the API client."]
    pub managed: ::std::option::Option<GoogleAppsCloudidentityDevicesV1ClientStateManagedEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the ClientState in format: `devices/{device_id}/deviceUsers/{device_user_id}/clientState/{partner_id}`, where partner_id corresponds to the partner storing the data. For partners belonging to the \"BeyondCorp Alliance\", this is the partner ID specified to you by Google. For all other callers, this is a string of the form: `{customer_id}-suffix`, where `customer_id` is your customer ID. The *suffix* is any string the caller specifies. This string will be displayed verbatim in the administration console. This suffix is used in setting up Custom Access Levels in Context-Aware Access. Your organization's customer ID can be obtained from the URL: `GET https://www.googleapis.com/admin/directory/v1/customers/my_customer` The `id` field in the response contains the customer ID starting with the letter 'C'. The customer ID to be used in this API is the string after the letter 'C' (not including 'C')"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ownerType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The owner of the ClientState"]
    pub owner_type: ::std::option::Option<GoogleAppsCloudidentityDevicesV1ClientStateOwnerTypeEnum>,
    #[serde(rename = "scoreReason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A descriptive cause of the health score."]
    pub score_reason: ::std::option::Option<::std::string::String>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Additional custom attribute values may be one of these types"]
pub struct GoogleAppsCloudidentityDevicesV1CustomAttributeValue {
    #[serde(rename = "boolValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents a boolean value."]
    pub bool_value: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "numberValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents a double value."]
    pub number_value: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "stringValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents a string value."]
    pub string_value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = " A Device within the Cloud Identity Devices API. Represents a Device known to Google Cloud, independent of the device ownership, type, and whether it is assigned or in use by a user."]
pub struct GoogleAppsCloudidentityDevicesV1Device {
    #[serde(rename = "androidSpecificAttributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Attributes specific to Android devices."]
    pub android_specific_attributes:
        ::std::option::Option<::std::boxed::Box<GoogleAppsCloudidentityDevicesV1AndroidAttributes>>,
    #[serde(rename = "assetTag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Asset tag of the device."]
    pub asset_tag: ::std::option::Option<::std::string::String>,
    #[serde(rename = "basebandVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Baseband version of the device."]
    pub baseband_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bootloaderVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Device bootloader version. Example: 0.6.7."]
    pub bootloader_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "brand")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Device brand. Example: Samsung."]
    pub brand: ::std::option::Option<::std::string::String>,
    #[serde(rename = "buildNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Build number of the device."]
    pub build_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "compromisedState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Represents whether the Device is compromised."]
    pub compromised_state:
        ::std::option::Option<GoogleAppsCloudidentityDevicesV1DeviceCompromisedStateEnum>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. When the Company-Owned device was imported. This field is empty for BYOD devices."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deviceType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Type of device."]
    pub device_type: ::std::option::Option<GoogleAppsCloudidentityDevicesV1DeviceDeviceTypeEnum>,
    #[serde(rename = "enabledDeveloperOptions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Whether developer options is enabled on device."]
    pub enabled_developer_options: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "enabledUsbDebugging")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Whether USB debugging is enabled on device."]
    pub enabled_usb_debugging: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "encryptionState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Device encryption state."]
    pub encryption_state:
        ::std::option::Option<GoogleAppsCloudidentityDevicesV1DeviceEncryptionStateEnum>,
    #[serde(rename = "imei")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. IMEI number of device if GSM device; empty otherwise."]
    pub imei: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kernelVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Kernel version of the device."]
    pub kernel_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastSyncTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Most recent time when device synced with this service."]
    pub last_sync_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "managementState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Management state of the device"]
    pub management_state:
        ::std::option::Option<GoogleAppsCloudidentityDevicesV1DeviceManagementStateEnum>,
    #[serde(rename = "manufacturer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Device manufacturer. Example: Motorola."]
    pub manufacturer: ::std::option::Option<::std::string::String>,
    #[serde(rename = "meid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. MEID number of device if CDMA device; empty otherwise."]
    pub meid: ::std::option::Option<::std::string::String>,
    #[serde(rename = "model")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Model name of device. Example: Pixel 3."]
    pub model: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Device in format: `devices/{device_id}`, where device_id is the unique id assigned to the Device."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "networkOperator")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Mobile or network operator of device, if available."]
    pub network_operator: ::std::option::Option<::std::string::String>,
    #[serde(rename = "osVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. OS version of the device. Example: Android 8.1.0."]
    pub os_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "otherAccounts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Domain name for Google accounts on device. Type for other accounts on device. On Android, will only be populated if |ownership_privilege| is |PROFILE_OWNER| or |DEVICE_OWNER|. Does not include the account signed in to the device policy app if that account's domain has only one account. Examples: \"com.example\", \"xyz.com\"."]
    pub other_accounts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "ownerType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Whether the device is owned by the company or an individual"]
    pub owner_type: ::std::option::Option<GoogleAppsCloudidentityDevicesV1DeviceOwnerTypeEnum>,
    #[serde(rename = "releaseVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. OS release version. Example: 6.0."]
    pub release_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "securityPatchTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. OS security patch update time on device."]
    pub security_patch_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serialNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Serial Number of device. Example: HT82V1A01076."]
    pub serial_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "wifiMacAddresses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "WiFi MAC addresses of device."]
    pub wifi_mac_addresses: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a user's use of a Device in the Cloud Identity Devices API. A DeviceUser is a resource representing a user's use of a Device"]
pub struct GoogleAppsCloudidentityDevicesV1DeviceUser {
    #[serde(rename = "compromisedState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Compromised State of the DeviceUser object"]
    pub compromised_state:
        ::std::option::Option<GoogleAppsCloudidentityDevicesV1DeviceUserCompromisedStateEnum>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When the user first signed in to the device"]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "firstSyncTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Most recent time when user registered with this service."]
    pub first_sync_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Default locale used on device, in IETF BCP-47 format."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastSyncTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Last time when user synced with policies."]
    pub last_sync_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "managementState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Management state of the user on the device."]
    pub management_state:
        ::std::option::Option<GoogleAppsCloudidentityDevicesV1DeviceUserManagementStateEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the DeviceUser in format: `devices/{device_id}/deviceUsers/{user_id}`, where user_id is the ID of the user associated with the user session."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "passwordState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Password state of the DeviceUser object"]
    pub password_state:
        ::std::option::Option<GoogleAppsCloudidentityDevicesV1DeviceUserPasswordStateEnum>,
    #[serde(rename = "userAgent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. User agent on the device for this specific user"]
    pub user_agent: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Email address of the user registered on the device."]
    pub user_email: ::std::option::Option<::std::string::String>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for wiping all data on the device."]
pub struct GoogleAppsCloudidentityDevicesV1WipeDeviceResponse {
    #[serde(rename = "device")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resultant Device object for the action. Note that asset tags will not be returned in the device object."]
    pub device: ::std::option::Option<::std::boxed::Box<GoogleAppsCloudidentityDevicesV1Device>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for wiping the user's account from the device."]
pub struct GoogleAppsCloudidentityDevicesV1WipeDeviceUserResponse {
    #[serde(rename = "deviceUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resultant DeviceUser object for the action."]
    pub device_user:
        ::std::option::Option<::std::boxed::Box<GoogleAppsCloudidentityDevicesV1DeviceUser>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A group within the Cloud Identity Groups API. A `Group` is a collection of entities, where each entity is either a user, another group, or a service account."]
pub struct Group {
    #[serde(rename = "additionalGroupKeys")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional entity key aliases for a Group."]
    pub additional_group_keys: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EntityKey>>>,
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time when the `Group` was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An extended description to help users determine the purpose of a `Group`. Must not be longer than 4,096 characters."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display name of the `Group`."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dynamicGroupMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Dynamic group metadata like queries and status."]
    pub dynamic_group_metadata: ::std::option::Option<::std::boxed::Box<DynamicGroupMetadata>>,
    #[serde(rename = "groupKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. The `EntityKey` of the `Group`."]
    pub group_key: ::std::option::Option<::std::boxed::Box<EntityKey>>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. One or more label entries that apply to the Group. Currently supported labels contain a key with an empty value. Google Groups are the default type of group and have a label with a key of `cloudidentity.googleapis.com/groups.discussion_forum` and an empty value. Existing Google Groups can have an additional label with a key of `cloudidentity.googleapis.com/groups.security` and an empty value added to them. **This is an immutable change and the security label cannot be removed once added.** Dynamic groups have a label with a key of `cloudidentity.googleapis.com/groups.dynamic`. Identity-mapped groups for Cloud Search have a label with a key of `system/groups/external` and an empty value. Examples: {\"cloudidentity.googleapis.com/groups.discussion_forum\": \"\"} or {\"system/groups/external\": \"\"}."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The [resource name](https://cloud.google.com/apis/design/resource_names) of the `Group`. Shall be of the form `groups/{group_id}`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. The resource name of the entity under which this `Group` resides in the Cloud Identity resource hierarchy. Must be of the form `identitysources/{identity_source_id}` for external- identity-mapped groups or `customers/{customer_id}` for Google Groups."]
    pub parent: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time when the `Group` was last updated."]
    pub update_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message representing a transitive group of a user or a group."]
pub struct GroupRelation {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Display name for this group."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "group")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name for this group."]
    pub group: ::std::option::Option<::std::string::String>,
    #[serde(rename = "groupKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Entity key has an id and a namespace. In case of discussion forums, the id will be an email address without a namespace."]
    pub group_key: ::std::option::Option<::std::boxed::Box<EntityKey>>,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Labels for Group resource."]
    pub labels: ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    #[serde(rename = "relationType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The relation between the member and the transitive group."]
    pub relation_type: ::std::option::Option<GroupRelationRelationTypeEnum>,
    #[serde(rename = "roles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Membership roles of the member for the group."]
    pub roles: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TransitiveMembershipRole>>>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message that is returned in LRO result of ListClientStates Operation."]
pub struct ListClientStatesResponse {
    #[serde(rename = "clientStates")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Client states meeting the list restrictions."]
    pub client_states: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ClientState>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results. Empty if there are no more results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message that is returned from the ListDeviceUsers method."]
pub struct ListDeviceUsersResponse {
    #[serde(rename = "deviceUsers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Devices meeting the list restrictions."]
    pub device_users: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DeviceUser>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results. Empty if there are no more results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message that is returned from the ListDevices method."]
pub struct ListDevicesResponse {
    #[serde(rename = "devices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Devices meeting the list restrictions."]
    pub devices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Device>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results. Empty if there are no more results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for GroupsService.ListGroups."]
pub struct ListGroupsResponse {
    #[serde(rename = "groups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `Group`s under the specified `parent`."]
    pub groups: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Group>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A continuation token to retrieve the next page of results, or empty if there are no more results available."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for MembershipsService.ListMemberships."]
pub struct ListMembershipsResponse {
    #[serde(rename = "memberships")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `Membership`s under the specified `parent`."]
    pub memberships: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Membership>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A continuation token to retrieve the next page of results, or empty if there are no more results available."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for GroupsService.LookupGroupName."]
pub struct LookupGroupNameResponse {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The [resource name](https://cloud.google.com/apis/design/resource_names) of the looked-up `Group`."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for MembershipsService.LookupMembershipName."]
pub struct LookupMembershipNameResponse {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The [resource name](https://cloud.google.com/apis/design/resource_names) of the looked-up `Membership`. Must be of the form `groups/{group_id}/memberships/{membership_id}`."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response containing resource names of the DeviceUsers associated with the caller's credentials."]
pub struct LookupSelfDeviceUsersResponse {
    #[serde(rename = "customer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The obfuscated customer Id that may be passed back to other Devices API methods such as List, Get, etc."]
    pub customer: ::std::option::Option<::std::string::String>,
    #[serde(rename = "names")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[Resource names](https://cloud.google.com/apis/design/resource_names) of the DeviceUsers in the format: `devices/{device_id}/deviceUsers/{user_resource_id}`, where device_id is the unique ID assigned to a Device and user_resource_id is the unique user ID"]
    pub names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results. Empty if there are no more results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message representing a transitive membership of a group."]
pub struct MemberRelation {
    #[serde(rename = "member")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name for this member if member is a GROUP, otherwise it is empty."]
    pub member: ::std::option::Option<::std::string::String>,
    #[serde(rename = "preferredMemberKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Entity key has an id and a namespace. In case of discussion forums, the id will be an email address without a namespace."]
    pub preferred_member_key: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EntityKey>>>,
    #[serde(rename = "relationType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The relation between the group and the transitive member."]
    pub relation_type: ::std::option::Option<MemberRelationRelationTypeEnum>,
    #[serde(rename = "roles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The membership role details (i.e name of role and expiry time)."]
    pub roles: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TransitiveMembershipRole>>>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A membership within the Cloud Identity Groups API. A `Membership` defines a relationship between a `Group` and an entity belonging to that `Group`, referred to as a \"member\"."]
pub struct Membership {
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time when the `Membership` was created."]
    pub create_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "memberKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The `EntityKey` of the member. Either `member_key` or `preferred_member_key` must be set when calling MembershipsService.CreateMembership but not both; both shall be set when returned."]
    pub member_key: ::std::option::Option<::std::boxed::Box<EntityKey>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The [resource name](https://cloud.google.com/apis/design/resource_names) of the `Membership`. Shall be of the form `groups/{group_id}/memberships/{membership_id}`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "preferredMemberKey")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. The `EntityKey` of the member. Either `member_key` or `preferred_member_key` must be set when calling MembershipsService.CreateMembership but not both; both shall be set when returned."]
    pub preferred_member_key: ::std::option::Option<::std::boxed::Box<EntityKey>>,
    #[serde(rename = "roles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `MembershipRole`s that apply to the `Membership`. If unspecified, defaults to a single `MembershipRole` with `name` `MEMBER`. Must not contain duplicate `MembershipRole`s with the same `name`."]
    pub roles: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MembershipRole>>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of the membership."]
    pub _type: ::std::option::Option<MembershipTypeEnum>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The time when the `Membership` was last updated."]
    pub update_time: ::std::option::Option<::std::string::String>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Membership graph's path information as an adjacency list."]
pub struct MembershipAdjacencyList {
    #[serde(rename = "edges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Each edge contains information about the member that belongs to this group. Note: Fields returned here will help identify the specific Membership resource (e.g name, preferred_member_key and role), but may not be a comprehensive list of all fields."]
    pub edges: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Membership>>>,
    #[serde(rename = "group")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resource name of the group that the members belong to."]
    pub group: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A membership role within the Cloud Identity Groups API. A `MembershipRole` defines the privileges granted to a `Membership`."]
pub struct MembershipRole {
    #[serde(rename = "expiryDetail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The expiry details of the `MembershipRole`. Expiry details are only supported for `MEMBER` `MembershipRoles`. May be set if `name` is `MEMBER`. Must not be set if `name` is any other value."]
    pub expiry_detail: ::std::option::Option<::std::boxed::Box<ExpiryDetail>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the `MembershipRole`. Must be one of `OWNER`, `MANAGER`, `MEMBER`."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The request message for MembershipsService.ModifyMembershipRoles."]
pub struct ModifyMembershipRolesRequest {
    #[serde(rename = "addRoles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `MembershipRole`s to be added. Adding or removing roles in the same request as updating roles is not supported. Must not be set if `update_roles_params` is set."]
    pub add_roles: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MembershipRole>>>,
    #[serde(rename = "removeRoles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `name`s of the `MembershipRole`s to be removed. Adding or removing roles in the same request as updating roles is not supported. It is not possible to remove the `MEMBER` `MembershipRole`. If you wish to delete a `Membership`, call MembershipsService.DeleteMembership instead. Must not contain `MEMBER`. Must not be set if `update_roles_params` is set."]
    pub remove_roles: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "updateRolesParams")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `MembershipRole`s to be updated. Updating roles in the same request as adding or removing roles is not supported. Must not be set if either `add_roles` or `remove_roles` is set."]
    pub update_roles_params:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UpdateMembershipRolesParams>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for MembershipsService.ModifyMembershipRoles."]
pub struct ModifyMembershipRolesResponse {
    #[serde(rename = "membership")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `Membership` resource after modifying its `MembershipRole`s."]
    pub membership: ::std::option::Option<::std::boxed::Box<Membership>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This resource represents a long-running operation that is the result of a network API call."]
pub struct Operation {
    #[serde(rename = "done")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
    pub done: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The error result of the operation in case of failure or cancellation."]
    pub error: ::std::option::Option<::std::boxed::Box<Status>>,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any."]
    pub metadata: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "response")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`."]
    pub response: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for GroupsService.SearchGroups."]
pub struct SearchGroupsResponse {
    #[serde(rename = "groups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `Group`s that match the search query."]
    pub groups: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Group>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A continuation token to retrieve the next page of results, or empty if there are no more results available."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for MembershipsService.SearchTransitiveGroups."]
pub struct SearchTransitiveGroupsResponse {
    #[serde(rename = "memberships")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of transitive groups satisfying the query."]
    pub memberships: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GroupRelation>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results available for listing."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for MembershipsService.SearchTransitiveMemberships."]
pub struct SearchTransitiveMembershipsResponse {
    #[serde(rename = "memberships")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of transitive members satisfying the query."]
    pub memberships: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MemberRelation>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to retrieve the next page of results, or empty if there are no more results."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
pub struct Status {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status code, which should be an enum value of google.rpc.Code."]
    pub code: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of messages that carry the error details. There is a common set of message types for APIs to use."]
    pub details: ::std::option::Option<
        ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    >,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client."]
    pub message: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Message representing the role of a TransitiveMembership."]
pub struct TransitiveMembershipRole {
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "TransitiveMembershipRole in string format. Currently supported TransitiveMembershipRoles: `\"MEMBER\"`, `\"OWNER\"`, and `\"MANAGER\"`."]
    pub role: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The details of an update to a `MembershipRole`."]
pub struct UpdateMembershipRolesParams {
    #[serde(rename = "fieldMask")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fully-qualified names of fields to update. May only contain the field `expiry_detail`."]
    pub field_mask: ::std::option::Option<::std::string::String>,
    #[serde(rename = "membershipRole")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The `MembershipRole`s to be updated. Only `MEMBER` `MembershipRoles` can currently be updated. May only contain a `MembershipRole` with `name` `MEMBER`."]
    pub membership_role: ::std::option::Option<::std::boxed::Box<MembershipRole>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The `UserInvitation` resource represents an email sent to an unmanaged user account (an email address that shares the domain of the Google Workspace customer but is not managed by it yet), inviting them to join the customer’s domain. If the user accepts the `UserInvitation`, the account will become a managed account."]
pub struct UserInvitation {
    #[serde(rename = "mailsSentCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of invitation emails sent to the user."]
    pub mails_sent_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shall be of the form `customers/{customer}/userinvitations/{user_email_address}`."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "State of the `UserInvitation`."]
    pub state: ::std::option::Option<UserInvitationStateEnum>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time when the `UserInvitation` was last updated."]
    pub update_time: ::std::option::Option<::std::string::String>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for wiping all data on the device."]
pub struct WipeDeviceRequest {
    #[serde(rename = "customer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer_id}`, where customer_id is the customer to whom the device belongs."]
    pub customer: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for wiping all data on the device."]
pub struct WipeDeviceResponse {
    #[serde(rename = "device")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resultant Device object for the action. Note that asset tags will not be returned in the device object."]
    pub device: ::std::option::Option<::std::boxed::Box<Device>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for starting an account wipe on device."]
pub struct WipeDeviceUserRequest {
    #[serde(rename = "customer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the customer. If you're using this API for your own organization, use `customers/my_customer` If you're using this API to manage another organization, use `customers/{customer_id}`, where customer_id is the customer to whom the device belongs."]
    pub customer: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for wiping the user's account from the device."]
pub struct WipeDeviceUserResponse {
    #[serde(rename = "deviceUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Resultant DeviceUser object for the action."]
    pub device_user: ::std::option::Option<::std::boxed::Box<DeviceUser>>,
}
