#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Associates `members` with a `role`."]
pub struct SasPortalAssignment {
    #[serde(rename = "members")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identities the role is assigned to. It can have the following values: * `{user_email}`: An email address that represents a specific Google account. For example: `alice@gmail.com`. * `{group_email}`: An email address that represents a Google group. For example, `viewers@gmail.com`."]
    pub members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Role that is assigned to `members`."]
    pub role: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for CreateSignedDevice."]
pub struct SasPortalCreateSignedDeviceRequest {
    #[serde(rename = "encodedDevice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. JSON Web Token signed using a CPI private key. Payload must be the JSON encoding of the device. The user_id field must be set."]
    pub encoded_device: ::std::option::Option<::std::string::String>,
    #[serde(rename = "installerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Unique installer id (CPI ID) from the Certified Professional Installers database."]
    pub installer_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Entity representing a SAS customer."]
pub struct SasPortalCustomer {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Name of the organization that the customer entity represents."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Resource name of the customer."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sasUserIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User IDs used by the devices belonging to this customer."]
    pub sas_user_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The Deployment."]
pub struct SasPortalDeployment {
    #[serde(rename = "allowedBillingModes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The allowed billing modes under this deployment."]
    pub allowed_billing_modes:
        ::std::option::Option<::std::vec::Vec<SasPortalDeploymentAllowedBillingModesEnum>>,
    #[serde(rename = "defaultBillingMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Default billing mode for the deployment and devices under it."]
    pub default_billing_mode: ::std::option::Option<SasPortalDeploymentDefaultBillingModeEnum>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The deployment's display name."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Resource name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sasUserIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User ID used by the devices belonging to this deployment. Each deployment should be associated with one unique user ID."]
    pub sas_user_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum SasPortalDeploymentAllowedBillingModesEnum {
    #[serde(rename = "BILLING_MODE_UNSPECIFIED")]
    #[doc = "Billing mode has not been specified."]
    BillingModeUnspecified,
    #[serde(rename = "MOBILE")]
    #[doc = "Price is based on category of CBSD: Category A, Category B registered with SAS."]
    Mobile,
    #[serde(rename = "FIXED_WIRELESS")]
    #[doc = "Price is based on type of CBSD: Base station or CPE."]
    FixedWireless,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Default billing mode for the deployment and devices under it."]
pub enum SasPortalDeploymentDefaultBillingModeEnum {
    #[serde(rename = "BILLING_MODE_UNSPECIFIED")]
    #[doc = "Billing mode has not been specified."]
    BillingModeUnspecified,
    #[serde(rename = "MOBILE")]
    #[doc = "Price is based on category of CBSD: Category A, Category B registered with SAS."]
    Mobile,
    #[serde(rename = "FIXED_WIRELESS")]
    #[doc = "Price is based on type of CBSD: Base station or CPE."]
    FixedWireless,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SasPortalDevice {
    #[serde(rename = "activeConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Current configuration of the device as registered to the SAS."]
    pub active_config: ::std::option::Option<::std::boxed::Box<SasPortalDeviceConfig>>,
    #[serde(rename = "deviceMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Device parameters that can be overridden by both SAS Portal and SAS registration requests."]
    pub device_metadata: ::std::option::Option<::std::boxed::Box<SasPortalDeviceMetadata>>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Device display name."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fccId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The FCC identifier of the device."]
    pub fcc_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "grantRangeAllowlists")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only ranges within the allowlists are available for new grants."]
    pub grant_range_allowlists:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SasPortalFrequencyRange>>>,
    #[serde(rename = "grants")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Grants held by the device."]
    pub grants: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SasPortalDeviceGrant>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The resource path name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "preloadedConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Configuration of the device, as specified via SAS Portal API."]
    pub preloaded_config: ::std::option::Option<::std::boxed::Box<SasPortalDeviceConfig>>,
    #[serde(rename = "serialNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A serial number assigned to the device by the device manufacturer."]
    pub serial_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Device state."]
    pub state: ::std::option::Option<SasPortalDeviceStateEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. Device state."]
pub enum SasPortalDeviceStateEnum {
    #[serde(rename = "DEVICE_STATE_UNSPECIFIED")]
    #[doc = "Unspecified state."]
    DeviceStateUnspecified,
    #[serde(rename = "RESERVED")]
    #[doc = "Device created in the SAS Portal, however, not yet registered with SAS."]
    Reserved,
    #[serde(rename = "REGISTERED")]
    #[doc = "Device registered with SAS."]
    Registered,
    #[serde(rename = "DEREGISTERED")]
    #[doc = "Device de-registered with SAS."]
    Deregistered,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about the device's air interface."]
pub struct SasPortalDeviceAirInterface {
    #[serde(rename = "radioTechnology")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Conditional. This field specifies the radio access technology that is used for the CBSD."]
    pub radio_technology: ::std::option::Option<SasPortalDeviceAirInterfaceRadioTechnologyEnum>,
    #[serde(rename = "supportedSpec")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. This field is related to the `radioTechnology` and provides the air interface specification that the CBSD is compliant with at the time of registration."]
    pub supported_spec: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Conditional. This field specifies the radio access technology that is used for the CBSD."]
pub enum SasPortalDeviceAirInterfaceRadioTechnologyEnum {
    #[serde(rename = "RADIO_TECHNOLOGY_UNSPECIFIED")]
    #[doc = ""]
    RadioTechnologyUnspecified,
    #[serde(rename = "E_UTRA")]
    #[doc = ""]
    EUtra,
    #[serde(rename = "CAMBIUM_NETWORKS")]
    #[doc = ""]
    CambiumNetworks,
    #[serde(rename = "FOUR_G_BBW_SAA_1")]
    #[doc = ""]
    FourGBbwSaa1,
    #[serde(rename = "NR")]
    #[doc = ""]
    Nr,
    #[serde(rename = "DOODLE_CBRS")]
    #[doc = ""]
    DoodleCbrs,
    #[serde(rename = "CW")]
    #[doc = ""]
    Cw,
    #[serde(rename = "REDLINE")]
    #[doc = ""]
    Redline,
    #[serde(rename = "TARANA_WIRELESS")]
    #[doc = ""]
    TaranaWireless,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about the device configuration."]
pub struct SasPortalDeviceConfig {
    #[serde(rename = "airInterface")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about this device's air interface."]
    pub air_interface: ::std::option::Option<::std::boxed::Box<SasPortalDeviceAirInterface>>,
    #[serde(rename = "callSign")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The call sign of the device operator."]
    pub call_sign: ::std::option::Option<::std::string::String>,
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "FCC category of the device."]
    pub category: ::std::option::Option<SasPortalDeviceConfigCategoryEnum>,
    #[serde(rename = "installationParams")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Installation parameters for the device."]
    pub installation_params: ::std::option::Option<::std::boxed::Box<SasPortalInstallationParams>>,
    #[serde(rename = "isSigned")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Whether the configuration has been signed by a CPI."]
    pub is_signed: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "measurementCapabilities")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Measurement reporting capabilities of the device."]
    pub measurement_capabilities:
        ::std::option::Option<::std::vec::Vec<SasPortalDeviceConfigMeasurementCapabilitiesEnum>>,
    #[serde(rename = "model")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about this device model."]
    pub model: ::std::option::Option<::std::boxed::Box<SasPortalDeviceModel>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "State of the configuration."]
    pub state: ::std::option::Option<SasPortalDeviceConfigStateEnum>,
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The last time the device configuration was edited."]
    pub update_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier of a device user."]
    pub user_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "FCC category of the device."]
pub enum SasPortalDeviceConfigCategoryEnum {
    #[serde(rename = "DEVICE_CATEGORY_UNSPECIFIED")]
    #[doc = "Unspecified device category."]
    DeviceCategoryUnspecified,
    #[serde(rename = "DEVICE_CATEGORY_A")]
    #[doc = "Category A."]
    DeviceCategoryA,
    #[serde(rename = "DEVICE_CATEGORY_B")]
    #[doc = "Category B."]
    DeviceCategoryB,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum SasPortalDeviceConfigMeasurementCapabilitiesEnum {
    #[serde(rename = "MEASUREMENT_CAPABILITY_UNSPECIFIED")]
    #[doc = ""]
    MeasurementCapabilityUnspecified,
    #[serde(rename = "MEASUREMENT_CAPABILITY_RECEIVED_POWER_WITH_GRANT")]
    #[doc = ""]
    MeasurementCapabilityReceivedPowerWithGrant,
    #[serde(rename = "MEASUREMENT_CAPABILITY_RECEIVED_POWER_WITHOUT_GRANT")]
    #[doc = ""]
    MeasurementCapabilityReceivedPowerWithoutGrant,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "State of the configuration."]
pub enum SasPortalDeviceConfigStateEnum {
    #[serde(rename = "DEVICE_CONFIG_STATE_UNSPECIFIED")]
    #[doc = ""]
    DeviceConfigStateUnspecified,
    #[serde(rename = "DRAFT")]
    #[doc = ""]
    Draft,
    #[serde(rename = "FINAL")]
    #[doc = ""]
    Final,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Device grant. It is an authorization provided by the Spectrum Access System to a device to transmit using specified operating parameters after a successful heartbeat by the device."]
pub struct SasPortalDeviceGrant {
    #[serde(rename = "channelType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of channel used."]
    pub channel_type: ::std::option::Option<SasPortalDeviceGrantChannelTypeEnum>,
    #[serde(rename = "expireTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The expiration time of the grant."]
    pub expire_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "frequencyRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The transmission frequency range."]
    pub frequency_range: ::std::option::Option<::std::boxed::Box<SasPortalFrequencyRange>>,
    #[serde(rename = "grantId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Grant Id."]
    pub grant_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maxEirp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum Equivalent Isotropically Radiated Power (EIRP) permitted by the grant. The maximum EIRP is in units of dBm/MHz. The value of `maxEirp` represents the average (RMS) EIRP that would be measured by the procedure defined in FCC part 96.41(e)(3)."]
    pub max_eirp: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "moveList")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The DPA move lists on which this grant appears."]
    pub move_list: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SasPortalDpaMoveList>>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "State of the grant."]
    pub state: ::std::option::Option<SasPortalDeviceGrantStateEnum>,
    #[serde(rename = "suspensionReason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the grant is suspended, the reason(s) for suspension."]
    pub suspension_reason: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of channel used."]
pub enum SasPortalDeviceGrantChannelTypeEnum {
    #[serde(rename = "CHANNEL_TYPE_UNSPECIFIED")]
    #[doc = ""]
    ChannelTypeUnspecified,
    #[serde(rename = "CHANNEL_TYPE_GAA")]
    #[doc = ""]
    ChannelTypeGaa,
    #[serde(rename = "CHANNEL_TYPE_PAL")]
    #[doc = ""]
    ChannelTypePal,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "State of the grant."]
pub enum SasPortalDeviceGrantStateEnum {
    #[serde(rename = "GRANT_STATE_UNSPECIFIED")]
    #[doc = ""]
    GrantStateUnspecified,
    #[serde(rename = "GRANT_STATE_GRANTED")]
    #[doc = "The grant has been granted but the device is not heartbeating on it."]
    GrantStateGranted,
    #[serde(rename = "GRANT_STATE_TERMINATED")]
    #[doc = "The grant has been terminated by the SAS."]
    GrantStateTerminated,
    #[serde(rename = "GRANT_STATE_SUSPENDED")]
    #[doc = "The grant has been suspended by the SAS."]
    GrantStateSuspended,
    #[serde(rename = "GRANT_STATE_AUTHORIZED")]
    #[doc = "The device is currently transmitting."]
    GrantStateAuthorized,
    #[serde(rename = "GRANT_STATE_EXPIRED")]
    #[doc = "The grant has expired."]
    GrantStateExpired,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Device data overridable by both SAS Portal and registration requests."]
pub struct SasPortalDeviceMetadata {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about the model of the device."]
pub struct SasPortalDeviceModel {
    #[serde(rename = "firmwareVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The firmware version of the device."]
    pub firmware_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hardwareVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The hardware version of the device."]
    pub hardware_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the device model."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "softwareVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The software version of the device."]
    pub software_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "vendor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the device vendor."]
    pub vendor: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An entry in a DPA's move list."]
pub struct SasPortalDpaMoveList {
    #[serde(rename = "dpaId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the DPA."]
    pub dpa_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "frequencyRange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The frequency range that the move list affects."]
    pub frequency_range: ::std::option::Option<::std::boxed::Box<SasPortalFrequencyRange>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
pub struct SasPortalEmpty {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Frequency range from `low_frequency` to `high_frequency`."]
pub struct SasPortalFrequencyRange {
    #[serde(rename = "highFrequencyMhz")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The highest frequency of the frequency range in MHz."]
    pub high_frequency_mhz: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "lowFrequencyMhz")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The lowest frequency of the frequency range in MHz."]
    pub low_frequency_mhz: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for GenerateSecret."]
pub struct SasPortalGenerateSecretRequest {}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for GenerateSecret."]
pub struct SasPortalGenerateSecretResponse {
    #[serde(rename = "secret")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The secret generated by the string and used by ValidateInstaller."]
    pub secret: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for `GetPolicy` method."]
pub struct SasPortalGetPolicyRequest {
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The resource for which the policy is being requested."]
    pub resource: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about the device installation parameters."]
pub struct SasPortalInstallationParams {
    #[serde(rename = "antennaAzimuth")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Boresight direction of the horizontal plane of the antenna in degrees with respect to true north. The value of this parameter is an integer with a value between 0 and 359 inclusive. A value of 0 degrees means true north; a value of 90 degrees means east. This parameter is optional for Category A devices and conditional for Category B devices."]
    pub antenna_azimuth: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "antennaBeamwidth")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "3-dB antenna beamwidth of the antenna in the horizontal-plane in degrees. This parameter is an unsigned integer having a value between 0 and 360 (degrees) inclusive; it is optional for Category A devices and conditional for Category B devices."]
    pub antenna_beamwidth: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "antennaDowntilt")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Antenna downtilt in degrees and is an integer with a value between -90 and +90 inclusive; a negative value means the antenna is tilted up (above horizontal). This parameter is optional for Category A devices and conditional for Category B devices."]
    pub antenna_downtilt: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "antennaGain")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Peak antenna gain in dBi. This parameter is an integer with a value between -127 and +128 (dBi) inclusive."]
    pub antenna_gain: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "antennaModel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If an external antenna is used, the antenna model is optionally provided in this field. The string has a maximum length of 128 octets."]
    pub antenna_model: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cpeCbsdIndication")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If present, this parameter specifies whether the CBSD is a CPE-CBSD or not."]
    pub cpe_cbsd_indication: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "eirpCapability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This parameter is the maximum device EIRP in units of dBm/10MHz and is an integer with a value between -127 and +47 (dBm/10 MHz) inclusive. If not included, SAS interprets it as maximum allowable EIRP in units of dBm/10MHz for device category."]
    pub eirp_capability: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Device antenna height in meters. When the `heightType` parameter value is \"AGL\", the antenna height should be given relative to ground level. When the `heightType` parameter value is \"AMSL\", it is given with respect to WGS84 datum."]
    pub height: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "heightType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies how the height is measured."]
    pub height_type: ::std::option::Option<SasPortalInstallationParamsHeightTypeEnum>,
    #[serde(rename = "horizontalAccuracy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A positive number in meters to indicate accuracy of the device antenna horizontal location. This optional parameter should only be present if its value is less than the FCC requirement of 50 meters."]
    pub horizontal_accuracy: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "indoorDeployment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the device antenna is indoor or not. `true`: indoor. `false`: outdoor."]
    pub indoor_deployment: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "latitude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Latitude of the device antenna location in degrees relative to the WGS 84 datum. The allowed range is from -90.000000 to +90.000000. Positive values represent latitudes north of the equator; negative values south of the equator."]
    pub latitude: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "longitude")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Longitude of the device antenna location in degrees relative to the WGS 84 datum. The allowed range is from -180.000000 to +180.000000. Positive values represent longitudes east of the prime meridian; negative values west of the prime meridian."]
    pub longitude: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "verticalAccuracy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A positive number in meters to indicate accuracy of the device antenna vertical location. This optional parameter should only be present if its value is less than the FCC requirement of 3 meters."]
    pub vertical_accuracy: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Specifies how the height is measured."]
pub enum SasPortalInstallationParamsHeightTypeEnum {
    #[serde(rename = "HEIGHT_TYPE_UNSPECIFIED")]
    #[doc = "Unspecified height type."]
    HeightTypeUnspecified,
    #[serde(rename = "HEIGHT_TYPE_AGL")]
    #[doc = "AGL height is measured relative to the ground level."]
    HeightTypeAgl,
    #[serde(rename = "HEIGHT_TYPE_AMSL")]
    #[doc = "AMSL height is measured relative to the mean sea level."]
    HeightTypeAmsl,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for `ListCustomers`."]
pub struct SasPortalListCustomersResponse {
    #[serde(rename = "customers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of customers that match the request."]
    pub customers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SasPortalCustomer>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A pagination token returned from a previous call to ListCustomers that indicates from where listing should continue. If the field is missing or empty, it means there are no more customers."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for ListDeployments."]
pub struct SasPortalListDeploymentsResponse {
    #[serde(rename = "deployments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The deployments that match the request."]
    pub deployments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SasPortalDeployment>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A pagination token returned from a previous call to ListDeployments that indicates from where listing should continue. If the field is missing or empty, it means there are no more deployments."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for ListDevices."]
pub struct SasPortalListDevicesResponse {
    #[serde(rename = "devices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The devices that match the request."]
    pub devices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SasPortalDevice>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A pagination token returned from a previous call to ListDevices that indicates from where listing should continue. If the field is missing or empty, it means there is no more devices."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for ListNodes."]
pub struct SasPortalListNodesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A pagination token returned from a previous call to ListNodes that indicates from where listing should continue. If the field is missing or empty, it means there is no more nodes."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The nodes that match the request."]
    pub nodes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SasPortalNode>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for MoveDeployment."]
pub struct SasPortalMoveDeploymentRequest {
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the new parent resource node or customer to reparent the deployment under."]
    pub destination: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for MoveDevice."]
pub struct SasPortalMoveDeviceRequest {
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the new parent resource node or customer to reparent the device under."]
    pub destination: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for MoveNode."]
pub struct SasPortalMoveNodeRequest {
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the new parent resource node or customer to reparent the node under."]
    pub destination: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The Node."]
pub struct SasPortalNode {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The node's display name."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Resource name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sasUserIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User ids used by the devices belonging to this node."]
    pub sas_user_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This resource represents a long-running operation that is the result of a network API call."]
pub struct SasPortalOperation {
    #[serde(rename = "done")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
    pub done: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The error result of the operation in case of failure or cancellation."]
    pub error: ::std::option::Option<::std::boxed::Box<SasPortalStatus>>,
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
#[doc = "Defines an access control policy to the resources."]
pub struct SasPortalPolicy {
    #[serde(rename = "assignments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of assignments"]
    pub assignments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SasPortalAssignment>>>,
    #[serde(rename = "etag")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the etag in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An etag is returned in the response to GetPolicy, and systems are expected to put that etag in the request to SetPolicy to ensure that their change will be applied to the same version of the policy. If no etag is provided in the call to GetPolicy, then the existing policy is overwritten blindly."]
    pub etag: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for `SetPolicy` method."]
pub struct SasPortalSetPolicyRequest {
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The policy to be applied to the `resource`."]
    pub policy: ::std::option::Option<::std::boxed::Box<SasPortalPolicy>>,
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The resource for which the policy is being specified. This policy replaces any existing policy."]
    pub resource: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for SignDevice."]
pub struct SasPortalSignDeviceRequest {
    #[serde(rename = "device")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The device to sign. The device fields name, fcc_id and serial_number must be set. The user_id field must be set."]
    pub device: ::std::option::Option<::std::boxed::Box<SasPortalDevice>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
pub struct SasPortalStatus {
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
#[doc = "Request message for `TestPermissions` method."]
pub struct SasPortalTestPermissionsRequest {
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of permissions to check for the `resource`."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The resource for which the permissions are being requested."]
    pub resource: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for `TestPermissions` method."]
pub struct SasPortalTestPermissionsResponse {
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A set of permissions that the caller is allowed."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for UpdateSignedDevice."]
pub struct SasPortalUpdateSignedDeviceRequest {
    #[serde(rename = "encodedDevice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The JSON Web Token signed using a CPI private key. Payload must be the JSON encoding of the device. The user_id field must be set."]
    pub encoded_device: ::std::option::Option<::std::string::String>,
    #[serde(rename = "installerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Unique installer ID (CPI ID) from the Certified Professional Installers database."]
    pub installer_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request for ValidateInstaller."]
pub struct SasPortalValidateInstallerRequest {
    #[serde(rename = "encodedSecret")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. JSON Web Token signed using a CPI private key. Payload must include a \"secret\" claim whose value is the secret."]
    pub encoded_secret: ::std::option::Option<::std::string::String>,
    #[serde(rename = "installerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Unique installer id (CPI ID) from the Certified Professional Installers database."]
    pub installer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "secret")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Secret returned by the GenerateSecret."]
    pub secret: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for ValidateInstaller."]
pub struct SasPortalValidateInstallerResponse {}
